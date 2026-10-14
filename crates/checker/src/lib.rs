use std::collections::HashSet;
use std::path::{Path, PathBuf};

use anyhow::anyhow;
use once_cell::sync::OnceCell;
use polars::{frame::DataFrame, prelude::*};
use tracing::info;

use crate::traits::CheckerError;

mod traits;
mod worker;

static CRITERIA: OnceCell<DataFrame> = OnceCell::new();

pub fn get_criteria() -> &'static DataFrame {
    CRITERIA.get().unwrap()
}

pub async fn init() -> Result<(), CheckerError> {
    let criteria_path = resolve_criteria_path(&dsa_env::get_env().criteria_path)?;
    tracing::info!(path = %criteria_path.display(), "Loading checker criteria");
    if !criteria_path.is_file() {
        return Err(CheckerError::OtherError(anyhow!(
            "criteria file not found: {}",
            criteria_path.display()
        )));
    }
    let df = CsvReadOptions::default()
        .with_has_header(true)
        .with_infer_schema_length(Some(0))
        .try_into_reader_with_file_path(Some(criteria_path))?
        .finish()?;

    CRITERIA.set(df).ok();

    worker::checker().await;

    Ok(())
}

fn resolve_criteria_path(configured_path: &str) -> Result<PathBuf, CheckerError> {
    let configured_path = Path::new(configured_path);
    if configured_path.is_absolute() || configured_path.exists() {
        return Ok(configured_path.to_path_buf());
    }

    let executable = std::env::current_exe()
        .map_err(|err| CheckerError::OtherError(anyhow!("failed to locate executable: {err}")))?;
    for directory in executable.ancestors() {
        let candidate = directory.join(configured_path);
        if candidate.exists() {
            return Ok(candidate);
        }
    }

    Ok(configured_path.to_path_buf())
}

pub async fn check(input: DataFrame) -> Result<f64, CheckerError> {
    compare(&input, get_criteria())
}

fn compare(input: &DataFrame, criteria: &DataFrame) -> Result<f64, CheckerError> {
    let rows1 = input.shape().0;
    let rows2 = criteria.shape().0;

    if rows1 == 0 || rows2 == 0 {
        return Ok(0.0);
    }

    let mut set1: HashSet<Vec<String>> = HashSet::new();
    let mut set2: HashSet<Vec<String>> = HashSet::new();

    for i in 0..rows1 {
        let row = input.get(i).ok_or(CheckerError::OtherError(anyhow!("")))?;
        let row = row
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>();

        set1.insert(row);
    }
    for i in 0..rows2 {
        let row = criteria
            .get(i)
            .ok_or(CheckerError::OtherError(anyhow!("")))?;
        let row = row
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>();

        set2.insert(row);
    }

    let intersection_size = set1.intersection(&set2).count();

    let similarity = intersection_size as f64 / std::cmp::max(rows1, rows2) as f64;

    info!(
        "{} / {}",
        intersection_size as f64,
        std::cmp::max(rows1, rows2) as f64
    );

    Ok(similarity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_repository_resources_from_the_executable_tree() {
        let path = resolve_criteria_path("resources/criteria.csv").unwrap();
        assert!(path.exists());
    }

    fn fixture(name: &str) -> DataFrame {
        CsvReadOptions::default()
            .with_has_header(true)
            .with_infer_schema_length(Some(0))
            .try_into_reader_with_file_path(Some(
                std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("../../resources")
                    .join(name),
            ))
            .unwrap()
            .finish()
            .unwrap()
    }

    #[test]
    fn scores_partial_and_complete_submissions() {
        let criteria = fixture("criteria.csv");
        assert_eq!(compare(&fixture("test.csv"), &criteria).unwrap(), 0.2);
        assert_eq!(compare(&criteria, &criteria).unwrap(), 1.0);
    }

    #[test]
    fn scores_empty_submissions_as_zero() {
        assert_eq!(
            compare(&DataFrame::empty(), &fixture("criteria.csv")).unwrap(),
            0.0
        );
    }

    #[test]
    fn distinguishes_values_containing_commas() {
        let input = df!("a" => ["x,y"], "b" => ["z"]).unwrap();
        let criteria = df!("a" => ["x"], "b" => ["y,z"]).unwrap();
        assert_eq!(compare(&input, &criteria).unwrap(), 0.0);
    }
}

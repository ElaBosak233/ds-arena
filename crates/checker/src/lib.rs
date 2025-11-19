use crate::traits::CheckerError;
use anyhow::anyhow;
use once_cell::sync::OnceCell;
use polars::{
    frame::DataFrame,
    prelude::*,
};
use std::collections::HashSet;
use std::path::PathBuf;
use std::str::FromStr;

mod traits;
mod worker;

static CRITERIA: OnceCell<DataFrame> = OnceCell::new();

pub fn get_criteria() -> &'static DataFrame {
    CRITERIA.get().unwrap()
}

pub async fn init() -> Result<(), CheckerError> {
    let df = CsvReadOptions::default()
        .with_has_header(true)
        .with_parse_options(CsvParseOptions::default().with_try_parse_dates(true))
        .try_into_reader_with_file_path(Some(PathBuf::from_str(&dsa_env::get_env().criteria_path).unwrap()))?
        .finish()?;

    CRITERIA.set(df).ok();

    worker::checker().await;

    Ok(())
}

pub async fn check(input: DataFrame) -> Result<f64, CheckerError> {
    let rows1 = input.shape().0;
    let rows2 = get_criteria().shape().0;

    if rows1 == 0 || rows2 == 0 {
        return Ok(0.0);
    }

    let mut set1: HashSet<String> = HashSet::new();
    let mut set2: HashSet<String> = HashSet::new();

    for i in 0..rows1 {
        let row = input.get(i).ok_or(CheckerError::OtherError(anyhow!("")))?;
        let row = row
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>()
            .join(",");

        set1.insert(row);
    }
    for i in 0..rows2 {
        let row = get_criteria()
            .get(i)
            .ok_or(CheckerError::OtherError(anyhow!("")))?;
        let row = row
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>()
            .join(",");

        set2.insert(row);
    }

    let intersection_size = set1.intersection(&set2).count();

    let similarity = intersection_size as f64 / std::cmp::max(rows1, rows2) as f64;
    Ok(similarity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() {
        init().await.unwrap();

        let df = CsvReadOptions::default()
            .with_has_header(true)
            .with_parse_options(CsvParseOptions::default().with_try_parse_dates(true))
            .try_into_reader_with_file_path(Some("../../resources/test.csv".into())).unwrap()
            .finish().unwrap();

        let res = check(df).await.unwrap();
        println!("{:?}", res);
    }
}

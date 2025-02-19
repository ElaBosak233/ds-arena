use std::collections::HashSet;

use anyhow::anyhow;
use once_cell::sync::OnceCell;
use polars::{
    frame::DataFrame,
    io::SerReader,
    prelude::{LazyCsvReader, LazyFileListReader},
};

use crate::traits::CheckerError;

mod traits;
mod worker;

static CRITERIA: OnceCell<DataFrame> = OnceCell::new();

pub fn get_criteria() -> &'static DataFrame {
    CRITERIA.get().unwrap()
}

pub async fn init() -> Result<(), CheckerError> {
    let df = LazyCsvReader::new(&dsa_env::get_env().criteria_path)
        .with_has_header(true)
        .finish()?
        .collect()?;

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
    use polars::prelude::{LazyCsvReader, LazyFileListReader};

    use super::*;

    #[tokio::test]
    async fn test() {
        init().await.unwrap();

        let df = LazyCsvReader::new("../../resources/test.csv")
            .with_has_header(true)
            .finish()
            .unwrap()
            .collect()
            .unwrap();

        let res = check(df).await.unwrap();
        println!("{:?}", res);
    }
}

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::model::Transaction;

pub fn parse_transactions_from_json(filepath: &str) -> Result<Vec<Transaction>, Box<dyn Error>> {
    let file = File::open(Path::new(filepath))?;
    Ok(serde_json::from_reader(BufReader::new(file))?)
}

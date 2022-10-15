use anyhow::Error;

pub mod tsv;

pub trait Output {
    fn write_output(&mut self, columns: Vec<String>, rows: Vec<Vec<String>>) -> Result<(), Error>;
}
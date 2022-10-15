use anyhow::Error;

mod csv;
pub use crate::output::csv::CsvOutput;
mod tsv;
pub use crate::output::tsv::TsvOutput;


pub trait Output {
    fn write_output(&mut self, columns: Vec<String>, rows: Vec<Vec<String>>) -> Result<(), Error>;
}
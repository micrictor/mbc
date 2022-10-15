use anyhow::Error;

mod tsv;
pub use crate::output::tsv::TsvOutput;

pub trait Output {
    fn write_output(&mut self, columns: Vec<String>, rows: Vec<Vec<String>>) -> Result<(), Error>;
}
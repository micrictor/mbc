use std::io::Write;

use anyhow::Error;
use crate::output::Output;

pub struct TsvOutput {
    pub file: Box<dyn Write>,
}

impl Output for TsvOutput {
    fn write_output(&mut self, columns: Vec<String>, rows: Vec<Vec<String>>) -> Result<(), Error> {
        writeln!(self.file, "{}", columns.join("\t"))?;
        for row in rows.iter() {
            write!(self.file, "{}", row.join("\t"))?;
            write!(self.file, "\n")?;
        }
        
        Ok(())
    }
}
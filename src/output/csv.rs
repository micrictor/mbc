use std::io::Write;

use anyhow::Error;
use crate::output::Output;

pub struct CsvOutput {
    pub file: Box<dyn Write>,
}

impl Output for CsvOutput {
    fn write_output(&mut self, columns: Vec<String>, rows: Vec<Vec<String>>) -> Result<(), Error> {
        writeln!(self.file, "{}", columns.join(","))?;
        for row in rows.iter() {
            write!(
                self.file,
                "{}",
                row.iter()
                    .map(|x| format!("\"{}\"", x))
                    .collect::<Vec<String>>()
                    .join(",")
            )?;
            write!(self.file, "\n")?;
        }
        
        Ok(())
    }
}
use std::{io::Write, collections::HashMap};

use anyhow::{Error, Context};
use crate::output::Output;

pub struct JsonOutput {
    pub file: Box<dyn Write>,
}

impl Output for JsonOutput {
    fn write_output(&mut self, columns: Vec<String>, rows: Vec<Vec<String>>) -> Result<(), Error> {
        for row in rows.iter() {
            let mut row_map: HashMap<String, String> = HashMap::new();
            for (idx, value) in row.iter().enumerate() {
                row_map.insert(
                    columns.get(idx).unwrap().to_string(), 
                    value.to_string()
                );
            }
            let json_blob = serde_json::to_string(&row_map)
                .with_context(|| "failed to serialize output row")?;
            write!(
                self.file,
                "{}",
                json_blob
            )?;
            write!(self.file, "\n")?;
        }
        
        Ok(())
    }
}
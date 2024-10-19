use std::fs::File;
use std::io::Write;
use std::path::Path;
use anyhow::{anyhow, Result};

pub fn write_to_file(path: &str, content: &str) -> Result<()> {
    let path = Path::new(path);
    let display = path.display();
    let mut file =  File::create(&path).map_err(|e| anyhow!("Error creating file {}: {}", display, e))?;

    file.write_all(content.as_bytes()).map_err(|e| anyhow!("Error writing file: {}", e))?;
    Ok(())
}
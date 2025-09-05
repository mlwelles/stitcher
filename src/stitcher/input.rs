use std::path::PathBuf;
use std::fs;
use crate::stitcher::format::Format;

#[derive(Debug)]
pub struct Input {
    pub path: PathBuf,
    pub extension: String,
    pub format: Format,
}

impl Input {
    pub fn new(path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        if !path.exists() {
            return Err(format!("Path does not exist: {}", path.display()).into());
        }
        
        if !path.is_file() {
            return Err(format!("Path is not a file: {}", path.display()).into());
        }
        
        let buffer = fs::read(&path)?;
        let kind = infer::get(&buffer);
        
        let (mime_type, extension) = if let Some(k) = kind {
            (k.mime_type(), k.extension().to_string())
        } else {
            return Err(format!("Could not determine MIME type for: {}", path.display()).into());
        };
        
        let format = Format::from_mime_type(mime_type)?;
        
        Ok(Input {
            path,
            extension,
            format,
        })
    }
}
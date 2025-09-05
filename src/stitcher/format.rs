#[derive(Debug, PartialEq)]
pub enum Format {
    Jpeg,
    Png, 
    Tiff,
    Mp4,
}

impl Format {
    pub fn as_str(&self) -> &'static str {
        match self {
            Format::Jpeg => "image/jpeg",
            Format::Png => "image/png",
            Format::Tiff => "image/tiff",
            Format::Mp4 => "video/mp4",
        }
    }
    
    pub fn from_mime_type(mime_type: &str) -> Result<Self, Box<dyn std::error::Error>> {
        match mime_type {
            "image/jpeg" => Ok(Format::Jpeg),
            "image/png" => Ok(Format::Png),
            "image/tiff" => Ok(Format::Tiff),
            "video/mp4" => Ok(Format::Mp4),
            _ => Err(format!("Unsupported MIME type: {}", mime_type).into()),
        }
    }
}
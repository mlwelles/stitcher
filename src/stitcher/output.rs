use std::fs;
use std::path::PathBuf;
use crate::stitcher::format::Format;
use crate::stitcher::input::Input;
use ffmpeg_sidecar::command::FfmpegCommand;

#[derive(Debug)]
pub struct Output {
    pub path: PathBuf,
    pub extension: String,
    pub format: Format,
}

impl Output {
    pub fn new(inputs: &Vec<Input>, path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        if inputs.is_empty() {
            return Err("No input sources provided".into());
        }
        
        let output_path = PathBuf::from(path);
        
        let ffmpeg = &mut FfmpegCommand::new();
        ffmpeg.args(["-framerate", "1/4"]);
        for input in inputs {
            if !input.path.exists() {
                return Err(format!("Input file does not exist: {}", input.path.display()).into());
            }
            let path = input.path.to_string_lossy();
            ffmpeg.input(path.as_ref());
        }
        let _ = &mut ffmpeg.codec_video("libx264")
            .args(["-r", "30"])
            .output(output_path.to_string_lossy().as_ref())
            .spawn()?
            .wait()?;

        let buffer = fs::read(&output_path)?;
        let kind = infer::get(&buffer);

        let (mime_type, extension) = if let Some(k) = kind {
            (k.mime_type(), k.extension().to_string())
        } else {
            return Err(format!("Could not determine MIME type for: {}", output_path.display()).into());
        };

        let format = Format::from_mime_type(mime_type)?;
        Ok(Output {
            path: output_path,
            extension,
            format,
        })
    }
}

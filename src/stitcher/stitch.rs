use std::path::PathBuf;
use crate::stitcher::input::Input;
use glob::glob;
use crate::stitcher::Output;

#[derive(Debug)]
pub struct Stitch {
    pub inputs: Vec<Input>,
}

impl Stitch {
    pub fn new(patterns: &[String]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut sources = Vec::new();
        
        for pattern in patterns {
            match Self::find_matching_inputs(pattern) {
                Ok(mut inputs) => {
                    sources.append(&mut inputs);
                }
                Err(_) => {
                    return Err(format!("Invalid input glob pattern: {}", pattern).into());
                }
            }
        }
        
        if sources.is_empty() {
            let pattern_list = patterns.join(", ");
            return Err(format!("No inputs found matching glob pattern(s): {}", pattern_list).into());
        }
        
        Ok(Stitch {
            inputs: sources,
        })
    }

    pub fn create(&self, path: &PathBuf) -> Result<Output, Box<dyn std::error::Error>> {
        Output::new(&self.inputs, path.clone())
    }
    
    fn find_matching_inputs(glob_pattern: &str) -> Result<Vec<Input>, Box<dyn std::error::Error>> {
        let mut sources = Vec::new();
        
        for entry in glob(glob_pattern)? {
            match entry {
                Ok(path) => {
                    if path.is_file() {
                        match Input::new(path.to_path_buf()) {
                            Ok(source) => sources.push(source),
                            Err(e) => return Err(e),
                        }
                    }
                }
                Err(e) => return Err(e.into()),
            }
        }
        
        sources.sort_by(|a, b| a.path.cmp(&b.path));
        Ok(sources)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stitcher::format::Format;
    use std::fs;
    use tempfile::TempDir;

    fn setup_test_files() -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        
        // Copy fixture files to the temp directory
        fs::copy("fixtures/input.jpg", temp_path.join("test1.jpg")).unwrap();
        fs::copy("fixtures/input.png", temp_path.join("test2.png")).unwrap();
        
        // Create a subdirectory with files
        fs::create_dir(temp_path.join("subdir")).unwrap();
        fs::copy("fixtures/input.tiff", temp_path.join("subdir").join("nested.tiff")).unwrap();
        temp_dir
    }
    #[test]
    fn test_stitch_create() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        fs::copy("fixtures/input.png", temp_path.join("input.png")).unwrap();
        fs::copy("fixtures/input2.png", temp_path.join("input2.png")).unwrap();
        fs::copy("fixtures/input3.png", temp_path.join("input3.png")).unwrap();
        fs::copy("fixtures/input.jpg", temp_path.join("input4.jpg")).unwrap();
        fs::copy("fixtures/input.tiff", temp_path.join("input5.tiff")).unwrap();
        let patterns = vec![format!("{}/*", temp_path.display())];
        let result = Stitch::new(&patterns);
        assert!(result.is_ok());
        let stitch = result.unwrap();
        assert_eq!(stitch.inputs.len(), 5);
        for input in stitch.inputs.iter() {
            assert!(input.path.exists());
            assert!(input.path.is_file());
        }
        let output_path = temp_path.join("output.mp4");
        let result = stitch.create(&output_path);
        assert!(result.is_ok(), "{}", result.unwrap_err().to_string());
        let output = result.unwrap();
        assert!(output.path.exists());
        assert!(output.path.is_file());
        let size = output.path.metadata().unwrap().len();
        assert!(size > 0);
        assert_eq!(output.format, Format::Mp4);
        assert_eq!(output.extension, "mp4");
        assert_eq!(output.path, output_path);
    }

    #[test]
    fn test_stitch_new_success() {
        let temp_dir = setup_test_files();
        let temp_path = temp_dir.path();
        
        let patterns = vec![
            format!("{}/*.jpg", temp_path.display()),
            format!("{}/*.png", temp_path.display())
        ];
        let result = Stitch::new(&patterns);
        
        assert!(result.is_ok());
        let stitch = result.unwrap();
        assert_eq!(stitch.inputs.len(), 2); // test1.jpg, test2.png
        
        // Verify that extension and input_type are populated correctly for each image type
        for source in &stitch.inputs {
            match source.path.extension().unwrap().to_str().unwrap() {
                "jpg" => {
                    assert_eq!(source.extension, "jpg");
                    assert_eq!(source.format, Format::Jpeg);
                    assert_eq!(source.format.as_str(), "image/jpeg");
                }
                "png" => {
                    assert_eq!(source.extension, "png");
                    assert_eq!(source.format, Format::Png);
                    assert_eq!(source.format.as_str(), "image/png");
                }
                _ => panic!("Unexpected file extension"),
            }
        }
    }

    #[test]
    fn test_stitch_new_invalid_pattern() {
        let patterns = vec!["[invalid".to_string()];
        let result = Stitch::new(&patterns);
        
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.starts_with("Invalid input glob pattern: "));
        assert!(error_msg.contains("[invalid"));
    }

    #[test]
    fn test_stitch_new_no_matching_files() {
        let temp_dir = setup_test_files();
        let temp_path = temp_dir.path();
        
        let patterns = vec![format!("{}/*.nonexistent", temp_path.display())];
        let result = Stitch::new(&patterns);
        
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.starts_with("No inputs found matching glob pattern(s): "));
        assert!(error_msg.contains("*.nonexistent"));
    }

    #[test]
    fn test_stitch_new_multiple_patterns_with_overlap() {
        let temp_dir = setup_test_files();
        let temp_path = temp_dir.path();
        
        let patterns = vec![
            format!("{}/test1.jpg", temp_path.display()),
            format!("{}/*.jpg", temp_path.display())
        ];
        let result = Stitch::new(&patterns);
        
        assert!(result.is_ok());
        let stitch = result.unwrap();
        // Should have duplicates: test1.jpg appears twice (once from each pattern)
        assert_eq!(stitch.inputs.len(), 2); // test1.jpg, test1.jpg
    }

    #[test]
    fn test_stitch_new_with_nested_files() {
        let temp_dir = setup_test_files();
        let temp_path = temp_dir.path();
        
        let patterns = vec![format!("{}/**/*.tiff", temp_path.display())];
        let result = Stitch::new(&patterns);
        
        assert!(result.is_ok());
        let stitch = result.unwrap();
        assert_eq!(stitch.inputs.len(), 1); // nested.tiff
        
        // Should be a TIFF image
        for source in &stitch.inputs {
            assert_eq!(source.extension, "tif");
            assert_eq!(source.format, Format::Tiff);
            assert_eq!(source.format.as_str(), "image/tiff");
        }
    }
}
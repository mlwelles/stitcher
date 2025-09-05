# Stitcher

A command-line tool for stitching media mirrored by [reflector](https://github.com/adicarlo/reflector) 
into playable videos.

## Features (so far)

- Process multiple image files using glob patterns
- Support for JPEG, PNG, and TIFF formats
- Automatic file type detection and validation
- Sorted input by filename within each pattern group
- Outputs movie stitched from matching input files;

## Dependencies
Dependencies

- [ffmpeg](https://ffmpeg.org/)

### Dependency Installation
On macOS:

`brew install ffmpeg`

On Debian-based systems:

`apt install -y ffmpeg`

## Usage

```bash
cargo run -- "*.jpg" "images/*.png" "**/*.tiff"
```

The tool accepts one or more glob patterns as arguments and processes all matching image files.

## Supported Input Formats

- **JPEG** (`image/jpeg`) - `.jpg` extension
- **PNG** (`image/png`) - `.png` extension  
- **TIFF** (`image/tiff`) - `.tif` extension

## Error Handling

The tool will return errors for:
- Invalid glob patterns
- Files that cannot be read
- Files with unsupported MIME types
- No files matching any of the provided patterns

## Development

Run tests:
```bash
cargo test
```

The test suite uses fixture images in the `fixtures/` directory to verify format detection and processing.
use clap::Parser;

mod stitcher;

use stitcher::Stitch;

#[derive(Parser)]
#[command(name = "stitcher")]
#[command(about = "A tool for stitching files together")]
struct Args {
    #[arg(help = "File patterns to process")]
    patterns: Vec<String>,
}

fn main() {
    let args = Args::parse();
    
    if args.patterns.is_empty() {
        println!("No file patterns provided");
        return;
    }
    
    match Stitch::new(&args.patterns) {
        Ok(stitch) => {
            println!("Stitch created with {} inputs:", stitch.inputs.len());
            for input in &stitch.inputs {
                println!("  {}", input.path.display());
            }
        }
        Err(e) => {
            eprintln!("Error creating stitch: {}", e);
        }
    }
}

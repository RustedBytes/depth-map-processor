use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the input 16-bit PNG depth map
    #[arg(short, long)]
    input: PathBuf,

    /// Path to save the normalized grayscale output
    #[arg(short, long, default_value = "depth_grayscale_rust.png")]
    output: PathBuf,

    /// Optional path to save the colorized visualization (Turbo colormap)
    #[arg(long)]
    viz: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let stats =
        depth_map_processor::process_depth_map(&args.input, &args.output, args.viz.as_deref())?;

    println!("Original Resolution: {}x{}", stats.width, stats.height);
    println!("Data type: u16");
    println!("Shape: ({}, {}, 1)", stats.height, stats.width);
    println!("Depth in meters (sample): {} m", stats.sample_depth_m);
    println!("Min depth: {}", stats.min_val);
    println!("Max depth: {}", stats.max_val);
    println!("Saved grayscale output to {}", args.output.display());

    if let Some(viz) = args.viz {
        println!("Saved visualization to {}", viz.display());
    }

    Ok(())
}

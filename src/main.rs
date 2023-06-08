use std::error::Error;
use std::path::Path;
use std::process;

use clap::{Args, Parser, Subcommand};

mod dalle;

#[derive(Parser)]
#[command(name = "openai")]
#[command(author = "Vail Gold")]
#[command(version = "0.0.1")]
#[command(about = "Interacts with OpenAI APIs via command line", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Uses Dall-E service
    DallE(DallEArgs),
}

#[derive(Args)]
struct DallEArgs {
    #[command(subcommand)]
    command: DallECommands,
}

#[derive(Subcommand)]
enum DallECommands {
    /// Generates images from a text prompt, saving generated images to provided directory
    /// and filenames are emitted to STDOUT
    Generate(DallEGenArgs),
}

#[derive(Args)]
struct DallEGenArgs {
    /// Text prompt to use to generate images
    prompt: String,
    /// Number of images to generate
    #[arg(long, default_value_t = 1)]
    n: u8,
    /// Size of image; must be 256, 512 or 1024
    #[arg(long, default_value = "1024")]
    size: String,
    /// Directory into which to save generated images
    #[arg(short, long, default_value = ".")]
    dir: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::DallE(dalle) => {
            match &dalle.command {
                DallECommands::Generate(gen) => {
                    let p = Path::new(&gen.dir);
                    if !(p.exists() && p.is_dir()) {
                        eprintln!("Directory does not exist: {}", gen.dir);
                        process::exit(1);
                    }
                    process::exit(dalle::generate(&gen.prompt, gen.n, &gen.size, &gen.dir))
                }
            }
        }
    }
}

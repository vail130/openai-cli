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
    Edit(DallEEditArgs),
}

#[derive(Args)]
struct DallEGenArgs {
    /// Text prompt of desired images
    prompt: String,
    /// Number of images
    #[arg(long, default_value_t = 1)]
    n: u8,
    /// Size of image; must be 256, 512 or 1024
    #[arg(long, default_value = "1024")]
    size: String,
    /// Directory into which to save images
    #[arg(short, long, default_value = ".")]
    dir: String,
}

#[derive(Args)]
struct DallEEditArgs {
    /// Path to image to edit
    image: String,
    /// Text prompt of desired images
    prompt: String,
    /// Path to mask image used to determine where to edit image
    #[arg(long, default_value = "")]
    mask: String,
    /// Number of images
    #[arg(long, default_value_t = 1)]
    n: u8,
    /// Size of image; must be 256, 512 or 1024
    #[arg(long, default_value = "1024")]
    size: String,
    /// Directory into which to save images
    #[arg(short, long, default_value = ".")]
    dir: String,
}

fn check_dir(dir: &String) {
    let p = Path::new(dir);
    if !(p.exists() && p.is_dir()) {
        eprintln!("Directory does not exist: {}", dir);
        process::exit(1);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let exit_code = match &cli.command {
        Commands::DallE(dalle) => match &dalle.command {
            DallECommands::Generate(gen) => {
                check_dir(&gen.dir);
                dalle::generate(&gen.prompt, gen.n, &gen.size, &gen.dir)
            }
            DallECommands::Edit(edit) => {
                check_dir(&edit.dir);
                dalle::edit(
                    &edit.image,
                    &edit.mask,
                    &edit.prompt,
                    edit.n,
                    &edit.size,
                    &edit.dir,
                )
            }
        },
    };
    process::exit(exit_code)
}

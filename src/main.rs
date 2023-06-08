use std::path::Path;
use clap::{Args, Parser, Subcommand};

mod dalle;

#[derive(Parser)]
#[command(name = "open-ai")]
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
    /// Generates images from a text prompt
    Generate(DallEGenArgs),
}

#[derive(Args)]
struct DallEGenArgs {
    /// Text prompt to use to generate images
    prompt: String,
    /// Number of images to generate
    #[arg(long, default_value_t = 1)]
    n: u8,
    /// Must be 256, 512 or 1024
    #[arg(long, default_value = "1024")]
    size: String,
    /// Directory to save generated image
    #[arg(short, long, default_value = ".")]
    dir: String,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::DallE(dalle) => {
            match &dalle.command {
                DallECommands::Generate(gen) => {
                    let p = Path::new(&gen.dir);
                    // TODO: Improve erroring
                    assert!(p.exists() && p.is_dir());
                    dalle::generate(&gen.prompt, gen.n, &gen.size, &gen.dir)
                }
            }
        }
    }
}

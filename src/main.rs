use std::{error::Error, path::Path, process};

use clap::{Args, Parser, Subcommand};

mod images;

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
    /// Uses Images service
    Images(ImagesArgs),
}

#[derive(Args)]
struct ImagesArgs {
    #[command(subcommand)]
    command: ImagesCommands,
}

#[derive(Subcommand)]
enum ImagesCommands {
    /// Creates images from a text prompt, saving generated images to provided directory
    /// and filenames are emitted to STDOUT
    Create(ImagesCreateArgs),
    /// Creates an edited or extended image given an original image and a prompt, saving generated images to provided directory
    /// and filenames are emitted to STDOUT
    Edit(ImagesEditArgs),
    /// Creates variations of an image, saving generated images to provided directory
    /// and filenames are emitted to STDOUT
    Variation(ImagesVariationArgs),
}

#[derive(Args)]
struct ImagesCreateArgs {
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
struct ImagesEditArgs {
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

#[derive(Args)]
struct ImagesVariationArgs {
    /// Path to image to derive variation
    image: String,
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
        Commands::Images(images) => match &images.command {
            ImagesCommands::Create(args) => {
                check_dir(&args.dir);
                images::create(&args.prompt, args.n, &args.size, &args.dir)
            }
            ImagesCommands::Edit(args) => {
                check_dir(&args.dir);
                images::edit(
                    &args.image,
                    &args.mask,
                    &args.prompt,
                    args.n,
                    &args.size,
                    &args.dir,
                )
            }
            ImagesCommands::Variation(args) => {
                check_dir(&args.dir);
                images::variation(&args.image, args.n, &args.size, &args.dir)
            }
        },
    };
    process::exit(exit_code)
}

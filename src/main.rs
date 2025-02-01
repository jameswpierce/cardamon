use clap::{Parser, Subcommand};

pub mod cardamon;

// const ARTIST_NAMESPACE: Uuid = uuid::uuid!("dd625495-4816-4cb8-81f0-13ec4629c2cb");
// const ALBUM_NAMESPACE: Uuid = uuid::uuid!("c117510b-b0dc-4b42-b84d-ddda5d69c5f4");
// const TRACK_NAMESPACE: Uuid = uuid::uuid!("a9c95d7c-c3b4-4217-adcc-8153600e3545");

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    output: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // builds cardamon's static site
    Build {},
    // starts the server
    Serve {},
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Build {}) => {
            println!("building cardamon...");
            cardamon::build::build()?;
        }
        Some(Commands::Serve {}) => {
            println!("starting cardamon server...");
            cardamon::serve::serve()?;
        }
        None => {}
    }

    Ok(())
}

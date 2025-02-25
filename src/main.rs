use clap::{Parser, Subcommand};

pub mod cardamon;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // builds cardamon's static site
    Build {},
    // starts the server
    Serve {},
    // starts server in dev mode (rebuilds with changes to templates folder)
    Dev {},
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    println!("cardamon v{}", env!("CARGO_PKG_VERSION"));

    match &cli.command {
        Some(Commands::Build {}) => {
            println!("building cardamon...");
            cardamon::build::build()?;
        }
        Some(Commands::Serve {}) => {
            println!("building cardamon...");
            cardamon::build::build()?;
            println!("starting cardamon server...");
            cardamon::serve::serve(false)?;
        }
        Some(Commands::Dev {}) => {
            println!("building cardamon...");
            cardamon::build::build()?;
            println!("starting cardamon server in dev mode...");
            cardamon::serve::serve(true)?;
        }
        None => {}
    }

    Ok(())
}

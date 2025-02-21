use clap::{Parser, Subcommand};

pub mod cardamon;

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
            println!("building cardamon...");
            cardamon::build::build()?;
            println!("starting cardamon server...");
            cardamon::serve::main()?;
        }
        None => {}
    }

    Ok(())
}

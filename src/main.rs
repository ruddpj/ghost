mod store;
mod session;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Commands {
    Save { name: String },
    Load { name: String },
}


fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Save { name } => {
            match session::save_session(name.clone()) {
                Ok(session) => {
                   store::serialize_session(&session)
                    .expect("Failed to save to disk");
                    println!("Session saved: {}", name);
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Load { name } => {
            match store::deserialize_session(name) {
                Ok(session) => session::load_session(session),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
}

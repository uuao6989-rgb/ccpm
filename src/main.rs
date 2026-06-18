use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ccpm", version = env!("CARGO_PKG_VERSION"), about = "C/C++ Package Manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {

    /// Create a new C/C++ project
    #[command(long_about = "Creates a new C/C++ project with the name following the command")]
    New,

    /// Refresh the local package cache
    #[command(long_about = "Refreshes CCPM to the newest version and fetches missing external dependencies.")]
    Refresh,

    /// Build the current project
    #[command(long_about = "Compiles the current project workspace, fetches missing dependencies, and outputs the binary to the bin/debug directory.")]
    Build,

    /// Create a release build
    #[command(long_about = "Compiles the current project workspace, fetches missing dependencies, and outputs the binary to the bin/release directory.")]
    Release,
    
    /// Print version information
    #[command(long_about = "Outputs the current installed version of CCPM and also tells you if an update is recommended or not.")]
    Version,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::New => println!("Creating new..."),
        Commands::Refresh => println!("Refreshing..."),
        Commands::Build => println!("Building..."),
        Commands::Release => println!("Releasing..."),
        Commands::Version => {
            // env!("CARGO_PKG_VERSION") dynamically pulls the version from Cargo.toml,
            // but since you hardcoded it in the attribute, we can print it here:
            print!("ccpm ");
            println!(env!("CARGO_PKG_VERSION"))
        }
    }
}
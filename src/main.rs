use clap::Parser;
use color_eyre::eyre::Result;

/// Password Vault 🔐 built with Rust that runs in the terminal.
#[derive(Parser, Debug)]
struct Cli {
    /// Application that requires a password
    #[clap(short, long)]
    domain: String,
    
    /// Login/Username used to create password
    #[clap(short, long)]
    login: String,
    
    /// Optional password to associate with domain and login
    #[clap(short, long)]
    password: Option<String>,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut args = Cli::parse();

    // Create random password if it has not been given
    if let None = &args.password {
        args.password = Some(volta::random_password());
    }
    
    dbg!("{}", &args);
    
    Ok(())
}


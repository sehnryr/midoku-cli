use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Path to the extension
    #[arg(short, long)]
    pub extension: String,

    /// Settings for the extension (json string)
    #[arg(short, long)]
    pub settings: Option<String>,
}

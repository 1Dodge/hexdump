use clap::Parser;

fn parse_int(src: &str) -> Result<u64, String> {
    if let Some(hex) = src.strip_prefix("0x") {
        u64::from_str_radix(hex, 16).map_err(|e| e.to_string())
    } else {
        src.parse::<u64>().map_err(|e| e.to_string())
    }
}

#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    /// Path to file to dump
    #[arg()]
    pub file: String,

    /// Visualization mode
    #[arg(short, long)]
    pub visualization: Option<String>,

    /// Start address of dump
    #[arg(short, long, value_parser = parse_int)]
    pub start: Option<u64>,

    /// Number of bytes to show
    #[arg(short, long, value_parser = parse_int)]
    pub num_bytes: Option<u64>,

    /// End address of dump
    #[arg(short, long, value_parser = parse_int)]
    pub end: Option<u64>,
}

impl Cli {
    pub fn get_args() -> Cli {
        Cli::parse()
    }
}

mod args;
#[macro_use]
mod color;
mod dump;

use args::Cli;
use dump::*;

fn main() {
    let cli = Cli::get_args();
    let mut dump = Dump::new(&cli.file);
    dump.check_args(&cli);
    dump.print_dump();
}

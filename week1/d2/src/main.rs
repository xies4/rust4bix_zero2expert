use clap::Parser;
use color_eyre::eyre::Result;
use std::fs::File;
use std::io::{self, Read, BufReader};

/// GC% toolkit - stream bytes and count total
#[derive(Parser, Debug)]
#[command(name = "gc", version, about = "GC% toolkit - stream bytes and count total")]
struct Args {
    /// The input file to process
    #[arg(value_name = "INPUT", default_value = "-")]
    input: String,

    /// Read from stdin when INPUT is "-"
    #[arg(long)]
    read_stdin: bool,
}

fn count_bytes<R: Read>(mut r: R) -> io::Result<u64> {
    let mut buf = [0u8; 64 * 1024];
    let mut total: u64 = 0;
    loop {
        let n = r.read(&mut buf)?;
        if n == 0 { break; }
        total += n as u64;
    }
    Ok(total)
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();
    
    // If input is "-" (default) and stdin is not available, show help
    if args.input == "-" {
        // Check if stdin is a terminal (no piped input)
        if atty::is(atty::Stream::Stdin) {
            // No input provided, show help
            Args::parse_from(&["d2", "--help"]);
            return Ok(());
        }
        // Read from stdin
        println!("INPUT\t{}", args.input);
        let stdin = io::stdin();
        let handle = stdin.lock();
        let total = count_bytes(handle)?;
        println!("BYTES\t{}", total);
    } else {
        // Read from file
        println!("INPUT\t{}", args.input);
        let file = File::open(&args.input)?;
        let reader = BufReader::new(file);
        let total = count_bytes(reader)?;
        println!("BYTES\t{}", total);
    }
    Ok(())
}

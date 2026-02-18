use clap::Parser;
use color_eyre::eyre::Result;

#[derive(Parser)]
#[command(name = "gc", version, about = "GC% toolkit — Day 1 skeleton")]
struct Args {
    /// The input file to process
    #[arg(value_name = "INPUT", default_value = "-")]
    input: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();
    println!("INPUT\t{}", args.input);
    // Todo: open input file. 
    Ok(())
}
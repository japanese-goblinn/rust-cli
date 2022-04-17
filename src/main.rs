use clap::Parser;
use anyhow::Result;
use std::io::BufWriter;
use grrs::*;

fn main() -> Result<()> {
  let args = Args::parse();
  let file = open_file(&args.path)?;

  // writing to stdout is not cheap so we using writer instead of print macro
  // to not acquire lock to stdout at every write we acquire it here with `lock()`
  let stout = std::io::stdout();
  let writer = BufWriter::new(stout.lock());
  find_matches(file, &args.pattern, writer)
}

use clap::Parser;
use anyhow::{Context, Result};
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser, Debug)]
pub struct Args {
  pub pattern: String,

  // PathBuf is like a String but for file system paths that work cross-platform.
  #[clap(parse(from_os_str))]
  pub path: PathBuf
}

pub fn open_file(path: &PathBuf) -> Result<File, anyhow::Error> {
  File::open(path)
    .with_context(|| format!("Could not open file: {:?}", path))
}

pub fn find_matches(
  file: File, 
  pattern: &str,
  mut writer: impl std::io::Write
) -> Result<(), anyhow::Error> {
  let mut reader = BufReader::new(file);
  let mut line = String::new();
  let mut lines_count = 0;
  loop {
    let bytes_read = reader.read_line(&mut line)
      .with_context(|| "Could not read line from file".to_string())?;
    if bytes_read == 0 {
      return Ok(());
    }
    lines_count += 1;
    if line.contains(pattern) {
      writeln!(writer, "{}. {}", lines_count, line)
        .with_context(|| "Could not write to writer".to_string())?;
    }
    line.clear();
  }
}

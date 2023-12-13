use clap::Parser;
use std::{fs::File, io::Read, path::PathBuf};

#[derive(Parser)]
struct Args {
    path: PathBuf,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("no digit found: {0}")]
    NoDigitFound(String),
    #[error("not a number: {0}")]
    NotNuber(String),
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let mut content = String::new();
    File::open(args.path)?.read_to_string(&mut content)?;
    let sum = content
        .lines()
        .map(extract_number)
        .try_fold(0_u32, |acc, value| value.map(|v| acc + v as u32))?;
    println!("{sum}");
    Ok(())
}

fn extract_number(line: &str) -> Result<u8, Error> {
    let a_index = line
        .find(|c: char| c.is_ascii_digit())
        .ok_or_else(|| Error::NoDigitFound(line.into()))?;
    let a = unsafe { line.get_unchecked(a_index..a_index + 1) };
    let b_index = line
        .rfind(|c: char| c.is_ascii_digit())
        .ok_or_else(|| Error::NoDigitFound(line.into()))?;
    let b = unsafe { line.get_unchecked(b_index..b_index + 1) };
    let joined = format!("{a}{b}");
    joined.parse().map_err(|_| Error::NotNuber(joined))
}

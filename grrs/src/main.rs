use anyhow::{Context, Result};
use clap::Parser;
use grrs::find_matches;
use std::{fs, io::stdout};

#[derive(Debug, Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", &args.path.display()))?;

    find_matches(&content, &args.pattern, &mut stdout())?;

    Ok(())
}

#[test]
fn find_a_match() -> Result<()> {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result)?;
    assert_eq!(result, b"lorem ipsum\n");
    Ok(())
}

use anyhow::{Context, Result};
use std::io::Write;
pub fn find_matches(content: &str, pattern: &str, mut writer: impl Write) -> Result<()> {
    for (i, line) in content.lines().enumerate() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)
                .with_context(|| format!("Error reading line number `{}`", i))?;
        }
    }
    Ok(())
}

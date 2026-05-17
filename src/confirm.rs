use std::io::{self, Write};

use anyhow::{bail, Result};

pub fn confirm_or_bail(action: &str, yes: bool) -> Result<()> {
    if yes {
        return Ok(());
    }

    eprint!("{action} Type 'yes' to continue: ");
    io::stderr().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    if input.trim() == "yes" {
        Ok(())
    } else {
        bail!("aborted")
    }
}

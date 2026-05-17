use anyhow::Result;
use serde::Serialize;

pub fn json<T: Serialize>(value: &T) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

pub fn value_or_json<T: Serialize>(
    json_output: bool,
    value: &T,
    human: impl FnOnce() -> Result<()>,
) -> Result<()> {
    if json_output {
        json(value)
    } else {
        human()
    }
}

pub fn json_or_done<T: Serialize>(json_output: bool, value: &T, message: &str) -> Result<()> {
    if json_output {
        json(value)
    } else {
        println!("{message}");
        Ok(())
    }
}

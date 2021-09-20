use anyhow::Result;
use xshell::{cmd, Shell};

pub(crate) fn run(sh: &Shell) -> Result<()> {
    cmd!(sh, "cargo build -p xtask")
        .env("UPDATE_XFLAGS", "1")
        .run()?;
    Ok(())
}

use anyhow::Result;
use xshell::{cmd, Shell};

use crate::flags::Phpt;

pub(crate) fn run(sh: &Shell, cmd: &Phpt) -> Result<()> {
    let runner_args = &cmd.args;
    cmd!(sh, "cargo build -p php-testsuite").run()?;
    cmd!(sh, "cargo test --test phpt -- {runner_args...}").run()?;
    Ok(())
}

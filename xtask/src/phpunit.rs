use anyhow::Result;
use xshell::{cmd, Shell};

use crate::flags::Phpunit;

pub(crate) fn run(sh: &Shell, cmd: &Phpunit) -> Result<()> {
    let runner_args = &cmd.args;
    cmd!(sh, "cargo build -p php-testsuite").run()?;
    cmd!(sh, "cargo test --test phpunit -- {runner_args...}").run()?;
    Ok(())
}

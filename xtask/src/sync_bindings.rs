use std::{env, path::PathBuf};

use anyhow::{anyhow, Result};
use xshell::{cmd, Shell};

pub(crate) fn run(sh: &Shell) -> Result<()> {
    let target_dir = env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| sh.current_dir().join("target"));
    let build_dir = target_dir.join("debug/build");

    cmd!(sh, "cargo clean -p ext-php-rs").run()?;
    cmd!(sh, "cargo build").run()?;
    let output = cmd!(
        sh,
        "find {build_dir} -type f -path '*/ext-php-rs-*/out/bindings.rs'"
    )
    .read()?;

    let matches: Vec<_> = output.lines().map(str::trim).collect();

    match matches.len() {
        0 => Err(anyhow!("Could not find bindings file.")),
        1 => {
            let src = *matches.first().unwrap();
            let dst = sh.current_dir().join("docsrs_bindings.rs");
            sh.copy_file(src, dst)?;
            Ok(())
        }
        _ => Err(anyhow!("Found multiple bindings files: {:#?}", matches)),
    }
}

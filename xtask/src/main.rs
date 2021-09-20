use std::path::{Path, PathBuf};

use anyhow::Result;
use xshell::Shell;

mod coverage;
mod flags;
mod phpt;
mod phpunit;
mod rebuild_self;
mod sync_bindings;

use flags::{Xtask, XtaskCmd};

fn main() -> Result<()> {
    let cli = Xtask::from_env_or_exit();
    let sh = Shell::new()?;
    sh.change_dir(project_root());

    match &cli.subcommand {
        XtaskCmd::Coverage(cmd) => coverage::run(&sh, cmd),
        XtaskCmd::Phpunit(cmd) => phpunit::run(&sh, cmd),
        XtaskCmd::Phpt(cmd) => phpt::run(&sh, cmd),
        XtaskCmd::SyncBindings(_) => sync_bindings::run(&sh),
        XtaskCmd::RebuildSelf(_) => rebuild_self::run(&sh),
    }
}

fn project_root() -> PathBuf {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .unwrap_or_else(|_| std::env!("CARGO_MANIFEST_DIR").to_string());
    Path::new(&manifest_dir)
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

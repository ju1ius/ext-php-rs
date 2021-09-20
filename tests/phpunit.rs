use std::{
    env::{
        self,
        consts::{DLL_PREFIX, DLL_SUFFIX},
    },
    path::{Path, PathBuf},
    process::{exit, Command},
};

use anyhow::{Context, Result};

const TESTSUITE_PATH: &str = "crates/php-testsuite";
const DLL_NAME: &str = "extphprs_testsuite";

fn main() -> Result<()> {
    let phpunit_args: Vec<_> = env::args_os().skip(1).collect();

    let manifest_dir: PathBuf = env::var("CARGO_MANIFEST_DIR")
        .context("Could not read `CARGO_MANIFEST_DIR`.")?
        .into();
    let test_dir = manifest_dir.join(TESTSUITE_PATH);
    let target_dir = Path::new(std::env!("OUT_DIR")).ancestors().nth(3).unwrap();

    let dll_name = format!("{}{}{}", DLL_PREFIX, DLL_NAME, DLL_SUFFIX);
    let dll_path = target_dir.join(dll_name);

    let php_bin = env::var("PHP").unwrap_or("php".to_string());
    let php_unit = env::var("PHPUNIT")
        .map(PathBuf::from)
        .unwrap_or(test_dir.join("php/vendor/bin/phpunit"));
    let php_unit_cfg = test_dir.join("phpunit.xml");

    let status = Command::new(php_bin)
        .arg("-d")
        .arg(format!("extension={}", dll_path.to_string_lossy()))
        .arg(&php_unit)
        .arg("-c")
        .arg(&php_unit_cfg)
        .args(phpunit_args)
        .status()?;

    if !status.success() {
        exit(1);
    }
    Ok(())
}

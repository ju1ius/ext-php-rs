use std::{
    collections::HashSet,
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
    let manifest_dir: PathBuf = env::var("CARGO_MANIFEST_DIR")
        .context("Could not read `CARGO_MANIFEST_DIR`.")?
        .into();
    // The target profile directory is always the 3rd last part of the path:
    // <target_dir>/<profile>/build/integration-XXXXXX/out
    //              └ 3       └ 2   └ 1                └ 0
    let target_dir = Path::new(std::env!("OUT_DIR")).ancestors().nth(3).unwrap();
    let test_dir = manifest_dir.join(TESTSUITE_PATH);
    let tests_path = test_dir.join("php/tests");

    let runner_args = get_runner_args(tests_path.to_string_lossy().to_string());
    let dll_name = format!("{}{}{}", DLL_PREFIX, DLL_NAME, DLL_SUFFIX);
    let dll_path = target_dir.join(dll_name);

    let php_bin = env::var("PHP").unwrap_or("php".to_string());
    let runner_bin = test_dir.join("php/run-tests.php");

    let status = Command::new(php_bin)
        .arg("-n")
        .arg(runner_bin)
        .arg("-d")
        .arg(format!("extension={}", dll_path.to_string_lossy()))
        .args(runner_args)
        .status()?;

    if !status.success() {
        exit(1);
    }

    Ok(())
}

fn get_runner_args(default_path: String) -> Vec<String> {
    let mut args: Vec<_> = env::args().skip(1).collect();
    if args.len() == 0 {
        // No arguments given, return sane defaults
        return vec!["-n".to_string(), default_path];
    }
    // If a positional argument is given,
    // we must not insert the default test directory
    if !has_positional_args(&args) {
        args.push(default_path);
    }
    args
}

macro_rules! literal_set {
    ($($value: literal)+) => {
        {
            let mut hs = HashSet::new();
            $(hs.insert($value);)+
            hs
        }
    };
}

fn has_positional_args(args: &Vec<String>) -> bool {
    // we don't include `-j` because the runner only accepts `-jN`, not `-j N`
    let flags_accepting_args = literal_set! {
        "-l" "-r" "-w" "-a" "-W" "-c" "-d" "-M" "-p" "-s"
        "--temp-source" "--temp-target" "--set-timeout"
        "--context" "--show-slow" "--repeat"
    };

    let mut has_positional = false;
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        if arg.starts_with('-') {
            if flags_accepting_args.contains(arg.as_str()) {
                it.next();
            }
            continue;
        }
        has_positional = true;
        break;
    }

    has_positional
}

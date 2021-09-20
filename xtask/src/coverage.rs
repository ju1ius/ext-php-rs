use std::collections::HashMap;

use anyhow::{bail, Context, Result};
use xshell::{cmd, Shell};

use crate::flags::Coverage as Args;

pub(crate) fn run(sh: &Shell, args: &Args) -> Result<()> {
    let vars = get_llvm_cov_env(sh)?;
    let report_args = get_report_args(args)?;

    cmd!(sh, "cargo llvm-cov clean --workspace")
        .envs(&vars)
        .run()?;
    cmd!(sh, "cargo build --workspace --all-features --exclude xtask")
        .envs(&vars)
        .run()?;
    let result = cmd!(sh, "cargo test --workspace --all-features --exclude xtask")
        .envs(&vars)
        .run();
    cmd!(sh, "cargo llvm-cov report {report_args...}")
        .envs(&vars)
        .run()?;

    result.with_context(|| "Test suite failed.")
}

fn get_report_args(args: &Args) -> Result<Vec<String>> {
    let mut res = Vec::new();
    if let Some(fmt) = &args.format {
        match fmt.as_ref() {
            "html" | "lcov" | "json" | "text" => res.push(format!("--{}", fmt)),
            _ => bail!("Unknown report format: {}", fmt),
        }
    }
    if let Some(path) = &args.output {
        let path = path.to_string_lossy().into_owned();
        match args.format.as_deref() {
            Some("html") => res.extend(["--output-dir".to_string(), path]),
            _ => res.extend(["--output-path".to_string(), path]),
        };
    }
    if args.open {
        res.push("--open".to_string());
    }
    Ok(res)
}

fn get_llvm_cov_env(sh: &Shell) -> Result<HashMap<String, String>> {
    let output = cmd!(sh, "cargo llvm-cov show-env").quiet().read()?;
    let mut vars = output
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .fold(HashMap::new(), |mut acc, line| {
            let mut parts = line.splitn(2, '=');
            if let Some(key) = parts.next() {
                let value = parts.next().unwrap_or("").trim_matches('"');
                acc.insert(key.to_string(), value.to_string());
            }
            acc
        });
    // instruct bindgen to run layout tests
    vars.insert("EXT_PHP_RS_TEST".into(), "1".into());
    // Ensure that crates/php-testsuite builds to the correct target directory
    // to collect coverage artifacts
    vars.insert(
        "CARGO_TARGET_DIR".into(),
        vars.get("CARGO_LLVM_COV_TARGET_DIR").unwrap().clone(),
    );
    Ok(vars)
}

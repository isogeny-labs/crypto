use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn version(program: &str, args: &[&str]) -> String {
    match Command::new(program).args(args).output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            format!(
                "program={program}; success={}; stdout={}; stderr={}",
                output.status.success(),
                stdout.trim().replace('\n', " | "),
                stderr.trim().replace('\n', " | ")
            )
        }
        Err(error) => format!("program={program}; success=false; error={error}"),
    }
}

fn git(root: &Path, args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err(format!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn git_path(root: &Path, path: &str) -> Option<PathBuf> {
    let value = git(root, &["rev-parse", "--git-path", path]).ok()?;
    let path = PathBuf::from(value);
    Some(if path.is_absolute() {
        path
    } else {
        root.join(path)
    })
}

fn emit_git_rerun_paths(root: &Path) {
    for path in [git_path(root, "HEAD"), git_path(root, "index")]
        .into_iter()
        .flatten()
    {
        println!("cargo:rerun-if-changed={}", path.display());
    }
    if let Ok(reference) = git(root, &["symbolic-ref", "-q", "HEAD"]) {
        if let Some(path) = git_path(root, &reference) {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
    if let Some(path) = git_path(root, "packed-refs") {
        println!("cargo:rerun-if-changed={}", path.display());
    }
}

fn main() {
    let rustc = env::var("RUSTC").unwrap_or_else(|_| "rustc".to_owned());
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    println!(
        "cargo:rustc-env=HARNESS_BUILD_RUSTC={}",
        version(&rustc, &["--version", "--verbose"])
    );
    println!(
        "cargo:rustc-env=HARNESS_BUILD_CARGO={}",
        version(&cargo, &["--version", "--verbose"])
    );

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("manifest directory"));
    let repository_root = manifest_dir
        .join("../../..")
        .canonicalize()
        .expect("repository root");
    let git_head = git(&repository_root, &["rev-parse", "--verify", "HEAD"])
        .unwrap_or_else(|error| format!("invalid:{error}"));
    let git_status = git(
        &repository_root,
        &["status", "--porcelain=v1", "--untracked-files=all"],
    )
    .unwrap_or_else(|error| format!("git-error:{error}"));
    let git_dirty = !git_status.is_empty();
    println!("cargo:rustc-env=HARNESS_BUILD_GIT_HEAD={git_head}");
    println!("cargo:rustc-env=HARNESS_BUILD_GIT_DIRTY={git_dirty}");
    println!(
        "cargo:rustc-env=HARNESS_BUILD_GIT_STATUS={}",
        git_status.replace('\n', " | ")
    );
    emit_git_rerun_paths(&repository_root);
    println!("cargo:rerun-if-changed=build.rs");
}

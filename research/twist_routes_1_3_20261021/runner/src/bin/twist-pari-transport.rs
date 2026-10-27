//! Verification-process transport only; the PARI script is not modified.
#[path = "../../../../../src/hash/sha256.rs"]
mod source_sha256;
use serde_json::json;
use std::{
    error::Error,
    fs,
    os::unix::process::ExitStatusExt,
    path::Path,
    process::{Command, Stdio},
    time::{SystemTime, UNIX_EPOCH},
};
fn sha(p: impl AsRef<Path>) -> Result<String, Box<dyn Error>> {
    Ok(hex::encode(source_sha256::sha256(&fs::read(p)?)))
}
fn epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
fn main() -> Result<(), Box<dyn Error>> {
    let a = std::env::args().skip(1).collect::<Vec<_>>();
    if a.len() != 2 {
        return Err("usage: twist-pari-transport SCRIPT FRESH-RECEIPT-DIR".into());
    }
    let script = Path::new(&a[0]);
    let out = Path::new(&a[1]);
    if out.exists() {
        return Err("fresh receipt required".into());
    }
    fs::create_dir_all(out)?;
    let root = "/workspace/shared/twist-tools-20261023/cas/root/usr";
    let gp = format!("{root}/bin/gp");
    let stdout = out.join("pari.stdout");
    let stderr = out.join("pari.stderr");
    let args = vec![
        "--signal=TERM".to_owned(),
        "--kill-after=1s".into(),
        "300s".into(),
        gp.clone(),
        "-f".into(),
        "-q".into(),
        "-s".into(),
        "64000000".into(),
        script.display().to_string(),
    ];
    let start = epoch();
    let status = Command::new("/usr/bin/timeout")
        .args(&args)
        .env(
            "LD_LIBRARY_PATH",
            format!("{root}/lib/x86_64-linux-gnu:{root}/lib/x86_64-linux-gnu/singular"),
        )
        .stdout(Stdio::from(fs::File::create(&stdout)?))
        .stderr(Stdio::from(fs::File::create(&stderr)?))
        .status()?;
    let receipt = json!({"schema":"twist.PARI-receipt/v1","argv":std::iter::once("/usr/bin/timeout".to_owned()).chain(args).collect::<Vec<_>>(),
        "started_epoch_s":start,"finished_epoch_s":epoch(),"exit_code":status.code(),"signal":status.signal(),"script_sha256":sha(script)?,"gp_sha256":sha(gp)?,
        "stdout":stdout,"stderr":stderr,"stdout_sha256":sha(&stdout)?,"stderr_sha256":sha(&stderr)?,"initial_stack_bytes":64000000,"original_script_unchanged":true});
    fs::write(
        out.join("RECEIPT.json"),
        serde_json::to_string_pretty(&receipt)? + "\n",
    )?;
    if !status.success() {
        return Err("PARI process failed; preserve receipt".into());
    }
    Ok(())
}

use pencil_native_canonical_export::{
    adapter_hash, check_export, load, package, validate_protocol, write_exclusive,
};
use std::path::Path;
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1)
    }
}
fn run() -> Result<(), String> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() == 2 && a[1] == "adapter-hash" {
        println!("{}", adapter_hash());
        return Ok(());
    }
    if a.len() == 3 && a[1] == "validate-protocol" {
        let path = Path::new(&a[2]);
        let protocol = load(path)?;
        validate_protocol(&protocol, path.parent())?;
        println!(
            "{}",
            serde_json::json!({"schema":"pencil-native-protocol-validation/v1","passed":true,"freeze_status":protocol["freeze"]["status"],"launch_authorized":false,"document_modified":false})
        );
        return Ok(());
    }
    if a.len() == 5 && a[1] == "check-export" && a[3] == "--protocol" {
        let findings = check_export(&load(Path::new(&a[2]))?, &load(Path::new(&a[4]))?)?;
        println!(
            "{}",
            serde_json::json!({"findings":findings,"qualified":false})
        );
        return if findings.is_empty() {
            Ok(())
        } else {
            Err("canonical semantic findings".into())
        };
    }
    if a.len() != 4 || a[2] != "--out" {
        return Err(
            "usage: pencil-native-canonical-export REQUEST --out NEW_OUTPUT | adapter-hash".into(),
        );
    }
    let request_path = std::fs::canonicalize(&a[1]).map_err(|e| e.to_string())?;
    let mut request = load(&request_path)?;
    if request.get("_request_binding").is_some() {
        return Err("CLI request must not contain internal _request_binding".into());
    }
    request["_request_binding"] = serde_json::json!({"path":request_path,"sha256":pencil_native_canonical_export::digest(&request_path)?});
    let (e, f) = package(&request)?;
    write_exclusive(Path::new(&a[3]), &e)?;
    println!(
        "{}",
        serde_json::json!({"export":a[3],"canonical_findings":f,"qualified":false})
    );
    if f.is_empty() {
        Ok(())
    } else {
        Err("export retains incomplete-row findings".into())
    }
}

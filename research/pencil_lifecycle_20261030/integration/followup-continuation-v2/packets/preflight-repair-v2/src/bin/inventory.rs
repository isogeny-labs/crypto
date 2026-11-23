//! Static inventory adapter only. Actual-session verification remains a distinct typed API.
use pencil_preflight_binding_core::{
    binding::parse,
    canonical::sha256_hex,
    inventory::{self, Input},
};
use serde::Deserialize;
use std::{fs::OpenOptions, io::Read, path::Path};
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FilePin {
    path: String,
    sha256: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Route {
    route: String,
    plan: FilePin,
    spec: FilePin,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Artifact {
    name: String,
    file: FilePin,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    schema: String,
    stage: String,
    routes: Vec<Route>,
    artifacts: Vec<Artifact>,
}
fn read_once(path: &Path, cap: usize) -> Result<Vec<u8>, String> {
    use std::os::unix::fs::OpenOptionsExt;
    let f = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
        .open(path)
        .map_err(|e| e.to_string())?;
    let meta = f.metadata().map_err(|e| e.to_string())?;
    if !meta.is_file() || meta.len() > cap as u64 {
        return Err("not bounded regular file".into());
    }
    let mut out = vec![];
    f.take(cap as u64 + 1)
        .read_to_end(&mut out)
        .map_err(|e| e.to_string())?;
    if out.len() > cap {
        return Err("file grew beyond cap".into());
    }
    Ok(out)
}
fn pinned(pin: &FilePin, total: &mut usize) -> Result<Vec<u8>, String> {
    let b = read_once(Path::new(&pin.path), 32 * 1024 * 1024)?;
    if sha256_hex(&b) != pin.sha256 {
        return Err("file hash mismatch".into());
    }
    *total = total.checked_add(b.len()).ok_or("byte total overflow")?;
    if *total > 64 * 1024 * 1024 {
        return Err("total byte cap".into());
    }
    Ok(b)
}
fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: inventory REQUEST_FILE EXACT_SHA256".into());
    }
    let b = read_once(Path::new(&args[1]), 1024 * 1024)?;
    if sha256_hex(&b) != args[2] {
        return Err("request hash mismatch".into());
    }
    let req: Request =
        serde_json::from_value(parse(&b, 1024 * 1024)?).map_err(|e| e.to_string())?;
    if req.schema != "pencil.static_inventory_request/v1" || req.stage != "static_inventory_only" {
        return Err("not static inventory stage".into());
    }
    let mut total = 0;
    let mut routes = vec![];
    for r in &req.routes {
        routes.push((
            r.route.as_str(),
            pinned(&r.plan, &mut total)?,
            pinned(&r.spec, &mut total)?,
        ));
    }
    let mut arts = vec![];
    for a in &req.artifacts {
        arts.push((a.name.as_str(), pinned(&a.file, &mut total)?));
    }
    let inputs: Vec<_> = routes
        .iter()
        .map(|(n, p, s)| Input {
            route: n,
            plan: p,
            spec: s,
        })
        .collect();
    let artifacts: Vec<_> = arts.iter().map(|(n, b)| (*n, b.as_slice())).collect();
    let checked = inventory::check(&inputs, &artifacts)?;
    println!(
        "{}",
        serde_json::json!({"stage":checked.stage,"software_bindings_complete":false,"scientific_execution_admitted":false,"cohort_workloads":checked.workloads,"cohort_slots":checked.slots,"separate_gate_slots":18,"request_sha256":args[2],"missing_actual_bindings":"checked separately per actual session; no future identities fabricated"})
    );
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(2)
    }
}

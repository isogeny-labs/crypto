//! Bounded exact-byte file adapter. Never launches work or authenticates custody.
use super::{
    canonical::sha256_hex,
    diagnostic_binding_core::{parse, OwnerObservation},
    diagnostic_inventory::Input,
    diagnostic_preflight::{self, ActualSession, Report},
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, fs::OpenOptions, io::Read, path::Path};
const REQUEST_CAP: usize = 1024 * 1024;
const FILE_CAP: usize = 32 * 1024 * 1024;
const TOTAL_CAP: usize = 64 * 1024 * 1024;
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
struct Actual {
    route: String,
    host: FilePin,
    session: FilePin,
    preparation: FilePin,
    launch: FilePin,
    contexts: Vec<FilePin>,
    owner_observation: FilePin,
}
#[derive(Deserialize)]
#[serde(tag = "stage", deny_unknown_fields)]
enum Request {
    #[serde(rename = "static_inventory_only")]
    Static {
        schema: String,
        routes: Vec<Route>,
        artifacts: Vec<Artifact>,
    },
    #[serde(rename = "actual_session_subset_checked")]
    Actual {
        schema: String,
        routes: Vec<Route>,
        artifacts: Vec<Artifact>,
        actual: Box<Actual>,
    },
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct UnauthenticatedOwner {
    session_sha256: String,
    host_sha256: String,
    preparation_sha256: String,
    binary_sha256: String,
    runner_pid: u64,
    boot_id: String,
    time_namespace: String,
    time_namespace_offsets: String,
    origin_ns: u64,
}
#[derive(Serialize)]
pub struct FileReport {
    pub schema: &'static str,
    pub request_sha256: String,
    pub loaded_evidence_bytes: usize,
    pub owner_authentication_verified: bool,
    pub external_owner_verification_required: bool,
    pub result: Report,
}
#[derive(Default)]
struct Loader {
    total: usize,
    paths: BTreeSet<String>,
    identities: BTreeSet<(u64, u64)>,
}
impl Loader {
    #[cfg(unix)]
    fn read(&mut self, path: &str, cap: usize) -> Result<Vec<u8>, String> {
        use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
        if !self.paths.insert(path.to_owned()) {
            return Err("duplicate evidence path".into());
        }
        let f = OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
            .open(Path::new(path))
            .map_err(|e| format!("open evidence: {e}"))?;
        let m = f.metadata().map_err(|e| e.to_string())?;
        if !m.is_file() || m.len() > cap as u64 {
            return Err("not bounded regular file".into());
        }
        if !self.identities.insert((m.dev(), m.ino())) {
            return Err("duplicate evidence file identity".into());
        }
        let remaining = TOTAL_CAP
            .checked_sub(self.total)
            .ok_or("aggregate byte cap")?;
        let limit = cap.min(remaining);
        let mut b = Vec::new();
        f.take(limit as u64 + 1)
            .read_to_end(&mut b)
            .map_err(|e| e.to_string())?;
        if b.len() > limit {
            return Err("file or aggregate byte cap".into());
        }
        self.total += b.len();
        Ok(b)
    }
    #[cfg(not(unix))]
    fn read(&mut self, _path: &str, _cap: usize) -> Result<Vec<u8>, String> {
        Err("bounded no-follow adapter requires Unix".into())
    }
    fn pin(&mut self, p: &FilePin, cap: usize) -> Result<Vec<u8>, String> {
        let b = self.read(&p.path, cap)?;
        if sha256_hex(&b) != p.sha256 {
            return Err("evidence hash mismatch".into());
        }
        Ok(b)
    }
}
/// Read-only software validation. Self-supplied owner JSON is never custody authentication.
pub fn check_file(request_path: &str, request_sha256: &str) -> Result<FileReport, String> {
    let mut loader = Loader::default();
    let raw = loader.read(request_path, REQUEST_CAP)?;
    if sha256_hex(&raw) != request_sha256 {
        return Err("request hash mismatch".into());
    }
    let request: Request = serde_json::from_value(parse(&raw, REQUEST_CAP)?)
        .map_err(|e| format!("request schema: {e}"))?;
    let (schema, routes, artifacts, actual) = match request {
        Request::Static {
            schema,
            routes,
            artifacts,
        } => (schema, routes, artifacts, None),
        Request::Actual {
            schema,
            routes,
            artifacts,
            actual,
        } => (schema, routes, artifacts, Some(actual)),
    };
    if schema != "ecbench.diagnostic_preflight_request/v1" {
        return Err("unsupported request schema".into());
    }
    if routes.len() != 9 || artifacts.len() != 4 {
        return Err("exact nine routes/four artifacts required".into());
    }
    let mut names = BTreeSet::new();
    let mut loaded = Vec::new();
    for r in routes {
        if !names.insert(r.route.clone()) {
            return Err("duplicate route".into());
        }
        loaded.push((
            r.route,
            loader.pin(&r.plan, FILE_CAP)?,
            loader.pin(&r.spec, FILE_CAP)?,
        ));
    }
    let mut names = BTreeSet::new();
    let mut arts = Vec::new();
    for a in artifacts {
        if !names.insert(a.name.clone()) {
            return Err("duplicate artifact".into());
        }
        arts.push((a.name, loader.pin(&a.file, FILE_CAP)?));
    }
    let inputs: Vec<_> = loaded
        .iter()
        .map(|(route, plan, spec)| Input { route, plan, spec })
        .collect();
    let artrefs: Vec<_> = arts
        .iter()
        .map(|(n, b)| (n.as_str(), b.as_slice()))
        .collect();
    let result = if let Some(a) = actual {
        // Validates immutable inventory before processing session metadata; no workloads constructed.
        diagnostic_preflight::check_inventory(&inputs, &artrefs)?;
        let (_, plan, spec) = loaded
            .iter()
            .find(|(r, _, _)| *r == a.route)
            .ok_or("actual route absent")?;
        if a.contexts.is_empty() || a.contexts.len() > 22680 {
            return Err("actual context count bound".into());
        }
        let host = loader.pin(&a.host, 4 * 1024 * 1024)?;
        let session = loader.pin(&a.session, 4 * 1024 * 1024)?;
        let preparation = loader.pin(&a.preparation, 65536)?;
        let launch = loader.pin(&a.launch, 65536)?;
        let owner_bytes = loader.pin(&a.owner_observation, 65536)?;
        let o: UnauthenticatedOwner = serde_json::from_value(parse(&owner_bytes, 65536)?)
            .map_err(|e| format!("owner comparison schema: {e}"))?;
        let owner = OwnerObservation {
            session_sha256: o.session_sha256,
            host_sha256: o.host_sha256,
            preparation_sha256: o.preparation_sha256,
            binary_sha256: o.binary_sha256,
            runner_pid: o.runner_pid,
            boot_id: o.boot_id,
            time_namespace: o.time_namespace,
            time_namespace_offsets: o.time_namespace_offsets,
            origin_ns: o.origin_ns,
        };
        let mut contexts = Vec::new();
        for c in a.contexts {
            contexts.push(loader.pin(&c, 65536)?);
        }
        diagnostic_preflight::check_actual_session(
            &inputs,
            &artrefs,
            ActualSession {
                route: &a.route,
                plan,
                spec,
                host: &host,
                session: &session,
                preparation: &preparation,
                launch: &launch,
                actual_contexts: &contexts,
                owner_observation: &owner,
            },
        )?
    } else {
        diagnostic_preflight::check_inventory(&inputs, &artrefs)?
    };
    Ok(FileReport {
        schema: "ecbench.diagnostic_preflight_file_report/v1",
        request_sha256: request_sha256.into(),
        loaded_evidence_bytes: loader.total,
        owner_authentication_verified: false,
        external_owner_verification_required: true,
        result,
    })
}
/// The dispatch arm passes arguments AFTER `diagnostic-preflight`; no other command accepted.
pub fn cli(args: &[String]) -> Result<String, String> {
    if args.len() != 4 || args[0] != "--request" || args[2] != "--request-sha256" {
        return Err(
            "usage: ecbench diagnostic-preflight --request PATH --request-sha256 HEX".into(),
        );
    }
    serde_json::to_string(&check_file(&args[1], &args[3])?).map_err(|e| e.to_string())
}
#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Temp(std::path::PathBuf);
    impl Temp {
        fn new() -> Self {
            let p = std::env::temp_dir().join(format!(
                "preflight-adapter-test-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::create_dir(&p).unwrap();
            Self(p)
        }
        fn put(&self, n: &str, b: &[u8]) -> String {
            let p = self.0.join(n);
            std::fs::write(&p, b).unwrap();
            p.to_str().unwrap().into()
        }
    }
    impl Drop for Temp {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
    #[test]
    fn exact_byte_pin_and_no_duplicate_alias() {
        let t = Temp::new();
        let p = t.put("a", b"abc");
        let mut l = Loader::default();
        assert_eq!(
            l.pin(
                &FilePin {
                    path: p.clone(),
                    sha256: sha256_hex(b"abc")
                },
                3
            )
            .unwrap(),
            b"abc"
        );
        assert_eq!(l.read(&p, 3).unwrap_err(), "duplicate evidence path");
        let alias = t.0.join("hardlink");
        std::fs::hard_link(&p, &alias).unwrap();
        assert_eq!(
            l.read(alias.to_str().unwrap(), 3).unwrap_err(),
            "duplicate evidence file identity"
        );
        assert_eq!(
            Loader::default()
                .pin(
                    &FilePin {
                        path: p,
                        sha256: sha256_hex(b"abcd")
                    },
                    4
                )
                .unwrap_err(),
            "evidence hash mismatch"
        );
    }
    #[test]
    fn bounded_regular_nofollow_and_aggregate() {
        let t = Temp::new();
        let p = t.put("a", b"abc");
        assert_eq!(
            Loader::default().read(&p, 2).unwrap_err(),
            "not bounded regular file"
        );
        let alias = t.0.join("symlink");
        std::os::unix::fs::symlink(&p, &alias).unwrap();
        assert!(Loader::default().read(alias.to_str().unwrap(), 3).is_err());
        assert!(Loader::default().read(t.0.to_str().unwrap(), 100).is_err());
        let mut l = Loader {
            total: TOTAL_CAP - 2,
            ..Loader::default()
        };
        assert_eq!(l.read(&p, 3).unwrap_err(), "file or aggregate byte cap");
    }
    #[test]
    fn strict_request_stage_and_hash() {
        let t = Temp::new();
        let base = serde_json::json!({"schema":"ecbench.diagnostic_preflight_request/v1","stage":"static_inventory_only","routes":[],"artifacts":[]});
        let b = serde_json::to_vec(&base).unwrap();
        let p = t.put("request", &b);
        assert_eq!(
            check_file(&p, &"0".repeat(64)).err().unwrap(),
            "request hash mismatch"
        );
        assert_eq!(
            check_file(&p, &sha256_hex(&b)).err().unwrap(),
            "exact nine routes/four artifacts required"
        );
        for (k, v) in [
            ("actual", serde_json::json!({})),
            ("admit", serde_json::json!(true)),
            ("stage", serde_json::json!("unsupported")),
        ] {
            let mut x = base.clone();
            x[k] = v;
            let b = serde_json::to_vec(&x).unwrap();
            let p = t.put("bad", &b);
            assert!(check_file(&p, &sha256_hex(&b))
                .err()
                .unwrap()
                .starts_with("request schema:"));
        }
        let b =
            b"{\"stage\":\"static_inventory_only\",\"stage\":\"actual_session_subset_checked\"}";
        let p = t.put("duplicate", b);
        assert!(check_file(&p, &sha256_hex(b)).is_err());
        assert!(cli(&[]).is_err());
    }
}

//! Independent expected-identity comparison for fresh owner artifact custody.
//! No full journal closure, mathematical acceptance or allocated-worker inference.
use crate::{contract::*, io::error, model::State};
use serde_json::{json, Value};
use std::os::unix::fs::OpenOptionsExt;
use std::{io::Read, path::Path};
fn bounded(path: &Path, cap: usize) -> Result<Vec<u8>, String> {
    let f = std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
        .open(path)
        .map_err(error)?;
    if !f.metadata().map_err(error)?.is_file() {
        return Err("owner artifact not regular".into());
    }
    let mut b = Vec::new();
    f.take(cap as u64 + 1).read_to_end(&mut b).map_err(error)?;
    if b.len() > cap {
        return Err("owner artifact cap".into());
    }
    Ok(b)
}
pub fn check(s: &State, pod_uid: &str, dir: &Path) -> Result<Value, String> {
    let admission = s
        .request
        .fresh_owner
        .as_ref()
        .ok_or("owner admission missing")?;
    let transport = s
        .request
        .diagnostic_transport
        .as_ref()
        .ok_or("diagnostic transport missing")?;
    let image = s
        .request
        .build
        .image_uri
        .rsplit_once("@sha256:")
        .ok_or("fresh image digest")?
        .1;
    let job = s
        .benchmark_uid
        .as_deref()
        .ok_or("expected Job UID absent")?;
    let manifest = s
        .benchmark_manifest
        .as_ref()
        .ok_or("immutable fresh manifest missing")?;
    let env = manifest["spec"]["template"]["spec"]["containers"][0]["env"]
        .as_array()
        .ok_or("fresh manifest env missing")?;
    let values: Vec<_> = env
        .iter()
        .filter(|v| v["name"] == "ISOLAB_OWNER_STOP_UTC")
        .collect();
    if values.len() != 1 {
        return Err("exact fresh cutoff missing".into());
    }
    let stop = values[0]["value"]
        .as_str()
        .ok_or("cutoff string")?
        .parse::<u64>()
        .map_err(error)?;
    if stop == 0 || stop > admission.stop_utc_seconds || stop > s.request.deadline_unix_seconds {
        return Err("immutable cutoff beyond admission".into());
    }
    let mut records = Vec::new();
    for phase in ["run", "replay"] {
        let raw = bounded(&dir.join(format!("owner-{phase}-config.json")), 65536)?;
        let Unique(config) = serde_json::from_slice(&raw).map_err(error)?;
        let expected = json!({"schema":"isolab.fresh-owner-config/v1","phase":phase,"request_sha256":s.request_sha256,"launch_sha256":transport.fresh.sha256,"launch":"/opt/isolab/diagnostic-input/fresh-launch.json","pod_uid":pod_uid,"job_uid":job,"image_digest":image,"binary_sha256":s.request.build.ecbench_sha256,"binary":"/usr/local/bin/ecbench","argv":if phase=="run"{json!(["run","--spec","/work/results/input-spec.json","--out","/work/results/session","--cpus","auto","--diagnostic-launch","/opt/isolab/diagnostic-input/fresh-launch.json"])}else{json!(["verify","--dir","/work/results/session","--replay-all","--exit-code","--out","/work/results/audit.json"])},"spec":"/work/results/input-spec.json","spec_sha256":transport.fresh.spec_bytes_sha256,"session":"/work/results/session","receipt":format!("/work/results/owner-{phase}.json"),"stop_utc_seconds":stop});
        if config != expected {
            return Err(format!(
                "{phase}: config differs from controller expected identities/argv"
            ));
        }
        let receipt_raw = bounded(&dir.join(format!("owner-{phase}.json")), 262144)?;
        let Unique(receipt) = serde_json::from_slice(&receipt_raw).map_err(error)?;
        let checker_raw = bounded(&dir.join(format!("owner-{phase}-check.json")), 65536)?;
        let Unique(checker) = serde_json::from_slice(&checker_raw).map_err(error)?;
        let receipt_hash = digest(&receipt_raw);
        let config_hash = digest(&raw);
        if receipt["schema"] != "isolab.fresh-owner-observation/v1"
            || receipt["binding"] != expected
            || receipt["config_sha256"] != config_hash
            || receipt["exit_code"] != 0
            || receipt["reap_observed"] != true
            || receipt["findings"] != json!([])
            || receipt.get("total_active_worker_ns") != Some(&Value::Null)
            || receipt.get("accepted_external_closure") != Some(&Value::Null)
            || checker["schema"] != "isolab.fresh-owner-check/v1"
            || checker["phase"] != phase
            || checker["receipt_sha256"] != receipt_hash
            || checker["config_sha256"] != config_hash
            || checker["identity_and_observation_checked"] != true
            || checker.get("accepted_external_closure") != Some(&Value::Null)
        {
            return Err(format!(
                "{phase}: incomplete or inconsistent observation/checker binding"
            ));
        }
        if phase == "run" {
            for (name, key, cap) in [
                ("session-preparation.json", "preparation_sha256", 262144),
                ("resource-journal.json", "journal_sha256", 8 * 1024 * 1024),
            ] {
                if receipt["session_binding"][key]
                    != digest(&bounded(&dir.join("session").join(name), cap)?)
                {
                    return Err("run journal/preparation custody mismatch".into());
                }
            }
        } else if !receipt["session_binding"].is_null() {
            return Err("replay fabricated closure".into());
        }
        records
            .push(json!({"phase":phase,"receipt_sha256":receipt_hash,"config_sha256":config_hash}));
    }
    let Unique(build) =
        serde_json::from_slice(&bounded(&dir.join("build-info.json"), 65536)?).map_err(error)?;
    if build["fresh_owner_sha256"] != admission.binary_sha256 {
        return Err("owner executable build binding mismatch".into());
    }
    Ok(
        json!({"schema":"isolab.fresh-owner-controller-check/v1","request_sha256":s.request_sha256,"pod_uid":pod_uid,"job_uid":job,"records":records,"scope":"controller-expected identities and retained bytes; native full journal closure remains separate","accepted_external_closure":null,"total_active_worker_ns":null}),
    )
}
// Reject duplicate keys recursively before interpreting untrusted metadata.
struct Unique(Value);
impl<'de> serde::Deserialize<'de> for Unique {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct V;
        impl<'de> serde::de::Visitor<'de> for V {
            type Value = Unique;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "JSON without duplicate keys")
            }
            fn visit_bool<E: serde::de::Error>(self, v: bool) -> Result<Unique, E> {
                Ok(Unique(json!(v)))
            }
            fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<Unique, E> {
                Ok(Unique(json!(v)))
            }
            fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Unique, E> {
                Ok(Unique(json!(v)))
            }
            fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<Unique, E> {
                Ok(Unique(json!(v)))
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Unique, E> {
                Ok(Unique(json!(v)))
            }
            fn visit_unit<E: serde::de::Error>(self) -> Result<Unique, E> {
                Ok(Unique(Value::Null))
            }
            fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut a: A) -> Result<Unique, A::Error> {
                let mut out = Vec::new();
                while let Some(Unique(v)) = a.next_element()? {
                    out.push(v);
                }
                Ok(Unique(Value::Array(out)))
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(self, mut a: A) -> Result<Unique, A::Error> {
                let mut out = serde_json::Map::new();
                while let Some(k) = a.next_key::<String>()? {
                    if out.contains_key(&k) {
                        return Err(serde::de::Error::custom("duplicate JSON key"));
                    }
                    let Unique(v) = a.next_value()?;
                    out.insert(k, v);
                }
                Ok(Unique(Value::Object(out)))
            }
        }
        d.deserialize_any(V)
    }
}

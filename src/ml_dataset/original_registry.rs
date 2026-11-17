//! Immutable receipts for originals, committed before an original-worker token.
//! A missing terminal registry never authorizes reconstruction of original roles.
use super::{storage, types::Artifact};
use serde::{Deserialize, Serialize};
use std::{fs::OpenOptions, io::Read, path::Path};

pub(super) const PATH: &str = "worker-originals.json";
pub(super) const ROLE: &str = "verified-worker-original-registry";
const SCHEMA: &str = "crypto.ml-dataset.worker-original-registry/v1";
const CAP: usize = 8192;
const ORIGINALS: [(&str, &str); 3] = [
    ("public.json", "public-instance"),
    ("witness.json", "restricted-known-answer"),
    ("factor-base.json", "factor-base"),
];
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Checkpoint {
    schema: String,
    artifacts: [Artifact; 3],
}
fn validate(root: &Path, checkpoint: &Checkpoint) -> Result<(), String> {
    if checkpoint.schema != SCHEMA { return Err("original registry schema mismatch".into()); }
    for (artifact, (path, role)) in checkpoint.artifacts.iter().zip(ORIGINALS) {
        if artifact.path != path || artifact.role != role || artifact.encoding != "json" {
            return Err("original registry path/role/encoding mismatch".into());
        }
        crate::store::index::validate_sha256(&artifact.sha256)?;
    }
    storage::verify_manifest(root, &checkpoint.artifacts)
}
/// These descriptors must be the actual immutable storage receipts returned by
/// original verification/writes; do not build descriptors by hashing names later.
pub(super) fn publish(root: &Path, artifacts: [&Artifact; 3]) -> Result<Artifact, String> {
    let checkpoint = Checkpoint { schema: SCHEMA.into(), artifacts: artifacts.map(Clone::clone) };
    validate(root, &checkpoint)?;
    storage::write_json(root, PATH, ROLE, &checkpoint)
}
fn load(root: &Path) -> Result<Option<(Checkpoint, Artifact)>, String> {
    let path = root.join(PATH);
    match std::fs::symlink_metadata(&path) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(e.to_string()),
        Ok(_) => {},
    }
    let path = storage::contained_path(root, PATH, false)?;
    let mut options = OpenOptions::new(); options.read(true);
    #[cfg(unix)] { use std::os::unix::fs::OpenOptionsExt; options.custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK); }
    let file = options.open(&path).map_err(|e| e.to_string())?;
    let meta = file.metadata().map_err(|e| e.to_string())?;
    if !meta.is_file() || meta.len() > CAP as u64 { return Err("original registry must be a bounded regular file".into()); }
    let mut raw = Vec::new(); raw.try_reserve_exact(CAP + 1).map_err(|_| "original registry allocation")?;
    file.take((CAP + 1) as u64).read_to_end(&mut raw).map_err(|e| e.to_string())?;
    if raw.len() > CAP { return Err("original registry exceeds cap".into()); }
    let checkpoint: Checkpoint = serde_json::from_slice(&raw).map_err(|e| format!("original registry: {e}"))?;
    validate(root, &checkpoint)?;
    // Only the checkpoint's own descriptor is made from this bounded read.
    // Original descriptors remain the producer's pre-token storage receipts.
    let receipt = Artifact { path: PATH.into(), role: ROLE.into(), encoding: "json".into(), bytes: raw.len() as u64, sha256: super::digest(&raw) };
    storage::check_file(&path, &receipt.sha256, receipt.bytes)?;
    Ok(Some((checkpoint, receipt)))
}
/// Preserve absent-checkpoint legacy behavior. With a terminal registry, demand
/// exact joins; never repair a contradictory or incomplete terminal registry.
pub(super) fn merge(root: &Path, mut registry: Vec<Artifact>, terminal_exists: bool) -> Result<Vec<Artifact>, String> {
    storage::verify_manifest(root, &registry)?;
    let Some((checkpoint, receipt)) = load(root)? else { return Ok(registry); };
    if terminal_exists {
        for expected in checkpoint.artifacts.iter().chain(std::iter::once(&receipt)) {
            let mut matching = registry.iter().filter(|a| a.path == expected.path);
            if matching.next() != Some(expected) || matching.next().is_some() {
                return Err("terminal registry contradicts original checkpoint".into());
            }
        }
    } else {
        if !registry.is_empty() { return Err("missing terminal registry has unexpected members".into()); }
        registry.extend(checkpoint.artifacts);
        registry.push(receipt);
    }
    Ok(registry)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Fixture { root: std::path::PathBuf, originals: [Artifact;3] }
    impl Fixture {
        fn new() -> Self {
            let root=std::env::temp_dir().join(format!("original-registry-{}-{}",std::process::id(),NEXT.fetch_add(1,Ordering::Relaxed)));
            std::fs::create_dir(&root).unwrap();
            let originals=ORIGINALS.map(|(path,role)|storage::write_json(&root,path,role,&json!({"fixture":path})).unwrap());
            Self{root,originals}
        }
        fn publish(&self)->Artifact {publish(&self.root,[&self.originals[0],&self.originals[1],&self.originals[2]]).unwrap()}
        fn checkpoint(&self)->Checkpoint{Checkpoint{schema:SCHEMA.into(),artifacts:self.originals.clone()}}
    }
    impl Drop for Fixture {fn drop(&mut self){let _=std::fs::remove_dir_all(&self.root);}}
    #[test] fn collector_recovers_actual_checkpoint_without_terminal_registry(){
        let f=Fixture::new();let receipt=f.publish();assert!(!f.root.join("worker-artifacts.json").exists());
        let found=super::super::collect_cell_artifacts(&f.root).unwrap();
        for expected in f.originals.iter().chain(std::iter::once(&receipt)){assert_eq!(found.iter().find(|a|a.path==expected.path),Some(expected));}
    }
    #[test] fn complete_registry_coexists_without_duplicate_or_replacement(){
        let f=Fixture::new();let receipt=f.publish();let mut final_registry=f.originals.to_vec();final_registry.push(receipt);
        storage::write_json(&f.root,"worker-artifacts.json","artifact-registry",&final_registry).unwrap();
        let found=super::super::collect_cell_artifacts(&f.root).unwrap();
        for expected in &final_registry{assert_eq!(found.iter().filter(|a|a.path==expected.path).count(),1);assert_eq!(found.iter().find(|a|a.path==expected.path),Some(expected));}
    }
    #[test] fn absent_checkpoint_keeps_legacy_partial_public_role(){
        let f=Fixture::new();let found=super::super::collect_cell_artifacts(&f.root).unwrap();
        assert_eq!(found.iter().find(|a|a.path=="public.json").unwrap().role,"unregistered-partial-artifact");
    }
    #[test] fn malformed_extra_duplicate_and_wrong_role_checkpoint_refuse(){
        for variant in 0..6 {let f=Fixture::new();let mut value=serde_json::to_value(f.checkpoint()).unwrap();match variant{
            0=>value["schema"]=json!("unknown"),1=>value["unexpected"]=json!(true),
            2=>value["artifacts"][1]=value["artifacts"][0].clone(),3=>value["artifacts"][0]["role"]=json!("factor-base"),
            4=>value["artifacts"][2]["encoding"]=json!("native"),_=>value["artifacts"].as_array_mut().unwrap().pop().map(|_|()).unwrap(),
        };storage::write_json(&f.root,PATH,ROLE,&value).unwrap();assert!(super::super::collect_cell_artifacts(&f.root).is_err());}
    }
    #[test] fn hash_mutation_and_checkpoint_capacity_refuse(){
        let f=Fixture::new();f.publish();std::fs::write(f.root.join("public.json"),b"changed").unwrap();assert!(merge(&f.root,vec![],false).is_err());
        let g=Fixture::new();std::fs::write(g.root.join(PATH),vec![b' ';CAP+1]).unwrap();assert!(merge(&g.root,vec![],false).is_err());
    }
    #[test] fn final_missing_duplicate_and_role_conflicts_refuse(){
        for variant in 0..4 {let f=Fixture::new();let receipt=f.publish();let mut entries=f.originals.to_vec();entries.push(receipt);match variant{
            0=>{entries.remove(0);},1=>entries.push(entries[0].clone()),2=>entries[0].role="unregistered-partial-artifact".into(),_=>{entries.pop();},
        };assert!(merge(&f.root,entries,true).is_err());}
    }
    #[test] fn path_reserved_and_receipt_publish_failure_is_returned(){
        assert!(super::super::worker_reserved_artifact_path(PATH));let f=Fixture::new();std::fs::create_dir(f.root.join(PATH)).unwrap();
        assert!(publish(&f.root,[&f.originals[0],&f.originals[1],&f.originals[2]]).is_err());
    }
}

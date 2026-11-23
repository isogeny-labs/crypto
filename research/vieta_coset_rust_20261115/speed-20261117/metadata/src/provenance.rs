//! Exact file/scope metadata verification against pre-call publication receipts.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
fn load(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
pub fn audit(root: &str) -> Value {
    let mut panels = vec![];
    for panel in ["preflight", "main"] {
        let receipt = load(format!("{root}/run/{panel}-publication.json"));
        let mut files = 0;
        for pin in receipt["files"].as_array().unwrap() {
            let b = fs::read(pin["path"].as_str().unwrap()).unwrap();
            assert_eq!(
                format!("{:x}", Sha256::digest(&b)),
                pin["sha256"].as_str().unwrap()
            );
            files += 1;
        }
        let (mut cells, mut inputs, mut configs) = (0, 0, vec![]);
        for entry in fs::read_dir(format!("{root}/configs/{panel}")).unwrap() {
            let path = entry.unwrap().path();
            let c = load(&path);
            let id = path.file_stem().unwrap().to_str().unwrap();
            let (mut n, mut ni) = (0, 0);
            for entry in fs::read_dir(format!("{root}/run/{panel}/{id}")).unwrap() {
                let path = entry.unwrap().path();
                if path.extension().is_none_or(|x| x != "json") {
                    continue;
                }
                let name = path.file_name().unwrap().to_str().unwrap();
                if !name.starts_with("cell-") && !name.starts_with("input-") {
                    continue;
                }
                let r = load(&path);
                for key in ["field", "representation", "m", "seed", "rounds", "warmups"] {
                    assert_eq!(r["scope"][key], c[key], "{id}:{name}:{key}");
                }
                if name.starts_with("cell-") {
                    n += 1;
                    cells += 1;
                } else {
                    ni += 1;
                    inputs += 1;
                }
            }
            assert_eq!(
                ni,
                c["rounds"].as_u64().unwrap() + c["warmups"].as_u64().unwrap()
            );
            assert_eq!(n, 6 * ni);
            configs.push(json!({"physical_id":id,"seed":c["seed"],"cells":n,"inputs":ni,"scope_matches_published_config":true}));
        }
        panels.push(json!({"panel":panel,"publication_revision":receipt["published_revision"],"publication_file_hashes_checked":files,"cells":cells,"inputs":inputs,"configs":configs}));
    }
    json!({"schema":"vieta-published-config-provenance/v1","panels":panels,"canonical_seed_table_pass":false,"scope":"Exact byte hashes and actual row scope metadata match pre-call Git publication/configs. This does not repair the frozen explicit_list canonical seed-table registration error or confer independent signed standing. No benchmark was rerun."})
}

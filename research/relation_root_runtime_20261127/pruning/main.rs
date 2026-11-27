//! Full fixed-six-input, multi-degree pruning/replay; no timings or group labels.
#[cfg(test)]
use certified_relations::Proof;
use certified_relations::{cheap, degree_space, relation_labels, Poly, Row};
#[path = "../next-source/prune.rs"]
mod prune;
use serde::Deserialize;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fs,
    io::Write,
    path::{Path, PathBuf},
};
type Result<T> = std::result::Result<T, String>;
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Case {
    id: String,
    path: String,
    sha256: String,
    prime: u64,
    n: usize,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    schema: String,
    cases: Vec<Case>,
}
fn write(root: &Path, name: &str, v: &impl serde::Serialize) -> Result<()> {
    let bytes = serde_json::to_vec_pretty(v).map_err(|e| e.to_string())?;
    if bytes.len() > 16 * 1024 * 1024 {
        return Err("output budget".into());
    }
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(root.join(name))
        .map_err(|e| e.to_string())?;
    f.write_all(&bytes).map_err(|e| e.to_string())?;
    f.sync_all().map_err(|e| e.to_string())
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 4 || !["scan", "replay"].contains(&args[1].as_str()) {
        return Err("joint-pruning scan|replay manifest output".into());
    }
    let manifest: Manifest =
        serde_json::from_slice(&fs::read(&args[2]).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    if manifest.schema != "fixed-last-star-inputs/v1" || manifest.cases.len() != 6 {
        return Err("fixed manifest schema/count".into());
    }
    let expected: BTreeSet<_> = [2147483647, 9007199254740881]
        .into_iter()
        .flat_map(|p| [3, 4, 6].into_iter().map(move |n| (p, n)))
        .collect();
    if manifest
        .cases
        .iter()
        .map(|c| (c.prime, c.n))
        .collect::<BTreeSet<_>>()
        != expected
    {
        return Err("fixed roster".into());
    }
    if manifest
        .cases
        .iter()
        .any(|c| c.id != format!("p{}-n{}", c.prime, c.n))
    {
        return Err("fixed case IDs".into());
    }
    let out = PathBuf::from(&args[3]);
    if args[1] == "scan" {
        fs::create_dir(&out).map_err(|e| e.to_string())?;
    }
    let mut records = vec![];
    for case in manifest.cases {
        let bytes = fs::read(&case.path).map_err(|e| e.to_string())?;
        if format!("{:x}", Sha256::digest(&bytes)) != case.sha256 {
            return Err("fixed input hash".into());
        }
        let system: certified_relations::System =
            serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
        if system.prime != case.prime || system.variables.len() != case.n {
            return Err("fixed input field/dimension".into());
        }
        let f = system.decode()?;
        let e: Vec<_> = cheap::consequences(&f, case.n, case.prime)
            .into_iter()
            .flat_map(|g| [g.cubic_difference, g.cubic_syzygy])
            .collect();
        let limits = prune::Limits::default();
        let name = format!("p{}-n{}.json", case.prime, case.n);
        let report: prune::Report = if args[1] == "scan" {
            let r = prune::prune(&f, case.n, case.prime, &e, &[2, 3, 4], None, limits)?;
            prune::replay(&f, case.n, case.prime, &e, &[2, 3, 4], None, &r, limits)?;
            write(&out, &name, &r)?;
            r
        } else {
            serde_json::from_slice(&fs::read(out.join(&name)).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?
        };
        prune::replay(
            &f,
            case.n,
            case.prime,
            &e,
            &[2, 3, 4],
            None,
            &report,
            limits,
        )?;
        let mut altered = report.clone();
        altered.selected_original_indices = if altered.selected_original_indices.is_empty() {
            vec![0]
        } else {
            vec![]
        };
        if prune::replay(
            &f,
            case.n,
            case.prime,
            &e,
            &[2, 3, 4],
            None,
            &altered,
            limits,
        )
        .is_ok()
        {
            return Err("summary canary accepted".into());
        }
        let mut altered = report.clone();
        altered.trials[0].accepted = !altered.trials[0].accepted;
        if prune::replay(
            &f,
            case.n,
            case.prime,
            &e,
            &[2, 3, 4],
            None,
            &altered,
            limits,
        )
        .is_ok()
        {
            return Err("decision canary accepted".into());
        }
        records.push(json!({"case":case.id,"prime":case.prime,"n":case.n,"input_sha256":case.sha256,"before":e.len(),"after":report.selected_original_indices.len(),"selected":report.selected_original_indices,"caps":[2,3,4],"status":report.status,"trial_count":report.trials.len(),"all_bounded_derivations_and_memberships_replayed":true,"label_audit":"not applicable: fixed polynomial fixtures have no curve/group-row metadata","canaries_rejected":2}));
    }
    let summary = json!({"schema":"joint-pruning-fixed-matrix/v1","cases":records,"case_count":6,"solving_degree":null,"regularity":null,"timing_claim":false,"15x_admitted":false});
    if args[1] == "scan" {
        write(&out, "summary.json", &summary)?;
    } else {
        let stored: serde_json::Value =
            serde_json::from_slice(&fs::read(out.join("summary.json")).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        if stored != summary {
            return Err("stored summary mismatch".into());
        }
    }
    let mut roster: BTreeSet<String> = expected
        .iter()
        .map(|(p, n)| format!("p{p}-n{n}.json"))
        .collect();
    roster.insert("summary.json".into());
    let mut actual = BTreeSet::new();
    for entry in fs::read_dir(&out).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        if !entry.file_type().map_err(|e| e.to_string())?.is_file() {
            return Err("nonregular output".into());
        }
        actual.insert(entry.file_name().to_str().ok_or("output name")?.to_string());
    }
    if actual != roster {
        return Err("output roster mismatch".into());
    }
    println!("{summary}");
    Ok(())
}

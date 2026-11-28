//! Full fixed-six-input, multi-degree pruning/replay; no timings or group labels.
#[cfg(test)]
use certified_relations::Proof;
use certified_relations::{cheap, degree_space, relation_labels, Poly, Row};
#[path = "../next-source/prune_v2.rs"]
mod prune;
use serde::Deserialize;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fs,
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
struct Bounded<T> {
    inner: T,
    left: usize,
}
impl<T: std::io::Read> std::io::Read for Bounded<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        if self.left == 0 {
            let mut probe = [0];
            return if self.inner.read(&mut probe)? == 0 {
                Ok(0)
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "plaintext budget",
                ))
            };
        }
        let count = buf.len().min(self.left);
        let read = self.inner.read(&mut buf[..count])?;
        self.left -= read;
        Ok(read)
    }
}
impl<T: std::io::Write> std::io::Write for Bounded<T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if buf.len() > self.left {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "plaintext budget",
            ));
        }
        let written = self.inner.write(buf)?;
        self.left -= written;
        Ok(written)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}
#[test]
fn budget_reader_rejects_a_hidden_suffix_instead_of_faking_eof() {
    let bad = Bounded {
        inner: std::io::Cursor::new(b"{}  x"),
        left: 2,
    };
    assert!(serde_json::from_reader::<_, serde_json::Value>(bad).is_err());
    let good = Bounded {
        inner: std::io::Cursor::new(b"{}"),
        left: 2,
    };
    assert_eq!(
        serde_json::from_reader::<_, serde_json::Value>(good).unwrap(),
        json!({})
    );
}
fn write(root: &Path, name: &str, value: &impl serde::Serialize) -> Result<()> {
    let file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(root.join(name))
        .map_err(|e| e.to_string())?;
    let mut encoder = flate2::write::GzEncoder::new(file, flate2::Compression::fast());
    serde_json::to_writer(
        Bounded {
            inner: &mut encoder,
            left: 512 * 1024 * 1024,
        },
        value,
    )
    .map_err(|e| e.to_string())?;
    encoder
        .finish()
        .map_err(|e| e.to_string())?
        .sync_all()
        .map_err(|e| e.to_string())
}
fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
    let file = fs::File::open(path).map_err(|e| e.to_string())?;
    if file.metadata().map_err(|e| e.to_string())?.len() > 128 * 1024 * 1024 {
        return Err("compressed input budget".into());
    }
    let decoder = flate2::read::GzDecoder::new(file);
    serde_json::from_reader(Bounded {
        inner: decoder,
        left: 512 * 1024 * 1024,
    })
    .map_err(|e| e.to_string())
}
#[cfg(test)]
#[path = "../next-source/prune.rs"]
mod full_control;
#[test]
fn short_circuit_matches_full_cap_control_with_and_without_scalar_labels() {
    let f = vec![
        Poly::from([(vec![2], 1)]),
        Poly::from([(vec![2], 1), (vec![1], 1)]),
    ];
    let g = Row {
        poly: Poly::from([(vec![1], 1)]),
        proof: Proof::from([((1, vec![0]), 1), ((0, vec![0]), 100)]),
    };
    let e = vec![g.clone(), g];
    let scalar = relation_labels::LabelSystem {
        scalar_prime: 37,
        columns: 1,
        rows: vec![vec![1], vec![1]],
        a: vec![0, 0],
        b: vec![0, 1],
    };
    for caps in [&[1usize, 2][..], &[2usize][..]] {
        for labels in [None, Some(&scalar)] {
            let old = full_control::prune(
                &f,
                1,
                101,
                &e,
                caps,
                labels,
                full_control::Limits::default(),
            )
            .unwrap();
            let new = prune::prune(&f, 1, 101, &e, caps, labels, prune::Limits::default()).unwrap();
            assert_eq!(old.selected_original_indices, new.selected_original_indices);
            assert_eq!(
                old.trials
                    .iter()
                    .map(|x| (x.removed, x.accepted))
                    .collect::<Vec<_>>(),
                new.trials
                    .iter()
                    .map(|x| (x.removed, x.accepted))
                    .collect::<Vec<_>>()
            );
            prune::replay(&f, 1, 101, &e, caps, labels, &new, prune::Limits::default()).unwrap();
        }
    }
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
        let name = format!("p{}-n{}.json.gz", case.prime, case.n);
        let report: prune::Report = if args[1] == "scan" {
            let r = prune::prune(&f, case.n, case.prime, &e, &[2, 3, 4], None, limits)?;
            prune::replay(&f, case.n, case.prime, &e, &[2, 3, 4], None, &r, limits)?;
            write(&out, &name, &r)?;
            r
        } else {
            read_json(&out.join(&name))?
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
        write(&out, "summary.json.gz", &summary)?;
    } else {
        let stored: serde_json::Value = read_json(&out.join("summary.json.gz"))?;
        if stored != summary {
            return Err("stored summary mismatch".into());
        }
    }
    let mut roster: BTreeSet<String> = expected
        .iter()
        .map(|(p, n)| format!("p{p}-n{n}.json.gz"))
        .collect();
    roster.insert("summary.json.gz".into());
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

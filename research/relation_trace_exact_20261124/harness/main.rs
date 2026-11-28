//! Fixed-input correctness qualification and replay; never a performance producer.
use certified_relations::{
    cheap, degree_space as ds, relation_labels as labels, Poly, Proof, Row, System,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};
type Result<T> = std::result::Result<T, String>;
fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Case {
    id: String,
    path: String,
    sha256: String,
    prime: u64,
    n: usize,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    schema: String,
    cases: Vec<Case>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Wire {
    polynomial: Vec<ds::CoefficientTerm>,
    membership: Vec<ds::MembershipTerm>,
}
impl Wire {
    fn from(row: &Row) -> Self {
        Self {
            polynomial: row
                .poly
                .iter()
                .map(|(e, c)| ds::CoefficientTerm {
                    exponents: e.clone(),
                    coefficient: *c,
                })
                .collect(),
            membership: row
                .proof
                .iter()
                .map(|((i, m), c)| ds::MembershipTerm {
                    input_index: *i,
                    multiplier: m.clone(),
                    coefficient: *c,
                })
                .collect(),
        }
    }
    fn row(&self) -> Result<Row> {
        let mut poly = Poly::new();
        for t in &self.polynomial {
            if poly.insert(t.exponents.clone(), t.coefficient).is_some() {
                return Err("duplicate polynomial term".into());
            }
        }
        let mut proof = Proof::new();
        for t in &self.membership {
            if proof
                .insert((t.input_index, t.multiplier.clone()), t.coefficient)
                .is_some()
            {
                return Err("duplicate witness term".into());
            }
        }
        Ok(Row { poly, proof })
    }
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Record {
    schema: String,
    case: String,
    input_sha256: String,
    degree: usize,
    presentation: String,
    extra: Vec<Wire>,
    raw: ds::Snapshot,
    closed: ds::Snapshot,
    raw_rank_gain: Option<usize>,
    closed_rank_gain: Option<usize>,
    f4_solving_degree: Option<usize>,
    intrinsic_regularity: Option<usize>,
}
#[derive(Clone)]
struct Presentation {
    name: String,
    extra: Vec<Row>,
}
fn presentations(f: &[Poly], n: usize, p: u64) -> Vec<Presentation> {
    let mut candidates = vec![];
    for pair in cheap::consequences(f, n, p) {
        candidates.push(pair.cubic_difference);
        candidates.push(pair.cubic_syzygy);
    }
    let mut all = vec![Presentation {
        name: "base".into(),
        extra: vec![],
    }];
    for (i, g) in candidates.iter().enumerate() {
        all.push(Presentation {
            name: format!("single-{i}"),
            extra: vec![g.clone()],
        });
    }
    all.push(Presentation {
        name: "all".into(),
        extra: candidates.clone(),
    });
    for i in 0..candidates.len() {
        all.push(Presentation {
            name: format!("minus-{i}"),
            extra: candidates
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, g)| g.clone())
                .collect(),
        });
    }
    all
}
fn input(case: &Case) -> Result<(System, Vec<Poly>)> {
    let bytes = fs::read(&case.path).map_err(|e| e.to_string())?;
    if digest(&bytes) != case.sha256 {
        return Err("original input hash mismatch".into());
    }
    let system: System = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    if system.prime != case.prime || system.variables.len() != case.n {
        return Err("original input dimensions mismatch".into());
    }
    let f = system.decode()?;
    Ok((system, f))
}
fn manifest(path: &Path) -> Result<Manifest> {
    let m: Manifest = serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    let wanted: BTreeSet<_> = [2147483647, 9007199254740881]
        .into_iter()
        .flat_map(|p| [3usize, 4, 6].map(|n| (p, n)))
        .collect();
    let got: BTreeSet<_> = m.cases.iter().map(|c| (c.prime, c.n)).collect();
    if m.schema != "fixed-last-star-inputs/v1"
        || m.cases.len() != 6
        || got != wanted
        || m.cases
            .iter()
            .any(|c| c.id != format!("p{}-n{}", c.prime, c.n))
    {
        return Err("manifest must bind all six original prime/n cases".into());
    }
    Ok(m)
}
fn write_new(path: &Path, value: &impl Serialize, budget: &mut usize) -> Result<()> {
    let mut bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?;
    bytes.push(b'\n');
    if bytes.len() > *budget {
        return Err("total output-byte budget".into());
    }
    let mut f = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    f.write_all(&bytes).map_err(|e| e.to_string())?;
    *budget -= bytes.len();
    Ok(())
}
fn gain(b: Option<usize>, a: Option<usize>) -> Result<Option<usize>> {
    match (b, a) {
        (Some(x), Some(y)) => Ok(Some(y.checked_sub(x).ok_or("negative rank gain")?)),
        _ => Ok(None),
    }
}
fn verified(f: &[Poly], case: &Case, e: &[Row], s: &ds::Snapshot) -> Result<Option<usize>> {
    Ok(ds::replay(f, case.n, case.prime, e, s, ds::Limits::default())?.exact_space_rank)
}
fn check_record(
    r: &Record,
    case: &Case,
    f: &[Poly],
    presentation: &Presentation,
    baseline: &Record,
) -> Result<()> {
    if r.schema != "degree-space-record/v1"
        || r.case != case.id
        || r.input_sha256 != case.sha256
        || r.presentation != presentation.name
        || r.degree != baseline.degree
        || r.degree != r.raw.degree
        || r.degree != r.closed.degree
        || r.raw.mode != ds::Mode::Raw
        || r.closed.mode != ds::Mode::Closed
        || r.f4_solving_degree.is_some()
        || r.intrinsic_regularity.is_some()
    {
        return Err("record identity/degree/invariant mismatch".into());
    }
    let extra = r.extra.iter().map(Wire::row).collect::<Result<Vec<_>>>()?;
    if extra != presentation.extra {
        return Err("undeclared consequence family".into());
    }
    let raw = verified(f, case, &extra, &r.raw)?;
    let closed = verified(f, case, &extra, &r.closed)?;
    let br = verified(f, case, &[], &baseline.raw)?;
    let bc = verified(f, case, &[], &baseline.closed)?;
    if gain(br, raw)? != r.raw_rank_gain || gain(bc, closed)? != r.closed_rank_gain {
        return Err("rank gain mismatch".into());
    }
    Ok(())
}
fn name(case: &Case, d: usize, p: &str) -> String {
    format!("{}-d{d}-{p}.json", case.id)
}
fn scan_one(case: &Case, out: &Path, budget: &mut usize) -> Result<Vec<serde_json::Value>> {
    let (_system, f) = input(case)?;
    let presentations = presentations(&f, case.n, case.prime);
    let mut summaries = vec![];
    write_new(
        &out.join(format!("{}-plan.json", case.id)),
        &json!({"case":case.id,"input_sha256":case.sha256,"degrees":[2,3,4],"presentations":presentations.iter().map(|p|&p.name).collect::<Vec<_>>(),"selection":"all fixed cheap-consequence singles, all, and every leave-one-out; no reference basis"}),
        budget,
    )?;
    for d in [2, 3, 4] {
        let raw = ds::snapshot(
            &f,
            case.n,
            case.prime,
            d,
            &[],
            ds::Mode::Raw,
            ds::Limits::default(),
        )?;
        let closed = ds::snapshot(
            &f,
            case.n,
            case.prime,
            d,
            &[],
            ds::Mode::Closed,
            ds::Limits::default(),
        )?;
        let br = verified(&f, case, &[], &raw)?;
        let bc = verified(&f, case, &[], &closed)?;
        let base = Record {
            schema: "degree-space-record/v1".into(),
            case: case.id.clone(),
            input_sha256: case.sha256.clone(),
            degree: d,
            presentation: "base".into(),
            extra: vec![],
            raw,
            closed,
            raw_rank_gain: gain(br, br)?,
            closed_rank_gain: gain(bc, bc)?,
            f4_solving_degree: None,
            intrinsic_regularity: None,
        };
        check_record(&base, case, &f, &presentations[0], &base)?;
        write_new(&out.join(name(case, d, "base")), &base, budget)?;
        summaries.push(json!({"case":case.id,"degree":d,"presentation":"base","raw_rank":br,"closed_rank":bc,"raw_gain":base.raw_rank_gain,"closed_gain":base.closed_rank_gain,"raw_status":base.raw.status,"closed_status":base.closed.status}));
        for p in &presentations[1..] {
            let raw = ds::snapshot(
                &f,
                case.n,
                case.prime,
                d,
                &p.extra,
                ds::Mode::Raw,
                ds::Limits::default(),
            )?;
            let closed = ds::snapshot(
                &f,
                case.n,
                case.prime,
                d,
                &p.extra,
                ds::Mode::Closed,
                ds::Limits::default(),
            )?;
            let rr = verified(&f, case, &p.extra, &raw)?;
            let cr = verified(&f, case, &p.extra, &closed)?;
            let r = Record {
                schema: "degree-space-record/v1".into(),
                case: case.id.clone(),
                input_sha256: case.sha256.clone(),
                degree: d,
                presentation: p.name.clone(),
                extra: p.extra.iter().map(Wire::from).collect(),
                raw,
                closed,
                raw_rank_gain: gain(br, rr)?,
                closed_rank_gain: gain(bc, cr)?,
                f4_solving_degree: None,
                intrinsic_regularity: None,
            };
            check_record(&r, case, &f, p, &base)?;
            write_new(&out.join(name(case, d, &p.name)), &r, budget)?;
            summaries.push(json!({"case":case.id,"degree":d,"presentation":p.name,"raw_rank":rr,"closed_rank":cr,"raw_gain":r.raw_rank_gain,"closed_gain":r.closed_rank_gain,"raw_status":r.raw.status,"closed_status":r.closed.status}));
        }
    }
    Ok(summaries)
}
fn scan(manifest_path: &Path, out: &Path) -> Result<bool> {
    let m = manifest(manifest_path)?;
    fs::create_dir(out).map_err(|e| e.to_string())?;
    let mut budget = 120 * 1024 * 1024usize;
    let mut metadata_budget = 4 * 1024 * 1024usize;
    let mut records = vec![];
    let mut errors = vec![];
    for case in &m.cases {
        match scan_one(case, out, &mut budget) {
            Ok(rows) => records.extend(rows),
            Err(error) => {
                write_new(
                    &out.join(format!("{}-failure.json", case.id)),
                    &json!({"case":case.id,"error":error}),
                    &mut metadata_budget,
                )?;
                errors.push(json!({"case":case.id,"error":error}));
            }
        }
    }
    let censored = records
        .iter()
        .filter(|r| r["raw_rank"].is_null() || r["closed_rank"].is_null())
        .count();
    let positive_closed = records
        .iter()
        .filter(|r| r["closed_gain"].as_u64().is_some_and(|v| v > 0))
        .count();
    let raw_only = records
        .iter()
        .filter(|r| r["raw_gain"].as_u64().is_some_and(|v| v > 0) && r["closed_gain"] == 0)
        .count();
    let report = json!({"schema":"degree-space-scan/v1","cases_attempted":m.cases.len(),"case_errors":errors,"records":records,"censored_records":censored,"positive_closed_gain_records":positive_closed,"raw_only_gain_records":raw_only,"label_audit_for_these_polynomial_inputs":"not applicable: no curve/group-row metadata supplied","f4_solving_degree":null,"intrinsic_regularity":null,"performance_experiment":false,"original_15x_admitted":false});
    write_new(&out.join("summary.json"), &report, &mut metadata_budget)?;
    println!(
        "{}",
        json!({"records":records.len(),"case_errors":errors.len(),"censored":censored,"positive_closed":positive_closed,"raw_only":raw_only})
    );
    Ok(errors.is_empty() && censored == 0)
}
fn replay(manifest_path: &Path, out: &Path) -> Result<()> {
    let m = manifest(manifest_path)?;
    let mut count = 0;
    let mut rows = vec![];
    let mut expected = BTreeSet::from(["summary.json".to_string()]);
    for case in &m.cases {
        let (_, f) = input(case)?;
        let ps = presentations(&f, case.n, case.prime);
        let plan_name = format!("{}-plan.json", case.id);
        let plan: serde_json::Value =
            serde_json::from_slice(&fs::read(out.join(&plan_name)).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        let wanted = json!({"case":case.id,"input_sha256":case.sha256,"degrees":[2,3,4],"presentations":ps.iter().map(|p|&p.name).collect::<Vec<_>>(),"selection":"all fixed cheap-consequence singles, all, and every leave-one-out; no reference basis"});
        if plan != wanted {
            return Err("prospective case roster mismatch".into());
        }
        expected.insert(plan_name);
        for d in [2, 3, 4] {
            let base: Record = serde_json::from_slice(
                &fs::read(out.join(name(case, d, "base"))).map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
            if base.degree != d {
                return Err("baseline degree roster mismatch".into());
            }
            for p in &ps {
                let path = name(case, d, &p.name);
                let r: Record =
                    serde_json::from_slice(&fs::read(out.join(&path)).map_err(|e| e.to_string())?)
                        .map_err(|e| e.to_string())?;
                check_record(&r, case, &f, p, &base)?;
                let raw = verified(&f, case, &p.extra, &r.raw)?;
                let closed = verified(&f, case, &p.extra, &r.closed)?;
                rows.push(json!({"case":case.id,"degree":d,"presentation":p.name,"raw_rank":raw,"closed_rank":closed,"raw_gain":r.raw_rank_gain,"closed_gain":r.closed_rank_gain,"raw_status":r.raw.status,"closed_status":r.closed.status}));
                expected.insert(path);
                count += 1;
            }
        }
    }
    let actual: BTreeSet<_> = fs::read_dir(out)
        .map_err(|e| e.to_string())?
        .map(|p| {
            p.map_err(|e| e.to_string())
                .map(|p| p.file_name().to_string_lossy().into_owned())
        })
        .collect::<Result<_>>()?;
    if actual != expected {
        return Err("missing or unexpected artifact roster".into());
    }
    let censored = rows
        .iter()
        .filter(|r| r["raw_rank"].is_null() || r["closed_rank"].is_null())
        .count();
    let positive_closed = rows
        .iter()
        .filter(|r| r["closed_gain"].as_u64().is_some_and(|v| v > 0))
        .count();
    let raw_only = rows
        .iter()
        .filter(|r| r["raw_gain"].as_u64().is_some_and(|v| v > 0) && r["closed_gain"] == 0)
        .count();
    let wanted = json!({"schema":"degree-space-scan/v1","cases_attempted":m.cases.len(),"case_errors":[],"records":rows,"censored_records":censored,"positive_closed_gain_records":positive_closed,"raw_only_gain_records":raw_only,"label_audit_for_these_polynomial_inputs":"not applicable: no curve/group-row metadata supplied","f4_solving_degree":null,"intrinsic_regularity":null,"performance_experiment":false,"original_15x_admitted":false});
    let summary: serde_json::Value =
        serde_json::from_slice(&fs::read(out.join("summary.json")).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    if summary != wanted {
        return Err("derived summary or admission metadata mismatch".into());
    }
    println!(
        "{}",
        json!({"schema":"degree-space-replay/v1","records_replayed":count,"all_retained_basis_rows_and_derivations_checked":true,"all_spaces_exact":censored==0,"derived_summary_equal":true,"performance_experiment":false})
    );
    Ok(())
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct LabelRecord {
    schema: String,
    input_sha256: String,
    audit: labels::Audit,
    scope: String,
}
fn index(dir: &Path, target: &Path) -> Result<()> {
    let mut files = std::collections::BTreeMap::new();
    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let e = entry.map_err(|e| e.to_string())?;
        if !e.file_type().map_err(|e| e.to_string())?.is_file() {
            return Err("index needs regular files".into());
        }
        let bytes = fs::read(e.path()).map_err(|e| e.to_string())?;
        files.insert(
            e.file_name().to_string_lossy().into_owned(),
            json!({"bytes":bytes.len(),"sha256":digest(&bytes)}),
        );
    }
    let mut index_budget = usize::MAX;
    write_new(
        target,
        &json!({"schema":"degree-artifact-index/v1","files":files}),
        &mut index_budget,
    )
}
fn canary(mode: &str, source: &Path, dest: &Path) -> Result<()> {
    fs::create_dir(dest).map_err(|e| e.to_string())?;
    for entry in fs::read_dir(source).map_err(|e| e.to_string())? {
        let e = entry.map_err(|e| e.to_string())?;
        if !e.file_type().map_err(|e| e.to_string())?.is_file() {
            return Err("canary needs regular files".into());
        }
        fs::hard_link(e.path(), dest.join(e.file_name())).map_err(|e| e.to_string())?;
    }
    let name = if mode == "tamper-summary" {
        "summary.json"
    } else {
        "p2147483647-n3-d3-single-0.json"
    };
    let target = dest.join(name);
    if mode == "tamper-roster" {
        fs::remove_file(&target).map_err(|e| e.to_string())?;
    } else {
        let mut v: serde_json::Value =
            serde_json::from_slice(&fs::read(&target).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        match mode {
            "tamper-summary" => v["original_15x_admitted"] = json!(true),
            "tamper-inputhash" => v["input_sha256"] = json!("0".repeat(64)),
            "tamper-proof" => {
                let x = v["closed"]["basis"][0]["membership"][0]["coefficient"]
                    .as_u64()
                    .ok_or("missing mutation witness")?;
                v["closed"]["basis"][0]["membership"][0]["coefficient"] =
                    json!(if x == 1 { 2 } else { 1 });
            }
            "tamper-degree" => {
                let x = v["closed"]["basis"][0]["derivation"]["multiplier"][0]
                    .as_u64()
                    .ok_or("missing derivation")?;
                v["closed"]["basis"][0]["derivation"]["multiplier"][0] = json!(x + 4);
            }
            _ => return Err("unknown canary".into()),
        }
        // Replace only the canary inode; never truncate the hardlinked original.
        let temp = dest.join("replacement.tmp");
        let mut replacement_budget = usize::MAX;
        write_new(&temp, &v, &mut replacement_budget)?;
        fs::rename(temp, target).map_err(|e| e.to_string())?;
    }
    index(dest, &dest.with_extension("index.json"))?;
    Ok(())
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 4 {
        return Err("scan|replay|labels|labels-replay INPUT OUTPUT".into());
    }
    let input_path = PathBuf::from(&args[2]);
    let output = PathBuf::from(&args[3]);
    match args[1].as_str() {
        "index" => index(&input_path, &output)?,
        mode if mode.starts_with("tamper-") => canary(mode, &input_path, &output)?,
        "scan" => {
            if !scan(&input_path, &output)? {
                std::process::exit(3);
            }
        }
        "replay" => replay(&input_path, &output)?,
        "labels" => {
            let bytes = fs::read(&input_path).map_err(|e| e.to_string())?;
            let system: labels::LabelSystem =
                serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
            let a = labels::audit(&system, labels::Limits::default())?;
            let mut budget = 16 * 1024 * 1024;
            write_new(
                &output,
                &json!({"schema":"label-audit-record/v1","input_sha256":digest(&bytes),"audit":a,"scope":"abstract scalar equations only; no supplied curve or subgroup proof"}),
                &mut budget,
            )?;
        }
        "labels-replay" => {
            let bytes = fs::read(&input_path).map_err(|e| e.to_string())?;
            let system: labels::LabelSystem =
                serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
            let r: LabelRecord =
                serde_json::from_slice(&fs::read(output).map_err(|e| e.to_string())?)
                    .map_err(|e| e.to_string())?;
            if r.schema != "label-audit-record/v1"
                || r.input_sha256 != digest(&bytes)
                || r.scope != "abstract scalar equations only; no supplied curve or subgroup proof"
            {
                return Err("label record identity mismatch".into());
            }
            labels::replay(&system, &r.audit, labels::Limits::default())?;
            println!("label replay ok");
        }
        _ => return Err("unknown mode".into()),
    }
    Ok(())
}

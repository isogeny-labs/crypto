//! Separately preregistered fresh comparison. Original protocols stay unchanged.
use super::*;
use compact_speed_analysis::{paired_statistics, Statistics};
use plotters::prelude::*;
const SEEDS: [u64; 3] = [41011, 43013, 47017];
const NAMES: [&str; 3] = ["prior-fastest-streamed", "same-source-streamed", "frontier"];
const MODES: [&str; 3] = [
    "both_updates_fused",
    "both_updates_fused",
    "both_updates_frontier",
];
const CYCLE: [[usize; 3]; 6] = [
    [0, 1, 2],
    [1, 2, 0],
    [2, 0, 1],
    [2, 1, 0],
    [1, 0, 2],
    [0, 2, 1],
];
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    prior: PathBuf,
    matched: PathBuf,
    candidate: PathBuf,
    timed_streamed: PathBuf,
    timed_frontier: PathBuf,
    singular: PathBuf,
    msolve: PathBuf,
    runtime_manifest: PathBuf,
    cpu: String,
}
impl Config {
    fn routes(&self) -> [&Path; 3] {
        [&self.prior, &self.matched, &self.candidate]
    }
}
fn positive(v: &Value) -> Result<u64> {
    v.as_u64()
        .filter(|n| *n > 0)
        .ok_or_else(|| "positive cost required".into())
}
fn sum(values: &[u64]) -> Result<u64> {
    values.iter().try_fold(0u64, |s, v| {
        s.checked_add(*v).ok_or_else(|| "cost overflow".into())
    })
}
fn includes_one(s: &Statistics) -> bool {
    s.percentile_95[0] <= 1. && s.percentile_95[1] >= 1.
}
fn answer_guard(a: &Value, mode: &str) -> Result<()> {
    if a["kernel_mode"] != mode
        || a["completion_exports"] != 0
        || a["root_api"] != true
        || a["native_product_criterion"] != true
        || a["consumer"]["kind"] != "separate-rust-arithmetic"
    {
        return Err("wrong native mode/checking flags".into());
    }
    Ok(())
}
fn values(p: &Value) -> Result<[[u64; 2]; 3]> {
    if p["all_complete_bases_equal"] != true {
        return Err("panel incomplete/mismatched".into());
    }
    let rows = p["samples"]
        .as_array()
        .filter(|r| r.len() == 3)
        .ok_or("three native samples")?;
    let digest = rows[0]["basis_sha256"]
        .as_str()
        .filter(|s| s.len() == 64)
        .ok_or("basis digest")?;
    let common = sum(&[
        p["generation_ns"].as_u64().ok_or("generation")?,
        positive(&p["preparation_ns"])?,
        positive(&p["reference_ns"])?,
        positive(&p["equality_ns"])?,
    ])?;
    let mut result = [[0; 2]; 3];
    for (i, s) in rows.iter().enumerate() {
        if s["status"] != "complete" || s["pipeline"] != NAMES[i] || s["basis_sha256"] != digest {
            return Err("native sample identity/completeness".into());
        }
        answer_guard(&s["answer"], MODES[i])?;
        let work = positive(&s["work_ns"])?;
        let paid = positive(&s["paid_ns"])?;
        if paid != sum(&[work, common])? {
            return Err("paid sum mismatch".into());
        }
        result[i] = [work, paid];
    }
    Ok(result)
}
fn aa_result(pairs: &[Value]) -> Result<(Value, bool)> {
    if pairs.len() != 7 {
        return Err("exactly seven A/A pairs".into());
    }
    let mut ratios: [Vec<[f64; 2]>; 3] = std::array::from_fn(|_| Vec::new());
    for (j, pair) in pairs.iter().enumerate() {
        let order = if j % 2 == 0 { ["A", "B"] } else { ["B", "A"] };
        if pair["execution_order"] != json!(order) {
            return Err("A/A execution labels".into());
        }
        for label in ["A", "B"] {
            if pair[label]["order"] != json!([0, 1, 2]) {
                return Err("A/A route order".into());
            }
        }
        let a = values(&pair["A"])?;
        let b = values(&pair["B"])?;
        for i in 0..3 {
            ratios[i].push([
                a[i][0] as f64 / b[i][0] as f64,
                a[i][1] as f64 / b[i][1] as f64,
            ]);
        }
    }
    let mut all = true;
    let mut out = Vec::new();
    for i in 0..3 {
        let s = paired_statistics(&ratios[i])?;
        let admitted = s.iter().all(includes_one);
        all &= admitted;
        out.push(json!({"arm":NAMES[i],"pipeline_work":s[0],"full_paid":s[1],"admitted":admitted}));
    }
    Ok((json!(out), all))
}
fn main_result(blocks: &[Value]) -> Result<Value> {
    if blocks.len() != 21 {
        return Err("exactly21 counterbalanced blocks".into());
    }
    let mut ratios = Vec::new();
    let mut ceilings = Vec::new();
    for (i, p) in blocks.iter().enumerate() {
        if p["order"] != json!(CYCLE[i % 6]) {
            return Err("main block order".into());
        }
        let v = values(p)?;
        let common = v[2][1]
            .checked_sub(v[2][0])
            .filter(|n| *n > 0)
            .ok_or("common fee")?;
        ratios.push([
            v[0][0] as f64 / v[2][0] as f64,
            v[1][0] as f64 / v[2][0] as f64,
            v[0][1] as f64 / v[2][1] as f64,
            v[1][1] as f64 / v[2][1] as f64,
        ]);
        ceilings.push([
            v[0][1] as f64 / common as f64,
            v[1][1] as f64 / common as f64,
        ]);
    }
    let stats = paired_statistics(&ratios)?;
    let ceiling = paired_statistics(&ceilings)?;
    let rows:Vec<_>=(0..2).map(|i|json!({"control":NAMES[i],"pipeline_work":stats[i],"full_paid":stats[i+2],"free_candidate_paid_ceiling":ceiling[i]})).collect();
    Ok(
        json!({"comparisons":rows,"all_controls_and_scopes_increment":stats.iter().all(|s|s.adjusted_lower_1_over_720>1.),"all_controls_and_scopes_10x":stats.iter().all(|s|s.adjusted_lower_1_over_720>10.),"all_controls_and_scopes_15x":stats.iter().all(|s|s.adjusted_lower_1_over_720>15.),"standing":"unsigned fresh replication on specified seeds; no old-family admission"}),
    )
}
fn validate_fixture_scope(n: usize, seed: u64, qualification: bool) -> Result<()> {
    if (qualification && (n != 3 || seed != 7))
        || (!qualification && (n != 16 || !SEEDS.contains(&seed)))
    {
        return Err("fixed separate fresh/qualification roster".into());
    }
    Ok(())
}
fn fixture(out: &Path, n: usize, seed: u64, qualification: bool) -> Result<Value> {
    validate_fixture_scope(n, seed, qualification)?;
    fs::create_dir(out)?;
    let start = Instant::now();
    let f = if qualification {
        workload::generate(P, 3, 7)?
    } else {
        if n != 16 || !SEEDS.contains(&seed) {
            return Err("fresh roster fixed before generation".into());
        }
        #[cfg(feature = "scale-screen")]
        {
            let edges: Vec<_> = (0..n - 1).map(|i| (i, n - 1)).collect();
            workload::generate_scale_graph(P, n, seed, &edges)?
        }
        #[cfg(not(feature = "scale-screen"))]
        {
            return Err("explicit scale-screen feature required".into());
        }
    };
    let generation = ns(start).max(1);
    let start = Instant::now();
    if !workload::witness_check(&f.system, &f.witness) {
        return Err("public fixture witness mismatch".into());
    }
    let s = &f.system;
    let inputs = s.decode()?;
    let bytes = serde_json::to_vec(s)?;
    write_new(&out.join("system.json"), &bytes)?;
    save(&out.join("public-fixture.json"), &f)?;
    for algorithm in ["std", "slimgb"] {
        write_new(
            &out.join(format!("{algorithm}.sing")),
            singular_code(s, &inputs, algorithm).as_bytes(),
        )?;
    }
    write_new(
        &out.join("input.ms"),
        format!(
            "{}\n{}\n{}\n",
            s.variables.join(","),
            P,
            inputs
                .iter()
                .map(|f| expr(f, &s.variables))
                .collect::<Vec<_>>()
                .join(",\n")
        )
        .as_bytes(),
    )?;
    Ok(
        json!({"variables":n,"seed":seed,"input_sha256":hash(&bytes),"generation_ns":generation,"preparation_ns":ns(start).max(1),"qualification":qualification}),
    )
}
struct Panel<'a> {
    r: &'a Runner,
    c: &'a Config,
    input: &'a Path,
    item: &'a Value,
}
impl Panel<'_> {
    fn capture(&self, out: &Path, order: [usize; 3]) -> Result<Value> {
        fs::create_dir(out)?;
        let bytes = fs::read(self.input.join("system.json"))?;
        if hash(&bytes) != self.item["input_sha256"] {
            return Err("changed prepared input".into());
        }
        let s: System = serde_json::from_slice(&bytes)?;
        let start = Instant::now();
        let (basis, _) = self.r.references(self.input, out, &s, 0)?;
        let reference_ns = ns(start).max(1);
        let mut rows = vec![Value::Null; 3];
        for i in order {
            let start = Instant::now();
            let attempt = super::sparse_capture::fused_sample(
                self.r,
                self.c.routes()[i],
                self.input,
                &out.join(NAMES[i]),
                "compact-sparse",
                s.variables.len(),
                Some(&basis),
            );
            let work = ns(start).max(1);
            let mut v = match attempt {
                Ok(v) => v,
                Err(e) => {
                    save(
                        &out.join("failed-attempt.json"),
                        &json!({"arm":NAMES[i],"charged_failed_work_ns":work,"reason":e.to_string()}),
                    )?;
                    return Err(e);
                }
            };
            answer_guard(&v["answer"], MODES[i])?;
            v["pipeline"] = json!(NAMES[i]);
            v["work_ns"] = json!(work);
            rows[i] = v;
        }
        let start = Instant::now();
        let equal = rows
            .iter()
            .all(|s| s["basis_sha256"] == rows[0]["basis_sha256"]);
        let equality = ns(start).max(1);
        let generation = positive(&self.item["generation_ns"])?;
        let prep = positive(&self.item["preparation_ns"])?;
        let common = sum(&[generation, prep, reference_ns, equality])?;
        for v in &mut rows {
            v["paid_ns"] = json!(sum(&[positive(&v["work_ns"])?, common])?);
        }
        let panel = json!({"order":order,"input_sha256":self.item["input_sha256"],"generation_ns":generation,"preparation_ns":prep,"reference_ns":reference_ns,"equality_ns":equality,"all_complete_bases_equal":equal,"samples":rows});
        values(&panel)?;
        save(&out.join("panel.json"), &panel)?;
        Ok(panel)
    }
    fn diagnostics(&self, out: &Path) -> Result<Value> {
        fs::create_dir(out)?;
        let bytes = fs::read(self.input.join("system.json"))?;
        let s: System = serde_json::from_slice(&bytes)?;
        let start = Instant::now();
        let (basis, _) = self.r.references(self.input, out, &s, 0)?;
        let reference_ns = ns(start).max(1);
        let mut rows = Vec::new();
        for (name, exe, mode) in [
            ("streamed", &self.c.timed_streamed, MODES[1]),
            ("frontier", &self.c.timed_frontier, MODES[2]),
        ] {
            let start = Instant::now();
            let sample = super::sparse_capture::fused_sample(
                self.r,
                exe,
                self.input,
                &out.join(name),
                "compact-sparse",
                s.variables.len(),
                Some(&basis),
            )?;
            let parent = ns(start).max(1);
            answer_guard(&sample["answer"], mode)?;
            let phases = sample["answer"]["diagnostic_phase_ns"]
                .as_object()
                .filter(|m| m.len() == 4)
                .ok_or("four diagnostic phases")?;
            let mut total = 0u64;
            for name in [
                "input_parse",
                "construction_and_wire",
                "wire_write",
                "root_check",
            ] {
                total = sum(&[
                    total,
                    phases
                        .get(name)
                        .and_then(Value::as_u64)
                        .ok_or("phase cost")?,
                ])?;
            }
            let remainder = parent
                .checked_sub(total)
                .ok_or("diagnostic phases exceed parent")?;
            rows.push(json!({"mode":mode,"parent_ns":parent,"phase_ns":phases,"parent_other_ns":remainder,"reference_ns":reference_ns,"sample":sample}));
        }
        let result = json!({"diagnostics":rows,"scope":"separate instrumented calls; not paired statistics"});
        save(&out.join("diagnostics.json"), &result)?;
        Ok(result)
    }
}
fn pins(c: &Config, config: &Path) -> Result<Value> {
    let mut a = serde_json::Map::new();
    for (name, path) in [
        ("capture", std::env::current_exe()?),
        ("config", config.to_path_buf()),
        (
            "protocol",
            PathBuf::from(
                "research/relation_degree_trace_20261115/FRESH_NATIVE_COMPARISON_PROTOCOL.md",
            ),
        ),
        ("prior", c.prior.clone()),
        ("matched", c.matched.clone()),
        ("candidate", c.candidate.clone()),
        ("timed_streamed", c.timed_streamed.clone()),
        ("timed_frontier", c.timed_frontier.clone()),
        ("singular", c.singular.clone()),
        ("msolve", c.msolve.clone()),
        ("runtime_manifest", c.runtime_manifest.clone()),
    ] {
        a.insert(name.into(), json!({"path":path,"sha256":file_hash(&path)?}));
    }
    let manifest: Value = serde_json::from_slice(&fs::read(&c.runtime_manifest)?)?;
    for (i, row) in manifest["runtime"]
        .as_array()
        .filter(|r| !r.is_empty() && r.len() <= 64)
        .ok_or("bounded runtime manifest")?
        .iter()
        .enumerate()
    {
        let path = PathBuf::from(row["path"].as_str().ok_or("runtime path")?);
        let digest = file_hash(&path)?;
        if row["sha256"] != digest {
            return Err("runtime differs from frozen manifest".into());
        }
        a.insert(format!("runtime-{i}"), json!({"path":path,"sha256":digest}));
    }
    Ok(Value::Object(a))
}
fn unchanged(pins: &Value) -> Result<bool> {
    for p in pins.as_object().ok_or("pins")?.values() {
        if file_hash(Path::new(p["path"].as_str().ok_or("pin path")?))? != p["sha256"] {
            return Ok(false);
        }
    }
    Ok(true)
}
pub(super) fn run(out: &Path, config: &Path, mode: &str) -> Result<()> {
    let qualification = match mode {
        "qualification" => true,
        "fresh" => false,
        _ => return Err("qualification or fresh only".into()),
    };
    let c: Config = serde_json::from_slice(&fs::read(config)?)?;
    if c.cpu != "0" {
        return Err("fixedCPU0".into());
    }
    let identities = pins(&c, config)?;
    fs::create_dir(out)?;
    write_new(&out.join("config.json"), &fs::read(config)?)?;
    save(
        &out.join("pre-call.json"),
        &json!({"identities":identities,"qualification":qualification,"fresh_seeds":SEEDS,"reserved_original_seeds_untouched":true}),
    )?;
    let mut r = Runner {
        worker: c.candidate.clone(),
        singular: c.singular.clone(),
        msolve: c.msolve.clone(),
        cpu: c.cpu.clone(),
        worker_hash: file_hash(&c.candidate)?,
        global: Instant::now(),
        case: Instant::now(),
        global_limit: Duration::from_secs(600),
        case_limit: Duration::from_secs(180),
    };
    let roster: Vec<_> = if qualification {
        vec![(3, 7)]
    } else {
        SEEDS.into_iter().map(|s| (16, s)).collect()
    };
    let mut cases = Vec::new();
    for (n, seed) in roster {
        r.case = Instant::now();
        let id = if qualification {
            "qualification-n3-s7".to_string()
        } else {
            format!("last-n{n}-s{seed}")
        };
        let dir = out.join(&id);
        fs::create_dir(&dir)?;
        let mut case = json!({"id":id,"seed":seed,"variables":n,"warmup":null,"aa_pairs":[],"aa":null,"blocks":[],"result":null,"failure":null,"diagnostics":null});
        let attempt = (|| -> Result<()> {
            let input = dir.join("input");
            let item = fixture(&input, n, seed, qualification)?;
            case["input"] = item.clone();
            save(&input.join("manifest.json"), &item)?;
            let panel = Panel {
                r: &r,
                c: &c,
                input: &input,
                item: &item,
            };
            case["warmup"] = panel.capture(&dir.join("warmup"), [0, 1, 2])?;
            if !qualification {
                let mut pairs = Vec::new();
                for j in 0..7 {
                    let labels = if j % 2 == 0 { ["A", "B"] } else { ["B", "A"] };
                    let mut pair = json!({"execution_order":labels});
                    for label in labels {
                        pair[label] =
                            panel.capture(&dir.join(format!("aa-{j}-{label}")), [0, 1, 2])?;
                    }
                    pairs.push(pair);
                    case["aa_pairs"] = json!(pairs);
                }
                let (aa, admitted) = aa_result(&pairs)?;
                case["aa"] = aa;
                if admitted {
                    let mut blocks = Vec::new();
                    for j in 0..21 {
                        blocks.push(panel.capture(&dir.join(format!("main-{j}")), CYCLE[j % 6])?);
                        case["blocks"] = json!(blocks);
                    }
                    case["result"] = main_result(&blocks)?;
                }
            }
            case["diagnostics"] = panel.diagnostics(&dir.join("phase-diagnostics"))?;
            Ok(())
        })();
        if let Err(e) = attempt {
            case["failure"] = json!(e.to_string());
        }
        case["case_wall_ns"] = json!(ns(r.case).max(1));
        save(&dir.join("outcome.json"), &case)?;
        eprintln!("{id}: fresh matched outcome retained");
        cases.push(case);
    }
    let same = unchanged(&identities)?;
    let all = !qualification
        && cases.len() == 3
        && cases.iter().all(|c| {
            c["failure"].is_null() && c["result"]["all_controls_and_scopes_increment"] == true
        });
    save(
        &out.join("summary.json"),
        &json!({"schema":"fresh-matched-native/v1","qualification":qualification,"identities_unchanged":same,"cases":cases,"all_cases_increment":all,"scope":"separate fixed fresh replication; unsigned; original family holdouts untouched"}),
    )?;
    if !same {
        return Err("changed pinned file".into());
    }
    Ok(())
}
// Integer costs/flags are exact; decimal statistic roundtrips allow four ulps.
fn same_statistics(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => {
            if let (Some(x), Some(y)) = (x.as_u64(), y.as_u64()) {
                return x == y;
            }
            match (x.as_f64(), y.as_f64()) {
                (Some(x), Some(y)) => (x - y).abs() <= 4. * f64::EPSILON * x.abs().max(1.),
                _ => false,
            }
        }
        (Value::Array(x), Value::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(a, b)| same_statistics(a, b))
        }
        (Value::Object(x), Value::Object(y)) => {
            x.len() == y.len()
                && x.iter()
                    .all(|(k, v)| y.get(k).is_some_and(|w| same_statistics(v, w)))
        }
        _ => a == b,
    }
}
fn replay_call(dir: &Path, input: &[u8], answer: &Value, digest: &Value, n: usize) -> Result<()> {
    if !fs::read(dir.join("call.stderr"))?.is_empty() {
        return Err("saved native diagnostics".into());
    }
    let saved: Value = serde_json::from_slice(&fs::read(dir.join("call.stdout"))?)?;
    if saved != *answer {
        return Err("answer/file mismatch".into());
    }
    let wire = fs::read(dir.join("proof/witness.json"))?;
    let checked = relation_replay_checker::verify_json(input, &wire)?;
    let basis = native_basis(&json!(&checked.basis), n)?;
    if basis_hash(&basis) != *digest
        || serde_json::to_value(&checked.stats)? != answer["consumer"]["stats"]
    {
        return Err("replayed basis/stats mismatch".into());
    }
    Ok(())
}
fn replay_references(dir: &Path, s: &System, p: &Value) -> Result<()> {
    let reference: Value = serde_json::from_slice(&fs::read(dir.join("references.json"))?)?;
    let basis = native_basis(&reference["basis"], s.variables.len())?;
    if basis_hash(&basis) != reference["reference_basis_sha256"]
        || reference["reference_basis_sha256"] != p["samples"][0]["basis_sha256"]
    {
        return Err("reference basis mismatch".into());
    }
    let routes = reference["routes"]
        .as_array()
        .filter(|r| r.len() == 3)
        .ok_or("three reference routes")?;
    let mut total = 0;
    for (i, name) in ["std", "slimgb", "msolve"].iter().enumerate() {
        if routes[i]["route"] != *name {
            return Err("reference route omitted/reordered".into());
        }
        total = sum(&[total, positive(&routes[i]["whole_route_ns"])?])?;
        if !fs::read(dir.join(format!("setup-{name}.stderr")))?.is_empty() {
            return Err("reference diagnostics".into());
        }
        let parsed = if *name == "msolve" {
            msolve_basis(
                &fs::read_to_string(dir.join("msolve-basis.txt"))?,
                &s.variables,
            )?
        } else {
            singular_basis(
                &fs::read_to_string(dir.join(format!("setup-{name}.stdout")))?,
                s.variables.len(),
            )?
        };
        if parsed != basis {
            return Err("raw mature reference mismatch".into());
        }
    }
    if total > positive(&p["reference_ns"])? {
        return Err("reference phases exceed parent".into());
    }
    Ok(())
}
// Every completed retained panel is replayed, including partial failed cases.
fn replay_panel(raw: &Path, id: &str, label: &str, p: &Value) -> Result<usize> {
    values(p)?;
    let dir = raw.join(id).join(label);
    let saved: Value = serde_json::from_slice(&fs::read(dir.join("panel.json"))?)?;
    if saved != *p {
        return Err("panel summary/file mismatch".into());
    }
    let input = fs::read(raw.join(id).join("input/system.json"))?;
    if hash(&input) != p["input_sha256"] {
        return Err("panel input mismatch".into());
    }
    let s: System = serde_json::from_slice(&input)?;
    replay_references(&dir, &s, p)?;
    for (i, name) in NAMES.iter().enumerate() {
        replay_call(
            &dir.join(name),
            &input,
            &p["samples"][i]["answer"],
            &p["samples"][i]["basis_sha256"],
            s.variables.len(),
        )?;
    }
    Ok(3)
}
fn phase_total(p: &Value, parent: u64, other: u64) -> Result<()> {
    let phases = p
        .as_object()
        .filter(|r| r.len() == 4)
        .ok_or("four phase keys")?;
    let mut total = other;
    for name in [
        "input_parse",
        "construction_and_wire",
        "wire_write",
        "root_check",
    ] {
        total = sum(&[
            total,
            phases
                .get(name)
                .and_then(Value::as_u64)
                .ok_or("phase missing")?,
        ])?;
    }
    if total != parent {
        return Err("phase accounting mismatch".into());
    }
    Ok(())
}
fn replay_diagnostics(raw: &Path, id: &str, d: &Value) -> Result<String> {
    if d.is_null() {
        return Ok(String::new());
    }
    let dir = raw.join(id).join("phase-diagnostics");
    let saved: Value = serde_json::from_slice(&fs::read(dir.join("diagnostics.json"))?)?;
    if saved != *d {
        return Err("diagnostic file mismatch".into());
    }
    let input = fs::read(raw.join(id).join("input/system.json"))?;
    let s: System = serde_json::from_slice(&input)?;
    let rows = d["diagnostics"]
        .as_array()
        .filter(|a| a.len() == 2)
        .ok_or("two diagnostic modes")?;
    let mut md = String::new();
    for (i, name) in ["streamed", "frontier"].iter().enumerate() {
        let r = &rows[i];
        let a = &r["sample"]["answer"];
        answer_guard(a, MODES[i + 1])?;
        phase_total(
            &r["phase_ns"],
            positive(&r["parent_ns"])?,
            r["parent_other_ns"].as_u64().ok_or("phase remainder")?,
        )?;
        if a["diagnostic_phase_ns"] != r["phase_ns"] {
            return Err("phase payload mismatch".into());
        }
        replay_call(
            &dir.join(name),
            &input,
            a,
            &r["sample"]["basis_sha256"],
            s.variables.len(),
        )?;
        md += &format!("| {id} | {name} |");
        for key in [
            "input_parse",
            "construction_and_wire",
            "wire_write",
            "root_check",
        ] {
            md += &format!(
                " {:.6} |",
                r["phase_ns"][key].as_u64().unwrap() as f64 / 1e6
            );
        }
        md += &format!(
            " {:.6} |\n",
            r["parent_other_ns"].as_u64().unwrap() as f64 / 1e6
        );
    }
    Ok(md)
}
pub(super) fn report(raw: &Path, monitor: &Path, out: &Path, pcc: &str) -> Result<()> {
    let s: Value = serde_json::from_slice(&fs::read(raw.join("summary.json"))?)?;
    if s["schema"] != "fresh-matched-native/v1" || s["identities_unchanged"] != true {
        return Err("fresh summary schema/pins".into());
    }
    let pre: Value = serde_json::from_slice(&fs::read(raw.join("pre-call.json"))?)?;
    if !unchanged(&pre["identities"])? {
        return Err("current pins differ".into());
    }
    let qualification = s["qualification"] == true;
    let mut monitor_ok = qualification;
    if !qualification {
        let m: Value = serde_json::from_str(
            fs::read_to_string(monitor)?
                .lines()
                .next()
                .ok_or("monitor")?,
        )?;
        monitor_ok = m["contended_samples"] == 0 && m["exit_status"] == 0;
    }
    let cases = s["cases"]
        .as_array()
        .filter(|c| c.len() == if qualification { 1 } else { 3 })
        .ok_or("fixed case roster")?;
    let mut checked_answers = 0;
    let mut md=format!("# Fresh matched native comparison\n\n{pcc}. Qualification:{qualification}; monitor admission:{monitor_ok}. Three native routes use identical complete proof checking and identical common fees. Prior fastest is the primary comparison; matched same-source39 is the secondary control. Unsigned localL0; original family holdouts remain untouched. [pcc:unverified]\n\n| Case | Control | Work median | Paid median | Work adjusted lower | Paid adjusted lower | Paid/shared ceiling |\n|---|---|---:|---:|---:|---:|---:|\n");
    let mut points = Vec::new();
    let mut audited = Vec::new();
    let mut phase_md=String::from("\n## Separate phase diagnostics (milliseconds)\n\n| Case | Mode | Input parse | Construction/wire | Wire write | Root replay | Parent remainder |\n|---|---|---:|---:|---:|---:|---:|\n");
    for (i, c) in cases.iter().enumerate() {
        let id = c["id"].as_str().ok_or("case id")?;
        if c["seed"]
            != if qualification {
                json!(7)
            } else {
                json!(SEEDS[i])
            }
        {
            return Err("fresh seed/order changed".into());
        }
        let recorded: Value =
            serde_json::from_slice(&fs::read(raw.join(id).join("outcome.json"))?)?;
        if recorded != *c {
            return Err("case outcome/file mismatch".into());
        }
        if !c["input"].is_null() {
            let bytes = fs::read(raw.join(id).join("input/system.json"))?;
            let system: System = serde_json::from_slice(&bytes)?;
            if hash(&bytes) != c["input"]["input_sha256"]
                || system.variables.len() != if qualification { 3 } else { 16 }
            {
                return Err("fresh input scope mismatch".into());
            }
            let fixture: workload::Fixture =
                serde_json::from_slice(&fs::read(raw.join(id).join("input/public-fixture.json"))?)?;
            if serde_json::to_value(&fixture.system)? != serde_json::to_value(&system)?
                || !workload::witness_check(&system, &fixture.witness)
            {
                return Err("public fixture mismatch".into());
            }
        }
        let completed = c["blocks"].as_array().ok_or("completed blocks")?;
        for (j, p) in completed.iter().enumerate() {
            if p["order"] != json!(CYCLE[j % 6]) {
                return Err("partial block order".into());
            }
            checked_answers += replay_panel(raw, id, &format!("main-{j}"), p)?;
        }
        phase_md += &replay_diagnostics(raw, id, &c["diagnostics"])?;
        if !c["warmup"].is_null() {
            checked_answers += replay_panel(raw, id, "warmup", &c["warmup"])?;
        }
        let pairs = c["aa_pairs"].as_array().ok_or("AApairs")?;
        for (j, p) in pairs.iter().enumerate() {
            for label in ["A", "B"] {
                checked_answers += replay_panel(raw, id, &format!("aa-{j}-{label}"), &p[label])?;
            }
        }
        if !c["failure"].is_null() {
            if !c["aa"].is_null() {
                let (aa, _) = aa_result(pairs)?;
                if !same_statistics(&aa, &c["aa"]) {
                    return Err("failed-case AA mismatch".into());
                }
            }
            if !c["result"].is_null() {
                let result = main_result(completed)?;
                if !same_statistics(&result, &c["result"]) {
                    return Err("failed-case main mismatch".into());
                }
            }
        }
        if c["failure"].is_null() && !qualification {
            let (aa, admitted) = aa_result(pairs)?;
            if !same_statistics(&aa, &c["aa"]) {
                return Err("recomputedAA mismatch".into());
            }
            let blocks = c["blocks"].as_array().ok_or("blocks")?;
            if admitted {
                let result = main_result(blocks)?;
                if !same_statistics(&result, &c["result"]) {
                    return Err("recomputed main mismatch".into());
                }
                for (j, r) in result["comparisons"]
                    .as_array()
                    .ok_or("comparisons")?
                    .iter()
                    .enumerate()
                {
                    let w = &r["pipeline_work"];
                    let p = &r["full_paid"];
                    md += &format!(
                        "| {id} | {} | {:.6} | {:.6} | {:.6} | {:.6} | {:.6} |\n",
                        NAMES[j],
                        w["median_paired_ratio"].as_f64().unwrap(),
                        p["median_paired_ratio"].as_f64().unwrap(),
                        w["adjusted_lower_1_over_720"].as_f64().unwrap(),
                        p["adjusted_lower_1_over_720"].as_f64().unwrap(),
                        r["free_candidate_paid_ceiling"]["median_paired_ratio"]
                            .as_f64()
                            .unwrap()
                    );
                    points.push((i * 2 + j, p["median_paired_ratio"].as_f64().unwrap()));
                }
            } else {
                if !blocks.is_empty() || !c["result"].is_null() {
                    return Err("main after rejectedAA".into());
                }
                md += &format!("| {id} | AA rejected | — | — | — | — | — |\n");
            }
        } else if !c["failure"].is_null() {
            md += &format!("| {id} | failure retained | — | — | — | — | — |\n");
        }
        audited.push(json!({"id":id,"failure":c["failure"],"aa":c["aa"],"result":c["result"],"diagnostics":c["diagnostics"],"case_wall_ns":c["case_wall_ns"]}));
    }
    let all = !qualification
        && cases.iter().all(|c| {
            c["failure"].is_null() && c["result"]["all_controls_and_scopes_increment"] == true
        });
    if s["all_cases_increment"] != all {
        return Err("family aggregate mismatch".into());
    }
    md += &phase_md;
    md+=&format!("\n{checked_answers} captured uninstrumented answers replayed from original bytes/wires; all-case increment flag:{all}. Monitor-valid result requires:{monitor_ok}. A rejected/incomplete case is never pooled away. [pcc:unverified]\n\nGeneration/preparation and complete fresh3CAS references are fully charged to every route without amortization; native work already includes construction/full native checks/root replay/output consumption. The conditional ceiling assumes only candidate work becomes free while the same shared fee remains; it is not a global complexity bound. Instrumented phase calls and capture/report/statistic overhead are separate study costs and never mixed into these ratios. All raw calls/failures/AA/main/phase details remain. [pcc:unverified]\n\n![Paid ratios](ratios.svg){{width=90%}}\n");
    fs::create_dir(out)?;
    save(
        &out.join("audit.json"),
        &json!({"checked_uninstrumented_answers":checked_answers,"raw_wires_replayed":true,"paid_sums_orders_and_statistics_recomputed":true,"monitor_ok":monitor_ok,"conditional_all_cases_increment":all,"admitted_all_cases_increment":all&&monitor_ok,"cases":audited,"standing":"unsigned producer audit"}),
    )?;
    write_new(&out.join("report.md"), md.as_bytes())?;
    let path = out.join("ratios.svg");
    let root = SVGBackend::new(&path, (1000, 500)).into_drawing_area();
    root.fill(&WHITE)?;
    let upper = points.iter().map(|p| p.1).fold(1.2f64, f64::max) * 1.15;
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Fresh full-paid control / frontier ratios",
            ("sans-serif", 22),
        )
        .margin(20)
        .x_label_area_size(45)
        .y_label_area_size(70)
        .build_cartesian_2d(0usize..6, 0f64..upper)?;
    chart
        .configure_mesh()
        .x_desc("Fixed seed/control index; full mapping in report")
        .y_desc("Paired median, no pooling")
        .draw()?;
    chart.draw_series(LineSeries::new([(0, 1.), (6, 1.)], &BLACK))?;
    chart.draw_series(
        points
            .iter()
            .map(|(x, y)| Circle::new((*x, *y), 6, BLUE.filled())),
    )?;
    root.present()?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn panel(cost: u64, order: [usize; 3]) -> Value {
        let samples:Vec<_>=(0..3).map(|i|json!({"status":"complete","pipeline":NAMES[i],"basis_sha256":"a".repeat(64),"work_ns":cost,"paid_ns":cost+10,"answer":{"kernel_mode":MODES[i],"completion_exports":0,"root_api":true,"native_product_criterion":true,"consumer":{"kind":"separate-rust-arithmetic"}}})).collect();
        json!({"order":order,"generation_ns":1,"preparation_ns":2,"reference_ns":3,"equality_ns":4,"all_complete_bases_equal":true,"samples":samples})
    }
    #[test]
    fn original_holdouts_and_any_unplanned_seed_are_rejected_before_generation() {
        for seed in [19001, 23003, 29009, 7, 101] {
            assert!(validate_fixture_scope(16, seed, false).is_err());
        }
        for seed in SEEDS {
            assert!(validate_fixture_scope(16, seed, false).is_ok());
            assert!(validate_fixture_scope(8, seed, false).is_err());
            assert!(validate_fixture_scope(3, seed, true).is_err());
        }
        assert!(validate_fixture_scope(3, 7, true).is_ok());
    }
    #[test]
    fn statistics_tolerance_preserves_exact_flags_and_integer_costs() {
        assert!(same_statistics(
            &json!({"ratio":1.23456789,"flag":true}),
            &json!({"ratio":1.2345678900000002,"flag":true})
        ));
        assert!(!same_statistics(
            &json!({"flag":true}),
            &json!({"flag":false})
        ));
        assert!(!same_statistics(
            &json!(10000000000000000u64),
            &json!(10000000000000001u64)
        ));
        assert!(!same_statistics(&json!(1.2), &json!(1.21)));
    }
    #[test]
    fn diagnostic_phase_sum_rejects_missing_and_overlapping_costs() {
        let p = json!({"input_parse":1,"construction_and_wire":2,"wire_write":3,"root_check":4});
        assert!(phase_total(&p, 15, 5).is_ok());
        assert!(phase_total(&p, 14, 5).is_err());
        assert!(phase_total(&json!({"input_parse":1}), 1, 0).is_err());
    }
    #[test]
    fn common_costs_are_identical_and_overflow_is_rejected() {
        assert_eq!(values(&panel(20, [0, 1, 2])).unwrap(), [[20, 30]; 3]);
        assert!(sum(&[u64::MAX, 1]).is_err());
        let mut p = panel(20, [0, 1, 2]);
        p["samples"][1]["paid_ns"] = json!(29);
        assert!(values(&p).is_err());
    }
    #[test]
    fn every_complete_mode_and_digest_is_required() {
        for i in 0..3 {
            for key in ["status", "basis_sha256"] {
                let mut p = panel(20, [0, 1, 2]);
                p["samples"][i][key] = Value::Null;
                assert!(values(&p).is_err());
            }
            let mut p = panel(20, [0, 1, 2]);
            p["samples"][i]["answer"]["kernel_mode"] = json!("wrong");
            assert!(values(&p).is_err());
        }
    }
    #[test]
    fn aa_labels_and_all_six_gates_remain() {
        let pairs:Vec<_>=(0..7).map(|j|json!({"execution_order":if j%2==0{["A","B"]}else{["B","A"]},"A":panel(20,[0,1,2]),"B":panel(20,[0,1,2])})).collect();
        assert!(aa_result(&pairs).unwrap().1);
        let mut wrong = pairs.clone();
        wrong[1]["execution_order"] = json!(["A", "B"]);
        assert!(aa_result(&wrong).is_err());
        let bad: Vec<_> = pairs
            .into_iter()
            .map(|mut p| {
                p["A"] = panel(40, [0, 1, 2]);
                p
            })
            .collect();
        assert!(!aa_result(&bad).unwrap().1);
    }
    #[test]
    fn balanced_main_and_common_cost_ceiling_are_checked() {
        let blocks: Vec<_> = (0..21).map(|i| panel(20, CYCLE[i % 6])).collect();
        let r = main_result(&blocks).unwrap();
        assert_eq!(r["all_controls_and_scopes_increment"], false);
        assert_eq!(
            r["comparisons"][0]["free_candidate_paid_ceiling"]["median_paired_ratio"],
            3.
        );
        let mut wrong = blocks;
        wrong[3]["order"] = json!([0, 1, 2]);
        assert!(main_result(&wrong).is_err());
    }
}

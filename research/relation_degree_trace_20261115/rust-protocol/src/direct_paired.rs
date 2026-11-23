//! Prospective direct-construction comparison; original protocols remain unchanged.
use super::*;
use certified_relations::{cheap, verify, Row, Term};
use compact_speed_analysis::{paired_statistics, paired_statistics_25, Statistics};
use relation_ablations as a;
const NAMES: [&str; 4] = ["rank", "direct-all", "direct-prefix", "frontier"];
const ORDERS4: [[usize; 4]; 8] = [
    [0, 1, 2, 3],
    [1, 2, 3, 0],
    [2, 3, 0, 1],
    [3, 0, 1, 2],
    [3, 2, 1, 0],
    [0, 3, 2, 1],
    [1, 0, 3, 2],
    [2, 1, 0, 3],
];
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    f4: PathBuf,
    frontier: PathBuf,
    singular: PathBuf,
    msolve: PathBuf,
    isolation: PathBuf,
    protocol: PathBuf,
    cpu: String,
}
fn sum(xs: &[u64]) -> Result<u64> {
    xs.iter().try_fold(0u64, |a, b| {
        a.checked_add(*b).ok_or_else(|| "cost overflow".into())
    })
}
fn pos(v: &Value) -> Result<u64> {
    v.as_u64()
        .filter(|v| *v > 0)
        .ok_or_else(|| "positive cost required".into())
}
fn construct(inputs: &[Poly], n: usize, mode: usize) -> Result<(Vec<Row>, Value)> {
    if mode == 0 {
        return rank_relations::select(inputs, n, 4);
    }
    if mode == 3 {
        return Ok((vec![], json!({"method":"original frontier input"})));
    }
    if mode > 3 || inputs.len() < 4 {
        return Err("unsupported direct input/mode".into());
    }
    let source = if mode == 2 { &inputs[..4] } else { inputs };
    let pairs = cheap::consequences(source, n, P);
    if pairs.len() < 2 || (mode == 2 && pairs.len() != 2) {
        return Err("two exact recognized pairs required".into());
    }
    let rows: Vec<_> = pairs
        .iter()
        .take(2)
        .map(|p| p.cubic_syzygy.clone())
        .collect();
    if !rows.iter().all(|r| verify(r, inputs, P)) {
        return Err("direct original-input identity".into());
    }
    Ok((
        rows,
        json!({"source_equations":source.len(),"constructed_pairs":pairs.len(),"method":NAMES[mode]}),
    ))
}
fn frontier_guard(v: &Value) -> Result<()> {
    if v["kernel_mode"] != "both_updates_frontier"
        || v["completion_exports"] != 0
        || v["root_api"] != true
        || v["native_product_criterion"] != true
        || v["consumer"]["kind"] != "separate-rust-arithmetic"
    {
        return Err("frontier checks/mode changed".into());
    }
    Ok(())
}
fn values(p: &Value) -> Result<[[u64; 2]; 4]> {
    let samples = p["samples"]
        .as_array()
        .filter(|v| v.len() == 4)
        .ok_or("four samples")?;
    if p["all_complete_bases_equal"] != true || p["same_augmented_inputs"] != true {
        return Err("panel equality guard".into());
    }
    let common = sum(&[
        pos(&p["generation_ns"])?,
        pos(&p["preparation_ns"])?,
        pos(&p["reference_ns"])?,
        pos(&p["equality_ns"])?,
    ])?;
    let mut result = [[0; 2]; 4];
    for (i, s) in samples.iter().enumerate() {
        if s["name"] != NAMES[i]
            || s["status"] != "complete"
            || s["basis_sha256"] != samples[0]["basis_sha256"]
        {
            return Err("sample labels/bases".into());
        }
        let work = pos(&s["work_ns"])?;
        let paid = pos(&s["paid_ns"])?;
        if paid != sum(&[work, common])? {
            return Err("paid sum".into());
        }
        let phases = &s["phases"];
        let included = sum(&[
            phases["construction_ns"].as_u64().ok_or("construction")?,
            pos(&phases["preparation_ns"])?,
            pos(&phases["identity_ns"])?,
            pos(&phases["solve_ns"])?,
            phases["other_ns"].as_u64().ok_or("other")?,
        ])?;
        if included != work {
            return Err("work phase sum".into());
        }
        result[i] = [work, paid];
    }
    Ok(result)
}
fn aa(pairs: &[Value]) -> Result<(Value, bool)> {
    if pairs.len() != 7 {
        return Err("seven AA pairs".into());
    }
    let mut ratios = vec![];
    for (j, p) in pairs.iter().enumerate() {
        if p["execution_order"] != json!(if j % 2 == 0 { ["A", "B"] } else { ["B", "A"] })
            || p["A"]["order"] != json!([0, 1, 2, 3])
            || p["B"]["order"] != json!([0, 1, 2, 3])
        {
            return Err("AA order".into());
        }
        let a = values(&p["A"])?;
        let b = values(&p["B"])?;
        ratios.push(std::array::from_fn::<_, 8, _>(|k| {
            a[k / 2][k % 2] as f64 / b[k / 2][k % 2] as f64
        }));
    }
    let stats = paired_statistics(&ratios)?;
    let admitted = stats
        .iter()
        .all(|s| s.percentile_95[0] <= 1. && s.percentile_95[1] >= 1.);
    Ok((json!(stats), admitted))
}
fn main_stats(blocks: &[Value]) -> Result<Value> {
    if blocks.len() != 25 {
        return Err("25 main blocks".into());
    }
    let controls = [0, 1, 3];
    let mut ratios = vec![];
    for (j, p) in blocks.iter().enumerate() {
        if p["order"] != json!(ORDERS4[j % 8]) {
            return Err("main order".into());
        }
        let v = values(p)?;
        ratios.push(std::array::from_fn::<_, 6, _>(|k| {
            v[controls[k / 2]][k % 2] as f64 / v[2][k % 2] as f64
        }));
    }
    let stats: [Statistics; 6] = paired_statistics_25(&ratios)?;
    Ok(
        json!({"statistics":stats,"numeric_all_controls_increment":stats.iter().all(|s|s.adjusted_lower_1_over_720>1.),"numeric_15x":stats.iter().all(|s|s.adjusted_lower_1_over_720>15.)}),
    )
}
fn sample(
    r: &Runner,
    c: &Config,
    input: &Path,
    dir: &Path,
    system: &System,
    reference: &Basis,
    index: usize,
) -> Result<Value> {
    fs::create_dir(dir)?;
    let start = Instant::now();
    let n = system.variables.len();
    let inputs = system.decode()?;
    let phase = Instant::now();
    let (rows, selection) = construct(&inputs, n, index)?;
    let construction = ns(phase);
    let phase = Instant::now();
    let mut augmented = system.clone();
    for row in &rows {
        augmented.polynomials.push(
            row.poly
                .iter()
                .map(|(m, c)| Term {
                    exponents: m.clone(),
                    coefficient: *c as i64,
                })
                .collect(),
        );
    }
    let bytes = serde_json::to_vec(&augmented)?;
    let augmented_hash = hash(&bytes);
    write_new(&dir.join("system.json"), &bytes)?;
    save(
        &dir.join("identities.json"),
        &json!({"rows":rows.iter().map(a::wire).collect::<Vec<_>>(),"selection":selection}),
    )?;
    write_new(
        &dir.join("identity.sing"),
        a::identity_script(system, &inputs, &rows).as_bytes(),
    )?;
    let preparation = ns(phase).max(1);
    let phase = Instant::now();
    let (status, bytes, _) = r.process(
        &r.singular,
        &["-q".into(), dir.join("identity.sing").display().to_string()],
        None,
        &dir.join("identity"),
    )?;
    if !status.success()
        || std::str::from_utf8(&bytes)?.trim() != "IDENTITIES_OK"
        || !fs::read(dir.join("identity.stderr"))?.is_empty()
    {
        return Err("fresh Singular identity check".into());
    }
    let identity = ns(phase).max(1);
    let phase = Instant::now();
    let (answer, digest) = if index == 3 {
        let v = sparse_capture::fused_sample(
            r,
            &c.frontier,
            input,
            &dir.join("native"),
            "compact-sparse",
            n,
            Some(reference),
        )?;
        frontier_guard(&v["answer"])?;
        (v["answer"].clone(), v["basis_sha256"].clone())
    } else {
        r.sample(Arm::F4, dir, &dir.join("f4"), n, reference, &augmented_hash)?;
        let v: Value = serde_json::from_slice(&fs::read(dir.join("f4.call.json"))?)?;
        (v["worker_output"].clone(), v["basis_sha256"].clone())
    };
    let solve = ns(phase).max(1);
    let work = ns(start).max(1);
    let other = work
        .checked_sub(sum(&[construction, preparation, identity, solve])?)
        .ok_or("overlapping phases")?;
    let v = json!({"name":NAMES[index],"status":"complete","basis_sha256":digest,"augmented_sha256":augmented_hash,"answer":answer,"work_ns":work,"phases":{"construction_ns":construction,"preparation_ns":preparation,"identity_ns":identity,"solve_ns":solve,"other_ns":other}});
    save(&dir.join("sample.json"), &v)?;
    Ok(v)
}
fn panel(r: &Runner, c: &Config, out: &Path, n: usize, order: [usize; 4]) -> Result<Value> {
    fs::create_dir(out)?;
    let start = Instant::now();
    let fixture =
        workload::generate_graph(P, n, 7, &(0..n - 1).map(|i| (i, n - 1)).collect::<Vec<_>>())?;
    let generation = ns(start).max(1);
    let start = Instant::now();
    if !workload::witness_check(&fixture.system, &fixture.witness) {
        return Err("public fixture check".into());
    }
    let input = out.join("original");
    fs::create_dir(&input)?;
    a::prepare(&input, &fixture.system)?;
    save(&input.join("public-fixture.json"), &fixture)?;
    let preparation = ns(start).max(1);
    let start = Instant::now();
    let (reference, _) = r.references(&input, out, &fixture.system, 0)?;
    let reference_ns = ns(start).max(1);
    let mut samples = vec![Value::Null; 4];
    for i in order {
        let dir = out.join(NAMES[i]);
        let start = Instant::now();
        match sample(r, c, &input, &dir, &fixture.system, &reference, i) {
            Ok(v) => samples[i] = v,
            Err(e) => {
                save(
                    &out.join("failed-attempt.json"),
                    &json!({"arm":NAMES[i],"charged_failed_ns":ns(start),"reason":e.to_string()}),
                )?;
                return Err(e);
            }
        }
    }
    let start = Instant::now();
    let bases = samples
        .iter()
        .all(|s| s["basis_sha256"] == samples[0]["basis_sha256"]);
    let same = samples[..3]
        .iter()
        .all(|s| s["augmented_sha256"] == samples[0]["augmented_sha256"]);
    let equality = ns(start).max(1);
    let common = sum(&[generation, preparation, reference_ns, equality])?;
    for s in &mut samples {
        s["paid_ns"] = json!(sum(&[pos(&s["work_ns"])?, common])?);
    }
    let p = json!({"order":order,"generation_ns":generation,"preparation_ns":preparation,"reference_ns":reference_ns,"equality_ns":equality,"all_complete_bases_equal":bases,"same_augmented_inputs":same,"samples":samples});
    values(&p)?;
    save(&out.join("panel.json"), &p)?;
    Ok(p)
}
fn pins(c: &Config, config: &Path) -> Result<Value> {
    let mut p = serde_json::Map::new();
    for (name, path) in [
        ("driver", std::env::current_exe()?),
        ("config", config.into()),
        ("f4", c.f4.clone()),
        ("frontier", c.frontier.clone()),
        ("singular", c.singular.clone()),
        ("msolve", c.msolve.clone()),
        ("isolation", c.isolation.clone()),
        (
            "isolation_dependency",
            c.isolation
                .parent()
                .ok_or("isolation parent")?
                .join("result_switches.py"),
        ),
        ("protocol", c.protocol.clone()),
    ] {
        p.insert(name.into(), json!({"path":path,"sha256":file_hash(&path)?}));
    }
    Ok(Value::Object(p))
}
pub(super) fn run(out: &Path, config: &Path, mode: &str) -> Result<()> {
    let dims: &[usize] = match mode {
        "qualification" => &[3],
        "development" => &[3, 4, 6],
        _ => return Err("fixed modes".into()),
    };
    let c: Config = serde_json::from_slice(&fs::read(config)?)?;
    if c.cpu != "0" {
        return Err("fixedCPU0".into());
    }
    fs::create_dir(out)?;
    fs::copy(config, out.join("config.json"))?;
    let before = pins(&c, config)?;
    save(&out.join("pins-before.json"), &before)?;
    let mut r = Runner {
        worker: c.f4.clone(),
        worker_hash: file_hash(&c.f4)?,
        singular: c.singular.clone(),
        msolve: c.msolve.clone(),
        cpu: c.cpu.clone(),
        global: Instant::now(),
        case: Instant::now(),
        global_limit: Duration::from_secs(600),
        case_limit: Duration::from_secs(180),
    };
    let mut cases = vec![];
    for &n in dims {
        r.case = Instant::now();
        let wall = Instant::now();
        let dir = out.join(format!("n{n}"));
        fs::create_dir(&dir)?;
        let mut v = json!({"n":n,"seed":7,"warmup":null,"aa":[],"main":[],"error":null,"aa_admitted":false,"result":null});
        let attempt = (|| -> Result<()> {
            v["warmup"] = panel(&r, &c, &dir.join("warmup"), n, [0, 1, 2, 3])?;
            if mode == "qualification" {
                return Ok(());
            }
            for j in 0..7 {
                let mut pair =
                    json!({"execution_order":if j%2==0{json!(["A","B"])}else{json!(["B","A"])}});
                for label in if j % 2 == 0 { ["A", "B"] } else { ["B", "A"] } {
                    pair[label] = panel(
                        &r,
                        &c,
                        &dir.join(format!("aa-{j}-{label}")),
                        n,
                        [0, 1, 2, 3],
                    )?;
                }
                v["aa"].as_array_mut().unwrap().push(pair);
            }
            let (stats, admitted) = aa(v["aa"].as_array().unwrap())?;
            v["aa_statistics"] = stats;
            v["aa_admitted"] = json!(admitted);
            for j in 0..25 {
                v["main"].as_array_mut().unwrap().push(panel(
                    &r,
                    &c,
                    &dir.join(format!("main-{j}")),
                    n,
                    ORDERS4[j % 8],
                )?);
            }
            v["result"] = main_stats(v["main"].as_array().unwrap())?;
            Ok(())
        })();
        if let Err(e) = attempt {
            v["error"] = json!(e.to_string());
        }
        v["case_wall_ns"] = json!(ns(wall));
        save(&dir.join("outcome.json"), &v)?;
        cases.push(v);
    }
    let after = pins(&c, config)?;
    save(&out.join("pins-after.json"), &after)?;
    let commit = Command::new("git").args(["rev-parse", "HEAD"]).output()?;
    save(
        &out.join("summary.json"),
        &json!({"schema":"direct-paired/v1","mode":mode,"source_commit":String::from_utf8(commit.stdout)?.trim(),"pins_unchanged":before==after,"cases":cases,"capture_wall_ns":ns(r.global)}),
    )
}
fn same(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Object(a), Value::Object(b)) => {
            a.len() == b.len() && a.iter().all(|(k, v)| b.get(k).is_some_and(|b| same(v, b)))
        }
        (Value::Array(a), Value::Array(b)) => {
            a.len() == b.len() && a.iter().zip(b).all(|(a, b)| same(a, b))
        }
        (Value::Number(a), Value::Number(b)) => {
            if let (Some(a), Some(b)) = (a.as_u64(), b.as_u64()) {
                a == b
            } else {
                a.as_f64().zip(b.as_f64()).is_some_and(|(a, b)| {
                    (a - b).abs() <= 4. * f64::EPSILON * a.abs().max(b.abs()).max(1.)
                })
            }
        }
        _ => a == b,
    }
}
fn replay(dir: &Path, p: &Value, n: usize) -> Result<usize> {
    values(p)?;
    let saved: Value = serde_json::from_slice(&fs::read(dir.join("panel.json"))?)?;
    if !same(&saved, p) {
        return Err("panel/file mismatch".into());
    }
    let original = fs::read(dir.join("original/system.json"))?;
    let system: System = serde_json::from_slice(&original)?;
    let inputs = system.decode()?;
    let fixture: workload::Fixture =
        serde_json::from_slice(&fs::read(dir.join("original/public-fixture.json"))?)?;
    let expected =
        workload::generate_graph(P, n, 7, &(0..n - 1).map(|i| (i, n - 1)).collect::<Vec<_>>())?;
    if serde_json::to_value(&fixture)? != serde_json::to_value(expected)?
        || original != serde_json::to_vec(&fixture.system)?
        || system.prime != P
        || system.variables.len() != n
    {
        return Err("fixed original fixture".into());
    }
    let reference = a::reference_file(&dir.join("references.json"), n)?;
    for alg in ["std", "slimgb", "msolve"] {
        let b = if alg == "msolve" {
            msolve_basis(
                &fs::read_to_string(dir.join("msolve-basis.txt"))?,
                &system.variables,
            )?
        } else {
            singular_basis(
                &fs::read_to_string(dir.join(format!("setup-{alg}.stdout")))?,
                n,
            )?
        };
        if b != reference {
            return Err("raw original references differ".into());
        }
    }
    for i in 0..4 {
        let d = dir.join(NAMES[i]);
        let s = &p["samples"][i];
        let (rows, selection) = construct(&inputs, n, i)?;
        let ids: Value = serde_json::from_slice(&fs::read(d.join("identities.json"))?)?;
        if ids != json!({"rows":rows.iter().map(a::wire).collect::<Vec<_>>(),"selection":selection})
        {
            return Err("constructor/identity replay".into());
        }
        let mut augmented = system.clone();
        for row in rows {
            if !verify(&row, &inputs, P) {
                return Err("original membership".into());
            }
            augmented.polynomials.push(
                row.poly
                    .iter()
                    .map(|(m, c)| Term {
                        exponents: m.clone(),
                        coefficient: *c as i64,
                    })
                    .collect(),
            );
        }
        let bytes = fs::read(d.join("system.json"))?;
        if bytes != serde_json::to_vec(&augmented)? || s["augmented_sha256"] != hash(&bytes) {
            return Err("augmented bytes".into());
        }
        if fs::read_to_string(d.join("identity.stdout"))?.trim() != "IDENTITIES_OK"
            || !fs::read(d.join("identity.stderr"))?.is_empty()
        {
            return Err("raw identity marker".into());
        }
        let basis = if i == 3 {
            frontier_guard(&s["answer"])?;
            let stdout: Value = serde_json::from_slice(&fs::read(d.join("native/call.stdout"))?)?;
            if stdout != s["answer"] || !fs::read(d.join("native/call.stderr"))?.is_empty() {
                return Err("frontier raw response".into());
            }
            let checked = relation_replay_checker::verify_json(
                &original,
                &fs::read(d.join("native/proof/witness.json"))?,
            )?;
            if serde_json::to_value(checked.stats)? != s["answer"]["consumer"]["stats"] {
                return Err("frontier replay stats".into());
            }
            native_basis(&json!(checked.basis), n)?
        } else {
            let output: Value = serde_json::from_slice(&fs::read(d.join("f4.stdout"))?)?;
            let call: Value = serde_json::from_slice(&fs::read(d.join("f4.call.json"))?)?;
            if output != s["answer"]
                || call["worker_output"] != output
                || call["complete"] != true
                || call["mature_basis_equal"] != true
                || !fs::read(d.join("f4.stderr"))?.is_empty()
            {
                return Err("F4 raw response/checks".into());
            }
            native_basis(&output["basis"], n)?
        };
        if basis != reference || s["basis_sha256"] != basis_hash(&basis) {
            return Err("full original basis equality".into());
        }
    }
    Ok(4)
}
pub(super) fn report(raw: &Path, monitor: &Path, out: &Path) -> Result<()> {
    let summary: Value = serde_json::from_slice(&fs::read(raw.join("summary.json"))?)?;
    if summary["schema"] != "direct-paired/v1"
        || summary["pins_unchanged"] != true
        || fs::read(raw.join("pins-before.json"))? != fs::read(raw.join("pins-after.json"))?
    {
        return Err("schema/pins".into());
    }
    let m: Value = serde_json::from_str(
        fs::read_to_string(monitor)?
            .lines()
            .next()
            .ok_or("monitor")?,
    )?;
    let monitor_ok =
        m["contended_samples"] == 0 && m["exit_status"] == 0 && m["reserved_cpus"] == json!([0]);
    if summary["mode"] != "qualification" && summary["mode"] != "development" {
        return Err("fixed study mode".into());
    }
    let qualification = summary["mode"] == "qualification";
    let dims: &[usize] = if qualification { &[3] } else { &[3, 4, 6] };
    let cases = summary["cases"]
        .as_array()
        .filter(|v| v.len() == dims.len())
        .ok_or("case roster")?;
    let mut checked = 0;
    let mut all = true;
    let mut audits = vec![];
    let mut md=String::from("# Direct two-cubic repeated comparison\n\nFixed development seed7 at p2147483647. Common original reference/generation/preparation fees and all per-route identity/full-basis checks are paid. Ratios are control/direct-prefix; above1 favors prefix. A/A rejections remain descriptive, never admitted. Unsigned producer evidence. [pcc:unverified]\n\n|n|Control|Work median|Work adjusted lower|Paid median|Paid adjusted lower|All A/A pass|\n|---|---|---:|---:|---:|---:|---|\n");
    for (index, c) in cases.iter().enumerate() {
        let n = dims[index];
        let dir = raw.join(format!("n{n}"));
        let disk: Value = serde_json::from_slice(&fs::read(dir.join("outcome.json"))?)?;
        if !same(c, &disk) || c["n"] != n || c["seed"] != 7 {
            return Err("case identity".into());
        }
        if !c["warmup"].is_null() {
            if c["warmup"]["order"] != json!([0, 1, 2, 3]) {
                return Err("warmup order".into());
            }
            checked += replay(&dir.join("warmup"), &c["warmup"], n)?;
        }
        for (j, p) in c["aa"].as_array().ok_or("AA")?.iter().enumerate() {
            for label in ["A", "B"] {
                checked += replay(&dir.join(format!("aa-{j}-{label}")), &p[label], n)?;
            }
        }
        for (j, p) in c["main"].as_array().ok_or("main")?.iter().enumerate() {
            checked += replay(&dir.join(format!("main-{j}")), p, n)?;
        }
        if !qualification && c["error"].is_null() {
            let (stats, admitted) = aa(c["aa"].as_array().unwrap())?;
            let result = main_stats(c["main"].as_array().unwrap())?;
            if !same(&stats, &c["aa_statistics"])
                || c["aa_admitted"] != admitted
                || !same(&result, &c["result"])
            {
                return Err("statistics or gates mismatch".into());
            }
            all &= admitted && result["numeric_all_controls_increment"] == true;
            for (j, name) in ["rank", "direct-all", "frontier"].iter().enumerate() {
                let work = &result["statistics"][2 * j];
                let paid = &result["statistics"][2 * j + 1];
                md += &format!(
                    "|{n}|{name}|{}|{}|{}|{}|{admitted}|\n",
                    work["median_paired_ratio"],
                    work["adjusted_lower_1_over_720"],
                    paid["median_paired_ratio"],
                    paid["adjusted_lower_1_over_720"]
                );
            }
            audits
                .push(json!({"n":n,"aa_statistics":stats,"aa_admitted":admitted,"result":result}));
        } else {
            all = false;
            audits.push(json!({"n":n,"qualification":qualification,"error":c["error"]}));
        }
    }
    let admitted = all && monitor_ok && !qualification;
    md+=&format!("\nReplayed answers:{checked}. Monitor valid:{monitor_ok}. Sustained gain across all cases and all controls/scopes:{admitted}. No pooling or replacement. Every panel, phase, failed attempt, identity and native replay wire remains in raw records. Phase clocks are descriptive; parent remainder has no established physical attribution. [pcc:unverified]\n");
    fs::create_dir(out)?;
    save(
        &out.join("audit.json"),
        &json!({"schema":"direct-paired-audit/v1","qualification":qualification,"checked_answers":checked,"all_constructors_identities_raw_bases_and_frontier_wires_replayed":true,"costs_orders_and_statistics_recomputed":true,"monitor_ok":monitor_ok,"admitted_sustained_gain":admitted,"cases":audits}),
    )?;
    write_new(&out.join("report.md"), md.as_bytes())?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn p(work: [u64; 4], order: [usize; 4]) -> Value {
        let samples:Vec<_>=work.iter().enumerate().map(|(i,w)|json!({"name":NAMES[i],"status":"complete","basis_sha256":"a","work_ns":w,"paid_ns":w+4,"phases":{"construction_ns":1,"preparation_ns":1,"identity_ns":1,"solve_ns":1,"other_ns":w-4}})).collect();
        json!({"order":order,"all_complete_bases_equal":true,"same_augmented_inputs":true,"generation_ns":1,"preparation_ns":1,"reference_ns":1,"equality_ns":1,"samples":samples})
    }
    #[test]
    fn accounting_and_selection_guards() {
        let mut v = p([10; 4], [0, 1, 2, 3]);
        assert!(values(&v).is_ok());
        v["samples"][2]["paid_ns"] = json!(11);
        assert!(values(&v).is_err());
        let mut v = p([10; 4], [0, 1, 2, 3]);
        v["same_augmented_inputs"] = json!(false);
        assert!(values(&v).is_err());
    }
    #[test]
    fn aa_rejections_and_fixed_main_are_preserved() {
        let mut pairs = vec![];
        for j in 0..7 {
            pairs.push(json!({"execution_order":if j%2==0{json!(["A","B"])}else{json!(["B","A"])},"A":p([10;4],[0,1,2,3]),"B":p([10;4],[0,1,2,3])}));
        }
        assert!(aa(&pairs).unwrap().1);
        for pair in &mut pairs {
            pair["A"] = p([20, 10, 10, 10], [0, 1, 2, 3]);
        }
        assert!(!aa(&pairs).unwrap().1);
        let mut blocks: Vec<_> = (0..25)
            .map(|j| p([40, 30, 10, 20], ORDERS4[j % 8]))
            .collect();
        assert_eq!(
            main_stats(&blocks).unwrap()["numeric_all_controls_increment"],
            true
        );
        blocks[0]["order"] = json!([3, 2, 1, 0]);
        assert!(main_stats(&blocks).is_err());
    }
    #[test]
    fn direct_rows_equal_rank_and_are_original_identities() {
        for n in [3, 4, 6] {
            let f = workload::generate_graph(
                P,
                n,
                7,
                &(0..n - 1).map(|i| (i, n - 1)).collect::<Vec<_>>(),
            )
            .unwrap();
            let inputs = f.system.decode().unwrap();
            let (rank, _) = construct(&inputs, n, 0).unwrap();
            for mode in [1, 2] {
                let (rows, _) = construct(&inputs, n, mode).unwrap();
                assert_eq!(
                    rows.iter().map(a::wire).collect::<Vec<_>>(),
                    rank.iter().map(a::wire).collect::<Vec<_>>()
                );
                assert_eq!(rows.len(), 2);
                assert!(rows.iter().all(|r| verify(r, &inputs, P)));
            }
        }
    }
    #[test]
    fn orders_are_position_balanced() {
        for i in 0..4 {
            let mut counts = [0; 4];
            for o in ORDERS4 {
                counts[o.iter().position(|v| *v == i).unwrap()] += 1;
            }
            assert_eq!(counts, [2; 4]);
        }
    }
    #[test]
    fn malformed_prefix_and_mode_refused() {
        assert!(construct(&[], 3, 2).is_err());
        assert!(construct(&[], 3, 9).is_err());
    }
    #[test]
    fn overflow_and_float_flags() {
        assert!(sum(&[u64::MAX, 1]).is_err());
        assert!(!same(&json!(true), &json!(1)));
        assert!(!same(&json!(5), &json!(6)));
        assert!(same(&json!(1.2), &json!(1.2 + f64::EPSILON)));
    }
}

//! Fixed, paid early-rank selection. No completed basis enters construction.
use super::*;
use certified_relations::{cheap, degree, trace, verify, Row, Term};
use relation_ablations as a;
use std::collections::BTreeMap;
const CLASSES: [&str; 6] = [
    "star",
    "differences",
    "syzygies",
    "both",
    "rank-budget",
    "count-control",
];
fn mul(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % P as u128) as u64
}
fn inverse(mut a: u64) -> u64 {
    let mut e = P - 2;
    let mut b = 1;
    while e > 0 {
        if e & 1 == 1 {
            b = mul(b, a);
        }
        a = mul(a, a);
        e >>= 1;
    }
    b
}
#[derive(Clone, Default)]
struct Space {
    pivots: BTreeMap<Vec<usize>, Poly>,
}
impl Space {
    fn add(&mut self, mut f: Poly) -> bool {
        while let Some((m, c)) = f.first_key_value().map(|(m, c)| (m.clone(), *c)) {
            if let Some(g) = self.pivots.get(&m) {
                for (m, v) in g {
                    let old = f.get(m).copied().unwrap_or(0);
                    let x = (old + P - mul(c, *v)) % P;
                    if x == 0 {
                        f.remove(m);
                    } else {
                        f.insert(m.clone(), x);
                    }
                }
            } else {
                let inv = inverse(c);
                for c in f.values_mut() {
                    *c = mul(*c, inv);
                }
                self.pivots.insert(m, f);
                return true;
            }
        }
        false
    }
    fn with_candidate(&self, f: &Poly, n: usize) -> Self {
        let mut s = self.clone();
        s.add(f.clone());
        for i in 0..n {
            let shifted = f
                .iter()
                .map(|(m, c)| {
                    let mut m = m.clone();
                    m[i] += 1;
                    (m, *c)
                })
                .collect();
            s.add(shifted);
        }
        s
    }
}
fn canonical(f: &Poly) -> Poly {
    let c = *f.first_key_value().unwrap().1;
    let inv = inverse(c);
    f.iter().map(|(m, c)| (m.clone(), mul(*c, inv))).collect()
}
pub(super) fn matrix_probe(out: &Path) -> Result<()> {
    fs::create_dir(out)?;
    let matrices = [
        [[1, 2, 0], [3, 6, 0], [0, 0, 1]],
        [[P - 1, 1, 0], [1, P - 1, 0], [0, 0, 1]],
        [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
        [[0; 3]; 3],
    ];
    let mut ranks = vec![];
    let mut script = format!("LIB \"linalg.lib\";ring r={P},(x,y,z),dp;\n");
    for (i, m) in matrices.iter().enumerate() {
        let mut space = Space::default();
        for row in m {
            let f = row
                .iter()
                .enumerate()
                .filter(|(_, c)| **c != 0)
                .map(|(j, c)| {
                    let mut e = vec![0; 3];
                    e[j] = 1;
                    (e, *c)
                })
                .collect();
            space.add(f);
        }
        ranks.push(space.pivots.len());
        script += &format!(
            "matrix A{i}[3][3]={};list Z{i}=gaussred(A{i});print(\"RANK:\"+string(Z{i}[4]));\n",
            m.iter()
                .flatten()
                .map(u64::to_string)
                .collect::<Vec<_>>()
                .join(",")
        );
    }
    script += "quit;\n";
    save(
        &out.join("native.json"),
        &json!({"matrices":matrices,"ranks":ranks}),
    )?;
    write_new(&out.join("rank.sing"), script.as_bytes())?;
    Ok(())
}
pub(super) fn matrix_audit(probe: &Path, output: &Path) -> Result<()> {
    let n: Value = serde_json::from_slice(&fs::read(probe.join("native.json"))?)?;
    let ranks = fs::read_to_string(output)?
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            l.trim()
                .strip_prefix("RANK:")
                .ok_or("unexpected Singular line")?
                .parse::<usize>()
                .map_err(|_| "bad rank")
        })
        .collect::<std::result::Result<Vec<_>, _>>()?;
    if json!(ranks) != n["ranks"] || ranks != [2, 2, 3, 0] {
        return Err("field rank mismatch".into());
    }
    println!("MATRIX_RANKS_MATCH");
    Ok(())
}
fn cert_degree(r: &Row, inputs: &[Poly]) -> usize {
    r.proof
        .keys()
        .map(|(i, m)| m.iter().sum::<usize>() + degree(&inputs[*i]))
        .max()
        .unwrap_or(0)
}
fn eligible(r: &Row, inputs: &[Poly]) -> bool {
    degree(&r.poly) <= 3
        && r.poly.len() <= 16
        && !r.proof.is_empty()
        && r.proof.len() <= 16
        && cert_degree(r, inputs) <= 6
}
fn better(gain: usize, cost: usize, id: usize, bgain: usize, bcost: usize, bid: usize) -> bool {
    let lhs = gain as u128 * bcost as u128;
    let rhs = bgain as u128 * cost as u128;
    lhs > rhs || (lhs == rhs && (cost, id) < (bcost, bid))
}
pub(super) fn select(inputs: &[Poly], n: usize, class: usize) -> Result<(Vec<Row>, Value)> {
    if class >= CLASSES.len() {
        return Err("unregistered class".into());
    }
    if class == 0 {
        return Ok((
            vec![],
            json!({"candidates":[],"rank_selection":null,"selected_ids":[]}),
        ));
    }
    let pairs = cheap::consequences(inputs, n, P);
    let candidates: Vec<_> = pairs
        .iter()
        .flat_map(|p| [p.cubic_difference.clone(), p.cubic_syzygy.clone()])
        .collect();
    if candidates.len() != 2 * (n - 1)
        || !candidates
            .iter()
            .all(|r| degree(&r.poly) == 3 && verify(r, inputs, P))
    {
        return Err("direct candidate recognition/proof".into());
    }
    let mut seen = BTreeSet::new();
    let mut duplicate = Vec::new();
    for r in &candidates {
        duplicate.push(!seen.insert(canonical(&r.poly)));
    }
    let descriptions:Vec<_>=candidates.iter().enumerate().map(|(id,r)|json!({"id":id,"family":if id%2==0{"difference"}else{"syzygy"},"poly_terms":r.poly.len(),"proof_terms":r.proof.len(),"certificate_product_degree":cert_degree(r,inputs),"eligible":eligible(r,inputs),"duplicate":duplicate[id],"row":a::wire(r)})).collect();
    let (ids, rank_selection) = if class <= 3 {
        (
            (0..candidates.len())
                .filter(|i| !duplicate[*i] && (class == 3 || *i % 2 == class - 1))
                .collect::<Vec<_>>(),
            Value::Null,
        )
    } else {
        let sentinel = Row {
            poly: Poly::from([(vec![0; n], 1)]),
            proof: Default::default(),
        };
        let (closure, archive) = trace(inputs, n, P, &[sentinel], 4, 700);
        if closure.status != "degree_cap"
            || closure.solving_degree.is_some()
            || closure.trace.last().is_none_or(|e| !e.closure_completed)
            || !archive.iter().all(|(_, r)| verify(r, inputs, P))
        {
            return Err("degree4 closure not complete/proper".into());
        }
        let mut space = Space::default();
        for f in inputs {
            space.add(f.clone());
        }
        for (_, r) in &archive {
            space.add(r.poly.clone());
        }
        if space.pivots.len() != closure.trace.last().unwrap().rank {
            return Err("independent rank reconstruction".into());
        }
        let start_rank = space.pivots.len();
        let baseline_space = space.clone();
        let mut chosen = Vec::new();
        let mut rounds = Vec::new();
        let mut total_cost = 0;
        while chosen.len() < 2 {
            let mut winner = None;
            let mut trials = Vec::new();
            for (id, r) in candidates.iter().enumerate() {
                if duplicate[id]
                    || chosen.contains(&id)
                    || !eligible(r, inputs)
                    || total_cost + r.proof.len() > 32
                {
                    continue;
                }
                let trial = space.with_candidate(&r.poly, n);
                let gain = trial.pivots.len() - space.pivots.len();
                trials.push(json!({"id":id,"gain":gain,"proof_terms":r.proof.len(),"before_rank":space.pivots.len(),"after_rank":trial.pivots.len()}));
                if gain > 0
                    && winner.as_ref().is_none_or(|(bid, bgain, bcost, _)| {
                        better(gain, r.proof.len(), id, *bgain, *bcost, *bid)
                    })
                {
                    winner = Some((id, gain, r.proof.len(), trial));
                }
            }
            let selected = winner.as_ref().map(|v| v.0);
            rounds.push(json!({"trials":trials,"winner":selected}));
            if let Some((id, _, cost, trial)) = winner {
                chosen.push(id);
                total_cost += cost;
                space = trial;
            } else {
                break;
            }
        }
        let ids = if class == 4 {
            chosen.clone()
        } else {
            let c: Vec<_> = (0..candidates.len())
                .filter(|i| i % 2 == 0 && !duplicate[*i] && eligible(&candidates[*i], inputs))
                .take(chosen.len())
                .collect();
            if c.len() != chosen.len() {
                return Err("count control infeasible".into());
            }
            c
        };
        let mut exported = baseline_space;
        for id in &ids {
            exported = exported.with_candidate(&candidates[*id].poly, n);
        }
        let info = json!({"baseline_closure":closure,"baseline_rows":archive.iter().map(|(d,r)|json!({"degree":d,"row":a::wire(r)})).collect::<Vec<_>>(),"baseline_rank":start_rank,"selected_rank_ids":chosen,"selected_rank_proof_terms":total_cost,"rank_after_selection":space.pivots.len(),"exported_rank_gain":exported.pivots.len()-start_rank,"rounds":rounds,"scope":"immediate degree4 multiples over fixed original closure, no reclosure of trials"});
        (ids, info)
    };
    let rows = ids.iter().map(|i| candidates[*i].clone()).collect();
    Ok((
        rows,
        json!({"candidates":descriptions,"rank_selection":rank_selection,"selected_ids":ids}),
    ))
}
pub(super) fn build(input: &Path, out: &Path, class: usize) -> Result<()> {
    let s: System = serde_json::from_slice(&fs::read(input)?)?;
    let n = s.variables.len();
    if s.prime != P || ![3, 4, 6].contains(&n) {
        return Err("fixed selector grid".into());
    }
    let inputs = s.decode()?;
    let (rows, selection) = select(&inputs, n, class)?;
    save(
        &out.join("selection.json"),
        &json!({"rows":rows.iter().map(a::wire).collect::<Vec<_>>(),"selection":selection}),
    )?;
    println!("RANK_BUILD_OK");
    Ok(())
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    worker: PathBuf,
    singular: PathBuf,
    msolve: PathBuf,
    cpu: String,
}
fn cell(
    r: &mut Runner,
    out: &Path,
    n: usize,
    class: usize,
    base: &mut Option<Basis>,
) -> Result<Value> {
    fs::create_dir(out)?;
    r.case = Instant::now();
    let wall = Instant::now();
    let start = Instant::now();
    let edges: Vec<_> = (0..n - 1).map(|i| (i, n - 1)).collect();
    let fixture = workload::generate_graph(P, n, 7, &edges)?;
    let generation_ns = ns(start);
    save(&out.join("public-fixture.json"), &fixture)?;
    save(&out.join("original.json"), &fixture.system)?;
    if !workload::witness_check(&fixture.system, &fixture.witness) {
        return Err("original public fixture".into());
    }
    let start = Instant::now();
    let (status, bytes, _) = r.process(
        &std::env::current_exe()?,
        &[
            "rank-build".into(),
            out.join("original.json").display().to_string(),
            out.display().to_string(),
            class.to_string(),
        ],
        None,
        &out.join("build"),
    )?;
    let build_ns = ns(start);
    if !status.success()
        || std::str::from_utf8(&bytes)?.trim() != "RANK_BUILD_OK"
        || !fs::read(out.join("build.stderr"))?.is_empty()
    {
        return Err(format!("bounded rank build incomplete: {status}").into());
    }
    let start = Instant::now();
    let selected: Value = serde_json::from_slice(&fs::read(out.join("selection.json"))?)?;
    let original = fixture.system.decode()?;
    let rows = selected["rows"]
        .as_array()
        .ok_or("selected rows")?
        .iter()
        .map(a::row)
        .collect::<Result<Vec<_>>>()?;
    if !rows.iter().all(|r| verify(r, &original, P)) {
        return Err("selected identity".into());
    }
    let mut system = fixture.system.clone();
    for row in &rows {
        system.polynomials.push(
            row.poly
                .iter()
                .map(|(m, c)| Term {
                    exponents: m.clone(),
                    coefficient: *c as i64,
                })
                .collect(),
        );
    }
    let input_hash = a::prepare(out, &system)?;
    write_new(
        &out.join("identities.sing"),
        a::identity_script(&fixture.system, &original, &rows).as_bytes(),
    )?;
    let preparation_ns = ns(start);
    let start = Instant::now();
    let (status, bytes, _) = r.process(
        &r.singular,
        &[
            "-q".into(),
            out.join("identities.sing").display().to_string(),
        ],
        None,
        &out.join("identity-check"),
    )?;
    let identity_check_ns = ns(start);
    let identities_ok = status.success()
        && std::str::from_utf8(&bytes)?.trim() == "IDENTITIES_OK"
        && fs::read(out.join("identity-check.stderr"))?.is_empty();
    let mut failures = vec![];
    if !identities_ok {
        failures.push("Singular identities failed".to_string());
    }
    let start = Instant::now();
    let reference = r.references(out, out, &system, 0);
    let reference_ns = ns(start);
    let reference = match reference {
        Ok((b, _)) => Some(b),
        Err(e) => {
            failures.push(format!("references: {e}"));
            None
        }
    };
    if class == 0 {
        *base = reference.clone();
    }
    let same_ideal = base.as_ref().zip(reference.as_ref()).map(|(a, b)| a == b);
    let start = Instant::now();
    let empty = Basis::new();
    let sample = r.sample(
        Arm::F4,
        out,
        &out.join("f4"),
        n,
        reference.as_ref().unwrap_or(&empty),
        &input_hash,
    );
    let checked_f4_ns = ns(start);
    if let Err(e) = sample {
        failures.push(format!("F4: {e}"));
    }
    let mut diagnostics_ok = false;
    let mut diagnostic_ns = 0;
    if reference.is_some() {
        let start = Instant::now();
        let attempt = r.process(
            &std::env::current_exe()?,
            &[
                "relation-diagnostics".into(),
                out.join("system.json").display().to_string(),
                out.join("references.json").display().to_string(),
                out.join("diagnostic").display().to_string(),
                if n == 6 { "6" } else { "8" }.into(),
            ],
            None,
            &out.join("diagnostic-process"),
        );
        diagnostic_ns = ns(start);
        match attempt {
            Ok((s, b, _)) => {
                diagnostics_ok = s.success()
                    && std::str::from_utf8(&b)?.trim() == "DIAGNOSTICS_OK"
                    && fs::read(out.join("diagnostic-process.stderr"))?.is_empty();
                if !diagnostics_ok {
                    failures.push(format!("diagnostics: {s}"));
                }
            }
            Err(e) => failures.push(format!("diagnostics: {e}")),
        }
    }
    let paid_ns = [
        generation_ns,
        build_ns,
        preparation_ns,
        identity_check_ns,
        reference_ns,
        checked_f4_ns,
    ]
    .into_iter()
    .try_fold(0u64, |a, b| a.checked_add(b).ok_or("fee overflow"))?;
    Ok(
        json!({"n":n,"class":CLASSES[class],"class_index":class,"input_sha256":input_hash,"selected_rows":rows.len(),"same_ideal":same_ideal,"identities_ok":identities_ok,"diagnostics_ok":diagnostics_ok,"generation_ns":generation_ns,"build_ns":build_ns,"preparation_ns":preparation_ns,"identity_check_ns":identity_check_ns,"reference_ns":reference_ns,"checked_f4_ns":checked_f4_ns,"paid_ns":paid_ns,"separate_diagnostic_ns":diagnostic_ns,"case_wall_ns":ns(wall),"failures":failures}),
    )
}
pub(super) fn run(out: &Path, config: &Path, mode: &str) -> Result<()> {
    let dims: &[usize] = match mode {
        "qualification" => &[3],
        "development" => &[3, 4, 6],
        _ => return Err("fixed modes".into()),
    };
    let c: Config = serde_json::from_slice(&fs::read(config)?)?;
    if c.cpu != "0" {
        return Err("CPU0 required".into());
    }
    fs::create_dir(out)?;
    fs::copy(config, out.join("config.json"))?;
    let pins = json!({"worker":file_hash(&c.worker)?,"singular":file_hash(&c.singular)?,"msolve":file_hash(&c.msolve)?,"driver":file_hash(&std::env::current_exe()?)?});
    save(&out.join("pins-before.json"), &pins)?;
    let mut r = Runner {
        worker_hash: file_hash(&c.worker)?,
        worker: c.worker,
        singular: c.singular,
        msolve: c.msolve,
        cpu: c.cpu,
        global: Instant::now(),
        case: Instant::now(),
        global_limit: Duration::from_secs(600),
        case_limit: Duration::from_secs(180),
    };
    let mut cells = vec![];
    for &n in dims {
        let mut base = None;
        for (class, name) in CLASSES.iter().enumerate() {
            let dir = out.join(format!("n{n}-{name}"));
            let wall = Instant::now();
            let v = match cell(&mut r, &dir, n, class, &mut base) {
                Ok(v) => v,
                Err(e) => {
                    json!({"n":n,"class":name,"class_index":class,"failures":[e.to_string()],"case_wall_ns":ns(wall),"paid_ns":null})
                }
            };
            save(&dir.join("cell.json"), &v)?;
            cells.push(v);
        }
    }
    let after = json!({"worker":file_hash(&r.worker)?,"singular":file_hash(&r.singular)?,"msolve":file_hash(&r.msolve)?,"driver":file_hash(&std::env::current_exe()?)?});
    save(&out.join("pins-after.json"), &after)?;
    save(
        &out.join("summary.json"),
        &json!({"schema":"rank-budget/v1","mode":mode,"cells":cells,"pins_unchanged":pins==after,"capture_wall_ns":ns(r.global)}),
    )
}
fn optional(path: &Path, required: bool) -> Result<Value> {
    match fs::read(path)
        .map_err(|e| e.into())
        .and_then(|b| serde_json::from_slice(&b).map_err(|e| e.into()))
    {
        Ok(v) => Ok(v),
        Err(e) if required => Err(e),
        Err(_) => Ok(Value::Null),
    }
}
pub(super) fn report(raw: &Path, monitor: &Path, out: &Path) -> Result<()> {
    let s: Value = serde_json::from_slice(&fs::read(raw.join("summary.json"))?)?;
    let dims: &[usize] = match s["mode"].as_str() {
        Some("qualification") => &[3],
        Some("development") => &[3, 4, 6],
        _ => return Err("fixed mode".into()),
    };
    if s["schema"] != "rank-budget/v1"
        || s["pins_unchanged"] != true
        || fs::read(raw.join("pins-before.json"))? != fs::read(raw.join("pins-after.json"))?
    {
        return Err("schema/pins".into());
    }
    let mon: Value = serde_json::from_str(
        fs::read_to_string(monitor)?
            .lines()
            .next()
            .ok_or("monitor")?,
    )?;
    if mon["contended_samples"] != 0
        || mon["exit_status"] != 0
        || mon["reserved_cpus"] != json!([0])
    {
        return Err("isolation".into());
    }
    let cells = s["cells"]
        .as_array()
        .filter(|c| c.len() == dims.len() * 6)
        .ok_or("fixed cells")?;
    let mut audit = vec![];
    let mut costs=String::from("\n|n|Class|Generation ms|Build/selection ms|Prepare ms|Identity check ms|All3 refs ms|Checked F4 ms|Separate diagnostic ms|\n|---|---|---:|---:|---:|---:|---:|---:|---:|\n");
    let mut md=String::from("# Budgeted early-rank relation study\n\nFixed development p2147483647,seed7. Rank gain is a proxy; actual F4, affine closure and top-form Hilbert remain separate. Single-call times descriptive, no speedup admission. [pcc:unverified]\n\n|n|Class|Rows added|Early rank gain|F4 degree|Max rows|Max columns|Affine threshold|Top-form vanishing|Build ms|Paid ms|\n|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n");
    for (index, c) in cells.iter().enumerate() {
        let n = dims[index / 6];
        let class = index % 6;
        let dir = raw.join(format!("n{n}-{}", CLASSES[class]));
        let disk: Value = serde_json::from_slice(&fs::read(dir.join("cell.json"))?)?;
        if c != &disk || c["n"] != n || c["class"] != CLASSES[class] || c["class_index"] != class {
            return Err("cell order/record".into());
        }
        let mut selection = Value::Null;
        if c.get("input_sha256").is_some() {
            let fixture: workload::Fixture =
                serde_json::from_slice(&fs::read(dir.join("public-fixture.json"))?)?;
            let expected = workload::generate_graph(
                P,
                n,
                7,
                &(0..n - 1).map(|i| (i, n - 1)).collect::<Vec<_>>(),
            )?;
            if serde_json::to_value(&fixture)? != serde_json::to_value(expected)? {
                return Err("public original fixture".into());
            }
            let bytes = fs::read(dir.join("system.json"))?;
            let system: System = serde_json::from_slice(&bytes)?;
            if hash(&bytes) != c["input_sha256"]
                || !workload::witness_check(&system, &fixture.witness)
            {
                return Err("input/witness".into());
            }
            let original = fixture.system.decode()?;
            let (rows, recomputed) = select(&original, n, class)?;
            selection = serde_json::from_slice(&fs::read(dir.join("selection.json"))?)?;
            if selection
                != json!({"rows":rows.iter().map(a::wire).collect::<Vec<_>>(),"selection":recomputed})
            {
                return Err("selection/rank decisions replay".into());
            }
            let mut inputs = original;
            for row in rows {
                if !verify(&row, &inputs[..fixture.system.polynomials.len()], P) {
                    return Err("identity replay".into());
                }
                inputs.push(row.poly);
            }
            if inputs != system.decode()?
                || c["selected_rows"] != inputs.len() - fixture.system.polynomials.len()
            {
                return Err("augmented inputs/count".into());
            }
            let fees = [
                "generation_ns",
                "build_ns",
                "preparation_ns",
                "identity_check_ns",
                "reference_ns",
                "checked_f4_ns",
            ]
            .into_iter()
            .try_fold(0u64, |a, k| {
                a.checked_add(c[k].as_u64().ok_or("fee")?)
                    .ok_or("fee overflow")
            })?;
            if c["paid_ns"] != fees {
                return Err("paid sum".into());
            }
            if c["identities_ok"] == true
                && (fs::read_to_string(dir.join("identity-check.stdout"))?.trim()
                    != "IDENTITIES_OK"
                    || !fs::read(dir.join("identity-check.stderr"))?.is_empty())
            {
                return Err("Singular identity raw marker".into());
            }
            if dir.join("references.json").exists() {
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
                        return Err("raw mature bases".into());
                    }
                }
                let same = a::reference_file(&raw.join(format!("n{n}-star/references.json")), n)
                    .ok()
                    .map(|b| b == reference);
                if c["same_ideal"] != json!(same) {
                    return Err("same-ideal label".into());
                }
                let f = optional(&dir.join("f4.call.json"), false)?;
                if !f.is_null() {
                    let bytes = fs::read(dir.join("f4.stdout"))?;
                    let raw_f: Value = serde_json::from_slice(&bytes)?;
                    if f["worker_output"] != raw_f
                        || (f["complete"] == true && native_basis(&raw_f["basis"], n)? != reference)
                    {
                        return Err("raw F4 basis/record".into());
                    }
                }
            }
            let t = optional(
                &dir.join("diagnostic.trace.json"),
                c["diagnostics_ok"] == true,
            )?;
            if let Some(rows) = t["rows"].as_array() {
                for r in rows {
                    if !verify(&a::row(&r["row"])?, &inputs, P) {
                        return Err("diagnostic identities".into());
                    }
                }
            }
        }
        let f = optional(&dir.join("f4.call.json"), false)?;
        let t = optional(
            &dir.join("diagnostic.trace.json"),
            c["diagnostics_ok"] == true,
        )?;
        let h = optional(
            &dir.join("diagnostic.hilbert.json"),
            c["diagnostics_ok"] == true,
        )?;
        let o = &f["worker_output"];
        let rs = &selection["selection"]["rank_selection"];
        let gain = rs["exported_rank_gain"].as_u64();
        md += &format!(
            "|{n}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|\n",
            CLASSES[class],
            c["selected_rows"],
            json!(gain),
            o["observed_solving_degree"],
            o["f4_observations"]["max_rows"],
            o["f4_observations"]["max_columns"],
            t["run"]["solving_degree"],
            h["hilbert"]["top_form_d_reg"],
            c["build_ns"]
                .as_u64()
                .map(|v| format!("{:.6}", v as f64 / 1e6))
                .unwrap_or("null".into()),
            c["paid_ns"]
                .as_u64()
                .map(|v| format!("{:.6}", v as f64 / 1e6))
                .unwrap_or("null".into())
        );
        audit.push(json!({"cell":c,"selection":selection,"f4":o,"affine":t["run"],"hilbert":h}));
        let ms = |key: &str| {
            c[key]
                .as_u64()
                .map(|v| format!("{:.6}", v as f64 / 1e6))
                .unwrap_or("null".into())
        };
        costs += &format!(
            "|{n}|{}|{}|{}|{}|{}|{}|{}|{}|\n",
            CLASSES[class],
            ms("generation_ns"),
            ms("build_ns"),
            ms("preparation_ns"),
            ms("identity_check_ns"),
            ms("reference_ns"),
            ms("checked_f4_ns"),
            ms("separate_diagnostic_ns")
        );
    }
    md+=&format!("\nFlow:planned/attempted {}, cells with recorded failures {}. All retained. [pcc:unverified]\n",cells.len(),cells.iter().filter(|v|v["failures"].as_array().is_none_or(|f|!f.is_empty())).count());
    for c in cells {
        if c["failures"].as_array().is_some_and(|f| !f.is_empty()) {
            md += &format!(
                "\n{} n{}: {} [pcc:unverified]\n",
                c["class"], c["n"], c["failures"]
            );
        }
    }
    md += &costs;
    md+="\nEarly rank gain is for the actually exported rows. The count-control pays the identical selector but exports differences; hypothetical winning gains remain separately archived. Full discovery/selection/serialization/process/native/external checks and all3 reference fees are paid. Separate affine/Hilbert diagnostics are outside the paid F4 clock; reference/oracle fees remain in it. F4 nonzeros/fill absent, not zero. No intrinsic regularity or15x claim. [pcc:unverified]\n";
    fs::create_dir(out)?;
    save(
        &out.join("audit.json"),
        &json!({"schema":"rank-budget-audit/v1","mode":s["mode"],"all_rank_decisions_and_raw_proofs_replayed":true,"monitor_valid":true,"cells":audit}),
    )?;
    write_new(&out.join("report.md"), md.as_bytes())?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dependent_and_independent_rank() {
        let mut s = Space::default();
        assert!(s.add(Poly::from([(vec![0, 0, 0], 1), (vec![1, 0, 0], 2)])));
        assert!(!s.add(Poly::from([(vec![0, 0, 0], 3), (vec![1, 0, 0], 6)])));
        assert!(s.add(Poly::from([(vec![0, 1, 0], 1)])));
        assert_eq!(s.pivots.len(), 2);
    }
    #[test]
    fn exact_ties_and_ratio() {
        assert!(better(2, 3, 2, 3, 5, 1));
        assert!(better(2, 4, 1, 3, 6, 0));
        assert!(better(2, 4, 0, 2, 4, 1));
        assert!(!better(2, 4, 1, 2, 4, 0));
    }
    #[test]
    fn selection_is_positive_bounded_and_control_count() {
        let f = workload::generate_graph(P, 3, 7, &[(0, 2), (1, 2)]).unwrap();
        let inputs = f.system.decode().unwrap();
        let (rows, m) = select(&inputs, 3, 4).unwrap();
        let (control, c) = select(&inputs, 3, 5).unwrap();
        assert!(!rows.is_empty());
        assert!(rows.len() <= 2);
        assert_eq!(control.len(), rows.len());
        assert_eq!(m["rank_selection"]["rounds"], c["rank_selection"]["rounds"]);
        assert!(m["rank_selection"]["exported_rank_gain"].as_u64().unwrap() > 0);
        assert_eq!(c["rank_selection"]["exported_rank_gain"], 0);
        assert!(rows.iter().all(|r| eligible(r, &inputs)));
        assert!(
            m["rank_selection"]["selected_rank_proof_terms"]
                .as_u64()
                .unwrap()
                <= 32
        );
        for round in m["rank_selection"]["rounds"].as_array().unwrap() {
            for t in round["trials"].as_array().unwrap() {
                if t["id"].as_u64().unwrap() % 2 == 0 {
                    assert_eq!(t["gain"], 0);
                }
            }
        }
    }
    #[test]
    fn malformed_class_refused() {
        assert!(select(&[], 3, 6).is_err());
    }
    #[test]
    fn truncated_failed_artifact_nullable() {
        let p = std::env::temp_dir().join(format!("rank-partial-{}", std::process::id()));
        fs::write(&p, b"{partial").unwrap();
        assert!(optional(&p, false).unwrap().is_null());
        assert!(optional(&p, true).is_err());
        fs::remove_file(p).unwrap();
    }
}

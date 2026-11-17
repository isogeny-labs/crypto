//! Fixed development ablations; no fresh-input selection or solver-core changes.
use super::*;
use certified_relations::{degree, hilbert, projection, trace, verify, Row, Term};
use plotters::prelude::*;

const CLASSES: [&str; 6] = [
    "star",
    "root-projection",
    "nonroot-projections",
    "all-projections",
    "leaf-closure",
    "clique",
];
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    worker: PathBuf,
    singular: PathBuf,
    msolve: PathBuf,
    cpu: String,
}

fn edges(n: usize, class: usize) -> Vec<(usize, usize)> {
    if class == 5 {
        return (0..n)
            .flat_map(|i| (i + 1..n).map(move |j| (i, j)))
            .collect();
    }
    let mut e: Vec<_> = (0..n - 1).map(|i| (i, n - 1)).collect();
    if class == 4 {
        e.push((0, 1));
    }
    e.sort_unstable();
    e.dedup();
    e
}
pub(super) fn wire(row: &Row) -> Value {
    json!({"poly":row.poly.iter().collect::<Vec<_>>(),"proof":row.proof.iter().collect::<Vec<_>>()})
}
pub(super) fn row(v: &Value) -> Result<Row> {
    let f: Vec<(Vec<usize>, u64)> = serde_json::from_value(v["poly"].clone())?;
    let proof: Vec<((usize, Vec<usize>), u64)> = serde_json::from_value(v["proof"].clone())?;
    let r = Row {
        poly: f.iter().cloned().collect(),
        proof: proof.iter().cloned().collect(),
    };
    if r.poly.len() != f.len() || r.proof.len() != proof.len() {
        return Err("duplicate certificate key".into());
    }
    let n = f.first().ok_or("empty certified polynomial")?.0.len();
    if ![3, 4, 6].contains(&n)
        || f.iter().any(|(m, c)| m.len() != n || *c == 0 || *c >= P)
        || proof
            .iter()
            .any(|((_, m), c)| m.len() != n || *c == 0 || *c >= P)
    {
        return Err("noncanonical certificate shape/residue".into());
    }
    Ok(r)
}
pub(super) fn identity_script(s: &System, inputs: &[Poly], rows: &[Row]) -> String {
    let mut code = format!(
        "ring r={},({}),dp;\npoly h;\n",
        s.prime,
        s.variables.join(",")
    );
    for row in rows {
        let rhs = row
            .proof
            .iter()
            .map(|((i, m), c)| {
                let monomial = Poly::from([(m.clone(), *c)]);
                format!(
                    "({})*({})",
                    expr(&monomial, &s.variables),
                    expr(&inputs[*i], &s.variables)
                )
            })
            .collect::<Vec<_>>()
            .join("+");
        code += &format!(
            "h=({})-({});if(h!=0){{print(\"IDENTITY_FAILURE\");quit;}}\n",
            expr(&row.poly, &s.variables),
            if rhs.is_empty() { "0" } else { &rhs }
        );
    }
    code + "print(\"IDENTITIES_OK\");quit;\n"
}
pub(super) fn prepare(dir: &Path, s: &System) -> Result<String> {
    let inputs = s.decode()?;
    let bytes = serde_json::to_vec(s)?;
    write_new(&dir.join("system.json"), &bytes)?;
    for alg in ["std", "slimgb"] {
        write_new(
            &dir.join(format!("{alg}.sing")),
            singular_code(s, &inputs, alg).as_bytes(),
        )?;
    }
    write_new(
        &dir.join("input.ms"),
        format!(
            "{}\n{}\n{}\n",
            s.variables.join(","),
            s.prime,
            inputs
                .iter()
                .map(|f| expr(f, &s.variables))
                .collect::<Vec<_>>()
                .join(",\n")
        )
        .as_bytes(),
    )?;
    Ok(hash(&bytes))
}
fn selected(inputs: &[Poly], n: usize, class: usize) -> Result<(Vec<Row>, Value)> {
    if !(1..=3).contains(&class) {
        return Ok((Vec::new(), json!([])));
    }
    let (all, vertices) = projection::certified_quadratic(inputs, n, P);
    if all.len() < vertices.len() {
        return Err("vertex/proof layout".into());
    }
    let mut rows = Vec::new();
    let mut info = Vec::new();
    for (r, v) in all.into_iter().zip(vertices.iter()) {
        if !verify(&r, inputs, P)
            || r.poly.keys().any(|m| {
                m.iter()
                    .enumerate()
                    .any(|(i, e)| i != v.variable && *e != 0)
            })
        {
            return Err("non-univariate or invalid projection".into());
        }
        if class == 3 || (class == 1 && v.variable == n - 1) || (class == 2 && v.variable != n - 1)
        {
            rows.push(r);
            info.push(serde_json::to_value(v)?);
        }
    }
    Ok((
        rows,
        json!({"all_constructed_vertices":vertices,"selected_vertices":info}),
    ))
}
pub(super) fn reference_file(path: &Path, n: usize) -> Result<Basis> {
    let v: Value = serde_json::from_slice(&fs::read(path)?)?;
    native_basis(&v["basis"], n)
}
pub(super) fn diagnostics(input: &Path, reference: &Path, out: &Path, cap: usize) -> Result<()> {
    let s: System = serde_json::from_slice(&fs::read(input)?)?;
    let n = s.variables.len();
    if s.prime != P || ![3, 4, 6].contains(&n) || cap != if n == 6 { 6 } else { 8 } {
        return Err("unregistered diagnostic dimensions/cap".into());
    }
    let inputs = s.decode()?;
    let basis = reference_file(reference, n)?;
    let refs: Vec<_> = basis
        .into_iter()
        .map(|poly| Row {
            poly,
            proof: Default::default(),
        })
        .collect();
    let start = Instant::now();
    let (run, archive) = trace(&inputs, n, P, &refs, cap, 700);
    let trace_ns = ns(start);
    if !archive
        .iter()
        .all(|(d, r)| degree(&r.poly) <= *d && verify(r, &inputs, P))
    {
        return Err("trace identity mismatch".into());
    }
    save(
        &out.with_extension("trace.json"),
        &json!({"run":run,"rows":archive.iter().map(|(d,r)|json!({"degree":d,"row":wire(r)})).collect::<Vec<_>>(),"trace_ns":trace_ns,"reference_use":"stopping oracle only, never injected"}),
    )?;
    let start = Instant::now();
    let h = hilbert(&inputs, n, P, cap, 700);
    let hilbert_ns = ns(start);
    save(
        &out.with_extension("hilbert.json"),
        &json!({"hilbert":h,"hilbert_ns":hilbert_ns}),
    )?;
    println!("DIAGNOSTICS_OK");
    Ok(())
}

fn cell(
    r: &mut Runner,
    out: &Path,
    n: usize,
    class: usize,
    base_fixture: &workload::Fixture,
    base_basis: &mut Option<Basis>,
) -> Result<Value> {
    r.case = Instant::now();
    let all = Instant::now();
    fs::create_dir(out)?;
    let start = Instant::now();
    let f = workload::generate_graph(P, n, 7, &edges(n, class))?;
    let generation_ns = ns(start);
    if f.witness != base_fixture.witness
        || f.public_scalars != base_fixture.public_scalars
        || !workload::witness_check(&f.system, &f.witness)
    {
        return Err("point/scalar identity or witness mismatch".into());
    }
    save(&out.join("public-fixture.json"), &f)?;
    let original = f.system.decode()?;
    let start = Instant::now();
    let (rows, info) = selected(&original, n, class)?;
    save(
        &out.join("identities.json"),
        &json!({"rows":rows.iter().map(wire).collect::<Vec<_>>(),"vertices":info}),
    )?;
    let mut system = f.system.clone();
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
    let construction_ns = ns(start);
    let start = Instant::now();
    let input_hash = prepare(out, &system)?;
    write_new(
        &out.join("identities.sing"),
        identity_script(&f.system, &original, &rows).as_bytes(),
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
    let identities_ok = status.success()
        && fs::read(out.join("identity-check.stderr"))?.is_empty()
        && std::str::from_utf8(&bytes)?.trim() == "IDENTITIES_OK";
    let identity_check_ns = ns(start);
    let mut failures = Vec::new();
    if !identities_ok {
        failures.push("fresh Singular identity check failed".to_string());
    }
    let start = Instant::now();
    let reference = r.references(out, out, &system, 0);
    let reference_ns = ns(start);
    let basis = match reference {
        Ok((b, _)) => Some(b),
        Err(e) => {
            failures.push(format!("references: {e}"));
            None
        }
    };
    if class == 0 {
        *base_basis = basis.clone();
    }
    let same_ideal = base_basis.as_ref().zip(basis.as_ref()).map(|(a, b)| a == b);
    let empty = Basis::new();
    let start = Instant::now();
    let sample = r.sample(
        Arm::F4,
        out,
        &out.join("f4"),
        n,
        basis.as_ref().unwrap_or(&empty),
        &input_hash,
    );
    let checked_f4_ns = ns(start);
    if let Err(e) = sample {
        failures.push(format!("F4: {e}"));
    }
    let mut diagnostic_ns = 0;
    let mut diagnostics_ok = false;
    if basis.is_some() {
        let start = Instant::now();
        let exe = std::env::current_exe()?;
        let attempt = r.process(
            &exe,
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
            Ok((status, b, _)) => {
                diagnostics_ok = status.success()
                    && std::str::from_utf8(&b)?.trim() == "DIAGNOSTICS_OK"
                    && fs::read(out.join("diagnostic-process.stderr"))?.is_empty();
                if !diagnostics_ok {
                    failures.push(format!("diagnostic incomplete: {status}"));
                }
            }
            Err(e) => failures.push(format!("diagnostic: {e}")),
        }
    } else {
        failures.push("diagnostic unavailable: no completed reference".into());
    }
    let paid_ns = [
        generation_ns,
        construction_ns,
        preparation_ns,
        identity_check_ns,
        reference_ns,
        checked_f4_ns,
    ]
    .into_iter()
    .try_fold(0u64, |a, b| a.checked_add(b).ok_or("paid overflow"))?;
    Ok(
        json!({"n":n,"class":CLASSES[class],"class_index":class,"input_sha256":input_hash,"point_scalar_identity":true,"input_equations":system.polynomials.len(),"added_projection_rows":rows.len(),"same_affine_ideal_as_star":same_ideal,"identities_ok":identities_ok,"diagnostics_ok":diagnostics_ok,"generation_ns":generation_ns,"construction_ns":construction_ns,"preparation_ns":preparation_ns,"identity_check_ns":identity_check_ns,"reference_ns":reference_ns,"checked_f4_ns":checked_f4_ns,"paid_ns":paid_ns,"separate_diagnostic_ns":diagnostic_ns,"case_wall_ns":ns(all),"failures":failures}),
    )
}
pub(super) fn run(out: &Path, config: &Path, mode: &str) -> Result<()> {
    let dims: &[usize] = match mode {
        "qualification" => &[3],
        "development" => &[3, 4, 6],
        _ => return Err("fixed qualification/development modes only".into()),
    };
    let c: Config = serde_json::from_slice(&fs::read(config)?)?;
    if c.cpu != "0" {
        return Err("fixed CPU0".into());
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
    let mut cells = Vec::new();
    for &n in dims {
        let base = workload::generate_graph(P, n, 7, &edges(n, 0))?;
        let mut basis = None;
        for (class, name) in CLASSES.iter().enumerate() {
            let attempt_start = Instant::now();
            let dir = out.join(format!("n{n}-{name}"));
            let v = match cell(&mut r, &dir, n, class, &base, &mut basis) {
                Ok(v) => v,
                Err(e) => {
                    json!({"n":n,"class":name,"class_index":class,"failures":[format!("cell: {e}")],"same_affine_ideal_as_star":null,"case_wall_ns":ns(attempt_start),"paid_ns":null})
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
        &json!({"mode":mode,"cells":cells,"pins_unchanged":pins==after,"capture_wall_ns":ns(r.global),"scope":"18 fixed development cells; single calls descriptive, no speedup admission"}),
    )
}

pub(super) fn report(raw: &Path, monitor: &Path, out: &Path) -> Result<()> {
    let summary: Value = serde_json::from_slice(&fs::read(raw.join("summary.json"))?)?;
    let dims: &[usize] = if summary["mode"] == "qualification" {
        &[3]
    } else if summary["mode"] == "development" {
        &[3, 4, 6]
    } else {
        return Err("unknown study mode".into());
    };
    let cells = summary["cells"]
        .as_array()
        .filter(|a| a.len() == 6 * dims.len())
        .ok_or("missing fixed cells")?;
    if summary["pins_unchanged"] != true
        || fs::read(raw.join("pins-before.json"))? != fs::read(raw.join("pins-after.json"))?
    {
        return Err("pins changed".into());
    }
    let monitor_text = fs::read_to_string(monitor)?;
    let logs: Vec<Value> = monitor_text
        .lines()
        .map(serde_json::from_str)
        .collect::<std::result::Result<_, _>>()?;
    // Same default monitor rule as the fresh comparison, plus fixed CPU admission.
    let m = logs.first().ok_or("missing isolation monitor")?;
    if m["contended_samples"] != 0 || m["exit_status"] != 0 || m["reserved_cpus"] != json!([0]) {
        return Err("isolation rejected".into());
    }
    let mut audited = Vec::new();
    let mut text=String::from("# Fixed development relation ablations\n\nSingle bounded calls on public known-answer seed7 fixtures. No statistical speedup admission. Exact F4 observed degree, affine closure threshold, and top-form Hilbert first vanishing are different quantities. Missing/capped values remain null. Unsigned localL0. [pcc:unverified]\n\n| n | Class | Added | Same ideal | F4 degree | Max rows | Max columns | Affine closure | Top-form vanishing | Paid ms |\n|---|---|---:|---|---:|---:|---:|---:|---:|---:|\n");
    for (i, v) in cells.iter().enumerate() {
        let n = dims[i / 6];
        let class = i % 6;
        let dir = raw.join(format!("n{n}-{}", CLASSES[class]));
        if v["n"] != n || v["class"] != CLASSES[class] || v["class_index"] != class {
            return Err("fixed cell order".into());
        }
        let recorded: Value = serde_json::from_slice(&fs::read(dir.join("cell.json"))?)?;
        if recorded != *v {
            return Err("summary/cell mismatch".into());
        }
        if v.get("input_sha256").is_some() {
            let bytes = fs::read(dir.join("system.json"))?;
            if v["input_sha256"] != hash(&bytes) {
                return Err("input digest".into());
            }
            let s: System = serde_json::from_slice(&bytes)?;
            let inputs = s.decode()?;
            let fixture: workload::Fixture =
                serde_json::from_slice(&fs::read(dir.join("public-fixture.json"))?)?;
            let expected = workload::generate_graph(P, n, 7, &edges(n, class))?;
            if serde_json::to_value(&fixture)? != serde_json::to_value(expected)?
                || !workload::witness_check(&s, &fixture.witness)
            {
                return Err("fixture/witness identity".into());
            }
            let certs: Value = serde_json::from_slice(&fs::read(dir.join("identities.json"))?)?;
            let original = fixture.system.decode()?;
            let rows = certs["rows"].as_array().ok_or("identity rows")?;
            for c in rows {
                if !verify(&row(c)?, &original, P) {
                    return Err("projection certificate".into());
                }
            }
            let mut expected_inputs = original;
            for c in rows {
                expected_inputs.push(row(c)?.poly);
            }
            if inputs != expected_inputs {
                return Err("added inputs differ from certified rows".into());
            }
            if v["added_projection_rows"] != rows.len() || v["input_equations"] != inputs.len() {
                return Err("equation/projection counts".into());
            }
            if v["identities_ok"] == true
                && (fs::read_to_string(dir.join("identity-check.stdout"))?.trim()
                    != "IDENTITIES_OK"
                    || !fs::read(dir.join("identity-check.stderr"))?.is_empty())
            {
                return Err("raw identity marker".into());
            }
            let fee = [
                "generation_ns",
                "construction_ns",
                "preparation_ns",
                "identity_check_ns",
                "reference_ns",
                "checked_f4_ns",
            ]
            .into_iter()
            .try_fold(0u64, |a, k| {
                a.checked_add(v[k].as_u64().ok_or("missing fee")?)
                    .ok_or("fee overflow")
            })?;
            if v["paid_ns"] != fee {
                return Err("paid fee sum".into());
            }
            if dir.join("references.json").exists() {
                let reference = reference_file(&dir.join("references.json"), n)?;
                for alg in ["std", "slimgb", "msolve"] {
                    let b = if alg == "msolve" {
                        msolve_basis(
                            &fs::read_to_string(dir.join("msolve-basis.txt"))?,
                            &s.variables,
                        )?
                    } else {
                        singular_basis(
                            &fs::read_to_string(dir.join(format!("setup-{alg}.stdout")))?,
                            n,
                        )?
                    };
                    if b != reference {
                        return Err("raw mature basis mismatch".into());
                    }
                }
                let base = reference_file(&raw.join(format!("n{n}-star/references.json")), n).ok();
                let same = base.map(|b| b == reference);
                if v["same_affine_ideal_as_star"] != json!(same) {
                    return Err("ideal equality label".into());
                }
                if let Ok(bytes) = fs::read(dir.join("f4.call.json")) {
                    let f: Value = serde_json::from_slice(&bytes)?;
                    let raw_f: Value = serde_json::from_slice(&fs::read(dir.join("f4.stdout"))?)?;
                    if raw_f != f["worker_output"] {
                        return Err("raw F4/call mismatch".into());
                    }
                    if f["complete"] == true
                        && native_basis(&f["worker_output"]["basis"], n)? != reference
                    {
                        return Err("F4 full basis mismatch".into());
                    }
                }
            }
            if dir.join("diagnostic.trace.json").exists() {
                let t: Value =
                    serde_json::from_slice(&fs::read(dir.join("diagnostic.trace.json"))?)?;
                for a in t["rows"].as_array().ok_or("trace rows")? {
                    if !verify(&row(&a["row"])?, &inputs, P) {
                        return Err("trace certificate".into());
                    }
                }
            }
        }
        let f: Value = fs::read(dir.join("f4.call.json"))
            .ok()
            .and_then(|b| serde_json::from_slice(&b).ok())
            .unwrap_or(Value::Null);
        let t: Value = fs::read(dir.join("diagnostic.trace.json"))
            .ok()
            .and_then(|b| serde_json::from_slice(&b).ok())
            .unwrap_or(Value::Null);
        let h: Value = fs::read(dir.join("diagnostic.hilbert.json"))
            .ok()
            .and_then(|b| serde_json::from_slice(&b).ok())
            .unwrap_or(Value::Null);
        let o = &f["worker_output"];
        let paid = v["paid_ns"].as_u64().map(|x| x as f64 / 1e6);
        text += &format!(
            "|{n}|{}|{}|{}|{}|{}|{}|{}|{}|{}|\n",
            CLASSES[class],
            v["added_projection_rows"],
            v["same_affine_ideal_as_star"],
            o["observed_solving_degree"],
            o["f4_observations"]["max_rows"],
            o["f4_observations"]["max_columns"],
            t["run"]["solving_degree"],
            h["hilbert"]["top_form_d_reg"],
            paid.map(|p| format!("{p:.6}")).unwrap_or("null".into())
        );
        audited.push(json!({"cell":v,"f4":o,"affine_trace":t["run"],"top_form_hilbert":h,"trace_rows_checked":t["rows"].as_array().map(Vec::len)}));
    }
    text+=&format!("\nFlow: planned {}, attempted {}, cells with recorded failures {}. All rows retained; no statistical pooling. [pcc:unverified]\n",cells.len(),cells.len(),cells.iter().filter(|v|v["failures"].as_array().is_none_or(|f|!f.is_empty())).count());
    for v in cells {
        if v["failures"].as_array().is_some_and(|f| !f.is_empty()) {
            text += &format!(
                "\nRecorded failure n{} {}: {} [pcc:unverified]\n",
                v["n"], v["class"], v["failures"]
            );
        }
    }
    text+="\nPaid costs include generation, preparation, full projection construction even for subset arms, exact identity checking, fresh3CAS references and the complete checked F4 call. Reference/oracle construction and separate diagnostic costs are retained in audit.json; diagnostic time is not production solver speed. F4 matrix nonzeros/fill are unavailable when the unchanged worker does not emit them; affine diagnostic nonzeros are not substitutes. Geometric additions may change the ideal; equal checked complete canonical bases determine the label. Capped closure/Hilbert is not an intrinsic regularity or impossibility result. Star leaf projections may be absent; duplicate/effectively identical arms remain. [pcc:unverified]\n";
    fs::create_dir(out)?;
    save(
        &out.join("audit.json"),
        &json!({"mode":summary["mode"],"monitor_valid":true,"pins_unchanged":true,"raw_identities_and_completed_bases_replayed":true,"cells":audited}),
    )?;
    write_new(&out.join("report.md"), text.as_bytes())?;
    let file = out.join("degrees.svg");
    let plot = SVGBackend::new(&file, (1400, 650)).into_drawing_area();
    plot.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&plot)
        .caption(
            "Observed F4 degree by fixed cell (missing stays absent)",
            ("sans-serif", 24),
        )
        .margin(24)
        .x_label_area_size(50)
        .y_label_area_size(60)
        .build_cartesian_2d(0usize..cells.len(), 0u32..13u32)?;
    chart
        .configure_mesh()
        .x_desc("n3, n4, n6; six protocol classes in order")
        .y_desc("Observed F4 degree")
        .draw()?;
    chart.draw_series(audited.iter().enumerate().filter_map(|(i, a)| {
        a["f4"]["observed_solving_degree"]
            .as_u64()
            .map(|d| Circle::new((i, d as u32), 5, BLUE.filled()))
    }))?;
    plot.present()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn same_points_different_edges() {
        for n in [3, 4, 6] {
            let a = workload::generate_graph(P, n, 7, &edges(n, 0)).unwrap();
            for c in [4, 5] {
                let b = workload::generate_graph(P, n, 7, &edges(n, c)).unwrap();
                assert_eq!(a.witness, b.witness);
                assert_eq!(a.public_scalars, b.public_scalars);
            }
        }
    }
    #[test]
    fn projection_partition_and_certificates() {
        let f = workload::generate_graph(P, 3, 7, &edges(3, 0)).unwrap();
        let inputs = f.system.decode().unwrap();
        let (root, _) = selected(&inputs, 3, 1).unwrap();
        let (leaf, _) = selected(&inputs, 3, 2).unwrap();
        let (all, _) = selected(&inputs, 3, 3).unwrap();
        assert!(!root.is_empty());
        assert!(leaf.is_empty());
        assert_eq!(root.len() + leaf.len(), all.len());
        for r in all {
            assert!(verify(&row(&wire(&r)).unwrap(), &inputs, P));
            let mut bad = r.clone();
            *bad.poly.values_mut().next().unwrap() = 0;
            assert!(!verify(&bad, &inputs, P));
        }
    }
    #[test]
    fn duplicate_certificate_rejected() {
        assert!(row(&json!({"poly":[[[1],1],[[1],2]],"proof":[]})).is_err());
    }
    #[test]
    fn n3_geometric_duplicate_retained() {
        assert_eq!(edges(3, 4), edges(3, 5));
        assert_ne!(edges(4, 4), edges(4, 5));
    }
}

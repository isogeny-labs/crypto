//! Bounded diagnostic. Producers never read the retained root or cost records.
use super::*;
use std::collections::VecDeque;

const CATALOG_SHA: &str = "17ecb4c302526bc58e592466071e1c2704eaaf43a0a0ca1efe563e1bd69b3c10";
const SCOPE: &str = "Six fixed p37 seed0 polynomial systems; raw16 versus bounded degree-fall closure8, powers 1..3, original product degree <=16. Counted arithmetic only; diagnostic clocks, no relation collection, no target recovery, no complete pipeline or external/rho price.";

type DenseRow = (Vec<u64>, Vec<u64>);

fn actual_product_degree(polys: &[Poly], certificate: &Certificate) -> u16 {
    certificate
        .multipliers
        .iter()
        .zip(polys)
        .filter_map(|(h, g)| Some(poly_degree(h)? + poly_degree(g)?))
        .max()
        .unwrap_or(0)
}

fn closure(
    polys: &[Poly],
    a: u64,
    f: &mut Fp,
    deadline: Instant,
    working_degrees: &[u8],
    product_cap: u8,
    note: &mut impl FnMut(Value) -> R<()>,
) -> R<(Option<Certificate>, Vec<Value>)> {
    let targets = powers(a, f);
    let mut generators = Vec::new();
    for (i, poly) in polys.iter().enumerate() {
        if let Some(d) = poly_degree(poly) {
            need(
                d <= product_cap as u16,
                "original polynomial above product cap",
            )?;
            for m in monomials(product_cap - d as u8) {
                generators.push((i, m));
            }
        }
    }
    let gen_index: BTreeMap<_, _> = generators
        .iter()
        .enumerate()
        .map(|(j, &g)| (g, j))
        .collect();
    let mut attempts = Vec::new();
    for &d in working_degrees {
        need(Instant::now() < deadline, "cooperative deadline")?;
        let before = f.w;
        let mons = monomials(d);
        let index: BTreeMap<_, _> = mons.iter().enumerate().map(|(i, &m)| (m, i)).collect();
        let cols = mons.len();
        let mut queue: VecDeque<DenseRow> = VecDeque::new();
        for (ri, &(pi, shift)) in generators.iter().enumerate() {
            if poly_degree(&polys[pi]).unwrap() + degree(&shift) > d as u16 {
                continue;
            }
            let mut row = vec![0; cols];
            let mut rep = vec![0; generators.len()];
            rep[ri] = 1;
            for (&m, &c) in &polys[pi] {
                let shifted = std::array::from_fn(|i| m[i] + shift[i]);
                row[index[&shifted]] = c;
            }
            queue.push_back((row, rep));
        }
        let initial_rows = queue.len();
        let (mut processed, mut generated, mut refused, mut rank) = (0, 0, 0, 0);
        let mut basis: Vec<Option<DenseRow>> = vec![None; cols];
        note(json!({"phase":"closure-degree-start","working_degree":d,
            "initial_rows":initial_rows,"columns":cols,"representation_columns":generators.len(),
            "original_product_cap":product_cap,"work_so_far":f.w.json()}))?;
        while let Some((mut row, mut rep)) = queue.pop_front() {
            if Instant::now() >= deadline {
                note(
                    json!({"phase":"closure-timeout","working_degree":d,"rank":rank,
                    "processed_rows":processed,"pending_rows":queue.len()+1,
                    "representation_cap_refusals":refused,"work_so_far":f.w.json()}),
                )?;
                return Err("cooperative deadline".into());
            }
            processed += 1;
            for col in 0..cols {
                let scalar = row[col];
                if scalar == 0 {
                    continue;
                }
                if let Some((b, br)) = &basis[col] {
                    for j in col..cols {
                        if b[j] != 0 {
                            let v = f.mul(scalar, b[j]);
                            row[j] = f.sub(row[j], v);
                        }
                    }
                    for j in 0..rep.len() {
                        if br[j] != 0 {
                            let v = f.mul(scalar, br[j]);
                            rep[j] = f.sub(rep[j], v);
                        }
                    }
                    continue;
                }
                let inverse = f.inv(scalar);
                for v in &mut row[col..] {
                    if *v != 0 {
                        *v = f.mul(*v, inverse);
                    }
                }
                for v in &mut rep {
                    if *v != 0 {
                        *v = f.mul(*v, inverse);
                    }
                }
                // With columns in decreasing total degree the pivot has the
                // largest degree of this polynomial. Representations may have
                // larger original product degree and are bounded separately.
                if degree(&mons[col]) < d as u16 {
                    for variable in 0..3 {
                        let mut shifted_rep = vec![0; generators.len()];
                        let mut fits = true;
                        for (j, &c) in rep.iter().enumerate().filter(|(_, c)| **c != 0) {
                            let (pi, mut m) = generators[j];
                            m[variable] += 1;
                            if let Some(&at) = gen_index.get(&(pi, m)) {
                                shifted_rep[at] = c;
                            } else {
                                fits = false;
                                break;
                            }
                        }
                        if !fits {
                            refused += 1;
                            continue;
                        }
                        let mut shifted_row = vec![0; cols];
                        for (j, &c) in row.iter().enumerate().filter(|(_, c)| **c != 0) {
                            let mut m = mons[j];
                            m[variable] += 1;
                            shifted_row[index[&m]] = c;
                        }
                        queue.push_back((shifted_row, shifted_rep));
                        generated += 1;
                    }
                }
                basis[col] = Some((row, rep));
                rank += 1;
                break;
            }
        }
        let mut certificate = None;
        for (ki, target) in targets.iter().enumerate() {
            if poly_degree(target).unwrap_or(0) > d as u16 {
                continue;
            }
            let mut row = vec![0; cols];
            let mut weights = vec![0; generators.len()];
            for (&m, &v) in target {
                row[index[&m]] = v;
            }
            for col in 0..cols {
                let scalar = row[col];
                if scalar == 0 {
                    continue;
                }
                if let Some((b, br)) = &basis[col] {
                    for j in col..cols {
                        if b[j] != 0 {
                            let v = f.mul(scalar, b[j]);
                            row[j] = f.sub(row[j], v);
                        }
                    }
                    for j in 0..weights.len() {
                        if br[j] != 0 {
                            let v = f.mul(scalar, br[j]);
                            weights[j] = f.add(weights[j], v);
                        }
                    }
                }
            }
            if row.iter().all(|&v| v == 0) {
                let mut multipliers = vec![Poly::new(); 4];
                for (&(pi, m), &v) in generators.iter().zip(&weights) {
                    if v != 0 {
                        insert(&mut multipliers[pi], m, v, f);
                    }
                }
                let actual_degree = multipliers
                    .iter()
                    .zip(polys)
                    .filter_map(|(h, g)| Some(poly_degree(h)? + poly_degree(g)?))
                    .max()
                    .unwrap_or(0);
                need(
                    actual_degree <= product_cap as u16,
                    "representation product cap",
                )?;
                certificate = Some(Certificate {
                    d: actual_degree as u8,
                    k: ki as u8 + 1,
                    multipliers,
                });
                break;
            }
        }
        attempts.push(
            json!({"working_degree":d,"original_product_cap":product_cap,
            "initial_rows":initial_rows,"generated_rows":generated,"processed_rows":processed,
            "columns":cols,"representation_columns":generators.len(),"rank":rank,
            "representation_cap_refusals":refused,
            "found_power":certificate.as_ref().map(|c|c.k),
            "actual_certificate_product_degree":certificate.as_ref().map(|c|c.d),
            "work":f.w.minus(before).json()}),
        );
        note(json!({"phase":"closure-degree-completed","attempt":attempts.last().unwrap()}))?;
        if certificate.is_some() {
            return Ok((certificate, attempts));
        }
    }
    Ok((None, attempts))
}

fn bound_read(row: &Value, stem: &str) -> R<Value> {
    let path = Path::new(txt(&row[format!("{stem}_path")])?);
    need(
        sha(path)? == txt(&row[format!("{stem}_sha256")])?,
        "sealed input hash mismatch",
    )?;
    read(path)
}

fn prepare(catalog: &Path, digest: &str, old_run: &Path, out: &Path) -> R<()> {
    need(
        digest == CATALOG_SHA && sha(catalog)? == digest,
        "catalog hash",
    )?;
    need(!out.exists(), "fresh preparation directory required")?;
    std::fs::create_dir_all(out.join("inputs")).map_err(|e| e.to_string())?;
    let old = read(catalog)?;
    let mut workloads = Vec::new();
    for arm in ["phi-l0-active", "double-l0-active"] {
        let old_id = format!("p37-seed0-zero-{arm}");
        let original = arr(&old["workloads"])?
            .iter()
            .find(|r| r["id"] == old_id)
            .ok_or("missing selected arm")?;
        let pack = bound_read(original, "input")?;
        let reference = bound_read(original, "reference")?;
        let audit = bound_read(original, "audit")?;
        let public = bound_read(original, "public")?;
        need(
            audit["status"] == "pass"
                && audit["candidate_sha256"] == original["reference_sha256"]
                && audit["public_input_sha256"] == original["public_sha256"],
            "original independent audit binding",
        )?;
        let prior_path = old_run
            .join("cases")
            .join(&old_id)
            .join("first/record.json");
        let prior_hash = sha(&prior_path)?;
        let prior = read(&prior_path)?;
        need(
            prior["status"] == "complete" && prior["input_sha256"] == original["input_sha256"],
            "prior raw8 binding",
        )?;
        for index in 0..3 {
            let sys = &pack["systems"][index];
            need(num(&sys["index"])? == index as u64, "system order")?;
            let ref_sys = &reference["systems"][index];
            need(
                sys["equations"] == ref_sys["equations"] && sys["pair"] == ref_sys["pair"],
                "original system identity",
            )?;
            let ordinary = num(&sys["pair"][0])? as usize;
            need(
                sys["peeled_abscissa"] == public["base_coordinates"][ordinary][0][0],
                "peeled a binding",
            )?;
            let base_id = format!("p37-seed0-{arm}-sys{index}");
            let input_path = out.join("inputs").join(format!("{base_id}.json"));
            let input = json!({"schema":"twist-degree-closure-input/v1","id":base_id,
                "p":37,"seed":0,"arm":arm,"original_system_index":index,
                "pair":sys["pair"],"peeled_abscissa":sys["peeled_abscissa"],
                "equations":sys["equations"],"scope":"Equation-only producer input; no roots, outcomes, prior work or targets."});
            write_new(&input_path, &input)?;
            for mode in ["raw16", "closure8"] {
                let mut row = original.clone();
                let map = row.as_object_mut().ok_or("workload object")?;
                map.insert("id".into(), json!(format!("{base_id}-{mode}")));
                map.insert("mode".into(), json!(mode));
                map.insert("original_id".into(), json!(&old_id));
                map.insert("original_system_index".into(), json!(index));
                map.insert("original_input_path".into(), original["input_path"].clone());
                map.insert(
                    "original_input_sha256".into(),
                    original["input_sha256"].clone(),
                );
                map.insert("input_path".into(), json!(input_path));
                map.insert("input_sha256".into(), json!(sha(&input_path)?));
                map.insert("prior_path".into(), json!(prior_path));
                map.insert("prior_sha256".into(), json!(prior_hash));
                workloads.push(row);
            }
        }
    }
    need(workloads.len() == 12, "complete fixed diagnostic workload")?;
    let workload = json!({"schema":"twist-degree-closure-workload/v1","scope":SCOPE,
        "source_catalog_sha256":digest,"configurations":12,"producer_executions":24,
        "workloads":workloads});
    write_new(&out.join("workload.json"), &workload)?;
    let mut tsv = String::new();
    for row in arr(&workload["workloads"])? {
        tsv.push_str(&format!(
            "{}\t{}\t{}\t{}\n",
            txt(&row["id"])?,
            txt(&row["mode"])?,
            txt(&row["input_path"])?,
            txt(&row["input_sha256"])?
        ));
    }
    std::fs::write(out.join("workload.tsv"), tsv).map_err(|e| e.to_string())?;
    println!(
        "prepared 6 inputs and 12 configurations; workload {}",
        sha(&out.join("workload.json"))?
    );
    Ok(())
}

fn produce(input: &Path, digest: &str, mode: &str, out: &Path) -> R<()> {
    need(sha(input)? == digest, "producer input hash")?;
    need(["raw16", "closure8"].contains(&mode), "unknown mode")?;
    need(!out.exists(), "fresh producer directory required")?;
    std::fs::create_dir_all(out).map_err(|e| e.to_string())?;
    let pack = read(input)?;
    need(
        pack["schema"] == "twist-degree-closure-input/v1" && pack["p"] == 37,
        "input schema/prime",
    )?;
    let polys = arr(&pack["equations"])?
        .iter()
        .map(|v| parse_poly(v, 37))
        .collect::<R<Vec<_>>>()?;
    need(polys.len() == 4, "four equations required")?;
    let a = num(&pack["peeled_abscissa"])?;
    need(a < 37, "canonical peeled abscissa")?;
    let started = Instant::now();
    let deadline = started + Duration::from_secs(1180);
    let mut journal = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(out.join("journal.jsonl"))
        .map_err(|e| e.to_string())?;
    let mut note = |v: Value| -> R<()> {
        writeln!(journal, "{v}").map_err(|e| e.to_string())?;
        journal.sync_data().map_err(|e| e.to_string())
    };
    note(
        json!({"phase":"system-pending","mode":mode,"input_sha256":digest,"cooperative_seconds":1180}),
    )?;
    let mut f = Fp::new(37);
    let result = if mode == "raw16" {
        search(&polys, a, &mut f, deadline, 16, &mut note)
    } else {
        closure(&polys, a, &mut f, deadline, &[4, 5, 6, 7, 8], 16, &mut note)
    };
    let (certificate, attempts) = match result {
        Ok(v) => v,
        Err(e) => {
            write_new(
                &out.join("partial.json"),
                &json!({"schema":"twist-degree-closure-partial/v1",
                "status":"incomplete","reason":e,"input_sha256":digest,"mode":mode,"work":f.w.json(),"scope":SCOPE}),
            )?;
            return Err(e);
        }
    };
    let search_work = f.w;
    let mut check = Fp::new(37);
    if let Some(c) = &certificate {
        verify(&polys, a, c, &mut check)?;
    }
    let actual_degree = certificate
        .as_ref()
        .map(|c| actual_product_degree(&polys, c));
    let record = json!({"schema":"twist-degree-closure-producer/v1","status":"complete",
        "id":format!("{}-{mode}",txt(&pack["id"] )?),"input_sha256":digest,"p":37,
        "mode":mode,"original_system_index":pack["original_system_index"],"arm":pack["arm"],
        "certificate":certificate.as_ref().map(Certificate::json),"attempts":attempts,
        "actual_certificate_product_degree":actual_degree,
        "search_work":search_work.json(),"certificate_check_work":check.w.json(),
        "diagnostic_elapsed_ns":started.elapsed().as_nanos(),"complete_end_to_end_cost":Value::Null,"scope":SCOPE});
    note(
        json!({"phase":"system-completed","certified":certificate.is_some(),"search_work":search_work.json(),"check_work":check.w.json()}),
    )?;
    write_new(&out.join("record.json"), &record)
}

fn get_row(workload: &Path, digest: &str, id: &str) -> R<Value> {
    need(sha(workload)? == digest, "workload hash")?;
    let w = read(workload)?;
    need(
        w["schema"] == "twist-degree-closure-workload/v1" && arr(&w["workloads"])?.len() == 12,
        "workload schema/count",
    )?;
    arr(&w["workloads"])?
        .iter()
        .find(|r| r["id"] == id)
        .cloned()
        .ok_or("unknown configuration".into())
}

fn work_sum(attempts: &[Value], prelude: Work) -> R<Work> {
    let mut out = prelude;
    for a in attempts {
        out.muls = out
            .muls
            .checked_add(num(&a["work"]["modular_multiplications"])?)
            .ok_or("work overflow")?;
        out.adds = out
            .adds
            .checked_add(num(&a["work"]["modular_additions"])?)
            .ok_or("work overflow")?;
        out.invs = out
            .invs
            .checked_add(num(&a["work"]["inversion_calls"])?)
            .ok_or("work overflow")?;
    }
    Ok(out)
}

fn audit(workload: &Path, digest: &str, id: &str, candidate: &Path, out: &Path) -> R<()> {
    need(!out.exists(), "fresh audit directory required")?;
    let row = get_row(workload, digest, id)?;
    let pack = bound_read(&row, "input")?;
    let original = bound_read(&row, "original_input")?;
    let reference = bound_read(&row, "reference")?;
    let original_audit = bound_read(&row, "audit")?;
    let public = bound_read(&row, "public")?;
    let prior = bound_read(&row, "prior")?;
    let ix = num(&row["original_system_index"])? as usize;
    let sys = &reference["systems"][ix];
    need(
        original_audit["status"] == "pass"
            && original_audit["candidate_sha256"] == row["reference_sha256"]
            && original_audit["public_input_sha256"] == row["public_sha256"],
        "source independent audit binding",
    )?;
    need(
        prior["status"] == "complete" && prior["input_sha256"] == row["original_input_sha256"],
        "prior raw8 source binding",
    )?;
    need(
        pack["equations"] == sys["equations"] && pack["pair"] == sys["pair"],
        "original equations/pair",
    )?;
    need(
        pack["equations"] == original["systems"][ix]["equations"]
            && pack["peeled_abscissa"] == original["systems"][ix]["peeled_abscissa"],
        "original equation-only packet",
    )?;
    let ordinary = num(&pack["pair"][0])? as usize;
    need(
        pack["peeled_abscissa"] == public["base_coordinates"][ordinary][0][0],
        "public peeled abscissa",
    )?;
    let candidate_path = candidate.join("record.json");
    let rec = read(&candidate_path)?;
    need(
        rec["schema"] == "twist-degree-closure-producer/v1"
            && rec["status"] == "complete"
            && rec["id"] == id
            && rec["mode"] == row["mode"]
            && rec["arm"] == row["arm"]
            && rec["original_system_index"] == ix
            && rec["input_sha256"] == row["input_sha256"],
        "candidate identity",
    )?;
    let attempts = arr(&rec["attempts"])?;
    need(!attempts.is_empty(), "missing degree attempts")?;
    let mode = txt(&row["mode"])?;
    let max = if mode == "raw16" { 16 } else { 8 };
    for (j, attempt) in attempts.iter().enumerate() {
        let d = num(&attempt[if mode == "raw16" {
            "degree"
        } else {
            "working_degree"
        }])?;
        need(d == 4 + j as u64 && d <= max, "degree prefix")?;
        need(
            num(&attempt["rank"])? <= num(&attempt["columns"])?,
            "polynomial rank bound",
        )?;
        need(
            j + 1 == attempts.len() || attempt["found_power"].is_null(),
            "continued after certificate",
        )?;
    }
    if mode == "raw16" {
        let old_attempts = arr(&prior["systems"][ix]["attempts"])?;
        need(
            old_attempts.len() == 5 && prior["systems"][ix]["certificate"].is_null(),
            "prior raw8 completed negative",
        )?;
        need(
            attempts.len() >= 5,
            "raw16 must reproduce all old raw8 degrees",
        )?;
        need(
            attempts[..5] == old_attempts[..],
            "raw4..8 exact rank/counter prefix mismatch",
        )?;
    } else {
        for attempt in attempts {
            need(attempt["original_product_cap"] == 16, "closure product cap")?;
            need(
                num(&attempt["processed_rows"])?
                    == num(&attempt["initial_rows"])? + num(&attempt["generated_rows"])?,
                "closure queue not exhausted",
            )?;
        }
    }
    let polys = arr(&pack["equations"])?
        .iter()
        .map(|v| parse_poly(v, 37))
        .collect::<R<Vec<_>>>()?;
    let a = num(&pack["peeled_abscissa"])?;
    let mut prelude = Fp::new(37);
    let _ = powers(a, &mut prelude);
    need(
        work_sum(attempts, prelude.w)?.json() == rec["search_work"],
        "search work does not telescope",
    )?;
    let mut check = Fp::new(37);
    let mut roots_checked = 0;
    if rec["certificate"].is_null() {
        need(
            rec["actual_certificate_product_degree"].is_null(),
            "negative actual degree must be null",
        )?;
        need(
            attempts.len() == (max - 3) as usize
                && attempts.last().unwrap()["found_power"].is_null(),
            "bounded negative coverage",
        )?;
    } else {
        let certificate = Certificate::parse(&rec["certificate"], 37)?;
        verify(&polys, a, &certificate, &mut check)?;
        need(
            rec["actual_certificate_product_degree"] == actual_product_degree(&polys, &certificate),
            "actual original product degree mismatch",
        )?;
        if mode == "raw16" {
            need(
                attempts.last().unwrap()["degree"] == certificate.d,
                "raw search degree cap binding",
            )?;
        }
        need(
            attempts.last().unwrap()["found_power"] == certificate.k,
            "power stop binding",
        )?;
        if mode == "closure8" {
            need(
                attempts.last().unwrap()["actual_certificate_product_degree"] == certificate.d,
                "actual product degree binding",
            )?;
        }
        need(
            arr(&sys["accepted_representative_relations"])?.is_empty(),
            "certified system has accepted relation",
        )?;
        for root in arr(&sys["field_roots"])? {
            let point: [u64; 3] =
                serde_json::from_value(root.clone()).map_err(|e| e.to_string())?;
            need(
                polys.iter().all(|p| polynomial_value(p, point, 37) == 0),
                "retained root does not solve original equations",
            )?;
            let qpoly = q(a, &mut Fp::new(37));
            need(
                polynomial_value(&qpoly, point, 37) == 0,
                "certificate rejects a nonrepeated retained root",
            )?;
            roots_checked += 1;
        }
    }
    need(
        check.w.json() == rec["certificate_check_work"],
        "independent sparse-check work mismatch",
    )?;
    let summary = json!({"schema":"twist-degree-closure-audit/v1","status":"pass","id":id,
        "mode":mode,"arm":row["arm"],"original_system_index":ix,"input_sha256":row["input_sha256"],
        "candidate_sha256":sha(&candidate_path)?,"reference_sha256":row["reference_sha256"],"prior_raw8_sha256":row["prior_sha256"],
        "certified":!rec["certificate"].is_null(),"power":rec["certificate"]["power"],
        "actual_certificate_product_degree":rec["actual_certificate_product_degree"],
        "certificate_product_degree_cap":rec["certificate"]["degree_cap"],
        "last_working_degree":attempts.last().unwrap()[if mode=="raw16" {"degree"} else {"working_degree"}],
        "search_work":rec["search_work"],"certificate_check_work":rec["certificate_check_work"],
        "retained_roots_checked":roots_checked,"raw4_through8_exactly_reproduced":mode=="raw16",
        "complete_end_to_end_cost":Value::Null,"scope":SCOPE});
    std::fs::create_dir_all(out).map_err(|e| e.to_string())?;
    write_new(&out.join("summary.json"), &summary)
}

fn parity(first: &Path, repeat: &Path, out: &Path) -> R<()> {
    need(!out.exists(), "fresh parity path required")?;
    let mut a = read(first)?;
    let mut b = read(repeat)?;
    a.as_object_mut()
        .ok_or("record object")?
        .remove("diagnostic_elapsed_ns");
    b.as_object_mut()
        .ok_or("record object")?
        .remove("diagnostic_elapsed_ns");
    need(a == b, "deterministic parity mismatch")?;
    write_new(
        out,
        &json!({"schema":"twist-degree-closure-parity/v1","status":"pass",
        "first_sha256":sha(first)?,"repeat_sha256":sha(repeat)?,"ignored_only":["diagnostic_elapsed_ns"]}),
    )
}

fn summarize(run: &Path, workload: &Path, digest: &str, out: &Path) -> R<()> {
    need(!out.exists(), "fresh analysis directory required")?;
    need(sha(workload)? == digest, "workload hash")?;
    let w = read(workload)?;
    need(
        arr(&w["workloads"])?.len() == 12,
        "all twelve configurations required",
    )?;
    let mut rows = Vec::new();
    let mut complete = true;
    for row in arr(&w["workloads"])? {
        let id = txt(&row["id"])?;
        let case = run.join("cases").join(id);
        let check = (|| -> R<Value> {
            for suffix in ["first", "audit", "aa", "parity"] {
                let path = run.join("metadata").join(format!("{id}-{suffix}.exit"));
                need(
                    std::fs::read_to_string(&path)
                        .map_err(|e| e.to_string())?
                        .trim()
                        == "0",
                    "phase exit is not zero",
                )?;
            }
            let a = read(&case.join("audit/summary.json"))?;
            let p = read(&case.join("parity.json"))?;
            need(
                a["status"] == "pass" && p["status"] == "pass" && a["id"] == id,
                "audit/parity status",
            )?;
            need(
                a["candidate_sha256"] == sha(&case.join("first/record.json"))?
                    && p["first_sha256"] == a["candidate_sha256"]
                    && p["repeat_sha256"] == sha(&case.join("aa/record.json"))?,
                "audit/parity record binding",
            )?;
            let mut a = a;
            a["status"] = json!("verified-first-aa");
            a["audit_sha256"] = json!(sha(&case.join("audit/summary.json"))?);
            a["parity_sha256"] = json!(sha(&case.join("parity.json"))?);
            Ok(a)
        })();
        rows.push(match check {Ok(v)=>v,Err(e)=>{complete=false;json!({"id":id,"status":"unverified","reason":e,
            "certified":Value::Null,"search_work":Value::Null,"complete_end_to_end_cost":Value::Null})}});
    }
    std::fs::create_dir_all(out).map_err(|e| e.to_string())?;
    write_new(
        &out.join("summary.json"),
        &json!({"schema":"twist-degree-closure-panel/v1",
        "status":if complete {"complete"} else {"partial"},"workload_sha256":digest,
        "configurations":12,"expected_producer_executions":24,"rows":rows,
        "complete_end_to_end_cost":Value::Null,"scope":SCOPE}),
    )?;
    need(complete, "partial diagnostic panel")
}

fn run() -> R<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.iter().map(String::as_str).collect::<Vec<_>>().as_slice() {
        ["prepare",c,h,r,o]=>prepare(Path::new(c),h,Path::new(r),Path::new(o)),
        ["produce",p,h,m,o]=>produce(Path::new(p),h,m,Path::new(o)),
        ["audit",w,h,id,c,o]=>audit(Path::new(w),h,id,Path::new(c),Path::new(o)),
        ["parity",a,b,o]=>parity(Path::new(a),Path::new(b),Path::new(o)),
        ["summarize",r,w,h,o]=>summarize(Path::new(r),Path::new(w),h,Path::new(o)),
        _=>Err("usage: prepare CATALOG SHA OLD_RUN OUT | produce INPUT SHA MODE OUT | audit WORKLOAD SHA ID CAND_DIR OUT | parity FIRST AA OUT | summarize RUN WORKLOAD SHA OUT".into())
    }
}
pub fn entry() {
    if let Err(e) = run() {
        eprintln!("degree closure diagnostic rejected: {e}");
        std::process::exit(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn poly(terms: &[(M, u64)]) -> Poly {
        terms.iter().copied().collect()
    }
    fn deadline() -> Instant {
        Instant::now() + Duration::from_secs(20)
    }
    #[test]
    fn closure_uses_higher_original_product_degree_than_working_degree() {
        // f1=x², f2=x²-x, f3=xy-z: the degree fall x generates xy,
        // then z. The representation y(f1-f2)-f3 has product degree 3.
        let polys = vec![
            poly(&[([2, 0, 0], 1)]),
            poly(&[([2, 0, 0], 1), ([1, 0, 0], 36)]),
            poly(&[([1, 1, 0], 1), ([0, 0, 1], 36)]),
            Poly::new(),
        ];
        let (cert, attempts) = closure(
            &polys,
            0,
            &mut Fp::new(37),
            deadline(),
            &[2],
            3,
            &mut |_| Ok(()),
        )
        .unwrap();
        let cert = cert.unwrap();
        assert_eq!(cert.k, 1);
        assert_eq!(cert.d, 3);
        assert_eq!(attempts[0]["working_degree"], 2);
        verify(&polys, 0, &cert, &mut Fp::new(37)).unwrap();
        assert!(num(&attempts[0]["generated_rows"]).unwrap() > 0);
    }
    #[test]
    fn nonradical_requires_power_and_tampering_is_rejected() {
        let polys = vec![
            poly(&[([0, 0, 2], 1)]),
            Poly::new(),
            Poly::new(),
            Poly::new(),
        ];
        let (cert, _) =
            search(&polys, 0, &mut Fp::new(37), deadline(), 4, &mut |_| Ok(())).unwrap();
        let mut cert = cert.unwrap();
        assert_eq!(cert.k, 2);
        verify(&polys, 0, &cert, &mut Fp::new(37)).unwrap();
        cert.multipliers[0].insert([0, 0, 0], 2);
        assert!(verify(&polys, 0, &cert, &mut Fp::new(37)).is_err());
    }
    #[test]
    fn common_nonrepeated_root_cannot_be_certified() {
        let polys = vec![
            poly(&[([0, 0, 1], 1), ([0, 0, 0], 36)]),
            Poly::new(),
            Poly::new(),
            Poly::new(),
        ];
        let (raw, _) = search(&polys, 0, &mut Fp::new(37), deadline(), 4, &mut |_| Ok(())).unwrap();
        let (closed, _) = closure(
            &polys,
            0,
            &mut Fp::new(37),
            deadline(),
            &[4],
            4,
            &mut |_| Ok(()),
        )
        .unwrap();
        assert!(raw.is_none() && closed.is_none());
    }
    #[test]
    fn raw_search_cap_is_not_reported_as_actual_product_degree() {
        let polys = vec![
            poly(&[([0, 0, 1], 1)]),
            Poly::new(),
            Poly::new(),
            Poly::new(),
        ];
        let (certificate, _) =
            search(&polys, 0, &mut Fp::new(37), deadline(), 4, &mut |_| Ok(())).unwrap();
        let certificate = certificate.unwrap();
        assert_eq!(certificate.d, 4);
        assert_eq!(actual_product_degree(&polys, &certificate), 1);
        verify(&polys, 0, &certificate, &mut Fp::new(37)).unwrap();
    }
    #[test]
    fn sparse_verifier_enforces_original_product_cap() {
        let polys = vec![
            poly(&[([0, 0, 2], 1)]),
            Poly::new(),
            Poly::new(),
            Poly::new(),
        ];
        let cert = Certificate {
            d: 1,
            k: 2,
            multipliers: vec![
                poly(&[([0, 0, 0], 1)]),
                Poly::new(),
                Poly::new(),
                Poly::new(),
            ],
        };
        assert!(verify(&polys, 0, &cert, &mut Fp::new(37))
            .unwrap_err()
            .contains("degree exceeds"));
    }
}

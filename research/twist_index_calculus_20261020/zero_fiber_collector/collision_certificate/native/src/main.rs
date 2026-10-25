//! Exact sufficient rejection certificates. No F4, group or scalar recovery.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

type M = [u8; 3];
type Poly = BTreeMap<M, u64>;
type R<T> = Result<T, String>;
const PRIMES: [u64; 7] = [37, 73, 97, 157, 229, 241, 337];
const WORKLOAD_SHA: &str = "47604fa481c5c26ef17e9e9a41494461556c1b9356e20e9652a499926a499b77";

fn need(ok: bool, why: &str) -> R<()> {
    if ok {
        Ok(())
    } else {
        Err(why.into())
    }
}
fn num(v: &Value) -> R<u64> {
    v.as_u64()
        .ok_or_else(|| format!("missing unsigned integer: {v}"))
}
fn arr(v: &Value) -> R<&Vec<Value>> {
    v.as_array().ok_or_else(|| "missing array".into())
}
fn txt(v: &Value) -> R<&str> {
    v.as_str().ok_or_else(|| "missing string".into())
}
fn read(p: &Path) -> R<Value> {
    serde_json::from_slice(&std::fs::read(p).map_err(|e| format!("{}: {e}", p.display()))?)
        .map_err(|e| format!("{}: {e}", p.display()))
}
fn sha(p: &Path) -> R<String> {
    Ok(hex::encode(Sha256::digest(
        std::fs::read(p).map_err(|e| e.to_string())?,
    )))
}
fn write_new(p: &Path, v: &Value) -> R<()> {
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(p)
        .map_err(|e| e.to_string())?;
    f.write_all(&serde_json::to_vec(v).unwrap())
        .map_err(|e| e.to_string())?;
    f.write_all(b"\n").map_err(|e| e.to_string())?;
    f.sync_all().map_err(|e| e.to_string())
}
fn parse_poly(v: &Value, p: u64) -> R<Poly> {
    let terms: Vec<(M, u64)> = serde_json::from_value(v.clone()).map_err(|e| e.to_string())?;
    let mut out = Poly::new();
    for (m, c) in terms {
        need(c > 0 && c < p, "noncanonical coefficient")?;
        need(out.insert(m, c).is_none(), "duplicate monomial")?;
    }
    Ok(out)
}
fn poly_json(p: &Poly) -> Value {
    json!(p.iter().map(|(&m, &c)| (m, c)).collect::<Vec<_>>())
}
fn degree(m: &M) -> u16 {
    m.iter().map(|&x| x as u16).sum()
}
fn poly_degree(p: &Poly) -> Option<u16> {
    p.keys().map(degree).max()
}
fn monomials(d: u8) -> Vec<M> {
    let mut out = Vec::new();
    for i in 0..=d {
        for j in 0..=d - i {
            for k in 0..=d - i - j {
                out.push([i, j, k]);
            }
        }
    }
    out.sort_by(|a, b| degree(b).cmp(&degree(a)).then_with(|| b.cmp(a)));
    out
}

#[derive(Clone, Copy, Default)]
struct Work {
    muls: u64,
    adds: u64,
    invs: u64,
}
impl Work {
    fn json(self) -> Value {
        json!({"modular_multiplications":self.muls,"modular_additions":self.adds,"inversion_calls":self.invs})
    }
    fn minus(self, b: Self) -> Self {
        Self {
            muls: self.muls - b.muls,
            adds: self.adds - b.adds,
            invs: self.invs - b.invs,
        }
    }
}
struct Fp {
    p: u64,
    w: Work,
}
impl Fp {
    fn new(p: u64) -> Self {
        assert!(p >= 2 && p <= u32::MAX as u64);
        Self {
            p,
            w: Work::default(),
        }
    }
    fn add(&mut self, a: u64, b: u64) -> u64 {
        self.w.adds += 1;
        (a + b) % self.p
    }
    fn sub(&mut self, a: u64, b: u64) -> u64 {
        self.w.adds += 1;
        (a + self.p - b) % self.p
    }
    fn mul(&mut self, a: u64, b: u64) -> u64 {
        self.w.muls += 1;
        a * b % self.p
    }
    fn inv(&mut self, a: u64) -> u64 {
        assert!(a > 0);
        self.w.invs += 1;
        let (mut x, mut n, mut y) = (a, self.p - 2, 1);
        while n > 0 {
            if n & 1 != 0 {
                y = self.mul(y, x);
            }
            n >>= 1;
            if n > 0 {
                x = self.mul(x, x);
            }
        }
        y
    }
}
fn insert(p: &mut Poly, m: M, c: u64, f: &mut Fp) {
    let v = f.add(p.get(&m).copied().unwrap_or(0), c);
    if v == 0 {
        p.remove(&m);
    } else {
        p.insert(m, v);
    }
}
fn product(a: &Poly, b: &Poly, f: &mut Fp) -> Poly {
    let mut out = Poly::new();
    for (&x, &c) in a {
        for (&y, &d) in b {
            let m = std::array::from_fn(|i| x[i].checked_add(y[i]).unwrap());
            let v = f.mul(c, d);
            insert(&mut out, m, v, f);
        }
    }
    out
}
fn q(a: u64, f: &mut Fp) -> Poly {
    let a2 = f.mul(a, a);
    let a3 = f.mul(a2, a);
    let na2 = f.sub(0, a2);
    let neg1 = f.sub(0, 1);
    [
        ([0, 0, 0], a3),
        ([1, 0, 0], na2),
        ([0, 1, 0], a),
        ([0, 0, 1], neg1),
    ]
    .into_iter()
    .filter(|(_, c)| *c != 0)
    .collect()
}
fn powers(a: u64, f: &mut Fp) -> Vec<Poly> {
    let v = q(a, f);
    let v2 = product(&v, &v, f);
    let v3 = product(&v2, &v, f);
    vec![v, v2, v3]
}

#[derive(Clone)]
struct Certificate {
    d: u8,
    k: u8,
    multipliers: Vec<Poly>,
}
impl Certificate {
    fn json(&self) -> Value {
        json!({"degree_cap":self.d,"power":self.k,
        "multipliers":self.multipliers.iter().map(poly_json).collect::<Vec<_>>()})
    }
    fn parse(v: &Value, p: u64) -> R<Self> {
        let d = num(&v["degree_cap"])?;
        let k = num(&v["power"])?;
        need(
            (4..=8).contains(&d) && (1..=3).contains(&k),
            "certificate limit",
        )?;
        let multipliers = arr(&v["multipliers"])?
            .iter()
            .map(|v| parse_poly(v, p))
            .collect::<R<Vec<_>>>()?;
        need(multipliers.len() == 4, "certificate arity")?;
        Ok(Self {
            d: d as u8,
            k: k as u8,
            multipliers,
        })
    }
}
// This verifier expands sparse polynomials; it never uses the dense basis or
// the representation vectors that constructed the certificate.
fn verify(polys: &[Poly], a: u64, c: &Certificate, f: &mut Fp) -> R<()> {
    need(
        polys.len() == 4 && c.multipliers.len() == 4,
        "certificate arity",
    )?;
    let mut out = Poly::new();
    for (h, g) in c.multipliers.iter().zip(polys) {
        if let (Some(hd), Some(gd)) = (poly_degree(h), poly_degree(g)) {
            need(hd + gd <= c.d as u16, "certificate degree exceeds cap")?;
        }
        for (m, v) in product(h, g, f) {
            insert(&mut out, m, v, f);
        }
    }
    let expected = powers(a, f)[c.k as usize - 1].clone();
    need(out == expected, "false certificate identity")
}

// Find an exact row-span representation. No original roots or F4 results are
// available here. The high-degree columns are eliminated first.
fn search(
    polys: &[Poly],
    a: u64,
    f: &mut Fp,
    deadline: Instant,
    note: &mut impl FnMut(Value) -> R<()>,
) -> R<(Option<Certificate>, Vec<Value>)> {
    let targets = powers(a, f);
    let mut attempts = Vec::new();
    for d in 4..=8u8 {
        need(Instant::now() < deadline, "cooperative deadline")?;
        let before = f.w;
        let mons = monomials(d);
        let index: BTreeMap<_, _> = mons.iter().enumerate().map(|(i, &m)| (m, i)).collect();
        let mut generators = Vec::new();
        for (pi, poly) in polys.iter().enumerate() {
            if let Some(gd) = poly_degree(poly) {
                if gd <= d as u16 {
                    for m in monomials(d - gd as u8) {
                        generators.push((pi, m));
                    }
                }
            }
        }
        let rows = generators.len();
        let cols = mons.len();
        note(
            json!({"phase":"degree-start","degree":d,"rows":rows,"columns":cols,"work_so_far":f.w.json()}),
        )?;
        let mut basis: Vec<Option<(Vec<u64>, Vec<u64>)>> = vec![None; cols];
        let mut rank = 0;
        for (ri, &(pi, shift)) in generators.iter().enumerate() {
            if Instant::now() >= deadline {
                note(
                    json!({"phase":"degree-timeout","degree":d,"rows":rows,"columns":cols,"rank":rank,"rows_processed":ri,"work_so_far":f.w.json()}),
                )?;
                return Err("cooperative deadline".into());
            }
            let mut row = vec![0; cols];
            let mut representation = vec![0; rows];
            representation[ri] = 1;
            for (&m, &c) in &polys[pi] {
                let shifted = std::array::from_fn(|i| m[i] + shift[i]);
                row[index[&shifted]] = c;
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
                    for j in 0..rows {
                        if br[j] != 0 {
                            let v = f.mul(scalar, br[j]);
                            representation[j] = f.sub(representation[j], v);
                        }
                    }
                } else {
                    let inverse = f.inv(scalar);
                    for v in &mut row[col..] {
                        if *v != 0 {
                            *v = f.mul(*v, inverse);
                        }
                    }
                    for v in &mut representation {
                        if *v != 0 {
                            *v = f.mul(*v, inverse);
                        }
                    }
                    basis[col] = Some((row, representation));
                    rank += 1;
                    break;
                }
            }
        }
        for (ki, target) in targets.iter().enumerate() {
            let mut row = vec![0; cols];
            let mut weights = vec![0; rows];
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
                    for j in 0..rows {
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
                attempts.push(json!({"degree":d,"rows":rows,"columns":cols,"rank":rank,"found_power":ki+1,"work":f.w.minus(before).json()}));
                note(json!({"phase":"degree-completed","attempt":attempts.last().unwrap()}))?;
                return Ok((
                    Some(Certificate {
                        d,
                        k: ki as u8 + 1,
                        multipliers,
                    }),
                    attempts,
                ));
            }
        }
        attempts.push(json!({"degree":d,"rows":rows,"columns":cols,"rank":rank,"found_power":Value::Null,"work":f.w.minus(before).json()}));
        note(json!({"phase":"degree-completed","attempt":attempts.last().unwrap()}))?;
    }
    Ok((None, attempts))
}

fn polynomial_value(p: &Poly, e: [u64; 3], modulus: u64) -> u64 {
    // Audit-only direct evaluation, with u128 modular products.
    let mut out = 0u128;
    for (m, &c) in p {
        let mut v = c as u128;
        for i in 0..3 {
            for _ in 0..m[i] {
                v = v * e[i] as u128 % modulus as u128;
            }
        }
        out = (out + v) % modulus as u128;
    }
    out as u64
}

fn prepare(workload_path: &Path, expected_sha: &str, out: &Path) -> R<()> {
    need(
        expected_sha == WORKLOAD_SHA && sha(workload_path)? == expected_sha,
        "frozen workload hash",
    )?;
    need(!out.exists(), "output exists")?;
    let workload = read(workload_path)?;
    let tasks: Vec<_> = arr(&workload["workloads"])?
        .iter()
        .filter(|t| t["mode"] == "zero")
        .collect();
    need(tasks.len() == 42, "expected all 42 zero case-arms")?;
    std::fs::create_dir_all(out.join("inputs")).map_err(|e| e.to_string())?;
    let mut rows = Vec::new();
    let mut seen = BTreeSet::new();
    let mut tsv = String::new();
    for task in tasks {
        let id = txt(&task["id"])?;
        need(
            seen.insert(id.to_owned()) && !id.contains('/') && !id.contains(".."),
            "case identity",
        )?;
        let reference_path = PathBuf::from(txt(&task["zero_reference_path"])?);
        let public_path = PathBuf::from(txt(&task["public_path"])?);
        let audit_path = reference_path
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("audit/summary.json");
        need(
            sha(&reference_path)? == txt(&task["zero_reference_sha256"])?
                && sha(&public_path)? == txt(&task["public_sha256"])?,
            "source input seal",
        )?;
        let reference = read(&reference_path)?;
        let public = read(&public_path)?;
        let audit = read(&audit_path)?;
        need(
            audit["status"] == "pass"
                && audit["candidate_sha256"] == task["zero_reference_sha256"]
                && audit["public_input_sha256"] == task["public_sha256"],
            "independent reference audit binding",
        )?;
        need(
            reference["status"] == "pass"
                && num(&reference["undetermined_systems"])? == 0
                && reference["cooperative_deadline_hit"] == false,
            "incomplete reference",
        )?;
        for key in ["p", "seed", "arm"] {
            need(reference[key] == task[key], "reference workload identity")?;
        }
        let p = num(&task["p"])?;
        need(
            PRIMES.contains(&p) && num(&task["seed"])? < 3,
            "unregistered case",
        )?;
        let l0 = num(&reference["ordinary_abscissae"])? as usize;
        let base = arr(&public["base_coordinates"])?;
        need(l0 > 0 && l0 < base.len(), "ordinary base boundary")?;
        let mut xs = Vec::new();
        for point in &base[..l0] {
            let x: [u64; 4] =
                serde_json::from_value(point[0].clone()).map_err(|e| e.to_string())?;
            need(
                x[0] > 0 && x[0] < p && x[1..] == [0, 0, 0],
                "ordinary abscissa encoding",
            )?;
            xs.push(x[0]);
        }
        need(
            xs.iter().copied().collect::<BTreeSet<_>>().len() == l0,
            "duplicate ordinary abscissa",
        )?;
        let mut systems = Vec::new();
        for (i, s) in arr(&reference["systems"])?.iter().enumerate() {
            let pi = num(&s["pair"][0])? as usize;
            need(pi < l0, "ordinary pair index")?;
            let equations = arr(&s["equations"])?;
            need(equations.len() == 4, "polynomial arity")?;
            for eq in equations {
                parse_poly(eq, p)?;
            }
            systems.push(
                json!({"index":i,"pair":s["pair"],"peeled_abscissa":xs[pi],"equations":equations}),
            );
        }
        need(
            systems.len() as u64 == num(&reference["pair_representatives"])?
                && systems.len() as u64 == num(&reference["systems_attempted"])?,
            "incomplete pair coverage",
        )?;
        let input_path = out.join("inputs").join(format!("{id}.json"));
        write_new(
            &input_path,
            &json!({"schema":"twist-collision-input/v1","id":id,"p":p,"seed":task["seed"],"arm":task["arm"],
            "public_input_sha256":task["public_sha256"],"reference_sha256":task["zero_reference_sha256"],"systems":systems,
            "scope":"Exact public-bound polynomial inputs only; no roots, hits, old costs, factor logs or target scalar."}),
        )?;
        let input_sha = sha(&input_path)?;
        rows.push(json!({"id":id,"p":p,"seed":task["seed"],"arm":task["arm"],"input_path":input_path,"input_sha256":input_sha,
            "reference_path":reference_path,"reference_sha256":task["zero_reference_sha256"],"audit_path":audit_path,"audit_sha256":sha(&audit_path)?,"public_path":public_path,"public_sha256":task["public_sha256"]}));
        tsv.push_str(&format!(
            "{id}\t{}\t{input_sha}\t{}\t{}\t{}\t{}\n",
            input_path.display(),
            reference_path.display(),
            txt(&task["zero_reference_sha256"])?,
            audit_path.display(),
            sha(&audit_path)?
        ));
    }
    write_new(
        &out.join("workload.json"),
        &json!({"schema":"twist-collision-workload/v1","source_workload_sha256":expected_sha,"case_arms":42,"degree_caps":[4,5,6,7,8],"powers":[1,2,3],"workloads":rows}),
    )?;
    std::fs::write(out.join("workload.tsv"), tsv).map_err(|e| e.to_string())?;
    println!("prepared 42 sealed case-arms; no certificate search or F4 execution");
    Ok(())
}

fn produce(input_path: &Path, expected_sha: &str, out: &Path) -> R<()> {
    let start = Instant::now();
    let deadline = start + Duration::from_secs(1180);
    need(!out.exists(), "output exists")?;
    need(sha(input_path)? == expected_sha, "input hash")?;
    let input = read(input_path)?;
    need(
        input["schema"] == "twist-collision-input/v1",
        "input schema",
    )?;
    let p = num(&input["p"])?;
    need(PRIMES.contains(&p), "unregistered prime")?;
    std::fs::create_dir_all(out).map_err(|e| e.to_string())?;
    let mut journal = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(out.join("systems.jsonl"))
        .map_err(|e| e.to_string())?;
    let mut results = Vec::new();
    let mut total_search = Fp::new(p);
    let mut total_check = Fp::new(p);
    let mut certified = 0;
    for (i, s) in arr(&input["systems"])?.iter().enumerate() {
        need(num(&s["index"])? == i as u64, "system order")?;
        writeln!(journal, "{}", json!({"phase":"pending","index":i})).map_err(|e| e.to_string())?;
        journal.sync_all().map_err(|e| e.to_string())?;
        let before = total_search.w;
        let before_check = total_check.w;
        let polys = arr(&s["equations"])?
            .iter()
            .map(|v| parse_poly(v, p))
            .collect::<R<Vec<_>>>()?;
        need(polys.len() == 4, "polynomial arity")?;
        let a = num(&s["peeled_abscissa"])?;
        need(a < p, "abscissa range")?;
        let mut note = |event: Value| -> R<()> {
            writeln!(journal, "{}", json!({"system_index":i,"event":event}))
                .map_err(|e| e.to_string())?;
            journal.sync_all().map_err(|e| e.to_string())
        };
        let (certificate, attempts) = match search(
            &polys,
            a,
            &mut total_search,
            deadline,
            &mut note,
        ) {
            Ok(v) => v,
            Err(e) if e == "cooperative deadline" => {
                write_new(
                    &out.join("partial.json"),
                    &json!({"schema":"twist-collision-partial/v1","input_sha256":expected_sha,"completed_systems":results.len(),"pending_index":i,"reason":e,
                    "search_work":total_search.w.json(),"certificate_check_work":total_check.w.json()}),
                )?;
                return Err(e);
            }
            Err(e) => return Err(e),
        };
        if let Some(c) = &certificate {
            verify(&polys, a, c, &mut total_check)?;
            certified += 1;
        }
        let result = json!({"index":i,"status":if certificate.is_some(){"certified-no-distinct-factor-relation"}else{"fallback-needed"},
            "certificate":certificate.as_ref().map(Certificate::json),"attempts":attempts,
            "search_work":total_search.w.minus(before).json(),"certificate_check_work":total_check.w.minus(before_check).json()});
        writeln!(journal, "{}", json!({"phase":"completed","record":result}))
            .map_err(|e| e.to_string())?;
        journal.sync_all().map_err(|e| e.to_string())?;
        results.push(result);
    }
    write_new(
        &out.join("record.json"),
        &json!({"schema":"twist-collision-producer/v1","status":"complete","input_sha256":expected_sha,"id":input["id"],"p":p,"seed":input["seed"],"arm":input["arm"],
        "systems":results,"certified_systems":certified,"search_work":total_search.w.json(),"certificate_check_work":total_check.w.json(),"diagnostic_elapsed_ns":start.elapsed().as_nanos(),
        "scope":"Shared-host certificate diagnostic only. Inversion exponentiation multiplications are already in multiplication counts. No F4 fallback execution, target recovery, admitted time or complete pipeline price."}),
    )?;
    println!(
        "{}",
        json!({"status":"complete","systems":results.len(),"certified_systems":certified})
    );
    Ok(())
}

fn audit(
    workload_path: &Path,
    workload_sha: &str,
    id: &str,
    candidate_dir: &Path,
    out: &Path,
) -> R<()> {
    need(sha(workload_path)? == workload_sha, "audit workload hash")?;
    let workload = read(workload_path)?;
    need(
        workload["schema"] == "twist-collision-workload/v1"
            && workload["source_workload_sha256"] == WORKLOAD_SHA
            && num(&workload["case_arms"])? == 42,
        "audit workload schema",
    )?;
    let tasks: Vec<_> = arr(&workload["workloads"])?
        .iter()
        .filter(|t| t["id"] == id)
        .collect();
    need(tasks.len() == 1, "audit task identity")?;
    let task = tasks[0];
    let input_path = Path::new(txt(&task["input_path"])?);
    let input_sha = txt(&task["input_sha256"])?;
    let reference_path = Path::new(txt(&task["reference_path"])?);
    let reference_sha = txt(&task["reference_sha256"])?;
    let public_path = Path::new(txt(&task["public_path"])?);
    let public_sha = txt(&task["public_sha256"])?;
    let source_audit_path = Path::new(txt(&task["audit_path"])?);
    need(!out.exists(), "audit output exists")?;
    need(
        sha(input_path)? == input_sha
            && sha(reference_path)? == reference_sha
            && sha(public_path)? == public_sha
            && sha(source_audit_path)? == txt(&task["audit_sha256"])?,
        "audit input binding",
    )?;
    let source_audit = read(source_audit_path)?;
    need(
        source_audit["status"] == "pass"
            && source_audit["candidate_sha256"] == reference_sha
            && source_audit["public_input_sha256"] == public_sha,
        "original independent audit",
    )?;
    let public = read(public_path)?;
    let base = arr(&public["base_coordinates"])?;
    let input = read(input_path)?;
    let reference = read(reference_path)?;
    let path = candidate_dir.join("record.json");
    let candidate = read(&path)?;
    need(
        input["schema"] == "twist-collision-input/v1"
            && candidate["schema"] == "twist-collision-producer/v1"
            && candidate["status"] == "complete"
            && candidate["input_sha256"] == input_sha
            && input["reference_sha256"] == reference_sha,
        "candidate schema/binding",
    )?;
    for key in ["p", "seed", "arm"] {
        need(
            input[key] == reference[key] && candidate[key] == input[key],
            "case identity",
        )?;
    }
    let p = num(&input["p"])?;
    let inputs = arr(&input["systems"])?;
    let records = arr(&candidate["systems"])?;
    let references = arr(&reference["systems"])?;
    need(
        inputs.len() == records.len() && records.len() == references.len(),
        "omitted/extra system",
    )?;
    let (mut certified, mut roots_checked, mut rejected_f4_muls, mut original_f4_muls) =
        (0u64, 0u64, 0u64, 0u64);
    let mut check = Fp::new(p);
    let mut source_search = Work::default();
    let mut source_check = Work::default();
    for (i, ((s, r), original)) in inputs.iter().zip(records).zip(references).enumerate() {
        need(
            num(&s["index"])? == i as u64
                && num(&r["index"])? == i as u64
                && s["pair"] == original["pair"]
                && s["equations"] == original["equations"],
            "system identity",
        )?;
        let ordinary = num(&s["pair"][0])? as usize;
        need(
            ordinary < num(&reference["ordinary_abscissae"])? as usize && ordinary < base.len(),
            "peeled factor index",
        )?;
        let x: [u64; 4] =
            serde_json::from_value(base[ordinary][0].clone()).map_err(|e| e.to_string())?;
        need(
            x[1..] == [0, 0, 0] && num(&s["peeled_abscissa"])? == x[0],
            "peeled abscissa binding",
        )?;
        let old_muls = num(&original["f4_row_reduction_fp_muls"])?;
        original_f4_muls = original_f4_muls
            .checked_add(old_muls)
            .ok_or("counter overflow")?;
        for (target, key) in [
            (&mut source_search, "search_work"),
            (&mut source_check, "certificate_check_work"),
        ] {
            target.muls = target
                .muls
                .checked_add(num(&r[key]["modular_multiplications"])?)
                .ok_or("counter overflow")?;
            target.adds = target
                .adds
                .checked_add(num(&r[key]["modular_additions"])?)
                .ok_or("counter overflow")?;
            target.invs = target
                .invs
                .checked_add(num(&r[key]["inversion_calls"])?)
                .ok_or("counter overflow")?;
        }
        if r["certificate"].is_null() {
            need(r["status"] == "fallback-needed", "fallback label")?;
            let ds: Vec<_> = arr(&r["attempts"])?
                .iter()
                .map(|v| num(&v["degree"]))
                .collect::<R<_>>()?;
            need(ds == vec![4, 5, 6, 7, 8], "omitted unsuccessful degree")?;
        } else {
            need(
                r["status"] == "certified-no-distinct-factor-relation",
                "certificate label",
            )?;
            let c = Certificate::parse(&r["certificate"], p)?;
            let attempts = arr(&r["attempts"])?;
            let ds: Vec<_> = attempts
                .iter()
                .map(|v| num(&v["degree"]))
                .collect::<R<_>>()?;
            need(
                ds == (4..=c.d as u64).collect::<Vec<_>>(),
                "certificate degree prefix",
            )?;
            for attempt in &attempts[..attempts.len() - 1] {
                need(
                    attempt["found_power"].is_null(),
                    "ignored earlier certificate",
                )?;
            }
            need(
                num(&attempts.last().unwrap()["found_power"])? == c.k as u64,
                "certificate stopping power",
            )?;
            let polys = arr(&s["equations"])?
                .iter()
                .map(|v| parse_poly(v, p))
                .collect::<R<Vec<_>>>()?;
            let a = num(&s["peeled_abscissa"])?;
            verify(&polys, a, &c, &mut check)?;
            need(
                arr(&original["accepted_representative_relations"])?.is_empty(),
                "certificate would discard a valid relation",
            )?;
            let qpoly = q(a, &mut check);
            for root in arr(&original["field_roots"])? {
                let e: [u64; 3] =
                    serde_json::from_value(root.clone()).map_err(|e| e.to_string())?;
                need(
                    e.iter().all(|&v| v < p)
                        && polys.iter().all(|poly| polynomial_value(poly, e, p) == 0),
                    "false reference root",
                )?;
                need(
                    polynomial_value(&qpoly, e, p) == 0,
                    "nonrepeated root contradicts certificate",
                )?;
                roots_checked += 1;
            }
            certified += 1;
            rejected_f4_muls = rejected_f4_muls
                .checked_add(old_muls)
                .ok_or("counter overflow")?;
        }
    }
    need(
        num(&candidate["certified_systems"])? == certified
            && candidate["search_work"] == source_search.json()
            && candidate["certificate_check_work"] == source_check.json(),
        "work/count sum",
    )?;
    let hypothetical = source_search
        .muls
        .checked_add(source_check.muls)
        .and_then(|v| v.checked_add(original_f4_muls - rejected_f4_muls))
        .ok_or("projection overflow")?;
    std::fs::create_dir_all(out).map_err(|e| e.to_string())?;
    write_new(
        &out.join("summary.json"),
        &json!({"schema":"twist-collision-audit/v1","status":"pass","input_sha256":input_sha,"candidate_sha256":sha(&path)?,"reference_sha256":reference_sha,
        "id":input["id"],"p":p,"seed":input["seed"],"arm":input["arm"],"systems_checked":inputs.len(),"certified_systems":certified,"reference_roots_checked_for_certified_systems":roots_checked,
        "source_search_work":source_search.json(),"source_certificate_check_work":source_check.json(),"independent_polynomial_expansion_work":check.w.json(),
        "retained_original_f4_row_multiplications":original_f4_muls,"certified_systems_original_f4_row_multiplications":rejected_f4_muls,
        "calculated_search_plus_check_plus_retained_fallback_row_multiplications":hypothetical,
        "complete_end_to_end_cost":Value::Null,
        "scope":"Exact sparse certificate identities and all sealed roots checked; certified systems have no accepted physical relation. Static relation-preservation diagnostic. Projection combines actual certificate multiplication counts with old F4 row counts for uncertified systems; no fallback execution or admitted timing. Additions/inversions, other F4 primitives, memory and I/O remain unpriced."}),
    )?;
    println!(
        "{}",
        json!({"status":"pass","systems_checked":inputs.len(),"certified_systems":certified})
    );
    Ok(())
}

fn deterministic(record: &Value) -> R<Value> {
    let mut v = record.clone();
    need(
        v["schema"] == "twist-collision-producer/v1" && v["status"] == "complete",
        "parity schema/status",
    )?;
    v.as_object_mut()
        .ok_or("record object")?
        .remove("diagnostic_elapsed_ns")
        .ok_or("missing diagnostic interval")?;
    Ok(v)
}
fn parity(first: &Path, second: &Path, out: &Path) -> R<()> {
    need(!out.exists(), "parity output exists")?;
    let a = first.join("record.json");
    let b = second.join("record.json");
    need(
        deterministic(&read(&a)?)? == deterministic(&read(&b)?)?,
        "AA deterministic mismatch",
    )?;
    write_new(
        out,
        &json!({"schema":"twist-collision-parity/v1","status":"pass","first_sha256":sha(&a)?,"second_sha256":sha(&b)?,
        "scope":"Exact deterministic equality, excluding only each run's raw diagnostic elapsed interval."}),
    )
}
fn summarize(run: &Path, workload_path: &Path, expected_sha: &str, out: &Path) -> R<()> {
    need(
        !out.exists() && sha(workload_path)? == expected_sha,
        "summary output/workload binding",
    )?;
    let workload = read(workload_path)?;
    need(
        workload["schema"] == "twist-collision-workload/v1",
        "workload schema",
    )?;
    let tasks = arr(&workload["workloads"])?;
    need(tasks.len() == 42, "42 case-arms required")?;
    let mut rows = Vec::new();
    let mut failures = Vec::new();
    let mut exits = 0u64;
    for task in tasks {
        let id = txt(&task["id"])?;
        for pass in ["first", "aa"] {
            if run.join(format!("metadata/{id}-{pass}.exit")).is_file() {
                exits += 1;
            }
        }
        let row = (|| -> R<Value> {
            let root = run.join("cases").join(id);
            let first_path = root.join("first/record.json");
            let aa_path = root.join("aa/record.json");
            for phase in ["first", "audit", "aa", "parity"] {
                let path = run.join(format!("metadata/{id}-{phase}.exit"));
                need(
                    std::fs::read_to_string(&path)
                        .map_err(|e| format!("{}: {e}", path.display()))?
                        .trim()
                        == "0",
                    "nonzero case phase",
                )?;
            }
            let first = read(&first_path)?;
            let aa = read(&aa_path)?;
            let audit_path = root.join("audit/summary.json");
            let audit = read(&audit_path)?;
            let parity_path = root.join("parity.json");
            let pair = read(&parity_path)?;
            need(
                deterministic(&first)? == deterministic(&aa)?,
                "summary AA mismatch",
            )?;
            need(
                first["input_sha256"] == task["input_sha256"]
                    && audit["input_sha256"] == task["input_sha256"]
                    && audit["status"] == "pass"
                    && audit["candidate_sha256"] == sha(&first_path)?
                    && audit["reference_sha256"] == task["reference_sha256"],
                "summary audit binding",
            )?;
            need(
                pair["status"] == "pass"
                    && pair["first_sha256"] == sha(&first_path)?
                    && pair["second_sha256"] == sha(&aa_path)?,
                "summary parity binding",
            )?;
            for key in ["id", "p", "seed", "arm"] {
                need(
                    first[key] == task[key] && audit[key] == task[key],
                    "summary case identity",
                )?;
            }
            let mut row = audit;
            row["status"] = json!("verified-first-aa");
            row["audit_sha256"] = json!(sha(&audit_path)?);
            row["parity_sha256"] = json!(sha(&parity_path)?);
            row["first_diagnostic_elapsed_ns"] = first["diagnostic_elapsed_ns"].clone();
            row["aa_diagnostic_elapsed_ns"] = aa["diagnostic_elapsed_ns"].clone();
            Ok(row)
        })();
        match row {
            Ok(row) => rows.push(row),
            Err(reason) => {
                failures.push(json!({"id":id,"reason":reason}));
                rows.push(json!({"id":id,"p":task["p"],"seed":task["seed"],"arm":task["arm"],"status":"unverified","reason":reason}));
            }
        }
    }
    let mut comparisons = Vec::new();
    for p in PRIMES {
        for seed in 0..3 {
            let get = |arm: &str| -> R<&Value> {
                let found: Vec<_> = rows
                    .iter()
                    .filter(|r| r["p"] == p && r["seed"] == seed && r["arm"] == arm)
                    .collect();
                need(found.len() == 1, "paired row coverage")?;
                Ok(found[0])
            };
            let phi = get("phi-l0-active")?;
            let double = get("double-l0-active")?;
            let verified =
                phi["status"] == "verified-first-aa" && double["status"] == "verified-first-aa";
            comparisons.push(json!({"p":p,"seed":seed,"verified":verified,"phi_id":phi["id"],"doubling_id":double["id"],
            "phi_certified_systems":if verified{phi["certified_systems"].clone()}else{Value::Null},
            "doubling_certified_systems":if verified{double["certified_systems"].clone()}else{Value::Null},
            "phi_calculated_partial_multiplications":if verified{phi["calculated_search_plus_check_plus_retained_fallback_row_multiplications"].clone()}else{Value::Null},
            "doubling_calculated_partial_multiplications":if verified{double["calculated_search_plus_check_plus_retained_fallback_row_multiplications"].clone()}else{Value::Null},
            "complete_price_ratio":Value::Null}));
        }
    }
    let complete = failures.is_empty() && exits == 84;
    std::fs::create_dir_all(out).map_err(|e| e.to_string())?;
    write_new(
        &out.join("summary.json"),
        &json!({"schema":"twist-collision-panel-summary/v1","status":if complete{"complete-verified-first-aa"}else{"partial"},
        "workload_sha256":expected_sha,"expected_case_arms":42,"expected_producer_executions":84,"recorded_producer_exits":exits,"unverified":failures,"rows":rows,"comparisons":comparisons,
        "scope":"Static certificate diagnostic, no repeated F4 solves or integrated target-recovery experiment. All complete end-to-end prices and admitted speedups remain null."}),
    )?;
    if complete {
        Ok(())
    } else {
        Err("partial panel".into())
    }
}

fn main() {
    let a: Vec<_> = std::env::args().collect();
    let result = match a.get(1).map(String::as_str) {
        Some("prepare") if a.len() == 5 => prepare(Path::new(&a[2]), &a[3], Path::new(&a[4])),
        Some("produce") if a.len() == 5 => produce(Path::new(&a[2]), &a[3], Path::new(&a[4])),
        Some("audit") if a.len() == 7 => audit(
            Path::new(&a[2]),
            &a[3],
            &a[4],
            Path::new(&a[5]),
            Path::new(&a[6]),
        ),
        Some("parity") if a.len() == 5 => {
            parity(Path::new(&a[2]), Path::new(&a[3]), Path::new(&a[4]))
        }
        Some("summarize") if a.len() == 6 => {
            summarize(Path::new(&a[2]), Path::new(&a[3]), &a[4], Path::new(&a[5]))
        }
        _ => {
            eprintln!("usage: prepare WORKLOAD SHA OUT | produce INPUT SHA OUT | audit WORKLOAD SHA ID CANDIDATE OUT");
            std::process::exit(64);
        }
    };
    if let Err(e) = result {
        eprintln!("collision certificate rejected: {e}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn poly(terms: &[(M, u64)]) -> Poly {
        terms.iter().copied().collect()
    }
    #[test]
    fn cubic_is_product_of_root_differences_exhaustively_at_seven() {
        let p = 7;
        for a in 0..p {
            for x in 0..p {
                for y in 0..p {
                    for z in 0..p {
                        let e = [(x + y + z) % p, (x * y + x * z + y * z) % p, x * y * z % p];
                        let q = q(a, &mut Fp::new(p));
                        assert_eq!(
                            polynomial_value(&q, e, p),
                            (a + p - x) * (a + p - y) * (a + p - z) % p
                        );
                    }
                }
            }
        }
    }
    #[test]
    fn a_nonradical_fiber_requires_a_power() {
        let p = 7;
        let polynomials = vec![
            poly(&[([0, 0, 2], 1)]),
            poly(&[([1, 0, 0], 1)]),
            poly(&[([0, 1, 0], 1)]),
            Poly::new(),
        ];
        let (c, _) = search(
            &polynomials,
            0,
            &mut Fp::new(p),
            Instant::now() + Duration::from_secs(10),
            &mut |_| Ok(()),
        )
        .unwrap();
        let c = c.unwrap();
        assert_eq!(c.k, 2);
        verify(&polynomials, 0, &c, &mut Fp::new(p)).unwrap();
        let mut bad = c.clone();
        *bad.multipliers[0].values_mut().next().unwrap() = 2;
        assert!(verify(&polynomials, 0, &bad, &mut Fp::new(p)).is_err());
    }
    #[test]
    fn a_genuine_nonrepeated_root_prevents_rejection() {
        let p = 7;
        let polynomials = vec![
            poly(&[([1, 0, 0], 1), ([0, 0, 0], 5)]),
            poly(&[([0, 1, 0], 1), ([0, 0, 0], 2)]),
            poly(&[([0, 0, 1], 1), ([0, 0, 0], 4)]),
            Poly::new(),
        ];
        let (c, attempts) = search(
            &polynomials,
            1,
            &mut Fp::new(p),
            Instant::now() + Duration::from_secs(10),
            &mut |_| Ok(()),
        )
        .unwrap();
        assert!(c.is_none());
        assert_eq!(attempts.len(), 5);
        assert_ne!(polynomial_value(&q(1, &mut Fp::new(p)), [2, 5, 3], p), 0);
    }
    #[test]
    fn a_linear_certificate_is_checked_independently() {
        let p = 7;
        let polynomials = vec![
            poly(&[([1, 0, 0], 1), ([0, 0, 0], 6)]),
            poly(&[([0, 1, 0], 1)]),
            poly(&[([0, 0, 1], 1)]),
            Poly::new(),
        ];
        let (c, _) = search(
            &polynomials,
            1,
            &mut Fp::new(p),
            Instant::now() + Duration::from_secs(10),
            &mut |_| Ok(()),
        )
        .unwrap();
        let c = c.unwrap();
        assert_eq!(c.k, 1);
        verify(&polynomials, 1, &c, &mut Fp::new(p)).unwrap();
        assert!(parse_poly(&json!([[[0, 0, 0], 1], [[0, 0, 0], 2]]), p).is_err());
        assert!(parse_poly(&json!([[[0, 0, 0], 7]]), p).is_err());
    }
}

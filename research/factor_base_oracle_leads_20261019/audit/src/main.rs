//! Independent field/group replay and native summaries; no F4 rerun.
//! Arithmetic adapted from the completed chart screen, not a new host.
mod arithmetic;
use arithmetic::{Curve, Point, O};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fs,
    path::Path,
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
#[derive(Default)]
struct Rank {
    rows: BTreeMap<usize, Vec<u64>>,
}
impl Rank {
    fn insert(&mut self, mut row: Vec<u64>, r: u64) -> usize {
        let before = self.rows.len();
        for (&pivot, b) in &self.rows {
            let c = row[pivot];
            for (x, y) in row.iter_mut().zip(b) {
                *x = (*x + r - c * y % r) % r;
            }
        }
        if let Some(pivot) = row.iter().position(|x| *x != 0) {
            let (mut base, mut e, mut inverse) = (row[pivot], r - 2, 1);
            while e != 0 {
                if e & 1 != 0 {
                    inverse = inverse * base % r;
                }
                base = base * base % r;
                e >>= 1;
            }
            for x in &mut row {
                *x = *x * inverse % r;
            }
            self.rows.insert(pivot, row);
        }
        self.rows.len() - before
    }
}
fn read(path: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}
fn number(v: &Value, k: &str) -> Result<u64> {
    v[k].as_u64()
        .ok_or_else(|| format!("missing number {k}").into())
}
fn array<'a>(v: &'a Value, k: &str) -> Result<&'a Vec<Value>> {
    v[k].as_array()
        .ok_or_else(|| format!("missing array {k}").into())
}
fn point(v: &Value) -> Result<Point> {
    Ok(serde_json::from_value(v.clone())?)
}
fn points(v: &Value) -> Result<Vec<Point>> {
    Ok(serde_json::from_value(v.clone())?)
}
fn neg(p: Point) -> Point {
    if p.infinity {
        p
    } else {
        Point { y: p.x ^ p.y, ..p }
    }
}
fn psi(c: &Curve, p: Point) -> Result<Point> {
    let mut q = c.add(p, p)?;
    if !q.infinity {
        for _ in 1..c.n {
            q.x = c.sq(q.x);
            q.y = c.sq(q.y);
        }
    }
    Ok(q)
}
fn truth(c: &Curve, a: &[Point], b: &[Point], t: Point) -> Result<bool> {
    for &p in a {
        for &q in b {
            if c.add(p, q)? == t {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
fn decode(root: &[Value], basis: &Value, offset: usize) -> Result<u64> {
    let mut x = 0;
    for (j, b) in basis.as_array().ok_or("missing basis")?.iter().enumerate() {
        let v = root[offset + j].as_u64().ok_or("invalid bit")?;
        if v > 1 {
            return Err("non-Boolean root".into());
        }
        if v == 1 {
            x ^= b.as_u64().ok_or("invalid basis word")?;
        }
    }
    Ok(x)
}
fn eval(f: &Value, root: &[Value]) -> Result<u64> {
    let mut sum = 0;
    for term in f.as_array().ok_or("invalid polynomial")? {
        let exps = term[0].as_array().ok_or("invalid exponents")?;
        if exps.len() != root.len() {
            return Err("root dimension mismatch".into());
        }
        let mut value = term[1].as_u64().ok_or("invalid coefficient")?;
        for (e, x) in exps.iter().zip(root) {
            if e.as_u64().ok_or("invalid exponent")? != 0 {
                value &= x.as_u64().ok_or("invalid bit")?;
            }
        }
        sum ^= value;
    }
    Ok(sum & 1)
}
#[derive(Default)]
struct Stats {
    trials: u64,
    verified: u64,
    absent: u64,
    unresolved: u64,
    resource: u64,
    muls: u64,
    products: u64,
    roots: u64,
    rank: u64,
    max_degree: u64,
    partial: bool,
}
impl Stats {
    fn value(&self) -> Value {
        json!({"trials":self.trials,"verified":self.verified,"absent":self.absent,"unresolved":self.unresolved,"resource_limited":self.resource,"field_reduction_multiplications_completed_calls":self.muls,"encoding_term_products_completed_calls":self.products,"checked_algebraic_roots":self.roots,"rank_gain":self.rank,"max_successful_observed_degree":self.max_degree,"partial_counters":self.partial})
    }
}
fn scan_members(dir: &Path, root: &Path, out: &mut BTreeMap<String, String>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            scan_members(&path, root, out)?;
        } else {
            let rel = path
                .strip_prefix(root)?
                .to_str()
                .ok_or("non-UTF8 path")?
                .replace('\\', "/");
            if rel != "leads-manifest.json" {
                out.insert(rel, format!("{:x}", Sha256::digest(fs::read(&path)?)));
            }
        }
    }
    Ok(())
}
fn audit(run: &Path, models: &Path) -> Result<Value> {
    let manifest = read(&run.join("leads-manifest.json"))?;
    let mut members = BTreeMap::new();
    scan_members(run, run, &mut members)?;
    if serde_json::to_value(&members)? != manifest["members"] {
        return Err("independent member hash mismatch".into());
    }
    let registry = read(models)?;
    let summary = read(&run.join("summary.json"))?;
    let (mut witness_count, mut root_count, mut trial_count, mut pair_checks, mut lift_checks) =
        (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut cases = Vec::new();
    for case in array(&summary, "cases")? {
        let slug = case["curve"].as_str().ok_or("missing curve")?;
        let row = array(&registry, "curves")?
            .iter()
            .find(|r| r["slug"] == slug)
            .ok_or("unregistered curve")?;
        let c = Curve {
            n: number(case, "n")? as u32,
            modulus: arithmetic::hex(row["params"]["modulus"].as_str().ok_or("missing modulus")?)?,
            a: number(case, "a")?,
        };
        if number(&row["params"], "n")? != u64::from(c.n) || number(&row["params"], "a")? != c.a {
            return Err("model mismatch".into());
        }
        let r = number(case, "r")?;
        if !arithmetic::prime(r) {
            return Err("nonprime subgroup".into());
        }
        let g = point(&case["generator"])?;
        if !c.on_curve(g) || g.infinity || c.scale(g, r)? != O {
            return Err("bad generator".into());
        }
        let dir = run.join(case["id"].as_str().ok_or("missing id")?);
        let inventory = read(&dir.join("inventory.json"))?;
        let union = points(&inventory["union"])?;
        let quotient = points(&inventory["quotient"])?;
        let h = number(case, "cofactor")?;
        let columns = points(&inventory["projected_columns"])?;
        let mut expected_columns = BTreeSet::new();
        for p in &union {
            let q = c.scale(*p, h)?;
            if q != O {
                expected_columns.insert(q.min(neg(q)));
            }
        }
        if columns.iter().copied().collect::<BTreeSet<_>>() != expected_columns
            || columns.len() as u64 != number(case, "columns")?
        {
            return Err("wrong projected columns".into());
        }
        for p in &union {
            if !c.on_curve(*p) || p.infinity || !union.contains(&neg(*p)) {
                return Err("invalid signed point inventory".into());
            }
        }
        let mut support = BTreeSet::new();
        for (j, p) in union.iter().enumerate() {
            for q in &union[j..] {
                let s = c.add(*p, *q)?;
                pair_checks += 1;
                if s != O && c.scale(s, r)? == O {
                    support.insert(s);
                }
            }
        }
        let recorded_support: BTreeSet<_> = points(&inventory["support"])?.into_iter().collect();
        if support != recorded_support || support.len() as u64 != number(case, "support")? {
            return Err("support census mismatch".into());
        }
        if case["kind"] == "torsion" {
            for p in &union {
                if !quotient.contains(&psi(&c, *p)?) {
                    return Err("quotient preimage mismatch".into());
                }
                lift_checks += 1;
            }
            for p in &quotient {
                if !c.on_curve(*p) || *p == O || c.scale(*p, r)? != O {
                    return Err("quotient base outside prime subgroup".into());
                }
            }
        }
        let mut stats: BTreeMap<String, Stats> = BTreeMap::new();
        let mut random_targets = BTreeMap::new();
        let mut ranks: BTreeMap<String, Rank> = BTreeMap::new();
        for trial in array(case, "trials")? {
            trial_count += 1;
            let target = point(&trial["target"])?;
            if !c.on_curve(target) || target == O || c.scale(target, r)? != O {
                return Err("target outside prime subgroup".into());
            }
            let phase = trial["phase"].as_str().ok_or("missing phase")?;
            let arm = trial["arm"].as_str().ok_or("missing arm")?;
            if phase != "positive-control" {
                let scalar = number(trial, "public_control_scalar")?;
                if c.scale(g, scalar)? != target {
                    return Err("public scalar target mismatch".into());
                }
                let key = (phase.to_string(), number(trial, "trial")?);
                if let Some(old) = random_targets.insert(key, target) {
                    if old != target {
                        return Err("unmatched target arms".into());
                    }
                }
            }
            let expected = truth(&c, &union, &union, target)?;
            if expected != !trial["ground_truth"].is_null() {
                return Err("independent original truth mismatch".into());
            }
            let key = format!("{phase}/{arm}");
            let s = stats.entry(key).or_default();
            s.trials += 1;
            s.rank += number(trial, "rank_gain")?;
            let outcome = trial["result"]["status"]
                .as_str()
                .ok_or("missing outcome")?;
            match outcome {
                "verified" => s.verified += 1,
                "absent" => {
                    s.absent += 1;
                    if expected {
                        return Err("false absence".into());
                    }
                }
                "unresolved" => s.unresolved += 1,
                "resource-limited" => {
                    s.resource += 1;
                    s.partial = true;
                }
                _ => return Err("unknown outcome".into()),
            }
            if !trial["original_witness"].is_null() {
                let w = points(&trial["original_witness"])?;
                if w.len() != 2
                    || !union.contains(&w[0])
                    || !union.contains(&w[1])
                    || c.add(w[0], w[1])? != target
                {
                    return Err("invalid original witness".into());
                }
                witness_count += 1;
                if phase == "train" {
                    let mut relation = vec![0; columns.len()];
                    for p in &w {
                        let p = c.scale(*p, h)?;
                        if p == O {
                            continue;
                        }
                        let canonical = p.min(neg(p));
                        let col = columns
                            .binary_search(&canonical)
                            .map_err(|_| "missing projected column")?;
                        relation[col] =
                            (relation[col] + if p == canonical { 1 } else { r - 1 }) % r;
                    }
                    let printed: Vec<u64> = serde_json::from_value(trial["relation_row"].clone())?;
                    if printed != relation {
                        return Err("wrong projected relation row".into());
                    }
                    if ranks
                        .entry(arm.into())
                        .or_default()
                        .insert(relation.clone(), r) as u64
                        != number(trial, "rank_gain")?
                    {
                        return Err("wrong independently replayed rank gain".into());
                    }
                    let mut sum = O;
                    for (p, k) in columns.iter().zip(relation) {
                        sum = c.add(sum, c.scale(*p, k)?)?;
                    }
                    if sum != c.scale(target, h)? {
                        return Err("relation does not sum to projected target".into());
                    }
                } else if !trial["relation_row"].is_null() || number(trial, "rank_gain")? != 0 {
                    return Err("control or holdout polluted rank".into());
                }
            } else if outcome == "verified" {
                return Err("verified row has no witness".into());
            }
            let jobdir = run.join(
                trial["job_directory"]
                    .as_str()
                    .ok_or("missing job directory")?,
            );
            let job = read(&jobdir.join("job.json"))?;
            let jt = point(&job["target"])?;
            if jt
                != if arm == "torsion-quotient" {
                    psi(&c, target)?
                } else {
                    target
                }
            {
                return Err("wrong transported target".into());
            }
            let left = points(&job["left"])?;
            let right = points(&job["right"])?;
            for p in left.iter().chain(&right) {
                if !c.on_curve(*p) {
                    return Err("off-curve job inventory".into());
                }
            }
            if outcome == "resource-limited" {
                continue;
            }
            let mut orders = vec![(&left, &right)];
            if arm == "asymmetric-branches" {
                orders = vec![(&left, &left), (&left, &right), (&right, &right)];
                orders.sort_by_key(|(a, b)| a.len() * b.len());
            }
            for (index, branch) in array(&trial["result"], "branches")?.iter().enumerate() {
                s.muls += number(branch, "field_reduction_multiplications")?;
                s.products += number(branch, "encoding_term_products")?;
                let system =
                    read(&jobdir.join(branch["system"].as_str().ok_or("missing system")?))?;
                let (a, b) = orders[index];
                let bt = truth(&c, a, b, jt)?;
                if bt != !branch["ground_truth"].is_null() {
                    return Err("independent branch truth mismatch".into());
                }
                if branch["status"] == "absent" && bt {
                    return Err("false branch absence".into());
                }
                for root in array(branch, "roots")? {
                    let root = root.as_array().ok_or("missing root bits")?;
                    if root.len() != number(&system, "variables")? as usize {
                        return Err("root dimension mismatch".into());
                    }
                    for eq in array(&system, "equations")? {
                        if eval(eq, root)? != 0 {
                            return Err("invalid root under independent evaluation".into());
                        }
                    }
                    let x = decode(root, &system["basis_left"], 0)?;
                    let y = decode(
                        root,
                        &system["basis_right"],
                        system["basis_left"]
                            .as_array()
                            .ok_or("missing basis")?
                            .len(),
                    )?;
                    if !a.iter().any(|p| p.x == x) || !b.iter().any(|p| p.x == y) {
                        return Err("root violates exact branch membership".into());
                    }
                    let s3 = c.mul(c.sq(x), c.sq(y))
                        ^ c.mul(c.sq(jt.x), c.sq(x ^ y))
                        ^ c.mul(jt.x, c.mul(x, y))
                        ^ 1;
                    if s3 != 0 {
                        return Err("root violates independent S3".into());
                    }
                    root_count += 1;
                    s.roots += 1;
                }
                if branch["status"] == "verified" {
                    s.max_degree = s.max_degree.max(
                        number(branch, "input_degree")?
                            .max(number(branch, "observed_productive_degree")?),
                    );
                    let w = points(&branch["witness"])?;
                    if w.len() != 2
                        || !a.contains(&w[0])
                        || !b.contains(&w[1])
                        || c.add(w[0], w[1])? != jt
                    {
                        return Err("invalid branch witness".into());
                    }
                }
            }
        }
        let mut train = BTreeSet::new();
        let mut holdout = BTreeSet::new();
        for ((phase, _), p) in random_targets {
            if phase == "train" {
                if !train.insert(p) {
                    return Err("duplicate training target".into());
                }
            } else if phase == "holdout" && !holdout.insert(p) {
                return Err("duplicate holdout target".into());
            }
        }
        if !train.is_disjoint(&holdout) {
            return Err("holdout leaks training targets".into());
        }
        let aggregates: BTreeMap<_, _> = stats.iter().map(|(k, v)| (k, v.value())).collect();
        cases.push(json!({"id":case["id"],"curve":slug,"columns":case["columns"],"signed_points":case["signed_points"],"left_points":case["left_points"],"right_points":case["right_points"],"quotient_points":case["quotient_points"],"support":support.len(),"train_distinct":train.len(),"holdout_distinct":holdout.len(),"aggregates":aggregates,"total_cost_winner":null,"certified_degree_of_regularity":null}));
    }
    Ok(
        json!({"schema":"factor-base-oracle-independent-audit/v1","verified":true,"members":members.len(),"trial_rows":trial_count,"original_point_witnesses":witness_count,"algebraic_roots":root_count,"unique_case_unordered_pair_checks":pair_checks,"lift_preimage_checks":lift_checks,"cases":cases,"scope":"independent polynomial-field/group arithmetic and root evaluation on the same host; no independent-host certificate, F4 rerun, complete cost or regularity certificate"}),
    )
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: factor-base-oracle-audit RUN MODELS_JSON".into());
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&audit(Path::new(&args[1]), Path::new(&args[2]))?)?
    );
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn independent_boolean_evaluation_rejects_wrong_roots() {
        let f = json!([[[1, 1], 1], [[0, 0], 1]]);
        assert_eq!(eval(&f, &[json!(1), json!(1)]).unwrap(), 0);
        assert_eq!(eval(&f, &[json!(0), json!(1)]).unwrap(), 1);
    }
    #[test]
    fn quotient_translation_is_checked_on_all_small_field_points() {
        let c = Curve {
            n: 7,
            modulus: 0x83,
            a: 1,
        };
        let t = Point {
            x: 0,
            y: 1,
            infinity: false,
        };
        for x in 0..128 {
            for y in 0..128 {
                let p = Point {
                    x,
                    y,
                    infinity: false,
                };
                if c.on_curve(p) {
                    assert_eq!(psi(&c, p).unwrap(), psi(&c, c.add(p, t).unwrap()).unwrap());
                    if x != 0 {
                        assert_eq!(psi(&c, p).unwrap().x, x ^ c.inv(x).unwrap());
                    }
                }
            }
        }
    }
}

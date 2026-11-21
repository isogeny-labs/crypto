//! Independent u128 arithmetic and dense rank/kernel replay. No production solver imports.
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Write, path::Path};
type E = Box<dyn std::error::Error>;
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Matrix {
    modulus: u64,
    columns: usize,
    rows: Vec<Vec<(usize, u64)>>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Corpus {
    name: String,
    origin_sha256: String,
    matrix: Matrix,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Input {
    schema: String,
    origin_revision: String,
    corpora: Vec<Corpus>,
}
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn u(v: &Value) -> Result<u64, E> {
    v.as_u64().ok_or_else(|| "unsigned integer".into())
}
fn vector(v: &Value) -> Result<Vec<u64>, E> {
    v.as_array()
        .ok_or("vector")?
        .iter()
        .map(u)
        .collect::<Result<_, _>>()
}
#[derive(Clone, Copy)]
struct F(u64);
impl F {
    fn add(self, a: u64, b: u64) -> u64 {
        ((a as u128 + b as u128) % self.0 as u128) as u64
    }
    fn sub(self, a: u64, b: u64) -> u64 {
        ((a as u128 + self.0 as u128 - b as u128) % self.0 as u128) as u64
    }
    fn mul(self, a: u64, b: u64) -> u64 {
        (a as u128 * b as u128 % self.0 as u128) as u64
    }
    fn pow(self, mut a: u64, mut b: u64) -> u64 {
        let mut r = 1;
        while b > 0 {
            if b & 1 != 0 {
                r = self.mul(r, a);
            }
            a = self.mul(a, a);
            b >>= 1;
        }
        r
    }
    fn inv(self, a: u64) -> Result<u64, E> {
        if a == 0 {
            return Err("zero inverse".into());
        }
        let x = self.pow(a, self.0 - 2);
        if self.mul(x, a) != 1 {
            return Err("not a unit".into());
        }
        Ok(x)
    }
    fn dot(self, a: &[u64], b: &[u64]) -> u64 {
        a.iter()
            .zip(b)
            .fold(0, |s, (&x, &y)| self.add(s, self.mul(x, y)))
    }
}
fn validate(m: &Matrix) -> Result<(), E> {
    if m.modulus != 263807419614139553
        || m.columns < 2
        || m.columns > 4096
        || m.rows.len() < m.columns - 1
        || m.rows.len() > 8192
    {
        return Err("matrix bounds".into());
    }
    if m.rows.iter().any(|r| {
        r.windows(2).any(|w| w[0].0 >= w[1].0)
            || r.iter()
                .any(|&(j, a)| j >= m.columns || a == 0 || a >= m.modulus)
    }) {
        return Err("canonical matrix".into());
    }
    Ok(())
}
fn residual(m: &Matrix, h: &[u64]) -> bool {
    let f = F(m.modulus);
    h.len() == m.columns
        && h.last() == Some(&1)
        && h.iter().all(|&a| a < m.modulus)
        && m.rows
            .iter()
            .all(|r| r.iter().fold(0, |s, &(j, a)| f.add(s, f.mul(a, h[j]))) == 0)
}
fn dense_kernel(m: &Matrix) -> Result<(usize, Vec<u64>), E> {
    let f = F(m.modulus);
    let mut a = vec![vec![0; m.columns]; m.rows.len()];
    for (r, terms) in a.iter_mut().zip(&m.rows) {
        for &(j, x) in terms {
            r[j] = x;
        }
    }
    let mut pivots = Vec::new();
    for col in 0..m.columns {
        let rank = pivots.len();
        let Some(pivot) = (rank..a.len()).find(|&i| a[i][col] != 0) else {
            continue;
        };
        a.swap(rank, pivot);
        let inv = f.inv(a[rank][col])?;
        for value in a[rank].iter_mut().skip(col) {
            *value = f.mul(*value, inv);
        }
        for i in 0..a.len() {
            if i == rank {
                continue;
            }
            let factor = a[i][col];
            if factor == 0 {
                continue;
            }
            for j in col..m.columns {
                a[i][j] = f.sub(a[i][j], f.mul(factor, a[rank][j]));
            }
        }
        pivots.push(col);
    }
    if pivots.len() + 1 != m.columns || pivots.contains(&(m.columns - 1)) {
        return Err("normalized nullity-one rank gate".into());
    }
    let mut h = vec![0; m.columns];
    h[m.columns - 1] = 1;
    for (i, &j) in pivots.iter().enumerate() {
        h[j] = f.sub(0, a[i][m.columns - 1]);
    }
    if !residual(m, &h) {
        return Err("independent kernel residual".into());
    }
    Ok((pivots.len(), h))
}
fn normal(m: &Matrix, rw: &[u64], cw: &[u64], v: &[u64]) -> Vec<u64> {
    let f = F(m.modulus);
    let n = m.columns - 1;
    let mut out = vec![0; n];
    for (i, row) in m.rows.iter().enumerate() {
        let mut t = 0;
        for &(j, a) in row {
            if j < n {
                t = f.add(t, f.mul(f.mul(a, cw[j]), v[j]));
            }
        }
        t = f.mul(t, rw[i]);
        for &(j, a) in row {
            if j < n {
                out[j] = f.add(out[j], f.mul(f.mul(a, cw[j]), t));
            }
        }
    }
    out
}
fn lanczos_attempt(m: &Matrix, a: &Value) -> Result<Option<Vec<u64>>, E> {
    let n = m.columns - 1;
    let f = F(m.modulus);
    let rw = vector(&a["row_weights"])?;
    let cw = vector(&a["column_weights"])?;
    let shift = vector(&a["rhs_shift"])?;
    if rw.len() != m.rows.len()
        || cw.len() != n
        || shift.len() != n
        || rw.iter().chain(&cw).any(|&x| x == 0 || x >= m.modulus)
        || shift.iter().any(|&x| x >= m.modulus)
    {
        return Err("preconditioner dimensions/domain".into());
    }
    let mut b = vec![0; n];
    for (i, row) in m.rows.iter().enumerate() {
        let y = row
            .iter()
            .find(|&&(j, _)| j == n)
            .map_or(0, |&(_, a)| f.sub(0, a));
        for &(j, a) in row {
            if j < n {
                b[j] = f.add(b[j], f.mul(f.mul(a, cw[j]), f.mul(rw[i], y)));
            }
        }
    }
    let shifted = normal(m, &rw, &cw, &shift);
    for j in 0..n {
        b[j] = f.add(b[j], shifted[j]);
    }
    let mut w = b.clone();
    let mut prev = vec![0; n];
    let mut prev_image = vec![0; n];
    let mut prev_inv = 1;
    let mut x = vec![0; n];
    let steps = a["steps"].as_array().ok_or("steps")?;
    if steps.len() > n || u(&a["iterations"])? as usize != steps.len() {
        return Err("iteration count".into());
    }
    for (i, s) in steps.iter().enumerate() {
        let image = normal(m, &rw, &cw, &w);
        let d = f.dot(&w, &image);
        let inv = f.inv(d)?;
        let projection = f.mul(f.dot(&b, &w), inv);
        let alpha = f.mul(f.dot(&image, &image), inv);
        let beta = if i == 0 {
            0
        } else {
            f.mul(f.dot(&image, &prev_image), prev_inv)
        };
        if s != &json!({"diagonal":d,"alpha":alpha,"beta":beta,"projection":projection}) {
            return Err("Lanczos recurrence trace".into());
        }
        let mut next = vec![0; n];
        for j in 0..n {
            x[j] = f.add(x[j], f.mul(projection, w[j]));
            next[j] = f.sub(f.sub(image[j], f.mul(alpha, w[j])), f.mul(beta, prev[j]));
        }
        prev = w;
        prev_image = image;
        prev_inv = inv;
        w = next;
    }
    if steps.len() == n {
        if w.iter().any(|&a| a != 0) {
            if a["refusal"] != "Krylov basis did not close at dimension" {
                return Err("nonclosed Krylov trace".into());
            }
            return Ok(None);
        }
        let mut h = (0..n)
            .map(|j| f.mul(cw[j], f.sub(x[j], shift[j])))
            .collect::<Vec<_>>();
        h.push(1);
        if residual(m, &h) {
            if !a["refusal"].is_null() {
                return Err("successful trace refused".into());
            }
            return Ok(Some(h));
        }
        if a["refusal"] != "original-row residual check refused" {
            return Err("trace residual refusal".into());
        }
    } else if w.iter().all(|&x| x == 0) {
        if a["refusal"] != "early Krylov stop lacks full-rank certificate" {
            return Err("early stop refusal".into());
        }
    } else if f.dot(&w, &normal(m, &rw, &cw, &w)) == 0 {
        if a["refusal"] != "self-orthogonal Lanczos direction" {
            return Err("breakdown refusal".into());
        }
    } else {
        return Err("unexplained truncated attempt".into());
    }
    Ok(None)
}
fn price(counts: &Value) -> Result<u128, E> {
    let w = 58u128;
    let add = 13 * w;
    let mul = 2 * (w * w + 5 * w * (w - 1)) + add;
    let inv = 2 * w * add;
    let mut total = 0;
    for (k, v) in counts.as_object().ok_or("primitive object")? {
        let c = u(v)? as u128;
        let p = match k.as_str() {
            "modr_add" => add,
            "modr_mul" => mul,
            "modr_inv" => inv,
            "word_op" => 64,
            "rng_words" => 0,
            _ => {
                if c != 0 {
                    return Err(format!("unexpected primitive {k}").into());
                }
                0
            }
        };
        total += c * p;
    }
    Ok(total)
}
fn check_corpus(
    m: &Matrix,
    record: &Value,
    method: &str,
    expected: &[u64],
    rank: usize,
) -> Result<Value, E> {
    if record["foreign_thread_hooks"] != 0
        || record["other_modulus_events"]
            .as_array()
            .ok_or("other moduli")?
            .iter()
            .any(|x| x != 0)
    {
        return Err("counting ownership/modulus".into());
    }
    let r = &record["result"];
    if r["method"] != method {
        return Err("method mismatch".into());
    }
    let h = if r["kernel"].is_null() {
        None
    } else {
        Some(vector(&r["kernel"])?)
    };
    if let Some(h) = &h {
        if h != expected || !residual(m, h) || !r["refusal"].is_null() {
            return Err("kernel mismatch".into());
        }
    } else if r["refusal"].as_str().is_none() {
        return Err("missing refusal".into());
    }
    if method == "library-wiedemann" {
        if !r["rank_certificate"].is_null() {
            return Err("unsupported library rank certificate".into());
        }
    } else if h.is_some() && r["rank_certificate"] != rank {
        return Err("rank certificate mismatch".into());
    }
    let attempts = r["attempts"].as_array().ok_or("attempts")?;
    if method != "counted-rref" && (attempts.is_empty() || attempts.len() > 3) {
        return Err("attempt cap".into());
    }
    if method == "counted-rref" && !attempts.is_empty() {
        return Err("RREF attempts".into());
    }
    let mut last = None;
    for (i, a) in attempts.iter().enumerate() {
        if a["index"] != i {
            return Err("attempt ordering".into());
        }
        if last.is_some() {
            return Err("attempt after success".into());
        }
        if method == "randomized-lanczos" {
            last = lanczos_attempt(m, a)?;
        }
    }
    if method == "randomized-lanczos" && last != h {
        return Err("trace/output mismatch".into());
    }
    let p = &record["primitives"];
    let recomputed = price(p)?;
    if record["observed_incomplete_subtotal"]
        .as_str()
        .ok_or("subtotal")?
        .parse::<u128>()?
        != recomputed
    {
        return Err("price mismatch".into());
    }
    let mut sums = BTreeMap::<String, u64>::new();
    for v in record["phases"].as_object().ok_or("phases")?.values() {
        for (k, c) in v.as_object().ok_or("phase counts")? {
            *sums.entry(k.clone()).or_default() += u(c)?;
        }
    }
    for (k, v) in p.as_object().ok_or("primitive counts")? {
        if sums.get(k) != Some(&u(v)?) {
            return Err("phase conservation".into());
        }
    }
    let uniform = &record["uniform"];
    if u(&uniform["raw_draws"])? != u(&uniform["calls"])? + u(&uniform["rejections"])?
        || uniform["other_modulus_calls"] != 0
        || u(&p["rng_words"])? != u(&uniform["raw_draws"])?
    {
        return Err("uniform conservation".into());
    }
    for (own, global) in [
        ("products", "modr_mul"),
        ("additions", "modr_add"),
        ("inversions", "modr_inv"),
        ("words", "word_op"),
    ] {
        if u(&r["counts"][own])? > u(&p[global])? {
            return Err("explicit counter exceeds global tally".into());
        }
    }
    if method == "randomized-lanczos" {
        let steps: usize = attempts
            .iter()
            .map(|a| a["steps"].as_array().map_or(0, Vec::len))
            .sum();
        if u(&r["counts"]["inversions"])? != steps as u64
            || u(&p["modr_inv"])? != steps as u64
            || r["legacy_products"] != 0
            || u(&r["counts"]["draws"])? != u(&uniform["calls"])?
            || u(&r["counts"]["additions"])? != u(&p["modr_add"])?
        {
            return Err("Lanczos scalar event conservation".into());
        }
    }
    Ok(
        json!({"method":method,"verified_kernel":h.is_some(),"rank":rank,"attempts":attempts.len(),"trace_replayed":method=="randomized-lanczos","observed_incomplete_subtotal":recomputed.to_string(),"primitives":p,"inner_wall_ns":record["inner_wall_ns"]}),
    )
}
fn run(dir: &Path, out: &Path) -> Result<(), E> {
    let bytes = fs::read(dir.join("input.json"))?;
    let input: Input = serde_json::from_slice(&bytes)?;
    if input.schema != "cover.sparse-kernel.matrices/v1"
        || input.origin_revision != "fcda1e22cb0c5424ca3244f783e69be8b99792f4"
        || input.corpora.len() != 2
    {
        return Err("input identity".into());
    }
    let source = fs::read_to_string(dir.join("source-revision.txt"))?;
    let mut rows = Vec::new();
    for (i, c) in input.corpora.iter().enumerate() {
        validate(&c.matrix)?;
        if c.name != ["full-smooth", "quadratic-remainders"][i]
            || c.origin_sha256.len() != 64
            || c.matrix.columns != 509
            || c.matrix.rows.len() != [509, 510][i]
        {
            return Err("registered corpus".into());
        }
        let (rank, h) = dense_kernel(&c.matrix)?;
        for method in ["counted-rref", "library-wiedemann", "randomized-lanczos"] {
            if fs::read_to_string(dir.join(format!("{method}.exit-code.txt")))?.trim() != "0" {
                return Err("worker exit".into());
            }
            let worker: Value =
                serde_json::from_slice(&fs::read(dir.join(format!("{method}.raw.json")))?)?;
            if worker["schema"] != "cover.sparse-kernel.component/v1"
                || worker["method"] != method
                || worker["source_revision"] != source.trim()
                || worker["input_sha256"] != hash(&bytes)
                || worker["seed"] != 443
                || worker["max_attempts"] != 3
                || worker["complete_cost"] != false
                || !worker["complete_total"].is_null()
                || !worker["ratios"].is_null()
            {
                return Err("worker registration/scope".into());
            }
            if worker["corpora"].as_array().ok_or("corpora")?.len() != 2
                || worker["corpora"][i]["corpus"] != c.name
            {
                return Err("worker corpus".into());
            }
            let checked = check_corpus(&c.matrix, &worker["corpora"][i], method, &h, rank)?;
            rows.push(json!({"corpus":c.name,"rows":c.matrix.rows.len(),"columns":c.matrix.columns,"nonzeros":c.matrix.rows.iter().map(Vec::len).sum::<usize>(),"kernel_sha256":hash(&serde_json::to_vec(&h)?),"checked":checked}));
        }
    }
    let value = json!({"schema":"cover.sparse-kernel.qualified/v1","source_revision":source.trim(),"input_sha256":hash(&bytes),"complete_cost":false,"complete_total":null,"ratios":null,"scope":"recorded homogeneous matrix component; independent dense rank/kernel and Lanczos trace replay; partial counter conservation; no signed execution proof","rows":rows});
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(out)?;
    serde_json::to_writer_pretty(&mut f, &value)?;
    f.write_all(b"\n")?;
    Ok(())
}
fn main() -> Result<(), E> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 2 {
        return Err("usage: CHECK RUN NEW_OUTPUT".into());
    }
    run(Path::new(&a[0]), Path::new(&a[1]))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> Matrix {
        Matrix {
            modulus: 263807419614139553,
            columns: 3,
            rows: vec![
                vec![(0, 1), (2, 263807419614139551)],
                vec![(1, 1), (2, 263807419614139550)],
                vec![(0, 1), (1, 1), (2, 263807419614139548)],
            ],
        }
    }
    #[test]
    fn independent_dense_replay_and_bad_kernel() {
        let m = fixture();
        let (r, h) = dense_kernel(&m).unwrap();
        assert_eq!(r, 2);
        assert_eq!(h, [2, 3, 1]);
        assert!(residual(&m, &h));
        assert!(!residual(&m, &[2, 4, 1]));
    }
    #[test]
    fn deficient_input_refused() {
        let mut m = fixture();
        m.rows = vec![m.rows[0].clone(); 3];
        assert!(dense_kernel(&m).is_err());
    }
    #[test]
    fn inverse_and_price_controls() {
        let f = F(101);
        assert_eq!(f.inv(3).unwrap(), 34);
        assert!(f.inv(0).is_err());
        assert_eq!(
            price(&json!({"modr_mul":1,"modr_add":1,"modr_inv":1,"word_op":1,"rng_words":1}))
                .unwrap(),
            128824
        );
        assert!(price(&json!({"field_mul":1})).is_err());
    }
    #[test]
    fn strict_matrix_boundary() {
        let mut v = serde_json::to_value(json!({"modulus":101,"columns":2,"rows":[[[0,1],[1,1]]]}))
            .unwrap();
        v["rank"] = json!(1);
        assert!(serde_json::from_value::<Matrix>(v).is_err());
    }
    fn trace_fixture() -> (Matrix, Value) {
        let m = Matrix {
            modulus: 263807419614139553,
            columns: 2,
            rows: vec![vec![(0, 1), (1, 263807419614139551)]],
        };
        let a = json!({"index":0,"iterations":1,"refusal":null,"steps":[{"diagonal":4,"alpha":1,"beta":0,"projection":1}],"row_weights":[1],"column_weights":[1],"rhs_shift":[0]});
        (m, a)
    }
    #[test]
    fn trace_replay_and_corruptions() {
        let (m, a) = trace_fixture();
        assert_eq!(lanczos_attempt(&m, &a).unwrap(), Some(vec![2, 1]));
        for key in ["diagonal", "alpha", "beta", "projection"] {
            let mut bad = a.clone();
            bad["steps"][0][key] = json!(99);
            assert!(lanczos_attempt(&m, &bad).is_err());
        }
        let mut bad = a.clone();
        bad["column_weights"] = json!([0]);
        assert!(lanczos_attempt(&m, &bad).is_err());
        let mut bad = a.clone();
        bad["iterations"] = json!(0);
        assert!(lanczos_attempt(&m, &bad).is_err());
        let mut bad = a;
        bad["steps"] = json!([]);
        bad["iterations"] = json!(0);
        bad["refusal"] = json!("self-orthogonal Lanczos direction");
        assert!(lanczos_attempt(&m, &bad).is_err());
    }
    #[test]
    fn counting_and_endpoint_corruptions() {
        let (m, a) = trace_fixture();
        let p = json!({"modr_mul":1,"modr_add":1,"modr_inv":1,"word_op":1,"rng_words":0});
        let record = json!({"foreign_thread_hooks":0,"other_modulus_events":[0],"result":{"method":"randomized-lanczos","kernel":[2,1],"refusal":null,"rank_certificate":1,"attempts":[a],"legacy_products":0,"counts":{"products":1,"additions":1,"inversions":1,"words":1,"draws":0}},"primitives":p,"phases":{"linear_algebra":p},"uniform":{"calls":0,"raw_draws":0,"rejections":0,"other_modulus_calls":0},"observed_incomplete_subtotal":"128824"});
        assert!(check_corpus(&m, &record, "randomized-lanczos", &[2, 1], 1).is_ok());
        for path in [
            "/foreign_thread_hooks",
            "/other_modulus_events/0",
            "/result/kernel/0",
            "/result/rank_certificate",
            "/result/counts/additions",
            "/result/counts/inversions",
            "/uniform/other_modulus_calls",
            "/primitives/word_op",
            "/phases/linear_algebra/modr_add",
        ] {
            let mut bad = record.clone();
            *bad.pointer_mut(path).unwrap() = json!(99);
            assert!(
                check_corpus(&m, &bad, "randomized-lanczos", &[2, 1], 1).is_err(),
                "{path}"
            );
        }
        let mut bad = record;
        bad["observed_incomplete_subtotal"] = json!("128825");
        assert!(check_corpus(&m, &bad, "randomized-lanczos", &[2, 1], 1).is_err());
    }
}

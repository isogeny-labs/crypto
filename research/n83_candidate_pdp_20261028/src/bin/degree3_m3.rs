//! Bounded ordinary-degree-three closure for the retained 164-variable m3 systems.
//! Frozen m2 instrument remains in degree3.rs.
const MAX_ROWS: usize = 32768;
const MAX_OPERATIONS: usize = 131072;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeSet, VecDeque},
    fs::{self, File},
    io::{BufRead, BufReader, BufWriter, Read, Write},
    path::Path,
};
type Poly = BTreeSet<Vec<usize>>;
fn sha(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn read(p: &Path) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: &Path, v: &Value) {
    fs::write(p, serde_json::to_string_pretty(v).unwrap() + "\n").unwrap()
}
fn toggle(p: &mut Poly, m: Vec<usize>) {
    if !p.remove(&m) {
        p.insert(m);
    }
}
fn parse(expr: &str, n: usize) -> Poly {
    let mut p = Poly::new();
    if expr.contains('^') {
        let (a, b) = expr
            .split_once("^2-")
            .expect("only field equations use powers");
        assert_eq!(a, b);
        let i: usize = a
            .strip_prefix("v(")
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .parse()
            .unwrap();
        assert!((1..=n).contains(&i));
        return p;
    }
    for term in expr.split('+') {
        if term == "0" {
            continue;
        }
        let mut m = vec![];
        if term != "1" {
            for v in term.split('*') {
                let i: usize = v
                    .strip_prefix("v(")
                    .unwrap()
                    .strip_suffix(')')
                    .unwrap()
                    .parse()
                    .unwrap();
                assert!((1..=n).contains(&i));
                m.push(i - 1)
            }
        }
        m.sort_unstable();
        assert!(
            m.windows(2).all(|w| w[0] != w[1]),
            "non-field input must already be multilinear"
        );
        toggle(&mut p, m)
    }
    p
}
fn degree(p: &Poly) -> usize {
    p.iter().map(Vec::len).max().unwrap_or(0)
}
fn equations(text: &str, n: usize) -> Vec<(usize, Poly)> {
    let fields: BTreeSet<usize> = text
        .lines()
        .filter(|l| l.starts_with("I[") && l.contains("^2-v("))
        .map(|l| {
            let expr = l.split_once('=').unwrap().1;
            expr.split_once("^2-")
                .unwrap()
                .0
                .strip_prefix("v(")
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .parse()
                .unwrap()
        })
        .collect();
    assert_eq!(
        fields,
        (1..=n).collect(),
        "all original Boolean field equations required"
    );
    let result: Vec<(usize, Poly)> = text
        .lines()
        .filter(|l| l.starts_with("I["))
        .map(|l| {
            let (a, b) = l.split_once('=').unwrap();
            (
                a[2..a.len() - 1].parse().unwrap(),
                parse(b.strip_suffix(';').unwrap(), n),
            )
        })
        .collect();
    let ids: BTreeSet<_> = result.iter().map(|(id, _)| *id).collect();
    assert_eq!(
        ids,
        (1..=result.len()).collect(),
        "unique contiguous equation IDs required"
    );
    result
}
// Graded colex squarefree columns, valid only with all saved Boolean field equations.
struct Space {
    n: usize,
    cap: usize,
    cols: usize,
    words: usize,
    quadratic_cols: usize,
    products: Vec<Vec<usize>>,
}
fn choose2(n: usize) -> usize {
    n * n.saturating_sub(1) / 2
}
fn choose3(n: usize) -> usize {
    n * n.saturating_sub(1) * n.saturating_sub(2) / 6
}
impl Space {
    fn new(n: usize, cap: usize) -> Self {
        assert!((1..=164).contains(&n) && (cap == 2 || cap == 3));
        let quadratic_cols = 1 + n + choose2(n);
        let cols = quadratic_cols + if cap == 3 { choose3(n) } else { 0 };
        let mut sp = Self {
            n,
            cap,
            cols,
            words: cols.div_ceil(64),
            quadratic_cols,
            products: vec![],
        };
        let mut mons: Vec<Vec<usize>> = vec![vec![]];
        for i in 0..n {
            mons.push(vec![i]);
        }
        if cap == 3 {
            for j in 1..n {
                for i in 0..j {
                    mons.push(vec![i, j]);
                }
            }
        }
        sp.products = (0..n)
            .map(|v| {
                mons.iter()
                    .map(|m| {
                        let mut product = m.clone();
                        if let Err(i) = product.binary_search(&v) {
                            product.insert(i, v);
                        }
                        sp.col(&product)
                    })
                    .collect()
            })
            .collect();
        sp
    }
    fn col(&self, m: &[usize]) -> usize {
        assert!(m.len() <= self.cap);
        assert!(m.iter().all(|i| *i < self.n));
        assert!(m.windows(2).all(|w| w[0] < w[1]));
        match m {
            [] => 0,
            [i] => 1 + i,
            [i, j] => 1 + self.n + choose2(*j) + i,
            [i, j, k] => self.quadratic_cols + choose3(*k) + choose2(*j) + i,
            _ => unreachable!(),
        }
    }
    fn row(&self, p: &Poly) -> Vec<u64> {
        let mut row = vec![0; self.words];
        for m in p {
            let c = self.col(m);
            row[c / 64] ^= 1 << (c % 64);
        }
        row
    }
    fn deg(&self, row: &[u64]) -> usize {
        match pivot(row) {
            None | Some(0) => 0,
            Some(c) if c <= self.n => 1,
            Some(c) if c < self.quadratic_cols => 2,
            _ => 3,
        }
    }
    fn mul(&self, row: &[u64], v: usize) -> Vec<u64> {
        assert!(
            v < self.n && self.deg(row) < self.cap,
            "ordinary multiplication degree exceeds cap"
        );
        let mut out = vec![0; self.words];
        for (word, &bits) in row.iter().enumerate() {
            let mut bits = bits;
            while bits != 0 {
                let c = word * 64 + bits.trailing_zeros() as usize;
                bits &= bits - 1;
                let at = self.products[v][c];
                out[at / 64] ^= 1 << (at % 64);
            }
        }
        out
    }
}
fn bit(r: &[u64], c: usize) -> bool {
    r[c / 64] >> (c % 64) & 1 != 0
}
fn xor(a: &mut [u64], b: &[u64]) {
    for (x, y) in a.iter_mut().zip(b) {
        *x ^= *y;
    }
}
fn pivot(r: &[u64]) -> Option<usize> {
    r.iter()
        .enumerate()
        .rev()
        .find(|(_, w)| **w != 0)
        .map(|(i, w)| 64 * i + 63 - w.leading_zeros() as usize)
}
fn sources(polys: &[(usize, Poly)], cap: usize) -> VecDeque<Value> {
    polys
        .iter()
        .filter(|(_, p)| !p.is_empty() && degree(p) <= cap)
        .map(|(id, _)| json!({"equation":id}))
        .collect()
}
fn candidate(s: &Value, rows: &[Vec<u64>], polys: &[(usize, Poly)], sp: &Space) -> Vec<u64> {
    if let Some(id) = s["equation"].as_u64() {
        sp.row(&polys.iter().find(|(i, _)| *i == id as usize).unwrap().1)
    } else {
        sp.mul(
            &rows[s["parent"].as_u64().unwrap() as usize],
            s["variable"].as_u64().unwrap() as usize,
        )
    }
}
fn append_multiples(q: &mut VecDeque<Value>, row: &[u64], id: usize, sp: &Space) {
    if sp.deg(row) < sp.cap {
        for v in 0..sp.n {
            q.push_back(json!({"parent":id,"variable":v}));
        }
    }
}
fn evaluate(p: &Poly, values: &[bool]) -> bool {
    p.iter()
        .fold(false, |a, m| a ^ m.iter().all(|v| values[*v]))
}
fn affine_verdict(rows: &[Vec<u64>], sp: &Space, polys: &[(usize, Poly)]) -> Value {
    let has_unit = rows.iter().any(|r| pivot(r) == Some(0));
    let linear: Vec<_> = rows
        .iter()
        .enumerate()
        .filter(|(_, r)| sp.deg(r) == 1)
        .collect();
    let pivots: BTreeSet<_> = linear.iter().map(|(_, r)| pivot(r).unwrap() - 1).collect();
    let free: Vec<_> = (0..sp.n).filter(|i| !pivots.contains(i)).collect();
    let lower = polys.iter().all(|(_, p)| p.is_empty() || degree(p) >= 2)
        && rows.iter().any(|r| sp.deg(r) < 2);
    let mut solutions = vec![];
    let mut all_valid = None;
    if !has_unit && free.len() <= 20 {
        let mut linear_sorted = linear.clone();
        linear_sorted.sort_by_key(|(_, r)| pivot(r));
        let mut valid = true;
        for assignment in 0..1usize << free.len() {
            let mut values = vec![false; sp.n];
            for (j, v) in free.iter().enumerate() {
                values[*v] = assignment >> j & 1 != 0
            }
            for (_, r) in &linear_sorted {
                let p = pivot(r).unwrap();
                let rhs = bit(r, 0)
                    ^ (1..p)
                        .filter(|c| bit(r, *c))
                        .fold(false, |a, c| a ^ values[c - 1]);
                values[p - 1] = rhs;
            }
            let ok = polys.iter().all(|(_, p)| !evaluate(p, &values));
            valid &= ok;
            // Store every checked assignment, including counterexamples.
            solutions.push(json!({"free_assignment":assignment,"values_hex_words":values.chunks(64).map(|v|format!("{:016x}",v.iter().enumerate().fold(0u64,|a,(i,b)|a|((*b as u64)<<i)))).collect::<Vec<_>>(),"satisfies_all_input_equations":ok}));
        }
        all_valid = Some(valid);
    }
    let upper = has_unit || all_valid == Some(true);
    json!({"closure_degree":2,"closure_rank":rows.len(),"columns":sp.cols,"contains_one":has_unit,"linear_rank":linear.len(),"linear_basis_row_ids":linear.iter().map(|(i,_)|*i).collect::<Vec<_>>(),"free_variables_zero_based":free,"exhaustive_affine_completion":all_valid,"assignments":solutions,"last_fall_degree":if lower&&upper{Some(2)}else{None},"last_fall_lower_bound":if lower{Some(2)}else{None},"upper_bound_argument":if has_unit{"1 in V2 implies every polynomial is in Vmax(degree,2)"}else if upper{"Linear constraints in V2 plus original field equations form an ideal equal to F; exhaustive affine-space check; degree-compatible linear substitution gives bound2"}else{"No global stabilization certificate; a degree-two closure plateau is not a last-fall certificate"},"convention":"Ordinary polynomial ring over F2 with all Boolean field equations, computed through multilinear normal forms; no arbitrary Boolean-product degree shortcut"})
}
fn produce(input: &Value, text: &str, out: &Path, cap: usize) -> (Vec<Vec<u64>>, usize) {
    produce_limited(
        input,
        text,
        out,
        cap,
        if cap == 3 { MAX_ROWS } else { usize::MAX },
        if cap == 3 { MAX_OPERATIONS } else { usize::MAX },
    )
}
fn produce_limited(
    input: &Value,
    text: &str,
    out: &Path,
    cap: usize,
    max_rows: usize,
    max_operations: usize,
) -> (Vec<Vec<u64>>, usize) {
    assert!(max_rows > 0 && max_operations > 0);
    let n = input["n_vars"].as_u64().unwrap() as usize;
    let sp = Space::new(n, cap);
    let polys = equations(text, n);
    assert_eq!(
        polys.len(),
        input["input_equations"].as_u64().unwrap() as usize
    );
    fs::create_dir(out).unwrap();
    let mut data = BufWriter::new(File::create(out.join("basis.rows.u64le")).unwrap());
    data.write_all(if cap == 2 { b"N83V2R1\n" } else { b"N83V3R1\n" })
        .unwrap();
    data.write_all(&(n as u32).to_le_bytes()).unwrap();
    data.write_all(&(sp.cols as u64).to_le_bytes()).unwrap();
    data.write_all(&(sp.words as u64).to_le_bytes()).unwrap();
    let mut ops = BufWriter::new(File::create(out.join("operations.jsonl")).unwrap());
    let mut progress = BufWriter::new(File::create(out.join("progress.jsonl")).unwrap());
    let mut q = sources(&polys, cap);
    let mut rows: Vec<Vec<u64>> = vec![];
    let mut pivots: Vec<Option<usize>> = vec![None; sp.cols];
    let mut steps = 0;
    while !q.is_empty() && rows.len() < max_rows && steps < max_operations {
        let source = q.pop_front().unwrap();
        let mut row = candidate(&source, &rows, &polys, &sp);
        let mut reducers = vec![];
        while let Some(p) = pivot(&row) {
            if let Some(id) = pivots[p] {
                xor(&mut row[..=p / 64], &rows[id][..=p / 64]);
                reducers.push(id);
            } else {
                break;
            }
        }
        let inserted = if let Some(p) = pivot(&row) {
            let id = rows.len();
            pivots[p] = Some(id);
            append_multiples(&mut q, &row, id, &sp);
            for w in &row {
                data.write_all(&w.to_le_bytes()).unwrap();
            }
            rows.push(row);
            Some(id)
        } else {
            None
        };
        writeln!(
            ops,
            "{}",
            json!({"step":steps,"source":source,"xor_basis_rows":reducers,"inserted_row":inserted})
        )
        .unwrap();
        steps += 1;
        if steps % 128 == 0 {
            ops.flush().unwrap();
            data.flush().unwrap();
            writeln!(
                progress,
                "{}",
                json!({"steps":steps,"rank":rows.len(),"pending":q.len(),"complete":false})
            )
            .unwrap();
            progress.flush().unwrap();
        }
    }
    ops.flush().unwrap();
    data.flush().unwrap();
    writeln!(
        progress,
        "{}",
        json!({"steps":steps,"rank":rows.len(),"pending":q.len(),"complete":q.is_empty()})
    )
    .unwrap();
    progress.flush().unwrap();
    if cap == 3 {
        save(
            &out.join("closure_state.json"),
            &json!({
                "complete":q.is_empty(),"pending_candidates":q.len(),
                "max_rows":max_rows,"max_operations":max_operations,
                "stop_reason":if q.is_empty(){"queue_exhausted"}else if rows.len()>=max_rows{"row_cap"}else{"operation_cap"},
                "next_source":q.front(),"processed_candidates":steps,"rank":rows.len()
            }),
        );
    }
    if cap == 2 {
        assert!(q.is_empty());
        let mut result = affine_verdict(&rows, &sp, &polys);
        result["input"] = input.clone();
        result["system_sha256"] = json!(sha(text.as_bytes()));
        result["processed_candidates"] = json!(steps);
        result["closure_complete"] = json!(true);
        save(&out.join("result.json"), &result);
    }
    (rows, steps)
}
fn replay_space(input: &Value, text: &str, out: &Path, cap: usize) -> (Vec<Vec<u64>>, usize) {
    let n = input["n_vars"].as_u64().unwrap() as usize;
    let sp = Space::new(n, cap);
    let polys = equations(text, n);
    let mut q = sources(&polys, cap);
    let mut rows: Vec<Vec<u64>> = vec![];
    let mut data = BufReader::new(File::open(out.join("basis.rows.u64le")).unwrap());
    let mut header = [0; 28];
    data.read_exact(&mut header).unwrap();
    assert_eq!(
        &header[..8],
        if cap == 2 { b"N83V2R1\n" } else { b"N83V3R1\n" }
    );
    assert_eq!(
        u32::from_le_bytes(header[8..12].try_into().unwrap()) as usize,
        n
    );
    assert_eq!(
        u64::from_le_bytes(header[12..20].try_into().unwrap()) as usize,
        sp.cols
    );
    assert_eq!(
        u64::from_le_bytes(header[20..28].try_into().unwrap()) as usize,
        sp.words
    );
    let mut pivots = BTreeSet::new();
    let mut count = 0;
    for line in BufReader::new(File::open(out.join("operations.jsonl")).unwrap()).lines() {
        let op: Value = serde_json::from_str(&line.unwrap()).unwrap();
        assert_eq!(op["step"], count);
        let expected = q.pop_front().expect("extra operation");
        assert_eq!(op["source"], expected);
        let mut row = candidate(&expected, &rows, &polys, &sp);
        for id in op["xor_basis_rows"].as_array().unwrap() {
            let id = id.as_u64().unwrap() as usize;
            assert!(id < rows.len());
            let last = pivot(&rows[id]).expect("nonzero basis row") / 64;
            xor(&mut row[..=last], &rows[id][..=last]);
        }
        if let Some(id) = op["inserted_row"].as_u64() {
            assert_eq!(id as usize, rows.len());
            assert!(pivots.insert(pivot(&row).expect("zero inserted")));
            let mut saved = vec![0u64; sp.words];
            for w in &mut saved {
                let mut bytes = [0; 8];
                data.read_exact(&mut bytes).unwrap();
                *w = u64::from_le_bytes(bytes);
            }
            assert_eq!(row, saved);
            append_multiples(&mut q, &row, rows.len(), &sp);
            rows.push(row);
        } else {
            assert!(pivot(&row).is_none());
        }
        count += 1;
    }
    let complete = if cap == 2 {
        assert!(q.is_empty());
        true
    } else {
        let state = read(&out.join("closure_state.json"));
        let max_rows = state["max_rows"].as_u64().unwrap() as usize;
        let max_operations = state["max_operations"].as_u64().unwrap() as usize;
        assert!(max_rows > 0 && max_operations > 0);
        assert!(rows.len() <= max_rows && count <= max_operations);
        assert_eq!(state["rank"], rows.len());
        assert_eq!(state["processed_candidates"], count);
        assert_eq!(state["pending_candidates"], q.len());
        assert_eq!(state["next_source"], json!(q.front()));
        assert_eq!(state["complete"], q.is_empty());
        assert!(
            q.is_empty() || rows.len() == max_rows || count == max_operations,
            "unjustified early termination"
        );
        assert_eq!(
            state["stop_reason"],
            if q.is_empty() {
                "queue_exhausted"
            } else if rows.len() == max_rows {
                "row_cap"
            } else {
                "operation_cap"
            }
        );
        q.is_empty()
    };
    let mut tail = [0; 1];
    assert_eq!(data.read(&mut tail).unwrap(), 0);
    let expected = read(&out.join("result.json"));
    assert_eq!(expected["system_sha256"], sha(text.as_bytes()));
    assert_eq!(expected["input"], *input);
    assert_eq!(expected["closure_complete"], complete);
    assert_eq!(expected["closure_degree"], cap);
    assert_eq!(expected["processed_candidates"], count);
    assert_eq!(expected["closure_rank"], rows.len());
    if cap == 2 {
        for (k, v) in affine_verdict(&rows, &sp, &polys).as_object().unwrap() {
            assert_eq!(&expected[k], v, "V2 verdict mismatch {k}");
        }
    }
    (rows, count)
}
fn occupied(row: &[u64]) -> Vec<usize> {
    row.iter()
        .enumerate()
        .flat_map(|(i, &w)| {
            let mut w = w;
            let mut c = vec![];
            while w != 0 {
                c.push(i * 64 + w.trailing_zeros() as usize);
                w &= w - 1;
            }
            c
        })
        .collect()
}
fn strict_fall(rows: &[Vec<u64>], v2: &[Vec<u64>], n: usize) -> Value {
    let s2 = Space::new(n, 2);
    let s3 = Space::new(n, 3);
    let mut pivots = vec![None; s2.cols];
    for (id, r) in v2.iter().enumerate() {
        let p = pivot(r).unwrap();
        assert!(pivots[p].replace(id).is_none());
    }
    for (id, row) in rows.iter().enumerate() {
        if s3.deg(row) > 2 {
            continue;
        }
        let original = row[..s2.words].to_vec();
        let mut rem = original.clone();
        let mut reductions = vec![];
        while let Some(p) = pivot(&rem) {
            assert!(p < s2.cols);
            if let Some(i) = pivots[p] {
                xor(&mut rem, &v2[i]);
                reductions.push(i);
            } else {
                break;
            }
        }
        if pivot(&rem).is_some() {
            return json!({"v3_row_id":id,"polynomial_columns":occupied(&original),"v2_xor_basis_rows":reductions,"nonzero_v2_remainder_columns":occupied(&rem),"polynomial_degree":s2.deg(&original),"argument":"This row is in V3 by the full replay trace and outside complete V2 by nonzero echelon remainder; degree at most2 implies a degree3 fall and LFD at least3."});
        }
    }
    Value::Null
}
fn final_verdict(
    input: &Value,
    text: &str,
    rows: &[Vec<u64>],
    v2: &[Vec<u64>],
    steps: usize,
    state: &Value,
) -> Value {
    let n = input["n_vars"].as_u64().unwrap() as usize;
    let polys = equations(text, n);
    let s3 = Space::new(n, 3);
    let old = affine_verdict(v2, &Space::new(n, 2), &polys);
    let mut result = affine_verdict(rows, &s3, &polys);
    let witness = strict_fall(rows, v2, n);
    let higher = !witness.is_null();
    let upper = result["contains_one"] == true || result["exhaustive_affine_completion"] == true;
    let prior_exact = old["last_fall_degree"].as_u64();
    if prior_exact.is_some() {
        assert!(
            !higher,
            "V2 upper certificate contradicted by strict V3 fall"
        );
    }
    let exact = if let Some(d) = prior_exact {
        Some(d)
    } else if higher && upper {
        Some(3)
    } else {
        None
    };
    result["closure_degree"] = json!(3);
    result["last_fall_degree"] = json!(exact);
    result["last_fall_lower_bound"] = if higher {
        json!(3)
    } else {
        old["last_fall_lower_bound"].clone()
    };
    result["last_fall_upper_bound"] = if let Some(d) = prior_exact {
        json!(d)
    } else if upper {
        json!(3)
    } else {
        Value::Null
    };
    result["upper_bound_argument"] = json!(if prior_exact.is_some() {
        "Verified complete V2 already supplies exact LFD2 and a global upper certificate"
    } else if result["contains_one"] == true {
        "1 in V3 implies all ideal polynomials are in Vmax(3,degree)"
    } else if upper {
        "Linear constraints in V3 and original field equations generate the entire input ideal: every affine-space assignment was checked against every original equation. Degree-compatible substitution gives global upper bound3."
    } else {
        "No global LFD upper certificate; a closure plateau is insufficient"
    });
    result["strict_degree_three_fall"] = witness;
    result["v2_closure_rank"] = json!(v2.len());
    result["v2_linear_rank"] = old["linear_rank"].clone();
    result["input"] = input.clone();
    result["system_sha256"] = json!(sha(text.as_bytes()));
    result["processed_candidates"] = json!(steps);
    result["closure_complete"] = state["complete"].clone();
    result["closure_state"] = state.clone();
    result["schema"] = json!("n83.m3-degree-three-closure/v1");
    result
}
fn load_case(root: &Path, id: &str) -> (Value, String) {
    let input = read(&root.join(id).join("input.json"));
    assert!(input["family"] == "koblitz" || input["family"] == "binary");
    assert_eq!(input["m"], 3);
    assert_eq!(input["n_vars"], 164);
    assert_eq!(input["ell"], 27);
    let raw = fs::read(root.join(id).join("system.sing")).unwrap();
    assert_eq!(sha(&raw), input["source_sha256"]);
    let text = String::from_utf8(raw).unwrap();
    assert_eq!(text.lines().next().unwrap(), "ring r=2,(v(1..164)),dp;");
    (input, text)
}
fn v2_refs(path: &Path) -> Value {
    json!({"result_sha256":sha(&fs::read(path.join("result.json")).unwrap()),"operations_sha256":sha(&fs::read(path.join("operations.jsonl")).unwrap()),"rows_sha256":sha(&fs::read(path.join("basis.rows.u64le")).unwrap())})
}
fn run_case(input: &Value, text: &str, v2dir: &Path, out: &Path) -> Value {
    let (v2, v2steps) = replay_space(input, text, v2dir, 2);
    let (rows, steps) = produce(input, text, out, 3);
    let mut result = final_verdict(
        input,
        text,
        &rows,
        &v2,
        steps,
        &read(&out.join("closure_state.json")),
    );
    result["v2_reference"] = v2_refs(v2dir);
    save(&out.join("result.json"), &result);
    save(
        &out.join("v2-replay.json"),
        &json!({"verified":true,"rows":v2.len(),"operations":v2steps,"reference":result["v2_reference"]}),
    );
    result
}
fn replay_case(input: &Value, text: &str, v2dir: &Path, out: &Path) -> Value {
    let (v2, _) = replay_space(input, text, v2dir, 2);
    let (rows, steps) = replay_space(input, text, out, 3);
    let mut actual = final_verdict(
        input,
        text,
        &rows,
        &v2,
        steps,
        &read(&out.join("closure_state.json")),
    );
    actual["v2_reference"] = v2_refs(v2dir);
    assert_eq!(actual, read(&out.join("result.json")));
    json!({"verified":true,"input":input["id"],"rows":rows.len(),"operations":steps,"strict_degree_three_fall":!actual["strict_degree_three_fall"].is_null(),"last_fall_degree":actual["last_fall_degree"],"producer_replay":true,"independent_review":false})
}
fn control_text(eqs: &[&str], n: usize) -> String {
    let mut text = String::new();
    for (i, e) in eqs.iter().enumerate() {
        text += &format!("I[{}]={e};\n", i + 1);
    }
    for i in 1..=n {
        text += &format!("I[{}]=v({i})^2-v({i});\n", eqs.len() + i);
    }
    text
}
fn controls(out: &Path) -> Value {
    fs::create_dir(out).unwrap();
    let mut results = vec![];
    for (name, eqs, n, exact) in [
        (
            "strict-three",
            vec!["v(1)*v(2)+1", "v(2)*v(3)+1", "v(1)*v(3)"],
            3,
            Some(3),
        ),
        ("nonlinear-no-upper", vec!["v(1)*v(2)"], 3, None),
        (
            "degree-two-unit",
            vec!["v(1)*v(2)", "v(1)*v(2)+1"],
            2,
            Some(2),
        ),
    ] {
        let text = control_text(&eqs, n);
        let input = json!({"id":name,"n_vars":n,"input_equations":n+eqs.len()});
        let dir = out.join(name);
        fs::create_dir(&dir).unwrap();
        fs::write(dir.join("system.sing"), &text).unwrap();
        save(&dir.join("input.json"), &input);
        produce(&input, &text, &dir.join("v2"), 2);
        let result = run_case(&input, &text, &dir.join("v2"), &dir.join("v3"));
        assert_eq!(result["last_fall_degree"], json!(exact));
        let replay = replay_case(&input, &text, &dir.join("v2"), &dir.join("v3"));
        results.push(json!({"case":name,"expected_lfd":exact,"replay":replay}));
    }
    json!({"passed":true,"controls":results})
}
fn summary(root: &Path) {
    let protocol = read(&root.join("protocol-degree3-m3.json"));
    let mut rows = vec![];
    for case in protocol["extensions"]["cases"].as_array().unwrap() {
        let key = case["key"].as_str().unwrap();
        let status_path = root.join("status").join(format!("{key}.json"));
        let status = if status_path.exists() {
            read(&status_path)
        } else {
            json!({"run_exit_code":null,"replay_exit_code":null})
        };
        let result_path = root.join("cases").join(key).join("result.json");
        let result = if result_path.exists() {
            read(&result_path)
        } else {
            Value::Null
        };
        let verified = status["run_exit_code"] == 0 && status["replay_exit_code"] == 0;
        rows.push(json!({"key":key,"arm":case["arm"],"id":case["id"],"family":case["family"],"target_kind":case["target_kind"],"m":3,"n":83,"ell":27,"status":status,"verified":verified,"closure_complete":if verified{result["closure_complete"].clone()}else{Value::Null},"closure_state":result["closure_state"],"last_fall_degree":if verified{result["last_fall_degree"].clone()}else{Value::Null},"last_fall_lower_bound":if verified{result["last_fall_lower_bound"].clone()}else{Value::Null},"last_fall_upper_bound":if verified{result["last_fall_upper_bound"].clone()}else{Value::Null},"strict_degree_three_fall":verified && !result["strict_degree_three_fall"].is_null(),"closure_rank":if verified{result["closure_rank"].clone()}else{Value::Null},"linear_rank":if verified{result["linear_rank"].clone()}else{Value::Null},"v2_closure_rank":result["v2_closure_rank"],"v2_linear_rank":result["v2_linear_rank"],"system_sha256":case["system_sha256"],"factor_base_manifest_sha256":case["factor_base_manifest_sha256"],"degree_of_regularity":null,"solving_degree":null}));
    }
    save(
        &root.join("summary.json"),
        &json!({"schema":"n83.m3-degree-three-summary/v1","rows":rows,"scope":"Exact LFD or certified bounds on twelve retained m3 binary/Koblitz presentations only. A capped trace certifies only the admitted subspace, never complete V3. Unknown is null; no regularity or elite-base claim."}),
    );
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("summary") => summary(Path::new(&a[2])),
        Some("selfcheck") => println!("{}", controls(Path::new(&a[2]))),
        Some("run") | Some("replay") => {
            let (input, text) = load_case(Path::new(&a[2]), &a[3]);
            let result = if a[1] == "run" {
                run_case(&input, &text, Path::new(&a[4]), Path::new(&a[5]))
            } else {
                replay_case(&input, &text, Path::new(&a[4]), Path::new(&a[5]))
            };
            println!(
                "{}",
                json!({"id":input["id"],"verified":result["verified"],"closure_rank":result["closure_rank"],"linear_rank":result["linear_rank"],"last_fall_degree":result["last_fall_degree"],"last_fall_lower_bound":result["last_fall_lower_bound"],"last_fall_upper_bound":result["last_fall_upper_bound"]})
            );
        }
        _ => panic!("selfcheck FRESH_OUT | run/replay INPUTS ID V2_DIR OUTPUT"),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn columns() {
        let sp = Space::new(164, 3);
        assert_eq!(sp.cols, 735295);
        let mut cols = BTreeSet::new();
        cols.insert(sp.col(&[]));
        for i in 0..164 {
            cols.insert(sp.col(&[i]));
            for j in 0..i {
                cols.insert(sp.col(&[j, i]));
                for k in 0..j {
                    cols.insert(sp.col(&[k, j, i]));
                }
            }
        }
        assert_eq!(cols, (0..sp.cols).collect());
    }
    #[test]
    fn products_cross_word_boundary() {
        let sp = Space::new(164, 3);
        for a in [0, 63, 64, 127, 128, 163] {
            for b in [1, 62, 65, 126, 129, 162] {
                for v in [0, 63, 64, 127, 128, 163] {
                    let base: BTreeSet<_> = [a, b].into();
                    let mut product = base.clone();
                    product.insert(v);
                    let p: Poly = [base.into_iter().collect()].into();
                    let q: Poly = [product.into_iter().collect()].into();
                    assert_eq!(sp.mul(&sp.row(&p), v), sp.row(&q));
                }
            }
        }
    }
    #[test]
    fn capped_trace_and_false_completion_rejected() {
        for (rows_limit, op_limit) in [(1, 100), (100, 1)] {
            let dir = std::env::temp_dir()
                .join(format!("n83-m3-cap-{}-{rows_limit}", std::process::id()));
            fs::create_dir(&dir).unwrap();
            let text = control_text(&["v(1)*v(2)+1", "v(2)*v(3)+1", "v(1)*v(3)"], 3);
            let input = json!({"n_vars":3,"input_equations":6});
            let (v2, _) = produce(&input, &text, &dir.join("v2"), 2);
            let out = dir.join("v3");
            let (rows, steps) = produce_limited(&input, &text, &out, 3, rows_limit, op_limit);
            let state = read(&out.join("closure_state.json"));
            assert_eq!(state["complete"], false);
            let mut result = final_verdict(&input, &text, &rows, &v2, steps, &state);
            result["v2_reference"] = v2_refs(&dir.join("v2"));
            save(&out.join("result.json"), &result);
            replay_case(&input, &text, &dir.join("v2"), &out);
            assert!(result["last_fall_upper_bound"].is_null());
            let mut bad = state.clone();
            bad["complete"] = json!(true);
            save(&out.join("closure_state.original.json"), &state);
            save(&out.join("closure_state.json"), &bad);
            assert!(
                std::panic::catch_unwind(|| replay_case(&input, &text, &dir.join("v2"), &out))
                    .is_err()
            );
        }
    }
    #[test]
    fn product_degree_guard() {
        let sp = Space::new(4, 3);
        let p = [vec![0, 1, 2]].into();
        assert!(std::panic::catch_unwind(|| sp.mul(&sp.row(&p), 0)).is_err());
        let p = [vec![0, 1]].into();
        assert_eq!(sp.mul(&sp.row(&p), 0), sp.row(&p));
        let q = [vec![0, 1, 2]].into();
        assert_eq!(sp.mul(&sp.row(&p), 2), sp.row(&q));
    }
    #[test]
    fn field_equations_required() {
        assert!(std::panic::catch_unwind(|| equations("I[1]=v(1)*v(2);", 2)).is_err());
    }
    #[test]
    fn duplicate_equation_ids_rejected() {
        let text = control_text(&["v(1)*v(2)"], 2).replace("I[2]", "I[1]");
        assert!(std::panic::catch_unwind(|| equations(&text, 2)).is_err());
    }
    #[test]
    fn positive_negative_and_tamper() {
        let dir = std::env::temp_dir().join(format!("n83-degree3-controls-{}", std::process::id()));
        controls(&dir);
        let c = dir.join("strict-three");
        let text = fs::read_to_string(c.join("system.sing")).unwrap();
        let input = read(&c.join("input.json"));
        let ops = c.join("v3/operations.jsonl");
        let original = fs::read_to_string(&ops).unwrap();
        fs::write(c.join("v3/operations.original.jsonl"), &original).unwrap();
        fs::write(
            &ops,
            original.replacen("\"equation\":1", "\"equation\":999", 1),
        )
        .unwrap();
        assert!(std::panic::catch_unwind(|| replay_case(
            &input,
            &text,
            &c.join("v2"),
            &c.join("v3")
        ))
        .is_err());
    }
}

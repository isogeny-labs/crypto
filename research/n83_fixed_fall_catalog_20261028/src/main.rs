//! Exact degree-two Boolean closure, replayable row operations, and input catalog.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
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
    text.lines()
        .filter(|l| l.starts_with("I["))
        .map(|l| {
            let (a, b) = l.split_once('=').unwrap();
            (
                a[2..a.len() - 1].parse().unwrap(),
                parse(b.strip_suffix(';').unwrap(), n),
            )
        })
        .collect()
}
struct Space {
    n: usize,
    cols: usize,
    words: usize,
}
impl Space {
    fn new(n: usize) -> Self {
        let cols = 1 + n + n * (n - 1) / 2;
        Self {
            n,
            cols,
            words: cols.div_ceil(64),
        }
    }
    fn col(&self, m: &[usize]) -> usize {
        match m.len() {
            0 => 0,
            1 => 1 + m[0],
            2 => 1 + self.n + m[1] * (m[1] - 1) / 2 + m[0],
            _ => panic!("degree exceeds two"),
        }
    }
    fn row(&self, p: &Poly) -> Vec<u64> {
        let mut r = vec![0; self.words];
        for m in p {
            let c = self.col(m);
            r[c / 64] ^= 1 << (c % 64)
        }
        r
    }
    fn deg(&self, r: &[u64]) -> usize {
        match pivot(r) {
            None => 0,
            Some(0) => 0,
            Some(c) if c <= self.n => 1,
            _ => 2,
        }
    }
    fn mul(&self, r: &[u64], v: usize) -> Vec<u64> {
        assert!(v < self.n && self.deg(r) < 2);
        let mut out = vec![0; self.words];
        for c in 0..=self.n {
            if bit(r, c) {
                let m = if c == 0 || c == v + 1 {
                    vec![v]
                } else {
                    let mut m = vec![c - 1, v];
                    m.sort_unstable();
                    m
                };
                let at = self.col(&m);
                out[at / 64] ^= 1 << (at % 64)
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
        *x ^= *y
    }
}
fn pivot(r: &[u64]) -> Option<usize> {
    r.iter()
        .enumerate()
        .rev()
        .find(|(_, w)| **w != 0)
        .map(|(i, w)| 64 * i + 63 - w.leading_zeros() as usize)
}
fn sources(polys: &[(usize, Poly)]) -> VecDeque<Value> {
    polys
        .iter()
        .filter(|(_, p)| !p.is_empty() && degree(p) <= 2)
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
    if sp.deg(row) < 2 {
        for v in 0..sp.n {
            q.push_back(json!({"parent":id,"variable":v}));
        }
    }
}
fn evaluate(p: &Poly, values: &[bool]) -> bool {
    p.iter()
        .fold(false, |a, m| a ^ m.iter().all(|v| values[*v]))
}
fn verdict(rows: &[Vec<u64>], sp: &Space, polys: &[(usize, Poly)]) -> Value {
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
fn closure(input: &Value, text: &str, out: &Path) -> Value {
    let n = input["n_vars"].as_u64().unwrap() as usize;
    let sp = Space::new(n);
    let polys = equations(text, n);
    assert_eq!(
        polys.len(),
        input["input_equations"].as_u64().unwrap() as usize
    );
    fs::create_dir(out).unwrap();
    let mut data = BufWriter::new(File::create(out.join("basis.rows.u64le")).unwrap());
    data.write_all(b"N83V2R1\n").unwrap();
    data.write_all(&(n as u32).to_le_bytes()).unwrap();
    data.write_all(&(sp.cols as u64).to_le_bytes()).unwrap();
    data.write_all(&(sp.words as u64).to_le_bytes()).unwrap();
    let mut ops = BufWriter::new(File::create(out.join("operations.jsonl")).unwrap());
    let mut q = sources(&polys);
    let mut rows: Vec<Vec<u64>> = vec![];
    let mut pivots: BTreeMap<usize, usize> = BTreeMap::new();
    let mut steps = 0;
    while let Some(source) = q.pop_front() {
        let mut r = candidate(&source, &rows, &polys, &sp);
        let mut reducers = vec![];
        while let Some(p) = pivot(&r) {
            if let Some(&id) = pivots.get(&p) {
                xor(&mut r, &rows[id]);
                reducers.push(id)
            } else {
                break;
            }
        }
        let id = if let Some(p) = pivot(&r) {
            let id = rows.len();
            pivots.insert(p, id);
            append_multiples(&mut q, &r, id, &sp);
            for w in &r {
                data.write_all(&w.to_le_bytes()).unwrap()
            }
            rows.push(r);
            Some(id)
        } else {
            None
        };
        writeln!(
            ops,
            "{}",
            json!({"step":steps,"source":source,"xor_basis_rows":reducers,"inserted_row":id})
        )
        .unwrap();
        steps += 1;
    }
    data.flush().unwrap();
    ops.flush().unwrap();
    let mut result = verdict(&rows, &sp, &polys);
    result["input"] = input.clone();
    result["schema"] = json!("n83.degree-two-closure/v1");
    result["system_sha256"] = json!(sha(text.as_bytes()));
    result["processed_candidates"] = json!(steps);
    result["closure_complete"] = json!(true);
    save(&out.join("result.json"), &result);
    result
}
fn replay(input: &Value, text: &str, out: &Path) -> Value {
    let n = input["n_vars"].as_u64().unwrap() as usize;
    let sp = Space::new(n);
    let polys = equations(text, n);
    let mut q = sources(&polys);
    let mut rows: Vec<Vec<u64>> = vec![];
    let mut data = BufReader::new(File::open(out.join("basis.rows.u64le")).unwrap());
    let mut header = [0; 28];
    data.read_exact(&mut header).unwrap();
    assert_eq!(&header[..8], b"N83V2R1\n");
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
            xor(&mut row, &rows[id.as_u64().unwrap() as usize]);
        }
        if let Some(id) = op["inserted_row"].as_u64() {
            assert_eq!(id as usize, rows.len());
            let p = pivot(&row).expect("zero inserted");
            assert!(pivots.insert(p));
            let mut saved = vec![0; sp.words];
            for w in &mut saved {
                let mut b = [0; 8];
                data.read_exact(&mut b).unwrap();
                *w = u64::from_le_bytes(b);
            }
            assert_eq!(row, saved);
            append_multiples(&mut q, &row, rows.len(), &sp);
            rows.push(row);
        } else {
            assert!(pivot(&row).is_none())
        }
        count += 1;
    }
    assert!(q.is_empty());
    let mut tail = [0; 1];
    assert_eq!(data.read(&mut tail).unwrap(), 0);
    let expected = read(&out.join("result.json"));
    assert_eq!(expected["system_sha256"], sha(text.as_bytes()));
    assert_eq!(expected["input"], *input);
    assert_eq!(expected["processed_candidates"], count);
    let result = verdict(&rows, &sp, &polys);
    for (k, v) in result.as_object().unwrap() {
        assert_eq!(&expected[k], v, "{k}")
    }
    json!({"input":input["id"],"verified":true,"rows":rows.len(),"operations":count,"last_fall_degree":result["last_fall_degree"],"independent_review":false})
}
fn catalog(root: &Path, out: &Path) {
    fs::create_dir(out).unwrap();
    let inputs = read(&root.join("inputs.json"));
    let mut index = vec![];
    for input in inputs.as_array().unwrap() {
        let id = input["id"].as_str().unwrap();
        let raw = fs::read(root.join(id).join("system.sing")).unwrap();
        assert_eq!(sha(&raw), input["source_sha256"]);
        assert_eq!(input["ell"], 27);
        let text = std::str::from_utf8(&raw).unwrap();
        let n = input["n_vars"].as_u64().unwrap() as usize;
        let mut f =
            BufWriter::new(File::create(out.join(format!("{id}.equations.jsonl"))).unwrap());
        let mut offset = 0;
        let mut equations_count = 0;
        let mut terms = 0;
        let mut hist = BTreeMap::<usize, usize>::new();
        for (line_number, line) in text.split_inclusive('\n').enumerate() {
            let stmt = line.trim_end();
            if stmt.starts_with("I[") {
                let (left, expr) = stmt.split_once('=').unwrap();
                let mut row = json!({"equation_id":left[2..left.len()-1].parse::<usize>().unwrap(),"line_1_based":line_number+1,"byte_offset":offset,"byte_length":line.len(),"field_equation":expr.contains("^2-v("),"expression_encoding":if input["family"]=="prime"{"symbolic_integer_field_expression"}else{"expanded_F2_multilinear_terms_or_field_equation"}});
                if input["family"] != "prime" {
                    let p = parse(expr.strip_suffix(';').unwrap(), n);
                    let d = degree(&p);
                    let support: BTreeSet<_> = p.iter().flatten().copied().collect();
                    let mut dh = BTreeMap::<usize, usize>::new();
                    for mon in &p {
                        *dh.entry(mon.len()).or_default() += 1;
                    }
                    row["boolean_normal_form"] = json!({"degree":if p.is_empty(){None}else{Some(d)},"terms":p.len(),"term_degree_histogram":dh,"support_zero_based":support,"zero_after_field_reduction":p.is_empty()});
                    terms += p.len();
                    if !p.is_empty() {
                        *hist.entry(d).or_default() += 1;
                    }
                }
                writeln!(f, "{row}").unwrap();
                equations_count += 1;
            }
            offset += line.len();
        }
        assert_eq!(
            equations_count,
            input["input_equations"].as_u64().unwrap() as usize
        );
        index.push(json!({"input":input,"file":{"relative_path":format!("{id}/system.sing"),"sha256":sha(&raw),"bytes":raw.len(),"encoding":"UTF-8","format":"Singular source","line_count":text.lines().count()},"equation_index":format!("{id}.equations.jsonl"),"variable_blocks":{"base_selectors":{"blocks":input["m"],"bits_per_block":27,"first_index_zero_based":0},"intermediates":{"count":input["m"].as_u64().unwrap()-2,"coordinates_each":if input["family"]=="prime"{1}else{83},"first_index_zero_based":input["m"].as_u64().unwrap()*27}},"expanded_boolean_terms":if input["family"]=="prime"{None}else{Some(terms)},"nonzero_boolean_degree_histogram":hist}));
    }
    save(
        &out.join("index.json"),
        &json!({"schema":"n83.pdp-structure-catalog/v1","inputs":index,"equation_offsets":"byte offsets into the hash-identified original uncompressed UTF-8 system.sing","prime_expansion":"not performed; symbolic source expressions retained without invented term counts"}),
    );
}
fn load_case(root: &Path, id: &str) -> (Value, String) {
    let input = read(&root.join(id).join("input.json"));
    assert_ne!(input["family"], "prime");
    assert_eq!(input["ell"], 27);
    let raw = fs::read(root.join(id).join("system.sing")).unwrap();
    assert_eq!(sha(&raw), input["source_sha256"]);
    let text = String::from_utf8(raw).unwrap();
    assert_eq!(
        text.lines().next().unwrap(),
        format!("ring r=2,(v(1..{})),dp;", input["n_vars"])
    );
    (input, text)
}
fn summarize(root: &Path, out: &Path) {
    let inputs = read(&root.join("inputs.json"));
    let first = read(&out.join("first-fall/results.json"));
    let mut rows = vec![];
    for i in inputs.as_array().unwrap() {
        let id = i["id"].as_str().unwrap();
        let f = first
            .as_array()
            .unwrap()
            .iter()
            .find(|v| v["input"]["id"] == id)
            .unwrap();
        let cpath = out.join("closure").join(id).join("result.json");
        let c = if cpath.exists() {
            read(&cpath)
        } else {
            Value::Null
        };
        let status_path = out.join("status").join(format!("{id}.json"));
        let status = if status_path.exists() {
            read(&status_path)
        } else {
            json!({"closure":"not_applicable_prime"})
        };
        rows.push(json!({"id":id,"family":i["family"],"m":i["m"],"ell":27,"target_kind":i["target_kind"],"n_vars":i["n_vars"],"first_fall":f,"closure_status":status,"last_fall_degree":c["last_fall_degree"],"last_fall_lower_bound":c["last_fall_lower_bound"],"closure_rank":c["closure_rank"],"linear_rank":c["linear_rank"],"contains_one":c["contains_one"],"exhaustive_affine_completion":c["exhaustive_affine_completion"],"affine_assignments_checked":c["assignments"].as_array().map(Vec::len),"degree_of_regularity":null}));
    }
    save(
        &out.join("summary.json"),
        &json!({"schema":"n83.fixed-fall-summary/v1","rows":rows,"scope":"Exact certificates or bounds for these fixed ell27 input presentations only; no degree-of-regularity or elite-factor-base improvement claim"}),
    );
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("catalog") => catalog(Path::new(&a[2]), Path::new(&a[3])),
        Some("summarize") => summarize(Path::new(&a[2]), Path::new(&a[3])),
        Some("closure") | Some("replay") => {
            let (i, t) = load_case(Path::new(&a[2]), &a[3]);
            let v = if a[1] == "closure" {
                closure(&i, &t, Path::new(&a[4]))
            } else {
                replay(&i, &t, Path::new(&a[4]))
            };
            println!(
                "{}",
                json!({"input":a[3],"last_fall_degree":v["last_fall_degree"],"closure_rank":v["closure_rank"],"verified":v["verified"]})
            );
        }
        _ => panic!("catalog INPUTS NEW_OUTPUT | closure/replay INPUTS ID OUTPUT"),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn case(name: &str, text: &str, n: usize) -> (Value, std::path::PathBuf) {
        let dir = std::env::temp_dir().join(format!("n83-v2-{}-{name}", std::process::id()));
        let input = json!({"id":name,"n_vars":n,"input_equations":text.lines().count()});
        (input, dir)
    }
    #[test]
    fn columns_and_products() {
        let s = Space::new(8);
        let mut seen = BTreeSet::new();
        seen.insert(s.col(&[]));
        for i in 0..8 {
            seen.insert(s.col(&[i]));
            for j in i + 1..8 {
                seen.insert(s.col(&[i, j]));
            }
        }
        assert_eq!(seen.len(), s.cols);
        for i in 0..8 {
            let p = [vec![i]].into();
            assert_eq!(s.mul(&s.row(&p), i), s.row(&p));
        }
    }
    #[test]
    fn exact_lfd_two_and_tamper() {
        let text = "I[1]=v(1)*v(2);\nI[2]=v(1)*v(2)+1;\nI[3]=v(1)^2-v(1);\nI[4]=v(2)^2-v(2);\n";
        let (i, d) = case("unit", text, 2);
        let r = closure(&i, text, &d);
        assert_eq!(r["last_fall_degree"], 2);
        replay(&i, text, &d);
        let p = d.join("operations.jsonl");
        let original = fs::read_to_string(&p).unwrap();
        fs::write(&p, original.replace("\"equation\":1", "\"equation\":999")).unwrap();
        assert!(std::panic::catch_unwind(|| replay(&i, text, &d)).is_err());
        fs::write(&p, original).unwrap();
    }
    #[test]
    fn plateau_is_not_certificate() {
        let text = "I[1]=v(1)*v(2);\nI[2]=v(1)^2-v(1);\nI[3]=v(2)^2-v(2);\n";
        let (i, d) = case("nonlinear", text, 2);
        let r = closure(&i, text, &d);
        assert_eq!(r["last_fall_degree"], Value::Null);
        assert_eq!(r["exhaustive_affine_completion"], false);
        replay(&i, text, &d);
    }
    #[test]
    fn affine_space_certificate() {
        let text =
            "I[1]=v(1)*v(2)+v(1);\nI[2]=v(1)*v(2)+v(2);\nI[3]=v(1)^2-v(1);\nI[4]=v(2)^2-v(2);\n";
        let (i, d) = case("line", text, 2);
        let r = closure(&i, text, &d);
        assert_eq!(r["last_fall_degree"], 2);
        assert_eq!(r["linear_rank"], 1);
        assert_eq!(r["assignments"].as_array().unwrap().len(), 2);
        replay(&i, text, &d);
    }
}

//! Native tools for the E11 window-engine study
//! (`research/fall_degree_bounds_20261008/e11/`). They replace the study's
//! Python helpers subcommand for subcommand, with the same inputs and the same
//! output format (Python `json.dumps(indent=1)` layout and `repr` floats), so
//! the committed outputs can be regenerated and diffed byte for byte.
//!
//! Run from the repository root:
//!
//! ```text
//! fall_degree_e11_tools compare [--json OUT] e11/validate/*.jsonl   -> validation_summary.{txt,json}
//! fall_degree_e11_tools columns m:n:ell [...]                       exact column counts per window policy
//! fall_degree_e11_tools derived e11/validate/*.jsonl                -> derived.txt
//! fall_degree_e11_tools diff-rerun e11/validate e11/validate_final  -> diff_rerun.txt
//! fall_degree_e11_tools project                                     -> projection.json (PROJECTION)
//! fall_degree_e11_tools scaling-table                               -> scaling_table.json
//! fall_degree_e11_tools summ [FILE ...]                             one line per hky_window record
//! fall_degree_e11_tools maxrss RSS_FILE CMD [ARGS ...]              peak RSS (kB) of a child command
//! ```
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use std::path::Path;

const S: &str = "research/fall_degree_bounds_20261008";
const E11: &str = "research/fall_degree_bounds_20261008/e11";
const FS: &str = "research/dreg_fixed_surplus_20260923/runs";
const M4: &str = "research/dreg_m4_chain_20261006/runs";

// ---------------------------------------------------------------- output tree

/// Ordered JSON-like value, printed either as Python `json.dumps(indent=1)`
/// or as Python `repr` (dicts, tuples, `None`).
#[derive(Clone, Debug)]
enum J {
    Null,
    Bool(bool),
    Int(String),
    Float(f64),
    Str(String),
    Arr(Vec<J>),
    Tup(Vec<J>),
    Obj(Vec<(String, J)>),
}

/// Python `repr(float)`: shortest round-trip digits, scientific notation
/// outside `1e-4 <= |x| < 1e16`.
fn py_float(x: f64) -> String {
    if x.is_nan() {
        return "NaN".into();
    }
    if x.is_infinite() {
        return if x > 0.0 {
            "Infinity".into()
        } else {
            "-Infinity".into()
        };
    }
    if x == 0.0 {
        return if x.is_sign_negative() {
            "-0.0".into()
        } else {
            "0.0".into()
        };
    }
    let sci = format!("{:e}", x);
    let (mant, exp) = sci.split_once('e').unwrap();
    let exp: i32 = exp.parse().unwrap();
    if (-4..16).contains(&exp) {
        let s = format!("{}", x);
        if s.contains('.') {
            s
        } else {
            s + ".0"
        }
    } else {
        format!(
            "{}e{}{:02}",
            mant,
            if exp < 0 { '-' } else { '+' },
            exp.abs()
        )
    }
}

/// Python `round(x, nd)` (correctly rounded on the exact binary value).
fn py_round(x: f64, nd: usize) -> f64 {
    format!("{:.*}", nd, x).parse().unwrap()
}

/// A parsed JSON number as Python would hold it: `int` when it has no
/// fraction or exponent, `float` otherwise.
fn num_j(v: &Value) -> J {
    match v {
        Value::Number(n) => {
            let s = n.to_string();
            if s.contains(['.', 'e', 'E']) {
                J::Float(s.parse().unwrap())
            } else {
                J::Int(s)
            }
        }
        other => j(other),
    }
}

fn j(v: &Value) -> J {
    match v {
        Value::Null => J::Null,
        Value::Bool(b) => J::Bool(*b),
        Value::Number(_) => num_j(v),
        Value::String(s) => J::Str(s.clone()),
        Value::Array(a) => J::Arr(a.iter().map(j).collect()),
        Value::Object(o) => J::Obj(o.iter().map(|(k, v)| (k.clone(), j(v))).collect()),
    }
}

fn jint<T: ToString>(x: T) -> J {
    J::Int(x.to_string())
}

fn jstr(s: &str) -> J {
    J::Str(s.to_string())
}

fn json_str(s: &str) -> String {
    serde_json::to_string(s).unwrap()
}

fn dump(v: &J, ind: usize, out: &mut String) {
    let pad = |k: usize| " ".repeat(k);
    match v {
        J::Null => out.push_str("null"),
        J::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        J::Int(s) => out.push_str(s),
        J::Float(x) => out.push_str(&py_float(*x)),
        J::Str(s) => out.push_str(&json_str(s)),
        J::Arr(a) | J::Tup(a) => {
            if a.is_empty() {
                out.push_str("[]");
                return;
            }
            out.push_str("[\n");
            for (i, x) in a.iter().enumerate() {
                out.push_str(&pad(ind + 1));
                dump(x, ind + 1, out);
                out.push_str(if i + 1 < a.len() { ",\n" } else { "\n" });
            }
            out.push_str(&pad(ind));
            out.push(']');
        }
        J::Obj(o) => {
            if o.is_empty() {
                out.push_str("{}");
                return;
            }
            out.push_str("{\n");
            for (i, (k, x)) in o.iter().enumerate() {
                out.push_str(&pad(ind + 1));
                out.push_str(&json_str(k));
                out.push_str(": ");
                dump(x, ind + 1, out);
                out.push_str(if i + 1 < o.len() { ",\n" } else { "\n" });
            }
            out.push_str(&pad(ind));
            out.push('}');
        }
    }
}

/// `json.dumps(v, indent=1)`.
fn dumps(v: &J) -> String {
    let mut s = String::new();
    dump(v, 0, &mut s);
    s
}

/// Python `repr`.
fn repr(v: &J) -> String {
    match v {
        J::Null => "None".into(),
        J::Bool(b) => (if *b { "True" } else { "False" }).into(),
        J::Int(s) => s.clone(),
        J::Float(x) => match py_float(*x).as_str() {
            "NaN" => "nan".into(),
            "Infinity" => "inf".into(),
            "-Infinity" => "-inf".into(),
            s => s.into(),
        },
        J::Str(s) => format!("'{}'", s.replace('\\', "\\\\").replace('\'', "\\'")),
        J::Arr(a) => format!("[{}]", a.iter().map(repr).collect::<Vec<_>>().join(", ")),
        J::Tup(a) if a.len() == 1 => format!("({},)", repr(&a[0])),
        J::Tup(a) => format!("({})", a.iter().map(repr).collect::<Vec<_>>().join(", ")),
        J::Obj(o) => format!(
            "{{{}}}",
            o.iter()
                .map(|(k, x)| format!("'{}': {}", k, repr(x)))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

// ---------------------------------------------------------------- input helpers

/// `sorted(glob.glob(pat))` for patterns with `*` only in the file name.
fn glob(pat: &str) -> Vec<String> {
    if !pat.contains('*') {
        return if Path::new(pat).exists() {
            vec![pat.to_string()]
        } else {
            vec![]
        };
    }
    let (dir, name) = pat.rsplit_once('/').unwrap_or((".", pat));
    let parts: Vec<&str> = name.split('*').collect();
    let mut out = vec![];
    if let Ok(rd) = std::fs::read_dir(dir) {
        for e in rd.flatten() {
            let f = e.file_name().to_string_lossy().to_string();
            if f.starts_with('.') && !name.starts_with('.') {
                continue;
            }
            if wild(&f, &parts) {
                out.push(format!("{}/{}", dir, f));
            }
        }
    }
    out.sort();
    out
}

fn wild(f: &str, parts: &[&str]) -> bool {
    let (first, last) = (parts[0], parts[parts.len() - 1]);
    if !f.starts_with(first) || f.len() < first.len() + last.len() || !f.ends_with(last) {
        return false;
    }
    let mut rest = &f[first.len()..f.len() - last.len()];
    for p in &parts[1..parts.len() - 1] {
        match rest.find(p) {
            Some(i) => rest = &rest[i + p.len()..],
            None => return false,
        }
    }
    true
}

/// JSON records (lines starting with `{`) of every file matching `pat`.
fn lines(pat: &str) -> Vec<Value> {
    let mut v = vec![];
    for p in glob(pat) {
        v.extend(read_records(&p));
    }
    v
}

fn read_records(p: &str) -> Vec<Value> {
    let text = std::fs::read_to_string(p).unwrap_or_else(|e| panic!("{}: {}", p, e));
    text.lines()
        .filter(|l| l.starts_with('{'))
        .map(|l| serde_json::from_str(l).unwrap_or_else(|e| panic!("{}: {}", p, e)))
        .collect()
}

fn f(v: &Value) -> f64 {
    v.as_f64().unwrap_or(f64::NAN)
}

fn i(v: &Value) -> i64 {
    v.as_i64()
        .unwrap_or_else(|| v.as_f64().map(|x| x as i64).unwrap_or(0))
}

/// Python truthiness of a JSON value.
fn truthy(v: Option<&Value>) -> bool {
    match v {
        None | Some(Value::Null) => false,
        Some(Value::Bool(b)) => *b,
        Some(Value::Number(n)) => n.as_f64() != Some(0.0),
        Some(Value::String(s)) => !s.is_empty(),
        Some(Value::Array(a)) => !a.is_empty(),
        Some(Value::Object(o)) => !o.is_empty(),
    }
}

/// Python `==` on JSON values (`1 == 1.0`).
fn veq(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => match (x.as_i64(), y.as_i64()) {
            (Some(p), Some(q)) => p == q,
            _ => x.to_string() == y.to_string() || x.as_f64() == y.as_f64(),
        },
        (Value::Array(x), Value::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(p, q)| veq(p, q))
        }
        (Value::Object(x), Value::Object(y)) => {
            x.len() == y.len() && x.iter().all(|(k, p)| y.get(k).is_some_and(|q| veq(p, q)))
        }
        _ => a == b,
    }
}

/// Canonical text of a value, used as a hash key.
fn key_text(v: &Value) -> String {
    match v {
        Value::Number(n) => n.as_i64().map_or_else(|| n.to_string(), |x| x.to_string()),
        Value::Array(a) => format!("[{}]", a.iter().map(key_text).collect::<Vec<_>>().join(",")),
        other => other.to_string(),
    }
}

/// `(m, tuple(v_basis), x_r)`.
type DrawKey = (String, String, String);

fn draw_key(m: &Value, r: &Value) -> DrawKey {
    (key_text(m), key_text(&r["v_basis"]), key_text(&r["x_r"]))
}

/// `statistics.median` on numbers.
fn median(mut xs: Vec<f64>) -> f64 {
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = xs.len();
    if n % 2 == 1 {
        xs[n / 2]
    } else {
        (xs[n / 2 - 1] + xs[n / 2]) / 2.0
    }
}

/// `statistics.median` keeping Python's int/float distinction of the result.
fn median_j(mut vs: Vec<&Value>) -> J {
    vs.sort_by(|a, b| f(a).partial_cmp(&f(b)).unwrap());
    let n = vs.len();
    if n % 2 == 1 {
        num_j(vs[n / 2])
    } else {
        J::Float((f(vs[n / 2 - 1]) + f(vs[n / 2])) / 2.0)
    }
}

/// Python `max` over values: the first maximal element.
fn max_j(vs: Vec<&Value>) -> J {
    let mut best = vs[0];
    for v in &vs[1..] {
        if f(v) > f(best) {
            best = v;
        }
    }
    num_j(best)
}

fn comb(n: u64, k: u64) -> u128 {
    if k > n {
        return 0;
    }
    let mut c: u128 = 1;
    for t in 0..k {
        c = c * (n - t) as u128 / (t + 1) as u128;
    }
    c
}

// ---------------------------------------------------------------- references

struct Ref {
    d_f: Value,
    dims: BTreeMap<String, Value>,
}

fn dims(rec: &Value) -> BTreeMap<String, Value> {
    if let Some(pd) = rec.get("per_degree").and_then(Value::as_object) {
        return pd
            .iter()
            .map(|(k, v)| (k.clone(), v["dim"].clone()))
            .collect();
    }
    rec.get("closure_dims")
        .and_then(Value::as_object)
        .map(|o| o.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
        .unwrap_or_default()
}

fn self_files() -> Vec<String> {
    let mut v = vec![
        format!("{}/e6/B_3-18-6.jsonl", S),
        format!("{}/e6/B_5-10-2.rerun.jsonl", S),
        format!("{}/e6/B_4-13-4.par.jsonl", S),
    ];
    v.extend(glob(&format!("{}/e7/E7_*.jsonl", S)));
    v.extend(glob(&format!("{}/e8/E8_*.jsonl", S)));
    v
}

/// Dense reference records keyed by `(m, v_basis, x_r)` (compare.py `build_refs`).
fn build_refs() -> HashMap<DrawKey, Ref> {
    let refs = [
        (
            format!("{S}/e4/m3_5-2.jsonl"),
            format!("{FS}/cell-5-2-7.jsonl"),
        ),
        (
            format!("{S}/e4/m3_7-2.jsonl"),
            format!("{FS}/cell-7-2-7.jsonl"),
        ),
        (
            format!("{S}/e4/m3_7-3.jsonl"),
            format!("{FS}/cell-7-3-7.jsonl"),
        ),
        (
            format!("{S}/e4/x_m3_9-3.jsonl"),
            format!("{FS}/cell-9-3-7.jsonl"),
        ),
        (
            format!("{S}/e4/x_m3_11-4.jsonl"),
            format!("{FS}/cell-11-4-6.jsonl"),
        ),
        (
            format!("{S}/e4/x_m3_13-4.jsonl"),
            format!("{FS}/cell-13-4-6.jsonl"),
        ),
        (
            format!("{S}/e6/A_m3_13-5.jsonl"),
            format!("{S}/e6/inputs_13-5.jsonl"),
        ),
        (
            format!("{S}/e6/A_m3_15-5.jsonl"),
            format!("{S}/e6/inputs_15-5.jsonl"),
        ),
        (
            format!("{S}/e4/x_m4_5-2.jsonl"),
            format!("{M4}/cell-5-2-8.jsonl"),
        ),
        (
            format!("{S}/e4/x_m4_7-2.jsonl"),
            format!("{M4}/cell-7-2-7.u*.jsonl"),
        ),
        (
            format!("{S}/e4/x_m4_9-3.jsonl"),
            format!("{M4}/cell-9-3-6.u*.jsonl"),
        ),
    ];
    let mut out = HashMap::new();
    for (rf, inp) in &refs {
        let m = Value::from(if rf.contains("_m4_") { 4 } else { 3 });
        let mut byk: HashMap<(String, String), Value> = HashMap::new();
        for r in lines(inp) {
            if r.get("draw").is_some() && r.get("v_basis").is_some() {
                let k = (key_text(&r["cell"]), key_text(&r["draw"]));
                byk.insert(k, r);
            }
        }
        for r in lines(rf) {
            let Some(src) = byk.get(&(key_text(&r["cell"]), key_text(&r["draw"]))) else {
                continue;
            };
            out.insert(
                draw_key(&m, src),
                Ref {
                    d_f: r["d_f"].clone(),
                    dims: dims(&r),
                },
            );
        }
    }
    for rf in self_files() {
        for r in lines(&rf) {
            let zero = r.get("solutions").is_some_and(|s| veq(s, &Value::from(0)));
            if !zero || r.get("v_basis").is_none() {
                continue;
            }
            out.insert(
                draw_key(&r["m"], &r),
                Ref {
                    d_f: r["d_f"].clone(),
                    dims: dims(&r),
                },
            );
        }
    }
    out
}

fn le(v: &Value, k: f64) -> bool {
    v.as_f64().is_some_and(|x| x <= k)
}

// ---------------------------------------------------------------- compare

#[derive(Default)]
struct Cell {
    n: u64,
    cert: u64,
    cols: Vec<Value>,
    dense: Vec<u128>,
}

fn cmd_compare(mut args: &[String]) {
    let mut out_json = None;
    if args.first().map(String::as_str) == Some("--json") {
        out_json = Some(args[1].clone());
        args = &args[2..];
    }
    let refs = build_refs();
    let (mut rows, mut viol, mut dimbad, mut joined) = (vec![], vec![], vec![], 0u64);
    // insertion-ordered (policy, m, n, ell) -> cell
    let mut per: Vec<((String, Value, Value, Value), Cell)> = vec![];
    for p in args {
        for r in lines(p) {
            let Some(rr) = refs.get(&draw_key(&r["m"], &r)) else {
                continue;
            };
            joined += 1;
            let draw = j(&r["draw"]);
            if let Some(pd) = r["per_degree"].as_object() {
                for (k, v) in pd {
                    let v = &v["dim"];
                    if let Some(rd) = rr.dims.get(k) {
                        if !veq(rd, v) {
                            dimbad.push(J::Tup(vec![jstr(p), draw.clone(), jstr(k), j(v), j(rd)]));
                        }
                    }
                }
            }
            let le3 = &r["d_f_le3"];
            if !le3.is_null() && !veq(le3, &rr.d_f) {
                dimbad.push(J::Tup(vec![
                    jstr(p),
                    draw.clone(),
                    jstr("d_f<=3"),
                    j(le3),
                    j(&rr.d_f),
                ]));
            }
            if le3.is_null() && le(&rr.d_f, 3.0) {
                dimbad.push(J::Tup(vec![
                    jstr(p),
                    draw.clone(),
                    jstr("missed d_f<=3"),
                    J::Null,
                    j(&rr.d_f),
                ]));
            }
            let n = r["n_vars"].as_u64().unwrap();
            let dense: u128 = (0..5).map(|d| comb(n, d)).sum();
            let empty = serde_json::Map::new();
            let window = r["window"].as_object().unwrap_or(&empty);
            let mut wrow = vec![];
            for (pol, w) in window {
                if truthy(w.get("certified")) && !le(&rr.d_f, 4.0) {
                    viol.push(J::Tup(vec![jstr(p), draw.clone(), jstr(pol), j(&rr.d_f)]));
                }
                if veq(&rr.d_f, &Value::from(4)) {
                    let key = (
                        pol.clone(),
                        r["m"].clone(),
                        r["n"].clone(),
                        r["ell"].clone(),
                    );
                    let idx = match per.iter().position(|(k, _)| {
                        k.0 == key.0 && veq(&k.1, &key.1) && veq(&k.2, &key.2) && veq(&k.3, &key.3)
                    }) {
                        Some(ix) => ix,
                        None => {
                            per.push((key, Cell::default()));
                            per.len() - 1
                        }
                    };
                    let s = &mut per[idx].1;
                    s.n += 1;
                    s.cert += truthy(w.get("certified")) as u64;
                    s.cols.push(w["cols"].clone());
                    s.dense.push(dense);
                }
                let fields = ["certified", "cols", "dim", "secs"]
                    .iter()
                    .map(|kk| (kk.to_string(), w.get(*kk).map_or(J::Null, j)))
                    .collect();
                wrow.push((pol.clone(), J::Obj(fields)));
            }
            rows.push(J::Obj(vec![
                ("file".into(), jstr(p)),
                ("m".into(), j(&r["m"])),
                ("n".into(), j(&r["n"])),
                ("ell".into(), j(&r["ell"])),
                ("draw".into(), draw),
                ("n_vars".into(), jint(n)),
                ("dense_d_f".into(), j(&rr.d_f)),
                ("d_f_le3".into(), j(le3)),
                ("window".into(), J::Obj(wrow)),
            ]));
        }
    }
    println!(
        "joined draws: {}; degree-2/3 dim or d_F<=3 mismatches: {}; soundness violations: {}",
        joined,
        dimbad.len(),
        viol.len()
    );
    for d in &dimbad {
        println!("  MISMATCH {}", repr(d));
    }
    for v in &viol {
        println!("  VIOLATION {}", repr(v));
    }
    println!("policy  m:n:ell  N  certified/dense-d_F=4  cols  dense_cols");
    let mut order: Vec<usize> = (0..per.len()).collect();
    order.sort_by(|&a, &b| {
        let (ka, kb) = (&per[a].0, &per[b].0);
        (ka.0.as_str(), i(&ka.1), i(&ka.2), i(&ka.3)).cmp(&(
            kb.0.as_str(),
            i(&kb.1),
            i(&kb.2),
            i(&kb.3),
        ))
    });
    let mut tot: Vec<(String, u64, u64)> = vec![];
    for ix in order {
        let ((pol, m, n, ell), s) = &per[ix];
        println!(
            "{:6} {}:{}:{}  {}/{}  {}  {}",
            pol, m, n, ell, s.cert, s.n, s.cols[0], s.dense[0]
        );
        match tot.iter_mut().find(|t| &t.0 == pol) {
            Some(t) => {
                t.1 += s.cert;
                t.2 += s.n;
            }
            None => tot.push((pol.clone(), s.cert, s.n)),
        }
    }
    for (pol, a, b) in &tot {
        println!("RECALL {}: {}/{} = {:.3}", pol, a, b, *a as f64 / *b as f64);
    }
    if let Some(path) = out_json {
        let recall = tot
            .iter()
            .map(|(p, a, b)| {
                (
                    p.clone(),
                    J::Obj(vec![
                        ("certified".into(), jint(a)),
                        ("dense_df4".into(), jint(b)),
                    ]),
                )
            })
            .collect();
        let cells = per
            .iter()
            .map(|((pol, m, n, ell), s)| {
                J::Obj(vec![
                    ("policy".into(), jstr(pol)),
                    ("m".into(), j(m)),
                    ("n".into(), j(n)),
                    ("ell".into(), j(ell)),
                    ("certified".into(), jint(s.cert)),
                    ("dense_df4".into(), jint(s.n)),
                    ("cols".into(), j(&s.cols[0])),
                    ("dense_cols".into(), jint(s.dense[0])),
                ])
            })
            .collect();
        let doc = J::Obj(vec![
            ("joined".into(), jint(joined)),
            ("dim_mismatches".into(), J::Arr(dimbad)),
            ("violations".into(), J::Arr(viol)),
            ("recall".into(), J::Obj(recall)),
            ("cells".into(), J::Arr(cells)),
            ("rows".into(), J::Arr(rows)),
        ]);
        std::fs::write(&path, dumps(&doc)).unwrap_or_else(|e| panic!("{}: {}", path, e));
    }
}

// ---------------------------------------------------------------- columns

/// Block sizes and link block sets (bit masks) of a chained cell, as
/// `build_decomposition_system` lays them out: `m` summand blocks of `ell`
/// bits, then `m - 2` intermediate blocks of `n` bits.
fn layout(m: i64, n: u64, ell: u64) -> (Vec<u64>, Vec<u64>) {
    let mut sizes = vec![ell; m as usize];
    sizes.extend(std::iter::repeat_n(n, (m - 2).max(0) as usize));
    let x = |q: i64| q;
    let e = |q: i64| m + q;
    let bit = |q: i64| 1u64 << q;
    let mut links = vec![bit(x(0)) | bit(x(1)) | bit(e(0))];
    for q in 1..m - 2 {
        links.push(bit(e(q - 1)) | bit(x(q + 1)) | bit(e(q)));
    }
    links.push(bit(e(m - 3)) | bit(x(m - 1)));
    (sizes, links)
}

fn windows(links: &[u64], k: usize) -> Vec<u64> {
    if k >= links.len() {
        return vec![links.iter().fold(0, |a, b| a | b)];
    }
    (0..=links.len() - k)
        .map(|s| links[s..s + k].iter().fold(0, |a, b| a | b))
        .collect()
}

/// Degree-`d` Boolean monomials whose block support lies inside one window W(k).
fn count_w(m: i64, n: u64, ell: u64, k: usize, d: u64) -> u128 {
    let (sizes, links) = layout(m, n, ell);
    let wins = windows(&links, k);
    fn rec(sizes: &[u64], wins: &[u64], i: usize, left: u64, used: u64, mult: u128) -> u128 {
        if i == sizes.len() {
            return if left == 0 && wins.iter().any(|w| used & !w == 0) {
                mult
            } else {
                0
            };
        }
        let mut tot = 0;
        for c in 0..=left.min(sizes[i]) {
            let u = if c > 0 { used | 1 << i } else { used };
            tot += rec(sizes, wins, i + 1, left - c, u, mult * comb(sizes[i], c));
        }
        tot
    }
    rec(&sizes, &wins, 0, d, 0, 1)
}

fn parse_cell(c: &str) -> (i64, u64, u64) {
    let v: Vec<u64> = c.split(':').map(|x| x.parse().unwrap()).collect();
    (v[0] as i64, v[1], v[2])
}

fn low_cols(nv: u64) -> u128 {
    (0..4).map(|d| comb(nv, d)).sum()
}

fn cmd_columns(args: &[String]) {
    for cell in args {
        let (m, n, ell) = parse_cell(cell);
        let nv = m as u64 * ell + (m as u64 - 2) * n;
        let full4 = comb(nv, 4);
        let low = low_cols(nv);
        let mut row = vec![
            ("cell".to_string(), jstr(cell)),
            ("N".into(), jint(nv)),
            ("low_cols".into(), jint(low)),
            ("deg4_full".into(), jint(full4)),
            ("dense_cols".into(), jint(low + full4)),
        ];
        for k in 1..=2 {
            let w = count_w(m, n, ell, k, 4);
            row.push((format!("W{k}_deg4"), jint(w)));
            row.push((format!("W{k}_cols"), jint(low + w)));
            row.push((
                format!("W{k}_frac"),
                J::Float(py_round(w as f64 / full4 as f64, 3)),
            ));
        }
        println!("{}", repr(&J::Obj(row)));
    }
}

// ---------------------------------------------------------------- derived

fn cmd_derived(args: &[String]) {
    let refs = build_refs();
    type CellKey = (i64, i64, i64, i64);
    let cell_of = |r: &Value| (i(&r["m"]), i(&r["n"]), i(&r["ell"]), i(&r["n_vars"]));
    let mut dense_secs: HashMap<CellKey, Vec<f64>> = HashMap::new();
    for rf in self_files() {
        for r in lines(&rf) {
            let s4 = r.get("per_degree").and_then(|p| p.get("4"));
            if veq(&r["d_f"], &Value::from(4)) {
                if let Some(secs) = s4.and_then(|s| s.get("secs")) {
                    dense_secs.entry(cell_of(&r)).or_default().push(f(secs));
                }
            }
        }
    }
    #[derive(Default)]
    struct Pol {
        perrow: Vec<bool>,
        secs: Vec<f64>,
        bytes: Vec<f64>,
        ones: Vec<f64>,
    }
    let mut per: BTreeMap<CellKey, BTreeMap<&str, Pol>> = BTreeMap::new();
    for p in args {
        for r in lines(p) {
            match refs.get(&draw_key(&r["m"], &r)) {
                Some(rr) if veq(&rr.d_f, &Value::from(4)) => {}
                _ => continue,
            }
            let cell = cell_of(&r);
            for pol in ["w1x", "w2x"] {
                let Some(w) = r["window"].get(pol) else {
                    continue;
                };
                if !truthy(Some(w)) {
                    continue;
                }
                let d = per.entry(cell).or_default().entry(pol).or_default();
                d.perrow
                    .push(truthy(w.get("certified")) && veq(&w["exact_passes"], &Value::from(0)));
                d.secs.push(f(&w["secs"]));
                d.bytes.push(f(&w["basis_bytes"]));
                d.ones.push(f(&w["basis_ones"]));
            }
        }
    }
    let mut tot = [(0usize, 0usize); 2];
    println!(
        "m:n:ell N | w1 per-row | w1x secs median | dense deg-4 secs median | w1x basis MB median | density | w2 per-row"
    );
    let empty = Pol::default();
    for (cell, d) in &per {
        let w1 = d.get("w1x").unwrap_or(&empty);
        let w2 = d.get("w2x").unwrap_or(&empty);
        let a = w1.perrow.iter().filter(|x| **x).count();
        let b = w2.perrow.iter().filter(|x| **x).count();
        tot[0].0 += a;
        tot[0].1 += w1.perrow.len();
        tot[1].0 += b;
        tot[1].1 += w2.perrow.len();
        let dens = median(
            w1.ones
                .iter()
                .zip(&w1.bytes)
                .map(|(o, by)| o / (8.0 * by))
                .collect(),
        );
        let ds = match dense_secs.get(cell) {
            Some(v) if !v.is_empty() => format!("{:.1}", median(v.clone())),
            _ => "nan".into(),
        };
        println!(
            "{}:{}:{} {} | {}/{} | {:.1} | {} | {:.1} | {:.3} | {}/{}",
            cell.0,
            cell.1,
            cell.2,
            cell.3,
            a,
            w1.perrow.len(),
            median(w1.secs.clone()),
            ds,
            median(w1.bytes.clone()) / 1e6,
            dens,
            b,
            w2.perrow.len()
        );
    }
    for (k, (x, y)) in ["w1", "w2"].iter().zip(tot) {
        println!(
            "PER-ROW RECALL {}: {}/{} = {:.3}",
            k,
            x,
            y,
            x as f64 / y as f64
        );
    }
}

// ---------------------------------------------------------------- diff-rerun

fn rerun_key(r: &Value) -> Value {
    let dims: serde_json::Map<String, Value> = r["per_degree"]
        .as_object()
        .map(|o| {
            o.iter()
                .map(|(k, v)| (k.clone(), v["dim"].clone()))
                .collect()
        })
        .unwrap_or_default();
    let w = r["window"].get("w1x");
    let fields: Vec<Value> = [
        "certified",
        "dim",
        "rounds",
        "exact_passes",
        "exact_products",
        "gens_admitted",
        "muls_tried",
    ]
    .iter()
    .map(|k| w.and_then(|w| w.get(*k)).cloned().unwrap_or(Value::Null))
    .collect();
    Value::Array(vec![
        Value::Object(dims),
        r["d_f_le3"].clone(),
        Value::Array(fields),
    ])
}

fn cmd_diff_rerun(args: &[String]) {
    let (a_dir, b_dir) = (&args[0], &args[1]);
    let (mut same, mut diff) = (0u64, 0u64);
    for bp in glob(&format!("{}/*.jsonl", b_dir.trim_end_matches('/'))) {
        let base = Path::new(&bp)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let ap = Path::new(a_dir).join(&base);
        let amap: HashMap<String, Value> = read_records(ap.to_str().unwrap())
            .into_iter()
            .map(|r| (key_text(&r["draw"]), r))
            .collect();
        for b in read_records(&bp) {
            let a = &amap[&key_text(&b["draw"])];
            let (ka, kb) = (rerun_key(a), rerun_key(&b));
            if veq(&ka, &kb) {
                same += 1;
            } else {
                diff += 1;
                let t = |k: &Value| {
                    let a = k.as_array().unwrap();
                    repr(&J::Tup(a.iter().map(j).collect()))
                };
                println!("DIFF {} {} {} {}", bp, b["draw"], t(&ka), t(&kb));
            }
        }
    }
    println!("SAME {} DIFF {}", same, diff);
}

// ---------------------------------------------------------------- project

fn cmd_project() {
    let mut pts: BTreeMap<i64, BTreeMap<i64, Vec<f64>>> = BTreeMap::new();
    let mut fmt = vec![];
    let mut files = glob(&format!("{E11}/validate/*.jsonl"));
    files.extend(glob(&format!("{E11}/scaling/window_*.jsonl")));
    for p in &files {
        for r in read_records(p) {
            let Some(w) = r["window"].get("w1x") else {
                continue;
            };
            if !truthy(Some(w)) || !truthy(w.get("certified")) {
                continue;
            }
            let cols = f(&w["cols"]);
            pts.entry(i(&r["m"]))
                .or_default()
                .entry(i(&w["cols"]))
                .or_default()
                .push(f(&w["secs"]));
            fmt.push(f(&w["basis_bytes"]) / (cols * cols / 16.0));
        }
    }
    let fit = |m: i64| -> Option<(f64, f64, usize, f64)> {
        let (mut xs, mut ys) = (vec![], vec![]);
        for (c, s) in pts.get(&m)? {
            if *c >= 10000 {
                xs.push((*c as f64).ln());
                ys.push(median(s.clone()).max(0.05).ln());
            }
        }
        if xs.len() < 2 {
            return None;
        }
        let mean = |v: &[f64]| v.iter().sum::<f64>() / v.len() as f64;
        let (mx, my) = (mean(&xs), mean(&ys));
        let num: f64 = xs.iter().zip(&ys).map(|(x, y)| (x - mx) * (y - my)).sum();
        let den: f64 = xs.iter().map(|x| (x - mx) * (x - mx)).sum();
        let b = num / den;
        let cmax = xs.iter().map(|x| x.exp()).fold(f64::MIN, f64::max);
        Some((my - b * mx, b, xs.len(), cmax))
    };
    let fmin = fmt.iter().cloned().fold(f64::INFINITY, f64::min);
    let fmax = fmt.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    eprintln!(
        "basis_bytes / (cols^2/16): min {:.3} max {:.3} over {} certified runs",
        fmin,
        fmax,
        fmt.len()
    );
    let gb = |c: u128, nd: usize| J::Float(py_round(c as f64 * c as f64 / 16.0 / 1e9, nd));
    let mut out = vec![];
    for (m, n, ell) in [(5i64, 25u64, 5u64), (4, 32, 8), (3, 48, 16)] {
        let nv = m as u64 * ell + (m as u64 - 2) * n;
        let low = low_cols(nv);
        let dense = low + comb(nv, 4);
        let w1 = low + count_w(m, n, ell, 1, 4);
        let mut row = vec![
            ("cell".to_string(), jstr(&format!("{m}:{n}:{ell}"))),
            ("N".into(), jint(nv)),
            ("dense_cols".into(), jint(dense)),
            ("w1_cols".into(), jint(w1)),
            ("dense_basis_GB".into(), gb(dense, 1)),
            ("w1_basis_GB".into(), gb(w1, 1)),
            ("deg3_cols".into(), jint(low)),
            ("deg3_dense_GB".into(), gb(low, 2)),
        ];
        if let Some((a, b, k, cmax)) = fit(m) {
            let hours = (a + b * (w1 as f64).ln()).exp() / 3600.0;
            row.push((
                "time_fit".into(),
                J::Obj(vec![
                    ("exponent".into(), J::Float(py_round(b, 2))),
                    ("cells".into(), jint(k)),
                    (
                        "largest_measured_cols".into(),
                        jint(cmax.round_ties_even() as i64),
                    ),
                    (
                        "projected_hours_3_threads".into(),
                        J::Float(py_round(hours, 1)),
                    ),
                ]),
            ));
        }
        out.push(J::Obj(row));
    }
    println!("{}", dumps(&J::Arr(out)));
}

// ---------------------------------------------------------------- scaling-table

fn cmd_scaling_table() {
    let sdir = format!("{E11}/scaling");
    let mut out = vec![];
    let mut bad = 0u64;
    for wf in glob(&format!("{sdir}/window_*.jsonl")) {
        let tag = wf
            .split("window_")
            .nth(1)
            .unwrap()
            .trim_end_matches(".jsonl")
            .to_string();
        let ws_all = read_records(&wf);
        let df = format!("{sdir}/dense_{tag}.jsonl");
        let mut dmap: HashMap<String, Value> = HashMap::new();
        if Path::new(&df).exists() {
            for r in read_records(&df) {
                dmap.insert(key_text(&r["draw"]), r);
            }
        }
        let empty = Value::Object(Default::default());
        let mut rows: Vec<(&Value, &Value, Option<&Value>)> = vec![];
        for r in &ws_all {
            let w = r["window"].get("w1x").unwrap_or(&empty);
            let d = dmap.get(&key_text(&r["draw"]));
            if let Some(d) = d {
                if let Some(pd) = r["per_degree"].as_object() {
                    for (k, v) in pd {
                        if !veq(&d["per_degree"][k]["dim"], &v["dim"]) {
                            bad += 1;
                            eprintln!("MISMATCH {} {} {}", tag, r["draw"], k);
                        }
                    }
                }
                if truthy(w.get("certified")) && !le(&d["d_f"], 4.0) {
                    bad += 1;
                    eprintln!("VIOLATION {} {}", tag, r["draw"]);
                }
            }
            rows.push((r, w, d));
        }
        let r0 = &ws_all[0];
        let ws: Vec<&Value> = rows
            .iter()
            .map(|x| x.1)
            .filter(|w| truthy(Some(w)))
            .collect();
        let ds: Vec<&Value> = rows.iter().filter_map(|x| x.2).collect();
        let opt = |c: bool, v: J| if c { v } else { J::Null };
        let ds4: Vec<&Value> = ds
            .iter()
            .filter_map(|d| d["per_degree"].get("4"))
            .map(|p| &p["secs"])
            .collect();
        let dense_cols = match ds.first().and_then(|d| d["per_degree"].get("4")) {
            Some(p) => j(&p["of"]),
            None => J::Null,
        };
        let rss = |vs: Vec<f64>| J::Float(vs.into_iter().fold(f64::MIN, f64::max) / 1024.0);
        out.push(J::Obj(vec![
            (
                "cell".into(),
                jstr(&format!("{}:{}:{}", r0["m"], r0["n"], r0["ell"])),
            ),
            ("N".into(), j(&r0["n_vars"])),
            ("draws".into(), jint(ws_all.len())),
            (
                "d_f_le3".into(),
                jint(rows.iter().filter(|x| truthy(x.0.get("d_f_le3"))).count()),
            ),
            (
                "window_certified".into(),
                jint(ws.iter().filter(|w| truthy(w.get("certified"))).count()),
            ),
            (
                "dense_d_f".into(),
                J::Arr(ds.iter().map(|d| j(&d["d_f"])).collect()),
            ),
            (
                "window_cols".into(),
                opt(
                    !ws.is_empty(),
                    ws.first().map_or(J::Null, |w| j(&w["cols"])),
                ),
            ),
            ("dense_cols".into(), dense_cols),
            (
                "window_secs_median".into(),
                opt(
                    !ws.is_empty(),
                    if ws.is_empty() {
                        J::Null
                    } else {
                        median_j(ws.iter().map(|w| &w["secs"]).collect())
                    },
                ),
            ),
            (
                "window_secs_max".into(),
                if ws.is_empty() {
                    J::Null
                } else {
                    max_j(ws.iter().map(|w| &w["secs"]).collect())
                },
            ),
            (
                "dense_deg4_secs_median".into(),
                if ds.is_empty() {
                    J::Null
                } else {
                    median_j(ds4)
                },
            ),
            (
                "window_peak_rss_mb".into(),
                rss(rows.iter().map(|x| f(&x.0["vmhwm_kb"])).collect()),
            ),
            (
                "dense_peak_rss_mb".into(),
                if ds.is_empty() {
                    J::Null
                } else {
                    rss(ds.iter().map(|d| f(&d["maxrss_kb"])).collect())
                },
            ),
            (
                "window_basis_mb_median".into(),
                if ws.is_empty() {
                    J::Null
                } else {
                    let m = median(ws.iter().map(|w| f(&w["basis_bytes"])).collect());
                    J::Float(m / 1e6)
                },
            ),
            (
                "count_secs_median".into(),
                median_j(rows.iter().map(|x| &x.0["count_secs"]).collect()),
            ),
            ("counter".into(), j(&r0["counter"])),
        ]));
    }
    println!("{}", dumps(&J::Arr(out)));
    eprintln!("agreement problems: {}", bad);
}

// ---------------------------------------------------------------- summ

fn cmd_summ(args: &[String]) {
    let stdin = ["/dev/stdin".to_string()];
    let paths = if args.is_empty() { &stdin[..] } else { args };
    for p in paths {
        for r in read_records(p) {
            let dims = J::Obj(
                r["per_degree"]
                    .as_object()
                    .map(|o| o.iter().map(|(k, v)| (k.clone(), j(&v["dim"]))).collect())
                    .unwrap_or_default(),
            );
            let get = |w: &Value, k: &str| w.get(k).map_or(J::Null, j);
            let win = J::Obj(
                r["window"]
                    .as_object()
                    .map(|o| {
                        o.iter()
                            .map(|(k, w)| {
                                let t = ["certified", "cols", "dim", "secs", "vmhwm_kb"]
                                    .iter()
                                    .map(|f| {
                                        if *f == "cols" {
                                            j(&w["cols"])
                                        } else {
                                            get(w, f)
                                        }
                                    })
                                    .collect();
                                (k.clone(), J::Tup(t))
                            })
                            .collect()
                    })
                    .unwrap_or_default(),
            );
            println!(
                "{}:{}:{} draw={} N={} {} d<=3: {} {}",
                r["m"],
                r["n"],
                r["ell"],
                r["draw"],
                r["n_vars"],
                repr(&dims),
                repr(&j(&r["d_f_le3"])),
                repr(&win)
            );
        }
    }
}

// ---------------------------------------------------------------- maxrss

/// Run a command and write its peak RSS (kB, `ru_maxrss` of the reaped
/// children) to a file; stdout/stderr pass through, the exit code is kept.
fn cmd_maxrss(args: &[String]) -> i32 {
    let status = std::process::Command::new(&args[1])
        .args(&args[2..])
        .status()
        .unwrap_or_else(|e| panic!("{}: {}", args[1], e));
    let mut ru: libc::rusage = unsafe { std::mem::zeroed() };
    // SAFETY: getrusage writes into the zeroed struct we own.
    let rc = unsafe { libc::getrusage(libc::RUSAGE_CHILDREN, &mut ru) };
    assert_eq!(rc, 0, "getrusage failed");
    std::fs::write(&args[0], ru.ru_maxrss.to_string())
        .unwrap_or_else(|e| panic!("{}: {}", args[0], e));
    use std::os::unix::process::ExitStatusExt;
    match (status.code(), status.signal()) {
        (Some(c), _) => c,
        (None, Some(sig)) => 256 - sig,
        _ => 1,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let usage = "usage: fall_degree_e11_tools \
                 compare|columns|derived|diff-rerun|project|scaling-table|summ|maxrss ...";
    let Some(cmd) = args.get(1) else {
        eprintln!("{}", usage);
        std::process::exit(2);
    };
    let rest = &args[2..];
    match cmd.as_str() {
        "compare" => cmd_compare(rest),
        "columns" => cmd_columns(rest),
        "derived" => cmd_derived(rest),
        "diff-rerun" => cmd_diff_rerun(rest),
        "project" => cmd_project(),
        "scaling-table" => cmd_scaling_table(),
        "summ" => cmd_summ(rest),
        "maxrss" => std::process::exit(cmd_maxrss(rest)),
        _ => {
            eprintln!("{}", usage);
            std::process::exit(2);
        }
    }
}

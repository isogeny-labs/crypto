//! Run the Boolean-ring F4 (`pq_f4_f2::groebner_basis_f4`) on a system read from JSON
//! and print its statistics as one JSON line.
//!
//! Input file:
//!
//!     { "n_vars": 3, "polys": [ [[0,1],[0],[1]], [[2],[]] ] }
//!
//! `polys` is a list of polynomials; a polynomial is a list of monomials; a monomial is a
//! sorted list of distinct variable indices (`[]` is the constant 1).  Terms are XORed
//! (duplicates cancel).  Variables are `v_0 .. v_{n_vars-1}`, `n_vars <= MASK_BITS`.
//!
//!     bool_f4_file system.json [budget_seconds] [--solutions]
//!     bool_f4_file --self-test
//!
//! Output (one line): `n_vars`, `n_polys`, `input_max_degree`, every scalar `F4Stats` field,
//! `basis_is_one` (the system is inconsistent), `basis_degrees` (at most 200) and, with
//! `--solutions`, `n_solutions` (null above 2^20) and `solutions` (set-variable indices,
//! listed when at most 64).  `--self-test` solves {v0*v1 + v0 + v1, v2 + 1}: one solution [0,1,2].
use crypto_lib::cryptanalysis::pq_f4_f2::{groebner_basis_f4, solutions_from_reduced_basis};
use crypto_lib::cryptanalysis::pq_groebner_f2::{bit, F2BoolMono, F2BoolPoly, MASK_BITS};
use serde_json::{json, Value};
use std::time::Duration;

const MAX_FREE: usize = 20;

fn parse_system(v: &Value) -> Result<(usize, Vec<F2BoolPoly>), String> {
    let n_vars = v["n_vars"].as_u64().ok_or("missing n_vars")? as usize;
    if n_vars > MASK_BITS {
        return Err(format!("n_vars {n_vars} exceeds the mask width {MASK_BITS}"));
    }
    let mut polys = Vec::new();
    for p in v["polys"].as_array().ok_or("missing polys")? {
        let mut monos = Vec::new();
        for m in p.as_array().ok_or("poly is not a list")? {
            let mut mask = 0;
            for k in m.as_array().ok_or("monomial is not a list")? {
                let k = k.as_u64().ok_or("variable index is not an integer")? as usize;
                if k >= n_vars {
                    return Err(format!("variable index {k} >= n_vars {n_vars}"));
                }
                mask |= bit(k as u32);
            }
            monos.push(F2BoolMono::from_mask(mask));
        }
        // from_monos sorts and cancels duplicate pairs
        polys.push(F2BoolPoly::from_monos(monos, n_vars));
    }
    Ok((n_vars, polys))
}

fn run(n_vars: usize, polys: Vec<F2BoolPoly>, budget: Option<Duration>, want_sols: bool) -> Value {
    let n_polys = polys.len();
    let input_max_degree = polys
        .iter()
        .flat_map(|p| p.terms.iter().map(|t| t.degree()))
        .max()
        .unwrap_or(0);
    let (basis, stats) = groebner_basis_f4(polys, n_vars, budget);
    let mut out = serde_json::Map::new();
    out.insert("n_vars".into(), json!(n_vars));
    out.insert("n_polys".into(), json!(n_polys));
    out.insert("input_max_degree".into(), json!(input_max_degree));
    // F4Stats derives Serialize: keep the scalar fields, drop the array fields.
    if let Value::Object(m) = serde_json::to_value(stats).expect("serialize F4Stats") {
        for (k, v) in m {
            if !v.is_array() {
                out.insert(k, v);
            }
        }
    }
    let one = basis.len() == 1 && basis[0].terms.len() == 1 && basis[0].terms[0].mask == 0;
    out.insert("basis_is_one".into(), json!(one));
    let degs: Vec<u32> = basis
        .iter()
        .take(200)
        .map(|p| p.terms.iter().map(|t| t.degree()).max().unwrap_or(0))
        .collect();
    out.insert("basis_degrees".into(), json!(degs));
    if want_sols && !stats.timed_out && !stats.oversize {
        let mut tests = 0u64;
        match solutions_from_reduced_basis(&basis, n_vars, MAX_FREE, &mut tests) {
            Some(sols) => {
                out.insert("n_solutions".into(), json!(sols.len()));
                if sols.len() <= 64 {
                    let lists: Vec<Vec<usize>> = sols
                        .iter()
                        .map(|&s| (0..n_vars).filter(|&k| s & bit(k as u32) != 0).collect())
                        .collect();
                    out.insert("solutions".into(), json!(lists));
                }
            }
            None => {
                out.insert("n_solutions".into(), Value::Null);
            }
        }
    }
    Value::Object(out)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let want_sols = args.iter().any(|a| a == "--solutions");
    let self_test = args.iter().any(|a| a == "--self-test");
    let pos: Vec<&String> = args.iter().filter(|a| !a.starts_with("--")).collect();
    let budget = pos.get(1).map(|s| {
        Duration::from_secs_f64(s.parse::<f64>().expect("budget must be seconds (f64)"))
    });
    let (n_vars, polys, want_sols) = if self_test {
        let m = |ks: &[u32]| F2BoolMono::from_mask(ks.iter().fold(0, |a, &k| a | bit(k)));
        let p0 = F2BoolPoly::from_monos(vec![m(&[0, 1]), m(&[0]), m(&[1])], 3);
        let p1 = F2BoolPoly::from_monos(vec![m(&[2]), m(&[])], 3);
        (3, vec![p0, p1], true)
    } else {
        let path = pos.first().expect("usage: bool_f4_file FILE [budget_s] [--solutions]");
        let text = std::fs::read_to_string(path).expect("read input file");
        let v: Value = serde_json::from_str(&text).expect("parse input JSON");
        let (n, p) = parse_system(&v).unwrap_or_else(|e| panic!("bad input: {e}"));
        (n, p, want_sols)
    };
    println!("{}", run(n_vars, polys, budget, want_sols));
}

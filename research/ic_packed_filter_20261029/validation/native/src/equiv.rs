//! The acceptance test: each native output against the committed Python
//! output, byte for byte, else structurally with the largest relative
//! difference over numbers (a pass needs no structural difference and a
//! maximum relative difference <= 1e-12).

use crate::py::{self, Num};
use crate::v;
use serde_json::Value;
use std::io::Write;
use std::path::Path;

#[derive(Default)]
struct Cmp {
    max_rel: f64,
    structural: Vec<String>,
}

fn walk(a: &Value, b: &Value, path: &str, c: &mut Cmp) {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => {
            if x.is_f64() != y.is_f64() {
                c.structural.push(format!("{path}: int/float type differs"));
                return;
            }
            let (p, q) = (Num::of(a).f(), Num::of(b).f());
            if p != q {
                let rel = (p - q).abs() / p.abs().max(q.abs());
                if rel > c.max_rel {
                    c.max_rel = rel;
                }
            }
        }
        (Value::Object(x), Value::Object(y)) => {
            let kx: std::collections::BTreeSet<_> = x.keys().collect();
            let ky: std::collections::BTreeSet<_> = y.keys().collect();
            if kx != ky {
                c.structural.push(format!("{path}: key sets differ"));
            }
            for k in kx.intersection(&ky) {
                walk(&x[*k], &y[*k], &format!("{path}/{k}"), c);
            }
        }
        (Value::Array(x), Value::Array(y)) => {
            if x.len() != y.len() {
                c.structural.push(format!("{path}: lengths differ"));
            }
            for (i, (p, q)) in x.iter().zip(y).enumerate() {
                walk(p, q, &format!("{path}/{i}"), c);
            }
        }
        _ => {
            if a != b {
                c.structural.push(format!("{path}: {} != {}", py::repr(a), py::repr(b)));
            }
        }
    }
}

/// (label, committed Python output relative to validation/, native file name)
pub fn outputs() -> Vec<(String, String, String)> {
    let mut v = Vec::new();
    for n in ["c2_identity", "j1-P24", "j1-P32", "j1-inpod-crosscheck", "j2", "j3", "j4", "s3check", "s3requests", "scalars", "selftest"] {
        v.push((format!("code/out/{n}.json"), format!("code/out/{n}.json"), format!("{n}.json")));
    }
    for n in ["validator-report", "replay-certificate"] {
        v.push((format!("{n}.json"), format!("{n}.json"), format!("{n}.json")));
    }
    v
}

pub fn run(native_out: &Path, out: &Path) -> bool {
    let vdir = v::root().join("validation");
    let mut f = std::fs::File::create(out).unwrap();
    let mut all = true;
    for (label, py_rel, nat) in outputs() {
        let pp = vdir.join(&py_rel);
        let np = native_out.join(&nat);
        let pb = std::fs::read(&pp).unwrap_or_else(|e| v::stop(format!("{}: {e}", pp.display())));
        let nb = std::fs::read(&np).unwrap_or_else(|e| v::stop(format!("{}: {e}", np.display())));
        let byte_equal = pb == nb;
        let mut c = Cmp::default();
        if !byte_equal {
            let pv: Value = serde_json::from_slice(&pb).unwrap();
            let nv: Value = serde_json::from_slice(&nb).unwrap();
            walk(&pv, &nv, "", &mut c);
        }
        let ok = byte_equal || (c.structural.is_empty() && c.max_rel <= 1e-12);
        all &= ok;
        let line = serde_json::json!({
            "output": label,
            "python": format!("validation/{py_rel}"),
            "native": format!("validation/native/out/{nat}"),
            "python_sha256": v::sha256_file(&pp),
            "native_sha256": v::sha256_file(&np),
            "byte_equal": byte_equal,
            "max_rel_diff": if byte_equal { Value::Null } else { py::fval(c.max_rel) },
            "structural_differences": c.structural.len(),
            "first_structural_differences": c.structural.iter().take(5).collect::<Vec<_>>(),
            "pass": ok,
        });
        writeln!(f, "{}", serde_json::to_string(&line).unwrap()).unwrap();
        println!("{label:40} {}", if byte_equal { "byte-equal".to_string() } else { format!("max rel {:.3e}, structural {}", c.max_rel, c.structural.len()) });
    }
    all
}

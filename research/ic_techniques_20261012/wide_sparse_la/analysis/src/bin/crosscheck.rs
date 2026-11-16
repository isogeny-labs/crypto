//! Field-by-field comparison of two results.json files (e.g. the earlier Python
//! analysis against this Rust port).
//!
//!     wide-sla-crosscheck A.json B.json
//!
//! Every numeric leaf is compared by relative difference |a-b| / max(|a|,|b|).
//! Leaves under a bootstrap-dependent key (`*ci95` except the t-based
//! prediction interval `pi95`) and the curvature status/ci are reported
//! separately from the point estimates.  Strings, booleans, nulls, key sets
//! and array lengths must match exactly and are listed when they do not.

use serde_json::Value;

struct Stats {
    n_num: usize,
    n_exact: usize,
    structural: Vec<String>,
    point: Vec<(String, f64, f64, f64)>,
    boot: Vec<(String, f64, f64, f64)>,
}

fn is_boot(path: &str) -> bool {
    path.contains("ci95") && !path.contains("pi95")
}

fn walk(path: &str, a: &Value, b: &Value, st: &mut Stats) {
    match (a, b) {
        (Value::Object(x), Value::Object(y)) => {
            let kx: Vec<&String> = x.keys().collect();
            let ky: Vec<&String> = y.keys().collect();
            if kx != ky {
                st.structural.push(format!(
                    "{path}: key lists differ ({} vs {})",
                    kx.len(),
                    ky.len()
                ));
            }
            for (k, v) in x {
                match y.get(k) {
                    Some(w) => walk(&format!("{path}/{k}"), v, w, st),
                    None => st.structural.push(format!("{path}/{k}: missing in B")),
                }
            }
        }
        (Value::Array(x), Value::Array(y)) => {
            if x.len() != y.len() {
                st.structural
                    .push(format!("{path}: array length {} vs {}", x.len(), y.len()));
            }
            for (i, (v, w)) in x.iter().zip(y).enumerate() {
                walk(&format!("{path}[{i}]"), v, w, st);
            }
        }
        (Value::Number(x), Value::Number(y)) => {
            let (p, q) = (x.as_f64().unwrap(), y.as_f64().unwrap());
            st.n_num += 1;
            let rel = if p == q {
                0.0
            } else {
                (p - q).abs() / p.abs().max(q.abs())
            };
            if rel == 0.0 {
                st.n_exact += 1;
            }
            let rec = (path.to_string(), p, q, rel);
            if is_boot(path) {
                st.boot.push(rec);
            } else {
                st.point.push(rec);
            }
        }
        _ => {
            if a != b {
                st.structural
                    .push(format!("{path}: {} vs {}", short(a), short(b)));
            }
        }
    }
}

fn short(v: &Value) -> String {
    let s = v.to_string();
    if s.len() > 90 {
        format!("{}...", &s[..90])
    } else {
        s
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: wide-sla-crosscheck A.json B.json");
        std::process::exit(2);
    }
    let rd =
        |p: &str| -> Value { serde_json::from_str(&std::fs::read_to_string(p).unwrap()).unwrap() };
    let (a, b) = (rd(&args[1]), rd(&args[2]));
    let mut st = Stats {
        n_num: 0,
        n_exact: 0,
        structural: vec![],
        point: vec![],
        boot: vec![],
    };
    walk("", &a, &b, &mut st);
    let maxof = |v: &Vec<(String, f64, f64, f64)>| {
        v.iter()
            .cloned()
            .fold(None::<(String, f64, f64, f64)>, |m, x| match m {
                Some(y) if y.3 >= x.3 => Some(y),
                _ => Some(x),
            })
    };
    println!(
        "numeric leaves compared: {} (exactly equal: {})",
        st.n_num, st.n_exact
    );
    println!(
        "  non-bootstrap: {}   bootstrap-dependent (*ci95): {}",
        st.point.len(),
        st.boot.len()
    );
    println!(
        "non-bootstrap leaves with rel diff > 1e-9: {}",
        st.point.iter().filter(|x| x.3 > 1e-9).count()
    );
    if let Some(m) = maxof(&st.point) {
        println!(
            "max rel diff, non-bootstrap: {:.3e} at {} ({} vs {})",
            m.3, m.0, m.1, m.2
        );
    }
    let mut over: Vec<&(String, f64, f64, f64)> = st.point.iter().filter(|x| x.3 > 1e-9).collect();
    over.sort_by(|x, y| y.3.partial_cmp(&x.3).unwrap());
    for x in over.iter().take(40) {
        println!("  DIFF {} : {} vs {} (rel {:.3e})", x.0, x.1, x.2, x.3);
    }
    if !st.boot.is_empty() {
        let mut r: Vec<f64> = st.boot.iter().map(|x| x.3).collect();
        r.sort_by(|x, y| x.partial_cmp(y).unwrap());
        let med = r[r.len() / 2];
        println!("bootstrap-dependent leaves: max rel diff {:.3e}, median {:.3e}, #>1e-9: {}, #>1e-3: {}, #>1e-2: {}",
            r[r.len() - 1], med, r.iter().filter(|&&x| x > 1e-9).count(), r.iter().filter(|&&x| x > 1e-3).count(), r.iter().filter(|&&x| x > 1e-2).count());
        let mut bs: Vec<&(String, f64, f64, f64)> = st.boot.iter().collect();
        bs.sort_by(|x, y| y.3.partial_cmp(&x.3).unwrap());
        for x in bs.iter().take(10) {
            println!("  BOOT {} : {} vs {} (rel {:.3e})", x.0, x.1, x.2, x.3);
        }
    }
    println!(
        "structural / string / bool / null differences: {}",
        st.structural.len()
    );
    for s in st.structural.iter().take(40) {
        println!("  {s}");
    }
}

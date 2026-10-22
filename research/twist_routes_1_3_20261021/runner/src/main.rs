//! Native execution of the frozen twist-route experiment program.
#![allow(dead_code)]
mod conditional;
mod covers;
mod isogeny;
mod jacobian;
mod pari;
mod registry;
pub use registry::{digest, identity};
mod solver;
#[path = "../../../../src/ecc/curve.rs"]
pub mod source_curve;
#[path = "../../../../src/ecc/field.rs"]
pub mod source_field;
#[path = "../../../../src/ecc/point.rs"]
pub mod source_point;
#[path = "../../../../src/hash/sha256.rs"]
mod source_sha256;
#[path = "../../../../src/cryptanalysis/twist_ic_field.rs"]
mod toy_field;
mod trace;
#[path = "../../../../src/utils/mod.rs"]
pub mod utils;
pub use source_curve as curve;
pub use source_field as field;
pub use source_point as point;
pub mod ecc {
    pub use crate::source_curve as curve;
    pub use crate::source_field as field;
    pub use crate::source_point as point;
}
#[path = "../../../../src/cryptanalysis/f4_fp.rs"]
pub mod f4_fp;
#[path = "../../../../src/cryptanalysis/fx_hash.rs"]
pub mod fx_hash;
#[path = "../../../../src/cryptanalysis/groebner_f4.rs"]
pub mod groebner_f4;
#[path = "../../../../src/cryptanalysis/symmetrized_semaev.rs"]
pub mod source_symmetrized;
#[path = "../../../../src/cryptanalysis/twist_ic_algebra.rs"]
pub mod twist_ic_algebra;
pub mod cryptanalysis {
    pub use crate::f4_fp;
    pub use crate::source_symmetrized as symmetrized_semaev;
}
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::BTreeMap, fs, path::Path};
use toy_field::*;
type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Fixture {
    pub id: String,
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub order: u64,
    pub d: u64,
    pub generator: [u64; 2],
    pub role: String,
    pub extension_id: String,
    pub twist_id: String,
    pub cubic_extension_id: Option<String>,
    pub sextic_extension_id: Option<String>,
    pub sextic_modulus: Option<Vec<u64>>,
    pub sextic_flat_generator: Option<S>,
}
impl Fixture {
    pub fn curve(&self) -> Quad {
        Quad {
            p: self.p,
            d: self.d,
            a: self.a,
            b: self.b,
        }
    }
    pub fn group(&self) -> P {
        Some((Q(self.generator[0], 0), Q(self.generator[1], 0)))
    }
}
pub fn write_json(path: impl AsRef<Path>, v: &impl Serialize) -> Result<()> {
    fs::write(path, serde_json::to_string_pretty(v)? + "\n")?;
    Ok(())
}
fn next(s: &mut u64) -> u64 {
    *s = s.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = *s;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}
pub fn extension_order(p: u64, n: u64, k: u32) -> u128 {
    let t = p as i128 + 1 - n as i128;
    let (mut s0, mut s1) = (2i128, t);
    if k == 1 {
        return n as u128;
    }
    for _ in 2..=k {
        let s = t * s1 - p as i128 * s0;
        s0 = s1;
        s1 = s;
    }
    ((p as u128).pow(k) as i128 + 1 - s1) as u128
}
fn prepare(out: &Path) -> Result<()> {
    if out.exists() {
        return Err("prepare requires a fresh directory".into());
    }
    fs::create_dir_all(out)?;
    let registry_path = Path::new("docs/curves/registry.json");
    let registry_bytes = fs::read(registry_path)?;
    let registry: Value = serde_json::from_slice(&registry_bytes)?;
    let rows = registry["curves"].as_array().unwrap();
    let mut models = vec![];
    let mut rejects = vec![];
    for (p, a, b, role) in [
        (59, 1, 13, "structural"),
        (827, 1, 15, "structural"),
        (67, 1, 0, "even-control"),
        (113, 1, 0, "even-control"),
        (223, 208, 21, "odd-composite-control"),
    ] {
        models.push((p, a, b, role.to_string()));
    }
    // Existing even-order control coefficients are read from their registered models.
    for m in &mut models {
        if m.3 == "even-control" {
            let slug = if m.0 == 67 {
                "icv1-fp7-t4-0c09e92f"
            } else {
                "icv1-fp7-t2-4910f2d0"
            };
            let r = rows.iter().find(|r| r["slug"] == slug).unwrap();
            m.1 = r["params"]["a"].as_str().unwrap().parse()?;
            m.2 = r["params"]["b"].as_str().unwrap().parse()?;
        }
    }
    for bits in [8, 10, 12, 14] {
        for replicate in 0..2 {
            let mut seed = 20261021u64 + bits * 100 + replicate;
            let lo = 1u64 << (bits - 1);
            let mut accepted = None;
            for trial in 0..10000 {
                let mut p = lo + next(&mut seed) % lo;
                p |= 1;
                while !prime(p) {
                    p += 2;
                }
                if p >= 2 * lo {
                    continue;
                }
                let a = p - 3;
                let b = 1 + next(&mut seed) % (p - 1);
                if (4 * a % p * a % p * a + 27 * b % p * b) % p == 0 {
                    rejects.push(json!({"bits":bits,"replicate":replicate,"trial":trial,"p":p,"b":b,"reason":"singular"}));
                    continue;
                }
                let e = Quad::new(p, a, b);
                let n = e.rational().len() as u64;
                if prime(n)
                    && (p as i64 + 1 - n as i64) % p as i64 != 0
                    && !models.iter().any(|m| m.0 == p && m.1 == a && m.2 == b)
                {
                    accepted = Some((p, a, b, format!("ladder-{bits}-{replicate}")));
                    break;
                }
                rejects.push(json!({"bits":bits,"replicate":replicate,"trial":trial,"p":p,"b":b,"order":n,"reason":"nonprime-order, supersingular or duplicate"}));
            }
            models.push(accepted.ok_or("fixture search budget exhausted")?);
        }
    }
    let mut fixtures = vec![];
    let mut additions = BTreeMap::new();
    for (p, a, b, role) in models {
        let e = Quad::new(p, a, b);
        let ps = e.rational();
        let n = ps.len() as u64;
        let source = identity(p, a, b, n as u128, None);
        let id = source["slug"].as_str().unwrap().to_string();
        if let Some(old) = rows.iter().find(|r| r["slug"] == id) {
            assert_eq!(old["model_json"], source["model_json"]);
            assert_eq!(old["order"], source["order"]);
        } else {
            additions.insert(id.clone(), source);
        }
        let tw_a = e.d * e.d % p * a % p;
        let tw_b = e.d * e.d % p * e.d % p * b % p;
        let tw = Quad::new(p, tw_a, tw_b);
        let nt = tw.rational().len() as u64;
        assert_eq!(n + nt, 2 * (p + 1));
        let tw_row = identity(p, tw_a, tw_b, nt as u128, None);
        let tw_id = tw_row["slug"].as_str().unwrap().to_string();
        additions.insert(tw_id.clone(), tw_row);
        let ext = identity(p, a, b, n as u128 * nt as u128, Some(&[(p - e.d) % p, 0]));
        let ext_id = ext["slug"].as_str().unwrap().to_string();
        additions.insert(ext_id.clone(), ext);
        let (mut p3, mut p6, mut md6, mut generator6) = (None, None, None, None);
        if prime(n) && n % 2 == 1 {
            let k = Sext::new(e);
            let row = identity(p, a, b, extension_order(p, n, 3), Some(&[b, a, 0]));
            p3 = Some(row["slug"].as_str().unwrap().to_string());
            additions.insert(p3.clone().unwrap(), row);
            let c = (0..p)
                .find(|&c| {
                    let h = k.sub(S([0, 1, 0, 0, 0, 0]), k.c(c));
                    k.pow(h, ((p as u128).pow(3) - 1) / 2) != ONE
                })
                .unwrap();
            let theta = k.sqrt(k.sub(S([0, 1, 0, 0, 0, 0]), k.c(c))).unwrap();
            let md = vec![e.rhs(Q(c, 0)).0, 0, (3 * c * c + a) % p, 0, 3 * c % p, 0];
            let row = identity(p, a, b, extension_order(p, n, 6), Some(&md));
            p6 = Some(row["slug"].as_str().unwrap().to_string());
            additions.insert(p6.clone().unwrap(), row);
            assert_eq!(
                k.add(
                    k.pow(theta, 6),
                    k.add(
                        k.mul(k.c(md[4]), k.pow(theta, 4)),
                        k.add(k.mul(k.c(md[2]), k.pow(theta, 2)), k.c(md[0]))
                    )
                ),
                Z
            );
            assert_ne!(k.pow(theta, (p as u128).pow(3)), theta);
            assert_ne!(k.pow(theta, (p as u128).pow(2)), theta);
            md6 = Some(md);
            generator6 = Some(theta);
        }
        let g = ps
            .iter()
            .flatten()
            .find(|p| e.pm(Some(**p), n as u128).is_none())
            .ok_or("no full order generator")?;
        fixtures.push(Fixture {
            id,
            p,
            a,
            b,
            order: n,
            d: e.d,
            generator: [g.0 .0, g.1 .0],
            role,
            extension_id: ext_id,
            twist_id: tw_id,
            cubic_extension_id: p3,
            sextic_extension_id: p6,
            sextic_modulus: md6,
            sextic_flat_generator: generator6,
        });
    }
    // Composite/even controls need a nonidentity point, not a prime-order generator.
    write_json(
        out.join("fixtures.json"),
        &json!({"schema":"twist.execution-fixtures/v1","selection_seed":20261021,"registry_sha256":digest(&registry_bytes),"fixtures":fixtures,"rejected_ladder_candidates":rejects,"sextic_basis":"1,alpha,alpha^2,omega,alpha*omega,alpha^2*omega; alpha^3+A*alpha+B=0, omega^2=d; flat generator theta and its irreducible degree-six modulus are recorded"}),
    )?;
    let mut new_registry = registry.clone();
    let list = new_registry["curves"].as_array_mut().unwrap();
    for (id, row) in additions {
        if !list.iter().any(|r| r["slug"] == id) {
            list.push(row);
        }
    }
    list.sort_by(|a, b| a["slug"].as_str().cmp(&b["slug"].as_str()));
    write_json(out.join("registry.json"), &new_registry)?;
    println!(
        "{}",
        json!({"status":"prepared","fixtures":fixtures.len(),"manifest":out.join("fixtures.json"),"registry":out.join("registry.json")})
    );
    Ok(())
}
fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("prepare") => prepare(Path::new(args.get(2).ok_or("missing output directory")?)),
        Some("solver" | "f4-worker") => solver::main(&args[1..]),
        Some("run" | "trace" | "geometry" | "conditional" | "isogeny") => {
            let manifest = fs::read(args.get(2).ok_or("missing fixture manifest")?)?;
            let v: Value = serde_json::from_slice(&manifest)?;
            let fixtures: Vec<Fixture> = serde_json::from_value(v["fixtures"].clone())?;
            let out = Path::new(args.get(3).ok_or("missing output directory")?);
            if out.exists() {
                return Err("run requires a fresh directory".into());
            }
            fs::create_dir_all(out)?;
            let mut statuses = vec![];
            if args[1] == "run" || args[1] == "trace" {
                statuses.extend(trace::run(&fixtures, out)?);
            }
            if args[1] == "run" || args[1] == "geometry" {
                statuses.extend(covers::run(&fixtures, out)?);
            }
            if args[1] == "run" || args[1] == "conditional" {
                statuses.extend(conditional::run(&fixtures, out)?);
            }
            if args[1] == "isogeny" {
                statuses.extend(isogeny::run(&fixtures, out)?);
            }
            write_json(
                out.join("SUMMARY.json"),
                &json!({"schema":"twist.execution-summary/v1","fixture_manifest_sha256":digest(&manifest),"results":statuses,"performance_claim":false,"independent_host_replay":false}),
            )?;
            println!(
                "{}",
                json!({"status":"finished","experiments":statuses.len(),"output":out})
            );
            Ok(())
        }
        Some("transfer" | "pari") => {
            let manifest: Value =
                serde_json::from_slice(&fs::read(args.get(2).ok_or("missing fixtures")?)?)?;
            let fixtures: Vec<Fixture> = serde_json::from_value(manifest["fixtures"].clone())?;
            let selected: Vec<covers::Selected> =
                serde_json::from_slice(&fs::read(args.get(3).ok_or("missing selected covers")?)?)?;
            let out = Path::new(args.get(4).ok_or("missing fresh directory")?);
            if out.exists() {
                return Err("transfer requires a fresh directory".into());
            }
            fs::create_dir_all(out)?;
            let rows = if args[1] == "transfer" {
                covers::transfer(&fixtures, &selected, out)?
            } else {
                pari::run(&fixtures, &selected, out)?
            };
            println!(
                "{}",
                json!({"status":"finished-transfer","rows":rows.len(),"output":out})
            );
            Ok(())
        }
        _ => Err(
            "usage: twist-route-experiments prepare DIR | run FIXTURES FRESH_DIR | solver ..."
                .into(),
        ),
    }
}

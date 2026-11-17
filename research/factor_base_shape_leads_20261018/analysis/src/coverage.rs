//! Independent small binary-field arithmetic audit; no scalar-log search.
use super::{json, num, points, read, text, Result, Value};
use std::{collections::BTreeSet, path::Path};
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: u64,
    y: u64,
    infinity: bool,
}
const O: Point = Point {
    x: 0,
    y: 0,
    infinity: true,
};
struct Curve {
    n: u32,
    modulus: u64,
    a: u64,
}
impl Curve {
    fn mul(&self, mut x: u64, mut y: u64) -> u64 {
        let mut z = 0;
        let high = 1u64 << self.n;
        while y != 0 {
            if y & 1 != 0 {
                z ^= x;
            }
            y >>= 1;
            x <<= 1;
            if x & high != 0 {
                x ^= self.modulus;
            }
        }
        z
    }
    fn sq(&self, x: u64) -> u64 {
        self.mul(x, x)
    }
    fn inv(&self, x: u64) -> Result<u64> {
        if x == 0 {
            return Err("inverse of zero".into());
        }
        let (mut y, mut base, mut exponent) = (1, x, (1u64 << self.n) - 2);
        while exponent != 0 {
            if exponent & 1 != 0 {
                y = self.mul(y, base);
            }
            base = self.sq(base);
            exponent >>= 1;
        }
        if self.mul(x, y) != 1 {
            return Err("invalid field inverse".into());
        }
        Ok(y)
    }
    fn on_curve(&self, p: Point) -> bool {
        p.infinity
            || self.sq(p.y) ^ self.mul(p.x, p.y)
                == self.mul(self.sq(p.x), p.x) ^ self.mul(self.a, self.sq(p.x)) ^ 1
    }
    fn add(&self, p: Point, q: Point) -> Result<Point> {
        if p.infinity {
            return Ok(q);
        }
        if q.infinity {
            return Ok(p);
        }
        let (x, y) = if p.x == q.x {
            if p.y != q.y || p.x == 0 {
                return Ok(O);
            }
            let l = p.x ^ self.mul(p.y, self.inv(p.x)?);
            let x = self.sq(l) ^ l ^ self.a;
            (x, self.sq(p.x) ^ self.mul(l ^ 1, x))
        } else {
            let l = self.mul(p.y ^ q.y, self.inv(p.x ^ q.x)?);
            let x = self.sq(l) ^ l ^ p.x ^ q.x ^ self.a;
            (x, self.mul(l, p.x ^ x) ^ x ^ p.y)
        };
        let p = Point {
            x,
            y,
            infinity: false,
        };
        if !self.on_curve(p) {
            return Err("independent addition is off curve".into());
        }
        Ok(p)
    }
    fn scale(&self, mut p: Point, mut k: u64) -> Result<Point> {
        let mut q = O;
        while k != 0 {
            if k & 1 != 0 {
                q = self.add(q, p)?;
            }
            p = self.add(p, p)?;
            k >>= 1;
        }
        Ok(q)
    }
}
fn hex(s: &str) -> Result<u64> {
    Ok(u64::from_str_radix(s.trim_start_matches("0x"), 16)?)
}
fn prime(r: u64) -> bool {
    r >= 2
        && (2..)
            .take_while(|d| d <= &(r / d))
            .all(|d| !r.is_multiple_of(d))
}
pub fn coverage(root: &Path, registry: &Path, panels: &[String]) -> Result<Value> {
    let registry = read(registry)?;
    let models = registry["curves"]
        .as_array()
        .ok_or("registry missing curves")?;
    let mut rows = Vec::new();
    let (mut checks, mut witnesses) = (0usize, 0usize);
    for panel in panels {
        let summary = read(&root.join(panel).join("summary-0001.json"))?;
        for item in summary["candidates"]
            .as_array()
            .ok_or("missing candidates")?
        {
            let id = text(item, "id")?;
            let attempt = Path::new(text(item, "attempt")?)
                .file_name()
                .ok_or("invalid attempt")?;
            let base = read(
                &root
                    .join(panel)
                    .join("candidates")
                    .join(id)
                    .join(attempt)
                    .join("base.json"),
            )?;
            let slug = text(&base["curve"], "slug")?;
            if item["outcome"] != "completed" {
                rows.push(json!({"id":id,"panel":panel,"label":item["label"],"curve":slug,"outcome":item["outcome"],"prime_target_coverage":null}));
                continue;
            }
            let model = models
                .iter()
                .find(|m| m["slug"] == slug)
                .ok_or("unregistered model")?;
            let n = num(&model["params"], "n")? as u32;
            let a = num(&model["params"], "a")?;
            if !(2..=31).contains(&n)
                || a > 1
                || item["measurement"]["candidate"]["curve"]["a"] != a
                || item["measurement"]["candidate"]["curve"]["n"] != n
            {
                return Err("unsupported or mismatched model".into());
            }
            let c = Curve {
                n,
                modulus: hex(text(&model["params"], "modulus")?)?,
                a,
            };
            let r = num(&base["curve"], "r")?;
            if !prime(r) {
                return Err("nonprime subgroup".into());
            }
            let gen = &base["curve"]["generator"];
            let g = Point {
                x: hex(gen[0].as_str().ok_or("generator x")?)?,
                y: hex(gen[1].as_str().ok_or("generator y")?)?,
                infinity: false,
            };
            if !c.on_curve(g) || g.infinity || c.scale(g, r)? != O {
                return Err("independent generator-order check failed".into());
            }
            let pts: Vec<_> = points(&base)?
                .into_iter()
                .map(|(x, y)| {
                    Ok(Point {
                        x: hex(&x)?,
                        y: hex(&y)?,
                        infinity: false,
                    })
                })
                .collect::<Result<_>>()?;
            if pts.len() > 256 {
                return Err("coverage inventory exceeds frozen 256-point cap".into());
            }
            if pts.iter().any(|p| !c.on_curve(*p)) {
                return Err("independent inventory curve check failed".into());
            }
            let mut sums = BTreeSet::new();
            for &p in &pts {
                for &q in &pts {
                    let s = c.add(p, q)?;
                    if s != O {
                        sums.insert(s);
                    }
                }
            }
            let mut support = BTreeSet::new();
            for &s in &sums {
                if c.scale(s, r)? == O {
                    support.insert(s);
                }
            }
            let mut local_checks = 0;
            let mut local_witnesses = 0;
            for t in item["measurement"]["trials"]
                .as_array()
                .ok_or("missing trials")?
            {
                let target = Point {
                    x: t["target"][0].as_u64().ok_or("target x")?,
                    y: t["target"][1].as_u64().ok_or("target y")?,
                    infinity: false,
                };
                if !c.on_curve(target) {
                    return Err("independent target curve check failed".into());
                }
                if sums.contains(&target) != (t["ground_truth_decomposable"] == true) {
                    return Err("independent pair census contradicts native truth".into());
                }
                if (t["phase"] != "positive_control"
                    || item["measurement"]["positive_control_scope"] == "target-prime-subgroup")
                    && c.scale(target, r)? != O
                {
                    return Err("random probe outside subgroup".into());
                }
                if t["witness_verified"] == true {
                    let witness = t["witness_inventory_indices"]
                        .as_array()
                        .ok_or("missing witness")?;
                    let inv = base["points"]
                        .as_array()
                        .ok_or("missing native ordered points")?;
                    let get = |k: usize| -> Result<Point> {
                        let p = inv.get(k).ok_or("witness out of bounds")?;
                        Ok(Point {
                            x: hex(text(p, "x")?)?,
                            y: hex(text(p, "y")?)?,
                            infinity: false,
                        })
                    };
                    let p = get(witness[0].as_u64().ok_or("witness index")? as usize)?;
                    let q = get(witness[1].as_u64().ok_or("witness index")? as usize)?;
                    if c.add(p, q)? != target {
                        return Err("independent witness addition failed".into());
                    }
                    local_witnesses += 1;
                }
                local_checks += 1;
            }
            checks += local_checks;
            witnesses += local_witnesses;
            rows.push(json!({"id":id,"panel":panel,"label":item["label"],"curve":slug,"outcome":"completed",
              "r":r,"cofactor":base["curve"]["cofactor"],"columns":base["factor_base"]["columns"],"signed_points":pts.len(),
              "ordered_pairs_enumerated":pts.len()*pts.len(),"distinct_finite_pair_sums":sums.len(),
              "prime_target_coverage":support.len(),"possible_nonidentity_prime_targets":r-1,
              "native_probe_truth_checks":local_checks,"independent_witness_checks":local_witnesses,
              "accounting":"exhaustive diagnostic census; not a free decomposition oracle or speed result"}));
        }
    }
    Ok(
        json!({"schema":"factor_base_exact_coverage/v1","probe_truth_checks":checks,"point_witness_checks":witnesses,
      "arithmetic":"independent Rust polynomial-basis binary-field/group implementation, same host",
      "rows":rows,"degree_of_regularity":null,"full_pipeline_speedup":null}),
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    fn c() -> Curve {
        Curve {
            n: 7,
            modulus: 0x83,
            a: 1,
        }
    }
    fn g() -> Point {
        Point {
            x: 0x65,
            y: 0x59,
            infinity: false,
        }
    }
    #[test]
    fn every_nonzero_field_element_has_a_checked_inverse() {
        let c = c();
        for x in 1..128 {
            assert_eq!(c.mul(x, c.inv(x).unwrap()), 1);
        }
    }
    #[test]
    fn recorded_generator_has_order71_and_scalar_addition_agrees_with_repeated_addition() {
        let c = c();
        assert!(c.on_curve(g()));
        let mut p = O;
        for k in 0..=71 {
            assert_eq!(p, c.scale(g(), k).unwrap());
            p = c.add(p, g()).unwrap();
        }
        assert_eq!(c.scale(g(), 71).unwrap(), O);
    }
    #[test]
    fn inverse_points_and_associativity_cover_both_addition_branches() {
        let c = c();
        assert_eq!(
            c.add(
                g(),
                Point {
                    x: g().x,
                    y: g().x ^ g().y,
                    infinity: false
                }
            )
            .unwrap(),
            O
        );
        for i in 1..8 {
            for j in 1..8 {
                for k in 1..8 {
                    let (a, b, d) = (
                        c.scale(g(), i).unwrap(),
                        c.scale(g(), j).unwrap(),
                        c.scale(g(), k).unwrap(),
                    );
                    assert_eq!(
                        c.add(c.add(a, b).unwrap(), d).unwrap(),
                        c.add(a, c.add(b, d).unwrap()).unwrap()
                    );
                }
            }
        }
    }
}

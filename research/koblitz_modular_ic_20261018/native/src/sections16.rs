//! Explicit CM isogeny cover and elliptic factor bases for a diagonal representative.
use crate::{
    exact16::{det, Q},
    round04::read,
};
use num_bigint::BigInt as Z;
use num_integer::Integer;
use num_traits::{One, ToPrimitive, Zero};
use serde_json::{json, Value};
use std::path::Path;
#[derive(Clone, Debug)]
struct C {
    a: Q,
    b: Q,
}
impl C {
    fn z(n: i64) -> Self {
        Self {
            a: Q::z(n),
            b: Q::z(0),
        }
    }
    fn add(&self, c: &Self) -> Self {
        Self {
            a: self.a.add(&c.a),
            b: self.b.add(&c.b),
        }
    }
    fn neg(&self) -> Self {
        Self {
            a: self.a.neg(),
            b: self.b.neg(),
        }
    }
    fn sub(&self, c: &Self) -> Self {
        self.add(&c.neg())
    }
    fn mul(&self, c: &Self) -> Self {
        Self {
            a: self.a.mul(&c.a).sub(&self.b.mul(&c.b).mul(&Q::z(2))),
            b: self
                .a
                .mul(&c.b)
                .add(&self.b.mul(&c.a))
                .sub(&self.b.mul(&c.b)),
        }
    }
    fn bar(&self) -> Self {
        Self {
            a: self.a.sub(&self.b),
            b: self.b.neg(),
        }
    }
    fn div(&self, c: &Self) -> Self {
        let n = c.mul(&c.bar());
        assert!(n.b.zero());
        let d = self.mul(&c.bar());
        Self {
            a: d.a.div(&n.a),
            b: d.b.div(&n.a),
        }
    }
    fn val(&self) -> Value {
        json!({"a":self.a.integer().to_string(),"b":self.b.integer().to_string()})
    }
}
fn inner(h: &[Vec<C>], x: &[C], y: &[C]) -> C {
    let mut v = C::z(0);
    for i in 0..4 {
        for j in 0..4 {
            v = v.add(&x[i].bar().mul(&h[i][j]).mul(&y[j]));
        }
    }
    v
}
fn permutations() -> Vec<Vec<usize>> {
    let mut p = vec![];
    for a in 0..4 {
        for b in 0..4 {
            for c in 0..4 {
                for d in 0..4 {
                    let r = vec![a, b, c, d];
                    if (0..4).all(|i| (i + 1..4).all(|j| r[i] != r[j])) {
                        p.push(r);
                    }
                }
            }
        }
    }
    p
}
fn integral(v: &[C]) -> Vec<C> {
    let den = v
        .iter()
        .flat_map(|c| [&c.a, &c.b])
        .fold(Z::one(), |d, c| d.lcm(&c.d));
    let mut nums = v
        .iter()
        .map(|c| [&c.a.n * (&den / &c.a.d), &c.b.n * (&den / &c.b.d)])
        .collect::<Vec<_>>();
    let g = nums.iter().flatten().fold(Z::zero(), |a, b| a.gcd(b));
    assert!(!g.is_zero());
    for n in nums.iter_mut().flatten() {
        *n /= &g;
    }
    nums.into_iter()
        .map(|[a, b]| C {
            a: Q::new(a, Z::one()),
            b: Q::new(b, Z::one()),
        })
        .collect()
}
fn embedded(b: &[Vec<C>]) -> Vec<Vec<Z>> {
    let mut out = vec![vec![Z::zero(); 8]; 8];
    for i in 0..4 {
        for j in 0..4 {
            let a = b[j][i].a.integer();
            let w = b[j][i].b.integer();
            out[2 * i][2 * j] = a.clone();
            out[2 * i][2 * j + 1] = -2 * &w;
            out[2 * i + 1][2 * j] = w.clone();
            out[2 * i + 1][2 * j + 1] = a - w;
        }
    }
    out
}
fn riemann(h: &[Vec<C>]) -> Vec<Vec<Z>> {
    let mut j = vec![vec![Z::zero(); 8]; 8];
    for i in 0..4 {
        for k in 0..4 {
            for a in 0..2 {
                for b in 0..2 {
                    let mut x = vec![C::z(0); 4];
                    let mut y = x.clone();
                    x[i] = if a == 0 {
                        C::z(1)
                    } else {
                        C {
                            a: Q::z(0),
                            b: Q::z(1),
                        }
                    };
                    y[k] = if b == 0 {
                        C::z(1)
                    } else {
                        C {
                            a: Q::z(0),
                            b: Q::z(1),
                        }
                    };
                    j[2 * i + a][2 * k + b] = inner(h, &x, &y).b.integer();
                }
            }
        }
    }
    j
}
fn congruence(j: &[Vec<Z>], b: &[Vec<Z>]) -> Vec<Vec<Z>> {
    let mut a = vec![vec![Z::zero(); 8]; 8];
    for i in 0..8 {
        for k in 0..8 {
            for x in 0..8 {
                for y in 0..8 {
                    a[i][k] += &b[x][i] * &j[x][y] * &b[y][k];
                }
            }
        }
    }
    a
}
fn matrix(a: &[Vec<Z>]) -> String {
    format!(
        "!![{}]",
        a.iter()
            .map(|r| r
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(","))
            .collect::<Vec<_>>()
            .join(";")
    )
}
fn parity(z: &Z) -> u8 {
    z.mod_floor(&Z::from(2)).to_u8().unwrap()
}
fn delta(c: &C) -> u8 {
    parity(&(c.a.integer() * c.b.integer()))
}
fn factor_basis(d: usize, twist: u8) -> Vec<Value> {
    assert!(d <= 4096 && d >= 3);
    let bound = d - usize::from(twist);
    let mut v = vec![json!({"function":"1","x_power":0,"y_power":0,"pole_O":0,"pole_T":0})];
    for i in 1..=bound / 2 {
        v.push(json!({"function":"monomial","x_power":i,"y_power":0,"pole_O":2*i,"pole_T":0}));
    }
    if bound >= 3 {
        for i in 0..=(bound - 3) / 2 {
            v.push(
                json!({"function":"monomial","x_power":i,"y_power":1,"pole_O":2*i+3,"pole_T":0}),
            );
        }
    }
    if twist != 0 {
        v.push(json!({"function":"(y+1)/x","pole_O":1,"pole_T":1}));
    }
    assert_eq!(v.len(), d);
    v
}
pub fn run(dir: &Path) {
    let data = read(&dir.join("g15-theta-class-descent.json"));
    let mut records = vec![];
    let mut lean=String::from("import KoblitzRound15Theta\nset_option maxRecDepth 100000\nset_option maxHeartbeats 0\nnamespace KoblitzRound16Sections\n");
    for (g, r) in data["records"].as_array().unwrap().iter().enumerate() {
        let h = r["H"]
            .as_array()
            .unwrap()
            .iter()
            .map(|rr| {
                rr.as_array()
                    .unwrap()
                    .iter()
                    .map(|c| C {
                        a: Q::z(c["a"].as_i64().unwrap()),
                        b: Q::z(c["b"].as_i64().unwrap()),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut candidates = vec![];
        let mut best = None;
        for perm in permutations() {
            let mut cols: Vec<Vec<C>> = vec![];
            for k in &perm {
                let mut v = vec![C::z(0); 4];
                v[*k] = C::z(1);
                for c in &cols {
                    let ratio = inner(&h, c, &v).div(&inner(&h, c, c));
                    for i in 0..4 {
                        v[i] = v[i].sub(&c[i].mul(&ratio));
                    }
                }
                cols.push(integral(&v));
            }
            let d = cols
                .iter()
                .map(|c| {
                    let n = inner(&h, c, c);
                    assert!(n.b.zero());
                    let v = n.a.integer();
                    assert!(v > Z::zero());
                    v
                })
                .collect::<Vec<_>>();
            for i in 0..4 {
                for k in 0..4 {
                    if i != k {
                        let z = inner(&h, &cols[i], &cols[k]);
                        assert!(z.a.zero() && z.b.zero());
                    }
                }
            }
            let prod = d.iter().product::<Z>();
            let emb = embedded(&cols);
            let degree = det(emb.clone());
            assert_eq!(prod, Z::from(19) * &degree);
            let key = (
                degree.is_even(),
                (&degree % 11u32).is_zero(),
                degree.clone(),
            );
            candidates.push(json!({"permutation":perm,"diagonal":d.iter().map(ToString::to_string).collect::<Vec<_>>(),"cover_degree":degree.to_string()}));
            if best.as_ref().is_none_or(|(kk, _, _, _)| key < *kk) {
                best = Some((key, cols, d, emb));
            }
        }
        let ((_, _, degree), cols, d, emb) = best.unwrap();
        let differential = (0..4)
            .map(|i| (0..4).map(|k| cols[k][i].a.integer()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let differential_det = det(differential);
        let j = riemann(&h);
        let mut diag = vec![vec![C::z(0); 4]; 4];
        for i in 0..4 {
            diag[i][i] = C {
                a: Q::new(d[i].clone(), Z::one()),
                b: Q::z(0),
            };
        }
        let jd = riemann(&diag);
        assert_eq!(congruence(&j, &emb), jd);
        let correction = r["actual_theta_correction_bits"].as_u64().unwrap();
        let mut biases = vec![];
        let mut factors = vec![];
        let mut total = Z::one();
        for k in 0..4 {
            let v = &cols[k];
            let mut bias = 0u8;
            for i in 0..4 {
                bias ^= parity(&h[i][i].a.integer()) * delta(&v[i]);
                bias ^= (((correction >> i) & 1) as u8) * parity(&v[i].a.integer());
                for l in i + 1..4 {
                    let w = h[i][l].mul(&v[l]);
                    bias ^= delta(&v[i].add(&w)) ^ delta(&v[i]) ^ delta(&w);
                }
            }
            let dd = (&d[k] * 3u32).to_usize().unwrap();
            let basis = factor_basis(dd, bias);
            assert!(
                factors
                    .iter()
                    .map(|f: &Value| f["basis"].as_array().unwrap().len())
                    .sum::<usize>()
                    + basis.len()
                    <= 100000
            );
            total *= dd;
            biases.push(bias);
            factors.push(json!({"factor":k,"degree":dd,"Pic2_bias":bias,"divisor":if bias==0{"d*O"}else{"(d-1)*O+T"},"basis":basis}));
        }
        let basis = factor_basis(57, 0);
        total *= 57u32;
        factors.push(json!({"factor":4,"degree":57,"Pic2_bias":0,"divisor":"57*O","basis":basis}));
        assert_eq!(total, Z::from(87723) * &degree);
        records.push(json!({"graph":g,"target_trace":r["target_trace"],"H":r["H"],"candidates":candidates,"B_columns":cols.iter().map(|c|c.iter().map(C::val).collect::<Vec<_>>()).collect::<Vec<_>>(),"diagonal_H":d.iter().map(ToString::to_string).collect::<Vec<_>>(),"cover_degree":degree.to_string(),"cover_degree_odd":degree.is_odd(),"ordinary_differential_determinant":differential_det.to_string(),"ordinary_cover_separable":differential_det.is_odd(),"split_cover_field":"F32","F2_cover_map_constructed":false,"cover_degree_unit_at11":!(&degree%11u32).is_zero(),"B_integral8":emb.iter().map(|r|r.iter().map(ToString::to_string).collect::<Vec<_>>()).collect::<Vec<_>>(),"riemann_H":j.iter().map(|r|r.iter().map(ToString::to_string).collect::<Vec<_>>()).collect::<Vec<_>>(),"riemann_D":jd.iter().map(|r|r.iter().map(ToString::to_string).collect::<Vec<_>>()).collect::<Vec<_>>(),"diagonal_Pic2_biases":biases,"factor_bases":factors,"tensor_basis":{"representation":"Cartesian product of the five explicit factor-function lists;product rational functions","sections":total.to_string(),"materialized":false},"original_mixed_divisor_trivialization_constructed":false,"theta_action_matrices_constructed":false,"F2_quotient_sections_constructed":false,"proper_quotient_equations_constructed":false}));
        lean.push_str(&format!("def B{g} : Matrix (Fin 8) (Fin 8) ℤ := {}\ndef J{g} : Matrix (Fin 8) (Fin 8) ℤ := {}\ndef D{g} : Matrix (Fin 8) (Fin 8) ℤ := {}\ntheorem actual_diagonalization{g} : B{g}.transpose*J{g}*B{g}=D{g} := by decide +kernel\ntheorem actual_section_count{g} : 87723*({}:ℕ)={} := by norm_num\n",matrix(&emb),matrix(&j),matrix(&jd),degree,total));
        println!("G16:graph{g},diagonal{d:?},cover degree{degree},tensor representative sections{total};theta action/descent open");
    }
    lean.push_str("end KoblitzRound16Sections\n");
    for g in 0..4 {
        for n in ["actual_diagonalization", "actual_section_count"] {
            lean.push_str(&format!("#print axioms KoblitzRound16Sections.{n}{g}\n"));
        }
    }
    assert!(!dir.join("KoblitzRound16Sections.lean").exists());
    std::fs::write(dir.join("KoblitzRound16Sections.lean"), lean).unwrap();
    crate::round02::save(
        dir,
        "g16-diagonal-cover-section-bases.json",
        &json!({"schema":"koblitz.round16/g16-v1","records":records,"scope":"actual finite CM isogeny and explicit bases for an isomorphic diagonal representative;bundle trivialization,theta actions and target section descent unconstructed"}),
    );
}

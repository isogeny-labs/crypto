//! Exact integral Hermitian polarizations on the E^5 augmentation lattice.
use crate::{gluing, round02};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::Path;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
struct O {
    a: i128,
    b: i128,
} // a+b*w, w^2+w+2=0
impl O {
    fn new(a: i128, b: i128) -> Self {
        Self { a, b }
    }
    fn add(self, v: Self) -> Self {
        Self::new(self.a + v.a, self.b + v.b)
    }
    fn neg(self) -> Self {
        Self::new(-self.a, -self.b)
    }
    fn sub(self, v: Self) -> Self {
        self.add(v.neg())
    }
    fn mul(self, v: Self) -> Self {
        Self::new(
            self.a * v.a - 2 * self.b * v.b,
            self.a * v.b + self.b * v.a - self.b * v.b,
        )
    }
    fn bar(self) -> Self {
        Self::new(self.a - self.b, -self.b)
    }
}
type IMatrix = Vec<Vec<i128>>;
type FMatrix = Vec<Vec<u64>>;
fn det_o(h: &[Vec<O>]) -> O {
    if h.is_empty() {
        return O::new(1, 0);
    }
    (0..h.len()).fold(O::default(), |v, j| {
        let minor: Vec<_> = h
            .iter()
            .skip(1)
            .map(|r| {
                r.iter()
                    .enumerate()
                    .filter(|(c, _)| *c != j)
                    .map(|(_, v)| *v)
                    .collect()
            })
            .collect();
        let term = h[0][j].mul(det_o(&minor));
        v.add(if j % 2 == 0 { term } else { term.neg() })
    })
}
fn transpose(a: &IMatrix) -> IMatrix {
    (0..a[0].len())
        .map(|j| a.iter().map(|r| r[j]).collect())
        .collect()
}
fn multiply(a: &IMatrix, b: &IMatrix) -> IMatrix {
    a.iter()
        .map(|r| {
            (0..b[0].len())
                .map(|j| r.iter().enumerate().map(|(k, v)| v * b[k][j]).sum())
                .collect()
        })
        .collect()
}
fn modulo(a: &IMatrix, p: u64) -> FMatrix {
    a.iter()
        .map(|r| r.iter().map(|v| v.rem_euclid(p as i128) as u64).collect())
        .collect()
}
fn mm(a: &FMatrix, b: &FMatrix, p: u64) -> FMatrix {
    a.iter()
        .map(|r| {
            (0..b[0].len())
                .map(|j| r.iter().enumerate().map(|(k, v)| v * b[k][j]).sum::<u64>() % p)
                .collect()
        })
        .collect()
}
fn pow(mut a: u64, mut n: u64, p: u64) -> u64 {
    let mut b = 1;
    while n > 0 {
        if n & 1 == 1 {
            b = b * a % p;
        }
        a = a * a % p;
        n >>= 1;
    }
    b
}
fn nullspace(rows: FMatrix, p: u64) -> Vec<Vec<u64>> {
    let n = rows[0].len();
    let (r, pivots) = gluing::rref(rows, p);
    (0..n)
        .filter(|i| !pivots.contains(i))
        .map(|free| {
            let mut v = vec![0; n];
            v[free] = 1;
            for (i, c) in pivots.iter().enumerate() {
                v[*c] = (p - r[i][free]) % p;
            }
            v
        })
        .collect()
}
fn augmentation() -> IMatrix {
    (0..4)
        .map(|i| {
            (0..4)
                .map(|j| {
                    if i == 0 {
                        -1
                    } else if j + 1 == i {
                        1
                    } else {
                        0
                    }
                })
                .collect()
        })
        .collect()
}
fn form(u: O, v: O) -> Vec<Vec<O>> {
    let row = [O::default(), u, v, v.bar(), u.bar()];
    (0..4)
        .map(|i| {
            (0..4)
                .map(|j| {
                    row[(j + 5 - i) % 5]
                        .sub(row[(4 + 5 - i) % 5])
                        .sub(row[(j + 1) % 5])
                })
                .collect()
        })
        .collect()
}
fn riemann(h: &[Vec<O>]) -> IMatrix {
    let mut m = vec![vec![0; 8]; 8];
    for i in 0..4 {
        for j in 0..4 {
            let O { a, b } = h[i][j];
            m[2 * i][2 * j] = a;
            m[2 * i][2 * j + 1] = -2 * b;
            m[2 * i + 1][2 * j] = b;
            m[2 * i + 1][2 * j + 1] = a - b;
        }
    }
    let mut j = vec![vec![0; 8]; 8];
    for i in 0..4 {
        j[2 * i][2 * i + 1] = 1;
        j[2 * i + 1][2 * i] = -1;
    }
    multiply(&j, &m)
}
fn frobenius(t: i64) -> IMatrix {
    let s = augmentation();
    let w = if t == -1 {
        [[0, -2], [1, -1]]
    } else {
        [[0, 2], [-1, 1]]
    };
    (0..8)
        .map(|i| (0..8).map(|j| s[i / 2][j / 2] * w[i % 2][j % 2]).collect())
        .collect()
}
fn kernel_data(j: &IMatrix, frob: &IMatrix, p: u64) -> (FMatrix, FMatrix, u64) {
    let basis = nullspace(modulo(j, p), p);
    assert_eq!(basis.len(), 2);
    let k: FMatrix = (0..8).map(|i| vec![basis[0][i], basis[1][i]]).collect();
    let mut pair = 0i128;
    for row in 0..8 {
        for col in 0..8 {
            pair += basis[0][row] as i128 * j[row][col] * basis[1][col] as i128;
        }
    }
    assert_eq!(pair % p as i128, 0);
    let pairing = (pair / p as i128).rem_euclid(p as i128) as u64;
    assert_ne!(pairing, 0);
    let image = mm(&modulo(frob, p), &k, p);
    let (r, s, d) = (0..8)
        .find_map(|r| {
            (r + 1..8).find_map(|s| {
                let d = (k[r][0] * k[s][1] + p - k[r][1] * k[s][0] % p) % p;
                if d != 0 {
                    Some((r, s, d))
                } else {
                    None
                }
            })
        })
        .unwrap();
    let inv = pow(d, p - 2, p);
    let mut fk = vec![vec![0; 2]; 2];
    for c in 0..2 {
        fk[0][c] = (image[r][c] * k[s][1] + p - image[s][c] * k[r][1] % p) * inv % p;
        fk[1][c] = (k[r][0] * image[s][c] + p - k[s][0] * image[r][c] % p) * inv % p;
    }
    assert_eq!(image, mm(&k, &fk, p));
    (k, fk, pairing)
}
fn anti_maps(fk: &FMatrix, pair: u64, trace: i64, p: u64) -> Value {
    let e = vec![
        vec![0, (p - 2) % p],
        vec![1, trace.rem_euclid(p as i64) as u64],
    ];
    let mut eq = vec![vec![0; 4]; 4];
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                eq[2 * i + j][2 * k + j] = (eq[2 * i + j][2 * k + j] + fk[i][k]) % p;
                eq[2 * i + j][2 * i + k] = (eq[2 * i + j][2 * i + k] + p - e[k][j]) % p;
            }
        }
    }
    let bs = nullspace(eq, p);
    assert!(bs.len() <= 4);
    let mut invertible = 0;
    let mut anti = 0;
    let mut first = None;
    for code in 1..p.pow(bs.len() as u32) {
        let mut code = code;
        let mut v = vec![0; 4];
        for b in &bs {
            let c = code % p;
            code /= p;
            for i in 0..4 {
                v[i] = (v[i] + c * b[i]) % p;
            }
        }
        let d = (v[0] * v[3] + p - v[1] * v[2] % p) % p;
        if d == 0 {
            continue;
        }
        invertible += 1;
        let m = vec![vec![v[0], v[1]], vec![v[2], v[3]]];
        assert_eq!(mm(fk, &m, p), mm(&m, &e, p));
        if pair * d % p != p - 1 {
            continue;
        }
        anti += 1;
        if first.is_none() {
            first = Some(m);
        }
    }
    json!({"elliptic_trace":trace,"frobenius":e,"hom_dimension":bs.len(),"invertible_intertwiners":invertible,"anti_isometries":anti,"first_anti_isometry":first,"graph_isotropic":anti>0,"graph_half_dimension":if anti>0{Some(2)}else{None},"trivial_intersection_with_each_factor":anti>0})
}
pub fn run(dir: &Path) {
    run_scan(dir, false);
}
pub fn run_extended(dir: &Path) {
    run_scan(dir, true);
}
fn run_scan(dir: &Path, extended: bool) {
    let mut tested = Vec::new();
    let mut candidates = Vec::new();
    let mut positive = 0;
    for a in -3..=3 {
        for b in -3..=3 {
            for c in -3..=3 {
                for d in -3..=3 {
                    for residue in 0..if extended { 5 } else { 1 } {
                        let h = if extended {
                            let numerator = form(
                                O::new(5 * a + residue, 5 * b + 2 * residue),
                                O::new(5 * c + 2 * residue, 5 * d + 4 * residue),
                            );
                            numerator
                                .iter()
                                .map(|row| {
                                    row.iter()
                                        .map(|v| {
                                            assert_eq!((v.a % 5, v.b % 5), (0, 0));
                                            O::new(v.a / 5, v.b / 5)
                                        })
                                        .collect()
                                })
                                .collect()
                        } else {
                            form(O::new(a, b), O::new(c, d))
                        };
                        for i in 0..4 {
                            for j in 0..4 {
                                assert_eq!(h[i][j], h[j][i].bar());
                            }
                        }
                        let minors: Vec<_> = (1..=4)
                            .map(|n| {
                                let m: Vec<_> = h.iter().take(n).map(|r| r[..n].to_vec()).collect();
                                let v = det_o(&m);
                                assert_eq!(v.b, 0);
                                v.a
                            })
                            .collect();
                        let pos = minors.iter().all(|v| *v > 0);
                        if pos {
                            positive += 1;
                        }
                        let determinant = minors[3];
                        tested.push(json!({"coefficients":[a,b,c,d],"congruence_coset":residue,"leading_minors":minors,"positive_definite":pos}));
                        if !pos || ![1, 5, 11, 19].contains(&determinant) {
                            continue;
                        }
                        let j = riemann(&h);
                        let jt = transpose(&j);
                        assert_eq!(
                            jt,
                            j.iter()
                                .map(|r| r.iter().map(|v| -v).collect())
                                .collect::<IMatrix>()
                        );
                        for t in [-1, 1] {
                            let f = frobenius(t);
                            assert_eq!(
                                multiply(&multiply(&transpose(&f), &j), &f),
                                j.iter()
                                    .map(|r| r.iter().map(|v| 2 * v).collect())
                                    .collect::<IMatrix>()
                            );
                            let mut records = Vec::new();
                            let mut kernel = Value::Null;
                            if determinant > 1 {
                                let p = determinant as u64;
                                let (k, fk, pair) = kernel_data(&j, &f, p);
                                for trace in -2..=2 {
                                    records.push(anti_maps(&fk, pair, trace, p));
                                }
                                kernel = json!({"basis_columns":k,"frobenius":fk,"pairing_coefficient":pair,"dimension":2});
                            }
                            candidates.push(json!({"coefficients":[a,b,c,d],"congruence_coset":residue,"target_trace":t,"hermitian_matrix":h,"leading_minors":minors,"determinant":determinant,"polarization_type":if determinant==1{vec![1,1,1,1]}else{vec![1,1,1,determinant]},"riemann_form":j,"frobenius":f,"kernel":kernel,"elliptic_gluings":records,"positive_integral_invariant_form":true,"jacobian_realized":false}));
                        }
                    }
                }
            }
        }
    }
    println!(
        "GL2: {} forms, {} positive, {} sign-specific small-determinant candidates",
        tested.len(),
        positive,
        candidates.len()
    );
    round02::save(
        dir,
        if extended {
            "gl2b-integral-polarizations.json"
        } else {
            "gl2-integral-polarizations.json"
        },
        &json!({"schema":"koblitz.round03/gl2-v1","coefficient_bound":3,"forms_tested":tested.len(),"positive_forms":positive,"tests":tested,"candidates":candidates,"augmentation_action":augmentation(),"scope":"integral invariant Hermitian forms on the explicit augmentation model; positive forms and finite kernel graphs checked, quotient/Jacobian realization distinguished"}),
    );
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn natural_polarization() {
        let h = form(O::new(-1, 0), O::new(-1, 0));
        assert_eq!(det_o(&h), O::new(5, 0));
        for i in 0..4 {
            for j in 0..4 {
                assert_eq!(h[i][j], O::new(if i == j { 2 } else { 1 }, 0));
            }
        }
    }
}

fn det_integer(a: &IMatrix) -> i128 {
    let mut m = a.clone();
    let mut previous = 1;
    let mut sign = 1;
    for k in 0..m.len() - 1 {
        if m[k][k] == 0 {
            let p = (k + 1..m.len()).find(|i| m[*i][k] != 0).unwrap();
            m.swap(k, p);
            sign = -sign;
        }
        let pivot = m[k][k];
        for i in k + 1..m.len() {
            for j in k + 1..m.len() {
                let n = m[i][j] * pivot - m[i][k] * m[k][j];
                assert_eq!(n % previous, 0);
                m[i][j] = n / previous;
            }
        }
        for i in k + 1..m.len() {
            m[i][k] = 0;
        }
        previous = pivot;
    }
    sign * m[m.len() - 1][m.len() - 1]
}
pub fn verify(path: &Path) {
    let data: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
    let mut form_checks = 0;
    let mut kernel_checks = 0;
    let mut graphs = 0;
    for row in data["candidates"].as_array().unwrap() {
        let h: Vec<Vec<O>> = serde_json::from_value(row["hermitian_matrix"].clone()).unwrap();
        let j: IMatrix = serde_json::from_value(row["riemann_form"].clone()).unwrap();
        let f: IMatrix = serde_json::from_value(row["frobenius"].clone()).unwrap();
        let determinant = row["determinant"].as_i64().unwrap() as i128;
        assert_eq!(j, riemann(&h));
        assert_eq!(det_integer(&j), determinant * determinant);
        for n in 1..=4 {
            let m: Vec<_> = h.iter().take(n).map(|r| r[..n].to_vec()).collect();
            let v = det_o(&m);
            assert_eq!(v.b, 0);
            assert!(v.a > 0);
        }
        assert_eq!(
            multiply(&multiply(&transpose(&f), &j), &f),
            j.iter()
                .map(|r| r.iter().map(|v| 2 * v).collect())
                .collect::<IMatrix>()
        );
        form_checks += 1;
        if determinant == 1 {
            continue;
        }
        let p = determinant as u64;
        let k: FMatrix = serde_json::from_value(row["kernel"]["basis_columns"].clone()).unwrap();
        let fk: FMatrix = serde_json::from_value(row["kernel"]["frobenius"].clone()).unwrap();
        let pair = row["kernel"]["pairing_coefficient"].as_u64().unwrap();
        assert_eq!(gluing::rref(modulo(&j, p), p).1.len(), 6);
        assert_eq!(gluing::rref(k.clone(), p).1.len(), 2);
        assert!(mm(&modulo(&j, p), &k, p).iter().flatten().all(|v| *v == 0));
        assert_eq!(mm(&modulo(&f, p), &k, p), mm(&k, &fk, p));
        assert_eq!(
            (fk[0][0] * fk[1][1] + p - fk[0][1] * fk[1][0] % p) % p,
            2 % p
        );
        let mut lifted = 0i128;
        for r in 0..8 {
            for c in 0..8 {
                lifted += k[r][0] as i128 * j[r][c] * k[c][1] as i128;
            }
        }
        assert_eq!(lifted % p as i128, 0);
        assert_eq!((lifted / p as i128).rem_euclid(p as i128) as u64, pair);
        assert_ne!(pair, 0);
        kernel_checks += 1;
        for glue in row["elliptic_gluings"].as_array().unwrap() {
            let e: FMatrix = serde_json::from_value(glue["frobenius"].clone()).unwrap();
            let mut count = 0;
            if (fk[0][0] + fk[1][1]) % p == (e[0][0] + e[1][1]) % p {
                // Independent exhaustive matrix enumeration, with no Hom-space solver.
                for a in 0..p {
                    for b in 0..p {
                        for c in 0..p {
                            for d in 0..p {
                                let determinant = (a * d + p - b * c % p) % p;
                                if pair * determinant % p != p - 1 {
                                    continue;
                                }
                                let m = vec![vec![a, b], vec![c, d]];
                                if mm(&fk, &m, p) == mm(&m, &e, p) {
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
            assert_eq!(count, glue["anti_isometries"].as_u64().unwrap());
            if count > 0 {
                let m: FMatrix =
                    serde_json::from_value(glue["first_anti_isometry"].clone()).unwrap();
                assert_eq!(mm(&fk, &m, p), mm(&m, &e, p));
                assert_eq!(
                    pair * ((m[0][0] * m[1][1] + p - m[0][1] * m[1][0] % p) % p) % p,
                    p - 1
                );
                graphs += 1;
            }
        }
    }
    println!(
        "{}",
        json!({"status":"PASS","positive_forms_checked":form_checks,"integer_determinant_checks":form_checks,"kernel_pairings_and_frobenius_checked":kernel_checks,"anti_isometry_graph_cases":graphs,"uses_hom_nullspace_solver":false,"geometric_jacobian_claim":false})
    );
}

pub fn short_vectors(dir: &Path) {
    let input: Value = serde_json::from_slice(
        &std::fs::read(dir.join("gl2b-integral-polarizations.json")).unwrap(),
    )
    .unwrap();
    let mut results = Vec::new();
    for row in input["candidates"].as_array().unwrap() {
        if row["determinant"] != 19
            || !row["elliptic_gluings"]
                .as_array()
                .unwrap()
                .iter()
                .any(|g| g["anti_isometries"].as_u64().unwrap() > 0)
        {
            continue;
        }
        let h: Vec<Vec<O>> = serde_json::from_value(row["hermitian_matrix"].clone()).unwrap();
        let mut minimum = i128::MAX;
        let mut witness = None;
        let mut unit = None;
        for code in 0..3usize.pow(8) {
            let mut code = code;
            let mut z = Vec::new();
            for _ in 0..4 {
                let a = (code % 3) as i128 - 1;
                code /= 3;
                let b = (code % 3) as i128 - 1;
                code /= 3;
                z.push(O::new(a, b));
            }
            if z.iter().all(|v| *v == O::default()) {
                continue;
            }
            let mut norm = O::default();
            for i in 0..4 {
                for j in 0..4 {
                    norm = norm.add(z[i].bar().mul(h[i][j]).mul(z[j]));
                }
            }
            assert_eq!(norm.b, 0);
            assert!(norm.a > 0);
            if norm.a < minimum {
                minimum = norm.a;
                witness = Some(z.clone());
            }
            if norm.a == 1 {
                unit = Some(z);
            }
        }
        let mut retraction = Value::Null;
        if let Some(z) = &unit {
            let r: Vec<O> = (0..4)
                .map(|j| (0..4).fold(O::default(), |s, i| s.add(z[i].bar().mul(h[i][j]))))
                .collect();
            let rz = (0..4).fold(O::default(), |s, i| s.add(r[i].mul(z[i])));
            assert_eq!(rz, O::new(1, 0));
            // The row z^* H kills every H-kernel vector modulo19.
            let k: FMatrix =
                serde_json::from_value(row["kernel"]["basis_columns"].clone()).unwrap();
            for column in 0..2 {
                let mut image = O::default();
                for i in 0..4 {
                    image = image.add(r[i].mul(O::new(
                        k[2 * i][column] as i128,
                        k[2 * i + 1][column] as i128,
                    )));
                }
                assert_eq!((image.a.rem_euclid(19), image.b.rem_euclid(19)), (0, 0));
            }
            retraction =
                json!({"row":r,"composition_is_one":true,"kills_polarization_kernel_mod19":true});
        }
        results.push(json!({"coefficients":row["coefficients"],"congruence_coset":row["congruence_coset"],"target_trace":row["target_trace"],"vectors_tested":6560,"minimum_in_box":minimum,"minimizer":witness,"norm_one_witness":unit,"retraction":retraction,"scope":"bounded coordinate box; absent witness does not prove indecomposable"}));
    }
    let split = results
        .iter()
        .filter(|r| !r["norm_one_witness"].is_null())
        .count();
    println!(
        "GL4: {} glued determinant19 forms; {split} norm-one decomposition witnesses",
        results.len()
    );
    round02::save(
        dir,
        "gl4-short-vectors.json",
        &json!({"schema":"koblitz.round03/gl4-v1","coordinate_bound":1,"results":results,"forms_with_norm_one_witness":split}),
    );
}

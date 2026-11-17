//! Complementary F2-quadratic numerators with complete F32-quadratic y solving.
use super::*;
use std::collections::BTreeMap;
type MP = BTreeMap<[u8; 4], u16>;
fn exponents() -> Vec<[u8; 4]> {
    let mut z = vec![[0; 4]];
    for i in 0..4 {
        let mut x = [0; 4];
        x[i] = 1;
        z.push(x);
    }
    for i in 0..4 {
        for j in i..4 {
            let mut x = [0; 4];
            x[i] += 1;
            x[j] += 1;
            z.push(x);
        }
    }
    z
}
fn pmul(f: &SmallField, a: &MP, b: &MP) -> MP {
    let mut z = MP::new();
    for (x, c) in a {
        for (y, d) in b {
            let key = std::array::from_fn(|i| x[i] + y[i]);
            let v = z.get(&key).copied().unwrap_or(0) ^ f.mul(*c, *d);
            if v == 0 {
                z.remove(&key);
            } else {
                z.insert(key, v);
            }
        }
    }
    z
}
fn qpoly(q: usize) -> MP {
    let e = exponents();
    match q {
        0 => MP::from([(e[0], 1)]),
        1 => MP::from([(e[1], 1)]),
        2 => MP::from([(e[2], 1)]),
        3 => MP::from([(e[0], 1), (e[2], 1)]),
        4 => MP::from([(e[1], 1), (e[2], 1), ([0, 2, 0, 0], 1)]),
        5 => MP::from([(e[3], 1)]),
        6 => MP::from([(e[4], 1)]),
        7 => MP::from([(e[0], 1), (e[3], 1)]),
        _ => unreachable!(),
    }
}
fn scaled(f: &SmallField, a: &MP, c: u16) -> MP {
    a.iter()
        .filter_map(|(m, v)| {
            let z = f.mul(*v, c);
            (z != 0).then_some((*m, z))
        })
        .collect()
}
fn exact_constant(qid: usize, n: u16, m: &[u16], x: u16, y: u16) -> bool {
    let f = SmallField::new(5);
    let e = exponents();
    let nn = e
        .iter()
        .enumerate()
        .filter(|(i, _)| n >> i & 1 != 0)
        .map(|(_, v)| (*v, 1))
        .collect::<MP>();
    let mm = e
        .into_iter()
        .zip(m.iter().copied())
        .filter(|(_, c)| *c != 0)
        .collect::<MP>();
    let q = qpoly(qid);
    let q2 = pmul(&f, &q, &q);
    let q3 = pmul(&f, &q2, &q);
    nn == scaled(&f, &q2, x) && mm == scaled(&f, &q3, y)
}
pub fn run(dir: &Path, qid: usize) {
    assert!(qid < 8);
    let old = read(&dir.join("cm17-explicit-level77-incidence.json"));
    let hold = read(&dir.join("cm22-independent-elliptic-map-holdouts.json"));
    let f = SmallField::new(10);
    let em: Vec<u16> = serde_json::from_value(hold["embedding_F32_into_F1024"].clone()).unwrap();
    let mut points = old["finite_field_records"][4]["untwisted_incidence_points"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| {
            std::array::from_fn::<_, 5, _>(|j| {
                if j == 0 {
                    1
                } else {
                    em[v[j - 1].as_u64().unwrap() as usize]
                }
            })
        })
        .collect::<Vec<_>>();
    points.extend(hold["holdouts"].as_array().unwrap().iter().map(|v| {
        std::array::from_fn::<_, 5, _>(|j| {
            if j == 0 {
                1
            } else {
                v["input"][j - 1].as_u64().unwrap() as u16
            }
        })
    }));
    assert_eq!(points.len(), 18);
    let mut active = vec![];
    let mut skipped = vec![];
    let mut xt = vec![];
    let mut yt = vec![];
    for (id, b) in points.iter().enumerate() {
        let q = match qid {
            0 => 1,
            1 => b[1],
            2 => b[2],
            3 => 1 ^ b[2],
            4 => b[1] ^ f.mul(1 ^ b[2], b[2]),
            5 => b[3],
            6 => b[4],
            7 => 1 ^ b[3],
            _ => unreachable!(),
        };
        if q == 0 {
            skipped.push(id);
            continue;
        }
        active.push(id);
        let inv = f.inv(q);
        let inv2 = f.sqr(inv);
        let inv3 = f.mul(inv2, inv);
        let ms = monomials(b, &f);
        xt.push(ms.iter().map(|m| f.mul(*m, inv2)).collect::<Vec<_>>());
        yt.push(
            ms.iter()
                .flat_map(|m| (0..5).map(|bit| f.mul(f.mul(em[1 << bit], *m), inv3)))
                .collect::<Vec<_>>(),
        );
    }
    let ff = Field {
        m: 5,
        modulus: old["finite_field_records"][4]["modulus"].as_u64().unwrap(),
    };
    let mut records = vec![];
    for a in 0..2u16 {
        let mut allowed = vec![vec![false; 1024]; active.len()];
        for (j, id) in active.iter().enumerate() {
            if *id < 10 {
                for (x, z) in em.iter().enumerate() {
                    allowed[j][*z as usize] =
                        x == 0 || ff.trace(x as u32 ^ a as u32 ^ ff.inv(ff.sq(x as u32))) == 0;
                }
            } else {
                for x in 0..1024u16 {
                    allowed[j][x as usize] = x == 0 || f.trace(x ^ a ^ f.inv(f.sqr(x))) == 0;
                }
            }
        }
        let mut passed = 0;
        let mut consistent = 0;
        let mut fits = vec![];
        let mut spaces = vec![];
        for code in 0..1u16 << 15 {
            let xs = xt
                .iter()
                .map(|v| {
                    (0..15)
                        .filter(|i| code >> i & 1 != 0)
                        .fold(0, |z, i| z ^ v[i])
                })
                .collect::<Vec<_>>();
            if !xs.iter().enumerate().all(|(i, x)| allowed[i][*x as usize]) {
                continue;
            }
            passed += 1;
            let Some((part, basis)) = solve(equations(&f, &yt, &xs, a), 75) else {
                continue;
            };
            consistent += 1;
            if basis.len() > 20 {
                spaces.push(json!({"N_bits":code,"M_particular":decode(part,15),"M_nullspace":basis.iter().map(|b|decode(*b,15)).collect::<Vec<_>>()}));
                continue;
            }
            for mask in 0..1usize << basis.len() {
                let bits = basis
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| mask >> i & 1 != 0)
                    .fold(part, |z, (_, b)| z ^ b);
                let mc = decode(bits, 15);
                let ys = yt
                    .iter()
                    .map(|v| {
                        (0..75)
                            .filter(|i| bits >> i & 1 != 0)
                            .fold(0, |z, i| z ^ v[i])
                    })
                    .collect::<Vec<_>>();
                assert!(xs.iter().zip(&ys).all(|(x, y)| f.sqr(*y)
                    ^ f.mul(*x, *y)
                    ^ f.mul(*x, f.sqr(*x))
                    ^ f.mul(a, f.sqr(*x))
                    ^ 1
                    == 0));
                let candidate = if xs.iter().all(|x| *x == xs[0]) && ys.iter().all(|y| *y == ys[0])
                {
                    em.iter()
                        .position(|x| *x == xs[0])
                        .zip(em.iter().position(|y| *y == ys[0]))
                        .map(|(x, y)| (x as u16, y as u16))
                } else {
                    None
                };
                let exact = candidate.is_some_and(|(x, y)| exact_constant(qid, code, &mc, x, y));
                fits.push(json!({"N_bits":code,"M_coefficients":mc,"evaluated_x":xs,"evaluated_y":ys,"constant_value_F32":candidate.map(|(x,y)|[x,y]),"exact_free_polynomial_constant_map":exact}));
            }
        }
        let verified = fits
            .iter()
            .filter(|v| v["exact_free_polynomial_constant_map"] == true)
            .count();
        println!("CM25:q{qid},a{a},32768 quadratic F2 numerators,{passed} x-valid,{consistent} y-solves,{} finite fits,{verified} verified constants,{} unexpanded",fits.len(),spaces.len());
        records.push(json!({"a":a,"x_forms_examined":32768,"pointwise_valid_x":passed,"consistent_y_systems":consistent,"finite_fits":fits,"unexpanded_affine_spaces":spaces,"verified_constant_maps":verified}));
    }
    crate::round02::save(
        dir,
        &format!("cm25-quadratic-controlled-poles-q{qid}.json"),
        &json!({"schema":"koblitz.round20/cm25-v1","denominator_id":qid,"q":(["1","b","c","1+c","b+(1+c)c","lambda","nu","1+lambda"][qid]),"N_coefficient_field":"F2","N_basis_exponents":exponents(),"M_coefficient_field":"F32","M_basis_exponents":exponents(),"active_points":active,"zero_denominator_points":skipped,"records":records,"scope":"complete bounded complementary coefficient ansatz;finite fits need full function-field validation;free-polynomial constant identities separately checked"}),
    );
}
pub fn summary(dir: &Path) {
    let mut cases = vec![];
    let mut total = 0u64;
    let mut fits = 0;
    let mut constants = 0;
    let mut unexpanded = 0;
    let mut records = vec![];
    for q in 0..8 {
        let d = read(&dir.join(format!("cm25-quadratic-controlled-poles-q{q}.json")));
        for r in d["records"].as_array().unwrap() {
            total += r["x_forms_examined"].as_u64().unwrap();
            fits += r["finite_fits"].as_array().unwrap().len();
            unexpanded += r["unexpanded_affine_spaces"].as_array().unwrap().len();
            for v in r["finite_fits"].as_array().unwrap() {
                if v["exact_free_polynomial_constant_map"] == true {
                    constants += 1;
                    cases.push(format!(
                        "({},{},{})",
                        r["a"], v["constant_value_F32"][0], v["constant_value_F32"][1]
                    ));
                }
            }
            records.push(json!({"q":q,"a":r["a"],"fits":r["finite_fits"].as_array().unwrap().len(),"verified_constants":r["verified_constant_maps"]}));
        }
    }
    let text=format!("import KoblitzRound19Maps\nnamespace KoblitzRound20Maps\ndef cases : List (Nat×Nat×Nat) := [{}]\ntheorem all_verified_constant_residues : cases.all (fun c=> KoblitzRound19Maps.residual c==0)=true := by decide +kernel\ntheorem actual_constant_control_count : cases.length={} := by decide +kernel\nend KoblitzRound20Maps\n#print axioms KoblitzRound20Maps.all_verified_constant_residues\n#print axioms KoblitzRound20Maps.actual_constant_control_count\n",cases.join(","),constants);
    assert!(!dir.join("KoblitzRound20Maps.lean").exists());
    std::fs::write(dir.join("KoblitzRound20Maps.lean"), text).unwrap();
    crate::round02::save(
        dir,
        "cm26-quadratic-map-summary.json",
        &json!({"schema":"koblitz.round20/cm26-v1","x_forms_examined":total,"finite_fits":fits,"verified_constant_maps":constants,"unclassified_finite_fits":fits-constants,"unexpanded_affine_spaces":unexpanded,"records":records,"actual_integral_factor_map_constructed":false}),
    );
    println!("CM26:{total} quadratic cases,{fits} fits,{constants} exact constants,{} unclassified,{unexpanded} unexpanded",fits-constants);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn constant_map_test_uses_full_denominator_identity() {
        let mut mc = vec![0u16; 15];
        mc[0] = 1;
        assert!(exact_constant(0, 1, &mc, 1, 1));
        assert!(exact_constant(1, 1 << 5, &[0u16; 15], 1, 0));
        assert!(!exact_constant(1, (1 << 5) ^ 1, &[0u16; 15], 1, 0));
    }
}

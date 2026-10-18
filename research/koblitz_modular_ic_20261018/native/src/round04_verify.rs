//! Separate checks with the maintained binary library and BigInt elimination.
use crate::{
    binary_ecc::{self, BinaryCurve, BinaryPoint, F2mElement, IrreduciblePoly},
    round02,
    round04::*,
};
use num_bigint::{BigInt, BigUint};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};
fn point(v: &Value, m: u32) -> BinaryPoint {
    if v.is_array() {
        BinaryPoint::Affine {
            x: F2mElement::from_biguint(&BigUint::from(v[0].as_u64().unwrap()), m),
            y: F2mElement::from_biguint(&BigUint::from(v[1].as_u64().unwrap()), m),
        }
    } else {
        assert_eq!(v["infinity"], true);
        BinaryPoint::Infinity
    }
}
fn curve(m: u32, modulus: u64, a: u32, b: u32) -> BinaryCurve {
    BinaryCurve {
        m,
        irreducible: IrreduciblePoly {
            degree: m,
            low_terms: (0..m).filter(|i| modulus & (1 << i) != 0).collect(),
        },
        a: F2mElement::from_biguint(&BigUint::from(a), m),
        b: F2mElement::from_biguint(&BigUint::from(b), m),
        generator: BinaryPoint::Infinity,
        order: BigUint::from(11u32),
        cofactor: BigUint::from(1u32),
    }
}
fn sum(c: &BinaryCurve, ps: impl Iterator<Item = BinaryPoint>) -> BinaryPoint {
    ps.fold(BinaryPoint::Infinity, |s, p| {
        binary_ecc::curve::point_add(c, &s, &p)
    })
}
fn times(c: &BinaryCurve, p: &BinaryPoint, n: u64) -> BinaryPoint {
    binary_ecc::curve::scalar_mul(c, p, &BigUint::from(n))
}
fn source(v: &Value) -> Pt {
    if v.is_array() {
        Pt::A(v[0].as_u64().unwrap() as u32, v[1].as_u64().unwrap() as u32)
    } else {
        Pt::O
    }
}
type Mat = Vec<Vec<BigInt>>;
fn mm(a: &Mat, b: &Mat) -> Mat {
    a.iter()
        .map(|r| {
            (0..b[0].len())
                .map(|j| r.iter().enumerate().map(|(k, v)| v * &b[k][j]).sum())
                .collect()
        })
        .collect()
}
fn tr(a: &Mat) -> Mat {
    (0..a[0].len())
        .map(|i| a.iter().map(|r| r[i].clone()).collect())
        .collect()
}
fn mat(v: &Value) -> Mat {
    serde_json::from_value::<Vec<Vec<i64>>>(v.clone())
        .unwrap()
        .iter()
        .map(|r| r.iter().map(|v| BigInt::from(*v)).collect())
        .collect()
}
fn determinant(a: &Mat) -> BigInt {
    let n = a.len();
    let mut a = a.clone();
    let (mut old, mut sign) = (BigInt::from(1), 1i32);
    for k in 0..n - 1 {
        let p = (k..n).find(|i| a[*i][k] != BigInt::from(0));
        let Some(p) = p else {
            return BigInt::from(0);
        };
        if p != k {
            a.swap(p, k);
            sign = -sign;
        }
        let pivot = a[k][k].clone();
        for i in k + 1..n {
            for j in k + 1..n {
                let q = &a[i][j] * &pivot - &a[i][k] * &a[k][j];
                assert_eq!(&q % &old, BigInt::from(0));
                a[i][j] = q / &old;
            }
            a[i][k] = BigInt::from(0);
        }
        old = pivot;
    }
    BigInt::from(sign) * &a[n - 1][n - 1]
}
pub fn run(dir: &Path) {
    let q1 = read(&dir.join("q1-quotient-lattices.json"));
    let mut quotient_checks = 0;
    for r in q1["quotients"].as_array().unwrap() {
        let q = mat(&r["quotient_isogeny_matrix"]);
        let j = mat(&r["descended_alternating_form"]);
        let f = mat(&r["quotient_frobenius"]);
        assert_eq!(determinant(&j), BigInt::from(1));
        assert_eq!(
            determinant(&q),
            BigInt::from(r["isogeny_degree"].as_i64().unwrap())
        );
        assert_eq!(
            mm(&mm(&tr(&f), &j), &f),
            j.iter()
                .map(|r| r.iter().map(|v| v * 2).collect())
                .collect::<Mat>()
        );
        let pull = mm(&mm(&tr(&q), &j), &q);
        assert_eq!(pull[8][9], BigInt::from(r["ell"].as_i64().unwrap()));
        quotient_checks += 1;
    }
    let mut short_checks = 0;
    for r in q1["short_vectors"].as_array().unwrap() {
        let e = &r["enumeration"];
        let g: Vec<Vec<i128>> = serde_json::from_value(e["twice_gram"].clone()).unwrap();
        let det = e["gram_determinant"].as_i64().unwrap() as i128;
        let co: Vec<i128> =
            serde_json::from_value(e["inverse_diagonal_numerators"].clone()).unwrap();
        let big = mat(&e["twice_gram"]);
        assert_eq!(determinant(&big), BigInt::from(det));
        for n in 1..=8 {
            let principal: Mat = big[..n].iter().map(|r| r[..n].to_vec()).collect();
            assert!(determinant(&principal) > BigInt::from(0));
        }
        for (i, c) in co.iter().enumerate() {
            let minor: Mat = big
                .iter()
                .enumerate()
                .filter(|(r, _)| *r != i)
                .map(|(_, r)| {
                    r.iter()
                        .enumerate()
                        .filter(|(j, _)| *j != i)
                        .map(|(_, v)| v.clone())
                        .collect()
                })
                .collect();
            assert_eq!(determinant(&minor), BigInt::from(*c));
        }
        for c in co {
            assert!(6 * c < 4 * det);
            assert!(6 * c >= det);
        }
        let mut counts: BTreeMap<i128, usize> = BTreeMap::new();
        for mut code in 0..6561 {
            let mut z = vec![];
            for _ in 0..8 {
                z.push((code % 3) as i128 - 1);
                code /= 3;
            }
            let norm: i128 = (0..8)
                .map(|i| (0..8).map(|j| z[i] * g[i][j] * z[j]).sum::<i128>())
                .sum();
            assert_eq!(norm % 2, 0);
            if norm > 0 && norm <= 6 {
                *counts.entry(norm / 2).or_default() += 1;
            }
        }
        assert_eq!(serde_json::to_value(counts).unwrap(), e["counts_by_norm"]);
        short_checks += 1;
    }
    let q2 = read(&dir.join("q2-natural-quotient.json"));
    let modulus = q2["geometric_kernel_field"]["modulus"].as_u64().unwrap();
    let mut kernel_checks = 0;
    for r in q2["geometric_kernels"].as_array().unwrap() {
        let a = r["a"].as_u64().unwrap() as u32;
        let c = curve(24, modulus, a, 1);
        let ps: Vec<_> = r["kernel"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| point(&r["cofactor_point"], 24))
            .collect();
        for p in &ps {
            assert!(c.is_on_curve(p));
            assert_eq!(times(&c, p, 5), BinaryPoint::Infinity);
            for q in &ps {
                assert!(ps.contains(&binary_ecc::curve::point_add(&c, p, q)));
                kernel_checks += 1;
            }
        }
    }
    let m3 = read(&dir.join("m3b-paired-transport.json"));
    let modulus = m3["evaluation_field"]["modulus"].as_u64().unwrap();
    let embedding: Vec<u32> = serde_json::from_value(m3["embedding_F64"].clone()).unwrap();
    let ec = curve(30, modulus, 1, 1);
    let mut transport_checks = 0;
    for r in m3["records"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["status"] == "PASS")
    {
        let c = curve(
            30,
            modulus,
            embedding[r["underlying_a"].as_u64().unwrap() as usize],
            embedding[r["underlying_b"].as_u64().unwrap() as usize],
        );
        let p = point(&r["basis11"][0], 30);
        let q = point(&r["basis11"][1], 30);
        assert!(c.is_on_curve(&p) && c.is_on_curve(&q));
        assert_eq!(times(&c, &p, 11), BinaryPoint::Infinity);
        assert_eq!(times(&c, &q, 11), BinaryPoint::Infinity);
        assert!(!(0..11).any(|i| times(&c, &p, i) == q));
        let js = r["targets"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| point(p, 30))
            .collect::<Vec<_>>();
        assert!(js.iter().all(|p| ec.is_on_curve(p)));
        let src = match source(&r["source"]) {
            Pt::O => BinaryPoint::Infinity,
            Pt::A(x, y) => point(&json!([embedding[x as usize], embedding[y as usize]]), 30),
        };
        assert_eq!(sum(&ec, js.into_iter()), times(&ec, &src, 4));
        transport_checks += 1;
    }
    let data = read(&dir.join("m4-direct-modular-relations.json"));
    let cols = data["closed_points"].as_array().unwrap();
    let successor: Vec<usize> =
        serde_json::from_value(data["frobenius_successor"].clone()).unwrap();
    assert_eq!(successor.iter().collect::<BTreeSet<_>>().len(), 15840);
    let mut seen = BTreeSet::new();
    for c in cols {
        let members: Vec<usize> = serde_json::from_value(c["members"].clone()).unwrap();
        for i in 0..members.len() {
            assert!(seen.insert(members[i]));
            assert_eq!(successor[members[i]], members[(i + 1) % members.len()]);
        }
        assert_eq!(members.len() as u64, c["degree"].as_u64().unwrap());
    }
    assert_eq!(seen.len(), 15840);
    let mut images = vec![vec![]; 2];
    for (index, a) in [1, 0].iter().enumerate() {
        let c = curve(30, modulus, *a, 1);
        for col in cols {
            let p = point(&col["images"][index]["point"], 30);
            assert!(c.is_on_curve(&p));
            assert_eq!(times(&c, &p, 11), BinaryPoint::Infinity);
            images[index].push(p);
        }
    }
    let f = Field {
        m: 6,
        modulus: m3["source_field"]["modulus"].as_u64().unwrap(),
    };
    let base = Curve { f, a: 1, b: 1 };
    let mut checked = 0;
    let mut matrix = vec![];
    for r in data["principal_relations"].as_array().unwrap() {
        let params: Vec<u32> = serde_json::from_value(r["parameters"].clone()).unwrap();
        let roots = r["base_zeros"].as_array().unwrap();
        let mut multiplicity = BTreeMap::new();
        let mut total = 0;
        for root in roots {
            let p = source(&root["point"]);
            let m = root["multiplicity"].as_u64().unwrap();
            assert!(base.valid(p));
            let Pt::A(x, y) = p else {
                panic!("zero at infinity")
            };
            if params.len() == 2 {
                assert_eq!(y ^ f.mul(params[0], x) ^ params[1], 0);
            } else {
                assert_eq!(x, params[0]);
            }
            total += m;
            for j in 0..6 {
                *multiplicity.entry(base.frob(p, j)).or_insert(0u64) += m;
            }
        }
        assert_eq!(total, if params.len() == 2 { 3 } else { 2 });
        let row: Vec<u64> = serde_json::from_value(r["row_mod11"].clone()).unwrap();
        for (i, col) in cols.iter().enumerate() {
            let expected = multiplicity
                .get(&source(&col["source"]))
                .copied()
                .unwrap_or(0)
                % 11;
            assert_eq!(row[i], expected);
        }
        for (j, a) in [1, 0].iter().enumerate() {
            let c = curve(30, modulus, *a, 1);
            assert_eq!(
                sum(
                    &c,
                    row.iter().zip(&images[j]).map(|(n, p)| times(&c, p, *n))
                ),
                BinaryPoint::Infinity
            );
        }
        matrix.push(row.iter().map(|v| BigInt::from(*v)).collect());
        checked += 1;
    }
    let rank = round02::rank(matrix, &BigInt::from(11));
    assert_eq!(rank, 4);
    assert_eq!(data["rank_mod11"], rank);
    println!(
        "{}",
        json!({"status":"PASS","integer_quotient_checks":quotient_checks,"complete_short_vector_crosschecks":short_checks,"library_kernel_addition_checks":kernel_checks,"library_Hecke_and_basis_checks":transport_checks,"modular_orbits":cols.len(),"principal_rows_reconstructed":checked,"library_projected_point_checks":cols.len()*2,"independent_bigint_rank":rank,"scope":"separate arithmetic and relation checker; modular moduli/descent interpretation remains a written bridge"})
    );
}

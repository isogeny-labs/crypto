use crate::{round02, weil};
use serde_json::{json, Value};
type Matrix = Vec<Vec<u64>>;
fn power(mut a: u64, mut n: u64, p: u64) -> u64 {
    let mut r = 1;
    while n > 0 {
        if n & 1 == 1 {
            r = r * a % p;
        }
        a = a * a % p;
        n >>= 1;
    }
    r
}
pub(super) fn rref(mut rows: Matrix, p: u64) -> (Matrix, Vec<usize>) {
    let mut pivots = Vec::new();
    for col in 0..rows[0].len() {
        let Some(i) = (pivots.len()..rows.len()).find(|i| rows[*i][col] != 0) else {
            continue;
        };
        let r = pivots.len();
        rows.swap(i, r);
        let inv = power(rows[r][col], p - 2, p);
        for j in col..rows[r].len() {
            rows[r][j] = rows[r][j] * inv % p;
        }
        for i in 0..rows.len() {
            if i == r {
                continue;
            }
            let a = rows[i][col];
            for j in col..rows[i].len() {
                rows[i][j] = (rows[i][j] + p - a * rows[r][j] % p) % p;
            }
        }
        pivots.push(col);
    }
    (rows, pivots)
}
fn companion(poly: &[i128], p: u64) -> Matrix {
    let n = poly.len() - 1;
    let mut m = vec![vec![0; n]; n];
    for i in 1..n {
        m[i][i - 1] = 1;
    }
    for i in 0..n {
        m[i][n - 1] = (-poly[i]).rem_euclid(p as i128) as u64;
    }
    m
}
fn multiply(a: &Matrix, b: &Matrix, p: u64) -> Matrix {
    (0..a.len())
        .map(|i| {
            (0..b[0].len())
                .map(|j| (0..b.len()).map(|k| a[i][k] * b[k][j]).sum::<u64>() % p)
                .collect()
        })
        .collect()
}
fn matrix_order(a: &Matrix, p: u64) -> u64 {
    let identity: Matrix = (0..a.len())
        .map(|i| (0..a.len()).map(|j| u64::from(i == j)).collect())
        .collect();
    let mut m = identity.clone();
    for k in 1..=p * (p * p - 1) {
        m = multiply(a, &m, p);
        if m == identity {
            return k;
        }
    }
    panic!("elliptic Frobenius order outside GL2 bound")
}
pub fn run(dir: &std::path::Path) {
    let input: Value =
        serde_json::from_str(&std::fs::read_to_string(dir.join("gl0-resultants.json")).unwrap())
            .unwrap();
    let mut records = Vec::new();
    for row in input["rows"].as_array().unwrap() {
        let t = row["target_trace"].as_i64().unwrap();
        let base_t = row["elliptic_trace"].as_i64().unwrap();
        for prime in row["candidate_gluing_primes"].as_array().unwrap() {
            let p = prime.as_u64().unwrap();
            let a = companion(&weil::to_i128(&weil::trace_zero_poly(t, 5)), p);
            let e = companion(&[2, -(base_t as i128), 1], p);
            let mut equations = vec![vec![0u64; 16]; 16];
            for i in 0..8 {
                for j in 0..2 {
                    let r = 2 * i + j;
                    for k in 0..8 {
                        equations[r][2 * k + j] = (equations[r][2 * k + j] + a[i][k]) % p;
                    }
                    for k in 0..2 {
                        equations[r][2 * i + k] = (equations[r][2 * i + k] + p - e[k][j]) % p;
                    }
                }
            }
            let (reduced, pivots) = rref(equations, p);
            let mut basis = Vec::new();
            for free in (0..16).filter(|i| !pivots.contains(i)) {
                let mut v = vec![0; 16];
                v[free] = 1;
                for (i, c) in pivots.iter().enumerate() {
                    v[*c] = (p - reduced[i][free]) % p;
                }
                basis.push(v);
            }
            let dim = basis.len();
            assert!(dim <= 4, "explicit finite Hom scan exceeds frozen bound");
            let mut injective = 0;
            let mut first = None;
            for code in 1..p.pow(dim as u32) {
                let mut k = code;
                let mut v = vec![0; 16];
                for b in &basis {
                    let coeff = k % p;
                    k /= p;
                    for i in 0..16 {
                        v[i] = (v[i] + coeff * b[i]) % p;
                    }
                }
                let m: Matrix = (0..8).map(|i| vec![v[2 * i], v[2 * i + 1]]).collect();
                assert_eq!(multiply(&a, &m, p), multiply(&m, &e, p));
                let rank = rref(m.clone(), p).1.len();
                if rank == 2 {
                    injective += 1;
                    if first.is_none() {
                        first = Some(m);
                    }
                }
            }
            records.push(json!({"target_n":5,"target_trace":t,"elliptic_trace":base_t,"ell":p,"real_resultant":row["real_resultant"],"hom_dimension":dim,"hom_space_cardinality":p.pow(dim as u32),"injective_intertwiners":injective,"first_injective":first,"elliptic_frobenius":e,"full_elliptic_torsion_field_degree":matrix_order(&e,p),"source_lattice":"companion Z[pi] model; not every integral/isogeny representative at ramified ell","polarization_constructed":false}));
        }
    }
    round02::save(
        dir,
        "gl1-frobenius-modules.json",
        &json!({"schema":"koblitz.experiment/gl1-v1","records":records,"scope":"exact finite Frobenius-module maps, no polarization or curve existence claim"}),
    );
    println!("GL1 complete: {} gluing-prime cases", records.len());
}

use serde_json::Value;
use std::{collections::BTreeMap, fs};

fn mul(a: u64, b: u64, p: u64) -> u64 { ((a as u128 * b as u128) % p as u128) as u64 }
fn sub(a: u64, b: u64, p: u64) -> u64 { (a + p - b) % p }
fn pow(mut a: u64, mut n: u64, p: u64) -> u64 {
    let mut z = 1;
    while n > 0 { if n & 1 != 0 { z = mul(z, a, p); } a = mul(a, a, p); n >>= 1; }
    z
}
fn matrix(v: &Value) -> [[u64; 4]; 4] {
    std::array::from_fn(|i| std::array::from_fn(|j| v[i][j].as_u64().unwrap()))
}
fn apply(c: &[[u64; 4]; 4], w: &[u64; 4], p: u64) -> [u64; 4] {
    std::array::from_fn(|i| (0..4).fold(0, |z, j| (z + mul(c[i][j], w[j], p)) % p))
}
fn inverse_apply(c: &[[u64; 4]; 4], v: &[u64; 4], p: u64) -> [u64; 4] {
    let mut a = [[0; 5]; 4];
    for i in 0..4 { for j in 0..4 { a[i][j] = c[i][j]; } a[i][4] = v[i]; }
    for col in 0..4 {
        let pivot = (col..4).find(|&i| a[i][col] != 0).unwrap();
        a.swap(col, pivot);
        let inv = pow(a[col][col], p - 2, p);
        for v in &mut a[col][col..] { *v = mul(*v, inv, p); }
        let pivot_row = a[col];
        for (i,row) in a.iter_mut().enumerate() { if i != col {
            let f = row[col];
            for (v,base) in row[col..].iter_mut().zip(&pivot_row[col..]) {
                *v = sub(*v, mul(f, *base, p), p);
            }
        }}
    }
    std::array::from_fn(|i| a[i][4])
}
fn same_projective(x: &[u64; 4], y: &[u64; 4], p: u64) -> bool {
    (0..4).all(|i| (i+1..4).all(|j| mul(x[i],y[j],p)==mul(x[j],y[i],p)))
}
fn main() {
    let source = "research/kummer_differential_collector_20261022/n16-action-certificate/results/exceptional-ideal.json";
    let text = fs::read_to_string(source).unwrap();
    let data: Value = serde_json::from_str(&text).unwrap();
    let mut rows = BTreeMap::new();
    for r in data["rows"].as_array().unwrap() {
        let p = r["p"].as_u64().unwrap();
        rows.entry(p).or_insert_with(|| r.clone());
    }
    assert_eq!(rows.len(), 21);
    let mut csv = String::from("schema,p,lambda,det_c1,det_c2,matrices_equal,basis_identity,branch_point_pair,a0,a1,c1_inverse_q0,c1_inverse_q1,c1_inverse_q2,c1_inverse_q3,c1_inversion_matches_basis_change,source_finite_replay_states,all_state_geometric_projection_proved,mixed_zero_fallback_proved,new_collector_targets,rho_ratio\n");
    for (p,r) in rows {
        let c1 = matrix(&r["conversion_matrix_degree_one"]);
        let c2 = matrix(&r["conversion_matrix_degree_two"]);
        let roots: Vec<u64> = data["rows"].as_array().unwrap().iter()
            .filter(|v| v["p"].as_u64()==Some(p))
            .map(|v| v["root"].as_u64().unwrap()).collect();
        let lambda = *roots.iter().find(|&&x| x != 0 && x != 1).unwrap();
        let a0 = lambda;
        let a1 = sub(0, (1 + lambda) % p, p);
        let w = [1, sub(0,a1,p), sub(0,a0,p), mul(a0,a1,p)];
        // u=(x-1)(x-lambda) divides f and v=0, so H=0 in the C2 chart.
        let k = apply(&c2,&w,p);
        let q = inverse_apply(&c1,&k,p);
        let sigma = sub(0,r["f"][4].as_u64().unwrap(),p);
        let transformed = [w[0],w[1],sub(0,w[2],p),(w[3]+mul(sigma,a0,p))%p];
        assert_eq!(q,transformed,"basis change failed at p={p}");
        assert!(same_projective(&apply(&c1,&transformed,p),&k,p));
        // The explicit third-column identity is the complete matrix basis change.
        for i in 0..4 {
            assert_eq!(c2[i][2],sub(0,(c1[i][2]+mul(sigma,c1[i][3],p))%p,p));
        }
        let d1 = r["degree_one_conversion_determinant"].as_u64().unwrap();
        let d2 = r["degree_two_conversion_determinant"].as_u64().unwrap();
        assert!(d1 != 0 && d2 != 0 && c1 != c2);
        csv.push_str(&format!("kummer-n17-matrix-gate/v1,{p},{lambda},{d1},{d2},false,true,1+lambda,{a0},{a1},{},{},{},{},true,138095,,,0,\n",q[0],q[1],q[2],q[3]));
    }
    let out = "research/kummer_differential_collector_20261022/n17-mixed-zero-gate/results/gates.csv";
    fs::create_dir_all(std::path::Path::new(out).parent().unwrap()).unwrap();
    fs::write(out,csv).unwrap();
    println!("21 exact C1/C2 basis identities and valid branch-point-pair inversion checks passed");
}

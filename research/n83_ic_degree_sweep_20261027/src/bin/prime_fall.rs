//! Proof-assisted standard first-fall certificates. See fall/prime/ARGUMENT.md.
//! Does not identify the earliest NEW affine fall or last fall.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
type Mon = Vec<usize>;
type Poly = BTreeMap<Mon, u64>;
const Q: u64 = 65537;
fn sha(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn read(p: &Path) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: &Path, v: &Value) {
    fs::write(p, serde_json::to_string_pretty(v).unwrap() + "\n").unwrap()
}
fn add(a: &Poly, b: &Poly, c: u64) -> Poly {
    let mut r = a.clone();
    for (m, v) in b {
        let x = r.entry(m.clone()).or_default();
        *x = (*x + c * v) % Q;
    }
    r.retain(|_, v| *v != 0);
    r
}
fn mul(a: &Poly, b: &Poly, nb: usize) -> Poly {
    let mut r = Poly::new();
    for (x, c) in a {
        for (y, d) in b {
            let mut m = x.clone();
            m.extend(y);
            m.sort_unstable();
            if m.windows(2).any(|w| w[0] == w[1] && w[0] < nb) {
                continue;
            }
            let z = r.entry(m).or_default();
            *z = (*z + c * d) % Q;
        }
    }
    r.retain(|_, v| *v != 0);
    r
}
fn var(i: usize) -> Poly {
    [(vec![i], 1)].into()
}
fn lin(i: usize, ell: usize) -> Poly {
    (0..ell).map(|j| (vec![i * ell + j], 1u64 << j)).collect()
}
fn sq(x: &Poly, nb: usize) -> Poly {
    mul(x, x, nb)
}
fn h(x: &Poly, y: &Poly, z: &Poly, nb: usize) -> Poly {
    let xy = mul(x, y, nb);
    let xm = add(x, y, Q - 1);
    let xp = add(x, y, 1);
    let a = mul(&sq(&xm, nb), &sq(z, nb), nb);
    let b = mul(&mul(&xp, &xy, nb), z, nb);
    add(&add(&a, &b, Q - 2), &sq(&xy, nb), 1)
}
fn mons(n: usize, nb: usize, d: usize, start: usize, cur: &mut Mon, out: &mut Vec<Mon>) {
    if d == 0 {
        out.push(cur.clone());
        return;
    }
    for i in start..n {
        cur.push(i);
        mons(n, nb, d - 1, if i < nb { i + 1 } else { i }, cur, out);
        cur.pop();
    }
}
fn inv(mut x: u64) -> u64 {
    let mut n = Q - 2;
    let mut a = 1;
    while n > 0 {
        if n & 1 != 0 {
            a = a * x % Q;
        }
        x = x * x % Q;
        n >>= 1;
    }
    a
}
fn matrix_rank(rows: Vec<Poly>) -> usize {
    let mut piv = BTreeMap::<Mon, Poly>::new();
    for mut row in rows {
        while let Some((at, &coef)) = row.last_key_value() {
            let at = at.clone();
            if let Some(p) = piv.get(&at) {
                row = add(&row, p, Q - coef);
            } else {
                let inverse = inv(coef);
                for c in row.values_mut() {
                    *c = *c * inverse % Q;
                }
                piv.insert(at, row);
                break;
            }
        }
    }
    piv.len()
}
fn small_matrix(ell: usize, m: usize) -> Value {
    let nb = m * ell;
    let nv = nb + m - 2;
    let mut forms = Vec::new();
    for k in 0..m - 1 {
        let x = if k == 0 { lin(0, ell) } else { var(nb + k - 1) };
        let y = lin(k + 1, ell);
        forms.push(if k == m - 2 {
            mul(&sq(&x, nb), &sq(&y, nb), nb)
        } else {
            h(&x, &y, &var(nb + k), nb)
        });
    }
    let expected = ell / 2 + 4;
    assert!(
        expected < 8,
        "These controls have no quartic Koszul boundaries below degree8"
    );
    let mut records = Vec::new();
    let mut first = None;
    for degree in 4..=expected {
        let mut mm = Vec::new();
        mons(nv, nb, degree - 4, 0, &mut Vec::new(), &mut mm);
        let rows: Vec<_> = forms
            .iter()
            .flat_map(|f| mm.iter().map(|m| mul(f, &[(m.clone(), 1)].into(), nb)))
            .collect();
        let count = rows.len();
        let rank = matrix_rank(rows);
        if rank < count && first.is_none() {
            first = Some(degree);
        }
        records.push(json!({"degree":degree,"rows":count,"rank":rank,"kernel_dimension":count-rank,"koszul_boundary_dimension":0}));
    }
    assert_eq!(first, Some(expected));
    json!({"ell":ell,"m":m,"prime":Q,"measured_first_nontrivial_syzygy":first,"predicted":expected,"matrices":records})
}
fn choose(n: usize, k: usize) -> u128 {
    if k > n {
        return 0;
    }
    let mut a = 1;
    for j in 0..k.min(n - k) {
        a = a * (n - j) as u128 / (j + 1) as u128;
    }
    a
}
fn mm(mut a: u128, mut b: u128, p: u128) -> u128 {
    let mut r = 0;
    a %= p;
    while b > 0 {
        if b & 1 != 0 {
            r = (r + a) % p;
        }
        a = (a + a) % p;
        b >>= 1;
    }
    r
}
fn sub(a: u128, b: u128, p: u128) -> u128 {
    (a + p - b) % p
}
fn s3(x: u128, y: u128, z: u128, a: u128, b: u128, p: u128) -> u128 {
    let xy = mm(x, y, p);
    let delta = sub(x, y, p);
    let first = mm(mm(delta, delta, p), mm(z, z, p), p);
    let mid = mm(2, mm((x + y) % p, (xy + a) % p, p) + (2 * b) % p, p);
    let last = mm(sub(xy, a, p), sub(xy, a, p), p);
    sub(
        (sub(first, mm(mid, z, p), p) + last) % p,
        mm(4 * b, (x + y) % p, p),
        p,
    )
}
fn certificate(i: &Value) -> Value {
    let ell = i["ell"].as_u64().unwrap() as usize;
    let m = i["m"].as_u64().unwrap() as usize;
    assert_eq!(ell, 83 / m);
    assert!((2..=9).contains(&m));
    let p: u128 = i["field_modulus"].as_str().unwrap().parse().unwrap();
    let t: u128 = i["target_x"].as_str().unwrap().parse().unwrap();
    assert!(p > 83 && p > (1u128 << ell));
    let h = ell / 2;
    let ranks:Vec<_>=(0..=h).map(|r|{let domain=choose(ell,r);let codomain=choose(ell,r+2);let rank=domain.min(codomain);assert_eq!(domain>rank,r==h);json!({"input_degree":r,"domain_dimension":domain.to_string(),"codomain_dimension":codomain.to_string(),"theorem_rank":rank.to_string(),"kernel_dimension":(domain-rank).to_string()})}).collect();
    let pairs:Vec<_>=(0..h).map(|j|{let a=var(2*j);let b=var(2*j+1);assert!(mul(&add(&a,&b,Q-1),&add(&a,&b,1),ell).is_empty());json!({"normalized_variables":[2*j,2*j+1],"bit_coefficients":[(1u128<<(2*j)).to_string(),(1u128<<(2*j+1)).to_string()],"identity":"(y-z)(y+z)=y^2-z^2=0"})}).collect();
    // The two choices differ by one, and lie in the interval strictly below p.
    let mut l = (0..h).map(|j| 1u128 << (2 * j)).sum::<u128>();
    let mut choose_odd = false;
    if l == t {
        l += 1;
        choose_odd = true;
    }
    assert_ne!(l, t);
    let mut g = 1;
    for j in 0..h {
        let v = if j == 0 && choose_odd {
            sub(0, 2, p)
        } else {
            1u128 << (2 * j)
        };
        g = mm(g, v, p);
    }
    assert_ne!(g, 0);
    let coeff = mm(g, mm(sub(l, t, p), sub(l, t, p), p), p);
    assert_ne!(coeff, 0);
    let a: i128 = i["curve_a"].as_str().unwrap().parse().unwrap();
    let b: i128 = i["curve_b"].as_str().unwrap().parse().unwrap();
    assert!(a >= 0 && b >= 0);
    let affine = if m == 2 {
        let (x, value) = (0..=2u128)
            .map(|x| (x, mm(g, s3(x, l, t, a as u128, b as u128, p), p)))
            .find(|(_, v)| *v != 0)
            .unwrap();
        json!({"x0":x.to_string(),"last_x":l.to_string(),"g_value":g.to_string(),"product_value":value.to_string(),"nonzero":true})
    } else {
        json!({"last_x":l.to_string(),"g_value":g.to_string(),"intermediate_square_coefficient":coeff.to_string(),"nonzero":true})
    };
    let factors: Vec<_> = (0..m - 1)
        .map(|k| {
            if k == 0 {
                json!(["L0", "L1"])
            } else {
                json!([format!("U{}", k - 1), format!("L{}", k + 1)])
            }
        })
        .collect();
    let mut allocated = std::collections::BTreeSet::new();
    for factor in &factors {
        for v in factor.as_array().unwrap() {
            assert!(allocated.insert(v.as_str().unwrap()));
        }
    }
    json!({"schema":"prime-leading-first-fall.certificate/v1","input":i,"first_fall_degree":h+4,"definition":"least nontrivial leading-form syzygy modulo Koszul; Caminata-Gorla Definition1.3","first_new_affine_fall_degree":null,"last_fall_degree":null,"lower_bound_method":"Lefschetz theorem plus finite filtered Koszul complex; see ARGUMENT.md","lower_bound_is_full_matrix_measurement":false,"lefschetz_rank_certificates":ranks,"degenerate_quartic_disjoint_factors":factors,"upper_bound_witness":{"quartic_index":m-1,"multiplier_degree":h,"normalized_variable":"y_j=2^j*b_last,j","multiplier":"product over the displayed pairs of (y_even-y_odd)","pairs":pairs,"unpaired_variables":ell-2*h,"annihilates_last_L_squared":true,"nontriviality":"Earlier blocks and intermediates specialize to zero; other quartics vanish but multiplier survives"},"affine_nonzero_product":affine,"affine_product_degree_upper_bound":h+3,"affine_product_novelty_below_first_fall":"not established; do not identify with earliest new affine fall"})
}
fn selfcheck() -> Value {
    let matrices: Vec<_> = [(2, 4), (3, 3), (4, 3), (5, 2), (6, 2)]
        .into_iter()
        .map(|(e, m)| small_matrix(e, m))
        .collect();
    json!({"exact_small_matrix_controls":matrices,"all_passed":true})
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("selfcheck") => println!("{}", selfcheck()),
        Some("run") => {
            let old = read(Path::new(&a[2]));
            let root = Path::new(&a[3]);
            let out = Path::new(&a[4]);
            fs::create_dir(out).unwrap();
            let mut results = Vec::new();
            for r in old
                .as_array()
                .unwrap()
                .iter()
                .filter(|r| r["input"]["family"] == "prime")
            {
                let i = &r["input"];
                let id = i["id"].as_str().unwrap();
                assert_eq!(r["status"], "certified_no_fall_through_degree_4");
                let raw = fs::read(root.join(id).join("system.sing")).unwrap();
                assert_eq!(sha(&raw), i["source_sha256"]);
                let old_cert = out.join(format!("{id}-r3.json"));
                save(&old_cert, r);
                let verifier = std::env::var("R3_VERIFIER")
                    .expect("R3_VERIFIER must name the pinned presentation verifier");
                assert!(
                    std::process::Command::new(verifier)
                        .arg("replay")
                        .arg(root)
                        .arg(&old_cert)
                        .status()
                        .unwrap()
                        .success(),
                    "Original presentation replay failed"
                );
                let c = certificate(i);
                save(&out.join(format!("{id}.json")), &c);
                results.push(c);
            }
            assert_eq!(results.len(), 16);
            save(&out.join("results.json"), &json!(results));
            println!("16 proof-assisted leading-syzygy first-fall certificates; earliest new affine fall and last fall remain unresolved");
        }
        Some("replay") => {
            let c = read(Path::new(&a[2]));
            assert_eq!(c, certificate(&c["input"]));
            println!(
                "{}",
                json!({"input":c["input"]["id"],"replayed":true,"independent_review":false})
            );
        }
        _ => panic!("selfcheck | run R3_RESULTS INPUTS NEW_OUTPUT | replay CERTIFICATE"),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matrix_controls() {
        selfcheck();
    }
    #[test]
    fn arithmetic() {
        let p = (1u128 << 83) - 21;
        assert_eq!(mm(p - 1, p - 1, p), 1);
        assert_eq!(mm(0, p - 1, p), 0);
    }
    #[test]
    fn pair_zero_identity() {
        let x = var(0);
        let y = var(1);
        assert!(mul(&add(&x, &y, Q - 1), &add(&x, &y, 1), 2).is_empty());
        assert!(!mul(&add(&x, &y, Q - 1), &add(&x, &y, Q - 1), 2).is_empty());
    }
}

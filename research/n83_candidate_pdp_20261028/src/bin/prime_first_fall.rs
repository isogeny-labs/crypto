//! Exact compact prime m2 witnesses; see PRIME_FIRST_FALL_DESIGN.md.
#[allow(dead_code)]
#[path = "../prime_first_fall_donor.rs"]
mod donor;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, VecDeque},
    fs,
    path::Path,
};
type Poly = BTreeMap<u64, u128>;
fn sha(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn read(p: &Path) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: &Path, v: &Value) {
    fs::write(p, serde_json::to_string_pretty(v).unwrap() + "\n").unwrap();
}
fn add(a: u128, b: u128, p: u128) -> u128 {
    assert!(a < p && b < p && p < (1u128 << 127));
    (a + b) % p
}
fn neg(a: u128, p: u128) -> u128 {
    if a == 0 {
        0
    } else {
        p - a
    }
}
fn mul(a: u128, b: u128, p: u128) -> u128 {
    donor::modular_mul(a, b, p)
}
fn accum(a: &mut Poly, m: u64, c: u128, p: u128) {
    let v = add(*a.get(&m).unwrap_or(&0), c, p);
    if v == 0 {
        a.remove(&m);
    } else {
        a.insert(m, v);
    }
}
fn degree(a: &Poly) -> usize {
    a.keys().map(|m| m.count_ones() as usize).max().unwrap_or(0)
}
fn scaled_add(a: &mut Poly, b: &Poly, c: u128, p: u128) {
    for (&m, &v) in b {
        accum(a, m, mul(v, c, p), p);
    }
}
fn constant(c: u128) -> Poly {
    if c == 0 {
        Poly::new()
    } else {
        [(0, c)].into()
    }
}
fn linear(n: usize) -> Poly {
    (0..n).map(|i| (1u64 << i, 1)).collect()
}
fn multiply_x(a: &Poly, w: &[u128], p: u128) -> Poly {
    let mut out = Poly::new();
    for (&m, &c) in a {
        for (j, &weight) in w.iter().enumerate() {
            let bit = 1u64 << j;
            accum(
                &mut out,
                m | bit,
                if m & bit != 0 { mul(c, weight, p) } else { c },
                p,
            );
        }
    }
    out
}
// Independent general polynomial multiplication for controls and small B factors.
fn product(a: &Poly, b: &Poly, w: &[u128], p: u128) -> Poly {
    let mut out = Poly::new();
    for (&m, &c) in a {
        for (&n, &d) in b {
            let mut coefficient = mul(c, d, p);
            let mut repeats = m & n;
            while repeats != 0 {
                let j = repeats.trailing_zeros() as usize;
                coefficient = mul(coefficient, w[j], p);
                repeats &= repeats - 1;
            }
            accum(&mut out, m | n, coefficient, p);
        }
    }
    out
}
fn tensor(a: &Poly, b: &Poly, n: usize, p: u128) -> Poly {
    let mut out = Poly::new();
    for (&m, &c) in a {
        for (&k, &d) in b {
            accum(&mut out, m | (k << n), mul(c, d, p), p);
        }
    }
    out
}
fn h_poly(n: usize, p: u128) -> Poly {
    assert!(n >= 3 && n % 2 == 1 && n <= 27 && p > n as u128 && p < (1u128 << 127));
    let mut h = constant(1);
    for j in 0..n / 2 {
        let mut next = Poly::new();
        for (&m, &c) in &h {
            accum(&mut next, m | (1 << (2 * j)), c, p);
            accum(&mut next, m | (1 << (2 * j + 1)), neg(c, p), p);
        }
        h = next;
    }
    h
}
fn pairing(h: &Poly, a: &Poly, n: usize, p: u128) -> u128 {
    let all = (1u64 << n) - 1;
    h.iter().fold(0, |sum, (&m, &c)| {
        add(sum, mul(c, *a.get(&(all ^ m)).unwrap_or(&0), p), p)
    })
}
fn factors(w: &[u128], a: u128, b: u128, t: u128, p: u128) -> (Vec<Poly>, Vec<Poly>) {
    assert!(w.iter().all(|x| *x > 0 && *x < p));
    assert!(a < p && b < p && t < p);
    let h = h_poly(w.len(), p);
    let hx = multiply_x(&h, w, p);
    let hxx = multiply_x(&hx, w, p);
    let y = linear(w.len());
    let yy = product(&y, &y, w, p);
    let tt = mul(t, t, p);
    let linear_c = neg(add(mul(2, mul(a, t, p), p), mul(4, b, p), p), p);
    let mut b2 = yy.clone();
    scaled_add(&mut b2, &y, neg(mul(2, t, p), p), p);
    scaled_add(&mut b2, &constant(1), tt, p);
    let mut b1 = Poly::new();
    scaled_add(&mut b1, &yy, neg(mul(2, t, p), p), p);
    scaled_add(&mut b1, &y, neg(mul(2, add(tt, a, p), p), p), p);
    scaled_add(&mut b1, &constant(1), linear_c, p);
    let mut b0 = Poly::new();
    scaled_add(&mut b0, &yy, tt, p);
    scaled_add(&mut b0, &y, linear_c, p);
    scaled_add(
        &mut b0,
        &constant(1),
        add(mul(a, a, p), neg(mul(4, mul(b, t, p), p), p), p),
        p,
    );
    (vec![h, hx, hxx], vec![b0, b1, b2])
}
fn certify(w: &[u128], a: u128, b: u128, t: u128, p: u128) -> (Value, Vec<Poly>, Vec<Poly>) {
    let n = w.len();
    let r = n / 2;
    let d = r + 4;
    assert!((d as u128) < p);
    let (aa, bb) = factors(w, a, b, t, p);
    assert_eq!(aa[0].len(), 1 << r);
    assert_eq!(degree(&aa[0]), r);
    assert!(degree(&aa[1]) <= r + 1 && degree(&aa[2]) <= r + 1);
    let q1 = pairing(&aa[0], &aa[1], n, p);
    let q2 = pairing(&aa[0], &aa[2], n, p);
    let sign = donor::modular_pow(p - 2, r as u128, p);
    let sum = w.iter().fold(0, |s, &v| add(s, v, p));
    assert_eq!(q1, sign);
    assert_eq!(q2, mul(sign, sum, p));
    let direct = (0..3).fold(0, |s, i| {
        add(
            s,
            mul(
                pairing(&aa[0], &aa[i], n, p),
                *bb[i].get(&3).unwrap_or(&0),
                p,
            ),
            p,
        )
    });
    let delta = add(sum, neg(mul(2, t, p), p), p);
    let expected = mul(2, mul(sign, delta, p), p);
    assert_eq!(direct, expected);
    let mut ranks = vec![];
    for k in 0..r {
        let rank = donor::rank(n, k + 2, p);
        assert_eq!(rank["rank"], rank["source_dimension"]);
        ranks.push(rank);
    }
    let first = donor::rank(n, r + 2, p);
    assert!(first["rank"].as_u64().unwrap() < first["source_dimension"].as_u64().unwrap());
    let strict = direct != 0;
    let cert = json!({"schema":"n83.prime-first-fall-algebra/v1","ell":n,"r":r,"field_modulus":p.to_string(),
        "first_fall_degree":d,"first_fall_definition":"Nontrivial highest-form syzygy modulo Koszul and field-characteristic trivial syzygies; Definition1.3 Caminata-Gorla",
        "injective_multiplier_degrees":ranks,"first_noninjective_rank":first,
        "weights":w.iter().map(|x|x.to_string()).collect::<Vec<_>>(),"weight_sum":sum.to_string(),"target_x":t.to_string(),
        "curve_a":a.to_string(),"curve_b":b.to_string(),"pairing_h_hx":q1.to_string(),"pairing_h_hxx":q2.to_string(),
        "pairing_direct":direct.to_string(),"pairing_formula":expected.to_string(),"exceptional_delta":delta.to_string(),
        "strict_filtered_fall_verified":strict,"first_strict_filtered_fall_degree":if strict {Some(d)} else {None},
        "witness_degree_upper":d-1,"witness_degree_exact":if strict {Some(d-1)} else {None},
        "last_fall_lower_bound":if strict {Some(d)} else {None},"last_fall_upper_bound":null,"last_fall_degree":null,
        "A_factor_terms":aa.iter().map(|x|x.len()).collect::<Vec<_>>(),"A_factor_degrees":aa.iter().map(degree).collect::<Vec<_>>(),
        "B_factor_terms":bb.iter().map(|x|x.len()).collect::<Vec<_>>(),
        "exception_rule":"Zero pairing leaves this strict affine witness inconclusive; highest-form FFD is still certified. No global upper or exact LFD.",
        "proof_scope":"Source-bound native arithmetic and sparse witnesses plus written rank/Koszul/filtered-space argument; independent scientific review and full algebraic formalization pending"});
    (cert, aa, bb)
}
fn bytes(a: &Poly, n: usize, p: u128) -> Vec<u8> {
    let mut b = b"N83PFF1\n".to_vec();
    b.extend((n as u32).to_le_bytes());
    b.extend((degree(a) as u32).to_le_bytes());
    b.extend((a.len() as u64).to_le_bytes());
    b.extend(p.to_le_bytes());
    for (&m, &c) in a {
        assert!(m < (1 << n) && c > 0 && c < p);
        b.extend(m.to_le_bytes());
        b.extend(c.to_le_bytes());
    }
    b
}
fn parse(b: &[u8]) -> (usize, u128, Poly) {
    assert!(b.len() >= 40);
    assert_eq!(&b[..8], b"N83PFF1\n");
    let n = u32::from_le_bytes(b[8..12].try_into().unwrap()) as usize;
    let deg = u32::from_le_bytes(b[12..16].try_into().unwrap()) as usize;
    let count = u64::from_le_bytes(b[16..24].try_into().unwrap()) as usize;
    let p = u128::from_le_bytes(b[24..40].try_into().unwrap());
    assert!(n <= 27 && p > n as u128 && p < (1 << 127));
    assert_eq!(b.len(), 40 + 24 * count);
    let mut a = Poly::new();
    let mut prior = None;
    for row in b[40..].chunks_exact(24) {
        let m = u64::from_le_bytes(row[..8].try_into().unwrap());
        let c = u128::from_le_bytes(row[8..].try_into().unwrap());
        assert!(m < (1 << n) && c > 0 && c < p && prior.map_or(true, |v| m > v));
        prior = Some(m);
        a.insert(m, c);
    }
    assert_eq!(degree(&a), deg);
    (n, p, a)
}
fn source_f(n: usize, w: &[u128], a: u128, b: u128, t: u128, p: u128) -> Poly {
    let ww: Vec<_> = w.iter().chain(w).copied().collect();
    let x = linear(n);
    let y: Poly = linear(n).into_iter().map(|(m, c)| (m << n, c)).collect();
    let mut xm = x.clone();
    scaled_add(&mut xm, &y, p - 1, p);
    let mut xp = x.clone();
    scaled_add(&mut xp, &y, 1, p);
    let xy = product(&x, &y, &ww, p);
    let mut inner = xy.clone();
    scaled_add(&mut inner, &constant(1), a, p);
    let mut term = product(&xp, &inner, &ww, p);
    scaled_add(&mut term, &constant(1), mul(2, b, p), p);
    let mut out = product(&xm, &xm, &ww, p);
    for v in out.values_mut() {
        *v = mul(*v, mul(t, t, p), p);
    }
    out.retain(|_, v| *v != 0);
    scaled_add(&mut out, &term, neg(mul(2, t, p), p), p);
    let mut minus = xy;
    scaled_add(&mut minus, &constant(1), neg(a, p), p);
    scaled_add(&mut out, &product(&minus, &minus, &ww, p), 1, p);
    scaled_add(&mut out, &xp, neg(mul(4, b, p), p), p);
    out
}
fn expanded(aa: &[Poly], bb: &[Poly], n: usize, p: u128) -> Poly {
    let mut out = Poly::new();
    for i in 0..3 {
        scaled_add(&mut out, &tensor(&aa[i], &bb[i], n, p), 1, p);
    }
    out
}
// Independent dense closure in the small Boolean quotient, graded pivots high to low.
fn dense_reduce(row: &mut [u128], basis: &BTreeMap<usize, Vec<u128>>, p: u128) {
    for (&pivot, b) in basis.iter().rev() {
        let c = row[pivot];
        if c != 0 {
            for j in 0..=pivot {
                row[j] = add(row[j], neg(mul(c, b[j], p), p), p);
            }
        }
    }
}
fn dense_space(f: &Poly, w: &[u128], d: usize, p: u128) -> (Vec<u64>, BTreeMap<usize, Vec<u128>>) {
    assert!(w.len() <= 6);
    let mut masks: Vec<_> = (0..(1u64 << w.len()))
        .filter(|m| m.count_ones() as usize <= d)
        .collect();
    masks.sort_by_key(|m| (m.count_ones(), *m));
    let index: BTreeMap<_, _> = masks.iter().enumerate().map(|(i, &m)| (m, i)).collect();
    let mut basis = BTreeMap::new();
    let mut queue = VecDeque::from([f.clone()]);
    while let Some(poly) = queue.pop_front() {
        assert!(degree(&poly) <= d);
        let mut row = vec![0; masks.len()];
        for (m, c) in poly {
            row[index[&m]] = c;
        }
        dense_reduce(&mut row, &basis, p);
        let Some(pivot) = row.iter().rposition(|&v| v != 0) else {
            continue;
        };
        let inv = donor::modular_pow(row[pivot], p - 2, p);
        for v in &mut row {
            *v = mul(*v, inv, p);
        }
        let poly: Poly = masks
            .iter()
            .zip(&row)
            .filter(|(_, c)| **c != 0)
            .map(|(&m, &c)| (m, c))
            .collect();
        if degree(&poly) < d {
            for j in 0..w.len() {
                let v: Poly = [(1 << j, 1)].into();
                queue.push_back(product(&poly, &v, w, p));
            }
        }
        basis.insert(pivot, row);
    }
    (masks, basis)
}
fn dense_member(f: &Poly, space: &(Vec<u64>, BTreeMap<usize, Vec<u128>>), p: u128) -> bool {
    let (masks, basis) = space;
    let index: BTreeMap<_, _> = masks.iter().enumerate().map(|(i, &m)| (m, i)).collect();
    let mut row = vec![0; masks.len()];
    for (&m, &c) in f {
        let Some(&i) = index.get(&m) else {
            return false;
        };
        row[i] = c;
    }
    dense_reduce(&mut row, basis, p);
    row.iter().all(|&x| x == 0)
}
fn controls() -> Value {
    let p = 101;
    let mut rows = vec![];
    for n in [3, 5, 7] {
        for mode in 0..2 {
            let w: Vec<_> = (0..n)
                .map(|i| {
                    if mode == 0 {
                        (i + 1) as u128
                    } else {
                        (1u128 << i) % p
                    }
                })
                .collect();
            let sum = w.iter().sum::<u128>() % p;
            let critical = mul(sum, 51, p);
            for (a, b) in [(2, 3), (7, 11)] {
                for t in [0, 9, critical] {
                    let (cert, aa, bb) = certify(&w, a, b, t, p);
                    let f = source_f(n, &w, a, b, t, p);
                    let ww: Vec<_> = w.iter().chain(&w).copied().collect();
                    let direct = product(&aa[0], &f, &ww, p);
                    let compact = expanded(&aa, &bb, n, p);
                    assert_eq!(direct, compact);
                    assert!(degree(&direct) <= n / 2 + 3);
                    let strict = cert["strict_filtered_fall_verified"].as_bool().unwrap();
                    assert_eq!(strict, t != critical);
                    let mut closed = Value::Null;
                    if n == 3 {
                        let low = dense_space(&f, &ww, n / 2 + 3, p);
                        let high = dense_space(&f, &ww, n / 2 + 4, p);
                        assert!(dense_member(&direct, &high, p));
                        if strict {
                            assert!(!dense_member(&direct, &low, p));
                        }
                        closed = json!({"V_before_rank":low.1.len(),"V_at_rank":high.1.len(),"witness_in_before":dense_member(&direct,&low,p),"witness_in_at":true});
                    }
                    rows.push(json!({"ell":n,"weights":w.iter().map(|x|x.to_string()).collect::<Vec<_>>(),"a":a,"b":b,"t":t,"exceptional":t==critical,"expanded_equality":true,"direct_pairing":cert["pairing_direct"],"dense_closure":closed}));
                }
            }
        }
    }
    let rank_controls = donor::independent_controls();
    assert_eq!(rank_controls["all_passed"], true);
    json!({"schema":"n83.prime-first-fall-controls/v1","rows":rows,"rank_controls":rank_controls,"all_passed":true})
}
fn run(root: &Path, out: &Path, replay: bool) {
    let protocol = read(&root.join("protocol-prime-first-fall.json"));
    let cases = protocol["extensions"]["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 6);
    if !replay {
        fs::create_dir(out).unwrap();
    }
    let primality = donor::primality();
    if replay {
        assert_eq!(read(&out.join("primality.json")), primality);
    } else {
        save(&out.join("primality.json"), &primality);
    }
    let p = donor::field();
    let mut rows = vec![];
    for case in cases {
        let key = case["key"].as_str().unwrap();
        let input_dir = root.join("inputs").join(key);
        let input_bytes = fs::read(input_dir.join("input.json")).unwrap();
        assert_eq!(sha(&input_bytes), case["input_sha256"]);
        let input: Value = serde_json::from_slice(&input_bytes).unwrap();
        let text = fs::read_to_string(input_dir.join("system.sing")).unwrap();
        assert_eq!(sha(text.as_bytes()), case["source_sha256"]);
        let source = donor::source(&input, &text, case["arm"].as_str().unwrap());
        let weights: Vec<_> = source["weights"].as_array().unwrap()[..27]
            .iter()
            .map(|x| x["weight"].as_str().unwrap().parse::<u128>().unwrap())
            .collect();
        let t = input["target_x"].as_str().unwrap().parse().unwrap();
        let (algebra, aa, bb) = certify(&weights, 2, 3, t, p);
        let dir = out.join(key);
        if !replay {
            fs::create_dir(&dir).unwrap();
        }
        let mut files = vec![];
        for (prefix, factors) in [("A", &aa), ("B", &bb)] {
            for (i, factor) in factors.iter().enumerate() {
                let name = format!("{prefix}{i}.poly");
                let data = bytes(factor, 27, p);
                assert_eq!(parse(&data).2, *factor);
                if replay {
                    let stored = fs::read(dir.join(&name)).unwrap();
                    assert_eq!(stored, data);
                    assert_eq!(parse(&stored).2, *factor);
                } else {
                    fs::write(dir.join(&name), &data).unwrap();
                }
                files.push(json!({"path":name,"bytes":data.len(),"sha256":sha(&data),"terms":factor.len(),"degree":degree(factor)}));
            }
        }
        let cert = json!({"schema":"n83.prime-first-fall-certificate/v1","key":key,"input":input,"source":source,"algebra":algebra,"factor_files":files,
            "witness_circuit":{"operation":"sum_of_tensor_products","terms":[["A0.poly","B0.poly"],["A1.poly","B1.poly"],["A2.poly","B2.poly"]],"A_variables":"normalized z0..z26","B_variables":"normalized u0..u26","normalization":"z_i=w_i*v_i,u_i=w_i*v_(27+i)","relation":"g=NF(H*S3(X,Y,t)); H=A0"}});
        let path = dir.join("certificate.json");
        if replay {
            assert_eq!(read(&path), cert);
        } else {
            save(&path, &cert);
        }
        rows.push(json!({"key":key,"arm":case["arm"],"family":"prime","m":2,"ell":27,"target_kind":input["target_kind"],"source_sha256":input["source_sha256"],"factor_base_manifest_sha256":input["factor_base_manifest_sha256"],"first_fall_degree":algebra["first_fall_degree"],"strict_filtered_fall_verified":algebra["strict_filtered_fall_verified"],"first_strict_filtered_fall_degree":algebra["first_strict_filtered_fall_degree"],"last_fall_lower_bound":algebra["last_fall_lower_bound"],"last_fall_upper_bound":null,"last_fall_degree":null,"certificate":format!("{key}/certificate.json"),"verified":true}));
        println!("{} {}", if replay { "replayed" } else { "certified" }, key);
    }
    let summary = json!({"schema":"n83.prime-first-fall-summary/v1","rows":rows,"cases":6,"independent_scientific_review":false,"broader_goal":"active"});
    if replay {
        assert_eq!(read(&out.join("summary.json")), summary);
    } else {
        save(&out.join("summary.json"), &summary);
    }
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("controls") => {
            assert_eq!(args.len(), 3);
            save(Path::new(&args[2]), &controls());
        }
        Some("run") | Some("replay") => {
            assert_eq!(args.len(), 4);
            run(
                Path::new(&args[2]),
                Path::new(&args[3]),
                args[1] == "replay",
            );
        }
        _ => panic!("usage: prime_first_fall controls OUT | run ROOT OUT | replay ROOT OUT"),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn expanded_and_dense_controls() {
        assert_eq!(controls()["all_passed"], true);
    }
    #[test]
    fn encoding_roundtrip_and_corruption() {
        let h = h_poly(5, 101);
        let b = bytes(&h, 5, 101);
        assert_eq!(parse(&b).2, h);
        let mut bad = b.clone();
        bad[0] ^= 1;
        assert!(std::panic::catch_unwind(|| parse(&bad)).is_err());
        assert!(std::panic::catch_unwind(|| parse(&b[..b.len() - 1])).is_err());
    }
    #[test]
    fn reject_zero_weight_and_small_characteristic() {
        assert!(std::panic::catch_unwind(|| certify(&[1, 0, 3], 2, 3, 9, 101)).is_err());
        assert!(std::panic::catch_unwind(|| certify(&[1, 2, 1], 1, 1, 1, 3)).is_err());
    }
    #[test]
    fn explicit_h_annihilator() {
        for n in [3, 5, 7] {
            let p = 101;
            let h = h_poly(n, p);
            let x = linear(n);
            let zeros = vec![0; n];
            assert!(product(&h, &product(&x, &x, &zeros, p), &zeros, p).is_empty());
        }
    }
}

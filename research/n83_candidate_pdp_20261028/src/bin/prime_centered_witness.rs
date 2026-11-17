//! Centered terminal arithmetic. Filtered-space conclusions live in the reviewed proof.
#[path = "../prime_centered_binding.rs"]
mod binding;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
type Poly = BTreeMap<u64, u128>;
const P83: u128 = 4835703278761981908090881;
fn add(a: u128, b: u128, p: u128) -> u128 {
    (a + b) % p
}
fn neg(a: u128, p: u128) -> u128 {
    if a == 0 {
        0
    } else {
        p - a
    }
}
fn mul(mut a: u128, mut b: u128, p: u128) -> u128 {
    let mut r = 0;
    while b > 0 {
        if b & 1 != 0 {
            r = add(r, a, p);
        }
        a = add(a, a, p);
        b >>= 1;
    }
    r
}
fn pow(mut a: u128, mut e: u128, p: u128) -> u128 {
    let mut r = 1;
    while e > 0 {
        if e & 1 != 0 {
            r = mul(r, a, p);
        }
        a = mul(a, a, p);
        e >>= 1;
    }
    r
}
fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}
fn small_prime(p: u128) -> bool {
    p >= 2 && (2..).take_while(|d| d * d <= p).all(|d| p % d != 0)
}
fn prime_witness(p: u128) -> u128 {
    assert!(
        p == P83 || (p <= 1000 && small_prime(p)),
        "unsupported or composite modulus"
    );
    let factors = if p == P83 {
        vec![2, 5, 677, 13999, 23203]
    } else {
        let mut n = p - 1;
        let mut v = Vec::new();
        for d in 2..p {
            if n % d == 0 {
                v.push(d);
                while n % d == 0 {
                    n /= d;
                }
            }
        }
        v
    };
    let mut remaining = p - 1;
    for &q in &factors {
        assert!(small_prime(q));
        while remaining % q == 0 {
            remaining /= q;
        }
    }
    assert_eq!(remaining, 1);
    (2..100)
        .find(|&a| {
            pow(a, p - 1, p) == 1
                && factors
                    .iter()
                    .all(|&q| gcd(add(pow(a, (p - 1) / q, p), p - 1, p), p) == 1)
        })
        .expect("Lucas witness")
}
fn accum(a: &mut Poly, m: u64, c: u128, p: u128) {
    let v = add(*a.get(&m).unwrap_or(&0), c, p);
    if v == 0 {
        a.remove(&m);
    } else {
        a.insert(m, v);
    }
}
fn scale_add(a: &mut Poly, b: &Poly, c: u128, p: u128) {
    for (&m, &v) in b {
        accum(a, m, mul(v, c, p), p);
    }
}
fn degree(a: &Poly) -> Option<usize> {
    a.keys().map(|m| m.count_ones() as usize).max()
}
fn paired(n: usize, p: u128) -> Poly {
    let mut a = Poly::from([(0, 1)]);
    for j in 0..n / 2 {
        let mut b = Poly::new();
        for (m, c) in a {
            accum(&mut b, m | (1 << (2 * j)), c, p);
            accum(&mut b, m | (1 << (2 * j + 1)), neg(c, p), p);
        }
        a = b;
    }
    a
}
struct Ring {
    p: u128,
    w: Vec<u128>,
    b2: Vec<u128>,
    c: u128,
}
impl Ring {
    fn new(p: u128, w: Vec<u128>) -> Self {
        assert!(p > 2 && p < (1 << 127));
        assert!(w.len() >= 3 && w.len() <= 27 && w.len() % 2 == 1 && p > w.len() as u128);
        assert!(w.iter().all(|&x| x > 0 && x < p));
        let half = (p + 1) / 2;
        let b: Vec<_> = w.iter().map(|&x| mul(x, half, p)).collect();
        let c = b.iter().fold(0, |a, &x| add(a, x, p));
        let b2 = b.iter().map(|&x| mul(x, x, p)).collect();
        Self { p, w, b2, c }
    }
    fn times_x(&self, a: &Poly) -> Poly {
        let mut out = Poly::new();
        scale_add(&mut out, a, self.c, self.p);
        for (&m, &v) in a {
            for (j, &b2) in self.b2.iter().enumerate() {
                let bit = 1 << j;
                accum(
                    &mut out,
                    m ^ bit,
                    if m & bit == 0 { v } else { mul(v, b2, self.p) },
                    self.p,
                );
            }
        }
        out
    }
    fn project(&self, a: &Poly) -> [u128; 2] {
        let s = self.w.len() / 2;
        let mut out = [0, 0];
        for (&m, &v) in a {
            if (0..s).all(|j| ((m >> (2 * j)) & 3).count_ones() == 1) {
                let sign = (0..s).filter(|j| m & (1 << (2 * j + 1)) != 0).count() % 2;
                let k = ((m >> (2 * s)) & 1) as usize;
                out[k] = add(out[k], if sign == 0 { v } else { neg(v, self.p) }, self.p);
            }
        }
        let inv = pow((self.p + 1) / 2, s as u128, self.p);
        out.map(|v| mul(v, inv, self.p))
    }
    #[cfg(test)]
    fn product(&self, a: &Poly, b: &Poly) -> Poly {
        let mut out = Poly::new();
        for (&m, &c) in a {
            for (&n, &d) in b {
                let mut v = mul(c, d, self.p);
                let mut repeats = m & n;
                while repeats != 0 {
                    let j = repeats.trailing_zeros() as usize;
                    v = mul(v, self.b2[j], self.p);
                    repeats &= repeats - 1;
                }
                accum(&mut out, m ^ n, v, self.p);
            }
        }
        out
    }
}
fn circuit(t: u128, p: u128) -> [[u128; 3]; 3] {
    let t2 = mul(t, t, p);
    let minus4t12 = neg(add(mul(4, t, p), 12 % p, p), p);
    [
        [add(4 % p, neg(mul(12 % p, t, p), p), p), minus4t12, t2],
        [
            minus4t12,
            neg(add(mul(2, t2, p), 4 % p, p), p),
            neg(mul(2, t, p), p),
        ],
        [t2, neg(mul(2, t, p), p), 1],
    ]
}
struct ResultData {
    a: [Poly; 3],
    f: [Poly; 3],
    projection: [[u128; 2]; 3],
    expected: [[u128; 2]; 3],
    degree: usize,
}
fn calculate(r: &Ring, t: u128) -> ResultData {
    assert!(t < r.p);
    let a0 = paired(r.w.len(), r.p);
    let a1 = r.times_x(&a0);
    let a2 = r.times_x(&a1);
    let a = [a0, a1, a2];
    let b = circuit(t, r.p);
    let mut f: [Poly; 3] = std::array::from_fn(|_| Poly::new());
    for u in 0..3 {
        for x in 0..3 {
            scale_add(&mut f[u], &a[x], b[x][u], r.p);
        }
    }
    let projection = std::array::from_fn(|u| r.project(&f[u]));
    let cc = mul(r.c, r.c, r.p);
    let bl2 = *r.b2.last().unwrap();
    let expected = std::array::from_fn(|u| {
        [
            add(
                add(b[0][u], mul(r.c, b[1][u], r.p), r.p),
                mul(add(cc, bl2, r.p), b[2][u], r.p),
                r.p,
            ),
            add(b[1][u], mul(mul(2, r.c, r.p), b[2][u], r.p), r.p),
        ]
    });
    assert_eq!(projection, expected, "alternating projection formula");
    let degree = f
        .iter()
        .enumerate()
        .filter_map(|(u, a)| degree(a).map(|d| d + u))
        .max()
        .expect("nonzero remainder");
    assert_eq!(degree, r.w.len() / 2 + if t == r.c { 2 } else { 3 });
    assert!(projection.iter().flatten().any(|&x| x != 0));
    if t == r.c {
        assert_eq!(projection[2], [bl2, 0]);
    } else {
        assert_eq!(projection[2][1], mul(2, add(r.c, neg(t, r.p), r.p), r.p));
    }
    ResultData {
        a,
        f,
        projection,
        expected,
        degree,
    }
}
fn encode(a: &Poly, n: usize, p: u128) -> Vec<u8> {
    let mut b = b"N83CEN1\n".to_vec();
    b.extend((n as u32).to_le_bytes());
    b.extend((degree(a).unwrap_or(0) as u32).to_le_bytes());
    b.extend((a.len() as u64).to_le_bytes());
    b.extend(p.to_le_bytes());
    for (&m, &c) in a {
        assert!(m < (1u64 << n) && c > 0 && c < p);
        b.extend(m.to_le_bytes());
        b.extend(c.to_le_bytes());
    }
    b
}
fn decode(b: &[u8], n: usize, p: u128) -> Result<Poly, String> {
    if b.len() < 40 || &b[..8] != b"N83CEN1\n" {
        return Err("header".into());
    }
    let mut a = Poly::new();
    if (b.len() - 40) % 24 != 0 {
        return Err("length".into());
    }
    let mut prior = None;
    for row in b[40..].chunks_exact(24) {
        let m = u64::from_le_bytes(row[..8].try_into().unwrap());
        let c = u128::from_le_bytes(row[8..].try_into().unwrap());
        if m >= 1u64 << n || c == 0 || c >= p || prior.map_or(false, |q| m <= q) {
            return Err("noncanonical term".into());
        }
        prior = Some(m);
        a.insert(m, c);
    }
    if encode(&a, n, p) != b {
        return Err("header or degree mismatch".into());
    }
    Ok(a)
}
fn sha(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn value(v: &Value) -> u128 {
    v.as_str().expect("decimal string").parse().unwrap()
}
fn prepare(input: &Value) -> (Ring, u128, u128) {
    if input.get("binding").is_some() {
        binding::validate(input);
    }
    let p = value(&input["modulus"]);
    let witness = prime_witness(p);
    let w = input["weights"]
        .as_array()
        .unwrap()
        .iter()
        .map(value)
        .collect();
    let r = Ring::new(p, w);
    assert_eq!(input["ell"].as_u64().unwrap(), r.w.len() as u64);
    let t = value(&input["target"]);
    assert!(t < p);
    (r, t, witness)
}
fn payloads(input: &Value) -> BTreeMap<String, Vec<u8>> {
    let (r, t, witness) = prepare(input);
    let d = calculate(&r, t);
    let mut out = BTreeMap::new();
    out.insert(
        "input.json".into(),
        (serde_json::to_string_pretty(input).unwrap() + "\n").into_bytes(),
    );
    for (prefix, items) in [("A", &d.a), ("F", &d.f)] {
        for (j, a) in items.iter().enumerate() {
            let bytes = encode(a, r.w.len(), r.p);
            assert_eq!(decode(&bytes, r.w.len(), r.p).unwrap(), *a);
            out.insert(format!("{prefix}{j}.poly"), bytes);
        }
    }
    let coords = |a: [[u128; 2]; 3]| a.map(|row| row.map(|x| x.to_string()));
    let cert = json!({"schema":"n83.centered-witness/v1","modulus":r.p.to_string(),"ell":r.w.len(),"weights":r.w.iter().map(|x|x.to_string()).collect::<Vec<_>>(),"target":t.to_string(),"center":r.c.to_string(),"centered_target":t==r.c,"primality_lucas_witness":witness.to_string(),"remainder_degree":d.degree,"a_degrees":d.a.iter().map(degree).collect::<Vec<_>>(),"a_terms":d.a.iter().map(|a|a.len()).collect::<Vec<_>>(),"f_degrees":d.f.iter().map(degree).collect::<Vec<_>>(),"f_terms":d.f.iter().map(|a|a.len()).collect::<Vec<_>>(),"projection_u_coefficients":coords(d.projection),"expected_projection_u_coefficients":coords(d.expected),"projection_nonzero":true,"derivation_degree_upper_bound":r.w.len()/2+4,"full_polynomial_formalization_verified":false,"exact_lfd":null,"regularity":null,"source_binding":if input.get("binding").is_some() {json!({"status":"full_source_and_base_identity_verified","binding":input["binding"],"m":input["m"],"arm":input["arm"],"target_origin":input["target_origin"]})}else{json!("terminal equation S(U,X,t), curve a=2,b=3; full per-m source and factor-base binding not performed by this instrument")},"payload_hashes":out.iter().map(|(k,v)|(k.clone(),sha(v))).collect::<BTreeMap<_,_>>()});
    out.insert(
        "certificate.json".into(),
        (serde_json::to_string_pretty(&cert).unwrap() + "\n").into_bytes(),
    );
    let manifest: Vec<_> = out
        .iter()
        .map(|(k, v)| json!({"path":k,"bytes":v.len(),"sha256":sha(v)}))
        .collect();
    out.insert(
        "manifest.json".into(),
        (serde_json::to_string_pretty(&manifest).unwrap() + "\n").into_bytes(),
    );
    out
}
fn run(input: &Value, out: &Path) {
    assert!(!out.exists(), "output exists");
    let files = payloads(input);
    fs::create_dir_all(out).unwrap();
    for (name, bytes) in files {
        fs::write(out.join(name), bytes).unwrap();
    }
}
fn verify(out: &Path) -> bool {
    let input: Value = serde_json::from_slice(&fs::read(out.join("input.json")).unwrap()).unwrap();
    let want = payloads(&input);
    let names: std::collections::BTreeSet<_> = fs::read_dir(out)
        .unwrap()
        .map(|x| x.unwrap().file_name().into_string().unwrap())
        .collect();
    names == want.keys().cloned().collect()
        && want
            .iter()
            .all(|(name, bytes)| fs::read(out.join(name)).map_or(false, |b| b == *bytes))
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("prepare-panel") => {
            assert_eq!(
                args.len(),
                5,
                "prepare-panel PARENT_ROOT CASES_JSON FRESH_OUTPUT"
            );
            binding::prepare(
                Path::new(&args[2]),
                Path::new(&args[3]),
                Path::new(&args[4]),
            );
        }
        Some("generate") => {
            assert_eq!(args.len(), 4, "generate INPUT OUTPUT");
            let input: Value = serde_json::from_slice(&fs::read(&args[2]).unwrap()).unwrap();
            run(&input, Path::new(&args[3]));
            println!("generated {}", args[3]);
        }
        Some("verify") => {
            assert_eq!(args.len(), 3, "verify OUTPUT");
            assert!(verify(Path::new(&args[2])), "replay mismatch");
            println!("verified {}", args[2]);
        }
        _ => panic!("usage: prime_centered_witness generate INPUT OUTPUT | verify OUTPUT"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn weights(n: usize, p: u128, alternate: bool) -> Vec<u128> {
        let mut w: Vec<_> = (0..n).map(|j| (1u128 << j) % p).collect();
        if alternate {
            w[n - 1] = 37 % p;
        }
        w
    }
    fn literal(r: &Ring, a: &Poly) -> Poly {
        let mut out = a.clone();
        for j in 0..r.w.len() / 2 {
            let mut next = Poly::new();
            for (&m, &c) in &out {
                accum(&mut next, m, c, r.p);
                let bits = (1 << (2 * j)) | (1 << (2 * j + 1));
                let swapped = if (m & bits).count_ones() == 1 {
                    m ^ bits
                } else {
                    m
                };
                accum(&mut next, swapped, neg(c, r.p), r.p);
            }
            out = next;
        }
        let mut scaled = Poly::new();
        scale_add(
            &mut scaled,
            &out,
            pow((r.p + 1) / 2, (r.w.len() / 2) as u128, r.p),
            r.p,
        );
        scaled
    }
    fn expand(r: &Ring, v: [u128; 2]) -> Poly {
        let p = paired(r.w.len(), r.p);
        let mut out = Poly::new();
        scale_add(&mut out, &p, v[0], r.p);
        for (&m, &c) in &p {
            accum(&mut out, m | (1 << (r.w.len() - 1)), mul(c, v[1], r.p), r.p);
        }
        out
    }
    #[test]
    fn projection_matches_literal_on_complete_small_basis() {
        for n in [3, 5, 7] {
            let r = Ring::new(101, weights(n, 101, true));
            for m in 0..1u64 << n {
                let a = Poly::from([(m, 1)]);
                assert_eq!(expand(&r, r.project(&a)), literal(&r, &a));
            }
        }
    }
    #[test]
    fn all_low_degree_rows_annihilated() {
        for n in [3, 5, 7] {
            let r = Ring::new(103, weights(n, 103, true));
            for m in 0..1u64 << n {
                if m.count_ones() as usize >= n / 2 {
                    continue;
                }
                let mut a = Poly::from([(m, 1)]);
                for _ in 0..3 {
                    assert_eq!(r.project(&a), [0, 0]);
                    a = r.times_x(&a);
                }
            }
        }
    }
    #[test]
    fn multiplication_matches_general_quotient() {
        let r = Ring::new(101, weights(5, 101, true));
        let x: Poly = std::iter::once((0, r.c))
            .chain((0..5).map(|j| (1 << j, 1)))
            .collect();
        for m in 0..32 {
            let a = Poly::from([(m, 7)]);
            assert_eq!(r.times_x(&a), r.product(&a, &x));
        }
    }
    fn eval(a: &Poly, y: &[u128], p: u128) -> u128 {
        a.iter().fold(0, |s, (&m, &c)| {
            add(
                s,
                (0..y.len())
                    .filter(|j| m & (1 << j) != 0)
                    .fold(c, |v, j| mul(v, y[j], p)),
                p,
            )
        })
    }
    fn direct_s(u: u128, x: u128, t: u128, p: u128) -> u128 {
        let ux = mul(u, x, p);
        let delta = add(u, neg(x, p), p);
        let first = mul(mul(delta, delta, p), mul(t, t, p), p);
        let second = mul(
            mul(2, add(mul(add(u, x, p), add(ux, 2, p), p), 6, p), p),
            t,
            p,
        );
        let third = mul(add(ux, p - 2, p), add(ux, p - 2, p), p);
        let last = mul(12, add(u, x, p), p);
        add(
            add(first, neg(second, p), p),
            add(third, neg(last, p), p),
            p,
        )
    }
    #[test]
    fn exhaustive_pointwise_terminal_identity_and_degrees() {
        for p in [101, 103] {
            for n in [3, 5] {
                for alt in [false, true] {
                    let r = Ring::new(p, weights(n, p, alt));
                    for t in [r.c, add(r.c, 1, p), 0] {
                        let d = calculate(&r, t);
                        let root = std::env::var("N83_CENTERED_TEST_OUTPUT")
                            .expect("persistent test output");
                        let out = Path::new(&root)
                            .join(format!("exhaustive-p{p}-n{n}-alt{alt}-target{t}"));
                        let input = json!({"modulus":p.to_string(),"ell":n,"weights":r.w.iter().map(|x|x.to_string()).collect::<Vec<_>>(),"target":t.to_string(),"provenance":"exhaustive small development identity check"});
                        run(&input, &out);
                        for mask in 0..1u64 << n {
                            let y: Vec<_> =
                                r.w.iter()
                                    .enumerate()
                                    .map(|(j, &w)| {
                                        let b = mul(w, (p + 1) / 2, p);
                                        if mask & (1 << j) != 0 {
                                            b
                                        } else {
                                            neg(b, p)
                                        }
                                    })
                                    .collect();
                            let x = y.iter().fold(r.c, |s, &v| add(s, v, p));
                            for u in [0, 1, 2] {
                                let lhs = d.f.iter().enumerate().fold(0, |s, (k, a)| {
                                    add(s, mul(eval(a, &y, p), pow(u, k as u128, p), p), p)
                                });
                                assert_eq!(lhs, mul(eval(&d.a[0], &y, p), direct_s(u, x, t, p), p));
                            }
                        }
                    }
                }
            }
        }
    }
    #[test]
    fn n83_modulus_has_exact_lucas_certificate() {
        assert_eq!(prime_witness(P83), 3);
    }
    #[test]
    fn invalid_parameters_rejected() {
        for (p, w) in [
            (2, vec![1, 1, 1]),
            (101, vec![1, 2, 3, 4]),
            (101, vec![1, 0, 3]),
            (101, vec![1, 101, 3]),
        ] {
            assert!(std::panic::catch_unwind(|| Ring::new(p, w)).is_err());
        }
        assert!(std::panic::catch_unwind(|| prime_witness(91)).is_err());
    }
    #[test]
    fn codec_rejects_noncanonical_bytes() {
        let a = paired(3, 101);
        let b = encode(&a, 3, 101);
        assert_eq!(decode(&b, 3, 101).unwrap(), a);
        for index in [0, 8, 12, 16, 24] {
            let mut c = b.clone();
            c[index] ^= 1;
            assert!(decode(&c, 3, 101).is_err());
        }
        let mut c = b.clone();
        c[48..64].fill(0);
        assert!(decode(&c, 3, 101).is_err());
        let mut c = b;
        c.extend([0]);
        assert!(decode(&c, 3, 101).is_err());
    }
    #[test]
    fn fixture_replay_rejects_coefficient_mutation() {
        let root = std::env::var("N83_CENTERED_TEST_OUTPUT")
            .expect("set a fresh persistent test output directory");
        let out = Path::new(&root).join("small-fixture");
        let r = Ring::new(101, weights(3, 101, false));
        let input = json!({"modulus":"101","ell":3,"weights":["1","2","4"],"target":r.c.to_string(),"provenance":"development unit fixture, not n83 production"});
        run(&input, &out);
        assert!(verify(&out));
        let mutant = Path::new(&root).join("mutated-fixture");
        fs::create_dir_all(&mutant).unwrap();
        for f in fs::read_dir(&out).unwrap() {
            let f = f.unwrap();
            fs::copy(f.path(), mutant.join(f.file_name())).unwrap();
        }
        let path = mutant.join("F2.poly");
        let mut bytes = fs::read(&path).unwrap();
        bytes[48] ^= 1;
        fs::write(path, bytes).unwrap();
        assert!(!verify(&mutant));
    }
}

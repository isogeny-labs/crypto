//! Finite closed zero divisors of the actual recovered Hecke norm functions.
use super::*;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};
#[path = "ordinary13_lift.rs"]
pub(crate) mod lift;
static REDUCTIONS: AtomicUsize = AtomicUsize::new(0);
static REDUCTION_LIMIT: AtomicUsize = AtomicUsize::new(100_000_000);
fn parse(s: &str) -> B {
    let bytes = BigUint::parse_bytes(s.as_bytes(), 16)
        .unwrap()
        .to_bytes_le();
    clean(
        bytes
            .chunks(8)
            .map(|v| {
                let mut a = [0u8; 8];
                a[..v.len()].copy_from_slice(v);
                u64::from_le_bytes(a)
            })
            .collect(),
    )
}
fn divide(a: &[u64], b: &[u64]) -> (B, B) {
    let d = degree(b).expect("nonzero polynomial divisor");
    let mut r = a.to_vec();
    let mut q = vec![];
    while degree(&r).is_some_and(|e| e >= d) {
        assert!(
            REDUCTIONS.fetch_add(1, Ordering::Relaxed) < REDUCTION_LIMIT.load(Ordering::Relaxed),
            "frozen reduction cap"
        );
        let k = degree(&r).unwrap() - d;
        set(&mut q, k);
        r = add(&r, &shift(b, k, 2049));
    }
    (clean(q), r)
}
fn exact(a: &[u64], b: &[u64]) -> B {
    let (q, r) = divide(a, b);
    assert!(r.is_empty());
    q
}
fn gcd(mut a: B, mut b: B) -> B {
    while !b.is_empty() {
        let r = divide(&a, &b).1;
        a = b;
        b = r;
    }
    a
}
fn product(a: &[u64], b: &[u64]) -> B {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let n = degree(a).unwrap() + degree(b).unwrap() + 1;
    assert!(n <= 2049, "polynomial degree cap");
    mul(a, b, n)
}
fn msq(a: &[u64], p: &[u64]) -> B {
    divide(&sq(a, 2049), p).1
}
fn mmul(a: &[u64], b: &[u64], p: &[u64]) -> B {
    divide(&product(a, b), p).1
}
fn derivative(a: &[u64]) -> B {
    let mut z = vec![];
    each(a, |i| {
        if i % 2 == 1 {
            set(&mut z, i - 1)
        }
    });
    z
}
fn root(a: &[u64]) -> B {
    let mut z = vec![];
    each(a, |i| {
        assert_eq!(i % 2, 0);
        set(&mut z, i / 2)
    });
    z
}
fn is_one(a: &[u64]) -> bool {
    a == [1]
}
fn irreducible(p: &[u64]) -> bool {
    let d = degree(p).unwrap();
    if d == 0 {
        return false;
    }
    let x = divide(&[2], p).1;
    let mut a = x.clone();
    let mut primes = vec![];
    let mut r = d;
    let mut l = 2;
    while l * l <= r {
        if r % l == 0 {
            primes.push(l);
            while r % l == 0 {
                r /= l;
            }
        }
        l += 1;
    }
    if r > 1 {
        primes.push(r);
    }
    for i in 1..=d {
        a = msq(&a, p);
        if primes.iter().any(|l| i == d / l) && !is_one(&gcd(p.to_vec(), add(&a, &x))) {
            return false;
        }
    }
    a == x
}
fn equal_degree(p: B, d: usize, out: &mut Vec<B>, attempts: &mut usize) {
    let n = degree(&p).unwrap();
    if n == d {
        assert!(irreducible(&p));
        out.push(p);
        return;
    }
    assert_eq!(n % d, 0);
    let mut a = divide(&[2], &p).1;
    for _ in 0..4096 {
        *attempts += 1;
        let mut z = vec![];
        let mut v = a.clone();
        for _ in 0..d {
            z = add(&z, &v);
            v = msq(&v, &p);
        }
        let g = gcd(p.clone(), z);
        if !is_one(&g) && g != p {
            let q = exact(&p, &g);
            equal_degree(g, d, out, attempts);
            equal_degree(q, d, out, attempts);
            return;
        }
        a = mmul(&a, &[2], &p);
    }
    panic!("frozen equal-degree splitting cap");
}
fn squarefree(p: B, out: &mut Vec<B>, attempts: &mut usize) {
    if is_one(&p) {
        return;
    }
    let mut r = p;
    let mut a = vec![2];
    let mut d = 1;
    while degree(&r).is_some_and(|n| 2 * d <= n) {
        a = msq(&a, &r);
        let g = gcd(r.clone(), add(&a, &[2]));
        if !is_one(&g) {
            r = exact(&r, &g);
            equal_degree(g, d, out, attempts);
            if is_one(&r) {
                break;
            }
            a = divide(&a, &r).1;
        }
        d += 1;
    }
    if !is_one(&r) {
        assert!(irreducible(&r));
        out.push(r);
    }
}
fn factor_inner(p: B, scale: usize, out: &mut Vec<(B, usize)>, attempts: &mut usize) {
    if is_one(&p) {
        return;
    }
    let mut c = gcd(p.clone(), derivative(&p));
    let mut w = exact(&p, &c);
    let mut i = 1;
    while !is_one(&w) {
        let y = gcd(w.clone(), c.clone());
        let z = exact(&w, &y);
        let mut fs = vec![];
        squarefree(z, &mut fs, attempts);
        out.extend(fs.into_iter().map(|p| (p, i * scale)));
        w = y;
        c = exact(&c, &w);
        i += 1;
    }
    if !is_one(&c) {
        factor_inner(root(&c), 2 * scale, out, attempts);
    }
}
fn factors(p: &[u64]) -> (Vec<(B, usize)>, usize) {
    assert!(!p.is_empty());
    let mut fs = vec![];
    let mut attempts = 0;
    factor_inner(p.to_vec(), 1, &mut fs, &mut attempts);
    let mut merged: BTreeMap<String, (B, usize)> = BTreeMap::new();
    for (p, m) in fs {
        let e = merged.entry(hex(&p)).or_insert((p, 0));
        e.1 += m;
    }
    let fs: Vec<_> = merged.into_values().collect();
    let mut check = one();
    for (p, m) in &fs {
        assert!(irreducible(p));
        for _ in 0..*m {
            check = product(&check, p);
        }
    }
    assert_eq!(check, p);
    (fs, attempts)
}
struct K {
    p: B,
    d: usize,
}
impl K {
    fn new(p: B) -> Self {
        assert!(irreducible(&p));
        Self {
            d: degree(&p).unwrap(),
            p,
        }
    }
    fn red(&self, a: &[u64]) -> B {
        divide(a, &self.p).1
    }
    fn mul(&self, a: &[u64], b: &[u64]) -> B {
        mmul(a, b, &self.p)
    }
    fn sq(&self, a: &[u64]) -> B {
        msq(a, &self.p)
    }
    fn inv(&self, a: &[u64]) -> B {
        assert!(!a.is_empty());
        let mut z = one();
        for _ in 1..self.d {
            z = self.mul(&self.sq(&z), a);
        }
        z = self.sq(&z);
        assert_eq!(self.mul(a, &z), one());
        z
    }
    fn asroot(&self, g: &[u64]) -> Option<B> {
        let mut tr = vec![];
        let mut a = g.to_vec();
        for _ in 0..self.d {
            tr = add(&tr, &a);
            a = self.sq(&a);
        }
        assert!(tr.is_empty() || is_one(&tr));
        if !tr.is_empty() {
            return None;
        }
        let mut rows = vec![B::new(); self.d];
        for j in 0..self.d {
            let mut e = vec![];
            set(&mut e, j);
            let z = add(&self.sq(&e), &e);
            each(&z, |i| set(&mut rows[i], j));
        }
        each(g, |i| set(&mut rows[i], self.d));
        let mut piv = vec![];
        let mut rank = 0;
        for j in 0..self.d {
            if let Some(i) = (rank..self.d).find(|i| bit(&rows[*i], j)) {
                rows.swap(rank, i);
                for k in 0..self.d {
                    if k != rank && bit(&rows[k], j) {
                        rows[k] = add(&rows[k], &rows[rank]);
                    }
                }
                piv.push(j);
                rank += 1;
            }
        }
        for row in rows.iter().skip(rank) {
            assert!(!bit(row, self.d));
        }
        let mut w = vec![];
        for (i, j) in piv.iter().enumerate() {
            if bit(&rows[i], self.d) {
                set(&mut w, *j);
            }
        }
        assert_eq!(add(&self.sq(&w), &w), g);
        Some(w)
    }
}
#[derive(Clone)]
struct Z {
    p: B,
    y: Option<B>,
    degree: usize,
    mult: usize,
    class: String,
}
fn classify(k: &K, y: &Option<B>) -> String {
    let h = hex(&k.p);
    if h == "3" || h == "7" {
        return "supersingular".into();
    }
    if h == "2" {
        return "cusp".into();
    }
    if h == "b" || h == "d" {
        let x = k.red(&[2]);
        let den = add(&add(&k.sq(&x), &x), &one());
        let yc = k.mul(&add(&x, &one()), &k.inv(&den));
        if y.as_ref() == Some(&yc) {
            return "cusp".into();
        }
    }
    "ordinary".into()
}
fn insert(z: &mut BTreeMap<String, Z>, p: B, y: Option<B>, d: usize, m: usize) {
    let k = K::new(p.clone());
    let x = k.red(&[2]);
    if let Some(y) = &y {
        let rhs = add(&add(&k.mul(&k.sq(&x), &x), &k.sq(&x)), &one());
        assert_eq!(add(&k.sq(y), &k.mul(&x, y)), rhs);
    }
    let class = classify(&k, &y);
    let key = format!(
        "{}:{}",
        hex(&p),
        y.as_ref().map(|v| hex(v)).unwrap_or("quadratic".into())
    );
    let e = z.entry(key).or_insert(Z {
        p,
        y,
        degree: d,
        mult: 0,
        class,
    });
    e.mult += m;
}
fn zero_divisor(a: &[u64], b: &[u64]) -> (Vec<Z>, serde_json::Value) {
    let common = gcd(a.to_vec(), b.to_vec());
    let aa = exact(a, &common);
    let bb = exact(b, &common);
    let nr = add(
        &add(&product(&aa, &aa), &shift(&product(&aa, &bb), 1, 2049)),
        &product(&[0b1101], &product(&bb, &bb)),
    );
    let (gs, ga) = factors(&common);
    let (rs, ra) = factors(&nr);
    let mut z = BTreeMap::new();
    for (p, m) in &gs {
        let k = K::new(p.clone());
        let x = k.red(&[2]);
        if x.is_empty() {
            insert(&mut z, p.clone(), Some(one()), 1, 2 * m);
            continue;
        }
        let g = add(&add(&x, &one()), &k.inv(&k.sq(&x)));
        if let Some(w) = k.asroot(&g) {
            let y = k.mul(&x, &w);
            insert(&mut z, p.clone(), Some(y.clone()), k.d, *m);
            insert(&mut z, p.clone(), Some(add(&y, &x)), k.d, *m);
        } else {
            insert(&mut z, p.clone(), None, 2 * k.d, *m);
        }
    }
    for (p, m) in &rs {
        let k = K::new(p.clone());
        let av = k.red(&aa);
        let bv = k.red(&bb);
        assert!(!bv.is_empty(), "coprime A,B imply B unit at each norm root");
        let y = k.mul(&av, &k.inv(&bv));
        insert(&mut z, p.clone(), Some(y), k.d, *m);
    }
    let fj = |fs: &Vec<(B, usize)>| {
        fs.iter().map(|(p,m)|json!({"bits":hex(p),"degree":degree(p).unwrap(),"multiplicity":m,"Rabin_irreducible":true})).collect::<Vec<_>>()
    };
    (
        z.into_values().collect(),
        json!({"common_polynomial_bits":hex(&common),"primitive_A_bits":hex(&aa),"primitive_B_bits":hex(&bb),"primitive_norm_bits":hex(&nr),"common_factors":fj(&gs),"primitive_norm_factors":fj(&rs),"equal_degree_split_attempts":ga+ra,"factor_products_reconstructed":true}),
    )
}
pub fn run(dir: &Path) {
    let input = crate::round04::read(&dir.join("o13-cyclic121-Hecke-norms.json"));
    let mut records = vec![];
    let expected = [[84, 320, 100], [0, 412, 116]];
    for (i, r) in input["records"].as_array().unwrap().iter().enumerate() {
        let a = parse(r["A_bits"].as_str().unwrap());
        let b = parse(r["B_bits"].as_str().unwrap());
        let (z, fac) = zero_divisor(&a, &b);
        let mut degrees = [0usize; 3];
        let mut counts = [0usize; 3];
        let mut pts = vec![];
        for p in &z {
            let j = match p.class.as_str() {
                "cusp" => 0,
                "supersingular" => 1,
                "ordinary" => 2,
                _ => unreachable!(),
            };
            degrees[j] += p.degree * p.mult;
            counts[j] += 1;
            pts.push(json!({"x_minpoly_bits":hex(&p.p),"y_residue_bits":p.y.as_ref().map(|v|hex(v)),"y_quadratic_relation":if p.y.is_none(){Some("y^2+x*y+x^3+x^2+1=0")}else{None},"closed_degree":p.degree,"multiplicity":p.mult,"stratum":p.class}));
        }
        assert_eq!(
            degrees.iter().sum::<usize>() as u64,
            r["pole_bound"].as_u64().unwrap()
        );
        println!(
            "O14:{} closed points,degree partition {:?},ordinary points{}",
            z.len(),
            degrees,
            counts[2]
        );
        records.push(json!({"function":input["functions"][i],"points":pts,"strata_order":["cusp","supersingular","ordinary"],"stratum_degrees":degrees,"closed_point_counts":counts,"expected_from_round12":expected[i],"matches_independent_degrees":degrees==expected[i],"factorization":fac}));
    }
    let okay = records
        .iter()
        .all(|r| r["matches_independent_degrees"] == true);
    round02::save(
        dir,
        "o14-pushed-closed-supports.json",
        &json!({"schema":"koblitz.round13/o14-v1","records":records,"all_expected_strata_degrees_match":okay,"polynomial_reduction_steps":REDUCTIONS.load(Ordering::Relaxed),"source_cover_degree":5,"source_lift_enumerated":false,"projected_images_evaluated":false,"scope":"complete closed zeros of two pushed-down norm functions;source five-sheet lift and projected images remain open"}),
    );
    assert!(okay, "degree partitions disagree with round12 control");
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn repeated_and_equal_degree_factors() {
        let a = vec![0b1011];
        let b = vec![0b1101];
        let c = vec![0b111];
        let mut p = product(&a, &b);
        p = product(&p, &a);
        p = product(&p, &c);
        p = product(&p, &c);
        let (fs, _) = factors(&p);
        assert_eq!(fs.len(), 3);
        assert_eq!(fs.iter().map(|(_, m)| m).sum::<usize>(), 5);
    }
    #[test]
    fn as_solver_even_degree() {
        let k = K::new(vec![0b10011]);
        for n in 0..16 {
            let w = clean(vec![n]);
            let g = add(&k.sq(&w), &w);
            assert!(k.asroot(&g).is_some());
            if n != 0 {
                assert_eq!(k.mul(&w, &k.inv(&w)), one());
            }
        }
    }
}

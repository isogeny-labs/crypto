//! Ordinary middle points and full-level kernel fibers over exact residue fields.
use super::*;
use std::collections::BTreeSet;
type P = Vec<B>;
fn pt(mut a: P) -> P {
    while a.last().is_some_and(|x| x.is_empty()) {
        a.pop();
    }
    a
}
fn pa(a: &[B], b: &[B]) -> P {
    (0..a.len().max(b.len()))
        .map(|i| {
            add(
                a.get(i).map(Vec::as_slice).unwrap_or(&[]),
                b.get(i).map(Vec::as_slice).unwrap_or(&[]),
            )
        })
        .collect::<P>()
        .pipe(pt)
}
trait Pipe: Sized {
    fn pipe<R>(self, f: impl FnOnce(Self) -> R) -> R {
        f(self)
    }
}
impl<T> Pipe for T {}
fn km(k: &K, a: &[u64], b: &[u64]) -> B {
    if is_one(a) {
        return b.to_vec();
    }
    if is_one(b) {
        return a.to_vec();
    }
    k.mul(a, b)
}
fn ki(k: &K, a: &[u64]) -> B {
    if is_one(a) {
        one()
    } else {
        k.inv(a)
    }
}
fn pm(k: &K, a: &[B], b: &[B]) -> P {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut z = vec![vec![]; a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            z[i + j] = add(&z[i + j], &km(k, x, y));
        }
    }
    pt(z)
}
fn ps(k: &K, a: &[B], c: &[u64]) -> P {
    pt(a.iter().map(|x| km(k, x, c)).collect())
}
fn pd(k: &K, a: &[B], b: &[B]) -> (P, P) {
    assert!(!b.is_empty());
    let mut r = pt(a.to_vec());
    let mut q = vec![vec![]; r.len().saturating_sub(b.len()) + 1];
    let bi = ki(k, b.last().unwrap());
    while r.len() >= b.len() {
        let d = r.len() - b.len();
        let c = km(k, r.last().unwrap(), &bi);
        q[d] = add(&q[d], &c);
        for (i, v) in b.iter().enumerate() {
            r[d + i] = add(&r[d + i], &km(k, &c, v));
        }
        r = pt(r);
    }
    (pt(q), r)
}
fn monic(k: &K, a: P) -> P {
    if a.is_empty() {
        a
    } else {
        ps(k, &a, &ki(k, a.last().unwrap()))
    }
}
fn pg(k: &K, mut a: P, mut b: P) -> P {
    while !b.is_empty() {
        let r = pd(k, &a, &b).1;
        a = b;
        b = r;
    }
    monic(k, a)
}
fn psm(k: &K, a: &[B], p: &[B]) -> P {
    let mut z = vec![vec![]; 2 * a.len().saturating_sub(1) + 1];
    for (i, c) in a.iter().enumerate() {
        z[2 * i] = k.sq(c);
    }
    pd(k, &pt(z), p).1
}
fn pmm(k: &K, a: &[B], b: &[B], p: &[B]) -> P {
    pd(k, &pm(k, a, b), p).1
}
fn pq(k: &K, a: &[B], p: &[B]) -> P {
    let mut z = a.to_vec();
    for _ in 0..k.d {
        z = psm(k, &z, p);
    }
    z
}
fn pirred(k: &K, p: &[B]) -> bool {
    let n = p.len() - 1;
    let x = pd(k, &[vec![], one()], p).1;
    let mut a = x.clone();
    for i in 1..=n {
        a = pq(k, &a, p);
        if i <= n / 2 && n % i == 0 && pg(k, p.to_vec(), pa(&a, &x)).len() != 1 {
            return false;
        }
    }
    a == x
}
fn random(k: &K, seed: &mut u64) -> B {
    let z = (0..k.d.div_ceil(64))
        .map(|_| {
            *seed ^= *seed << 13;
            *seed ^= *seed >> 7;
            *seed ^= *seed << 17;
            *seed
        })
        .collect();
    cut(z, k.d)
}
fn pedf(k: &K, p: P, d: usize, out: &mut Vec<P>, seed: &mut u64, attempts: &mut usize) {
    if p.len() == d + 1 {
        assert!(pirred(k, &p));
        out.push(p);
        return;
    }
    assert_eq!((p.len() - 1) % d, 0);
    for _ in 0..4096 {
        *attempts += 1;
        let a = pt((0..p.len() - 1).map(|_| random(k, seed)).collect());
        let mut tr = vec![];
        let mut z = a;
        for _ in 0..k.d * d {
            tr = pa(&tr, &z);
            z = psm(k, &z, &p);
        }
        let g = pg(k, p.clone(), tr);
        if g.len() > 1 && g != p {
            let (q, r) = pd(k, &p, &g);
            assert!(r.is_empty());
            pedf(k, g, d, out, seed, attempts);
            pedf(k, q, d, out, seed, attempts);
            return;
        }
    }
    panic!("residue-field splitting cap");
}
fn pfactor(k: &K, p: P, seed: &mut u64) -> (Vec<P>, usize) {
    let p = monic(k, p);
    let mut deriv = vec![vec![]; p.len().saturating_sub(1)];
    for (i, c) in p.iter().enumerate() {
        if i % 2 == 1 {
            deriv[i - 1] = c.clone();
        }
    }
    assert_eq!(
        pg(k, p.clone(), pt(deriv)).len(),
        1,
        "simple division polynomial"
    );
    let mut r = p.clone();
    let mut a = vec![vec![], one()];
    let mut d = 1;
    let mut out = vec![];
    let mut attempts = 0;
    while r.len() > 1 && 2 * d <= r.len() - 1 {
        a = pq(k, &a, &r);
        let g = pg(k, r.clone(), pa(&a, &[vec![], one()]));
        if g.len() > 1 {
            let (q, rem) = pd(k, &r, &g);
            assert!(rem.is_empty());
            r = q;
            pedf(k, g, d, &mut out, seed, &mut attempts);
            if r.len() <= 1 {
                break;
            }
            a = pd(k, &a, &r).1;
        }
        d += 1;
    }
    if r.len() > 1 {
        assert!(pirred(k, &r));
        out.push(r);
    }
    let mut check = vec![one()];
    for f in &out {
        check = pm(k, &check, f);
    }
    assert_eq!(check, p);
    out.sort_by_key(|p| p.iter().map(|x| hex(x)).collect::<Vec<_>>());
    (out, attempts)
}
fn proots(k: &K, p: P, seed: &mut u64) -> Vec<B> {
    if p.len() <= 1 {
        return vec![];
    }
    let x = vec![vec![], one()];
    let g = pg(k, p.clone(), pa(&pq(k, &x, &p), &x));
    if g.len() <= 1 {
        return vec![];
    }
    let mut fs = vec![];
    let mut attempts = 0;
    pedf(k, g, 1, &mut fs, seed, &mut attempts);
    fs.into_iter()
        .map(|f| km(k, &f[0], &ki(k, &f[1])))
        .collect()
}
fn kp(k: &K, a: &[u64], mut n: usize) -> B {
    let mut z = one();
    let mut a = a.to_vec();
    while n > 0 {
        if n & 1 != 0 {
            z = km(k, &z, &a);
        }
        n >>= 1;
        if n > 0 {
            a = k.sq(&a);
        }
    }
    z
}
fn triple(k: &K, p: &(B, B)) -> [B; 3] {
    let (x, y) = p;
    let d = add(&add(&k.sq(x), x), &one());
    let eta = add(&add(&km(k, &d, y), x), &one());
    let t = km(k, &kp(k, x, 4), &ki(k, &eta));
    let aa = |z: &B| kp(k, &add(&add(&k.sq(z), z), &one()), 4);
    let a = aa(&t);
    let b = aa(&eta);
    let j7 = km(k, &a, &ki(k, &t));
    assert_eq!(j7, km(k, &b, &ki(k, &kp(k, &eta, 7))));
    [
        km(k, &a, &ki(k, &kp(k, &t, 7))),
        j7,
        km(k, &b, &ki(k, &eta)),
    ]
}
fn valid(k: &K, p: &(B, B)) -> bool {
    let (x, y) = p;
    add(
        &add(&k.sq(y), &km(k, x, y)),
        &add(&add(&kp(k, x, 3), &k.sq(x)), &one()),
    )
    .is_empty()
}
fn param(k: &K, j0: &B, j1: &B, seed: &mut u64) -> Vec<B> {
    let mut a = vec![vec![]; 9];
    for i in [0, 4, 8] {
        a[i] = one();
    }
    a[7] = j0.clone();
    let mut b = vec![vec![]; 9];
    for i in [0, 4, 8] {
        b[i] = one();
    }
    b[1] = j1.clone();
    proots(k, pg(k, a, b), seed)
}
fn phi(k: &K, terms: &[(usize, usize)], j: &B) -> P {
    let mut p = vec![vec![]; 13];
    let powers: Vec<_> = (0..13).map(|i| kp(k, j, i)).collect();
    for (a, b) in terms {
        p[*b] = add(&p[*b], &powers[*a]);
    }
    pt(p)
}
fn common(k: &K, terms: &[(usize, usize)], a: &B, b: &B, seed: &mut u64) -> Vec<B> {
    proots(k, pg(k, phi(k, terms, a), phi(k, terms, b)), seed)
}
fn points_json(p: &(B, B)) -> serde_json::Value {
    json!([hex(&p.0), hex(&p.1)])
}
fn decodepoint(p: &serde_json::Value) -> (B, B) {
    (parse(p[0].as_str().unwrap()), parse(p[1].as_str().unwrap()))
}
fn ppow(k: &K, a: &[B], n: usize) -> P {
    (0..n).fold(vec![one()], |z, _| pm(k, &z, a))
}
fn division(k: &K, b: &B, n: usize) -> P {
    let mut ds = vec![P::new(); n + 1];
    ds[1] = vec![one()];
    ds[2] = vec![vec![], one()];
    ds[3] = vec![b.clone(), vec![], vec![], one(), one()];
    ds[4] = vec![vec![], vec![], b.clone(), vec![], vec![], vec![], one()];
    for i in 5..=n {
        let m = i / 2;
        ds[i] = if i % 2 == 1 {
            pa(
                &pm(k, &ds[m + 2], &ppow(k, &ds[m], 3)),
                &pm(k, &ds[m - 1], &ppow(k, &ds[m + 1], 3)),
            )
        } else {
            let v = pm(
                k,
                &ds[m],
                &pa(
                    &pm(k, &ds[m + 2], &ppow(k, &ds[m - 1], 2)),
                    &pm(k, &ds[m - 2], &ppow(k, &ds[m + 1], 2)),
                ),
            );
            let (q, r) = pd(k, &v, &[vec![], one()]);
            assert!(r.is_empty());
            q
        };
    }
    ds[n].clone()
}
fn pinv(k: &K, a: &[B], p: &[B]) -> P {
    let (mut r0, mut r1) = (p.to_vec(), pd(k, a, p).1);
    let (mut t0, mut t1) = (vec![], vec![one()]);
    while !r1.is_empty() {
        let (q, r) = pd(k, &r0, &r1);
        let t = pa(&t0, &pm(k, &q, &t1));
        r0 = r1;
        r1 = r;
        t0 = t1;
        t1 = t;
    }
    assert_eq!(r0.len(), 1, "coprime isogeny kernel denominator");
    pd(k, &ps(k, &t0, &ki(k, &r0[0])), p).1
}
fn pevalmod(k: &K, a: &[B], x: &[B], p: &[B]) -> P {
    let mut z = vec![];
    for c in a.iter().rev() {
        z = pa(&pmm(k, &z, x, p), &[c.clone()]);
    }
    z
}
fn deriv(a: &[B]) -> P {
    let mut p = vec![vec![]; a.len().saturating_sub(1)];
    for (i, c) in a.iter().enumerate() {
        if i % 2 == 1 {
            p[i - 1] = c.clone();
        }
    }
    pt(p)
}
fn velux(k: &K, kernel: &[B], p: &[B]) -> P {
    let z = pmm(k, &deriv(kernel), &pinv(k, kernel, p), p);
    let x = vec![vec![], one()];
    pa(
        &pa(&x, &pmm(k, &x, &z, p)),
        &pmm(k, &[vec![], vec![], one()], &pmm(k, &z, &z, p), p),
    )
}
fn normpoly(k: &K, r: &[B], p: &[B]) -> P {
    let n = p.len() - 1;
    assert!(n <= 5);
    let mut matrix = vec![vec![vec![]; n]; n];
    let mut x = vec![one()];
    for j in 0..n {
        let v = pmm(k, r, &x, p);
        for (i, c) in v.into_iter().enumerate() {
            matrix[i][j] = c;
        }
        x = pmm(k, &x, &[vec![], one()], p);
    }
    fn rec(k: &K, m: &Vec<Vec<B>>, i: usize, used: u32, v: P, z: &mut P) {
        if i == m.len() {
            *z = pa(z, &v);
            return;
        }
        for j in 0..m.len() {
            if used >> j & 1 != 0 {
                continue;
            }
            let f = if i == j {
                vec![m[i][j].clone(), one()]
            } else {
                pt(vec![m[i][j].clone()])
            };
            if f.is_empty() {
                continue;
            }
            rec(k, m, i + 1, used | 1 << j, pm(k, &v, &f), z);
        }
    }
    let mut z = vec![];
    rec(k, &matrix, 0, 0, vec![one()], &mut z);
    assert_eq!(z.len(), n + 1);
    assert!(is_one(z.last().unwrap()));
    z
}
fn quotient_b(k: &K, b: &B, kernel: &[B]) -> B {
    let c = &kernel[kernel.len() - 2];
    add(&add(b, c), &k.sq(c))
}
fn doubling(k: &K, b: &B, p: &[B]) -> P {
    let x = vec![vec![], one()];
    let xi = pinv(k, &x, p);
    pa(
        &pd(k, &[vec![], vec![], one()], p).1,
        &ps(k, &pmm(k, &xi, &xi, p), b),
    )
}
fn kernels(k: &K, b: &B, ell: usize, seed: &mut u64) -> (Vec<P>, serde_json::Value) {
    let divp = division(k, b, ell);
    let (fs, attempts) = pfactor(k, divp.clone(), seed);
    let s = (ell - 1) / 2;
    let mut possible: BTreeMap<Vec<String>, P> = BTreeMap::new();
    for f in &fs {
        if f.len() == s + 1 {
            possible.insert(f.iter().map(|x| hex(x)).collect(), f.clone());
        } else if f.len() == 2 {
            let x = km(k, &f[0], &ki(k, &f[1]));
            let mut v = x.clone();
            let mut p = vec![one()];
            for _ in 0..s {
                assert!(!v.is_empty());
                p = pm(k, &p, &[v.clone(), one()]);
                v = add(&k.sq(&v), &km(k, b, &ki(k, &k.sq(&v))));
            }
            if v == x {
                possible.insert(p.iter().map(|x| hex(x)).collect(), p);
            }
        }
    }
    let mut out = vec![];
    let mut rejected = 0;
    for p in possible.into_values() {
        if !pd(k, &divp, &p).1.is_empty() || !pevalmod(k, &p, &doubling(k, b, &p), &p).is_empty() {
            rejected += 1;
            continue;
        }
        out.push(p);
    }
    let mut hist = BTreeMap::new();
    for f in &fs {
        *hist.entry(f.len() - 1).or_insert(0usize) += 1;
    }
    let rec = json!({"ell":ell,"division_degree":divp.len()-1,"irreducible_factor_degrees":hist,"split_attempts":attempts,"factor_product_checked":true,"rational_cyclic_kernels":out.len(),"rejected_non_kernel_candidates":rejected,"kernel_polynomials":out.iter().map(|p|p.iter().map(|x|hex(x)).collect::<Vec<_>>()).collect::<Vec<_>>()});
    (out, rec)
}
fn choose(k: &K, b: &B, kernels: &[P], j: &B) -> P {
    let target = ki(k, j);
    let out: Vec<_> = kernels
        .iter()
        .filter(|p| quotient_b(k, b, p) == target)
        .collect();
    assert_eq!(out.len(), 1, "unique ordinary kernel for requested j");
    out[0].clone()
}
fn transported(k: &K, b: &B, b7: &B, k1: &[B], k2: &[B], kernel11: &[B]) -> [B; 3] {
    let b0 = quotient_b(k, b, kernel11);
    let tk7 = normpoly(k, &velux(k, kernel11, k1), k1);
    let tk11 = normpoly(k, &velux(k, k1, kernel11), kernel11);
    let b70 = quotient_b(k, b7, &tk11);
    let b71 = quotient_b(k, &b0, &tk7);
    assert_eq!(b70, b71, "exact 7/11 commuting square");
    let tk2 = normpoly(k, &velux(k, &tk11, k2), k2);
    let b49 = quotient_b(k, &b70, &tk2);
    [ki(k, &b0), ki(k, &b70), ki(k, &b49)]
}
pub fn fibers(dir: &Path) {
    REDUCTION_LIMIT.store(1_000_000_000, Ordering::Relaxed);
    let input = crate::round04::read(&dir.join("o15-ordinary-middle-candidates.json"));
    let mut seed = 20261020u64;
    let mut records = vec![];
    let mut degrees = [0usize; 2];
    let mut pointcounts = [0usize; 2];
    for row in input["records"].as_array().unwrap() {
        let fi = row["function_index"].as_u64().unwrap() as usize;
        let pi = row["pushed_point_index"].as_u64().unwrap();
        let k = K::new(parse(row["field_modulus_bits"].as_str().unwrap()));
        println!("O16:start fn{fi},point{pi},field degree{}", k.d);
        let mut successes = vec![];
        for cand in row["middle_candidates"].as_array().unwrap() {
            let jm: Vec<B> = cand["middle_j"]
                .as_array()
                .unwrap()
                .iter()
                .map(|s| parse(s.as_str().unwrap()))
                .collect();
            let js: Vec<B> = row["source_target_j"]
                .as_array()
                .unwrap()
                .iter()
                .map(|s| parse(s.as_str().unwrap()))
                .collect();
            let jt: Vec<B> = cand["target_j"]
                .as_array()
                .unwrap()
                .iter()
                .map(|s| parse(s.as_str().unwrap()))
                .collect();
            let b = ki(&k, &jm[0]);
            let b7 = ki(&k, &jm[1]);
            let (ks1, r7a) = kernels(&k, &b, 7, &mut seed);
            let (ks2, r7b) = kernels(&k, &b7, 7, &mut seed);
            let k1 = choose(&k, &b, &ks1, &jm[1]);
            let k2 = choose(&k, &b7, &ks2, &jm[2]);
            assert_ne!(jm[0], jm[2], "the second 7-isogeny is not the dual of the first: a dual chain returns to the same j");
            let (ks11, r11) = kernels(&k, &b, 11, &mut seed);
            let kp = choose(&k, &b, &ks11, &js[0]);
            let kq = choose(&k, &b, &ks11, &jt[0]);
            assert_ne!(kp, kq);
            let gotp = transported(&k, &b, &b7, &k1, &k2, &kp);
            let gotq = transported(&k, &b, &b7, &k1, &k2, &kq);
            assert_eq!(gotp.to_vec(), js);
            assert_eq!(gotq.to_vec(), jt);
            let (fs, attempts) = pfactor(&k, kp.clone(), &mut seed);
            let mut points = vec![];
            for f in &fs {
                let d = k.d * (f.len() - 1);
                assert!(
                    [1, 5].contains(&(f.len() - 1)),
                    "Frob on F11^*/sign has order dividing5"
                );
                degrees[fi] += d;
                pointcounts[fi] += 1;
                points.push(json!({"base_field_modulus_bits":hex(&k.p),"extension_polynomial_coefficients":f.iter().map(|x|hex(x)).collect::<Vec<_>>(),"closed_degree":d,"multiplicity":1,"interpretation":"t=x(P11);Q11 lies in the second kernel and is determined by the fixed Weil pairing,mod simultaneous sign","rational_cyclic49_kernels":[k1.iter().map(|x|hex(x)).collect::<Vec<_>>(),k2.iter().map(|x|hex(x)).collect::<Vec<_>>()]}));
            }
            successes.push(json!({"middle":cand["middle"],"middle_j":cand["middle_j"],"target_label":cand["target_label"],"first11_kernel":kp.iter().map(|x|hex(x)).collect::<Vec<_>>(),"second11_kernel":kq.iter().map(|x|hex(x)).collect::<Vec<_>>(),"division_factorizations":[r7a,r7b,r11],"transported_target_triples":[gotp.iter().map(|x|hex(x)).collect::<Vec<_>>(),gotq.iter().map(|x|hex(x)).collect::<Vec<_>>()],"four_commuting_square_checks":true,"fiber_factor_attempts":attempts,"source_closed_points":points}));
        }
        assert_eq!(successes.len(), 1);
        println!(
            "O16:fn{fi},point{pi},transport verified,source degrees {:?}",
            successes[0]["source_closed_points"]
                .as_array()
                .unwrap()
                .iter()
                .map(|p| p["closed_degree"].clone())
                .collect::<Vec<_>>()
        );
        let record = json!({"function_index":fi,"pushed_point_index":pi,"field_degree":k.d,"verified_lifts":successes});
        round02::save(dir, &format!("o16-fn{fi}-point{pi}.json"), &record);
        records.push(record);
    }
    assert_eq!(degrees, [500, 580]);
    round02::save(
        dir,
        "o16-ordinary-source-fibers.json",
        &json!({"schema":"koblitz.round13/o16-v1","seed":20261020,"records":records,"ordinary_degrees":degrees,"closed_point_counts":pointcounts,"polynomial_reduction_steps":REDUCTIONS.load(Ordering::Relaxed),"full_support_under_written_norm_and_moduli_bridge":true,"projected_images_evaluated":false,"scope":"exact full-level fiber algebras,local multiplicity1 and degree exhaustion;coordinate/pairing identification written,not a full Lean modular-curve formalization"}),
    );
}
pub fn middles(dir: &Path) {
    let input = crate::round04::read(&dir.join("o14-pushed-closed-supports.json"));
    let base = crate::round04::read(&dir.join("m1-modular-base.json"));
    let terms: Vec<(usize, usize)> =
        serde_json::from_value(base["mod2_phi11_terms"].clone()).unwrap();
    let mut seed = 20261020u64;
    let mut records = vec![];
    for (fi, row) in input["records"].as_array().unwrap().iter().enumerate() {
        for (pi, p) in row["points"].as_array().unwrap().iter().enumerate() {
            if p["stratum"] != "ordinary" {
                continue;
            }
            let k = K::new(parse(p["x_minpoly_bits"].as_str().unwrap()));
            let x = k.red(&[2]);
            let y = parse(p["y_residue_bits"].as_str().unwrap());
            let source = (x.clone(), y.clone());
            assert!(valid(&k, &source));
            let js = triple(&k, &source);
            let mut targets = vec![("negative", (x.clone(), add(&y, &x)))];
            if fi == 1 {
                let u = ki(&k, &x);
                let v = km(&k, &add(&add(&add(&y, &k.sq(&x)), &x), &one()), &k.sq(&u));
                targets = vec![
                    ("P+T", (u.clone(), v.clone())),
                    ("T-P", (u.clone(), add(&v, &u))),
                ];
            }
            let mut attempts = vec![];
            let mut candidates = vec![];
            for (label, target) in targets {
                assert!(valid(&k, &target));
                let jt = triple(&k, &target);
                let commons: Vec<_> = (0..3)
                    .map(|i| common(&k, &terms, &js[i], &jt[i], &mut seed))
                    .collect();
                let mut count = 0;
                for j0 in &commons[0] {
                    assert!(!j0.is_empty());
                    for j7 in &commons[1] {
                        let t0s = param(&k, j0, j7, &mut seed);
                        if t0s.is_empty() {
                            continue;
                        }
                        for j49 in &commons[2] {
                            let t7s = param(&k, j7, j49, &mut seed);
                            for t0 in &t0s {
                                for t7 in &t7s {
                                    count += 1;
                                    assert!(count <= 4096);
                                    let mut xx = km(&k, t0, t7);
                                    for _ in 0..k.d - 2 {
                                        xx = k.sq(&xx);
                                    }
                                    let den = add(&add(&k.sq(&xx), &xx), &one());
                                    if den.is_empty() {
                                        continue;
                                    }
                                    let yy = km(&k, &add(&add(t7, &xx), &one()), &ki(&k, &den));
                                    let middle = (xx, yy);
                                    assert!(valid(&k, &middle));
                                    let jmid = [j0.clone(), j7.clone(), j49.clone()];
                                    assert_eq!(triple(&k, &middle), jmid);
                                    candidates.push(json!({"target_label":label,"target":points_json(&target),"target_j":jt.iter().map(|x|hex(x)).collect::<Vec<_>>(),"middle":points_json(&middle),"middle_j":jmid.iter().map(|x|hex(x)).collect::<Vec<_>>()}));
                                }
                            }
                        }
                    }
                }
                attempts.push(json!({"target_label":label,"common_j_counts":commons.iter().map(Vec::len).collect::<Vec<_>>(),"compatible_triples":count}));
            }
            println!(
                "O15:fn{fi},point{pi},degree{},{} middle candidates",
                k.d,
                candidates.len()
            );
            records.push(json!({"function_index":fi,"pushed_point_index":pi,"field_modulus_bits":hex(&k.p),"field_degree":k.d,"source_target":points_json(&source),"source_target_j":js.iter().map(|x|hex(x)).collect::<Vec<_>>(),"attempts":attempts,"middle_candidates":candidates,"actual_transport_verified":false}));
        }
    }
    round02::save(
        dir,
        "o15-ordinary-middle-candidates.json",
        &json!({"schema":"koblitz.round13/o15-v1","seed":20261020,"records":records,"polynomial_reduction_steps":REDUCTIONS.load(Ordering::Relaxed),"scope":"complete K-rational common-neighbor/triple candidates;actual level49 transport required before source fibers"}),
    );
}
pub fn images(dir: &Path) {
    let fibers = crate::round04::read(&dir.join("o16-ordinary-source-fibers.json"));
    let support = crate::round04::read(&dir.join("o14-pushed-closed-supports.json"));
    let lambdas: Vec<usize> = (0..5).map(|i| (1usize << i) % 11).collect();
    let mut table = vec![];
    let mut signs = vec![];
    let mut inverses = vec![];
    for u in 1usize..=5 {
        let inverse = (1..11).find(|v| u * u * v % 11 == 1).unwrap();
        inverses.push(inverse);
        let mut row = vec![];
        let mut negs = vec![];
        for (i, a) in lambdas.iter().enumerate() {
            let b = a * inverse % 11;
            let (j, neg) = (0..5)
                .find_map(|j| {
                    if lambdas[j] == b {
                        Some((j, false))
                    } else if 11 - lambdas[j] == b {
                        Some((j, true))
                    } else {
                        None
                    }
                })
                .unwrap();
            assert_eq!(
                neg,
                (i + j) % 2 == 1,
                "signed odd tuple transforms by the same permutation"
            );
            row.push(j);
            negs.push(neg);
        }
        table.push(row);
        signs.push(negs);
    }
    for i in 0..5 {
        assert_eq!(
            table.iter().map(|r| r[i]).collect::<BTreeSet<_>>(),
            (0..5).collect()
        );
    }
    let mut columns = vec![];
    let mut degrees = [vec![], vec![]];
    for r in fibers["records"].as_array().unwrap() {
        let fi = r["function_index"].as_u64().unwrap() as usize;
        let lift = &r["verified_lifts"][0];
        let pts = lift["source_closed_points"].as_array().unwrap();
        assert_eq!(pts.len(), 1);
        let p = &pts[0];
        let d = p["closed_degree"].as_u64().unwrap();
        assert_eq!(d, 5 * r["field_degree"].as_u64().unwrap());
        assert_eq!(
            p["extension_polynomial_coefficients"]
                .as_array()
                .unwrap()
                .len(),
            6
        );
        degrees[fi].push(d);
        columns.push(json!({"function_index":fi,"pushed_point_index":r["pushed_point_index"],"closed_degree":d,"source_fiber_is_one_closed_point":true,"even_projected_image":"zero","odd_projected_image":"zero","reason":"the entire degree-five scalar fiber is this closed point;deck norm of each augmented signed tuple is zero before the retained integral Frobenius/transfer maps"}));
    }
    round02::save(
        dir,
        "o17-ordinary-projected-images.json",
        &json!({"schema":"koblitz.round13/o17-v1","deck_representatives":[1,2,3,4,5],"line_parameters":lambdas,"inverse_deck_squares":inverses,"permutation_table":table,"raw_negative_flags":signs,"signed_odd_corrections_all_positive":true,"each_column_is_permutation":true,"columns":columns,"new_nonzero_11_primary_columns":0,"pointwise_projection_coordinates_computed":false,"scope":"integral deck-norm vanishing followed by the retained linear projections;not a new pointwise projection run"}),
    );
    let mut s=String::from("import KoblitzRound13\nset_option maxHeartbeats 0\nset_option maxRecDepth 100000\nnamespace KoblitzRound13Data\n");
    s.push_str(&format!(
        "def deck : Fin 5 → Fin 5 → Fin 5 := ![{}]\n",
        table
            .iter()
            .map(|r| format!(
                "![{}]",
                r.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            ))
            .collect::<Vec<_>>()
            .join(",")
    ));
    s.push_str(&format!("def lambda : Array Nat := #[{}]\ndef inverseSquares : Array Nat := #[{}]\ndef negative : Array (Array Bool) := #[{}]\n",lambdas.iter().map(ToString::to_string).collect::<Vec<_>>().join(","),inverses.iter().map(ToString::to_string).collect::<Vec<_>>().join(","),signs.iter().map(|r|format!("#[{}]",r.iter().map(|v|v.to_string()).collect::<Vec<_>>().join(","))).collect::<Vec<_>>().join(",")));
    s.push_str(r#"def deckCheck : Bool := (List.finRange 5).all fun r =>
 (((r.val+1)^2*inverseSquares[r.val]!)%11==1) &&
 (List.finRange 5).all fun i =>
  ((lambda[i.val]!*inverseSquares[r.val]!)%11 ==
   if (negative[r.val]!)[i.val]! then 11-lambda[(deck r i).val]! else lambda[(deck r i).val]!) &&
  ((negative[r.val]!)[i.val]! == ((i.val+(deck r i).val)%2==1))
theorem actual_deck_table : deckCheck=true := by decide +kernel
theorem uniform_deck_norm {A : Type*} [AddCommGroup A] (v : Fin 5 → A) (i : Fin 5) :
 (∑ r,v (deck r i))=∑ j,v j := by
 fin_cases i <;> simp [deck,Fin.sum_univ_succ] <;> abel
theorem actual_deck_augmentation_zero {A : Type*} [AddCommGroup A] (v : Fin 5 → A) (i : Fin 5) :
 (∑ r,((5:ℕ) • v (deck r i)-(∑ j,v j)))=0 := by
 rw [Finset.sum_sub_distrib,←Finset.smul_sum,uniform_deck_norm]
 simp
theorem projected_deck_zero {A B : Type*} [AddCommGroup A] [AddCommGroup B]
 (f : A →+ B) (v : Fin 5 → A) (i : Fin 5) :
 f (∑ r,((5:ℕ) • v (deck r i)-(∑ j,v j)))=0 := by
 rw [actual_deck_augmentation_zero,map_zero]
def pmodAux : Nat → Nat → Nat → Nat
 | 0,a,_ => a
 | n+1,a,p => if a=0 ∨ a.log2<p.log2 then a else pmodAux n (Nat.xor a (p<<<(a.log2-p.log2))) p
def pmod (a p : Nat) := pmodAux (a.log2+1) a p
def pmul (a b : Nat) : Nat := (List.range (b.log2+1)).foldl (fun z i => if b.testBit i then Nat.xor z (a<<<i) else z) 0
def pgcdAux : Nat → Nat → Nat → Nat
 | 0,a,_ => a
 | n+1,a,b => if b=0 then a else pgcdAux n b (pmod a b)
def pgcd (a b : Nat) := pgcdAux (a.log2+b.log2+2) a b
def frob (p : Nat) : Nat → Nat
 | 0 => 2
 | n+1 => pmod (pmul (frob p n) (frob p n)) p
def rabin (p d : Nat) (primes : List Nat) : Bool :=
 (p.log2==d) && (frob p d==2) && (primes.all fun l => pgcd p (Nat.xor (frob p (d/l)) 2)==1)
"#);
    let mut polynomials = BTreeMap::new();
    for r in support["records"].as_array().unwrap() {
        for p in r["points"].as_array().unwrap() {
            if p["stratum"] == "ordinary" {
                polynomials.insert(
                    p["x_minpoly_bits"].as_str().unwrap().to_string(),
                    p["closed_degree"].as_u64().unwrap(),
                );
            }
        }
    }
    let mut names = vec![
        "actual_deck_table".to_string(),
        "uniform_deck_norm".into(),
        "actual_deck_augmentation_zero".into(),
        "projected_deck_zero".into(),
    ];
    for (i, (p, d)) in polynomials.iter().enumerate() {
        let decimal = BigUint::parse_bytes(p.as_bytes(), 16)
            .unwrap()
            .to_str_radix(10);
        let primes = if *d == 50 { "[2,5]" } else { "[29]" };
        let name = format!("ordinary_base_polynomial_{i}_Rabin");
        s.push_str(&format!(
            "theorem {name} : rabin {decimal} {d} {primes}=true := by decide +kernel\n"
        ));
        names.push(name);
    }
    s.push_str(&format!("theorem source_degree_certificate : ({} : List Nat).sum=500 ∧ ({} : List Nat).sum=580 := by decide +kernel\n",format!("[{}]",degrees[0].iter().map(ToString::to_string).collect::<Vec<_>>().join(",")),format!("[{}]",degrees[1].iter().map(ToString::to_string).collect::<Vec<_>>().join(","))));
    names.push("source_degree_certificate".into());
    s.push_str("end KoblitzRound13Data\n");
    for n in &names {
        s.push_str(&format!("#print axioms KoblitzRound13Data.{n}\n"));
    }
    let path = dir.join("KoblitzRound13Data.lean");
    assert!(!path.exists());
    std::fs::write(path, s).unwrap();
    println!("O17:all6 closed images zero by actual C5 deck permutations;8 Lean data/structural certificates emitted");
}

/// Parsed verification uses the emitted towers rather than rerunning factor searches.
pub fn verify(dir: &Path) {
    REDUCTION_LIMIT.store(1_000_000_000, Ordering::Relaxed);
    let norms = crate::round04::read(&dir.join("o13-cyclic121-Hecke-norms.json"));
    let supports = crate::round04::read(&dir.join("o14-pushed-closed-supports.json"));
    let middle = crate::round04::read(&dir.join("o15-ordinary-middle-candidates.json"));
    let fibers = crate::round04::read(&dir.join("o16-ordinary-source-fibers.json"));
    let (u, v) = xy(PREC);
    let mut records = vec![];
    let mut totals = [0usize; 2];
    for (fi, r) in norms["records"].as_array().unwrap().iter().enumerate() {
        let a = parse(r["A_bits"].as_str().unwrap());
        let b = parse(r["B_bits"].as_str().unwrap());
        assert_eq!(
            recover(
                &parse(r["q_series_normalized_bits"].as_str().unwrap()),
                r["pole_bound"].as_u64().unwrap() as usize,
                &u,
                &v,
                PREC
            ),
            *r
        );
        let (zs, _) = zero_divisor(&a, &b);
        let out = &supports["records"][fi]["points"];
        assert_eq!(zs.len(), out.as_array().unwrap().len());
        for (z, p) in zs.iter().zip(out.as_array().unwrap()) {
            assert_eq!(hex(&z.p), p["x_minpoly_bits"]);
            assert_eq!(
                z.y.as_ref().map(|y| hex(y)),
                p["y_residue_bits"].as_str().map(str::to_owned)
            );
            assert_eq!(z.mult as u64, p["multiplicity"]);
            assert_eq!(z.degree as u64, p["closed_degree"]);
            assert_eq!(z.class, p["stratum"]);
        }
    }
    for r in fibers["records"].as_array().unwrap() {
        let fi = r["function_index"].as_u64().unwrap() as usize;
        let pi = r["pushed_point_index"].as_u64().unwrap() as usize;
        let base = &supports["records"][fi]["points"][pi];
        let k = K::new(parse(base["x_minpoly_bits"].as_str().unwrap()));
        let xp = k.red(&[2]);
        let yp = parse(base["y_residue_bits"].as_str().unwrap());
        assert!(valid(&k, &(xp.clone(), yp.clone())));
        assert_eq!(base["multiplicity"], 1);
        let input = middle["records"]
            .as_array()
            .unwrap()
            .iter()
            .find(|a| {
                a["function_index"] == r["function_index"]
                    && a["pushed_point_index"] == r["pushed_point_index"]
            })
            .unwrap();
        assert_eq!(decodepoint(&input["source_target"]), (xp, yp));
        let cand = &input["middle_candidates"][0];
        let lift = &r["verified_lifts"][0];
        let mid = decodepoint(&lift["middle"]);
        assert!(valid(&k, &mid));
        let jm = triple(&k, &mid);
        assert_ne!(jm[0], jm[2], "distinct endpoint j-invariants certify nonbacktracking of this 49-chain");
        let b = ki(&k, &jm[0]);
        let b7 = ki(&k, &jm[1]);
        let dec = |z: &serde_json::Value| {
            z.as_array()
                .unwrap()
                .iter()
                .map(|x| parse(x.as_str().unwrap()))
                .collect::<P>()
        };
        let kp = dec(&lift["first11_kernel"]);
        let kq = dec(&lift["second11_kernel"]);
        assert_ne!(kp, kq);
        let points = lift["source_closed_points"].as_array().unwrap();
        assert_eq!(points.len(), 1);
        let p = &points[0];
        let k1 = dec(&p["rational_cyclic49_kernels"][0]);
        let k2 = dec(&p["rational_cyclic49_kernels"][1]);
        for (bb, ell, ker) in [(&b, 7, &k1), (&b7, 7, &k2), (&b, 11, &kp), (&b, 11, &kq)] {
            assert_eq!(ker.len(), (ell + 1) / 2);
            assert!(pd(&k, &division(&k, bb, ell), ker).1.is_empty());
            assert!(pevalmod(&k, ker, &doubling(&k, bb, ker), ker).is_empty());
        }
        assert_eq!(quotient_b(&k, &b, &k1), b7);
        assert_eq!(quotient_b(&k, &b7, &k2), ki(&k, &jm[2]));
        let js = triple(&k, &decodepoint(&input["source_target"]));
        let jt = triple(&k, &decodepoint(&cand["target"]));
        assert_eq!(transported(&k, &b, &b7, &k1, &k2, &kp), js);
        assert_eq!(transported(&k, &b, &b7, &k1, &k2, &kq), jt);
        let extension = dec(&p["extension_polynomial_coefficients"]);
        assert_eq!(extension, kp);
        assert_eq!(extension.len(), 6);
        assert!(pirred(&k, &extension));
        let d = 5 * k.d;
        assert_eq!(p["closed_degree"], d as u64);
        assert_eq!(p["multiplicity"], 1);
        totals[fi] += d;
        records.push(json!({"function_index":fi,"pushed_point_index":pi,"base_degree":k.d,"closed_degree":d,"simple":true,"irreducible_degree5_fiber":true,"four_kernel_memberships_and_doubling_checks":true,"four_commuting_squares":true}));
    }
    assert_eq!(totals, [500, 580]);
    round02::save(
        dir,
        "o18-parsed-function-and-fiber-check.json",
        &json!({"schema":"koblitz.round13/o18-v1","functions_q_coefficients_checked":640,"two_pushed_divisors_reconstructed":true,"records":records,"ordinary_source_degrees":totals,"polynomial_reduction_steps":REDUCTIONS.load(Ordering::Relaxed),"external_independent_implementation":false,"scope":"separate parsed verification sharing native field and isogeny primitives;global modular/pairing bridge remains written"}),
    );
    println!(
        "O18:parsed functions,closed divisors,all6 source fields and24 commuting squares passed"
    );
}

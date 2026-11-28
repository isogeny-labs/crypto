//! Native correctness port of the complete historical cover path.
//! This module emits no timings and makes no performance claim.
use super::*;

type Poly = Vec<F>;
type Divisor = (Poly, Poly);
type Row = Vec<(usize, i64)>;
const G: Point = Some(([167, 623, 69], [1004, 699, 413]));
const CANDIDATE_LIMIT: usize = 1_000_000;

fn fp(x: i64) -> F {
    [x.rem_euclid(P), 0, 0]
}
fn trim(mut a: Poly) -> Poly {
    while a.last() == Some(&ZERO) {
        a.pop();
    }
    a
}
fn pc(x: F) -> Poly {
    trim(vec![x])
}
fn pa(a: &[F], b: &[F]) -> Poly {
    trim(
        (0..a.len().max(b.len()))
            .map(|i| add(*a.get(i).unwrap_or(&ZERO), *b.get(i).unwrap_or(&ZERO)))
            .collect(),
    )
}
fn ps(a: &[F], c: F) -> Poly {
    trim(a.iter().map(|&x| mul(x, c)).collect())
}
fn pn(a: &[F]) -> Poly {
    a.iter().map(|&x| neg(x)).collect()
}
fn pd(a: &[F], b: &[F]) -> Poly {
    pa(a, &pn(b))
}
fn pm(a: &[F], b: &[F]) -> Poly {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut r = vec![ZERO; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            r[i + j] = add(r[i + j], mul(x, y));
        }
    }
    trim(r)
}
fn pdiv(a: &[F], b: &[F]) -> (Poly, Poly) {
    assert!(!b.is_empty(), "polynomial division by zero");
    let mut r = trim(a.to_vec());
    let mut q = vec![ZERO; (a.len() + 1).saturating_sub(b.len())];
    let inverse = inv(*b.last().unwrap());
    while !r.is_empty() && r.len() >= b.len() {
        let i = r.len() - b.len();
        let c = mul(*r.last().unwrap(), inverse);
        q[i] = c;
        for (j, &x) in b.iter().enumerate() {
            r[i + j] = sub(r[i + j], mul(c, x));
        }
        r = trim(r);
    }
    (trim(q), r)
}
fn rem(a: &[F], b: &[F]) -> Poly {
    pdiv(a, b).1
}
fn quotient(a: &[F], b: &[F]) -> Poly {
    let (q, r) = pdiv(a, b);
    assert!(r.is_empty(), "inexact polynomial quotient");
    q
}
fn monic(a: &[F]) -> Poly {
    assert!(!a.is_empty());
    ps(a, inv(*a.last().unwrap()))
}
fn egcd(mut a: Poly, mut b: Poly) -> (Poly, Poly, Poly) {
    let (mut s, mut t, mut ss, mut tt) = (vec![ONE], vec![], vec![], vec![ONE]);
    while !b.is_empty() {
        let (q, r) = pdiv(&a, &b);
        (a, b) = (b, r);
        (s, ss) = (ss.clone(), pd(&s, &pm(&q, &ss)));
        (t, tt) = (tt.clone(), pd(&t, &pm(&q, &tt)));
    }
    assert!(!a.is_empty(), "gcd(0,0)");
    let c = inv(*a.last().unwrap());
    (ps(&a, c), ps(&s, c), ps(&t, c))
}
fn pinv(a: &[F], modulus: &[F]) -> Result<Poly, String> {
    let (d, s, _) = egcd(a.to_vec(), modulus.to_vec());
    require(
        d == vec![ONE],
        "exceptional pullback: noninvertible polynomial",
    )?;
    Ok(rem(&s, modulus))
}
fn peval(a: &[F], x: F) -> F {
    a.iter().rev().fold(ZERO, |r, &c| add(mul(r, x), c))
}
fn derivative(a: &[F]) -> Poly {
    trim(
        a.iter()
            .enumerate()
            .skip(1)
            .map(|(i, &c)| scale(c, i as i64))
            .collect(),
    )
}

struct Jacobian {
    f: Poly,
}
impl Jacobian {
    fn zero(&self) -> Divisor {
        (vec![ONE], vec![])
    }
    fn valid(&self, d: &Divisor) -> bool {
        !d.0.is_empty()
            && d.0.last() == Some(&ONE)
            && d.1.len() < d.0.len()
            && rem(&pd(&self.f, &pm(&d.1, &d.1)), &d.0).is_empty()
    }
    fn reduce(&self, u: Poly, v: Poly) -> Divisor {
        let mut u = monic(&u);
        let mut v = rem(&v, &u);
        while u.len() > 4 {
            u = monic(&quotient(&pd(&self.f, &pm(&v, &v)), &u));
            v = rem(&pn(&v), &u);
        }
        let d = (u, v);
        assert!(self.valid(&d), "invalid reduced divisor");
        d
    }
    fn add(&self, a: &Divisor, b: &Divisor) -> Divisor {
        let (u1, v1) = a;
        let (u2, v2) = b;
        let (d1, e1, e2) = egcd(u1.clone(), u2.clone());
        let (d, c1, c2) = egcd(d1, pa(v1, v2));
        let (s1, s2) = (pm(&c1, &e1), pm(&c1, &e2));
        let u = quotient(&pm(u1, u2), &pm(&d, &d));
        let v = quotient(
            &pa(
                &pa(&pm(&pm(&s1, u1), v2), &pm(&pm(&s2, u2), v1)),
                &pm(&c2, &pa(&pm(v1, v2), &self.f)),
            ),
            &d,
        );
        self.reduce(u, v)
    }
    fn scalar(&self, mut n: u64, d: &Divisor) -> Divisor {
        let mut r = self.zero();
        let mut q = d.clone();
        while n != 0 {
            if n & 1 != 0 {
                r = self.add(&r, &q);
            }
            q = self.add(&q, &q);
            n >>= 1;
        }
        r
    }
}

struct Geometry {
    curve: Curve,
    j: Jacobian,
    n: Poly,
    quartic: Poly,
    a: F,
    b: F,
    c: F,
    fb: Vec<(i64, i64)>,
    images: Vec<Point>,
    gi: usize,
}
impl Geometry {
    fn new() -> Result<Self, String> {
        let n = vec![fp(65), ONE, ZERO, ONE];
        let quartic = vec![ONE, fp(-520), fp(-2), ZERO, ONE];
        let f = pm(&n, &quartic);
        require(
            egcd(f.clone(), derivative(&f)).0 == vec![ONE],
            "singular cover",
        )?;
        let a = [0, 1, 0];
        let (b, c) = (pow(a, P as u64), pow(a, (P * P) as u64));
        require(
            a != b && a != c && b != c && pow(c, P as u64) == a,
            "cubic Frobenius orbit",
        )?;
        let mut roots = vec![None; P as usize];
        for y in 0..P {
            roots[(y * y % P) as usize].get_or_insert(y);
        }
        let fb = (0..P)
            .filter_map(|x| roots[peval(&f, fp(x))[0] as usize].map(|y| (x, y)))
            .collect();
        let mut s = Self {
            curve: Curve::new(),
            j: Jacobian { f },
            n,
            quartic,
            a,
            b,
            c,
            fb,
            images: vec![],
            gi: 0,
        };
        s.identities()?;
        for &(x, y) in &s.fb {
            let image = s.cover(x, y)?;
            require(s.curve.contains(image), "factor-base image off curve")?;
            if let Some((xx, _)) = image {
                require(
                    xx == div(peval(&s.quartic, fp(x)), scale(peval(&s.n, fp(x)), 4)),
                    "cover x-coordinate identity",
                )?;
            }
            s.images.push(s.curve.scalar(4, image));
        }
        s.gi = s
            .images
            .iter()
            .position(Option::is_some)
            .ok_or("empty generator images")?;
        require(
            s.images[s.gi] == G,
            "generator normalization differs from archive",
        )?;
        require(s.curve.scalar(ELL, G).is_none(), "generator order")?;
        Ok(s)
    }
    fn identities(&self) -> Result<(), String> {
        // Joux--Vitse Appendix A.1: clear both stages' denominators.
        let x = vec![ZERO, ONE];
        let xg = pd(&x, &pc(self.c));
        let delta = mul(sub(self.a, self.c), sub(self.b, self.c));
        let znum = pa(&pd(&pm(&x, &x), &pc(mul(self.c, self.c))), &pc(delta));
        let ab = mul(self.a, self.b);
        let sab = add(self.a, self.b);
        let cleared = pm(
            &xg,
            &pm(
                &pd(&znum, &ps(&xg, sab)),
                &pd(&pm(&znum, &znum), &ps(&pm(&xg, &xg), scale(ab, 4))),
            ),
        );
        require(
            cleared == self.j.f,
            "bielliptic quotient polynomial identity",
        )?;
        let xn = pd(&pm(&x, &x), &pc(scale(ab, 4)));
        let xd = ps(&pd(&x, &pc(sab)), fp(4));
        let yn = pm(
            &pd(&x, &pc(scale(self.a, 2))),
            &pd(&x, &pc(scale(self.b, 2))),
        );
        require(
            pm(&yn, &yn)
                == pa(
                    &pa(&pm(&xn, &xn), &ps(&pm(&xn, &xd), self.curve.a2)),
                    &ps(&pm(&xd, &xd), self.curve.a4),
                ),
            "two-isogeny polynomial identity",
        )
    }
    fn cover(&self, x: i64, y: i64) -> Result<Point, String> {
        let (x, y) = (fp(x), fp(y));
        require(mul(y, y) == peval(&self.j.f, x), "input off cover")?;
        let xc = sub(x, self.c);
        require(xc != ZERO, "exceptional map pole")?;
        let delta = mul(sub(self.a, self.c), sub(self.b, self.c));
        let z = add(add(x, self.c), div(delta, xc));
        let d = sub(sub(z, self.a), self.b);
        if d == ZERO {
            return Ok(None);
        }
        let xx = div(sub(mul(z, z), scale(mul(self.a, self.b), 4)), scale(d, 4));
        let yy = div(
            mul(y, mul(sub(z, scale(self.a, 2)), sub(z, scale(self.b, 2)))),
            scale(mul(mul(xc, xc), mul(d, d)), 8),
        );
        Ok(Some((xx, yy)))
    }
    fn pullback(&self, q: Point, correct_poles: bool) -> Result<Divisor, String> {
        let Some((xx, yy)) = q else {
            return Ok(self.j.zero());
        };
        require(self.curve.contains(q), "public target off elliptic curve")?;
        let u = pd(&self.quartic, &ps(&self.n, scale(xx, 4)));
        let x = vec![ZERO, ONE];
        let xc = pd(&x, &pc(self.c));
        let delta = mul(sub(self.a, self.c), sub(self.b, self.c));
        let z = rem(&pa(&pa(&x, &pc(self.c)), &ps(&pinv(&xc, &u)?, delta)), &u);
        let d = pd(&z, &pc(add(self.a, self.b)));
        let numerator = pm(
            &pd(&z, &pc(scale(self.a, 2))),
            &pd(&z, &pc(scale(self.b, 2))),
        );
        let denominator = ps(&pm(&pm(&xc, &xc), &pm(&d, &d)), fp(8));
        let v = rem(&ps(&pm(&denominator, &pinv(&numerator, &u)?), yy), &u);
        require(
            rem(&pd(&self.j.f, &pm(&v, &v)), &u).is_empty(),
            "pullback incidence",
        )?;
        let a = self.j.reduce(u, v);
        // pi^-1(O) is infinity plus the three branch points N=0.
        Ok(if correct_poles {
            self.j.add(&a, &(self.n.clone(), vec![]))
        } else {
            a
        })
    }
    fn descend(&self, q: Point) -> Result<Divisor, String> {
        let a = self.pullback(q, true)?;
        let frob = |d: &Divisor, e| {
            (
                d.0.iter().map(|&x| pow(x, e)).collect(),
                d.1.iter().map(|&x| pow(x, e)).collect(),
            )
        };
        let d = self.j.add(
            &self.j.add(&a, &frob(&a, P as u64)),
            &frob(&a, (P * P) as u64),
        );
        require(
            d.0.iter().chain(&d.1).all(|x| x[1] == 0 && x[2] == 0),
            "trace not defined over base field",
        )?;
        Ok(d)
    }
}

// Fixed, specified SplitMix64 stream: no dependency on the producer's NumPy RNG.
struct Rng(u64);
impl Rng {
    fn word(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 {
        // Rejection avoids reducing a biased complete 64-bit interval.
        let cutoff = n.wrapping_neg() % n;
        loop {
            let r = self.word();
            if r >= cutoff {
                return r % n;
            }
        }
    }
}
fn lm(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % ELL as u128) as u64
}
fn la(a: u64, b: u64) -> u64 {
    (a + b) % ELL
}
fn ls(a: u64, b: u64) -> u64 {
    (a + ELL - b) % ELL
}
fn lp(mut a: u64, mut n: u64) -> u64 {
    let mut r = 1;
    while n != 0 {
        if n & 1 != 0 {
            r = lm(r, a);
        }
        a = lm(a, a);
        n >>= 1;
    }
    r
}
fn dot(a: &[u64], b: &[u64]) -> u64 {
    a.iter().zip(b).fold(0, |s, (&x, &y)| la(s, lm(x, y)))
}
fn row_value(row: &Row, x: &[u64]) -> u64 {
    row.iter().fold(0, |s, &(i, c)| {
        la(s, lm(c.rem_euclid(ELL as i64) as u64, x[i]))
    })
}
fn original_equations(rows: &[Row], x: &[u64], gi: usize) -> bool {
    x[gi] == 1 && rows.iter().all(|r| row_value(r, x) == 0)
}
fn cg(rows: &[Row], n: usize, gi: usize, seed: u64) -> Option<Vec<u64>> {
    let mut rng = Rng(seed);
    for _ in 0..12 {
        let weights: Vec<_> = (0..rows.len() + 1)
            .map(|_| 1 + rng.below(ELL - 1))
            .collect();
        let apply = |x: &[u64]| {
            let mut out = vec![0; n];
            for (r, &w) in rows.iter().zip(&weights) {
                let y = lm(w, row_value(r, x));
                for &(j, c) in r {
                    out[j] = la(out[j], lm(c.rem_euclid(ELL as i64) as u64, y));
                }
            }
            out[gi] = la(out[gi], lm(weights[rows.len()], x[gi]));
            out
        };
        let mut x = vec![0; n];
        let mut r = vec![0; n];
        r[gi] = weights[rows.len()];
        let mut direction = r.clone();
        let mut norm = dot(&r, &r);
        for _ in 0..n {
            let ad = apply(&direction);
            let denominator = dot(&direction, &ad);
            if denominator == 0 || norm == 0 {
                break;
            }
            let alpha = lm(norm, lp(denominator, ELL - 2));
            for j in 0..n {
                x[j] = la(x[j], lm(alpha, direction[j]));
                r[j] = ls(r[j], lm(alpha, ad[j]));
            }
            if r.iter().all(|&x| x == 0) {
                if original_equations(rows, &x, gi) {
                    return Some(x);
                }
                break;
            }
            let next = dot(&r, &r);
            let beta = lm(next, lp(norm, ELL - 2));
            for j in 0..n {
                direction[j] = la(r[j], lm(beta, direction[j]));
            }
            norm = next;
        }
    }
    None
}

#[derive(Default)]
struct SetupStats {
    candidates: usize,
    smooth_relations: usize,
    original_rows: usize,
}
struct Solver {
    geometry: Geometry,
    logs: Vec<u64>,
    dg: Divisor,
    rows: Vec<Row>,
    stats: SetupStats,
}
fn bp(x: i64) -> i64 {
    x.rem_euclid(P)
}
fn bpow(mut x: i64, mut n: i64) -> i64 {
    let mut r = 1;
    while n != 0 {
        if n & 1 != 0 {
            r = bp(r * x);
        }
        x = bp(x * x);
        n >>= 1;
    }
    r
}
fn interpolate(points: &[(i64, i64); 4], inverse: &[i64]) -> [i64; 4] {
    let mut h = [0; 4];
    for i in 0..4 {
        let others: Vec<_> = (0..4).filter(|&j| j != i).map(|j| points[j].0).collect();
        let (a, b, c) = (others[0], others[1], others[2]);
        let x = points[i].0;
        let denom = bp(bp((x - a) * (x - b)) * (x - c));
        let scalar = bp(points[i].1 * inverse[denom as usize]);
        h[0] = bp(h[0] - scalar * bp(a * b) * c);
        h[1] = bp(h[1] + scalar * (a * b + a * c + b * c));
        h[2] = bp(h[2] - scalar * (a + b + c));
        h[3] = bp(h[3] + scalar);
    }
    h
}
fn eval_h(h: &[i64], x: i64) -> i64 {
    h.iter().rev().fold(0, |r, &c| bp(r * x + c))
}
impl Solver {
    fn new(seed: u64) -> Result<Self, String> {
        let geometry = Geometry::new()?;
        let n = geometry.fb.len();
        let mut rows = vec![];
        let mut usage = vec![0usize; n];
        usage[geometry.gi] = 1;
        for (j, &(_, y)) in geometry.fb.iter().enumerate() {
            if y == 0 {
                rows.push(vec![(j, 2)]);
                usage[j] += 1;
            }
        }
        let mut stats = SetupStats::default();
        let mut rng = Rng(seed);
        let inverse: Vec<_> = (0..P)
            .map(|x| if x == 0 { 0 } else { bpow(x, P - 2) })
            .collect();
        let logs = loop {
            require(
                stats.candidates < CANDIDATE_LIMIT,
                "bounded relation limit reached",
            )?;
            let mut priority: Vec<_> = (0..n).map(|j| (usage[j], rng.word(), j)).collect();
            priority.sort_unstable();
            for batch_i in 0..128.min(CANDIDATE_LIMIT - stats.candidates) {
                let mut indices = [priority[batch_i % n].2, 0, 0, 0];
                for j in 1..4 {
                    loop {
                        indices[j] = rng.below(n as u64) as usize;
                        if !indices[..j].contains(&indices[j]) {
                            break;
                        }
                    }
                }
                let points = indices.map(|j| {
                    let (x, y) = geometry.fb[j];
                    (x, if rng.below(2) == 0 { y } else { bp(-y) })
                });
                let h = interpolate(&points, &inverse);
                let mut row = vec![];
                for (j, &(x, y)) in geometry.fb.iter().enumerate() {
                    let value = eval_h(&h, x);
                    if value == y {
                        row.push((j, 1));
                    } else if value == bp(-y) {
                        row.push((j, -1));
                    }
                }
                stats.candidates += 1;
                if row.len() == 7 {
                    for &(j, _) in &row {
                        usage[j] += 1;
                    }
                    rows.push(row);
                    stats.smooth_relations += 1;
                }
            }
            if rows.len() >= n + 8 && usage.iter().all(|&u| u > 0) {
                if let Some(logs) = cg(
                    &rows,
                    n,
                    geometry.gi,
                    seed.wrapping_add(stats.candidates as u64),
                ) {
                    break logs;
                }
            }
        };
        stats.original_rows = rows.len();
        require(
            original_equations(&rows, &logs, geometry.gi),
            "original relation residual",
        )?;
        let dg = geometry.descend(G)?;
        require(
            dg != geometry.j.zero() && geometry.j.scalar(ELL, &dg) == geometry.j.zero(),
            "descended generator subgroup",
        )?;
        Ok(Self {
            geometry,
            logs,
            dg,
            rows,
            stats,
        })
    }
    fn check_factor_base_logs(&self) -> Result<(), String> {
        for (&a, &image) in self.logs.iter().zip(&self.geometry.images) {
            require(
                self.geometry.curve.scalar(a, G) == image,
                "factor-base logarithm mismatch",
            )?;
        }
        Ok(())
    }
    fn solve(&self, q: Point) -> Result<(u64, usize), String> {
        require(
            self.geometry.curve.contains(q),
            "public target off elliptic curve",
        )?;
        require(
            self.geometry.curve.scalar(ELL, q).is_none(),
            "public target outside subgroup",
        )?;
        let mut d = self.geometry.descend(q)?;
        for shift in 0..10_000usize {
            let roots: Vec<_> = (0..P).filter(|&x| peval(&d.0, fp(x)) == ZERO).collect();
            if roots.len() + 1 == d.0.len() {
                let mut total = 0;
                for x in roots {
                    let y = peval(&d.1, fp(x));
                    let i = self
                        .geometry
                        .fb
                        .binary_search_by_key(&x, |&(xx, _)| xx)
                        .map_err(|_| "smooth root not in factor base")?;
                    let positive = fp(self.geometry.fb[i].1);
                    require(
                        y == positive || y == neg(positive),
                        "invalid smooth ordinate",
                    )?;
                    total = if y == positive {
                        la(total, self.logs[i])
                    } else {
                        ls(total, self.logs[i])
                    };
                }
                // Trace-conorm followed by pushforward is [4] on this component.
                // Factor-base logs describe [cofactor] pi(P), cofactor=4.
                let answer = ls(lm(total, lp(16, ELL - 2)), shift as u64 % ELL);
                require(
                    self.geometry.curve.scalar(answer, G) == q,
                    "recovered scalar mismatch",
                )?;
                return Ok((answer, shift));
            }
            d = self.geometry.j.add(&d, &self.dg);
        }
        Err("bounded target-shift limit reached".into())
    }
}

fn solve_request(data: &Value) -> Result<Value, String> {
    let fields = data.as_object().ok_or("request must be an object")?;
    require(
        fields.len() == 2 && fields.contains_key("target") && fields.contains_key("relation_seed"),
        "request accepts only target and relation_seed; no known scalar",
    )?;
    let q = point(&data["target"])?;
    let seed = data["relation_seed"]
        .as_u64()
        .ok_or("missing relation seed")?;
    let solver = Solver::new(seed)?;
    solver.check_factor_base_logs()?;
    let (answer, shifts) = solver.solve(q)?;
    Ok(
        json!({"recovered_log":answer,"target_shifts":shifts,"factor_base":solver.logs.len(),
        "relation_seed":seed,"rng":"splitmix64-rejection-v1",
        "candidates":solver.stats.candidates,"smooth_relations":solver.stats.smooth_relations,
        "original_rows":solver.rows.len(),"all_factor_base_logs_checked":true}),
    )
}

pub(super) fn public_target(path: &str) -> Result<(), String> {
    let bytes = fs::read(path).map_err(|e| e.to_string())?;
    let request: Value = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    let result = solve_request(&request)?;
    println!("{}", serde_json::to_string_pretty(&json!({
        "schema":"genus3.native-public-target/v1","pcc_standing":"unverified",
        "request_sha256":sha(&bytes),"result":result,"timings_recorded":false,
        "scope":"native development solver for the fixed historical fixture; no performance admission"
    })).map_err(|e| e.to_string())?);
    Ok(())
}

pub(super) fn development_check() -> Result<(), String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let bytes = fs::read(root.join("historical/cold_target_audit_q1061_v65.json"))
        .map_err(|e| e.to_string())?;
    let data: Value = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    let mut checks = vec![];
    for row in data["raw_trials"]
        .as_array()
        .ok_or("missing trial array")?
        .iter()
        .take(2)
    {
        let q = point(&row["target"])?;
        let seed = row["relation_seed"]
            .as_u64()
            .ok_or("missing relation seed")?;
        // Fresh field/cover/factor base/relations/linear solve for each public Q.
        let solver = Solver::new(seed)?;
        solver.check_factor_base_logs()?;
        let (answer, shifts) = solver.solve(q)?;
        require(
            Some(answer) == row["recovered_log"].as_u64(),
            "archived known-answer mismatch",
        )?;
        checks.push(
            json!({"trial":row["trial"],"relation_seed":seed,"rng":"splitmix64-rejection-v1",
            "recovered_log":answer,"target_shifts":shifts,"factor_base":solver.logs.len(),
            "candidates":solver.stats.candidates,"smooth_relations":solver.stats.smooth_relations,
            "original_rows":solver.rows.len(),"all_factor_base_logs_checked":true}),
        );
    }
    println!("{}", serde_json::to_string_pretty(&json!({
        "schema":"genus3.native-development-check/v1", "status":"local_self_check_pass",
        "pcc_standing":"unverified", "raw_sha256":sha(&bytes), "fresh_context_per_target":true,
        "timings_recorded":false, "checks":checks,
        "not_checked":["operation accounting","matched strongest native rho","execution admission",
            "registered transfer identity","signed independent replay"]
    })).map_err(|e| e.to_string())?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn map_identities_and_pole_correction() {
        let mut g = Geometry::new().unwrap();
        let dg = g.descend(G).unwrap();
        let q = g.curve.scalar(1729, G);
        assert_eq!(g.descend(q).unwrap(), g.j.scalar(1729, &dg));
        assert_eq!(
            g.descend(g.curve.add(G, q)).unwrap(),
            g.j.add(&dg, &g.descend(q).unwrap())
        );
        let corrected = g.pullback(G, true).unwrap();
        let wrong = g.pullback(G, false).unwrap();
        assert_ne!(corrected, wrong);
        assert_eq!(
            g.j.add(&corrected, &(wrong.0, pn(&wrong.1))),
            (g.n.clone(), vec![])
        );
        assert_eq!(
            g.j.add(&(g.n.clone(), vec![]), &(g.n.clone(), vec![])),
            g.j.zero()
        );
        let uncorrected = g.pullback(G, false).unwrap();
        let frob = |d: &Divisor, e| {
            (
                d.0.iter().map(|&x| pow(x, e)).collect(),
                d.1.iter().map(|&x| pow(x, e)).collect(),
            )
        };
        let wrong_trace = g.j.add(
            &g.j.add(&uncorrected, &frob(&uncorrected, P as u64)),
            &frob(&uncorrected, (P * P) as u64),
        );
        assert_ne!(g.j.scalar(ELL, &wrong_trace), g.j.zero());
        g.a = add(g.a, ONE);
        assert!(g.identities().is_err());
    }
    #[test]
    fn native_fresh_public_target_and_mutations() {
        let solver = Solver::new(2026111507).unwrap();
        solver.check_factor_base_logs().unwrap();
        let q = solver.geometry.curve.scalar(1729, G);
        assert_eq!(solver.solve(q).unwrap().0, 1729);
        assert_eq!(solver.solve(None).unwrap().0, 0);
        assert_eq!(solver.solve(G).unwrap().0, 1);
        assert_eq!(
            solver
                .solve(solver.geometry.curve.scalar(ELL - 1, G))
                .unwrap()
                .0,
            ELL - 1
        );
        let mut rows = solver.rows.clone();
        rows.push(vec![(solver.geometry.gi, 1)]);
        assert!(!original_equations(&rows, &solver.logs, solver.geometry.gi));
        let Some((x, y)) = q else {
            panic!("nonzero target")
        };
        assert!(solver.solve(Some((x, add(y, ONE)))).is_err());
    }
    #[test]
    fn public_interface_rejects_known_scalar_input() {
        assert!(solve_request(&json!({"target":G,"relation_seed":1,"recovered_log":1})).is_err());
    }
    #[test]
    fn archived_targets_use_fresh_native_contexts() {
        development_check().unwrap();
    }
}

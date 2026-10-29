//! Exact larger one-prime CM carriers and finite cyclic-cocycle genus checks.
//! This does not construct explicit curve equations or subgroup injections.
mod dense_control;
use num_bigint::BigInt;
use num_traits::{One, Zero};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    env, fs,
    path::Path,
};
fn pow(mut a: usize, mut e: usize, p: usize) -> usize {
    let mut r = 1;
    while e > 0 {
        if e % 2 == 1 {
            r = r * a % p;
        }
        a = a * a % p;
        e /= 2;
    }
    r
}
fn prime(p: usize) -> bool {
    p >= 2 && (2..).take_while(|d| d * d <= p).all(|d| p % d != 0)
}
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let c = a % b;
        a = b;
        b = c;
    }
    a
}
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (g, x, y) = egcd(b, a % b);
    (g, y, x - (a / b) * y)
}
fn mul(a: &[BigInt], b: &[BigInt]) -> Vec<BigInt> {
    let mut c = vec![BigInt::zero(); a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            c[i + j] += x * y;
        }
    }
    c
}
fn trace_zero(n: usize, t: i64) -> Vec<BigInt> {
    let (mut a, mut b) = (BigInt::from(2), BigInt::from(t));
    for _ in 1..n {
        let c = t * &b - 2 * &a;
        a = b;
        b = c;
    }
    let mut top = vec![BigInt::zero(); 2 * n + 1];
    top[0] = BigInt::one();
    top[n] = -b;
    top[2 * n] = BigInt::one() << n;
    let mut q = vec![BigInt::zero(); 2 * n - 1];
    for k in 0..q.len() {
        let mut v = top[k].clone();
        if k >= 1 {
            v += t * &q[k - 1];
        }
        if k >= 2 {
            v -= 2 * &q[k - 2];
        }
        q[k] = v;
    }
    assert_eq!(
        mul(&q, &[BigInt::one(), BigInt::from(-t), BigInt::from(2)]),
        top
    );
    q.reverse();
    q
}
#[derive(Clone)]
struct Lift {
    to: Vec<usize>,
    phase: Vec<usize>,
    m: usize,
}
impl Lift {
    fn compose(a: &Self, b: &Self) -> Self {
        assert_eq!(a.m, b.m);
        Self {
            to: b.to.iter().map(|y| a.to[*y]).collect(),
            phase: (0..b.to.len())
                .map(|x| (b.phase[x] + a.phase[b.to[x]]) % a.m)
                .collect(),
            m: a.m,
        }
    }
    fn identity(&self) -> bool {
        self.to.iter().enumerate().all(|(i, y)| i == *y) && self.phase.iter().all(|s| *s == 0)
    }
    fn cycles(&self) -> Value {
        let n = self.to.len();
        let mut seen = vec![false; n];
        let mut total = 0;
        let mut hist = BTreeMap::<usize, usize>::new();
        let mut base_cycles = 0;
        for start in 0..n {
            if seen[start] {
                continue;
            }
            let (mut x, mut len, mut shift) = (start, 0, 0);
            loop {
                assert!(!seen[x]);
                seen[x] = true;
                len += 1;
                shift = (shift + self.phase[x]) % self.m;
                x = self.to[x];
                if x == start {
                    break;
                }
            }
            let g = gcd(self.m, shift);
            let period = self.m / g;
            assert_eq!(period * shift % self.m, 0);
            for k in 1..period {
                assert_ne!(k * shift % self.m, 0);
            }
            total += g;
            *hist.entry(len * period).or_default() += g;
            base_cycles += 1;
        }
        assert_eq!(
            hist.iter()
                .map(|(length, count)| length * count)
                .sum::<usize>(),
            n * self.m
        );
        json!({"base_cycles":base_cycles,"lifted_cycles":total,"lifted_cycle_length_histogram":hist})
    }
    fn materialize(&self) -> Vec<usize> {
        let mut p = vec![0; self.to.len() * self.m];
        for x in 0..self.to.len() {
            for z in 0..self.m {
                p[x * self.m + z] = self.to[x] * self.m + (z + self.phase[x]) % self.m;
            }
        }
        p
    }
}
fn cycle_count(p: &[usize]) -> usize {
    let mut seen = vec![false; p.len()];
    let mut count = 0;
    for i in 0..p.len() {
        if seen[i] {
            continue;
        }
        count += 1;
        let mut j = i;
        loop {
            assert!(!seen[j]);
            seen[j] = true;
            j = p[j];
            if j == i {
                break;
            }
        }
    }
    count
}
fn local(q: usize, logs: &[usize], n: usize, seven: bool, s: bool) -> Vec<(usize, usize)> {
    (0..=q)
        .map(|a| {
            let (c, d) = if a == q { (0, 1) } else { (1, a) };
            let (x, y) = if s {
                (d, (q - c) % q)
            } else {
                (c, (c + d) % q)
            };
            let (scale, next) = if x == 0 {
                (y, q)
            } else {
                (x, y * pow(x, q - 2, q) % q)
            };
            let phase = if seven {
                if pow(scale, 3, 7) == 1 {
                    0
                } else {
                    n
                }
            } else {
                logs[scale] % (2 * n)
            };
            (next, phase)
        })
        .collect()
}
fn lift(p: usize, n: usize, logs: &[usize], s: bool) -> Lift {
    let a = local(7, &[], n, true, s);
    let b = local(p, logs, n, false, s);
    let size = 8 * (p + 1);
    assert!(size <= 1_000_000, "frozen base-state bound");
    let mut to = vec![0; size];
    let mut phase = vec![0; size];
    for x in 0..8 {
        for y in 0..=p {
            let k = x * (p + 1) + y;
            to[k] = a[x].0 * (p + 1) + b[y].0;
            phase[k] = (a[x].1 + b[y].1) % (2 * n);
        }
    }
    Lift {
        to,
        phase,
        m: 2 * n,
    }
}
fn connected(s: &Lift, t: &Lift) -> Value {
    let n = s.to.len();
    let m = s.m;
    let mut potential = vec![usize::MAX; n];
    potential[0] = 0;
    let mut q = VecDeque::from([0]);
    while let Some(x) = q.pop_front() {
        for p in [s, t] {
            let y = p.to[x];
            if potential[y] == usize::MAX {
                potential[y] = (potential[x] + p.phase[x]) % m;
                q.push_back(y);
            }
        }
    }
    assert!(potential.iter().all(|x| *x != usize::MAX));
    let mut g = m;
    let mut chain = vec![];
    for x in 0..n {
        for (label, p) in [("S", s), ("T", t)] {
            let y = p.to[x];
            let residue = (potential[x] + p.phase[x] + m - potential[y]) % m;
            let next = gcd(g, residue);
            if next < g {
                let (gg, u, v) = egcd(g as i64, residue as i64);
                assert_eq!(gg, next as i64);
                assert_eq!(u * g as i64 + v * residue as i64, gg);
                chain.push(json!({"edge":label,"from":x,"to":y,"old_gcd":g,"loop_residue":residue,"bezout":[u,v],"new_gcd":next}));
                g = next;
            }
        }
    }
    assert_eq!(g, 1, "cyclic cover must be connected");
    json!({"base_graph_reached":n,"fiber_loop_gcd":g,"bezout_chain":chain})
}
fn cosets(p: usize, n: usize, logs: &[usize]) -> Value {
    let s = lift(p, n, logs, true);
    let t = lift(p, n, logs, false);
    let st = Lift::compose(&s, &t);
    assert!(Lift::compose(&s, &s).identity());
    assert!(Lift::compose(&st, &Lift::compose(&st, &st)).identity());
    let conn = connected(&s, &t);
    let mut bad = s.clone();
    bad.phase[0] = (bad.phase[0] + 1) % bad.m;
    assert!(
        !Lift::compose(&bad, &bad).identity(),
        "planted cocycle corruption was not detected"
    );
    let c = [s.cycles(), st.cycles(), t.cycles()];
    let counts = c
        .iter()
        .map(|v| v["lifted_cycles"].as_u64().unwrap() as usize)
        .collect::<Vec<_>>();
    let index = s.to.len() * s.m;
    let numerator = index + 2 - counts.iter().sum::<usize>();
    assert_eq!(numerator % 2, 0);
    let genus = numerator / 2;
    let full = if index <= 10_000_000 {
        let ss = s.materialize();
        let tt = t.materialize();
        let stt = (0..index).map(|i| ss[tt[i]]).collect::<Vec<_>>();
        assert!((0..index).all(|i| ss[ss[i]] == i && stt[stt[stt[i]]] == i));
        let cc = vec![cycle_count(&ss), cycle_count(&stt), cycle_count(&tt)];
        assert_eq!(cc, counts);
        json!({"status":"complete materialization within unchanged cap","cycles":cc})
    } else {
        json!({"status":"not materialized above original10000000state cap; exact cocycle certificate supplied","index":index})
    };
    json!({"base_states_stored":s.to.len(),"full_index":index,"cycles_S_ST_T":counts,"genus":genus,"cycle_certificates":c,"transitivity":conn,"S2_ST3_verified_on_all_base_states_and_fiber_shifts":true,"corrupt_phase_rejected":true,"full_materialization_control":full})
}
fn logs(p: usize) -> (usize, Vec<usize>) {
    let factors = (2..p)
        .filter(|d| (p - 1) % d == 0 && prime(*d))
        .collect::<Vec<_>>();
    let g = (2..p)
        .find(|g| factors.iter().all(|f| pow(*g, (p - 1) / f, p) != 1))
        .unwrap();
    let mut logs = vec![usize::MAX; p];
    let mut x = 1;
    for k in 0..p - 1 {
        assert_eq!(logs[x], usize::MAX);
        logs[x] = k;
        x = x * g % p;
    }
    assert_eq!(x, 1);
    (g, logs)
}
fn root_factor_certificate(n: usize, i: usize, j: usize) -> Value {
    let m = 2 * n;
    let units = (1..m).filter(|u| u % 2 == 1 && *u != n).collect::<Vec<_>>();
    let orbit = units
        .iter()
        .flat_map(|u| [(i * u % m, j * u % m), (j * u % m, i * u % m)])
        .collect::<BTreeSet<_>>();
    assert_eq!(orbit.len(), 2 * (n - 1));
    let mut actual = vec![];
    for u in &units {
        for sigma in 0..2 {
            actual.push((sigma, i * u % m));
            actual.push((1 - sigma, j * u % m));
        }
    }
    let mut expected = vec![];
    for parity in [i % 2, j % 2] {
        for branch in 0..2 {
            for a in 1..n {
                expected.push((branch, (parity * n + 2 * a) % m));
            }
        }
    }
    actual.sort_unstable();
    expected.sort_unstable();
    assert_eq!(actual, expected);
    let mut bad = actual.clone();
    bad[0].1 = (bad[0].1 + 1) % m;
    bad.sort_unstable();
    assert_ne!(bad, expected);
    let factor_a = trace_zero(n, if i % 2 == 0 { 1 } else { -1 });
    let factor_b = trace_zero(n, if j % 2 == 0 { 1 } else { -1 });
    let polynomial = mul(&factor_a, &factor_b);
    if n == 5 || n == 11 {
        assert_eq!(dense_control::dense_norm(n, i, j), polynomial);
        assert_eq!(dense_control::trace_zero(n, 1), trace_zero(n, 1));
        assert_eq!(dense_control::trace_zero(n, -1), trace_zero(n, -1));
    }
    assert_eq!(polynomial.len(), 4 * (n - 1) + 1);
    assert_eq!(polynomial.last().unwrap(), &BigInt::one());
    assert_eq!(polynomial[0], BigInt::one() << (2 * (n - 1)));
    json!({"a2_orbit":orbit.len(),"explicit_embedding_pairs":orbit,"root_label_count":actual.len(),"actual_root_labels":actual,"predicted_root_labels":expected,"root_multiset_bijection_checked":true,"corrupted_root_label_rejected":true,"dense_ring_control":n==5||n==11,"Frobenius_polynomial_X_ascending":polynomial.iter().map(ToString::to_string).collect::<Vec<_>>()})
}
fn main() {
    let out = env::args().nth(1).expect("fresh output directory");
    let out = Path::new(&out);
    assert!(!out.exists());
    fs::create_dir_all(out).unwrap();
    let ns = [5, 11, 83, 131, 163, 233, 283, 409, 571];
    let mut scan = vec![];
    let mut winners = vec![];
    for n in ns {
        assert!(prime(n) && n != 7);
        let mut best = None;
        for k in (1..=99).step_by(2) {
            let p = 2 * n * k + 1;
            if p > 100000 {
                scan.push(
                    json!({"n":n,"k":k,"p":p,"status":"outside unchanged conductor-prime bound"}),
                );
                continue;
            }
            if !prime(p) {
                scan.push(json!({"n":n,"k":k,"p":p,"status":"composite"}));
                continue;
            }
            let roots = (0..p)
                .filter(|r| (r * r + 2 + p - r) % p == 0)
                .collect::<Vec<_>>();
            if roots.len() != 2 {
                assert!(roots.is_empty());
                scan.push(json!({"n":n,"k":k,"p":p,"status":"inert"}));
                continue;
            }
            let (g, logs) = logs(p);
            assert_eq!(logs[p - 1] % (2 * n), n);
            let i = logs[roots[0]] % (2 * n);
            let j = logs[roots[1]] % (2 * n);
            if i % n == 0 || j % n == 0 {
                scan.push(json!({"n":n,"k":k,"p":p,"status":"nonprimitive Frobenius residue","exponents":[i,j]}));
                continue;
            }
            let orbit = (1..2 * n)
                .filter(|u| u % 2 == 1 && *u != n)
                .flat_map(|u| {
                    [
                        (i * u % (2 * n), j * u % (2 * n)),
                        (j * u % (2 * n), i * u % (2 * n)),
                    ]
                })
                .collect::<BTreeSet<_>>();
            if orbit.len() != 2 * (n - 1) {
                scan.push(json!({"n":n,"k":k,"p":p,"status":"a2 field orbit too small","orbit":orbit.len()}));
                continue;
            }
            scan.push(json!({"n":n,"k":k,"p":p,"status":"qualifying"}));
            if best.is_none() {
                best = Some((k, p, g, logs, roots, i, j));
            }
        }
        let (k, p, g, logs, roots, i, j) = best.expect("frozen scan has no qualifying prime");
        let e3 = if p % 3 == 1 { 4 } else { 0 };
        let mu = 8 * (p + 1);
        assert_eq!((mu - 12 - 4 * e3) % 12, 0);
        let g0 = (mu - 12 - 4 * e3) / 12;
        let genus = 2 * n * (g0 - 1) + 1;
        let cosets = cosets(p, n, &logs);
        assert_eq!(cosets["genus"].as_u64().unwrap() as usize, genus);
        let euler = root_factor_certificate(n, i, j);
        let plus = usize::from(i % 2 == 0) + usize::from(j % 2 == 0);
        let minus = 2 - plus;
        if let Some((expected_p, expected_genus, expected_plus)) = match n {
            5 => Some((11, 61, 1)),
            11 => Some((23, 309, 0)),
            83 => Some((499, 54781, 1)),
            131 => Some((263, 45589, 2)),
            _ => None,
        } {
            assert_eq!(
                (p, genus, plus),
                (expected_p, expected_genus, expected_plus)
            );
        }
        assert!((1..2 * n)
            .filter(|u| u % 2 == 1 && *u != n)
            .all(|u| n * u % (2 * n) == n));
        let rec = json!({"n":n,"k":k,"p":p,"level":7*p,"primitive_generator":g,"roots_alpha2_minus_alpha_plus2":roots,"eta_exponents":[i,j],"untwisted_E1_E0":[plus,minus],"diamond_twisted_E1_E0":[minus,plus],"coefficient_field_degree":2*(n-1),"character_cover_genus":genus,"coset_certificate":cosets,"Euler_certificate":euler,"explicit_curve_equations":false,"subgroup_transfer_evaluated":false});
        fs::write(
            out.join(format!("carrier-{n}.json")),
            serde_json::to_string_pretty(&rec).unwrap(),
        )
        .unwrap();
        println!("CM31 n={n} p={p} level={} genus={genus} full_index={} stored_base={} E1_E0=[{plus},{minus}]",7*p,16*n*(p+1),8*(p+1));
        winners.push(rec);
    }
    // Fixed independent level77 Euler coefficients from the R15 LMFDB control.
    let db = [
        1, 0, -3, 0, 5, 0, -3, 0, -11, 0, -12, 0, 80, 0, -192, 0, 256,
    ]
    .into_iter()
    .rev()
    .map(BigInt::from)
    .collect::<Vec<_>>();
    assert_eq!(dense_control::dense_norm(5, 4, 7), db);
    assert_ne!(dense_control::dense_norm(5, 4, 8), db);
    fs::write(out.join("results.json"),serde_json::to_string_pretty(&json!({"schema":"koblitz.cm31/v1","scan":scan,"winners":winners,"scope":"exact character/Euler/coset data; modularity and finite-field isogeny written, no explicit map","performance_measured":false})).unwrap()).unwrap();
    println!("CM31_ALL_CHECKS_PASS");
}

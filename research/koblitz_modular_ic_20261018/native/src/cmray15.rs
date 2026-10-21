//! One-prime CM characters and diamond-twisted lower-level carriers.
use crate::round04::read;
use num_bigint::BigInt;
use num_traits::{One, Zero};
use serde_json::{json, Value};
use std::{collections::BTreeSet, path::Path};
fn pow(mut a: usize, mut e: usize, p: usize) -> usize {
    let mut v = 1;
    while e > 0 {
        if e % 2 == 1 {
            v = v * a % p;
        }
        a = a * a % p;
        e /= 2;
    }
    v
}
fn prime(p: usize) -> bool {
    p >= 2 && (2..).take_while(|d| d * d <= p).all(|d| p % d != 0)
}
fn p_mul(a: &[BigInt], b: &[BigInt]) -> Vec<BigInt> {
    let mut c = vec![BigInt::zero(); a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            c[i + j] += x * y;
        }
    }
    c
}
pub(crate) fn an(n: usize, t: i64) -> Vec<BigInt> {
    let (mut a, mut b) = (BigInt::from(2), BigInt::from(t));
    for _ in 1..n {
        let c = t * &b - 2 * &a;
        a = b;
        b = c;
    }
    let mut p = vec![BigInt::zero(); 2 * n + 1];
    p[0] = BigInt::one() << n;
    p[n] = -b;
    p[2 * n] = BigInt::one();
    let d = [BigInt::from(2), BigInt::from(-t), BigInt::one()];
    let mut q = vec![BigInt::zero(); 2 * n - 1];
    for k in (2..=2 * n).rev() {
        let x = p[k].clone();
        q[k - 2] = x.clone();
        for j in 0..3 {
            p[k - 2 + j] -= &x * &d[j];
        }
    }
    assert!(p.iter().all(Zero::is_zero));
    q
}
type Cyc = Vec<[BigInt; 2]>;
fn zero(n: usize) -> Cyc {
    vec![[BigInt::zero(), BigInt::zero()]; n - 1]
}
fn shift(a: &Cyc, e: usize, kind: usize, k: i64, n: usize) -> Cyc {
    let mut out = vec![[BigInt::zero(), BigInt::zero()]; n];
    for (i, v) in a.iter().enumerate() {
        let w = match kind {
            0 => v.clone(),
            1 => [-2 * &v[1], &v[0] + &v[1]],
            2 => [&v[0] + 2 * &v[1], -v[0].clone()],
            _ => unreachable!(),
        };
        for j in 0..2 {
            out[(i + e) % n][j] += k * &w[j];
        }
    }
    let last = out.pop().unwrap();
    for v in &mut out {
        for j in 0..2 {
            v[j] -= &last[j];
        }
    }
    out
}
fn add(a: &mut Cyc, b: &Cyc) {
    for (i, v) in a.iter_mut().enumerate() {
        for j in 0..2 {
            v[j] += &b[i][j];
        }
    }
}
fn norm_euler(n: usize, i: usize, j: usize) -> Vec<BigInt> {
    let mut p = vec![zero(n)];
    p[0][0][0] = BigInt::one();
    let ii = (n + 1) / 2;
    for l in (1..2 * n).filter(|l| l % 2 == 1 && *l != n) {
        for conjugate in 0..2 {
            let a = i * l % (2 * n);
            let b = j * l % (2 * n);
            let mut next = vec![zero(n); p.len() + 2];
            for (k, c) in p.iter().enumerate() {
                add(&mut next[k + 2], c);
                add(
                    &mut next[k + 1],
                    &shift(
                        c,
                        a * ii % n,
                        if conjugate == 0 { 1 } else { 2 },
                        if a % 2 == 0 { -1 } else { 1 },
                        n,
                    ),
                );
                add(
                    &mut next[k + 1],
                    &shift(
                        c,
                        b * ii % n,
                        if conjugate == 0 { 2 } else { 1 },
                        if b % 2 == 0 { -1 } else { 1 },
                        n,
                    ),
                );
                add(
                    &mut next[k],
                    &shift(
                        c,
                        (a + b) * ii % n,
                        0,
                        if (a + b) % 2 == 0 { 2 } else { -2 },
                        n,
                    ),
                );
            }
            p = next;
        }
    }
    p.into_iter()
        .map(|c| {
            assert!(c.iter().skip(1).all(|v| v.iter().all(Zero::is_zero)));
            assert!(c[0][1].is_zero());
            c[0][0].clone()
        })
        .collect()
}
fn cycles(p: &[usize]) -> usize {
    let mut seen = vec![false; p.len()];
    let mut count = 0;
    for i in 0..p.len() {
        if seen[i] {
            continue;
        }
        count += 1;
        let mut j = i;
        while !seen[j] {
            seen[j] = true;
            j = p[j];
        }
        assert_eq!(j, i);
    }
    count
}
fn local(q: usize, logs: &[usize], n: usize, is_seven: bool, st: bool) -> Vec<(usize, usize)> {
    (0..=q)
        .map(|a| {
            let (c, d) = if a == q { (0, 1) } else { (1, a) };
            let (x, y) = if st {
                (d, (q - c) % q)
            } else {
                (c, (c + d) % q)
            };
            let (scale, next) = if x == 0 {
                (y, q)
            } else {
                (x, y * pow(x, q - 2, q) % q)
            };
            let phase = if is_seven {
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
fn cosets(p: usize, n: usize, logs: &[usize]) -> Value {
    let d = 8 * (p + 1) * 2 * n;
    if d > 10_000_000 {
        return json!({"index":d,"status":"explicit permutation cap;formula only"});
    }
    let a_s = local(7, &[], n, true, true);
    let a_t = local(7, &[], n, true, false);
    let b_s = local(p, logs, n, false, true);
    let b_t = local(p, logs, n, false, false);
    let mut s = vec![0; d];
    let mut t = vec![0; d];
    let code = |a: usize, b: usize, c: usize| (a * (p + 1) + b) * 2 * n + c;
    for a in 0..8 {
        for b in 0..=p {
            for c in 0..2 * n {
                let k = code(a, b, c);
                s[k] = code(a_s[a].0, b_s[b].0, (c + a_s[a].1 + b_s[b].1) % (2 * n));
                t[k] = code(a_t[a].0, b_t[b].0, (c + a_t[a].1 + b_t[b].1) % (2 * n));
            }
        }
    }
    assert!((0..d).all(|i| s[s[i]] == i));
    let st = (0..d).map(|i| s[t[i]]).collect::<Vec<_>>();
    assert!((0..d).all(|i| st[st[st[i]]] == i));
    let counts = [cycles(&s), cycles(&st), cycles(&t)];
    assert_eq!((d + 2).wrapping_sub(counts.iter().sum::<usize>()) % 2, 0);
    let genus = (d + 2 - counts.iter().sum::<usize>()) / 2;
    json!({"index":d,"cycles_S_ST_T":counts,"genus":genus,"S_squared_and_ST_cubed_checked":true,"status":"complete finite coset permutations"})
}
fn predicted(n: usize, plus: usize, minus: usize) -> Vec<BigInt> {
    let mut p = vec![BigInt::one()];
    for _ in 0..plus {
        p = p_mul(&p, &an(n, 1));
    }
    for _ in 0..minus {
        p = p_mul(&p, &an(n, -1));
    }
    p
}
fn lm_rows(dir: &Path) -> Vec<Value> {
    let r = read(&dir.join("lmfdb-CM-local-polynomials.json"));
    serde_json::from_str::<Value>(r["result"]["structuredContent"]["result"].as_str().unwrap())
        .unwrap()["rows"]
        .as_array()
        .unwrap()
        .clone()
}
pub fn run(dir: &Path) {
    let mut all = vec![];
    let mut winners = vec![];
    for n in [5, 11, 83, 131] {
        assert!(prime(n) && n != 7);
        let mut best = None;
        for k in (1..=99).step_by(2) {
            let p = 2 * n * k + 1;
            if p > 100000 {
                continue;
            }
            if !prime(p) {
                all.push(json!({"n":n,"k":k,"p":p,"status":"composite"}));
                continue;
            }
            let roots = (0..p)
                .filter(|r| ((r * r + 2) % p + p - r) % p == 0)
                .collect::<Vec<_>>();
            if roots.len() != 2 {
                assert!(roots.is_empty());
                all.push(json!({"n":n,"k":k,"p":p,"status":"inert"}));
                continue;
            }
            let factors = (2..p)
                .filter(|d| (p - 1) % d == 0 && prime(*d))
                .collect::<Vec<_>>();
            let g = (2..p)
                .find(|a| factors.iter().all(|l| pow(*a, (p - 1) / l, p) != 1))
                .unwrap();
            let mut logs = vec![usize::MAX; p];
            let mut x = 1;
            for e in 0..p - 1 {
                assert_eq!(logs[x], usize::MAX);
                logs[x] = e;
                x = x * g % p;
            }
            assert_eq!(x, 1);
            assert_eq!(logs[p - 1] % (2 * n), n);
            let i = logs[roots[0]] % (2 * n);
            let j = logs[roots[1]] % (2 * n);
            if i % n == 0 || j % n == 0 {
                all.push(json!({"n":n,"k":k,"p":p,"roots":roots,"exponents":[i,j],"status":"Frobenius value not primitive modulo n"}));
                continue;
            }
            let orbit = (1..2 * n)
                .filter(|l| l % 2 == 1 && *l != n)
                .flat_map(|l| {
                    [
                        (i * l % (2 * n), j * l % (2 * n)),
                        (j * l % (2 * n), i * l % (2 * n)),
                    ]
                })
                .collect::<BTreeSet<_>>();
            if orbit.len() != 2 * (n - 1) {
                all.push(json!({"n":n,"k":k,"p":p,"status":"a2 alone does not certify full coefficient field","a2_orbit":orbit.len()}));
                continue;
            }
            let plus = usize::from(i % 2 == 0) + usize::from(j % 2 == 0);
            let minus = 2 - plus;
            let e3 = if p % 3 == 1 { 4 } else { 0 };
            let mu = 8 * (p + 1);
            assert_eq!((mu - 12 - 4 * e3) % 12, 0);
            let base_genus = (mu - 12 - 4 * e3) / 12;
            let genus = 2 * n * (base_genus - 1) + 1;
            let rec = json!({"n":n,"k":k,"p":p,"level":7*p,"primitive_generator":g,"roots_of_X2_minus_X_plus2":roots,"eta_exponents_mod2n":[i,j],"a2_orbit_degree":orbit.len(),"nebentypus_order":2*n,"untwisted_E1_E0_multiplicities":[plus,minus],"diamond_quadratic_twist_E1_E0_multiplicities":[minus,plus],"base_X0_genus":base_genus,"carrier_genus":genus,"status":"CM-character candidate with full a2 field certificate","geometric_diamond_twist_needed_for_E0":minus==0});
            all.push(rec.clone());
            if best.is_none() {
                best = Some((rec, logs, i, j));
            }
        }
        let (mut rec, logs, i, j) = best.expect("no qualifying auxiliary prime within bound");
        let p = rec["p"].as_u64().unwrap() as usize;
        let m = rec["untwisted_E1_E0_multiplicities"].as_array().unwrap();
        let plus = m[0].as_u64().unwrap() as usize;
        let minus = m[1].as_u64().unwrap() as usize;
        let polynomial = norm_euler(n, i, j);
        assert_eq!(polynomial, predicted(n, plus, minus));
        rec["exact_Frobenius_polynomial_X_ascending"] = json!(polynomial
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>());
        rec["exact_L_polynomial_T_ascending"] = json!(polynomial
            .iter()
            .rev()
            .map(ToString::to_string)
            .collect::<Vec<_>>());
        let ct = cosets(p, n, &logs);
        if ct["status"] == "complete finite coset permutations" {
            assert_eq!(ct["genus"], rec["carrier_genus"]);
        }
        rec["independent_coset_genus_check"] = ct;
        let diamond = (0..2 * n).map(|e| (e + n) % (2 * n)).collect::<Vec<_>>();
        assert!((0..2 * n).all(|e| diamond[diamond[e]] == e && diamond[e] != e));
        assert!((1..2 * n)
            .filter(|l| l % 2 == 1 && *l != n)
            .all(|l| n * l % (2 * n) == n));
        rec["diamond_involution_character_checks"] = json!(true);
        println!("CM15:n{n},p{p},level{},genus{},raw E1/E0[{plus},{minus}],quadratic diamond twist exchanges signs",rec["level"],rec["carrier_genus"]);
        winners.push(rec);
    }
    let external = lm_rows(dir);
    for row in &external {
        let label = row["label"].as_str().unwrap();
        let expected = if label == "77.2.l.a" {
            predicted(5, 1, 1)
        } else {
            assert_eq!(label, "539.2.f.c");
            predicted(5, 2, 0)
        };
        let l = row["lpoly"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().parse::<BigInt>().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(l, expected.into_iter().rev().collect::<Vec<_>>());
    }
    assert_eq!(external.len(), 2);
    crate::round02::save(
        dir,
        "cm15-one-prime-carriers.json",
        &json!({"schema":"koblitz.round15/cm15-v1","scan":all,"winners":winners,"LMFDB_level77_and539_polynomials_match":true,"LMFDB_131_coverage":"no matching row returned in retained query;not evidence against existence","CM_field":"Q(sqrt(-7)),class number1,units{+-1}","character":"infinity type1,finite odd eta at one split prime;gamma*eta(gamma) independent of sign","level":"7*p","bridge":"CM newforms,modular abelian varieties,good reduction,Honda-Tate and geometric diamond twisting;written geometric/arithmetic argument","explicit_maps":false,"subgroup_injection_evaluated":false,"IC_efficiency_verdict":null}),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_norm_matches_independent_level77_database_control() {
        let l = [
            1, 0, -3, 0, 5, 0, -3, 0, -11, 0, -12, 0, 80, 0, -192, 0, 256,
        ];
        let expected = l.into_iter().rev().map(BigInt::from).collect::<Vec<_>>();
        assert_eq!(norm_euler(5, 4, 7), expected);
        assert_ne!(norm_euler(5, 4, 8), expected);
        assert_eq!(norm_euler(5, 4, 8), predicted(5, 2, 0));
    }
}

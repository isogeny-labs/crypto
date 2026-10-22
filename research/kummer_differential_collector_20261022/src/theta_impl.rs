/// Finite Fp theta-constant setup search for the published fast Kummer.
use serde_json::{json, Value};
use std::fs;
#[derive(Clone,Copy,Default,serde::Serialize)]
pub struct Ops { pub m:u64,pub a:u64,pub i:u64 }
thread_local! { static OPS:std::cell::Cell<Ops>=std::cell::Cell::new(Ops{m:0,a:0,i:0}); }
pub fn reset_ops(){OPS.with(|o|o.set(Ops::default()));}
pub fn read_ops()->Ops{OPS.with(|o|o.get())}
fn bump_m(){OPS.with(|o|{let mut x=o.get();x.m+=1;o.set(x);});}
fn bump_a(){OPS.with(|o|{let mut x=o.get();x.a+=1;o.set(x);});}
fn bump_i(){OPS.with(|o|{let mut x=o.get();x.i+=1;o.set(x);});}
fn raw_mul(a:u64,b:u64,p:u64)->u64{a*b%p}
fn raw_pow(mut a:u64,mut n:u64,p:u64)->u64{
    let mut out=1;while n>0{if n&1==1{out=raw_mul(out,a,p);}a=raw_mul(a,a,p);n>>=1;}out
}

fn add(a: u64, b: u64, p: u64) -> u64 {
    bump_a();
    (a + b) % p
}
fn sub(a: u64, b: u64, p: u64) -> u64 {
    bump_a();
    (a + p - b) % p
}
fn mul(a: u64, b: u64, p: u64) -> u64 {
    bump_m();
    a * b % p
}
fn pow(mut a: u64, mut n: u64, p: u64) -> u64 {
    let mut out = 1;
    while n > 0 {
        if n & 1 == 1 {
            out = mul(out, a, p);
        }
        a = mul(a, a, p);
        n >>= 1;
    }
    out
}
fn inv(a: u64, p: u64) -> u64 {
    assert_ne!(a, 0);
    bump_i();
    raw_pow(a, p - 2, p)
}
fn sqrt(a: u64, p: u64) -> Option<u64> {
    if a == 0 {
        return Some(0);
    }
    if pow(a, (p - 1) / 2, p) != 1 {
        return None;
    }
    let root = pow(a, (p + 1) / 4, p);
    assert_eq!(mul(root, root, p), a);
    Some(root)
}
fn sum4(a: u64, b: u64, c: u64, d: u64, p: u64) -> u64 {
    add(add(a, b, p), add(c, d, p), p)
}
fn theta_tuple(p: u64, lambda: u64, mu: u64, nu: u64) -> Vec<Value> {
    let mut out = Vec::new();
    let u2 = mul(mul(lambda, nu, p), inv(mu, p), p);
    let Some(u_root) = sqrt(u2, p) else {
        return out;
    };
    for u in [u_root, sub(0, u_root, p)] {
        if u == 0 {
            continue;
        }
        let v = mul(lambda, inv(u, p), p);
        let t = mul(nu, inv(u, p), p);
        if t == 0 || add(t, 1, p) == 0 {
            continue;
        }
        let alpha = mul(sub(t, 1, p), inv(add(t, 1, p), p), p);
        let alpha2 = mul(alpha, alpha, p);
        let up = sub(u, 1, p);
        let vp = sub(v, 1, p);
        let uplus = add(u, 1, p);
        let vplus = add(v, 1, p);
        let numerator = sub(mul(up, up, p), mul(alpha2, mul(uplus, uplus, p), p), p);
        let denominator = sub(mul(vp, vp, p), mul(alpha2, mul(vplus, vplus, p), p), p);
        if denominator == 0 {
            continue;
        }
        let w2 = mul(numerator, inv(denominator, p), p);
        let Some(w_root) = sqrt(w2, p) else {
            continue;
        };
        for w in [w_root, sub(0, w_root, p)] {
            if w == 0 {
                continue;
            }
            let (a, b, c, d, e, f) = (u, 1, mul(v, w, p), w, t, 1);
            let big_a = sum4(a, b, c, d, p);
            let big_b = sum4(a, b, sub(0, c, p), sub(0, d, p), p);
            let big_c = sum4(a, sub(0, b, p), c, sub(0, d, p), p);
            let big_d = sum4(a, sub(0, b, p), sub(0, c, p), d, p);
            if [a, b, c, d, e, f, big_a, big_b, big_c, big_d].contains(&0) {
                continue;
            }
            let denom = [
                mul(b, d, p),
                mul(d, f, p),
                mul(b, f, p),
                mul(big_a, big_b, p),
                sub(mul(a, d, p), mul(b, c, p), p),
                sub(mul(a, c, p), mul(b, d, p), p),
                sub(mul(a, b, p), mul(c, d, p), p),
            ];
            if denom.contains(&0) {
                continue;
            }
            let actual_lambda = mul(mul(a, c, p), inv(denom[0], p), p);
            let actual_mu = mul(mul(c, e, p), inv(denom[1], p), p);
            let actual_nu = mul(mul(a, e, p), inv(denom[2], p), p);
            let alpha_ok = mul(alpha2, mul(big_a, big_b, p), p) == mul(big_c, big_d, p);
            let t_ok = mul(e, inv(f, p), p) == mul(add(1, alpha, p), inv(sub(1, alpha, p), p), p);
            if actual_lambda == lambda && actual_mu == mu && actual_nu == nu && alpha_ok && t_ok {
                out.push(json!({"a":a,"b":b,"c":c,"d":d,"e":e,"f":f,
                    "A":big_a,"B":big_b,"C":big_c,"D":big_d,
                    "alpha":alpha,"lambda":lambda,"mu":mu,"nu":nu,
                    "all_equations_verified":true}));
            }
        }
    }
    out
}
fn main() {
    let input = "research/four_followups_20261021/results/kummer-model-gate.jsonl";
    let output = "research/kummer_theta_20261022/results/theta-models.jsonl";
    let mut lines = String::new();
    let mut count = 0;
    for text in fs::read_to_string(input).unwrap().lines() {
        let row: Value = serde_json::from_str(text).unwrap();
        let p = row["p"].as_u64().unwrap();
        assert_eq!(p % 4, 3);
        let roots: Vec<u64> = row["finite_rational_weierstrass_roots"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_u64().unwrap())
            .collect();
        let c5 = row["quintic_coefficients"][5].as_u64().unwrap();
        assert_eq!(roots.len(), 5);
        let mut ordered_pairs = 0;
        let mut square_pairs = 0;
        let mut eligible_orders = 0;
        let mut theta_tuples = 0;
        let mut selected = None;
        for &r0 in &roots {
            for &r1 in &roots {
                if r0 == r1 {
                    continue;
                }
                ordered_pairs += 1;
                let delta = sub(r1, r0, p);
                let scale = mul(c5, pow(delta, 5, p), p);
                let Some(y_scale) = sqrt(scale, p) else {
                    continue;
                };
                if y_scale == 0 {
                    continue;
                }
                square_pairs += 1;
                let delta_inverse = inv(delta, p);
                let remaining: Vec<u64> = roots
                    .iter()
                    .copied()
                    .filter(|&r| r != r0 && r != r1)
                    .map(|r| mul(sub(r, r0, p), delta_inverse, p))
                    .collect();
                for permutation in [
                    [0, 1, 2],
                    [0, 2, 1],
                    [1, 0, 2],
                    [1, 2, 0],
                    [2, 0, 1],
                    [2, 1, 0],
                ] {
                    eligible_orders += 1;
                    let (lambda, mu, nu) = (
                        remaining[permutation[0]],
                        remaining[permutation[1]],
                        remaining[permutation[2]],
                    );
                    let tuples = theta_tuple(p, lambda, mu, nu);
                    theta_tuples += tuples.len();
                    if selected.is_none() {
                        if let Some(constants) = tuples.into_iter().next() {
                            selected = Some(json!({"r0":r0,"r1":r1,"delta":delta,
                            "sqrt_scale":y_scale,"lambda_mu_nu":[lambda,mu,nu],
                            "constants":constants}));
                        }
                    }
                }
            }
        }
        lines.push_str(&format!(
            "{}\n",
            json!({"schema":"kummer-theta-basefield/v1",
            "p":p,"ordered_pairs":ordered_pairs,"square_pairs":square_pairs,
            "eligible_orders":eligible_orders,"theta_tuples":theta_tuples,
            "fp_theta_tuple_found":selected.is_some(),"selected":selected})
        ));
        count += 1;
    }
    assert_eq!(count, 21);
    fs::create_dir_all("research/kummer_theta_20261022/results").unwrap();
    fs::write(output, lines).unwrap();
}

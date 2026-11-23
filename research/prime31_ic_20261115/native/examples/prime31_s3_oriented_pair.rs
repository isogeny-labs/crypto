//! A denominator-free, oriented quadratic lift of S3 for the ordinary prime31 IC.
//! Algebraic identities are exact; F4 is a bounded diagnostic, not an ECDLP oracle.
//! Uses the native Dickson Base, Arithmetic and existing f4_fp_tower engine.
//!
//! cargo +1.100.0 test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_s3_oriented_pair
//! cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_s3_oriented_pair -- --f4
use prime31_algebraic_ic::{
    check_curve, Arithmetic, Base, PRIME31,
    f4_fp_tower::{f4_tower, RPoly, SquareRule, TowerF4Options, TowerRing},
};
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::time::{Duration, Instant};

const P: u64 = 2_147_483_647;
const DEPTH: usize = 6;
const A: u64 = 847_514_846;
const B: u64 = 2_087_444_724;
const X: usize = 0;
const Z: usize = DEPTH;
const N_TOWER: usize = 2 * DEPTH;

fn add(a: u64, b: u64) -> u64 {
    let v = a + b;
    if v >= P { v - P } else { v }
}
fn sub(a: u64, b: u64) -> u64 {
    if a >= b { a - b } else { a + P - b }
}
fn mul(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % P as u128) as u64
}
fn pow(mut b: u64, mut e: u64) -> u64 {
    let mut a = 1;
    while e > 0 {
        if e & 1 == 1 { a = mul(a, b); }
        b = mul(b, b);
        e >>= 1;
    }
    a
}
fn inv(x: u64) -> u64 {
    assert_ne!(x, 0);
    pow(x, P - 2)
}
fn f(x: u64) -> u64 {
    add(add(mul(mul(x, x), x), mul(A, x)), B)
}
fn s3_coeffs(x: u64, z: u64) -> (u64, u64, u64) {
    let sum = add(x, z);
    let xz = mul(x, z);
    let d = sub(x, z);
    let aa = mul(d, d);
    let bb = sub(0, mul(2, add(mul(sum, add(xz, A)), mul(2, B))));
    let cc = sub(mul(sub(xz, A), sub(xz, A)), mul(mul(4, B), sum));
    (aa, bb, cc)
}
fn s3(x: u64, z: u64, w: u64) -> u64 {
    let (a, b, c) = s3_coeffs(x, z);
    add(add(mul(a, mul(w, w)), mul(b, w)), c)
}
fn quadratic_lift_eliminant(x: u64, z: u64, w: u64) -> u64 {
    // D = 2*x^2 + x*z + a - (z-x)*w.
    // Res_y,t (y^2-f(x), t^2-(w+x+z), 2*y*t-D) gives
    // D^2 - 4*f(x)*(w+x+z), exactly the standard S3.
    let d = sub(add(add(mul(2, mul(x, x)), mul(x, z)), A),
                mul(sub(z, x), w));
    sub(mul(d, d), mul(mul(4, f(x)), add(add(w, x), z)))
}
fn lift_residuals(x: u64, z: u64, w: u64, y: u64, t: u64) -> [u64; 3] {
    [
        sub(mul(y, y), f(x)),
        sub(mul(t, t), add(add(w, x), z)),
        sub(add(mul(2, mul(y, t)), mul(sub(z, x), w)),
            add(add(mul(2, mul(x, x)), mul(x, z)), A)),
    ]
}
fn finite_s3_roots(x: u64, z: u64, ar: &mut Arithmetic) -> Result<Vec<u64>, String> {
    let (a, b, c) = s3_coeffs(x, z);
    if a == 0 {
        if b == 0 { return Err("unexpected diagonal S3 degeneration".into()); }
        return Ok(vec![mul(sub(0, c), inv(b))]);
    }
    let delta = sub(mul(b, b), mul(mul(4, a), c));
    let root = ar.sqrt(delta, P).ok_or("non-square S3 discriminant")?;
    if root == 0 { return Err("unexpected double S3 root in odd-prime-order base".into()); }
    let denominator = inv(mul(2, a));
    let r1 = mul(sub(root, b), denominator);
    let r2 = mul(sub(sub(0, root), b), denominator);
    if r1 == r2 { return Err("collapsed distinct S3 roots".into()); }
    let mut roots = vec![r1, r2];
    roots.sort_unstable();
    Ok(roots)
}
fn verify_identity_and_lifts() -> Result<Value, String> {
    let e = PRIME31;
    if (e.p, e.a, e.b, e.n) != (P, A, B, 2_147_446_831) {
        return Err("prime31 fixture changed".into());
    }
    let mut ar = Arithmetic::default();
    check_curve(e, &mut ar)?;
    let base = Base::build(e, DEPTH, &mut ar)?;
    if base.points.len() != 31 || base.terminal != 933_677_148 {
        return Err("Dickson B31 base changed".into());
    }
    let mut coefficient_checks = 0u64;
    for &(x, _) in &base.points {
        for &(z, _) in &base.points {
            for w in [0, 1, x, z, 1_234_567] {
                if s3(x, z, w) != quadratic_lift_eliminant(x, z, w) {
                    return Err("bilinear elimination identity differs from S3".into());
                }
                coefficient_checks += 1;
            }
        }
    }
    let mut finite_roots = 0usize;
    let mut diagonal = 0usize;
    let mut distinct_sums = BTreeSet::new();
    for (i, &(x, yi)) in base.points.iter().enumerate() {
        for &(z, yj) in base.points.iter().skip(i) {
            let roots = finite_s3_roots(x, z, &mut ar)?;
            let expected = if x == z { 1 } else { 2 };
            if roots.len() != expected { return Err("S3 root multiplicity changed".into()); }
            if x == z { diagonal += 1; }
            for w in roots {
                if s3(x, z, w) != 0 { return Err("bad S3 field root".into()); }
                let mut found = false;
                let oriented_y = [yi, sub(0, yi)];
                let oriented_z = [yj, sub(0, yj)];
                for &y in &oriented_y {
                    for &yy in &oriented_z {
                        if x == z && y != yy { continue; } // vertical inverse: no affine sum.
                        let t = if x == z {
                            mul(add(mul(3, mul(x, x)), A), inv(mul(2, y)))
                        } else {
                            mul(sub(yy, y), inv(sub(z, x)))
                        };
                        if lift_residuals(x, z, w, y, t) != [0, 0, 0] { continue; }
                        let actual = ar.add(e, Some((x, y)), Some((z, yy)));
                        if actual.is_some_and(|q| q.0 == w) {
                            found = true;
                        } else {
                            return Err("oriented algebraic relation fails group verification".into());
                        }
                    }
                }
                if !found { return Err("S3 root has no oriented rational lift".into()); }
                distinct_sums.insert(w);
                finite_roots += 1;
            }
        }
    }
    if finite_roots != 961 || diagonal != 31 {
        return Err(format!("root counts changed: {finite_roots}, diagonal {diagonal}"));
    }
    Ok(json!({
        "status":"producer-local semantic test [pcc:unverified]",
        "curve_p":P,"curve_order":e.n,"base_size":base.points.len(),
        "S3_eliminant_equality_sample_checks":coefficient_checks,
        "ordered_unordered_pairs":31*32/2,
        "rational_finite_S3_pair_roots":finite_roots,
        "diagonal_doubling_branches":diagonal,
        "signed_rational_lifts_checked":finite_roots,
        "unique_sum_abscissae":distinct_sums.len(),
        "identity_sum_branch":"excluded from finite x; explicit special case required",
    }))
}
fn term(width: usize, coefficient: u64, exponents: &[(usize,u32)])
    -> (Vec<u32>, u64)
{
    let mut powers = vec![0; width];
    for &(index, exponent) in exponents { powers[index] = exponent; }
    (powers, coefficient % P)
}
fn s3_raw(width: usize) -> Vec<(Vec<u32>,u64)> {
    let t = |c, ex: &[(usize,u32)]| term(width,c,ex);
    let w = N_TOWER;
    vec![
        t(1,&[(X,2),(w,2)]),
        t(1,&[(Z,2),(w,2)]),
        t(sub(0,2),&[(X,1),(Z,1),(w,2)]),
        t(sub(0,2),&[(X,2),(Z,1),(w,1)]),
        t(sub(0,2),&[(X,1),(Z,2),(w,1)]),
        t(sub(0,mul(2,A)),&[(X,1),(w,1)]),
        t(sub(0,mul(2,A)),&[(Z,1),(w,1)]),
        t(sub(0,mul(4,B)),&[(w,1)]),
        t(1,&[(X,2),(Z,2)]),
        t(sub(0,mul(2,A)),&[(X,1),(Z,1)]),
        t(mul(A,A),&[]),
        t(sub(0,mul(4,B)),&[(X,1)]),
        t(sub(0,mul(4,B)),&[(Z,1)]),
    ]
}
fn oriented_raw(width: usize) -> Vec<Vec<(Vec<u32>,u64)>> {
    let t = |c, ex: &[(usize,u32)]| term(width,c,ex);
    let y = N_TOWER;
    let slope = N_TOWER+1;
    let w = N_TOWER+2;
    vec![
        vec![
            t(1,&[(y,2)]),
            t(sub(0,1),&[(X,3)]),
            t(sub(0,A),&[(X,1)]),
            t(sub(0,B),&[]),
        ],
        vec![
            t(1,&[(slope,2)]),
            t(sub(0,1),&[(w,1)]),
            t(sub(0,1),&[(X,1)]),
            t(sub(0,1),&[(Z,1)]),
        ],
        vec![
            t(2,&[(y,1),(slope,1)]),
            t(1,&[(Z,1),(w,1)]),
            t(sub(0,1),&[(X,1),(w,1)]),
            t(sub(0,2),&[(X,2)]),
            t(sub(0,1),&[(X,1),(Z,1)]),
            t(sub(0,A),&[]),
        ],
    ]
}
fn ring(terminal: u64, n_free: usize) -> TowerRing {
    let mut rules = Vec::with_capacity(N_TOWER);
    for block in 0..2 {
        for level in 0..DEPTH {
            let last = level+1 == DEPTH;
            rules.push(SquareRule {
                next:(!last).then_some((block*DEPTH+level+1) as u8),
                a:0, b:u64::from(!last), c:0,
                d:if last {(terminal+2)%P} else {2},
            });
        }
    }
    TowerRing{p:P, rules, n_free}
}
fn factor_base_abscissa_poly(pts: &[(u64,u64)]) -> Vec<u64> {
    let mut poly = vec![1];
    for &(x,_) in pts {
        let mut next = vec![0;poly.len()+1];
        for (i, &c) in poly.iter().enumerate() {
            next[i] = add(next[i], mul(c, sub(0,x)));
            next[i+1] = add(next[i+1], c);
        }
        poly = next;
    }
    poly
}
fn factor_base_equation(ring: &TowerRing, poly: &[u64], at: usize) -> RPoly {
    let width = N_TOWER + ring.n_free;
    let raw: Vec<_> = poly.iter().enumerate().filter(|(_,c)|**c != 0)
        .map(|(degree,&c)|term(width,c,&[(at,degree as u32)])).collect();
    ring.from_raw(&raw)
}
fn f4_arms(caps: &[u64], exact_only: bool) -> Result<Value, String> {
    let mut ar = Arithmetic::default();
    let e = PRIME31;
    check_curve(e,&mut ar)?;
    let base = Base::build(e,DEPTH,&mut ar)?;
    if base.points.len()!=31 || base.terminal!=933_677_148 {
        return Err("expected prime31 B31 Dickson base".into());
    }
    let base_poly = factor_base_abscissa_poly(&base.points);
    if base_poly.len() != 32 || base_poly.last()!=Some(&1) {
        return Err("factor-base polynomial is not degree 31 monic".into());
    }
    let mut out = Vec::new();
    for &cap in caps {
        for exact_base in [false,true] {
            if exact_only && !exact_base { continue; }
            for oriented in [false,true] {
            let n_free = if oriented {3} else {1};
            let ring = ring(base.terminal,n_free);
            ring.check();
            let width = N_TOWER+n_free;
            let started = Instant::now();
            let mut input:Vec<RPoly> = if oriented {
                oriented_raw(width).into_iter().map(|eq|ring.from_raw(&eq)).collect()
            } else {
                vec![ring.from_raw(&s3_raw(width))]
            };
            if exact_base {
                input.push(factor_base_equation(&ring,&base_poly,X));
                input.push(factor_base_equation(&ring,&base_poly,Z));
            }
            let setup_ns = started.elapsed().as_nanos();
            let opts = TowerF4Options::new(12)
                .with_budget(Duration::from_secs(5))
                .with_max_nnz(cap).with_max_dense(cap);
            let started = Instant::now();
            let rep = f4_tower(&input,&ring,&opts);
            let full_f4_ns = started.elapsed().as_nanos();
            let censored = rep.oversize || rep.timed_out || rep.pairs_above_bound>0
                || rep.staircase_at_stop.is_some();
            let verdict = if censored {"undetermined"}
                else if rep.inconsistent {"complete_refutation"}
                else {"basis_only_no_root_extraction"};
            out.push(json!({
                "arm":if oriented {"oriented_chord_tangent"} else {"S3_x_only"},
                "exact_B31_membership":exact_base,
                "F4_cap":cap,
                "polynomial_count":input.len(),
                "tower_variables":N_TOWER,"free_variables":n_free,
                "input_terms_after_tower_nf":input.iter().map(RPoly::len).collect::<Vec<_>>(),
                "setup_ns":setup_ns,"whole_f4_wall_ns":full_f4_ns,
                "verdict":verdict,"oversize":rep.oversize,
                "timed_out":rep.timed_out,
                "degree_reached":rep.degree_reached,
                "solving_degree_max":rep.solving_degree_max,
                "max_cols":rep.max_cols,"max_nnz":rep.max_nnz,
                "max_dense_entries":rep.max_dense_entries,
                "muladds":rep.muladds,"steps":rep.steps,
                "basis_elements":rep.basis.len(),
                "f4_internal_reported_ms":rep.ms,
                "root_extraction":"not_implemented",
                "limits":{"degree":12,"max_nnz":cap,"max_dense":cap,"seconds":5},
            }));
            }
        }
    }
    Ok(json!({
        "standing":"[pcc:unverified]",
        "scope":if exact_only {"paired B31 30k/60k cap ladder; no roots or ECDLP"} else {"fixed S3 two-leaf structural F4 ablation; not m8 target oracle or ECDLP"},
        "p":P,"n":e.n,"factor_base_size":base.points.len(),
        "note":"unrestricted Dickson64 arms have unequal Fp projections because the oriented ideal enforces first-point liftability; compare exact B31 membership arms only for same rational projection",
        "arms":out
    }))
}
fn main() -> Result<(), String> {
    let args:Vec<String>=std::env::args().skip(1).collect();
    if args.len()>1 || args.first().is_some_and(|v|v!="--f4" && v!="--verify" && v!="--ladder") {
        return Err("usage: prime31_s3_oriented_pair [--verify|--f4|--ladder]".into());
    }
    let identity = verify_identity_and_lifts()?;
    println!("{}",serde_json::to_string_pretty(&identity).map_err(|x|x.to_string())?);
    if args.first().is_some_and(|v|v=="--f4") {
        println!("{}",serde_json::to_string_pretty(&f4_arms(&[15_000],false)?).map_err(|x|x.to_string())?);
    }
    if args.first().is_some_and(|v|v=="--ladder") {
        println!("{}",serde_json::to_string_pretty(&f4_arms(&[30_000,60_000],true)?).map_err(|x|x.to_string())?);
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    #[test]
    fn prime31_all_signed_pair_projections() {
        super::verify_identity_and_lifts().expect("S3 / oriented projection mismatch");
    }
    #[test]
    fn chord_tangent_diagonal_and_vertical_exception() {
        let x=super::PRIME31.g.0;
        let y=super::PRIME31.g.1;
        let t=super::mul(
            super::add(super::mul(3,super::mul(x,x)),super::A),
            super::inv(super::mul(2,y))
        );
        let w=super::sub(super::mul(t,t),super::add(x,x));
        assert_eq!(super::lift_residuals(x,x,w,y,t),[0,0,0]);
        let ar=prime31_algebraic_ic::Arithmetic::default();
        assert_eq!(ar.neg(super::PRIME31,Some((x,y))).unwrap().0,x);
    }
}

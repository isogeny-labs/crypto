//! Matched Rust F4 research instrument for the Prime31 m=2 Semaev relation
//! and m=4 factor-base presentation. No external CAS, no non-Rust generator.
//! All arms reuse crypto_lib::cryptanalysis::f4_fp on the SAME field and host.
//! Run (original repository):
//!   cargo run --release --locked --example prime31_trace_f4_pilot
//!   cargo run --release --locked --example prime31_trace_f4_pilot -- --m4
//!   cargo run --release --locked --example prime31_trace_f4_pilot -- --m4-s3
//!   cargo run --release --locked --example prime31_trace_f4_pilot -- --circuit
//! Use --m4 only to include the m4 membership-only stage; it is NOT an S5 solve.
//! This file has no native compile/run receipt yet, so no speed result is claimed.

use crypto_lib::cryptanalysis::f4_fp::{f4, normalise, F4Options, Ordering, Poly};
use crypto_lib::cryptanalysis::semaev_prime_poly::Fp;
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

type Mpoly = BTreeMap<Vec<u32>, u64>;

const P: u64 = 2_147_483_647;
const A: u64 = 847_514_846;
const B: u64 = 2_087_444_724;
const GX: u64 = 625_037_715;
const X2G: u64 = 1_923_055_775;
const X3G: u64 = 265_866_980;

fn constant(n: usize, x: u64) -> Mpoly {
    let mut ans = Mpoly::new();
    if x != 0 {
        ans.insert(vec![0; n], x);
    }
    ans
}

fn var(n: usize, i: usize) -> Mpoly {
    let mut e = vec![0; n];
    e[i] = 1;
    let mut ans = Mpoly::new();
    ans.insert(e, 1);
    ans
}

fn scale(f: Fp, a: &Mpoly, k: u64) -> Mpoly {
    if k % f.p == 0 {
        return Mpoly::new();
    }
    a.iter()
        .map(|(e, &c)| (e.clone(), f.mul(c, k % f.p)))
        .filter(|(_, c)| *c != 0)
        .collect()
}

fn add(f: Fp, a: &Mpoly, b: &Mpoly) -> Mpoly {
    let mut r = a.clone();
    for (e, &c) in b {
        let v = r.entry(e.clone()).or_insert(0);
        *v = f.add(*v, c);
    }
    r.retain(|_, c| *c != 0);
    r
}

fn sub(f: Fp, a: &Mpoly, b: &Mpoly) -> Mpoly {
    add(f, a, &scale(f, b, f.neg(1)))
}

fn mul(f: Fp, a: &Mpoly, b: &Mpoly) -> Mpoly {
    let mut r = Mpoly::new();
    for (ex, &cx) in a {
        for (ey, &cy) in b {
            let e: Vec<u32> = ex.iter().zip(ey).map(|(x, y)| x + y).collect();
            let v = r.entry(e).or_insert(0);
            *v = f.add(*v, f.mul(cx, cy));
        }
    }
    r.retain(|_, c| *c != 0);
    r
}

fn mul_variable(f: Fp, a: &Mpoly, index: usize, negative: bool) -> Mpoly {
    let mut result = Mpoly::new();
    for (e, &c) in a {
        let mut ex = e.clone();
        ex[index] += 1;
        result.insert(ex, if negative { f.neg(c) } else { c });
    }
    result
}

fn poly_from_roots(f: Fp, xs: &[u64]) -> Vec<u64> {
    let mut h = vec![1];
    for &x in xs {
        let mut next = vec![0; h.len() + 1];
        for (i, &c) in h.iter().enumerate() {
            next[i] = f.sub(next[i], f.mul(c, x));
            next[i + 1] = f.add(next[i + 1], c);
        }
        h = next;
    }
    h
}

fn membership(f: Fp, roots: &[u64], n: usize, index: usize) -> Mpoly {
    let x = var(n, index);
    roots.iter().fold(constant(n, 1), |acc, &w| {
        mul(f, &acc, &sub(f, &x, &constant(n, w)))
    })
}

/// Symbolic Newton moments. Output G_j by exact lower-unitriangular
/// preconditioning and raw H_j for the matched unconditioned reference.
fn moment_system(f: Fp, h: &[u64], m: usize) -> (Vec<Mpoly>, Vec<Mpoly>) {
    let max_power = h.len() + m - 2;
    let mut ps = vec![constant(m, m as u64)];
    for k in 1..=max_power {
        let mut next = Mpoly::new();
        for i in 1..=m.min(k.saturating_sub(1)) {
            let shift = mul_variable(f, &ps[k - i], i - 1, i % 2 == 0);
            next = add(f, &next, &shift);
        }
        if k <= m {
            let coef = if k % 2 == 0 { f.neg(k as u64) } else { k as u64 };
            next = add(f, &next, &scale(f, &var(m, k - 1), coef));
        }
        ps.push(next);
    }
    let mut raw: Vec<Mpoly> = Vec::new();
    for j in 0..m {
        let mut p = Mpoly::new();
        for (k, &coef) in h.iter().enumerate() {
            p = add(f, &p, &scale(f, &ps[k + j], coef));
        }
        raw.push(p);
    }
    let mut reduced = Vec::new();
    for j in 0..m {
        let mut p = raw[j].clone();
        for a in 1..=j {
            let term = mul_variable(f, &raw[j - a], a - 1, a % 2 == 1);
            p = add(f, &p, &term);
        }
        reduced.push(p);
    }
    for (j, p) in reduced.iter().enumerate() {
        let degree = max_degree(p);
        assert!(degree <= (h.len() - 1) as u32,
                "G_{j} degree {degree} exceeds B={}", h.len() - 1);
    }
    (raw, reduced)
}

/// A quadratic arithmetic-circuit lift of Newton power sums.
/// Variables: e1..em, P1..P_(B+m-1). P0=m is a constant.
/// Each recurrence fixes its P_k uniquely in terms of earlier variables;
/// H_j=SUM h_k P_(j+k)=0 is LINEAR in the P variables.
/// All generated equations have total degree <=2, but n_vars=O(B).
/// This trades dense G(e) expansion for B extra unknowns; not necessarily
/// faster for F4, and the setup/LA bill must count the extra dimensions.
fn moment_circuit(f: Fp, h: &[u64], m: usize) -> Vec<Mpoly> {
    assert!(m>0 && (m as u64)<f.p && h.len()>=2);
    let b=h.len()-1;
    let max_p=b+m-1;
    let n=m+max_p;
    let p_k=|k:usize| -> Mpoly {
        if k==0 { constant(n,m as u64) } else { var(n,m+k-1) }
    };
    let mut equations=Vec::new();
    for k in 1..=max_p {
        let mut eq=p_k(k);
        for i in 1..=m.min(k.saturating_sub(1)) {
            let product=mul(f,&var(n,i-1),&p_k(k-i));
            if i%2==1 {
                eq=sub(f,&eq,&product);
            } else {
                eq=add(f,&eq,&product);
            }
        }
        if k<=m {
            let coef=if k%2==1 {k as u64} else {f.neg(k as u64)};
            eq=sub(f,&eq,&scale(f,&var(n,k-1),coef));
        }
        assert!(max_degree(&eq)<=2);
        equations.push(eq);
    }
    for j in 0..m {
        let mut eq=Mpoly::new();
        for (k,&coef) in h.iter().enumerate() {
            eq=add(f,&eq,&scale(f,&p_k(j+k),coef));
        }
        assert!(max_degree(&eq)<=1);
        equations.push(eq);
    }
    equations
}

fn max_degree(p: &Mpoly) -> u32 {
    p.keys().map(|e| e.iter().sum::<u32>()).max().unwrap_or(0)
}

fn to_f4(f: Fp, p: &Mpoly) -> Poly {
    let terms: Vec<_> = p.iter().map(|(e, &c)| (e.clone(), c)).collect();
    normalise(&terms, f.p, Ordering::Grevlex)
}

fn evaluate(f: Fp, p: &Mpoly, values: &[u64]) -> u64 {
    let mut res = 0;
    for (e, &c) in p {
        let mut term = c;
        for (&exponent, &value) in e.iter().zip(values) {
            term = f.mul(term, f.pow(value, exponent as u64));
        }
        res = f.add(res, term);
    }
    res
}

/// S3(e1,e2,z) in a polynomial third coordinate z; specialize z to
/// a constant for the final target or keep it as an unrestricted auxiliary.
fn s3_symmetric_expr(f: Fp, e1: &Mpoly, e2: &Mpoly, z: &Mpoly) -> Mpoly {
    let n = e1.keys().next().expect("nonzero e1 variable").len();
    let d = sub(f, &mul(f, e1, e1), &scale(f, e2, 4));
    let mid = add(f, &mul(f, e1, &add(f, e2, &constant(n, A))), &constant(n, f.mul(2, B)));
    let c = sub(f, e2, &constant(n, A));
    let c0 = sub(f, &mul(f, &c, &c), &scale(f, e1, f.mul(4, B)));
    add(f, &sub(f, &mul(f, &d, &mul(f, z, z)), &scale(f, &mul(f, &mid, z), 2)), &c0)
}

/// For fixed z, S3 is quadratic in pair invariants e1,e2, whereas
/// ordered-coordinate S3 is quartic in x1,x2.
fn s3_symmetric(f: Fp, e1: &Mpoly, e2: &Mpoly, z: u64) -> Mpoly {
    let n = e1.keys().next().expect("nonzero e1 variable").len();
    s3_symmetric_expr(f, e1, e2, &constant(n, z))
}

fn s3_ordered_expr(f: Fp, x: &Mpoly, y: &Mpoly, z: &Mpoly) -> Mpoly {
    let e1 = add(f, x, y);
    let e2 = mul(f, x, y);
    s3_symmetric_expr(f, &e1, &e2, z)
}

/// Same S3 in ordered coordinates with a fixed target x.
fn s3_ordered(f: Fp, x: &Mpoly, y: &Mpoly, z: u64) -> Mpoly {
    let e1 = add(f, x, y);
    let e2 = mul(f, x, y);
    s3_symmetric(f, &e1, &e2, z)
}

fn prime31_base(f: Fp, size: usize) -> Vec<u64> {
    let mut w = vec![GX, X2G];
    for x in 0..1000 {
        let rhs = f.add(f.add(f.mul(f.mul(x, x), x), f.mul(A, x)), B);
        if (rhs == 0 || f.pow(rhs, (P - 1) / 2) == 1) && !w.contains(&x) {
            w.push(x);
        }
        if w.len() == size {
            return w;
        }
    }
    panic!("insufficient rational factor-base x coordinates")
}

fn run_arm(f: Fp, title: &str, polynomials: &[Mpoly], budget: Duration, bound: u32) {
    let n = polynomials[0].first_key_value().unwrap().0.len();
    let n_terms: usize = polynomials.iter().map(Mpoly::len).sum();
    let mut input = Vec::new();
    let setup = Instant::now();
    for p in polynomials {
        input.push(to_f4(f, p));
    }
    let setup_ms = setup.elapsed().as_secs_f64() * 1000.0;
    let opts = F4Options::new(Ordering::Grevlex, bound).with_budget(budget);
    let result = f4(&input, n, f.p, &opts);
    println!(
        "{title} vars={n} eqs={} terms={} max_input_degree={} setup_ms={:.3} f4_ms={:.3} degree={} reached={} cols={} rows={} field_ops={} inconsistent={} timeout={} above_bound={}",
        polynomials.len(), n_terms,
        polynomials.iter().map(max_degree).max().unwrap_or(0),
        setup_ms, result.ms, result.solving_degree, result.degree_reached,
        result.max_cols, result.max_rows, result.field_ops,
        result.inconsistent, result.timed_out, result.pairs_above_bound
    );
}

/// Actual planted Semaev relation G + 2G = 3G on the ordinary cofactor-one
/// Prime31 curve. This is an F4 relation-solver STAGE, not an unknown DLP.
fn run_m2(f: Fp, base: &[u64], budget: Duration, include_circuit: bool) {
    let h = poly_from_roots(f, base);
    let (raw, reduced) = moment_system(f, &h, 2);
    let s = f.add(GX, X2G);
    let t = f.mul(GX, X2G);
    let ordered = vec![
        s3_ordered(f, &var(2, 0), &var(2, 1), X3G),
        membership(f, base, 2, 0),
        membership(f, base, 2, 1),
    ];
    let sym_s3 = s3_symmetric(f, &var(2, 0), &var(2, 1), X3G);
    assert_eq!(evaluate(f, &ordered[0], &[GX, X2G]), 0);
    assert_eq!(evaluate(f, &sym_s3, &[s, t]), 0);
    for p in &raw {
        assert_eq!(evaluate(f, p, &[s, t]), 0);
    }
    for p in &reduced {
        assert_eq!(evaluate(f, p, &[s, t]), 0);
    }
    assert_eq!(max_degree(&sym_s3), 2);
    assert_eq!(max_degree(&ordered[0]), 4);
    let arm_h = vec![sym_s3.clone(), raw[0].clone(), raw[1].clone()];
    let arm_g = vec![sym_s3, reduced[0].clone(), reduced[1].clone()];
    println!("m2 planted relation x(G)={GX}, x(2G)={X2G}, x(3G)={X3G}, B={}",base.len());
    run_arm(f, "ordered-plus-S3", &ordered, budget, (base.len()*2+6) as u32);
    run_arm(f, "sym-raw-H-plus-S3", &arm_h, budget, (base.len()*2+6) as u32);
    run_arm(f, "sym-degreeB-G-plus-S3", &arm_g, budget, (base.len()*2+6) as u32);
    if include_circuit {
        let mut circuit=moment_circuit(f,&h,2);
        let n=2+(base.len()+2-1);
        let eq_s3=s3_symmetric(f,&var(n,0),&var(n,1),X3G);
        circuit.push(eq_s3);
        let xcoords=[GX,X2G];
        let mut pows=Vec::new();
        for k in 1..=base.len()+1 {
            pows.push(f.add(f.pow(xcoords[0],k as u64),f.pow(xcoords[1],k as u64)));
        }
        let mut vals=vec![s,t];
        vals.extend(pows);
        assert_eq!(vals.len(),n);
        for eq in &circuit {
            assert_eq!(evaluate(f,eq,&vals),0,"quadratic power-circuit planted witness");
        }
        run_arm(f, "sym-quadratic-Newton-circuit-plus-S3", &circuit,budget,
                (base.len()*2+6) as u32);
    }
}

/// Optional m4 membership ONLY: no S5/relations; controls cost of membership
/// preconditioning on precisely the same four-element source factor base.
fn run_m4_membership(f: Fp, base: &[u64], budget: Duration) {
    let h = poly_from_roots(f, base);
    let (raw, reduced) = moment_system(f, &h, 4);
    let xs = [GX, X2G, GX, X2G];
    let e1 = f.add(f.add(xs[0], xs[1]), f.add(xs[2], xs[3]));
    let mut es = vec![0; 4];
    for (i, &x) in xs.iter().enumerate() {
        for j in (1..=i).rev() {es[j]=f.add(es[j],f.mul(x,es[j-1]));}
        es[0]=f.add(es[0],x);
    }
    assert_eq!(es[0],e1);
    for p in raw.iter().chain(reduced.iter()) {assert_eq!(evaluate(f,p,&es),0);}
    let ordered: Vec<Mpoly> = (0..4).map(|i| membership(f, base, 4, i)).collect();
    println!("m4 membership-only, NOT an S5 relation solve; B={}",base.len());
    run_arm(f, "m4-ordered-membership", &ordered, budget, (base.len()*2+6) as u32);
    run_arm(f, "m4-sym-raw-H-membership", &raw, budget, (base.len()*2+6) as u32);
    run_arm(f, "m4-sym-degreeB-G-membership", &reduced, budget, (base.len()*2+6) as u32);
}

/// Reindex a two-variable symmetric membership equation into one
/// of two independent pair-invariant slots in the six-variable S3 chain.
fn pair_embed(p: &Mpoly, n: usize, start: usize) -> Mpoly {
    assert!(start+2<=n);
    let mut out = Mpoly::new();
    for (ex, &c) in p {
        assert_eq!(ex.len(),2);
        let mut dest=vec![0;n];
        dest[start]=ex[0];
        dest[start+1]=ex[1];
        out.insert(dest,c);
    }
    out
}

/// Planted genuine four-summand instance:
/// G + 2G + 3G + 4G = 10G, u=x(3G), v=x(7G).
/// Original 6-variable ordered S3 chain versus 6-variable two-pair
/// symmetric raw/reduced equations, same actual source curve and target.
/// Kept optional because every native F4 arm has an independent time budget.
fn run_m4_s3_relation(f: Fp, budget: Duration) {
    const X4G:u64 = 1_731_809_510;
    const X7G:u64 = 864_723_287;
    const X10G:u64 = 1_344_580_913;
    let base=[GX,X2G,X3G,X4G];
    let h=poly_from_roots(f,&base);
    let (raw,reduced)=moment_system(f,&h,2);
    let x:Vec<Mpoly>=(0..6).map(|i|var(6,i)).collect();
    let ordered = {
        let mut r=vec![
            s3_ordered_expr(f,&x[0],&x[1],&x[4]),
            s3_ordered_expr(f,&x[2],&x[3],&x[5]),
            s3_ordered(f,&x[4],&x[5],X10G),
        ];
        for i in 0..4 { r.push(membership(f,&base,6,i)); }
        r
    };
    let s12=f.add(GX,X2G);
    let t12=f.mul(GX,X2G);
    let s34=f.add(X3G,X4G);
    let t34=f.mul(X3G,X4G);
    let symvals=[s12,t12,s34,t34,X3G,X7G];
    let ordvals=[GX,X2G,X3G,X4G,X3G,X7G];
    let base_chain=[
        s3_symmetric_expr(f,&x[0],&x[1],&x[4]),
        s3_symmetric_expr(f,&x[2],&x[3],&x[5]),
        s3_ordered(f,&x[4],&x[5],X10G),
    ];
    let mut raw_arm=base_chain.to_vec();
    let mut red_arm=base_chain.to_vec();
    for start in [0,2] {
        for j in 0..2 {
            raw_arm.push(pair_embed(&raw[j],6,start));
            red_arm.push(pair_embed(&reduced[j],6,start));
        }
    }
    for eq in &ordered { assert_eq!(evaluate(f,eq,&ordvals),0,"ordered planted S3 identity"); }
    for eq in raw_arm.iter().chain(red_arm.iter()) {
        assert_eq!(evaluate(f,eq,&symvals),0,"symmetric planted S3 identity");
    }
    println!("m4 chained genuine relation G+2G+3G+4G=10G; u=x(3G), v=x(7G); B=4, 6 variables");
    run_arm(f,"m4-S3-ordered", &ordered,budget,14);
    run_arm(f,"m4-S3-sym-raw-H", &raw_arm,budget,14);
    run_arm(f,"m4-S3-sym-degreeB-G", &red_arm,budget,14);
}

fn main() {
    let f = Fp::new(P);
    let args: Vec<String> = std::env::args().skip(1).collect();
    let m4 = args.iter().any(|x| x=="--m4");
    let m4_s3 = args.iter().any(|x| x=="--m4-s3");
    let circuit = args.iter().any(|x| x=="--circuit");
    let sweep = args.iter().any(|x| x=="--sweep");
    let budget = Duration::from_secs(15);
    // Sweep is a fixed-prime, fixed-planted-target *component scaling* diagnostic.
    // It is not an asymptotic exponent or unknown-index relation-yield experiment.
    // The extra-variable quadratic circuit and 4-coordinate membership controls
    // remain restricted to the original small cells to contain F4 blowup.
    let sizes: &[usize] = if sweep {
        &[3, 4, 6, 8, 12, 16, 24, 32]
    } else {
        &[3, 4]
    };
    for &size in sizes {
        let base = prime31_base(f, size);
        run_m2(f, &base, budget, circuit && size <= 4);
        if m4 && size <= 4 { run_m4_membership(f, &base, budget); }
    }
    if m4_s3 { run_m4_s3_relation(f,budget); }
    println!("No complete relation rank, cold discrete-log recovery or asymptotic advantage asserted.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbolic_degree_and_exact_planted_s3_equivalence() {
        let f=Fp::new(P);
        for size in [2,3,4,6,8] {
            let base=prime31_base(f,size);
            let h=poly_from_roots(f,&base);
            for m in [2usize,3,4] {
                let (raw, reduced)=moment_system(f,&h,m);
                for j in 0..m {
                    assert_eq!(max_degree(&raw[j]),(size+j) as u32);
                    assert_eq!(max_degree(&reduced[j]),size as u32);
                }
                let roots=vec![GX;m];
                let mut es=vec![0u64;m];
                for (i,&x) in roots.iter().enumerate() {
                    for j in (1..=i).rev() { es[j]=f.add(es[j],f.mul(x,es[j-1])); }
                    es[0]=f.add(es[0],x);
                }
                for p in raw.iter().chain(reduced.iter()) {
                    assert_eq!(evaluate(f,p,&es),0);
                }
            }
        }
        let base=prime31_base(f,3);
        let h=poly_from_roots(f,&base);
        let (_,g)=moment_system(f,&h,2);
        let s=f.add(GX,X2G);
        let t=f.mul(GX,X2G);
        assert_eq!(evaluate(f,&s3_symmetric(f,&var(2,0),&var(2,1),X3G),&[s,t]),0);
        assert!(g.iter().all(|p| evaluate(f,p,&[s,t])==0));
    }
}

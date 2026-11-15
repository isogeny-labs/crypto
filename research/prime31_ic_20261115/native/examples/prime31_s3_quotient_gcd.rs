//! Target-dependent S3 resultant/GCD oracle on the inherited ordinary prime31
//! curve. Uses factor-base abscissae + public target only for algebraic discovery.
//! No EC additions, known ordinates, signed-sum tables or rho are consulted
//! until the independently checked candidate-lift step.
//!
//! Development experiment only; classical elimination, not a sub-rho claim.
//! cargo test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_s3_quotient_gcd
//! cargo run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_s3_quotient_gcd

use prime31_algebraic_ic::{check_curve, Arithmetic, Base, Point, Relation, PRIME31};
use serde_json::json;

const P: u64 = 2_147_483_647;
type Pol = Vec<u64>;

fn add(a: u64, b: u64) -> u64 { let t = a + b; if t >= P { t-P } else { t } }
fn sub(a: u64, b: u64) -> u64 { if a >= b { a-b } else { a+P-b } }
fn mul(a: u64, b: u64) -> u64 { ((a as u128 * b as u128) % P as u128) as u64 }
fn power(mut b: u64, mut e: u64) -> u64 {
    let mut a = 1;
    while e != 0 { if e & 1 != 0 { a = mul(a,b); } b=mul(b,b); e >>= 1; }
    a
}
fn trim(mut p: Pol) -> Pol { while p.last() == Some(&0) { p.pop(); } p }
fn sum(a: &[u64], b: &[u64]) -> Pol {
    let mut r = vec![0; a.len().max(b.len())];
    for (i,&v) in a.iter().enumerate() { r[i]=add(r[i],v); }
    for (i,&v) in b.iter().enumerate() { r[i]=add(r[i],v); }
    trim(r)
}
fn neg(a: &[u64]) -> Pol { a.iter().map(|&x| sub(0,x)).collect() }
fn diff(a: &[u64], b: &[u64]) -> Pol { sum(a, &neg(b)) }
fn scale(a: &[u64], k: u64) -> Pol { trim(a.iter().map(|&v| mul(v,k)).collect()) }
fn prod(a: &[u64], b: &[u64]) -> Pol {
    if a.is_empty() || b.is_empty() { return vec![]; }
    let mut r=vec![0; a.len()+b.len()-1];
    for (i,&x) in a.iter().enumerate() {
        if x==0 { continue; }
        for (j,&y) in b.iter().enumerate() {
            if y!=0 { r[i+j]=add(r[i+j],mul(x,y)); }
        }
    }
    trim(r)
}
fn divrem(f: &[u64], d: &[u64]) -> (Pol,Pol) {
    assert!(!d.is_empty());
    let mut r=f.to_vec();
    let mut q=vec![0; f.len().saturating_sub(d.len()).saturating_add(1)];
    let inv=power(*d.last().unwrap(),P-2);
    while r.len()>=d.len() {
        let j=r.len()-d.len();
        let c=mul(*r.last().unwrap(),inv);
        q[j]=c;
        for (i,&v) in d.iter().enumerate() { r[i+j]=sub(r[i+j],mul(c,v)); }
        r=trim(r);
    }
    (trim(q),r)
}
fn rem(f: &[u64], h: &[u64]) -> Pol { divrem(f,h).1 }
fn gcd(mut a: Pol, mut b: Pol) -> Pol {
    while !b.is_empty() { let r=rem(&a,&b); a=b; b=r; }
    if a.is_empty() { return a; }
    let inv=power(*a.last().unwrap(), P-2);
    scale(&a,inv)
}
fn invert(f: &[u64], h: &[u64]) -> Result<Pol,String> {
    let (mut r0,mut r1)=(h.to_vec(),rem(f,h));
    let (mut t0,mut t1)=(vec![],vec![1]);
    while !r1.is_empty() {
        let (q,r)=divrem(&r0,&r1);
        let t=diff(&t0,&prod(&q,&t1));
        r0=r1; r1=r; t0=t1; t1=t;
    }
    if r0.len()!=1 { return Err("nonunit leading coefficient in factor-base quotient".into()); }
    Ok(rem(&scale(&t0,power(r0[0],P-2)),h))
}
fn mprod(a: &[u64], b: &[u64], h: &[u64]) -> Pol { rem(&prod(a,b),h) }
fn eval(p: &[u64], x: u64) -> u64 {
    p.iter().rev().fold(0,|acc,&v| add(mul(acc,x),v))
}
fn base_poly(xs: &[u64]) -> Pol {
    xs.iter().fold(vec![1],|p,&x| prod(&p,&[sub(0,x),1]))
}
// S3(X,Y,t) viewed as quadratic in Y. No ordinates appear anywhere here.
fn s3_y(x: &[u64], t: u64) -> (Pol,Pol,Pol) {
    let tpoly=vec![t];
    let x_t=diff(x,&tpoly);
    let xt=prod(x,&tpoly);
    let x_plus_t=sum(x,&tpoly);
    let aa=prod(&x_t,&x_t);
    let bb=neg(&scale(&sum(&prod(&x_plus_t,&sum(&xt,&[PRIME31.a])),
                            &[mul(2,PRIME31.b)]),2));
    let xterm=diff(&xt,&[PRIME31.a]);
    let cc=diff(&prod(&xterm,&xterm),&scale(&x_plus_t,mul(4,PRIME31.b)));
    (trim(aa),trim(bb),trim(cc))
}
fn s3_scalar(x: u64, t: u64) -> Pol {
    let (a,b,c)=s3_y(&[x],t);
    vec![*c.first().unwrap_or(&0),*b.first().unwrap_or(&0),
         *a.first().unwrap_or(&0)]
        .into_iter().collect::<Pol>().pipe_trim()
}
trait TrimPol { fn pipe_trim(self) -> Self; }
impl TrimPol for Pol { fn pipe_trim(self) -> Self { trim(self) } }

#[derive(Debug)]
struct Algebraic {
    // Candidate abscissa index pairs only. No ordinates are supplied.
    pairs: Vec<(usize,usize)>,
    first_eliminant_degree: usize,
    target_x_in_base: bool,
}
fn quotient_oracle(xs: &[u64], t: u64) -> Result<Algebraic,String> {
    let h=base_poly(xs);
    // (X-t)^2 is a unit modulo h unless t is itself a factor-base x.
    // Split off that linear factor, then solve its exceptional branch directly.
    let special=xs.binary_search(&t).is_ok();
    let hregular=if special {
        let (q,r)=divrem(&h,&[sub(0,t),1]);
        if !r.is_empty() { return Err("failed to split exceptional factor".into()); }
        q
    } else { h.clone() };
    let mut roots=vec![];
    let mut degree=0;
    if hregular.len()>1 {
        let (aa,bb,cc)=s3_y(&[0,1],t);
        let ia=invert(&aa,&hregular)?;
        let beta=mprod(&bb,&ia,&hregular);
        let gamma=mprod(&cc,&ia,&hregular);
        let (mut r0,mut r1)=(vec![],vec![]);
        // Horner reduction of h(Y) by Y^2+beta Y+gamma, in Fp[X]/hregular.
        for &c in h.iter().rev() {
            let nr0=diff(&[c],&mprod(&gamma,&r1,&hregular));
            let nr1=diff(&r0,&mprod(&beta,&r1,&hregular));
            r0=rem(&nr0,&hregular); r1=rem(&nr1,&hregular);
        }
        // resultant of the linear remainder and the monic quadratic:
        // r0^2 - beta*r0*r1 + gamma*r1^2.
        let r01=mprod(&r0,&r1,&hregular);
        let tpol=sum(&diff(&mprod(&r0,&r0,&hregular),
                          &mprod(&beta,&r01,&hregular)),
                     &mprod(&gamma,&mprod(&r1,&r1,&hregular),&hregular));
        let d=gcd(hregular.clone(),rem(&tpol,&hregular));
        degree=d.len().saturating_sub(1);
        for (i,&x) in xs.iter().enumerate() {
            if x!=t && eval(&d,x)==0 { roots.push(i); }
        }
    }
    if special { roots.push(xs.binary_search(&t).unwrap()); }
    let mut pairs=vec![];
    for i in roots {
        // One-variable GCD, not an EC group-sum or a pair-table lookup.
        let q=s3_scalar(xs[i],t);
        let g=if q.is_empty() { h.clone() } else { gcd(h.clone(),q) };
        for (j,&y) in xs.iter().enumerate() {
            if eval(&g,y)==0 { pairs.push((i,j)); }
        }
    }
    pairs.sort_unstable();pairs.dedup();
    Ok(Algebraic {pairs,first_eliminant_degree:degree,target_x_in_base:special})
}
fn signed_lifts(base: &Base, target: Point, pairs: &[(usize,usize)],
                ar: &mut Arithmetic) -> Vec<Relation> {
    let mut rows=vec![];
    for &(i,j) in pairs {
        for si in [-1,1] {
            for sj in [-1,1] {
                let u=Some(base.points[i]);
                let v=Some(base.points[j]);
                let u=if si==1 {u} else {ar.neg(PRIME31,u)};
                let v=if sj==1 {v} else {ar.neg(PRIME31,v)};
                if ar.add(PRIME31,u,v)==target {
                    let mut entries=[(i,si),(j,sj)];
                    entries.sort_unstable();
                    rows.push(Relation{entries});
                }
            }
        }
    }
    rows.sort_unstable();rows.dedup();
    rows
}
fn evaluate(base: &Base, target: Point, ar: &mut Arithmetic)
    -> Result<(Algebraic,Vec<Relation>,u64),String>
{
    let r=target.ok_or("S3 affine target identity excluded; use direct inverse-pair treatment")?;
    if !ar.on_curve(PRIME31,target) {return Err("off-curve target".into());}
    let xs:Vec<u64>=base.points.iter().map(|v|v.0).collect();
    let result=quotient_oracle(&xs,r.0)?;
    let start=ar.counts.group_additions;
    let rows=signed_lifts(base,target,&result.pairs,ar);
    Ok((result,rows,ar.counts.group_additions-start))
}
fn main() -> Result<(),String> {
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar)?;
    assert_eq!(PRIME31.p,P);
    let base=Base::build(PRIME31,6,&mut ar)?;
    assert_eq!(base.points.len(),31);
    let positives=[
        ar.add(PRIME31,Some(base.points[3]),Some(base.points[9])),
        ar.add(PRIME31,Some(base.points[4]),Some(base.points[4])),
        ar.add(PRIME31,Some(base.points[0]),ar.neg(PRIME31,Some(base.points[7]))),
    ];
    let mut samples=vec![];
    for target in positives {
        let target=target.ok_or("unexpected identity planted positive")?;
        let (v,rows,c)=evaluate(&base,Some(target),&mut ar)?;
        if rows.is_empty() {return Err("algebraic positive extraction failed".into());}
        samples.push(json!({"kind":"held-apart positive control","target":target,
            "gcd_degree":v.first_eliminant_degree,"xy_pairs":v.pairs.len(),
            "signed_rows":rows.len(),"candidate_validation_group_additions":c,
            "exceptional_x":v.target_x_in_base}));
    }
    for k in [1,17,137,20261115] {
        let target=ar.scalar(PRIME31,k,Some(PRIME31.g))
            .ok_or("unexpected zero scalar")?;
        let (v,rows,c)=evaluate(&base,Some(target),&mut ar)?;
        // A random public point can genuinely have a relation; do not assume
        // all deterministic controls are negative.
        samples.push(json!({"kind":"public multiple, post-hoc scalar fixture only",
            "target":target,"gcd_degree":v.first_eliminant_degree,
            "xy_pairs":v.pairs.len(),"signed_rows":rows.len(),
            "candidate_validation_group_additions":c,
            "exceptional_x":v.target_x_in_base}));
    }
    println!("{}",json!({"standing":"[pcc:unverified]",
        "curve":"ordinary prime-order F_(2^31-1)","B":base.points.len(),
        "method":"target-dependent S3 univariate resultant/remainder/GCD",
        "polynomial_input_uses_known_ordinates":false,
        "polynomial_input_uses_EC_group_sums":false,
        "full_ECDLP_recovery":false,"samples":samples}));
    if std::env::args().any(|arg| arg == "--compare") {
        // Diagnostic A/B only: same Rust host/inputs, fixed base constructed
        // before timings. A bounded F4 "undetermined" is not a timed loss.
        use prime31_algebraic_ic::{decompose, Limits};
        use std::time::Instant;
        let positive=ar.add(PRIME31,Some(base.points[3]),Some(base.points[9]))
            .ok_or("missing positive target")?;
        let public=ar.scalar(PRIME31,17,Some(PRIME31.g))
            .ok_or("missing public target")?;
        let mut controls=vec![];
        for (kind,target) in [("positive",positive),("public",public)] {
            let started=Instant::now();
            let (algebra,rows,valid_adds)=evaluate(&base,Some(target),&mut ar)?;
            let gcd_ns=started.elapsed().as_nanos();
            let started=Instant::now();
            let old=decompose(PRIME31,&base,Some(target),
                Limits {degree:12,seconds:1,dense_entries:15_000,
                        extraction_nodes:50_000},&mut ar);
            let f4_ns=started.elapsed().as_nanos();
            if old.status=="complete" && old.relations!=rows {
                return Err("existing F4 and exact quotient give different signed relations".into());
            }
            controls.push(json!({
                "kind":kind, "fixed_base_excluded_from_both_stage_intervals":true,
                "gcd_stage_ns":gcd_ns, "algebraic_xy_pairs":algebra.pairs.len(),
                "gcd_post_extraction_validation_group_additions":valid_adds,
                "new_relations":rows.len(),
                "f4_stage_ns":f4_ns, "f4_status":old.status,
                "f4_relations":old.relations.len(),"f4_partial_muladds":old.f4_muladds,
                "f4_solving_degree":old.solving_degree,
                "stage_only_not_cold_solver":true
            }));
        }
        println!("{}",json!({"standing":"[pcc:unverified]","study":"paired developmental S3 oracle stages, NOT full ECDLP costs",
            "same_rust_host":true,"B":base.points.len(),"rows":controls}));
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn exhaustive(base: &Base,target: Point,ar: &mut Arithmetic) -> Vec<Relation> {
        let mut all=vec![];
        for i in 0..base.points.len() {
            for j in 0..base.points.len() {
                all.extend(signed_lifts(base,target,&[(i,j)],ar));
            }
        }
        all.sort_unstable();all.dedup();all
    }
    #[test]
    fn symbolic_oracle_matches_independent_generic_control() {
        let mut ar=Arithmetic::default();
        for depth in [2,6] {
            let base=Base::build(PRIME31,depth,&mut ar).unwrap();
            let mut targets=vec![];
            targets.push(ar.scalar(PRIME31,17,Some(PRIME31.g)));
            for i in 0..base.points.len().min(3) {
                for j in 0..base.points.len().min(3) {
                    targets.push(ar.add(PRIME31,Some(base.points[i]),Some(base.points[j])));
                }
                targets.push(Some(base.points[i])); // exceptional t = base x
            }
            for target in targets.into_iter().flatten() {
                let (found,rows,_)=evaluate(&base,Some(target),&mut ar).unwrap();
                let check=exhaustive(&base,Some(target),&mut ar);
                assert_eq!(rows,check,"algebraic vs signed point addition");
                for &(i,j) in &found.pairs {
                    assert!(i<base.points.len() && j<base.points.len());
                }
            }
        }
    }
}

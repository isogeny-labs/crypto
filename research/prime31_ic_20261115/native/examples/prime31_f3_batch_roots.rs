//! Exact public-target Semaev F3 quadratic-root membership gate for original Prime31.
//! A two-summand index-calculus COMPONENT, not a complete or novel ECDLP solver.
//! No precomputed group-sum tables, known target logs or witness hints enter the extractor.
//! Protocol: ../rr9/F3_BATCH_ROOT_PROTOCOL_20261119.md
use prime31_algebraic_ic::{check_curve, Arithmetic, Counts, Point, PRIME31};
use serde::Serialize;
use serde_json::json;
use std::collections::{BTreeSet, HashMap};
use std::time::Instant;

const P: u64 = 2_147_483_647;
const N: u64 = 2_147_446_831;
const A: u64 = 847_514_846;
const B: u64 = 2_087_444_724;
const SEED: u64 = 0x4633_524f_4f54_3331;
type Witness = (usize, i8, usize, i8);

fn add(x: u64, y: u64) -> u64 {
    let z = x + y;
    if z >= P { z - P } else { z }
}
fn sub(x: u64, y: u64) -> u64 {
    if x >= y { x - y } else { x + P - y }
}
#[derive(Clone, Copy, Default, Debug, Serialize)]
struct FieldOps { mul: u64, inversions: u64 }
impl FieldOps {
    fn mul(&mut self, x: u64, y: u64) -> u64 {
        self.mul += 1;
        // Both operands are reduced modulo 2^31-1, so the product fits in u64.
        (x * y) % P
    }
    fn pow(&mut self, mut x: u64, mut e: u64) -> u64 {
        let mut result = 1;
        while e > 0 {
            if e & 1 != 0 { result = self.mul(result, x); }
            x = self.mul(x, x);
            e >>= 1;
        }
        result
    }
    fn batch_inverse(&mut self, values: &[u64]) -> Vec<u64> {
        let mut prefix = vec![1; values.len() + 1];
        for (i, &v) in values.iter().enumerate() {
            assert_ne!(v, 0, "unhandled singular F3 denominator");
            prefix[i + 1] = self.mul(prefix[i], v);
        }
        self.inversions += 1;
        let mut inv_all = self.pow(prefix[values.len()], P - 2);
        let mut out = vec![0; values.len()];
        for i in (0..values.len()).rev() {
            out[i] = self.mul(prefix[i], inv_all);
            inv_all = self.mul(inv_all, values[i]);
        }
        out
    }
}
fn f3_coefficients(op: &mut FieldOps, r: u64, u: u64) -> (u64,u64,u64) {
    // F3(u,v,r) = A*v^2 + B*v + C.
    let d = sub(r, u);
    let alpha = op.mul(d, d);
    let ru = op.mul(r, u);
    let sum = add(r, u);
    let mix = op.mul(sum, add(ru, A));
    let beta = sub(0, op.mul(2, add(mix, (2 * B) % P)));
    let z = sub(ru, A);
    let gamma = sub(op.mul(z, z), op.mul((4 * B) % P, sum));
    (alpha, beta, gamma)
}
fn f3_value(op: &mut FieldOps, x: u64, y: u64, z: u64) -> u64 {
    let (a,b,c) = f3_coefficients(op,x,z);
    add(add(op.mul(a,op.mul(y,y)),op.mul(b,y)),c)
}
fn base31(ar: &mut Arithmetic) -> Vec<Point> {
    let mut base=Vec::new();
    for x in 0..10_000u64 {
        let rhs=ar.rhs(PRIME31,x);
        if let Some(y)=ar.sqrt(rhs,P) {
            if y!=0 {
                let p=Some((x,y.min(P-y)));
                assert!(ar.on_curve(PRIME31,p));
                base.push(p);
                if base.len()==31 { break; }
            }
        }
    }
    assert_eq!(base.len(),31);
    assert_eq!(base.iter().take(8).map(|q|q.unwrap().0).collect::<Vec<_>>(),
               vec![0,1,2,3,5,6,8,9]);
    base
}
fn splitmix64(seed:&mut u64)->u64 {
    *seed=seed.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z=*seed;
    z=(z^(z>>30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z=(z^(z>>27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z^(z>>31)
}
fn nonzero_scalar(seed:&mut u64)->u64 {
    let threshold=N.wrapping_neg()%N;
    loop {
        let v=splitmix64(seed);
        if v>=threshold {
            let k=v%N;
            if k!=0 {return k;}
        }
    }
}
#[derive(Clone, Debug, Serialize)]
struct Case { kind: &'static str, q: Point }
fn panel(base:&[Point])->Vec<Case> {
    let mut ar=Arithmetic::default();
    let mut seed=SEED;
    let mut out=Vec::new();
    while out.len()<64 {
        let i=(splitmix64(&mut seed)%31) as usize;
        let j=(splitmix64(&mut seed)%31) as usize;
        let pi=if splitmix64(&mut seed)&1==0 {base[i]} else {ar.neg(PRIME31,base[i])};
        let pj=if splitmix64(&mut seed)&1==0 {base[j]} else {ar.neg(PRIME31,base[j])};
        if let Some(q)=ar.add(PRIME31,pi,pj) {
            out.push(Case {kind:"blinded_planted",q:Some(q)});
        }
    }
    for _ in 0..256 {
        let d=nonzero_scalar(&mut seed);
        let q=ar.scalar(PRIME31,d,Some(PRIME31.g));
        assert!(q.is_some());
        out.push(Case {kind:"natural_unlabelled",q});
    }
    for &q in base {
        out.push(Case {kind:"degenerate_x",q});
    }
    assert_eq!(out.len(),351);
    out
}
fn signed(ar:&Arithmetic,p:Point,s:i8)->Point {
    if s==1 {p} else {ar.neg(PRIME31,p)}
}
fn verify_pair(ar:&mut Arithmetic,q:Point,base:&[Point],i:usize,j:usize,
               found:&mut BTreeSet<Witness>) {
    // Only reached after a field-polynomial root matches a public factor-base x.
    for &si in &[-1i8,1] {
        for &sj in &[-1i8,1] {
            let pi=signed(ar,base[i],si);
            let pj=signed(ar,base[j],sj);
            if ar.add(PRIME31,pi,pj)==q {found.insert((i,si,j,sj));}
        }
    }
}
fn base_lookup(base:&[Point])->HashMap<u64,usize> {
    let mut by_x=HashMap::new();
    for (i,&q) in base.iter().enumerate() {
        let x=q.expect("factor base cannot contain infinity").0;
        assert!(by_x.insert(x,i).is_none());
    }
    by_x
}
fn extract_f3(ar:&mut Arithmetic,op:&mut FieldOps,base:&[Point],
              by_x:&HashMap<u64,usize>,q:Point,
              degenerate:&mut u64)->Vec<Witness> {
    let (r,s)=q.expect("nonzero public target");
    let mut coeff=Vec::with_capacity(base.len());
    let mut denoms=Vec::with_capacity(base.len());
    for &p in base {
        let (x,_)=p.unwrap();
        let (alpha,beta,gamma)=f3_coefficients(op,r,x);
        if alpha==0 {
            *degenerate+=1;
            assert_ne!(beta,0,"y=0 excluded on odd prime-order group");
            denoms.push(beta);
        } else {
            denoms.push(add(alpha,alpha));
        }
        coeff.push((alpha,beta,gamma));
    }
    let inverses=op.batch_inverse(&denoms);
    let mut found=BTreeSet::new();
    for (i,&pi) in base.iter().enumerate() {
        let (_,yi)=pi.unwrap();
        let (alpha,beta,gamma)=coeff[i];
        if alpha==0 {
            let root=op.mul(sub(0,gamma),inverses[i]);
            if let Some(&j)=by_x.get(&root) {
                verify_pair(ar,q,base,i,j,&mut found);
            }
        } else {
            // Exact discriminant: beta^2-4*alpha*gamma=(4*s*yi)^2.
            let discr_sqrt=op.mul(op.mul(4,s),yi);
            for numerator in [sub(discr_sqrt,beta),sub(sub(0,beta),discr_sqrt)] {
                let root=op.mul(numerator,inverses[i]);
                if let Some(&j)=by_x.get(&root) {
                    verify_pair(ar,q,base,i,j,&mut found);
                }
            }
        }
    }
    found.into_iter().collect()
}
fn extract_group_reference(ar:&mut Arithmetic,base:&[Point],
                            by_x:&HashMap<u64,usize>,q:Point)->Vec<Witness> {
    let mut found=BTreeSet::new();
    for (i,&pi) in base.iter().enumerate() {
        // Reference only: generic affine R ± P_i, never used by the F3 extractor.
        for candidate in [ar.add(PRIME31,q,pi), ar.add(PRIME31,q,ar.neg(PRIME31,pi))] {
            if let Some((x,_))=candidate {
                if let Some(&j)=by_x.get(&x) {
                    verify_pair(ar,q,base,i,j,&mut found);
                }
            }
        }
    }
    found.into_iter().collect()
}
#[derive(Serialize)]
struct Pass {
    witnesses: Vec<Vec<Witness>>,
    field_ops: FieldOps,
    curve_ops: Counts,
    degenerate_roots: u64,
    elapsed_ns: u128
}
fn run_pass(base:&[Point],by_x:&HashMap<u64,usize>,cases:&[Case],use_f3:bool)->Pass {
    let mut ar=Arithmetic::default();
    let mut op=FieldOps::default();
    let mut degenerate=0;
    let t=Instant::now();
    let mut witnesses=Vec::with_capacity(cases.len());
    for c in cases {
        witnesses.push(if use_f3 {
            extract_f3(&mut ar,&mut op,base,by_x,c.q,&mut degenerate)
        } else {
            extract_group_reference(&mut ar,base,by_x,c.q)
        });
    }
    let elapsed_ns=t.elapsed().as_nanos();
    Pass {witnesses,field_ops:op,curve_ops:ar.counts,
          degenerate_roots:degenerate,elapsed_ns}
}
fn study(rounds:usize)->serde_json::Value {
    assert!(rounds>=1);
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar).expect("unchanged ordinary Prime31 curve");
    assert_eq!((PRIME31.p,PRIME31.a,PRIME31.b,PRIME31.n),(P,A,B,N));
    let base=base31(&mut ar);
    let cases=panel(&base);
    let by_x=base_lookup(&base);
    let setup=ar.counts;
    let mut f3_times=Vec::new();
    let mut group_times=Vec::new();
    let mut first_f3=None;
    let mut first_group=None;
    for iter in 0..rounds {
        let (f3,reference)=if iter%2==0 {
            (run_pass(&base,&by_x,&cases,true),run_pass(&base,&by_x,&cases,false))
        } else {
            let reference=run_pass(&base,&by_x,&cases,false);
            let f3=run_pass(&base,&by_x,&cases,true);
            (f3,reference)
        };
        assert_eq!(f3.witnesses,reference.witnesses,"disagreement in signed full fibre");
        assert!(f3.degenerate_roots>=31);
        assert!(cases.iter().take(64).enumerate()
                    .all(|(i,_)|!f3.witnesses[i].is_empty()));
        f3_times.push(f3.elapsed_ns);
        group_times.push(reference.elapsed_ns);
        if first_f3.is_none() {first_f3=Some(f3);}
        if first_group.is_none() {first_group=Some(reference);}
    }
    f3_times.sort();
    group_times.sort();
    let f3=first_f3.unwrap();
    let reference=first_group.unwrap();
    let summarize=|start:usize,end:usize| {
        let witness=&f3.witnesses[start..end];
        json!({"cases":end-start,"hits":witness.iter().filter(|r|!r.is_empty()).count(),
               "misses":witness.iter().filter(|r|r.is_empty()).count(),
               "ordered_signed_witnesses":witness.iter().map(Vec::len).sum::<usize>()})
    };
    let proofs:Vec<_>=cases.iter().enumerate().filter_map(|(i,c)|
        f3.witnesses[i].first().map(|w|json!({"index":i,"kind":c.kind,
                                           "q":c.q,"witness":w}))).collect();
    json!({
        "schema":"prime31.semaev.f3.batch_root.v1",
        "status":"original_source_rust_experiment",
        "curve":{"p":P,"a":A,"b":B,"n":N,"cofactor":1,"generator":PRIME31.g},
        "seed":format!("{SEED:016x}"),
        "protocol":"research/prime31_ic_20261115/rr9/F3_BATCH_ROOT_PROTOCOL_20261119.md",
        "base":base,"samples":cases,
        "cohorts":{"planted":summarize(0,64),"natural":summarize(64,320),
                   "degenerate":summarize(320,351)},
        "all_full_fibre_comparisons_match":true,
        "field_ops_f3":f3.field_ops,
        "verification_curve_ops_f3":f3.curve_ops,
        "reference_curve_ops":reference.curve_ops,
        "source_setup_curve_ops":setup,
        "f3_degenerate_linear_roots":f3.degenerate_roots,
        "rounds":rounds,"f3_times_ns_sorted":f3_times,
        "generic_reference_times_ns_sorted":group_times,
        "f3_median_ns":f3_times[rounds/2],
        "generic_median_ns":group_times[rounds/2],
        "certificates_first_per_hit":proofs,
        "nonenumerative_two_factor_polynomial_roots":true,
        "forbidden_group_sum_tables":0,
        "generic_group_equivalent_at_two_factor_degree":true,
        "complete_ecdlp_solved":false,
        "pcc_verified":false
    })
}
fn main() {
    println!("{}",serde_json::to_string_pretty(&study(8)).unwrap());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn discriminant_and_roots_exact() {
        let mut ar=Arithmetic::default();
        let base=base31(&mut ar);
        let panel=panel(&base);
        let mut ops=FieldOps::default();
        for c in panel.iter().take(96).chain(panel.iter().skip(320)) {
            let (r,s)=c.q.unwrap();
            for &pi in &base {
                let (x,y)=pi.unwrap();
                let (aa,bb,cc)=f3_coefficients(&mut ops,r,x);
                let lhs=sub(ops.mul(bb,bb),ops.mul(ops.mul(4,aa),cc));
                let ds=ops.mul(ops.mul(4,s),y);
                assert_eq!(lhs,ops.mul(ds,ds));
                if aa!=0 {
                    let inv=ops.pow(add(aa,aa),P-2);
                    for root in [ops.mul(sub(ds,bb),inv),ops.mul(sub(sub(0,bb),ds),inv)] {
                        assert_eq!(f3_value(&mut ops,x,root,r),0);
                    }
                } else {
                    let root=ops.mul(sub(0,cc),ops.pow(bb,P-2));
                    assert_eq!(f3_value(&mut ops,x,root,r),0);
                }
            }
        }
    }
    #[test]
    fn planted_and_natural_full_fibres() {
        let x=study(1);
        assert_eq!(x["all_full_fibre_comparisons_match"],true);
        assert_eq!(x["cohorts"]["planted"]["hits"],64);
        assert!(x["f3_degenerate_linear_roots"].as_u64().unwrap()>=31);
    }
}

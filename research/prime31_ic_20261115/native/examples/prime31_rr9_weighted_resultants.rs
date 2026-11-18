//! Eight distinct weighted-slot RR9 resultant certificate on the original Prime31 curve.
//! Complete public coefficient *verification*, not an unknown-chart extractor or ECDLP solver.
//! Frozen protocol: RR9_WEIGHTED_SLOT_RESULTANT_PROTOCOL_20261118.md
use prime31_algebraic_ic::{check_curve, Arithmetic, Point, PRIME31};
use serde_json::{json, Value};
use std::collections::HashSet;
use std::time::Instant;

const P: u64 = 2_147_483_647;
const B: usize = 31;
const W: [u64; 8] = [1, 3, 5, 7, 11, 13, 17, 19];

fn add(a: u64, b: u64) -> u64 { (a + b) % P }
fn sub(a: u64, b: u64) -> u64 { (a + P - b) % P }
fn mul(a: u64, b: u64) -> u64 { ((a as u128 * b as u128) % P as u128) as u64 }
fn pow(mut a: u64, mut e: u64) -> u64 {
    let mut out = 1;
    while e > 0 {
        if e & 1 != 0 { out = mul(out, a); }
        a = mul(a, a);
        e >>= 1;
    }
    out
}
fn inv(a: u64) -> u64 { assert_ne!(a, 0); pow(a, P - 2) }

fn base(ar: &mut Arithmetic) -> Vec<(u64, u64)> {
    let mut out = Vec::new();
    for x in 0..10_000u64 {
        let rhs = ar.rhs(PRIME31, x);
        if let Some(y) = ar.sqrt(rhs, P) {
            if y != 0 {
                let pt = (x, y.min(P - y));
                assert!(ar.on_curve(PRIME31, Some(pt)));
                out.push(pt);
            }
        }
        if out.len() == B { break; }
    }
    assert_eq!(out.len(), B);
    assert_eq!(out[..8].iter().map(|q| q.0).collect::<Vec<_>>(),
               vec![0, 1, 2, 3, 5, 6, 8, 9]);
    out
}
fn slots(ar: &mut Arithmetic, pts: &[(u64, u64)]) -> Vec<Vec<(u64, u64)>> {
    let mut result = Vec::new();
    let mut xs = HashSet::new();
    for &w in &W {
        let mut row = Vec::new();
        for &pt in pts {
            let q = ar.scalar(PRIME31, w, Some(pt)).expect("nonzero weighted point");
            assert!(ar.on_curve(PRIME31, Some(q)));
            assert!(xs.insert(q.0), "weighted x-coordinate overlaps between slot images");
            row.push(q);
        }
        result.push(row);
    }
    assert_eq!(xs.len(), B * W.len());
    result
}
fn target(t: usize, slots: &[Vec<(u64,u64)>], ar: &mut Arithmetic)
    -> (Point, Vec<(u64,u64)>, Vec<usize>, Vec<i8>)
{
    let mut r = None;
    let mut used = Vec::new();
    let mut indices = Vec::new();
    let mut signs = Vec::new();
    for (j, row) in slots.iter().enumerate() {
        let index = (3*t + 7*j + j*j) % B;
        let sign = if (t + 3*j) % 4 < 2 { 1i8 } else { -1i8 };
        let pt = if sign == 1 { Some(row[index]) } else {
            ar.neg(PRIME31, Some(row[index]))
        }.unwrap();
        used.push(pt);
        r = ar.add(PRIME31, r, Some(pt));
        indices.push(index);
        signs.push(sign);
    }
    assert!(r.is_some(), "planted target unexpectedly identity");
    (r, used, indices, signs)
}

// The eight planted signed points impose eight linear equations in seven
// normalized RR9 coefficients. All point data stays outside the public checker.
fn chart(r: (u64,u64), signed: &[(u64,u64)]) -> [u64;7] {
    let mut mat = [[0u64;8];8];
    for (i, &(x,y)) in signed.iter().enumerate() {
        let powers = [1, x, mul(x,x), mul(mul(x,x),x)];
        let u = sub(x,r.0);
        let v = add(y,r.1);
        for j in 0..4 { mat[i][j] = mul(u,powers[j]); }
        for j in 0..3 { mat[i][4+j] = mul(v,powers[j]); }
        mat[i][7] = sub(0,mul(v,powers[3]));
    }
    for c in 0..7 {
        let k = (c..8).find(|&j|mat[j][c] != 0).expect("interpolation rank 7");
        mat.swap(c,k);
        let inverse = inv(mat[c][c]);
        for j in c..8 { mat[c][j] = mul(mat[c][j],inverse); }
        for i in 0..8 {
            if i==c {continue;}
            let factor = mat[i][c];
            if factor != 0 {
                for j in c..8 {mat[i][j] = sub(mat[i][j],mul(factor,mat[c][j]));}
            }
        }
    }
    assert_eq!(mat[7],[0u64;8],"planted eighth point must satisfy F");
    std::array::from_fn(|i|mat[i][7])
}
fn pmul(a: &[u64],b: &[u64]) -> Vec<u64> {
    let mut out=vec![0;a.len()+b.len()-1];
    for (i,&u) in a.iter().enumerate() {
        for (j,&v) in b.iter().enumerate() {out[i+j]=add(out[i+j],mul(u,v));}
    }
    out
}
fn padd(a: &[u64],b: &[u64],negative:bool) -> Vec<u64> {
    (0..a.len().max(b.len())).map(|i|{
        let u=a.get(i).copied().unwrap_or(0);
        let v=b.get(i).copied().unwrap_or(0);
        if negative {sub(u,v)} else {add(u,v)}
    }).collect()
}
fn monic_m(r: (u64,u64),c: &[u64;7]) -> Vec<u64> {
    let a=&c[..4];
    let v=[c[4],c[5],c[6],1u64];
    let d=[add(mul(r.0,r.0),PRIME31.a),r.0,1];
    let left=pmul(&d,&pmul(&v,&v));
    let aa=pmul(&[sub(0,r.0),1],&pmul(a,a));
    let av: Vec<u64>=pmul(a,&v).into_iter().map(|z|mul(mul(2,r.1),z)).collect();
    let m=padd(&padd(&left,&aa,true),&av,true);
    assert_eq!(m.len(),9);
    assert_eq!(m[8],1);
    m
}
fn eval(p: &[u64],x:u64) -> u64 {
    p.iter().rev().fold(0,|a,&c|add(mul(a,x),c))
}
fn signed_f(r:(u64,u64),c:&[u64;7],q:(u64,u64))->u64 {
    let a=eval(&c[..4],q.0);
    let v=eval(&[c[4],c[5],c[6],1],q.0);
    add(mul(sub(q.0,r.0),a),mul(add(q.1,r.1),v))
}
fn resultant_zeroes(m:&[u64],slot:&[(u64,u64)]) -> (Vec<usize>,u64) {
    let mut roots=Vec::new();
    let mut resultant=1u64;
    for (i,pt) in slot.iter().enumerate() {
        let value=eval(m,pt.0);
        if value==0 {roots.push(i);}
        resultant=mul(resultant,value);
    }
    assert_eq!(resultant==0,!roots.is_empty());
    (roots,resultant)
}
// A target-conditioned verifier: receives the chart, target, and public slots,
// not the planted witness, not an ECDLP scalar, and not generic half-sum tables.
fn verify_chart(r: Point,c:&[u64;7],slots:&[Vec<(u64,u64)>],ar:&mut Arithmetic)
    -> (Vec<usize>,Vec<i8>,Vec<u64>)
{
    let r=r.expect("nonidentity target");
    let m=monic_m(r,c);
    assert_eq!(signed_f(r,c,(r.0,sub(0,r.1))),0,"ninth zero");
    let mut indices=Vec::new();
    let mut signs=Vec::new();
    let mut products=Vec::new();
    let mut sum=None;
    for slot in slots {
        let (roots,product)=resultant_zeroes(&m,slot);
        assert_eq!(roots.len(),1,"unique slot x-root on registered fixtures");
        let i=roots[0];
        let p=slot[i];
        let np=(p.0,sub(0,p.1));
        let good_p=signed_f(r,c,p)==0;
        let good_n=signed_f(r,c,np)==0;
        assert_ne!(good_p,good_n,"exactly one signed root per x");
        let signed=if good_p{p}else{np};
        sum=ar.add(PRIME31,sum,Some(signed));
        indices.push(i);
        signs.push(if good_p{1}else{-1});
        products.push(product);
    }
    assert_eq!(sum,Some(r),"eight signed points must replay target");
    (indices,signs,products)
}
fn overlap_hall_counterexample(slots:&[Vec<(u64,u64)>])->(bool,bool) {
    // Monic M has 8 distinct roots, distributed 1 in slot 0, 2 in slot 1,
    // 1 each in slots 2..6. Duplicate slot 0 as slot 7. All 8 resultants
    // vanish, yet slot 0 and slot 7 compete for a single x root.
    let mut roots=vec![slots[0][0].0,slots[1][0].0,slots[1][1].0];
    for slot in slots.iter().take(7).skip(2) {roots.push(slot[0].0);}
    assert_eq!(roots.len(),8);
    let mut m=vec![1u64];
    for &x in &roots {m=pmul(&m,&[sub(0,x),1]);}
    let mut modified=slots.to_vec();
    modified[7]=slots[0].clone();
    let every_resultant_zero=modified.iter().all(|slot|resultant_zeroes(&m,slot).1==0);
    let roots0=resultant_zeroes(&m,&modified[0]).0;
    let roots7=resultant_zeroes(&m,&modified[7]).0;
    assert_eq!(roots0,vec![0]);
    assert_eq!(roots7,vec![0]);
    // No injective matching: slot 0 and slot 7 share precisely one available root.
    let has_matching=roots0.len()+roots7.len()>=2 && roots0!=roots7;
    (every_resultant_zero,has_matching)
}
fn study()->Value {
    let started=Instant::now();
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar).expect("original cofactor-one prime31 curve");
    let base=base(&mut ar);
    let slots=slots(&mut ar,&base);
    let setup_ns=started.elapsed().as_nanos();
    let mut output=Vec::new();
    let mut algebraic_ns=0u128;
    for t in 0..16usize {
        let (target,signed,indices,signs)=target(t,&slots,&mut ar);
        let r=target.unwrap();
        let c=chart(r,&signed);
        let started=Instant::now();
        let (found_indices,found_signs,resultants)=verify_chart(target,&c,&slots,&mut ar);
        algebraic_ns+=started.elapsed().as_nanos();
        assert_eq!(found_indices,indices);
        assert_eq!(found_signs,signs);
        assert!(resultants.iter().all(|&v|v==0));
        let mut mutated=c;
        mutated[0]=add(mutated[0],1);
        let altered_m=monic_m(r,&mutated);
        let mutated_hit_counts:Vec<usize>=slots.iter()
            .map(|slot|resultant_zeroes(&altered_m,slot).0.len()).collect();
        assert!(mutated_hit_counts.iter().any(|&k|k==0),
            "same-target chart mutation falsely accepted");
        output.push(json!({
            "fixture":t,"target":r,"chart":c,"root_indices":found_indices,
            "root_signs":found_signs,"mutated_root_counts":mutated_hit_counts,
            "valid_eight_resultants":true,"replayed_sum":true
        }));
    }
    let (false_constraints,has_matching)=overlap_hall_counterexample(&slots);
    assert!(false_constraints&&!has_matching);
    json!({
        "schema":"prime31.rr9.weighted-resultants.v1",
        "date":"2026-11-18","p":P,"a":PRIME31.a,"b":PRIME31.b,
        "n":PRIME31.n,"cofactor":1,
        "weights":W,"base_x":base.iter().map(|p|p.0).collect::<Vec<_>>(),
        "unique_weighted_x":B*W.len(),
        "fixtures":output,
        "planted_certificates_verified":16,
        "all_mutations_rejected":true,
        "hall_overlap_all_resultants_zero":false_constraints,
        "hall_overlap_injective_matching":has_matching,
        "setup_elapsed_ns":setup_ns.to_string(),
        "algebraic_component_elapsed_ns":algebraic_ns.to_string(),
        "producer_status":"rust-original-curve-fixtures",
        "unknown_public_chart_extractions":0,
        "complete_unknown_ecdlp_solves":0,
        "matched_complete_solver_benchmarks":0,
        "pcc":"unverified"
    })
}
fn main(){println!("{}",serde_json::to_string_pretty(&study()).unwrap());}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_original_curve_eight_weighted_slot_resultants() {
        let j=study();
        assert_eq!(j["unique_weighted_x"],248);
        assert_eq!(j["planted_certificates_verified"],16);
        assert_eq!(j["fixtures"].as_array().unwrap().len(),16);
        assert_eq!(j["hall_overlap_all_resultants_zero"],true);
        assert_eq!(j["hall_overlap_injective_matching"],false);
        assert_eq!(j["complete_unknown_ecdlp_solves"],0);
    }
    #[test]
    fn coefficients_given_to_verifier_are_target_conditioned() {
        let mut ar=Arithmetic::default();
        check_curve(PRIME31,&mut ar).unwrap();
        let b=base(&mut ar);
        let s=slots(&mut ar,&b);
        let (r,pts,_,_)=target(0,&s,&mut ar);
        let c=chart(r.unwrap(),&pts);
        assert_eq!(signed_f(r.unwrap(),&c,(r.unwrap().0,sub(0,r.unwrap().1))),0);
        assert_eq!(verify_chart(r,&c,&s,&mut ar).0.len(),8);
    }
}
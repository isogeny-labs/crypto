//! Public-target-conditioned RR9 factor/resultant and exact Jacobian.
//! REUSES original prime31_algebraic_ic EC arithmetic, no generic 4+4 point-sum tables.
//! The 16 planted signed words/arm are GRADER FIXTURES ONLY, not candidate inputs.
//! No unknown chart solver, and no complete non-generic ECDLP claim.
use prime31_algebraic_ic::{check_curve, Arithmetic, Point, PRIME31};
use serde_json::{json, Value};
use std::collections::HashSet;
use std::time::Instant;

const P: u64 = 2_147_483_647;
const N: u64 = 2_147_446_831;
const OLD: [u64; 8] = [1, 3, 5, 7, 11, 13, 17, 19];
const TERNARY: [u64; 8] = [1, 3, 9, 27, 81, 243, 729, 2187];

fn add(x: u64, y: u64) -> u64 { (x + y) % P }
fn sub(x: u64, y: u64) -> u64 { (x + P - y) % P }
fn mul(x: u64, y: u64) -> u64 { ((x as u128 * y as u128) % P as u128) as u64 }
fn power(mut x: u64, mut e: u64) -> u64 {
    let mut v = 1;
    while e > 0 {
        if (e & 1) != 0 { v = mul(v, x); }
        x = mul(x, x);
        e >>= 1;
    }
    v
}
fn inverse(x: u64) -> u64 { assert_ne!(x, 0); power(x, P - 2) }

fn canonical_b8(ar: &mut Arithmetic) -> Vec<(u64, u64)> {
    let mut result = Vec::new();
    for x in 0..10_000u64 {
        if let Some(y) = ar.sqrt(ar.rhs(PRIME31, x), P) {
            if y != 0 {
                let q = (x, y.min(P - y));
                assert!(ar.on_curve(PRIME31, Some(q)));
                result.push(q);
                if result.len() == 8 { break; }
            }
        }
    }
    assert_eq!(result.len(), 8);
    assert_eq!(result.iter().map(|q| q.0).collect::<Vec<_>>(),
               vec![0, 1, 2, 3, 5, 6, 8, 9]);
    result
}
fn public_slot_points(ar: &mut Arithmetic, base: &[(u64,u64)], weights: &[u64;8])
    -> Vec<Vec<(u64,u64)>> {
    let mut distinct = HashSet::new();
    let slots: Vec<Vec<(u64,u64)>> = weights.iter().map(|&weight| {
        base.iter().map(|&q| {
            let p = ar.scalar(PRIME31, weight, Some(q)).expect("weighted point is nonzero");
            assert!(distinct.insert(p.0), "weighted slot x overlap");
            p
        }).collect()
    }).collect();
    assert_eq!(distinct.len(), 64);
    slots
}
fn eval(c: &[u64], x: u64) -> u64 {
    c.iter().rev().fold(0, |acc,&a| add(mul(acc,x),a))
}
fn planted_signed_points(ar: &mut Arithmetic, slots: &[Vec<(u64,u64)>], t: usize)
    -> (Point, Vec<(u64,u64)>, Vec<usize>, Vec<i8>) {
    let mut sum = None;
    let mut points = Vec::new();
    let mut indices = Vec::new();
    let mut signs = Vec::new();
    for (j,slot) in slots.iter().enumerate() {
        let i = (3*t + 7*j + j*j) % 8;
        let sign = if (t+3*j)%4 < 2 {1i8} else {-1i8};
        let (x,y) = slot[i];
        let signed = (x, if sign == 1 {y} else {sub(0,y)});
        sum = ar.add(PRIME31, sum, Some(signed));
        points.push(signed); indices.push(i); signs.push(sign);
    }
    assert!(sum.is_some());
    (sum, points, indices, signs)
}
// This grader-only chart recovery is never invoked by the public polynomial API.
// Eight signed points impose an augmented 8x8 matrix for seven coefficients.
fn grader_chart(r: (u64,u64), signed: &[(u64,u64)]) -> [u64;7] {
    let mut mat = [[0u64;8];8];
    for (i,&(x,y)) in signed.iter().enumerate() {
        let x2 = mul(x,x);
        let powers = [1,x,x2,mul(x2,x)];
        let u = sub(x,r.0);
        let v = add(y,r.1);
        for j in 0..4 {mat[i][j] = mul(u,powers[j]);}
        for j in 0..3 {mat[i][4+j] = mul(v,powers[j]);}
        mat[i][7] = sub(0,mul(v,powers[3]));
    }
    for j in 0..7 {
        let pivot = (j..8).find(|&i|mat[i][j]!=0).expect("rank seven");
        mat.swap(j,pivot);
        let inv = inverse(mat[j][j]);
        for k in j..8 {mat[j][k] = mul(mat[j][k],inv);}
        for i in 0..8 {
            if i == j {continue;}
            let factor = mat[i][j];
            if factor != 0 {
                for k in j..8 {
                    mat[i][k] = sub(mat[i][k],mul(factor,mat[j][k]));
                }
            }
        }
    }
    assert_eq!(mat[7],[0u64;8],"eighth planted point violates chart");
    std::array::from_fn(|i|mat[i][7])
}
// M=(x^2+r*x+r^2+a)V^2 - (x-r)A^2 - 2*s*A*V.
// Partial derivatives: da_i=(-2(x-r)A-2sV)*x^i,
// dv_i=(2dV-2sA)*x^i, for i=0,1,2.
fn norm_and_partial(r:(u64,u64), c:&[u64;7], x:u64) -> (u64,[u64;7]) {
    let x2=mul(x,x);
    let xp=[1,x,x2,mul(x2,x)];
    let a=eval(&c[..4],x);
    let v=add(eval(&c[4..],x),xp[3]);
    let u=sub(x,r.0);
    let d=add(add(x2,mul(r.0,x)),add(mul(r.0,r.0),PRIME31.a));
    let m=sub(sub(mul(d,mul(v,v)),mul(u,mul(a,a))),
              mul(mul(2,r.1),mul(a,v)));
    let da=sub(0,add(mul(mul(2,u),a),mul(mul(2,r.1),v)));
    let dv=sub(mul(mul(2,d),v),mul(mul(2,r.1),a));
    let mut grad=[0u64;7];
    for i in 0..4 {grad[i]=mul(da,xp[i]);}
    for i in 0..3 {grad[4+i]=mul(dv,xp[i]);}
    (m,grad)
}
// PUBLIC ALGEBRAIC FRONT END. Inputs are ONLY public R, chart candidate c,
// and public unsigned slot coordinates. Uses 0 elliptic additions, 0 inversions.
// H_j is factored; its 7 partial derivatives use prefix/suffix products.
fn public_resultants_and_jacobian(r:(u64,u64),c:&[u64;7],
                                  slots:&[Vec<(u64,u64)>])
    -> ([u64;8],[[u64;7];8]) {
    assert_eq!(slots.len(),8);
    let mut hs=[0u64;8];
    let mut jac=[[0u64;7];8];
    for (j,slot) in slots.iter().enumerate() {
        assert_eq!(slot.len(),8);
        let mut values=[0u64;8];
        let mut partials=[[0u64;7];8];
        for (i,&(x,_)) in slot.iter().enumerate() {
            let (m,dm)=norm_and_partial(r,c,x);
            values[i]=m; partials[i]=dm;
        }
        let mut prefix=[1u64;9];
        for i in 0..8 {prefix[i+1]=mul(prefix[i],values[i]);}
        hs[j]=prefix[8];
        let mut suffix=1u64;
        for i in (0..8).rev() {
            let other=mul(prefix[i],suffix);
            for k in 0..7 {
                jac[j][k]=add(jac[j][k],mul(other,partials[i][k]));
            }
            suffix=mul(suffix,values[i]);
        }
    }
    (hs,jac)
}
#[derive(Clone,Copy)]
struct Dual { v:u64, d:u64 }
fn constant(x:u64)->Dual { Dual{v:x,d:0} }
fn dadd(x:Dual,y:Dual)->Dual { Dual{v:add(x.v,y.v),d:add(x.d,y.d)} }
fn dsub(x:Dual,y:Dual)->Dual { Dual{v:sub(x.v,y.v),d:sub(x.d,y.d)} }
fn dmul(x:Dual,y:Dual)->Dual {
    Dual{v:mul(x.v,y.v),d:add(mul(x.v,y.d),mul(x.d,y.v))}
}
// INDEPENDENT differentiation path: construct all operations in the
// dual-number ring Fp[epsilon]/epsilon^2, rather than using norm_and_partial.
fn dual_resultant(r:(u64,u64), c:&[u64;7], slot:&[(u64,u64)], axis:usize)
    -> Dual {
    assert!(axis<7);
    let mut result=constant(1);
    for &(x,_) in slot {
        let xx=constant(x); let one=constant(1);
        let x2=dmul(xx,xx);
        let x3=dmul(x2,xx);
        let mut vars=[constant(0);7];
        for k in 0..7 {vars[k]=Dual{v:c[k],d:u64::from(k==axis)};}
        let a=dadd(dadd(vars[0],dmul(vars[1],xx)),
                   dadd(dmul(vars[2],x2),dmul(vars[3],x3)));
        let v=dadd(x3,dadd(dadd(vars[4],dmul(vars[5],xx)),dmul(vars[6],x2)));
        let dd=dadd(dadd(x2,dmul(constant(r.0),xx)),
                    constant(add(mul(r.0,r.0),PRIME31.a)));
        let u=dsub(xx,constant(r.0));
        let cross=dmul(constant(mul(2,r.1)),dmul(a,v));
        let norm=dsub(dsub(dmul(dd,dmul(v,v)),dmul(u,dmul(a,a))),cross);
        result=dmul(result,norm);
        let _=one;
    }
    result
}
fn rank7(mut m:[[u64;7];8])->usize {
    let mut rank=0usize;
    for col in 0..7 {
        let Some(k)=(rank..8).find(|&i|m[i][col]!=0) else {continue;};
        m.swap(rank,k);
        let inv=inverse(m[rank][col]);
        for j in col..7 {m[rank][j]=mul(m[rank][j],inv);}
        for i in (rank+1)..8 {
            let factor=m[i][col];
            if factor!=0 {
                for j in col..7 {m[i][j]=sub(m[i][j],mul(factor,m[rank][j]));}
            }
        }
        rank+=1;
    }
    rank
}
fn algebraic_f(r:(u64,u64), c:&[u64;7], q:(u64,u64))->u64 {
    let a=eval(&c[..4],q.0);
    let v=add(eval(&c[4..],q.0),power(q.0,3));
    add(mul(sub(q.0,r.0),a),mul(add(q.1,r.1),v))
}
fn arm(ar:&mut Arithmetic, base:&[(u64,u64)], name:&str,
       weights:[u64;8], fixture_count:usize)->Value {
    let setup_start=Instant::now();
    let slots=public_slot_points(ar,base,&weights);
    let setup_ns=setup_start.elapsed().as_nanos();
    let mut fixtures=Vec::new();
    let mut eval_ns=0u128;
    let mut agreement_count=0usize;
    let mut rank7_count=0usize;
    for t in 0..fixture_count {
        let (r,pts,indices,signs)=planted_signed_points(ar,&slots,t);
        let target=r.expect("planted sum is nonidentity");
        let c=grader_chart(target,&pts);
        assert!(pts.iter().all(|&q|algebraic_f(target,&c,q)==0));
        let begin=Instant::now();
        let (h,jac)=public_resultants_and_jacobian(target,&c,&slots);
        eval_ns+=begin.elapsed().as_nanos();
        assert_eq!(h,[0;8],"planted chart failed public factor resultants");
        let rk=rank7(jac);
        rank7_count+=usize::from(rk==7);
        assert_eq!(rk,7,"planted resultant Jacobian is not full column rank");
        for j in 0..8 {
            for k in 0..7 {
                let d=dual_resultant(target,&c,&slots[j],k);
                assert_eq!(d.v,h[j],"independent value mismatch");
                assert_eq!(d.d,jac[j][k],"independent dual derivative mismatch");
                agreement_count+=1;
            }
        }
        let mut bad=c;
        bad[0]=add(bad[0],1);
        let (hb,_)=public_resultants_and_jacobian(target,&bad,&slots);
        assert!(hb.iter().any(|&z|z!=0),"perturbed planted chart retained all roots");
        fixtures.push(json!({
            "fixture_only":true,"case":t,"target":target,
            "signed_indices":indices,"signed_signs":signs,
            "chart":c,"resultants":h,"jacobian":jac,"rank":rk,
            "mutated_resultants":hb
        }));
    }
    json!({
        "arm":name,"weights":weights,"b8_base_points":base,
        "disjoint_public_slot_x":64,
        "fixture_count":fixture_count,
        "planted_ranks_full_column":rank7_count,
        "dual_derivatives_verified":agreement_count,
        "timings_ns":{
            "public_support_setup":setup_ns.to_string(),
            "public_polynomial_evaluations_only":eval_ns.to_string()
        },
        "fixtures":fixtures
    })
}
fn study(num:usize)->Value {
    assert!(num>0 && num<=16);
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar).expect("unchanged original curve");
    assert_eq!(PRIME31.n,N);
    let base=canonical_b8(&mut ar);
    let old=arm(&mut ar,&base,"old",OLD,num);
    let ternary=arm(&mut ar,&base,"ternary",TERNARY,num);
    // Exact combinatorial arithmetic, not a computational security claim:
    // (2*B)^m = 16^8 = 2^32, four windows <=4*16^8 = 2^34.
    let support: u128=16u128.pow(8);
    assert_eq!(support,1u128<<32);
    let scaling: Vec<Value>=[31u32,64,96,128,192,256].iter().map(|&bits| {
        json!({"hypothetical_group_size_bits":bits,
               "min_single_window_uniform_shifts_log2_lower_bound":bits.saturating_sub(32),
               "min_four_window_uniform_shifts_log2_lower_bound":bits.saturating_sub(34)})
    }).collect();
    json!({
        "schema":"prime31.rr9.public-resultant-gradient.v1",
        "status":"source-specific producer gate, pcc:unverified",
        "date_utc":"2026-11-19",
        "curve":{"p":P,"a":PRIME31.a,"b":PRIME31.b,"n":N,"cofactor":1},
        "candidate_inputs":"public R, coefficient candidate, public slot x only",
        "generic_group_sum_tables_used":false,
        "public_unknown_chart_extractor_implemented":false,
        "complete_nongeneric_ecdlp_solver_implemented":false,
        "formal_support_words_per_window":support.to_string(),
        "exact_uniform_mean_ordered_witnesses":format!("{}/{}",support,N),
        "fixed_arity_scaling_necessary_bound":scaling,
        "arms":[old,ternary]
    })
}
fn main() {
    println!("{}",serde_json::to_string_pretty(&study(16)).unwrap());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_curve_old_and_ternary_public_rr9_gradient_matches_dual() {
        let v=study(2);
        assert_eq!(v["generic_group_sum_tables_used"],false);
        assert_eq!(v["complete_nongeneric_ecdlp_solver_implemented"],false);
        for a in v["arms"].as_array().unwrap() {
            assert_eq!(a["planted_ranks_full_column"],2);
            assert_eq!(a["dual_derivatives_verified"],112);
        }
    }
    #[test]
    fn exact_fixed_arity_support_bound_and_falsifier() {
        assert_eq!(16u128.pow(8),1u128<<32);
        assert_eq!(4*16u128.pow(8),1u128<<34);
        assert!((16u128.pow(8) as f64)/(N as f64)>2.0);
        // At 256-bit group order, uniform shifts need >= 2^224 for
        // one such fixed-size window, even if algebraic membership is free.
        assert_eq!(256u32-32,224);
        assert_eq!(256u32-34,222);
    }
}

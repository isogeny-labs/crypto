//! Prime31 exact finite factor-base quotient bridge for non-generic m8.
//!
//! H(X)=prod_{admitted x}(X-x) replaces each SIX-level Dickson tower
//! with one squarefree degree-31 rational-base constraint. K(U), built
//! from S3(x_i,x_j,U) *without point ordinates or group sums*, is the
//! exact rational finite pair-sum abscissa support, with multiplicities.
//! This is only preprocessing + an independent semantic test: no target-
//! dependent m8 solve, no runtime superiority, no generic fallback.
//!
//! cargo +1.100.0 test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_m8_quotient_bridge
//! cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_m8_quotient_bridge

use prime31_algebraic_ic::{Arithmetic, Base, PRIME31, check_curve};
use serde_json::json;
use std::collections::BTreeSet;

const P:u64=2_147_483_647;
const A:u64=847_514_846;
const B:u64=2_087_444_724;
fn add(x:u64,y:u64)->u64{let z=x+y;if z>=P{z-P}else{z}}
fn sub(x:u64,y:u64)->u64{if x>=y{x-y}else{x+P-y}}
fn mul(x:u64,y:u64)->u64{((x as u128*y as u128) % P as u128) as u64}
fn product(a:&[u64],b:&[u64])->Vec<u64>{
    if a.is_empty() || b.is_empty(){return vec![]}
    let mut v=vec![0; a.len()+b.len()-1];
    for (i,&x) in a.iter().enumerate(){
        for (j,&y) in b.iter().enumerate(){
            v[i+j]=add(v[i+j],mul(x,y));
        }
    }
    while v.last()==Some(&0){v.pop();}
    v
}
fn value(poly:&[u64],x:u64)->u64{
    poly.iter().rev().fold(0,|acc,&c|add(mul(acc,x),c))
}
fn derivative(poly:&[u64])->Vec<u64>{
    poly.iter().enumerate().skip(1).map(|(i,&c)|mul(i as u64,c)).collect()
}
fn power(mut x:u64,mut e:u64)->u64 {
    let mut out=1u64;
    while e!=0 {
        if e&1!=0{out=mul(out,x);}
        x=mul(x,x);
        e>>=1;
    }
    out
}
// Exact Euclidean check for squarefreeness over Fp; this does NOT
// factor K or enumerate pair roots and is not an attack oracle.
fn remainder(mut a:Vec<u64>,b:&[u64])->Result<Vec<u64>,String>{
    let lead=*b.last().ok_or("division by zero polynomial")?;
    if lead==0{return Err("unnormalized divisor".into());}
    let inverse=power(lead,P-2);
    while a.len()>=b.len(){
        let offset=a.len()-b.len();
        let factor=mul(*a.last().ok_or("empty dividend")?,inverse);
        for (j,&coefficient) in b.iter().enumerate(){
            let k=offset+j;
            a[k]=sub(a[k],mul(factor,coefficient));
        }
        while a.last()==Some(&0){a.pop();}
    }
    Ok(a)
}
fn gcd_degree(mut a:Vec<u64>,mut b:Vec<u64>)->Result<usize,String>{
    while !b.is_empty(){
        let remainder=remainder(a,&b)?;
        a=b;
        b=remainder;
    }
    Ok(a.len().saturating_sub(1))
}
fn s3_pair(x:u64,y:u64)->Vec<u64>{
    // S3(x,y,U) on E:y^2=x^3+a x+b. Off-diagonal quadratic,
    // diagonal degree-one; the identity branch is not an affine root.
    let s=add(x,y);
    let xy=mul(x,y);
    let aa=mul(sub(x,y),sub(x,y));
    let bb=sub(0,mul(2,add(mul(s,add(xy,A)),mul(2,B))));
    let cc=sub(mul(sub(xy,A),sub(xy,A)),mul(mul(4,B),s));
    let mut p=vec![cc,bb,aa];
    while p.last()==Some(&0){p.pop();}
    p
}
fn run()->Result<serde_json::Value,String>{
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar)?;
    let base=Base::build(PRIME31,6,&mut ar)?;
    if base.points.len()!=31 || base.values.len()!=64 {
        return Err("prime31 depth-six base changed".into());
    }
    let xs:Vec<_>=base.points.iter().map(|&(x,_)|x).collect();
    let xset:BTreeSet<_>=xs.iter().copied().collect();
    if xset.len()!=31{return Err("non-squarefree base".into());}
    let h=xs.iter().fold(vec![1],|acc,&x|product(&acc,&[sub(0,x),1]));
    if h.len()!=32 || h[31]!=1{return Err("invalid rational-base H".into());}
    let dh=derivative(&h);
    for &x in &xs{
        if value(&h,x)!=0 || value(&dh,x)==0 {
            return Err("H vanishes incorrectly or is inseparable".into());
        }
    }
    for &x in &base.values{
        if (value(&h,x)==0)!=xset.contains(&x) {
            return Err("H admits a non-curve-liftable Dickson x".into());
        }
    }
    // Candidate polynomial constructed solely with PUBLIC factor-base x's.
    // Group additions below are confined to *independent semantic checking*.
    let mut k=vec![1u64];
    let mut quadratics=0u64;
    let mut linears=0u64;
    for (i,&x) in xs.iter().enumerate(){
        for (j,&y) in xs.iter().enumerate().skip(i){
            let s=s3_pair(x,y);
            if i==j {
                if s.len()!=2 {return Err("unexpected doubled-point exceptional S3".into());}
                linears+=1;
            } else {
                if s.len()!=3 {return Err("unexpected non-diagonal exceptional S3".into());}
                quadratics+=1;
            }
            k=product(&k,&s);
        }
    }
    if k.len()!=962 || quadratics!=465 || linears!=31{
        return Err("pair-support algebraic degree mismatch".into());
    }
    // The original fixed B31 support was squarefree in an earlier Python
    // diagnostic. Independently check it now using the Rust Fp Euclidean
    // algorithm before claiming a reduced/separable coordinate quotient.
    let squarefree_gcd_degree=gcd_degree(k.clone(),derivative(&k))?;
    if squarefree_gcd_degree!=0 {
        return Err(format!("K has repeated roots: gcd degree {squarefree_gcd_degree}"));
    }
    let mut pair_tests=0u64;
    for (i,&u) in base.points.iter().enumerate(){
        for &v in base.points.iter().skip(i){
            let plus=ar.add(PRIME31,Some(u),Some(v))
                .ok_or("unexpected identity for fixed positive base ordinates")?;
            if value(&k,plus.0)!=0 {return Err("S3 pair-support misses group sum".into());}
            pair_tests+=1;
            if u!=v {
                let minus=ar.add(PRIME31,Some(u),ar.neg(PRIME31,Some(v)))
                    .ok_or("unexpected equal base points")?;
                if value(&k,minus.0)!=0 {
                    return Err("S3 pair-support misses signed difference".into());
                }
                pair_tests+=1;
            }
        }
    }
    if pair_tests!=961 {return Err("expected 961 affine pair witnesses".into());}
    // Exact quotient dimension, not a memory allocation or attack-work count:
    let dimension=(31u128).pow(8);
    let pair_dimension=(961u128).pow(4);
    if dimension!=pair_dimension || dimension!=852_891_037_441u128 {
        return Err("quotient dimensions inconsistent".into());
    }
    Ok(json!({
      "standing":"[pcc:unverified]",
      "field_prime":P,"curve_order":PRIME31.n,"cofactor":1,
      "base_size":xs.len(),"full_dickson_x":base.values.len(),
      "rational_base_H_degree":h.len()-1,"H_squarefree":true,
      "pair_support_K_degree":k.len()-1,
      "pair_support_K_gcd_derivative_degree":squarefree_gcd_degree,
      "pair_support_quotient_is_reduced_over_Fp":squarefree_gcd_degree==0,
      "pair_S3_quadratic_factors":quadratics,"diagonal_linear_factors":linears,
      "checked_affine_signed_pair_roots":pair_tests,
      "eight_leaf_quotient_dimension":dimension.to_string(),
      "four_pair_coordinate_quotient_dimension":pair_dimension.to_string(),
      "elimination_variables_before":"48 tower + 3 sum x",
      "candidate_representation":"8 base-x leaves + 3 sum x, or 4 pair-sum x + implicit S5",
      "status":"preprocessing identity gate only: no m8 natural root, no completed IC call",
      "caveat":"U=identity and repeated/canceling intermediates need explicit exceptional handling"
    }))
}
fn main()->Result<(),String>{
    let report=run()?;
    println!("{}",serde_json::to_string_pretty(&report).map_err(|e|e.to_string())?);
    Ok(())
}
#[cfg(test)]
mod tests {
    #[test]
    fn exact_rational_quotient_bridge(){super::run().unwrap();}
}

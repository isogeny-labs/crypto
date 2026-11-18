//! Independent D9 Bézout identity replayer for PR #4604.
//! Reconstructs eight public quadrics directly, NOT via candidate's M or BM.
//! Usage: prime31_rr9_d9_unit_verify <candidate-output.json>
//! This is a Rust certificate verifier, not an ECDLP solver.
use std::collections::HashMap;
use std::fs;
use serde_json::{json, Value};

const P: u64 = 2_147_483_647;
const A: u64 = 847_514_846;
const B: u64 = 2_087_444_724;
const XS: [u64; 8] = [0, 1, 2, 3, 5, 6, 8, 9];
type E = [u8; 7];

fn add(a: u64, b: u64) -> u64 { (a+b) % P }
fn sub(a: u64, b: u64) -> u64 { (a+P-b) % P }
fn mul(a: u64, b: u64) -> u64 { (a*b) % P }
fn plus(a: &E, b: &E) -> E { std::array::from_fn(|i| a[i] + b[i]) }
fn variable(i: usize) -> E {
    let mut out = [0u8;7];
    out[i] = 1;
    out
}
fn accumulate(q: &mut HashMap<E,u64>, ex: E, scale: u64) {
    if scale == 0 { return; }
    let before = q.get(&ex).copied().unwrap_or(0);
    let after = add(before, scale);
    if after == 0 { q.remove(&ex); } else { q.insert(ex,after); }
}
fn polynomial_h() -> [u64;9] {
    let mut h = vec![1u64];
    for x in XS {
        let mut next = vec![0;h.len()+1];
        for (i, &c) in h.iter().enumerate() {
            next[i] = sub(next[i], mul(x, c));
            next[i+1] = add(next[i+1], c);
        }
        h = next;
    }
    h.try_into().expect("monic degree-eight H")
}
fn public_quadrics(r: u64,s: u64) -> [HashMap<E,u64>;8] {
    let mut eq: [HashMap<E,u64>;8] = std::array::from_fn(|_|HashMap::new());
    let d = [add(mul(r,r),A),r,1];
    for i in 0..4usize {
        for j in 0..4usize {
            let ai = variable(i);
            let aj = variable(j);
            let vi = if i == 3 {[0u8;7]} else {variable(4+i)};
            let vj = if j == 3 {[0u8;7]} else {variable(4+j)};
            for (k, &v) in d.iter().enumerate() {
                if i+j+k < 8 {
                    accumulate(&mut eq[i+j+k],plus(&vi,&vj),v);
                }
            }
            if i+j < 8 {
                accumulate(&mut eq[i+j],plus(&ai,&aj),r);
                accumulate(&mut eq[i+j],plus(&ai,&vj),sub(0,mul(2,s)));
            }
            if i+j+1 < 8 {
                accumulate(&mut eq[i+j+1],plus(&ai,&aj),P-1);
            }
        }
    }
    let h = polynomial_h();
    for i in 0..8 {
        accumulate(&mut eq[i],[0;7],sub(0,h[i]));
    }
    eq
}
// Exactly the affine-degree ordering of the frozen public Macaulay basis.
// Independently implemented here, without importing the producer code.
fn degree_rec(i:usize, rem:u8, mon:&mut E, output:&mut Vec<E>) {
    if i == 7 {
        if rem == 0 {output.push(*mon);}
        return;
    }
    for v in 0..=rem {
        mon[i]=v;
        degree_rec(i+1,rem-v,mon,output);
    }
}
fn all_monomials(deg:u8)->Vec<E> {
    let mut out=Vec::new();
    let mut mon=[0u8;7];
    for k in 0..=deg {degree_rec(0,k,&mut mon,&mut out);}
    out
}
fn verify(raw:&Value)->Result<Value,String> {
    if raw["schema"] != "prime31.rr9.d9-blackbox-negative.v1" ||
        raw["p"] != P || raw["n"] != 2_147_446_831u64 ||
        raw["a"] != A || raw["b"] != B ||
        raw["base_x"] != json!(XS) ||
        raw["degree"] != 9 || raw["columns"] != 11_440 ||
        raw["rows"] != 27_456 ||
        raw["unit_bezout_identity_replayed"] != true {
        return Err("not a frozen prime31 D9 negative unit record".into());
    }
    let res=raw["public_residual"].as_array().ok_or("missing residual")?;
    let r=res[0].as_u64().ok_or("invalid residual x")?;
    let s=res[1].as_u64().ok_or("invalid residual y")?;
    if r >= P || s >= P {return Err("off-field residual".into());}
    let rhs=add(add(mul(mul(r,r),r),mul(A,r)),B);
    if mul(s,s) != rhs {return Err("point off original prime31 curve".into());}
    if XS.contains(&r) {return Err("exceptional x(R) excluded".into());}
    let coeff=raw["unit_bezout_coefficients"].as_array().ok_or("no unit witness")?;
    let multipliers=all_monomials(7);
    let basis=all_monomials(9);
    if basis.len()!=11440 || multipliers.len()!=3432 || coeff.len()!=8*multipliers.len() {
        return Err("D9 dimensions mismatch".into());
    }
    let index:HashMap<E,usize> = basis.iter().copied().enumerate()
        .map(|(i,e)|(e,i)).collect();
    let eq=public_quadrics(r,s);
    let mut check=vec![0u64;basis.len()];
    let mut nonzero=0usize;
    for (qi,q) in eq.iter().enumerate() {
        for (mi,m) in multipliers.iter().enumerate() {
            let w=coeff[qi*multipliers.len()+mi].as_u64().ok_or("bad certificate scalar")?;
            if w>=P {return Err("certificate scalar out of range".into());}
            if w==0 {continue;}
            nonzero+=1;
            for (ex, &c) in q {
                let at=*index.get(&plus(ex,m)).ok_or("out-of-range D9 monomial")?;
                check[at]=add(check[at],mul(w,c));
            }
        }
    }
    if check[0] != 1 || check[1..].iter().any(|&v|v!=0) {
        return Err("independently rebuilt polynomial unit identity FAILED".into());
    }
    Ok(json!({"schema":"prime31.rr9.d9-independent-unit-replay.v1",
      "p":P,"n":2_147_446_831u64,"public_residual":[r,s],
      "macaulay_rows":coeff.len(),"macaulay_cols":basis.len(),
      "witness_nonzero_coefficients":nonzero,
      "independent_quadrics_bezout_unit_identity":true,
      "verified_without_bm_or_sign_masks":true,
      "complete_unknown_ecdlp_solves":0,"pcc":"unverified"}))
}
fn main()->Result<(),String>{
    let path=std::env::args().nth(1).ok_or("usage: <result.json>")?;
    let doc:Value=serde_json::from_str(&fs::read_to_string(path).map_err(|e|e.to_string())?)
        .map_err(|e|e.to_string())?;
    let result=verify(&doc)?;
    println!("{}",serde_json::to_string_pretty(&result).unwrap());
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_polynomial_and_monomial_replay_semantics(){
        assert_eq!(polynomial_h()[8],1);
        assert_eq!(all_monomials(7).len(),3432);
        assert_eq!(all_monomials(9).len(),11440);
        let r=625037715;let s=2100627251;
        let q=public_quadrics(r,s);
        assert_eq!(q.len(),8);
        let mut sum=0usize;
        for pol in &q {sum+=pol.len();}
        assert!(sum>20);
    }
}

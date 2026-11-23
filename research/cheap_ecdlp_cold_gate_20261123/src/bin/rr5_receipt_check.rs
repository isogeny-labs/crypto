//! Independent, read-only Prime31 receipt gate for RR5-style mixed public RHS.
//! Does not invoke the producer extractor, reuse its row-echelon solver, or
//! accept a target discrete logarithm as input. NO speed or PCC claim.
//! This tool is an isolated checker over the original project's curve arithmetic.
use prime31_algebraic_ic::{check_curve, Arithmetic, Counts, Point, PRIME31};
use serde::{Deserialize, Serialize};
use std::{env, fs, process};

const N: u64 = 2_147_446_831;

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct MixedRow {
    // Exact signed leaves; repeated indices and their signs are significant.
    terms: Vec<[i64; 2]>,
    // Original public query RHS: sum signed P_i = [a]G + [b]Q.
    a: u64,
    b: u64,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Receipt {
    base: Vec<[u64; 2]>,
    target_q: Option<[u64; 2]>,
    relations: Vec<MixedRow>,
    #[serde(default)]
    stop_when_target_determined: bool,
}
#[derive(Serialize, Debug)]
struct Report {
    p: u64,
    n: u64,
    base_size: usize,
    submitted_rows: usize,
    authenticated_rows: usize,
    unchecked_tail_rows: usize,
    independent_augmented_rank: usize,
    dependent_rows: usize,
    // Partial linear-algebra diagnostics, NOT a full cost model:
    gauss_jordan_row_mul_sub_pairs: u64,
    gauss_jordan_inversions: u64,
    first_determined_relation_prefix: Option<usize>,
    recovered: Option<u64>,
    scalar_replay_passed: bool,
    stop_when_target_determined: bool,
    arithmetic_diagnostic: Counts,
    scope: &'static str,
}
fn add(a: u64, b: u64) -> u64 { (a+b)%N }
fn sub(a: u64, b: u64) -> u64 { (a+N-b)%N }
fn neg(a: u64) -> u64 { if a==0 {0} else {N-a} }
fn mul(a: u64, b: u64) -> u64 { ((a as u128*b as u128)%(N as u128)) as u64 }
fn pow(mut x: u64, mut n: u64) -> u64 {
    let mut ans=1;
    while n>0 {
        if n&1==1 {ans=mul(ans,x);}
        x=mul(x,x);n>>=1;
    }
    ans
}
fn point(q: Option<[u64;2]>) -> Point {q.map(|[x,y]|(x,y))}

// A Gauss-Jordan reduction independent of the RR5 producer's RowSpace.
// Pivot columns are ordered left-to-right; the last unknown is log_G(Q).
// Thus a pivot in the final column certifies the target's uniqueness, even
// when some earlier factor-base logarithms remain free.
struct GaussJordan {
    pivots: Vec<Option<Vec<u64>>>,
    rank: usize,
    row_mul_sub_pairs: u64,
    inversions: u64,
}
impl GaussJordan {
    fn new(variables: usize) -> Self {
        Self {pivots:vec![None;variables],rank:0,row_mul_sub_pairs:0,inversions:0}
    }
    fn insert(&mut self, mut row: Vec<u64>) -> Result<bool,String> {
        let vars=self.pivots.len();
        if row.len()!=vars+1 || row.iter().any(|&x| x>=N) {
            return Err("invalid augmented row".into());
        }
        for col in 0..vars {
            if row[col]==0 {continue;}
            if let Some(existing)=&self.pivots[col] {
                let factor=row[col];
                for j in col..=vars {
                    row[j]=sub(row[j],mul(factor,existing[j]));
                    self.row_mul_sub_pairs+=1;
                }
                continue;
            }
            let inverse=pow(row[col],N-2);
            self.inversions+=1;
            for item in &mut row[col..] {*item=mul(*item,inverse);}
            // Maintain *reduced* echelon form. Unlike the producer, eliminate
            // the new pivot out of all previously stored rows as well.
            for old in self.pivots.iter_mut().flatten() {
                let factor=old[col];
                if factor!=0 {
                    for j in col..=vars {
                        old[j]=sub(old[j],mul(factor,row[j]));
                        self.row_mul_sub_pairs+=1;
                    }
                }
            }
            self.pivots[col]=Some(row);
            self.rank+=1;
            return Ok(true);
        }
        if row[vars]!=0 {Err("inconsistent authenticated augmented equations".into())}
        else {Ok(false)}
    }
    fn unique_target(&self)->Option<u64>{
        // Column B is the target unknown. Earlier free columns cannot enter
        // a pivot row at B: if pivot B exists, that row is d=constant.
        self.pivots.last().and_then(|row|row.as_ref()).map(|r|r[self.pivots.len()])
    }
}
fn signed_sum(ar:&mut Arithmetic,base:&[Point],terms:&[[i64;2]])->Result<(Point,Vec<u64>),String>{
    if terms.len()>4 {return Err("more than four signed entries".into());}
    // Reject oversized signed multiplicities BEFORE summing their absolute
    // values; arbitrary i64 JSON must never overflow a development checker.
    if terms.iter().any(|x|x[1]==0 || !(-4..=4).contains(&x[1])) {
        return Err("invalid signed multiplicity".into());
    }
    let leaves: u64=terms.iter().map(|x|x[1].unsigned_abs()).sum();
    if leaves>4 {return Err("more than four signed leaves".into());}
    let mut lhs=None;
    let mut coefficients=vec![0;base.len()];
    for &[index,c] in terms {
        if index<0 || (index as usize)>=base.len() || c==0 || !(-4..=4).contains(&c) {
            return Err(format!("invalid signed term ({index},{c})"));
        }
        let k=index as usize;
        coefficients[k]=add(coefficients[k],c.rem_euclid(N as i64) as u64);
        let signed=if c<0 {ar.neg(PRIME31,base[k])} else {base[k]};
        for _ in 0..c.unsigned_abs(){lhs=ar.add(PRIME31,lhs,signed);}
    }
    Ok((lhs,coefficients))
}
fn verify(data:&Receipt)->Result<Report,String>{
    if data.base.is_empty() || data.base.len()>4096 ||
       data.relations.len()>65_536 {return Err("receipt exceeds fixed caps".into());}
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar)?;
    if let Some([x,y])=data.target_q {
        if x>=PRIME31.p || y>=PRIME31.p {
            return Err("public Q coordinates are not canonical field elements".into());
        }
    }
    let q=point(data.target_q);
    if !ar.on_curve(PRIME31,q) {return Err("public Q is off curve".into());}
    let mut base=Vec::with_capacity(data.base.len());
    for (i,&xy) in data.base.iter().enumerate(){
        if xy[0]>=PRIME31.p || xy[1]>=PRIME31.p {
            return Err(format!("base[{i}] coordinates are noncanonical"));
        }
        if !ar.on_curve(PRIME31,Some((xy[0],xy[1]))) {
            return Err(format!("base[{i}] is off curve"));
        }
        if data.base[..i].contains(&xy) {return Err(format!("duplicate base[{i}]"));}
        base.push(point(Some(xy)));
    }
    let variables=base.len()+1;
    let mut matrix=GaussJordan::new(variables);
    let mut checked=0;
    let mut dependent=0;
    let mut first=if q.is_none(){Some(0)} else {None};
    let mut recovered=if q.is_none(){Some(0)} else {None};
    for (i,rel) in data.relations.iter().enumerate(){
        if data.stop_when_target_determined && recovered.is_some() {break;}
        if rel.a>=N || rel.b>=N {
            return Err(format!("relation[{i}] a/b noncanonical"));
        }
        let (lhs,mut coeffs)=signed_sum(&mut ar,&base,&rel.terms)?;
        let ag=ar.scalar(PRIME31,rel.a,Some(PRIME31.g));
        let bq=ar.scalar(PRIME31,rel.b,q);
        if lhs!=ar.add(PRIME31,ag,bq){
            return Err(format!("relation[{i}] failed original-arithmetic public RHS replay"));
        }
        // coeff(log P) - b * log(Q) = a.
        coeffs.push(neg(rel.b));
        coeffs.push(rel.a);
        let independent=matrix.insert(coeffs)?;
        checked+=1;
        if !independent {dependent+=1;}
        if recovered.is_none(){
            if let Some(d)=matrix.unique_target(){
                if ar.scalar(PRIME31,d,Some(PRIME31.g))!=q{
                    return Err(format!("relation[{i}] target scalar replay failed"));
                }
                first=Some(i+1);
                recovered=Some(d);
            }
        }
    }
    Ok(Report{
        p:PRIME31.p,n:N,base_size:base.len(),
        submitted_rows:data.relations.len(),authenticated_rows:checked,
        unchecked_tail_rows:data.relations.len()-checked,
        independent_augmented_rank:matrix.rank,
        dependent_rows:dependent,
        gauss_jordan_row_mul_sub_pairs:matrix.row_mul_sub_pairs,
        gauss_jordan_inversions:matrix.inversions,
        first_determined_relation_prefix:first,
        recovered,scalar_replay_passed:recovered.is_some(),
        stop_when_target_determined:data.stop_when_target_determined,
        arithmetic_diagnostic:ar.counts,
        scope:"read-only public mixed-RHS authentication and unique target replay; no extraction, full-cost benchmark or PCC",
    })
}
fn main(){
    let args:Vec<String>=env::args().collect();
    if args.len()!=2 {eprintln!("usage: rr5_receipt_check <public-rr5-receipt.json>");process::exit(2);}
    let result=fs::read_to_string(&args[1]).map_err(|e|e.to_string())
        .and_then(|s|serde_json::from_str::<Receipt>(&s).map_err(|e|e.to_string()))
        .and_then(|x|verify(&x));
    match result {
        Ok(r)=>println!("{}",serde_json::to_string_pretty(&r).expect("JSON report")),
        Err(e)=>{eprintln!("FAIL: {e}");process::exit(1)}
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn fixture()->Receipt {
        let mut ar=Arithmetic::default();
        let g=Some(PRIME31.g);
        let base=(1..=4).map(|k|{
            let (x,y)=ar.scalar(PRIME31,k,g).unwrap();
            [x,y]
        }).collect();
        let target_q=ar.scalar(PRIME31,5,g).map(|(x,y)|[x,y]);
        Receipt {base,target_q,stop_when_target_determined:false,relations:vec![
            MixedRow{terms:vec![[0,1]],a:1,b:0},
            MixedRow{terms:vec![[0,1]],a:N-4,b:1},
        ]}
    }
    #[test]
    fn mixed_rhs_recovers_q_before_base_rank() {
        let out=verify(&fixture()).unwrap();
        assert_eq!(out.independent_augmented_rank,2);
        assert_eq!(out.base_size,4);
        assert_eq!(out.recovered,Some(5));
        assert_eq!(out.first_determined_relation_prefix,Some(2));
        assert!(out.scalar_replay_passed);
    }
    #[test]
    fn one_mixed_row_is_underdetermined() {
        let mut f=fixture();f.relations.remove(0);
        let out=verify(&f).unwrap();
        assert_eq!(out.recovered,None);
        assert_eq!(out.first_determined_relation_prefix,None);
        assert!(!out.scalar_replay_passed);
    }
    #[test]
    fn unrelated_early_row_delays_visibility_not_scalar() {
        let mut f=fixture();
        let original=verify(&f).unwrap();
        f.relations.insert(0,MixedRow{terms:vec![[1,1]],a:2,b:0});
        let delayed=verify(&f).unwrap();
        assert_eq!(original.recovered,delayed.recovered);
        assert_eq!(original.first_determined_relation_prefix,Some(2));
        assert_eq!(delayed.first_determined_relation_prefix,Some(3));
    }
    #[test]
    fn signed_duplicate_keeps_rank_and_replay() {
        let mut f=fixture();
        f.relations.push(MixedRow{terms:vec![[0,-1]],a:N-1,b:0});
        let out=verify(&f).unwrap();
        assert_eq!(out.dependent_rows,1);
        assert_eq!(out.recovered,Some(5));
    }
    #[test]
    fn invalid_mixed_b_is_rejected() {
        let mut f=fixture();
        f.relations[1].b=2;
        assert!(verify(&f).is_err());
    }
    #[test]
    fn malformed_point_is_rejected() {
        let mut f=fixture();
        f.target_q=Some([PRIME31.p,1]);
        assert!(verify(&f).is_err());
    }
    #[test]
    fn stop_before_poisoned_tail_never_authenticates_it() {
        let mut f=fixture();
        f.relations.push(MixedRow{terms:vec![[0,1]],a:2,b:0});
        f.stop_when_target_determined=true;
        let result=verify(&f).unwrap();
        assert_eq!((result.authenticated_rows,result.unchecked_tail_rows),(2,1));
        assert_eq!(result.recovered,Some(5));
        f.stop_when_target_determined=false;
        assert!(verify(&f).is_err());
    }
    #[test]
    fn zero_target_is_visible_at_prefix_zero() {
        let mut f=fixture();
        f.target_q=None;
        f.relations.clear();
        let out=verify(&f).unwrap();
        assert_eq!(out.recovered,Some(0));
        assert_eq!(out.first_determined_relation_prefix,Some(0));
    }
    #[test]
    fn reject_i64_min_without_overflow_or_panic() {
        let mut f=fixture();
        f.relations[0].terms=vec![[0,i64::MIN],[1,i64::MIN]];
        assert!(verify(&f).is_err());
    }
    #[test]
    fn reject_noncanonical_factor_base_coordinates() {
        let mut f=fixture();
        f.base[0][0]=PRIME31.p;
        assert!(verify(&f).is_err());
    }
    #[test]
    fn reject_more_than_four_signed_leaves() {
        let mut f=fixture();
        f.relations[0].terms=vec![[0,4],[1,1]];
        assert!(verify(&f).is_err());
    }
}

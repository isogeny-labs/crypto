//! Independent-from-the-solver affine arithmetic and whole-cold JSON gate audit.
//! This is producer-side audit, not signed external PCC certification.
use serde_json::{json,Value};
use std::{collections::HashSet,env,fs};

const P:u64=2_147_483_647;
const N:u64=2_147_446_831;
const A:u64=847_514_846;
const B:u64=2_087_444_724;
const G:(u64,u64)=(625_037_715,2_100_627_251);
type Pt=Option<(u64,u64)>;

fn bad(s:&str)->String{s.to_string()}
fn sub(a:u64,b:u64,m:u64)->u64{if a>=b{a-b}else{m-(b-a)}}
fn mul(a:u64,b:u64,m:u64)->u64{((a as u128*b as u128)%m as u128) as u64}
fn pow(mut b:u64,mut e:u64,m:u64)->u64{
    let mut r=1u64;while e>0 {if e&1!=0{r=mul(r,b,m);} b=mul(b,b,m);e>>=1;}r
}
fn on_curve((x,y):(u64,u64))->bool{
    x<P&&y<P&&mul(y,y,P)==((mul(mul(x,x,P),x,P)+mul(A,x,P))%P+B)%P
}
fn add(p:Pt,q:Pt)->Pt{
    // The group identity must be a *two-sided* identity; Option '?' would
    // incorrectly map O+P and P+O to O, corrupting independent replay.
    let ((x,y),(u,v))=match (p,q){
        (None,r)=>return r,
        (r,None)=>return r,
        (Some(p),Some(q))=>(p,q)
    };
    if x==u&&y!=v{return None}
    let slope=if x==u {
        if y==0{return None}
        let num=(3*mul(x,x,P)+A)%P;
        mul(num,pow((2*y)%P,P-2,P),P)
    }else{
        mul(sub(v,y,P),pow(sub(u,x,P),P-2,P),P)
    };
    let xx=sub(sub(mul(slope,slope,P),x,P),u,P);
    let yy=sub(mul(slope,sub(x,xx,P),P),y,P);
    Some((xx,yy))
}
fn scalar(mut n:u64,p:Pt)->Pt{
    let mut r=None;let mut b=p;
    while n>0{if n&1!=0{r=add(r,b);}b=add(b,b);n>>=1;}
    r
}
fn num(v:&Value)->Result<u64,String>{v.as_u64().ok_or_else(||bad("expected u64"))}
fn vals<'a>(v:&'a Value)->Result<&'a Vec<Value>,String>{
    v.as_array().ok_or_else(||bad("expected array"))
}
fn cold(v:&Value)->Result<u64,String>{num(&v["cold_wall_ns"])}
fn source_attempts(v:&Value)->Result<&Vec<Value>,String>{vals(&v["attempts"])}
fn read_log(run:&Value,aps:bool)->Result<u64,String>{
    if aps{num(&source_attempts(run)?.last().ok_or("missing attempt")?["solution"]["log"])}
    else{num(&run["log"])}
}
fn main()->Result<(),String>{
    let path=env::args().nth(1).ok_or("pass raw JSON path")?;
    let doc:Value=serde_json::from_str(&fs::read_to_string(path)
        .map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
    if num(&doc["p"])?!=P||num(&doc["n"])?!=N||
       num(&doc["a"])?!=A||num(&doc["b"])?!=B||
       num(&doc["h"])?!=1||num(&doc["cases"])?!=24 {
        return Err(bad("unexpected source curve or fixture count"));
    }
    let entries=vals(&doc["cases_detail"])?;
    if entries.len()!=24{return Err(bad("case list length"))}
    let mut seen=HashSet::new();
    let mut totals=[0u128;5];
    let mut correct=[0usize;5];
    let mut wins=0usize;
    let mut pair_checks=0u64;
    for (case,row) in entries.iter().enumerate(){
        if num(&row["case"])?!=case as u64{return Err(bad("case index mismatch"))}
        let qv=vals(&row["public_Q"])?;
        if qv.len()!=2{return Err(bad("bad Q"))}
        let q=(num(&qv[0])?,num(&qv[1])?);
        let k=num(&row["caller_known_answer_only"])?;
        if k<=1||k>=N||!seen.insert(k)||!on_curve(q)||scalar(k,Some(G))!=Some(q) {
            return Err(format!("incorrect or duplicate affine target case={case}"));
        }
        let arms=[
            &row["old_publicy"],&row["new_linear_chord"],&row["linear_chord_AA"],
            &row["unfolded_rho32"],&row["unfolded_rho64"]
        ];
        let order=vals(&row["chronological_order"])?;
        if order.len()!=5{return Err(bad("arm order length"))}
        let mut order_set=HashSet::new();
        for x in order{order_set.insert(num(x)?);}
        if order_set!=HashSet::from([0u64,1,2,3,4]){
            return Err(bad("missing/duplicate rotating-order arm"));
        }
        for (i,run) in arms.iter().enumerate(){
            if run["complete_target_scalar_recovered"].as_bool()!=Some(true){
                return Err(format!("incomplete solve case={case} arm={i}"));
            }
            let recovered=read_log(run,i<3)?;
            if recovered!=k||scalar(recovered,Some(G))!=Some(q){
                return Err(format!("invalid recovered scalar case={case} arm={i}"));
            }
            totals[i]+=u128::from(cold(run)?);correct[i]+=1;
            if i<3{
                if run["public_Q"]!=row["public_Q"]{
                    return Err(format!("wrong APS public Q case={case} arm={i}"));
                }
                let steps=source_attempts(run)?;
                if steps.is_empty()||steps.len()>6{return Err(bad("APS rounds cap"))}
                for step in steps{
                    let sol=&step["solution"];
                    if sol["status"]=="verified"{
                        let alpha=num(&sol["alpha"])?;
                        let beta=num(&sol["beta"])?;
                        let signed=(alpha as u128 + beta as u128*k as u128)%N as u128;
                        if signed!=0 || beta==0{
                            return Err(format!("incorrect signed S3 certificate case={case} arm={i}"));
                        }
                    }
                }
            }
        }
        let old=source_attempts(arms[0])?;
        for i in [1usize,2] {
            let other=source_attempts(arms[i])?;
            if old.len()!=other.len(){
                return Err(format!("different attempt count case={case}"));
            }
            for (a,b) in old.iter().zip(other.iter()){
                for key in ["round","seed","profile","rejected_identity",
                            "rejected_duplicate_x","s3_pair_quadratics",
                            "s3_batch_inversions","signed_candidates","solution"] {
                    if a[key]!=b[key]{
                        return Err(format!("different signed transcript case={case} arm={i} key={key}"));
                    }
                }
                pair_checks+=1;
            }
        }
        if cold(arms[1])?<cold(arms[0])? {wins+=1;}
    }
    let provided=vals(&doc["cold_total_ns"])?;
    for i in 0..5{
        if provided[i].as_str()!=Some(totals[i].to_string().as_str()){
            return Err(format!("incorrect aggregate total arm={i}"));
        }
    }
    let ratio=totals[1] as f64 / totals[0] as f64;
    let drift=(totals[1] as f64 / totals[2] as f64-1.0).abs();
    let passes=ratio<=0.95&&drift<=0.05&&wins>=18&&correct==[24usize;5];
    if doc["passes_frozen_5pct_gate"].as_bool()!=Some(passes) ||
       num(&doc["candidate_paired_wins"])?!=wins as u64{
        return Err(bad("producer speed-gate or paired-win mismatch"));
    }
    println!("{}",serde_json::to_string_pretty(&json!({
        "standing":"[pcc:unverified]",
        "auditor":"separate Rust affine group law; raw JSON aggregate and equality recompute",
        "targets":entries.len(),"verified_complete_calls":correct.iter().sum::<usize>(),
        "affine_scalar_replays":correct.iter().sum::<usize>(),
        "paired_S3_attempt_checks":pair_checks,
        "cold_total_ns":totals.map(|v|v.to_string()),
        "candidate_over_previous_ratio":ratio,
        "candidate_repeat_drift":drift,
        "candidate_paired_wins":wins,
        "five_percent_gate":passes,
        "rho_is_not_strongest_WZ":true,
        "independent_signed_PCC":false
    })).map_err(|e|e.to_string())?);
    Ok(())
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]fn affine_rules(){
        assert!(on_curve(G));
        assert_eq!(scalar(1,Some(G)),Some(G));
        assert_eq!(scalar(N,Some(G)),None);
        assert_eq!(add(Some(G),Some((G.0,sub(0,G.1,P)))),None);
        let two=scalar(2,Some(G)).unwrap();
        assert!(on_curve(two));assert_eq!(add(Some(G),Some(G)),Some(two));
    }
}

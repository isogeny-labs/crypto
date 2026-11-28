//! EXP-R31-SYM8: complete finite-rational symmetric eight-leaf membership
//! and signed Semaev-root semantic equivalence gate on the fixed ordinary
//! prime31 curve. NOT target-conditioned S9 elimination or a DLP solver.
//!
//! Algebraic side uses Fp polynomial division and S3 roots ONLY. The group
//! side independently checks all 2^8 y-sign choices AFTER each algebraic
//! decision and is never used to generate or correct algebraic roots.
//! This is a small-B correctness reference for a future nonenumerative
//! target-conditioned symmetric quotient, not a claimed speed benchmark.
//!
//! cargo +1.100.0 test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_m8_symmetric_root_gate
//! cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_m8_symmetric_root_gate

use prime31_algebraic_ic::{check_curve, Arithmetic, Base, Point, PRIME31};
use serde_json::json;
use std::collections::BTreeSet;
use std::time::Instant;

const P:u64=2_147_483_647;
const A:u64=847_514_846;
const B:u64=2_087_444_724;
const SEED:u64=0x5359_4d38_4230_3136;
type Poly=Vec<u64>;
fn add(x:u64,y:u64)->u64{let z=x+y;if z>=P{z-P}else{z}}
fn sub(x:u64,y:u64)->u64{if x>=y{x-y}else{x+P-y}}
fn mul(x:u64,y:u64)->u64{((x as u128*y as u128)%P as u128) as u64}
fn pow(mut x:u64,mut exp:u64)->u64{
    let mut acc=1;
    while exp!=0{
        if exp&1!=0{acc=mul(acc,x);}
        x=mul(x,x);exp>>=1;
    }
    acc
}
fn poly_mul(a:&[u64],b:&[u64])->Poly{
    if a.is_empty()||b.is_empty(){return Vec::new();}
    let mut z=vec![0;a.len()+b.len()-1];
    for (i,&x) in a.iter().enumerate(){
        for (j,&y) in b.iter().enumerate(){z[i+j]=add(z[i+j],mul(x,y));}
    }
    z
}
fn monic_roots(xs:&[u64])->Poly{
    xs.iter().fold(vec![1],|acc,&x|poly_mul(&acc,&[sub(0,x),1]))
}
fn is_zero_remainder(mut a:Poly,monic:&[u64])->bool{
    if monic.is_empty()||*monic.last().unwrap_or(&0)!=1{return false;}
    while a.len()>=monic.len(){
        let lead=*a.last().unwrap_or(&0);
        let off=a.len()-monic.len();
        for (i,&c) in monic.iter().enumerate(){
            a[off+i]=sub(a[off+i],mul(lead,c));
        }
        while a.last()==Some(&0){a.pop();}
    }
    a.is_empty()
}
fn divide_linear(f:&[u64],x:u64)->Option<Poly>{
    if f.len()<2{return None;}
    let n=f.len()-1;
    let mut q=vec![0;n];
    q[n-1]=f[n];
    for j in (1..n).rev(){q[j-1]=add(f[j],mul(x,q[j]));}
    (add(f[0],mul(x,q[0]))==0).then_some(q)
}
// A candidate f comes only from unordered multiset polynomial coefficients,
// never from secret indices. Certify f|H^8 and extract all finite base roots.
fn certified_finite_leaves(f:&[u64],xs:&[u64],h8:&[u64])
->Result<Vec<u64>,String>{
    if f.len()!=9||f[8]!=1{return Err("expected monic degree-eight f".into());}
    if !is_zero_remainder(h8.to_vec(),f){
        return Err("candidate f does not divide H^8".into());
    }
    let mut residual=f.to_vec();
    let mut recovered=Vec::with_capacity(8);
    for &x in xs{
        while residual.len()>1{
            let Some(next)=divide_linear(&residual,x) else{break;};
            recovered.push(x);residual=next;
        }
    }
    if residual!=[1]||recovered.len()!=8{
        return Err("unrecovered/nonrational base root".into());
    }
    Ok(recovered)
}
// S3(x,y,z) as a quadratic in z, including the degree-one diagonal.
fn s3_successors(x:u64,y:u64)->Result<(Vec<Option<u64>>,u64),String>{
    let sum=add(x,y);
    let xy=mul(x,y);
    let a=mul(sub(x,y),sub(x,y));
    let b=sub(0,mul(2,add(mul(sum,add(xy,A)),mul(2,B))));
    let c=sub(mul(sub(xy,A),sub(xy,A)),mul(mul(4,B),sum));
    if a==0 {
        if b==0 {return Err("unsupported zero S3 diagonal".into());}
        let root=mul(sub(0,c),pow(b,P-2));
        if add(mul(b,root),c)!=0 {return Err("linear S3 root fails".into());}
        // x(P)+x(±P) yields 2P or the point at infinity.
        return Ok((vec![None,Some(root)],1));
    }
    let disc=sub(mul(b,b),mul(mul(4,a),c));
    let sq=pow(disc,(P+1)/4);
    if mul(sq,sq)!=disc{return Err("S3 discriminant nonsquare".into());}
    let inv=pow(mul(2,a),P-2);
    let r1=mul(sub(sq,b),inv);
    let r2=mul(sub(sub(0,sq),b),inv);
    let mut ans=vec![Some(r1),Some(r2)];
    ans.sort_unstable();ans.dedup();
    for &r in &ans {
        let z=r.ok_or("unexpected identity in quadratic")?;
        if add(add(mul(a,mul(z,z)),mul(b,z)),c)!=0{
            return Err("quadratic S3 root fails".into());
        }
    }
    Ok((ans,0))
}
// Field-only evaluation of existential S9 semantics by recursively
// eliminating one signed abscissa at a time. This is intentionally an
// enumerative evaluation of a GIVEN f, not a discovery algorithm for f.
fn signed_s3_fibre(xs:&[u64])->Result<(BTreeSet<Option<u64>>,u64,u64),String>{
    let mut states=BTreeSet::from([None]);
    let mut pair_calls=0;
    let mut exceptional=0;
    for &x in xs{
        let mut next=BTreeSet::new();
        for s in states{
            if let Some(u)=s{
                let (roots,e)=s3_successors(u,x)?;
                pair_calls+=1;
                exceptional+=e;
                next.extend(roots);
            }else{next.insert(Some(x));}
        }
        states=next;
    }
    Ok((states,pair_calls,exceptional))
}
fn signed_full_group_states(xs:&[u64],base:&Base,ar:&mut Arithmetic)
->Result<BTreeSet<Point>,String>{
    let mut states=BTreeSet::from([None]);
    for &x in xs{
        let p=base.points.iter().find(|pt|pt.0==x)
            .copied().ok_or("base x lacks canonical point")?;
        let minus=ar.neg(PRIME31,Some(p));
        let mut next=BTreeSet::new();
        for state in states{
            next.insert(ar.add(PRIME31,state,Some(p)));
            next.insert(ar.add(PRIME31,state,minus));
        }
        states=next;
    }
    Ok(states)
}
fn choose_multisets(
    width:usize,first:usize,b:usize,partial:&mut Vec<usize>,
    out:&mut Vec<[usize;8]>
){
    if partial.len()==width{
        let arr:[usize;8]=partial.clone().try_into().expect("exact length eight");
        out.push(arr);return;
    }
    for i in first..b{
        partial.push(i);
        choose_multisets(width,i,b,partial,out);
        partial.pop();
    }
}
fn next_u64(s:&mut u64)->u64{
    *s=s.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut v=*s;
    v=(v^(v>>30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    v=(v^(v>>27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    v^(v>>31)
}
fn sample_b8()->Vec<[usize;8]>{
    let mut rng=SEED;
    let mut selected=BTreeSet::from([[0usize;8],[7usize;8]]);
    while selected.len()<128{
        let mut row=[0usize;8];
        for i in &mut row{*i=(next_u64(&mut rng)%8) as usize;}
        row.sort_unstable();selected.insert(row);
    }
    selected.into_iter().collect()
}
fn run(exhaustive_b8:bool)->Result<serde_json::Value,String>{
    let start=Instant::now();
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar)?;
    let base=Base::build(PRIME31,6,&mut ar)?;
    if base.points.len()!=31 {return Err("unexpected original prime31 base".into());}
    let public=ar.scalar(PRIME31,17,Some(PRIME31.g))
        .ok_or("fixed public scalar is infinity")?;
    let mut totals=Vec::new();
    let mut all_cases=0u64;
    for b in [4usize,8]{
        if exhaustive_b8 && b==4 {continue;}
        let xs:Vec<u64>=base.points[..b].iter().map(|p|p.0).collect();
        if xs.iter().copied().collect::<BTreeSet<_>>().len()!=b{
            return Err("duplicate base x".into());
        }
        let h=monic_roots(&xs);
        let mut h8=vec![1];
        for _ in 0..8{h8=poly_mul(&h8,&h);}
        let mut multisets=Vec::new();
        if b==4 {
            choose_multisets(8,0,b,&mut Vec::new(),&mut multisets);
            if multisets.len()!=165{return Err("wrong B4 multiset count".into());}
        }else if exhaustive_b8{
            choose_multisets(8,0,b,&mut Vec::new(),&mut multisets);
            if multisets.len()!=6_435{return Err("wrong exhaustive B8 count".into());}
        }else{
            multisets=sample_b8();
            if multisets.len()!=128{return Err("wrong B8 sample count".into());}
        }
        let mut positive=0u64;
        let mut zero_target_hits=0u64;
        let mut identity_branches=0u64;
        let mut s3_calls=0u64;
        let mut field_ns=0u128;
        let mut group_ns=0u128;
        for (case,indices) in multisets.iter().enumerate(){
            let original:Vec<u64>=indices.iter().map(|&i|xs[i]).collect();
            let f=monic_roots(&original);
            let t=Instant::now();
            let recovered=certified_finite_leaves(&f,&xs,&h8)?;
            if recovered!=original{return Err("canonical roots disagree".into());}
            let (states,calls,exceptional)=signed_s3_fibre(&recovered)?;
            field_ns+=t.elapsed().as_nanos();
            s3_calls+=calls;identity_branches+=exceptional;
            let g=Instant::now();
            let ground=signed_full_group_states(&original,&base,&mut ar)?;
            let mut all_plus=None;
            for &i in indices{
                all_plus=ar.add(PRIME31,all_plus,Some(base.points[i]));
            }
            let targets=[all_plus,Some(public),Some(base.points[0]),None];
            for (kind,target) in targets.into_iter().enumerate(){
                let actual=ground.contains(&target);
                let algebraic=states.contains(&target.map(|pt|pt.0));
                if actual!=algebraic{
                    return Err(format!("B{b} case={case} kind={kind} signed x mismatch: algebraic={algebraic}, fullpoint={actual}"));
                }
                if algebraic{positive+=1;}
                if kind==3 && algebraic{zero_target_hits+=1;}
                all_cases+=1;
            }
            group_ns+=g.elapsed().as_nanos();
        }
        // Malformed polynomial and unknown root must be rejected before
        // Semaev evaluation; no point-operation control can fix these.
        let mut forged=vec![xs[0];8];
        let nonbase=(0..P).find(|z|!xs.contains(z)).ok_or("no out-of-base x")?;
        forged[7]=nonbase;
        let bad=monic_roots(&forged);
        if certified_finite_leaves(&bad,&xs,&h8).is_ok(){
            return Err("nonbase root admitted".into());
        }
        let mut not_monic=monic_roots(&vec![xs[0];8]);
        not_monic[8]=2;
        if certified_finite_leaves(&not_monic,&xs,&h8).is_ok(){
            return Err("nonmonic polynomial admitted".into());
        }
        if certified_finite_leaves(&[1;8],&xs,&h8).is_ok(){
            return Err("wrong-degree polynomial admitted".into());
        }
        if zero_target_hits==0 {
            return Err(format!("B{b} failed to exercise identity branch"));
        }
        totals.push(json!({
            "base_size":b,
            "multisets":multisets.len(),
            "target_tests":4*multisets.len(),
            "field_s3_calls":s3_calls,
            "diagonal_infinity_branches":identity_branches,
            "true_positive_target_checks":positive,
            "identity_positive_multisets":zero_target_hits,
            "false_positive":0,"false_negative":0,
            "field_only_candidate_evaluation_ns":field_ns.to_string(),
            "independent_signed_group_verification_ns":group_ns.to_string(),
            "tamper_tests_rejected":3,
            "candidate_selection":if b==4 {"all 165 unordered multisets"}
                else if exhaustive_b8 {"all 6435 unordered B8 multisets"}
                else {"128 seeded distinct unordered multisets"},
            "scope":"given-f only; no S9 target-conditioned root search"
        }));
    }
    Ok(json!({
        "standing":"[pcc:unverified]",
        "curve_field":P,"group_order":PRIME31.n,"cofactor":1,
        "study":"finite-rational S9 signed-semantics correctness gate; not a DLP algorithm",
        "matched_target_tests":all_cases,
        "exhaustive_b8":exhaustive_b8,
        "panels":totals,
        "B31_unordered_x_multisets":"48903492",
        "full_cold_validation_ns":start.elapsed().as_nanos().to_string(),
        "no_target_log_or_group_point_sum_in_algebraic_side":true,
        "limitations":"enumerative candidate-f validator, not target-conditioned S9 solving; cannot claim non-generic or full ECDLP improvement"
    }))
}
fn main()->Result<(),String>{
    let args:Vec<_>=std::env::args().skip(1).collect();
    let exhaustive=match args.as_slice(){
        []=>false,
        [flag] if flag=="--exhaustive-b8"=>true,
        _=>return Err("usage: prime31_m8_symmetric_root_gate [--exhaustive-b8]".into())
    };
    let result=run(exhaustive)?;
    let expected=if exhaustive{25_740}else{1_172};
    if result["matched_target_tests"].as_u64()!=Some(expected){
        return Err("incomplete registered symmetric m8 panel".into());
    }
    println!("{}",serde_json::to_string_pretty(&result).map_err(|e|e.to_string())?);
    Ok(())
}
#[cfg(test)]
mod tests {
    #[test]
    fn symmetric_rational_root_equivalence(){
        let result=super::run(false).unwrap();
        assert_eq!(result["matched_target_tests"],1172);
    }
}

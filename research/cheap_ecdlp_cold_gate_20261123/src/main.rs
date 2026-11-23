//! Public-witness admission and target-span gate over the project's original
//! ordinary Prime31 curve. This is NOT a relation finder or a timed solver.
//! Uses the existing native EC arithmetic; no hidden scalar is used by rank.
use prime31_algebraic_ic::{check_curve, Arithmetic, Counts, Point, PRIME31};
use serde::{Deserialize, Serialize};
use std::{env, fs, process};

const N: u64 = 2_147_446_831;

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Relation { terms: Vec<[i64; 2]>, rhs: u64 }
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Target {
    q: Option<[u64; 2]>, offset: u64, terms: Vec<[i64; 2]>,
    // Optional AFTER-recovery known answer; never used to solve a row.
    claimed_log: Option<u64>,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Certificate {
    base: Vec<[u64; 2]>, relations: Vec<Relation>, targets: Vec<Target>,
}
#[derive(Debug, Serialize)]
struct TargetResult {
    index: usize,
    determined: bool,
    recovered: Option<u64>,
    // Earliest input relation prefix spanning this target's coefficients.
    // It is a row-stream property, not a claim about extractor cost or timing.
    first_determined_relation_prefix: Option<usize>,
    group_replay_passed: bool,
    scalar_replay_passed: bool,
}
#[derive(Debug, Serialize)]
struct Report {
    p: u64, n: u64, base_size: usize, relation_count: usize,
    relation_rank: usize, dependent_relations: usize,
    target_count: usize, determined_targets: usize,
    targets: Vec<TargetResult>, arithmetic_diagnostic: Counts,
    scope: &'static str,
}
fn add(a:u64,b:u64)->u64 {(a+b)%N}
fn sub(a:u64,b:u64)->u64 {(a+N-b)%N}
fn neg(a:u64)->u64 {if a==0 {0} else {N-a}}
fn mul(a:u64,b:u64)->u64 {((a as u128*b as u128)%N as u128) as u64}
fn pow(mut x:u64,mut e:u64)->u64 {
    let mut z=1;
    while e!=0 {if e&1!=0 {z=mul(z,x)} x=mul(x,x);e>>=1}
    z
}
fn to_point(q:Option<[u64;2]>)->Point {q.map(|[x,y]|(x,y))}
fn coefficient_row(terms:&[[i64;2]],size:usize)->Result<Vec<u64>,String>{
    if terms.len()>32 {return Err("overlong relation".into())}
    let mut row=vec![0;size+1];
    for &[i,c] in terms {
        if i<0 || i as usize>=size || c==0 || !(-8..=8).contains(&c) {
            return Err(format!("invalid term index={i}, coefficient={c}"))
        }
        row[i as usize]=add(row[i as usize],c.rem_euclid(N as i64) as u64);
    }
    Ok(row)
}
fn group_sum(ar:&mut Arithmetic,pts:&[Point],terms:&[[i64;2]])->Result<Point,String>{
    let mut sum=None;
    for &[i,c] in terms {
        if i<0 || i as usize>=pts.len() || c==0 || !(-8..=8).contains(&c){
            return Err("invalid term during EC replay".into())
        }
        let p=if c<0 {ar.neg(PRIME31,pts[i as usize])} else {pts[i as usize]};
        for _ in 0..c.unsigned_abs() {sum=ar.add(PRIME31,sum,p);}
    }
    Ok(sum)
}
// Each independent pivot records the 1-based position of the relation that
// introduced it. Pivots are never replaced, so this is a chronological basis.
struct Echelon {rows:Vec<Option<(Vec<u64>,usize)>>,rank:usize}
impl Echelon {
    fn new(cols:usize)->Self {Self{rows:vec![None;cols],rank:0}}
    fn reduce(&self,row:&mut [u64]){
        let cols=self.rows.len();
        for i in 0..cols {
            if row[i]==0 {continue}
            if let Some((base,_))=&self.rows[i] {
                let k=row[i];
                for j in i..=cols {row[j]=sub(row[j],mul(k,base[j]));}
            }
        }
    }
    fn insert(&mut self,mut row:Vec<u64>,relation_prefix:usize)->Result<bool,String>{
        self.reduce(&mut row);
        let cols=self.rows.len();
        if let Some(i)=(0..cols).find(|&i|row[i]!=0){
            let inverse=pow(row[i],N-2);
            for x in &mut row[i..] {*x=mul(*x,inverse);}
            self.rows[i]=Some((row,relation_prefix));self.rank+=1;Ok(true)
        } else if row[cols]!=0 {
            Err("inconsistent authenticated RHS rows".into())
        } else {Ok(false)}
    }
    // Reducing (c,0) yields residual RHS -c dot L. A unique expansion in
    // the chronological independent pivot basis uses a well-defined subset
    // of pivots. Their latest birth index is the *earliest* prefix at which
    // the target was determined, without rescanning each relation prefix.
    fn projected(&self,mut c:Vec<u64>)->Option<(u64,usize)>{
        let cols=self.rows.len();
        let mut earliest_prefix=0;
        for i in 0..cols {
            if c[i]==0 {continue}
            let Some((base,birth))=&self.rows[i] else {return None};
            let k=c[i];
            for j in i..=cols {c[j]=sub(c[j],mul(k,base[j]));}
            earliest_prefix=earliest_prefix.max(*birth);
        }
        Some((neg(c[cols]),earliest_prefix))
    }
}
fn verify(data:&Certificate)->Result<Report,String>{
    if data.base.is_empty() || data.base.len()>4096 ||
        data.relations.len()>65_536 || data.targets.len()>4096 {
        return Err("input exceeds fixed verifier caps".into())
    }
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar)?;
    let mut base=Vec::new();
    for (i,&xy) in data.base.iter().enumerate(){
        let p=to_point(Some(xy));
        if !ar.on_curve(PRIME31,p){return Err(format!("base[{i}] invalid curve point"))}
        if data.base[..i].contains(&xy){return Err(format!("duplicate base[{i}]"))}
        base.push(p);
    }
    let mut matrix=Echelon::new(base.len());
    let mut dependent=0;
    for (i,r) in data.relations.iter().enumerate(){
        if r.rhs>=N {return Err(format!("relation[{i}] RHS out of range"))}
        let lhs=group_sum(&mut ar,&base,&r.terms)?;
        let rhs=ar.scalar(PRIME31,r.rhs,Some(PRIME31.g));
        if lhs!=rhs {return Err(format!("relation[{i}] EC replay failed"))}
        let mut row=coefficient_row(&r.terms,base.len())?;
        row[base.len()]=r.rhs;
        if !matrix.insert(row,i+1)?{dependent+=1}
    }
    let mut targets=Vec::new();
    let mut determined=0;
    for (i,t) in data.targets.iter().enumerate(){
        if t.offset>=N {return Err(format!("target[{i}] offset out of range"))}
        let q=to_point(t.q);
        if !ar.on_curve(PRIME31,q){return Err(format!("target[{i}] Q off curve"))}
        let witness=group_sum(&mut ar,&base,&t.terms)?;
        let shift=ar.scalar(PRIME31,t.offset,Some(PRIME31.g));
        if witness!=ar.add(PRIME31,q,shift) {
            return Err(format!("target[{i}] public decomposition failed EC replay"))
        }
        // Harness known answer only: never incorporated into the row space.
        if let Some(k)=t.claimed_log {
            if k>=N || ar.scalar(PRIME31,k,Some(PRIME31.g))!=q {
                return Err(format!("target[{i}] claimed_log independent replay failed"))
            }
        }
        let projection=matrix.projected(coefficient_row(&t.terms,base.len())?);
        let first_determined_relation_prefix=projection.map(|(_,prefix)|prefix);
        let recovered=projection.map(|(v,_)|sub(v,t.offset));
        if let Some(d)=recovered {
            if ar.scalar(PRIME31,d,Some(PRIME31.g))!=q {
                return Err(format!("target[{i}] recovered scalar EC replay failed"))
            }
            if let Some(k)=t.claimed_log {
                if d!=k {return Err(format!("target[{i}] recovered != claimed"))}
            }
            determined+=1;
        }
        targets.push(TargetResult{index:i,determined:recovered.is_some(),
            recovered,first_determined_relation_prefix,
            // The public group witness passed for every accepted target,
            // including those whose logarithm is still underdetermined.
            group_replay_passed:true,scalar_replay_passed:recovered.is_some()});
    }
    Ok(Report{p:PRIME31.p,n:N,base_size:base.len(),
        relation_count:data.relations.len(),relation_rank:matrix.rank,
        dependent_relations:dependent,target_count:targets.len(),
        determined_targets:determined,targets,arithmetic_diagnostic:ar.counts,
        scope:"source-reusing public-certificate verification; no solver/timing/PCC claims"})
}
fn main(){
    let args=env::args().collect::<Vec<_>>();
    if args.len()!=2 {eprintln!("usage: cheap-ecdlp-certificate <public.json>");process::exit(2)}
    let result=fs::read_to_string(&args[1]).map_err(|e|e.to_string())
        .and_then(|s|serde_json::from_str::<Certificate>(&s).map_err(|e|e.to_string()))
        .and_then(|x|verify(&x));
    match result {
        Ok(r)=>println!("{}",serde_json::to_string_pretty(&r).expect("serialize report")),
        Err(e)=>{eprintln!("FAIL: {e}");process::exit(1)}
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn sample()->Certificate{
        let mut ar=Arithmetic::default();
        let g=Some(PRIME31.g);
        let base=(1..=4).map(|k|{
            let (x,y)=ar.scalar(PRIME31,k,g).expect("nonzero sample");
            [x,y]
        }).collect();
        let q=ar.scalar(PRIME31,3,g).map(|(x,y)|[x,y]);
        Certificate{base,relations:vec![
            Relation{terms:vec![[0,1]],rhs:1},
            Relation{terms:vec![[1,1]],rhs:2},
        ],targets:vec![
            Target{q,offset:0,terms:vec![[0,1],[1,1]],claimed_log:Some(3)},
            Target{q,offset:0,terms:vec![[2,1]],claimed_log:Some(3)},
        ]}
    }
    #[test]
    fn rank_deficient_but_one_target_determined(){
        let r=verify(&sample()).expect("valid public samples");
        assert_eq!((r.relation_rank,r.base_size,r.determined_targets),(2,4,1));
        assert_eq!(r.targets[0].recovered,Some(3));
        assert_eq!(r.targets[1].recovered,None);
        assert_eq!(r.targets[0].first_determined_relation_prefix,Some(2));
        assert_eq!(r.targets[1].first_determined_relation_prefix,None);
        assert!(r.targets[1].group_replay_passed);
        assert!(!r.targets[1].scalar_replay_passed);
    }
    #[test]
    fn homogeneous_row_recovers_previous_underdetermined_target(){
        let mut c=sample();
        c.relations.push(Relation{terms:vec![[2,1],[1,-1],[0,-1]],rhs:0});
        let r=verify(&c).expect("P3-P2-P1=O");
        assert_eq!((r.relation_rank,r.determined_targets),(3,2));
        assert_eq!(r.targets[1].recovered,Some(3));
        assert_eq!(r.targets[1].first_determined_relation_prefix,Some(3));
    }
    #[test]
    fn recover_unknown_target_without_harness_scalar(){
        let mut c=sample();
        for t in &mut c.targets {t.claimed_log=None;}
        let result=verify(&c).expect("solver uses only public targets and rows");
        assert_eq!(result.targets[0].recovered,Some(3));
        assert_eq!(result.targets[1].recovered,None);
    }
    #[test]
    fn shifted_unknown_public_target_recovers_subtracted_offset(){
        let mut c=sample();
        // Q=G, Q+[1]G=2G=P1. Neither the fixture secret nor a
        // post-recovery claim is provided to the verifier.
        c.targets=vec![Target{
            q:Some([PRIME31.g.0,PRIME31.g.1]),
            offset:1,terms:vec![[1,1]],claimed_log:None,
        }];
        let out=verify(&c).expect("valid shifted target");
        assert_eq!(out.determined_targets,1);
        assert_eq!(out.targets[0].recovered,Some(1));
    }
    #[test]
    fn signed_negation_and_identity_target_group_replays(){
        let mut c=sample();
        let mut ar=Arithmetic::default();
        let neg=ar.neg(PRIME31,Some(PRIME31.g)).map(|(x,y)|[x,y]);
        c.targets=vec![
            Target{q:neg,offset:0,terms:vec![[0,-1]],claimed_log:None},
            Target{q:None,offset:0,terms:vec![[0,1],[0,-1]],claimed_log:None},
        ];
        let out=verify(&c).expect("negative and point-at-infinity equations");
        assert_eq!(out.determined_targets,2);
        assert_eq!(out.targets[0].recovered,Some(N-1));
        assert_eq!(out.targets[1].recovered,Some(0));
    }
    #[test]
    fn relation_order_changes_first_visibility_but_not_answer(){
        let mut c=sample();
        c.targets=vec![Target{
            q:Some([PRIME31.g.0,PRIME31.g.1]),
            offset:0,terms:vec![[0,1]],claimed_log:None,
        }];
        let first=verify(&c).expect("public G, first pivot");
        assert_eq!(first.targets[0].first_determined_relation_prefix,Some(1));
        c.relations.swap(0,1);
        let second=verify(&c).expect("same independent rows, reordered");
        assert_eq!(second.targets[0].first_determined_relation_prefix,Some(2));
        assert_eq!(first.targets[0].recovered,second.targets[0].recovered);
    }
    #[test]
    fn identity_is_determined_with_zero_relation_prefix(){
        let mut c=sample();
        c.relations.clear();
        c.targets=vec![Target{
            q:None,offset:0,terms:vec![[0,1],[0,-1]],claimed_log:None,
        }];
        let r=verify(&c).expect("zero-coefficient target");
        assert_eq!(r.relation_rank,0);
        assert_eq!(r.targets[0].first_determined_relation_prefix,Some(0));
        assert_eq!(r.targets[0].recovered,Some(0));
        assert!(r.targets[0].group_replay_passed);
        assert!(r.targets[0].scalar_replay_passed);
    }
    #[test]
    fn reject_false_rhs(){let mut c=sample();c.relations[0].rhs=2;assert!(verify(&c).is_err());}
    #[test]
    fn reject_false_target_witness(){let mut c=sample();c.targets[0].offset=1;assert!(verify(&c).is_err());}
    #[test]
    fn reject_false_harness_answer(){let mut c=sample();c.targets[0].claimed_log=Some(4);assert!(verify(&c).is_err());}
    #[test]
    fn accept_negative_signed_relation_and_count_dependency(){
        let mut c=sample();c.relations.push(Relation{terms:vec![[0,-1]],rhs:N-1});
        let r=verify(&c).expect("negative sign, exact RHS");
        assert_eq!(r.dependent_relations,1);
    }
}

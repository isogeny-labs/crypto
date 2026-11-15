//! Dickson first-collision branches for the exact ordinary prime31 curve.
//! Field-algebra preconditioning gate and *existing* Rust F4 diagnostic.
//! The group-pair checks are validators only, never an ECDLP relation oracle.
//! [pcc:unverified]
use prime31_algebraic_ic::{
    check_curve, Arithmetic, Base, PRIME31,
    f4_fp_tower::{f4_tower, RPoly, SquareRule, TowerF4Options, TowerRing},
};
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};
use std::time::Duration;

const P: u64 = 2_147_483_647;
const A: u64 = 847_514_846;
const B: u64 = 2_087_444_724;
const DEPTH: usize = 6;

fn mul(x: u64, y: u64) -> u64 { ((x as u128 * y as u128) % P as u128) as u64 }
fn add(x: u64, y: u64) -> u64 { let z=x+y; if z>=P {z-P} else {z} }
fn sub(x: u64, y: u64) -> u64 { if x>=y {x-y} else {x+P-y} }
fn power(mut x: u64, mut k: u64) -> u64 {
    let mut out=1;
    while k>0 { if k&1==1 {out=mul(out,x);} x=mul(x,x); k>>=1; }
    out
}
fn next(x: u64) -> u64 { sub(mul(x,x),2) }
fn first_collision(mut x: u64, mut y: u64) -> Result<usize,String> {
    if x==y {return Err("diagonal not a distinct-pair branch".into());}
    for j in 0..DEPTH {
        if x!=y && add(x,y)==0 {return Ok(j);}
        x=next(x); y=next(y);
    }
    Err("distinct same-fibre points did not collide".into())
}
fn sqroot(x: u64) -> Option<u64> {
    let r=power(x,(P+1)/4);
    (mul(r,r)==x).then_some(r)
}
// S3(x,y,z)= a*z^2 + b*z + c. All roots below are computed
// with prime-field polynomial arithmetic, *not* elliptic-curve sums.
fn s3_roots(x:u64,y:u64)->Result<Vec<u64>,String>{
    let xy=mul(x,y);
    let s=add(x,y);
    let diff=sub(x,y);
    let qa=mul(diff,diff);
    let qb=sub(0,mul(2,add(mul(s,add(xy,A)),mul(2,B))));
    let minus_a=sub(xy,A);
    let qc=sub(mul(minus_a,minus_a),mul(4,mul(B,s)));
    if qa==0 {
        if qb==0 {return Err("unexpected degenerate S3 diagonal".into());}
        return Ok(vec![mul(sub(0,qc),power(qb,P-2))]);
    }
    let discr=sub(mul(qb,qb),mul(4,mul(qa,qc)));
    let rhs_x=add(add(mul(mul(x,x),x),mul(A,x)),B);
    let rhs_y=add(add(mul(mul(y,y),y),mul(A,y)),B);
    if discr!=mul(16,mul(rhs_x,rhs_y)) {return Err("S3 discriminant identity failed".into());}
    let d=sqroot(discr).ok_or("non-rational S3 roots for two liftable points")?;
    let inv=power(mul(2,qa),P-2);
    let mut roots=vec![mul(sub(d,qb),inv),mul(sub(sub(0,qb),d),inv)];
    roots.sort_unstable(); roots.dedup();
    if roots.len()!=2 {return Err("unexpected repeated S3 root".into());}
    Ok(roots)
}
fn ring(c:u64)->TowerRing {
    let mut rules=Vec::new();
    for block in 0..2 {
        for level in 0..DEPTH {
            let last=level+1==DEPTH;
            rules.push(SquareRule{
                next:(!last).then_some((block*DEPTH+level+1) as u8),
                a:0,b:u64::from(!last),c:0,
                d:if last {(c+2)%P} else {2},
            });
        }
    }
    TowerRing{p:P,rules,n_free:1}
}
fn mono(ex: &[(usize,u32)], c:u64)->(Vec<u32>,u64) {
    let mut powers=vec![0_u32;2*DEPTH+1];
    for &(i,j) in ex {powers[i]=j;}
    (powers,c)
}
fn raw_s3()->Vec<(Vec<u32>,u64)> {
    let x=0; let y=DEPTH; let w=2*DEPTH;
    let two_a=mul(2,A); let four_b=mul(4,B);
    vec![
        mono(&[(x,2),(w,2)],1), mono(&[(x,1),(y,1),(w,2)],P-2),
        mono(&[(y,2),(w,2)],1),
        mono(&[(x,2),(y,1),(w,1)],P-2),
        mono(&[(x,1),(y,2),(w,1)],P-2),
        mono(&[(x,1),(w,1)],sub(0,two_a)),
        mono(&[(y,1),(w,1)],sub(0,two_a)),
        mono(&[(w,1)],sub(0,four_b)),
        mono(&[(x,2),(y,2)],1),
        mono(&[(x,1),(y,1)],sub(0,two_a)),
        mono(&[],mul(A,A)),
        mono(&[(x,1)],sub(0,four_b)),
        mono(&[(y,1)],sub(0,four_b)),
    ]
}
fn branch_poly(r:&TowerRing,j:Option<usize>)->RPoly {
    match j {
        Some(k)=>r.from_raw(&[
            mono(&[(k,1)],1),
            mono(&[(DEPTH+k,1)],1),
        ]),
        None=>r.from_raw(&[
            mono(&[(0,1)],1),
            mono(&[(DEPTH,1)],P-1),
        ])
    }
}
fn witness_eval(x:u64,y:u64,w:u64)->Vec<u64> {
    let mut xs=vec![0_u64;2*DEPTH+1];
    xs[0]=x; xs[DEPTH]=y; xs[2*DEPTH]=w;
    for j in 1..DEPTH {
        xs[j]=next(xs[j-1]);
        xs[DEPTH+j]=next(xs[DEPTH+j-1]);
    }
    xs
}
fn main()->Result<(),String> {
    let args:Vec<String>=std::env::args().skip(1).collect();
    if args.len()>1 || (!args.is_empty() && args[0]!="--f4") {
        return Err("usage: prime31_dickson_pair_branches [--f4]".into());
    }
    let do_f4=!args.is_empty();
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar)?;
    let fb=Base::build(PRIME31,DEPTH,&mut ar)?;
    if fb.terminal!=933_677_148 || fb.values.len()!=64 || fb.points.len()!=31 {
        return Err("ordinary prime31 Dickson fixture drift".into());
    }
    let mut full=[0_usize;DEPTH];
    for (i,&x) in fb.values.iter().enumerate() {
        for &y in fb.values.iter().skip(i+1) {
            full[first_collision(x,y)?]+=1;
        }
    }
    if full!=[32,64,128,256,512,1024] {return Err(format!("full-fibre branches changed: {full:?}"));}
    let mut filtered=[0_usize;DEPTH];
    let mut roots_by_branch=[0_usize;DEPTH+1];
    let mut root_x=BTreeSet::new();
    let mut first_pair: BTreeMap<usize,(u64,u64,u64)>=BTreeMap::new();
    let ring=ring(fb.terminal);
    ring.check();
    let poly=ring.from_raw(&raw_s3());
    for (i,&(x,yy)) in fb.points.iter().enumerate() {
        for &(y,zz) in fb.points.iter().skip(i) {
            let branch=if x==y {None} else {Some(first_collision(x,y)?)};
            if let Some(j)=branch {filtered[j]+=1;}
            let solutions=s3_roots(x,y)?;
            let expected_roots=if branch.is_none(){1}else{2};
            if solutions.len()!=expected_roots {
                return Err("signed-pair S3 root multiplicity drift".into());
            }
            for &w in &solutions {
                if !root_x.insert(w) {return Err("duplicate S3 pair-sum abscissa".into());}
                roots_by_branch[branch.unwrap_or(DEPTH)]+=1;
                let witness=witness_eval(x,y,w);
                if ring.eval(&poly,&witness)!=0
                    || ring.eval(&branch_poly(&ring,branch),&witness)!=0 {
                    return Err("S3/Dickson branch polynomial residual nonzero".into());
                }
                if branch.is_some() && !first_pair.contains_key(&branch.unwrap()) {
                    first_pair.insert(branch.unwrap(),(x,y,w));
                }
            }
            // Validator only: algebraic S3 roots above were computed without EC addition.
            let sum=ar.add(PRIME31,Some((x,yy)),Some((y,zz)))
                .ok_or("unexpected identity for positive pair")?;
            if !solutions.contains(&sum.0) {return Err("curve S3 + ordinate failed".into());}
            if x!=y {
                let diff=ar.add(PRIME31,Some((x,yy)),Some((y,(P-zz)%P)))
                    .ok_or("unexpected inverse-pair identity")?;
                if !solutions.contains(&diff.0) || diff.0==sum.0 {
                    return Err("opposite ordinate S3 root failed".into());
                }
            }
        }
    }
    if filtered!=[8,10,31,62,120,234] || roots_by_branch!=[16,20,62,124,240,468,31]
        || root_x.len()!=961 || first_pair.len()!=DEPTH {
        return Err(format!("prime31 liftable branches drift: {filtered:?}; {roots_by_branch:?}"));
    }
    let mut f4_runs=Vec::new();
    if do_f4 {
        let opts=TowerF4Options::new(12)
            .with_budget(Duration::from_secs(5))
            .with_max_nnz(15_000)
            .with_max_dense(15_000);
        // This is an *unconditioned fixed S3 preprocessor*, independent of R.
        // Keep every branch, including diagonal; do not discard failed calls.
        for branch in (0..DEPTH).map(Some).chain(std::iter::once(None)) {
            let extra=branch_poly(&ring,branch);
            let witness=if let Some(j)=branch {
                let (x,y,w)=first_pair[&j];
                Some(witness_eval(x,y,w))
            } else {
                let (x,_)=fb.points[0];
                Some(witness_eval(x,x,s3_roots(x,x)?[0]))
            };
            let rep=f4_tower(&[poly.clone(),extra],&ring,&opts);
            let xs=witness.ok_or("missing positive witness")?;
            if rep.inconsistent || rep.basis.iter().any(|q|ring.eval(q,&xs)!=0) {
                return Err(format!("branched F4 refuted a true positive branch {branch:?}"));
            }
            f4_runs.push(json!({
                "branch":branch.map_or("diagonal".to_string(),|j|j.to_string()),
                "degree_reached":rep.degree_reached,"steps":rep.steps,
                "oversize":rep.oversize,"timed_out":rep.timed_out,
                "max_dense_entries":rep.max_dense_entries,"max_nnz":rep.max_nnz,
                "max_cols":rep.max_cols,"muladds":rep.muladds,"ms_engine_only":rep.ms,
                "inconsistent":rep.inconsistent,
                "verdict":if rep.oversize||rep.timed_out||rep.pairs_above_bound>0||rep.staircase_at_stop.is_some(){
                    "undetermined"
                }else{"basis_only; no extraction"},
            }));
        }
    }
    println!("{}",json!({
        "standing":"[pcc:unverified]",
        "scope":"field-algebra Dickson first-collision preconditioner and optional Rust F4 development ablation; no complete prime31 ECDLP",
        "p":P,"n":PRIME31.n,"terminal":fb.terminal,"depth":DEPTH,
        "fibre_points":fb.values.len(),"curve_liftable_points":fb.points.len(),
        "full_unordered_pair_branch_hist":full,
        "liftable_unordered_pair_branch_hist":filtered,
        "s3_abscissa_roots_by_branch_including_diagonal_last":roots_by_branch,
        "distinct_algebraic_s3_pair_abscissae":root_x.len(),
        "s3_pair_roots_with_multiplicity":roots_by_branch.iter().sum::<usize>(),
        "f4_branch_ablations":f4_runs,
        "reused":"prime31_algebraic_ic Base/Arithmetic and src/cryptanalysis/f4_fp_tower.rs",
        "exclude_from_solver":"generic EC group additions occur only in test validation"
    }));
    Ok(())
}

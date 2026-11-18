//! Exact seven-way Dickson/S3 first-collision *quotient ring* experiment.
//! Reuses native Rust Base, Arithmetic and the actual shared F4 engine.
//! A rational solution is retained only after signed elliptic-curve checks;
//! this diagnostic does not implement an m8 oracle or solve the ECDLP.
//! [pcc:unverified]
use prime31_algebraic_ic::{
    check_curve, Arithmetic, Base, PRIME31,
    f4_fp_tower::{f4_tower, RPoly, SquareRule, TowerF4Options, TowerRing},
};
use serde_json::json;
use std::collections::BTreeMap;
use std::time::Duration;

const P:u64=2_147_483_647;
const A:u64=847_514_846;
const B:u64=2_087_444_724;
const DEPTH:usize=6;

fn add(x:u64,y:u64)->u64{let z=x+y;if z>=P{z-P}else{z}}
fn sub(x:u64,y:u64)->u64{if x>=y{x-y}else{x+P-y}}
fn mul(x:u64,y:u64)->u64{((x as u128*y as u128)%P as u128) as u64}
fn pow(mut x:u64,mut n:u64)->u64{
    let mut out=1;while n>0{if n&1==1{out=mul(out,x);}x=mul(x,x);n>>=1;}out
}
fn sqrt(x:u64)->Option<u64>{
    let r=pow(x,(P+1)/4);(mul(r,r)==x%P).then_some(r)
}
fn f(x:u64)->u64{sub(mul(x,x),2)}
fn first_collision(mut x:u64,mut y:u64)->Result<usize,String>{
    if x==y{return Err("diagonal".into());}
    for j in 0..DEPTH{
        if x!=y && add(x,y)==0{return Ok(j);}
        x=f(x);y=f(y);
    }
    Err("same-fibre pair lacks first collision".into())
}
fn s3_roots(x:u64,y:u64)->Result<Vec<u64>,String>{
    let xy=mul(x,y);
    let a=mul(sub(x,y),sub(x,y));
    let b=sub(0,mul(2,add(mul(add(x,y),add(xy,A)),mul(2,B))));
    let c=sub(mul(sub(xy,A),sub(xy,A)),mul(4,mul(B,add(x,y))));
    if a==0 {
        if b==0 {return Err("S3 degenerate diagonal".into());}
        return Ok(vec![mul(sub(0,c),pow(b,P-2))]);
    }
    let disc=sub(mul(b,b),mul(4,mul(a,c)));
    let d=sqrt(disc).ok_or("S3 quadratic has no rational roots")?;
    let inv=pow(mul(2,a),P-2);
    let mut roots=vec![mul(sub(d,b),inv),mul(sub(sub(0,b),d),inv)];
    roots.sort_unstable();roots.dedup();
    if roots.len()!=2{return Err("S3 roots merged".into());}
    Ok(roots)
}
// Seven quotient rings:
// diagonal: x=y; level 0: y=-x; levels j>=1: y_0^2=y_1+2,
// ..., y_(j-1)^2=2-x_j, followed by the *single* original x depth-6 tower.
// This eliminates 6-j redundant second-tower variables and enforces each
// branch algebraically rather than appending branch polynomials to the
// unreduced two-full-tower system.
fn quotient(j:Option<usize>, terminal:u64)->(TowerRing,usize,Option<usize>){
    let n_y=j.unwrap_or(0); // j=0 and diagonal both eliminate y entirely.
    let mut rules=Vec::with_capacity(n_y+DEPTH);
    if n_y>=1{
        for k in 0..n_y{
            let final_y=k+1==n_y;
            rules.push(SquareRule{
                next:Some(if final_y {(n_y+n_y) as u8} else {(k+1) as u8}),
                a:0,
                b:if final_y {P-1} else {1},
                c:0,d:2,
            });
        }
    }
    for k in 0..DEPTH{
        let terminal_x=k+1==DEPTH;
        rules.push(SquareRule{
            next:(!terminal_x).then_some((n_y+k+1) as u8),
            a:0,b:u64::from(!terminal_x),c:0,
            d:if terminal_x {(terminal+2)%P} else {2},
        });
    }
    (TowerRing{p:P,rules,n_free:1},n_y,if n_y>0{Some(0)}else{None})
}
// Raw S3(x,y,w), as ordered (x-degree,y-degree,w-degree,coefficient),
// with all three *abscissa* variables. The branch j0 or diagonal substitutes
// y=-x or y=x before conversion to the quotient normal form.
fn terms()->[(u32,u32,u32,u64);13]{
    [
        (2,0,2,1),(1,1,2,P-2),(0,2,2,1),
        (2,1,1,P-2),(1,2,1,P-2),
        (1,0,1,sub(0,mul(2,A))),(0,1,1,sub(0,mul(2,A))),
        (0,0,1,sub(0,mul(4,B))),
        (2,2,0,1),(1,1,0,sub(0,mul(2,A))),
        (0,0,0,mul(A,A)),
        (1,0,0,sub(0,mul(4,B))),(0,1,0,sub(0,mul(4,B))),
    ]
}
fn input(r:&TowerRing,n_y:usize,j:Option<usize>)->RPoly{
    let mut exps:BTreeMap<Vec<u32>,u64>=BTreeMap::new();
    for (ex,ey,ew,c) in terms(){
        let mut powers=vec![0u32;r.n_vars()];
        if n_y==0 {
            powers[0]=ex+ey;
        } else {
            powers[n_y]=ex;powers[0]=ey;
        }
        powers[n_y+DEPTH]=ew;
        let coeff=if n_y==0 && j==Some(0) && ey%2==1 {sub(0,c)}else{c};
        let entry=exps.entry(powers).or_insert(0);
        *entry=add(*entry,coeff);
    }
    let raw:Vec<_>=exps.into_iter().filter(|(_,v)|*v!=0).collect();
    r.from_raw(&raw)
}
fn lifted_witness(x:u64,y:u64,w:u64,j:Option<usize>,ring:&TowerRing,n_y:usize)->Result<Vec<u64>,String>{
    if let Some(level)=j{
        if first_collision(x,y)?!=level{return Err("witness in wrong branch".into());}
    }else if x!=y{return Err("diagonal witness x!=y".into());}
    let mut values=vec![0_u64;ring.n_vars()];
    for k in 0..DEPTH{
        values[n_y+k]=if k==0{x}else{f(values[n_y+k-1])};
    }
    if n_y>0 {
        for k in 0..n_y{
            values[k]=if k==0{y}else{f(values[k-1])};
        }
        if mul(values[n_y-1],values[n_y-1])!=sub(2,values[n_y+n_y]){
            return Err("compressed second tower not satisfied".into());
        }
    }else if j==Some(0) && add(x,y)!=0{
        return Err("j0 requires y=-x".into());
    }
    values[n_y+DEPTH]=w;
    Ok(values)
}
fn main()->Result<(),String>{
    let args:Vec<String>=std::env::args().skip(1).collect();
    if args.len()>1 || (!args.is_empty() && args[0]!="--f4"){
        return Err("usage: prime31_dickson_collapsed_s3 [--f4]".into());
    }
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar)?;
    let fb=Base::build(PRIME31,DEPTH,&mut ar)?;
    if fb.terminal!=933_677_148 || fb.values.len()!=64 || fb.points.len()!=31{
        return Err("original curve or Dickson base drift".into());
    }
    let mut arms=Vec::new();
    let mut all_s3_roots=0usize;
    let mut distinct_abscissae=std::collections::BTreeSet::new();
    for branch in (0..DEPTH).map(Some).chain(std::iter::once(None)){
        let (ring,n_y,_yidx)=quotient(branch,fb.terminal);
        ring.check();
        let equation=input(&ring,n_y,branch);
        let mut matches=0usize;
        let mut roots=0usize;
        let mut first_witness=None;
        for (i,&(x,ay)) in fb.points.iter().enumerate(){
            for &(y,by) in fb.points.iter().skip(i){
                let belongs=if x==y {branch.is_none()}else{branch==Some(first_collision(x,y)?)};
                if !belongs {continue;}
                matches+=1;
                for w in s3_roots(x,y)?{
                    let eval=lifted_witness(x,y,w,branch,&ring,n_y)?;
                    if ring.eval(&equation,&eval)!=0 {
                        return Err(format!("collapsed S3 not zero for branch {branch:?}"));
                    }
                    if first_witness.is_none(){first_witness=Some(eval);}
                    distinct_abscissae.insert(w);
                    roots+=1;
                }
                // Independent group validation, never used to discover solutions.
                let plus=ar.add(PRIME31,Some((x,ay)),Some((y,by)))
                    .ok_or("unexpected P+Q identity")?;
                if !s3_roots(x,y)?.contains(&plus.0){
                    return Err("S3 candidate disagrees with independent group sum".into());
                }
            }
        }
        if matches==0{return Err("empty branch".into());}
        let mut row=json!({
            "branch":branch.map_or("diagonal".to_string(),|j|j.to_string()),
            "n_tower":ring.n_tower(),
            "n_free":ring.n_free,
            "normal_form_terms":equation.len(),
            "liftable_unordered_pairs":matches,
            "s3_finite_roots":roots,
            "ec_additions":"verification only"
        });
        if args.first().is_some() {
            let opts=TowerF4Options::new(12).with_budget(Duration::from_secs(5))
                .with_max_nnz(15_000).with_max_dense(15_000);
            let rep=f4_tower(&[equation],&ring,&opts);
            if let Some(xs)=first_witness{
                if rep.inconsistent || rep.basis.iter().any(|q|ring.eval(q,&xs)!=0) {
                    return Err(format!("F4 refuted witnessed collapsed branch {branch:?}"));
                }
            }
            row["f4"]=json!({
                "oversize":rep.oversize,"timed_out":rep.timed_out,
                "pairs_above_bound":rep.pairs_above_bound,
                "staircase_at_stop":rep.staircase_at_stop,
                "degree_reached":rep.degree_reached,
                "steps":rep.steps,"max_cols":rep.max_cols,
                "max_dense_entries":rep.max_dense_entries,"max_nnz":rep.max_nnz,
                "muladds":rep.muladds,"ms_engine_only":rep.ms,
                "basis_elements":rep.basis.len(),
                "verdict":if rep.oversize||rep.timed_out||rep.pairs_above_bound>0||rep.staircase_at_stop.is_some(){
                    "undetermined"
                }else{"basis_only; no extraction"},
            });
        }
        arms.push(row);
        all_s3_roots+=roots;
    }
    if all_s3_roots!=961||distinct_abscissae.len()!=961 {
        return Err("collapsed fibre S3 root census changed".into());
    }
    println!("{}",json!({
        "standing":"[pcc:unverified]",
        "scope":"exact Dickson first-collision quotient-ring S3 preconditioning; development F4 only, no complete ECDLP",
        "p":P,"n":PRIME31.n,"depth":DEPTH,"actual_liftable_base":fb.points.len(),
        "distinct_finite_pair_s3_roots":distinct_abscissae.len(),
        "all_seven_branches_required":true,
        "branches":arms,
        "reused":"existing native Rust Base/Arithmetic/f4_fp_tower.rs"
    }));
    Ok(())
}

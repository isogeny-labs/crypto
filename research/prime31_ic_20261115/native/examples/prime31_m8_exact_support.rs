//! Exact prime31 signed 4+4 target reachability; NOT algebraic F4 or ECDLP cost.
//! Uses existing Rust PRIME31, Arithmetic and Dickson Base. [pcc:unverified]
//! Run prefix (CI-sized): cargo run --release --example prime31_m8_exact_support
//! Run full census: cargo run --release --example prime31_m8_exact_support -- --full
use prime31_algebraic_ic::{check_curve, Arithmetic, Base, Point, PRIME31};
use serde_json::json;
use std::time::Instant;

const P: u64 = 2_147_483_647;
const N: u64 = 2_147_446_831;
const L: usize = 21_840;
const R: usize = 29_120;
#[inline] fn plus(x:u64,y:u64)->u64 {let t=x+y;if t>=P {t-P}else{t}}
#[inline] fn minus(x:u64,y:u64)->u64 {if x>=y{x-y}else{x+P-y}}
#[inline] fn times(x:u64,y:u64)->u64 {
    // Mersenne reduction of a product of canonical Fp residues.
    let z=x*y;let z=(z&P)+(z>>31);let z=(z&P)+(z>>31);
    if z>=P {z-P}else{z}
}
fn power(mut x:u64,mut k:u64)->u64 {
    let mut v=1;
    while k>0 {if k&1==1 {v=times(v,x);}x=times(x,x);k>>=1;}v
}
fn fours(base:&Base,ar:&mut Arithmetic,start:usize,end:usize)->Vec<Point> {
    let mut out=Vec::new();
    for i in start..end {for j in i+1..end {for k in j+1..end {for l in k+1..end {
        for mask in 0..16 {
            let mut sum=None;
            for (bit,&idx) in [i,j,k,l].iter().enumerate() {
                let p=Some(base.points[idx]);
                let term=if mask&(1<<bit)==0 {p}else{ar.neg(PRIME31,p)};
                sum=ar.add(PRIME31,sum,term);
            }
            out.push(sum);
        }
    }}}}
    out
}
fn mark(bits:&mut [u64],q:Point,count:&mut u64) {
    // Full-group encoding: the x coordinate and y parity distinguish P/-P.
    let key=match q {Some((x,y))=>2*x+(y&1),None=>2*P} as usize;
    let w=&mut bits[key>>6];let mask=1u64<<(key&63);
    if *w&mask==0 {*w|=mask;*count+=1;}
}
fn census(limit:usize)->Result<serde_json::Value,String>{
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar)?;
    let base=Base::build(PRIME31,6,&mut ar)?;
    if PRIME31.p!=P||PRIME31.n!=N||base.terminal!=933_677_148
        ||base.points.len()!=31 {return Err("registered curve/base changed".into());}
    let left=fours(&base,&mut ar,0,15);
    let right=fours(&base,&mut ar,15,31);
    if left.len()!=L||right.len()!=R {return Err("split template count changed".into());}
    let mut bits=vec![0u64;(2*P as usize+1).div_ceil(64)];
    let mut den=vec![0u64;R];
    let mut prefix=vec![0u64;R];
    let mut distinct=0u64;
    let mut total=0u64;
    let mut exceptionals=0u64;
    let mut checked=0u64;
    let mut inversions=0u64;
    let start=Instant::now();
    for &a in left.iter().take(limit) {
        let Some((ax,ay))=a else {
            for &b in &right {mark(&mut bits,b,&mut distinct);total+=1;}
            continue;
        };
        let mut product=1u64;
        for (j,&b) in right.iter().enumerate() {
            prefix[j]=product;
            den[j]=match b {
                None=>0,
                Some((bx,by)) if bx==ax && plus(ay,by)==0=>0,
                Some((bx,_)) if bx==ax=>plus(ay,ay),
                Some((bx,_))=>minus(bx,ax)
            };
            if den[j]!=0 {product=times(product,den[j]);}
        }
        let mut inverse=power(product,P-2);
        inversions+=1;
        for j in (0..R).rev() {
            let b=right[j];
            let q=if den[j]==0 {
                exceptionals+=1;
                if b.is_none(){a}else{None}
            }else{
                let id=times(inverse,prefix[j]);
                inverse=times(inverse,den[j]);
                let (bx,by)=b.ok_or("nonzero denominator from identity")?;
                let nom=if bx==ax{plus(times(3,times(ax,ax)),PRIME31.a)}
                        else{minus(by,ay)};
                let slope=times(nom,id);
                let x=minus(minus(times(slope,slope),ax),bx);
                let y=minus(times(slope,minus(ax,x)),ay);
                Some((x,y))
            };
            // Sparse independent verification against the inherited EC adder.
            if total%1_048_576==0 {
                if ar.add(PRIME31,a,b)!=q {
                    return Err(format!("batched point mismatch at {total}"));
                }
                checked+=1;
            }
            mark(&mut bits,q,&mut distinct);
            total+=1;
        }
    }
    if total!=limit as u64*R as u64 {return Err("count mismatch".into());}
    if (limit==100&&distinct!=2_911_034)
       ||(limit==L&&distinct!=552_008_270) {
        return Err(format!("producer cross-check mismatch: {distinct}"));
    }
    Ok(json!({
        "standing":"[pcc:unverified]",
        "scope":"exact signed 4+4 group reachability, not F4 extraction or ECDLP",
        "p":P,"n":N,"depth":6,"B":31,
        "left_templates":L,"right_templates":R,
        "prefix_left":limit,"templates_processed":total,
        "distinct_reachable_target_points":distinct,
        "uniform_target_probability_exact":format!("{distinct}/{N}"),
        "batch_inversions":inversions,
        "identity_or_doubling_cases":exceptionals,
        "inherited_EC_adder_spot_checks":checked,
        "development_elapsed_seconds_not_solver_benchmark":start.elapsed().as_secs_f64()
    }))
}
fn main()->Result<(),String>{
    let args:Vec<_>=std::env::args().skip(1).collect();
    let limit=match args.as_slice(){
        []=>100,
        [v] if v=="--full"=>L,
        [v] if v.starts_with("--prefix-left=")=>
            v[14..].parse::<usize>().map_err(|e|e.to_string())?,
        _=>return Err("usage: prime31_m8_exact_support [--full|--prefix-left=N]".into())
    };
    if limit==0||limit>L{return Err("prefix out of range".into());}
    println!("{}",census(limit)?);
    Ok(())
}

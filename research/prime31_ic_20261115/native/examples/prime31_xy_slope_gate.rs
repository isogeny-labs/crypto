//! Prime31 hybrid x/y slope projection: oriented-point semantic control.
//! Uses existing Rust Arithmetic and Dickson Base; not an IC relation oracle.
//! cargo run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_xy_slope_gate
use prime31_algebraic_ic::{Arithmetic,Base,PRIME31,check_curve};
use serde_json::json;
const P:u64=2_147_483_647;
fn mul(a:u64,b:u64)->u64{((a as u128*b as u128)%(P as u128)) as u64}
fn sub(a:u64,b:u64)->u64{(a+P-b)%P}
fn pow(mut a:u64,mut n:u64)->u64{
    let mut r=1;
    while n>0{if n&1==1{r=mul(r,a);}a=mul(a,a);n>>=1;}r
}
fn run()->Result<serde_json::Value,String>{
    let e=PRIME31;
    let mut ar=Arithmetic::default();
    check_curve(e,&mut ar)?;
    let base=Base::build(e,6,&mut ar)?;
    if base.points.len()!=31{return Err("unexpected B31 fixture".into());}
    let mut distinct_y=std::collections::BTreeSet::new();
    for &(x,y) in &base.points {
        if !distinct_y.insert(y){return Err("base y collision".into());}
        // y-fixed cubic: x^3 + a*x + b - y^2 = 0.
        let f=(mul(mul(x,x),x)+mul(e.a,x)+e.b+P-mul(y,y))%P;
        if f!=0{return Err("cubic curve equation failed".into());}
    }
    let mut oriented=0u64;
    let mut exceptional=0u64;
    for &q in base.points.iter().take(8){
        for &r in base.points.iter().take(8){
            let (x,y)=q;
            let (z,w)=r;
            if x==z{exceptional+=1;continue;}
            // t*(z-x)=w-y; x3=t^2-x-z; y3=t*(x-x3)-y.
            let t=mul(sub(w,y),pow(sub(z,x),P-2));
            if mul(t,sub(z,x))!=sub(w,y){return Err("slope constraint failed".into());}
            let xx=sub(sub(mul(t,t),x),z);
            let yy=sub(mul(t,sub(x,xx)),y);
            if ar.add(e,Some(q),Some(r))!=Some((xx,yy)){
                return Err("oriented hybrid sum failed".into());
            }
            oriented+=1;
        }
    }
    if oriented!=56||exceptional!=8{return Err("unexpected gate counts".into());}
    Ok(json!({"standing":"[pcc:unverified]","curve":"prime31 ordinary cofactor-one",
              "base_size":base.points.len(),"unique_y":distinct_y.len(),
              "oriented_slope_identities":oriented,"equal_x_exceptions":exceptional,
              "note":"semantic control only; no y-cubic factorization or IC solver timing in Rust"}))
}
fn main(){match run(){Ok(v)=>println!("{}",serde_json::to_string_pretty(&v).unwrap()),
                             Err(e)=>{eprintln!("{e}");std::process::exit(1);}}}
#[cfg(test)]
mod tests{#[test]fn hybrid_slope_semantics(){super::run().unwrap();}}

//! Prime31 RR5 exact four-leaf algebraic gate.
//! Inputs to discovery are the PUBLIC target and factor-base x coordinates.
//! No group-sum tables or factor-base pair/quadruple enumeration.
//! The native F4 solver's solve() is deliberately not called: it can
//! enumerate all p field values as a fallback. Failure to triangularize
//! is Undetermined, not an empty-query certificate.
//! This first implementation caps B at 8: dense symbolic remainders
//! are not an acceptable B677 complexity claim.

use crypto_lib::cryptanalysis::f4_fp::{f4,normalise,F4Options,Ordering,Poly};
use crypto_lib::cryptanalysis::glv_invariant_base::poly_roots_fp_counted_until;
use crypto_lib::cryptanalysis::ic_boundary::{PrimeCurve,PrimePoint,GroupOps};
use crypto_lib::cryptanalysis::semaev_prime_poly::Fp;
use std::collections::{BTreeMap,BTreeSet};
use std::time::{Instant,Duration};

const P:u64=2_147_483_647;
const A:u64=847_514_846;
const B:u64=2_087_444_724;
const G:PrimePoint=PrimePoint{x:625_037_715,y:2_100_627_251,infinity:false};
type E=BTreeMap<[u32;3],u64>;

fn c(x:u64)->E {if x==0{E::new()}else{BTreeMap::from([([0;3],x)])}}
fn v(i:usize)->E {let mut e=[0;3];e[i]=1;BTreeMap::from([(e,1)])}
fn add(a:&E,b:&E,f:Fp)->E {
    let mut x=a.clone();for (&k,&t) in b {
        let z=f.add(x.get(&k).copied().unwrap_or(0),t);
        if z==0{x.remove(&k);}else{x.insert(k,z);}
    } x
}
fn scale(a:&E,k:u64,f:Fp)->E {
    a.iter().filter_map(|(&e,&z)|{
        let q=f.mul(z,k);(q!=0).then_some((e,q))
    }).collect()
}
fn sub(a:&E,b:&E,f:Fp)->E {add(a,&scale(b,f.neg(1),f),f)}
fn mul(a:&E,b:&E,f:Fp)->Result<E,String> {
    let mut r=E::new();
    for (&x,&u) in a {
        for (&y,&w) in b {
            let z=[x[0]+y[0],x[1]+y[1],x[2]+y[2]];
            let s=f.add(r.get(&z).copied().unwrap_or(0),f.mul(u,w));
            if s==0{r.remove(&z);}else{r.insert(z,s);}
            if r.len()>20_000{return Err("dense three-variable monomial cap".into());}
        }
    }
    Ok(r)
}
fn ev(a:&E,sol:[u64;3],f:Fp)->u64 {
    a.iter().fold(0,|z,(e,&k)|{
        let mut t=k;
        for i in 0..3 {t=f.mul(t,f.pow(sol[i],e[i] as u64));}
        f.add(z,t)
    })
}
fn group()->PrimeCurve{PrimeCurve{p:P,a:A,b:B}}
fn scalar(curve:&PrimeCurve,mut p:PrimePoint,mut n:u64)->PrimePoint{
    let mut acc=PrimePoint::INFINITY;let mut ops=GroupOps::default();
    while n>0 {
        if n&1!=0 {acc=curve.add(&mut ops,acc,p);}
        n>>=1;if n!=0{p=curve.double(&mut ops,p);}
    }acc
}
fn base4()->Vec<PrimePoint>{
    let curve=group();
    (1..=4).map(|k|scalar(&curve,G,k)).collect()
}
fn polynomial(xs:&[u64],f:Fp)->Vec<u64> {
    let mut h=vec![1];
    for &x in xs {
        let mut nxt=vec![0;h.len()+1];
        for (i,&z) in h.iter().enumerate(){
            nxt[i]=f.sub(nxt[i],f.mul(x,z));
            nxt[i+1]=f.add(nxt[i+1],z);
        }h=nxt;
    }h
}
fn quartic(r:PrimePoint,f:Fp)->Result<[E;5],String>{
    if r.infinity{return Err("RR5 finite R chart only".into());}
    // F=(x-u)(alpha+beta*x)+(y-v)(x+t), F(R)=0.
    // D=[v(x+t)-(x-u)(alpha+beta*x)]^2-fcurve(x)(x+t)^2
    //  = -(x-u)*M(x), M monic quartic.
    let (alpha,beta,t)=(v(0),v(1),v(2));
    let z1=add(&sub(&c(r.y),&alpha,f),&scale(&beta,r.x,f),f);
    let z0=add(&scale(&t,r.y,f),&scale(&alpha,r.x,f),f);
    let d4=sub(&mul(&beta,&beta,f)?,&scale(&t,2,f),f);
    let d3=sub(&sub(&scale(&mul(&beta,&z1,f)?,f.neg(2),f),
        &mul(&t,&t,f)?,f),&c(A),f);
    let d2=sub(&sub(&sub(&mul(&z1,&z1,f)?,
        &scale(&mul(&beta,&z0,f)?,2,f),f),
        &scale(&t,f.mul(2,A),f),f),&c(B),f);
    let d1=sub(&sub(&scale(&mul(&z1,&z0,f)?,2,f),
        &scale(&mul(&t,&t,f)?,A,f),f),
        &scale(&t,f.mul(2,B),f),f);
    let h4=c(f.neg(1));
    let h3=add(&d4,&scale(&h4,r.x,f),f);
    let h2=add(&d3,&scale(&h3,r.x,f),f);
    let h1=add(&d2,&scale(&h2,r.x,f),f);
    let h0=add(&d1,&scale(&h1,r.x,f),f);
    Ok([scale(&h0,f.neg(1),f),scale(&h1,f.neg(1),f),
        scale(&h2,f.neg(1),f),scale(&h3,f.neg(1),f),c(1)])
}
fn system(q:PrimePoint,base:&[PrimePoint])->Result<Vec<Poly>,String>{
    let curve=group();let f=Fp::new(P);
    if q.infinity || !curve.is_on_curve(q){
        return Err("invalid finite public target".into());
    }
    if !(4..=8).contains(&base.len()){
        return Err("B>8 needs unexpanded quotient circuit".into());
    }
    if base.iter().any(|&pt|pt.infinity||!curve.is_on_curve(pt)){
        return Err("invalid base".into());
    }
    let mut xs=base.iter().map(|pt|pt.x).collect::<Vec<_>>();
    xs.sort_unstable();
    if xs.windows(2).any(|p|p[0]==p[1]){
        return Err("base needs distinct abscissae".into());
    }
    let m=quartic(curve.neg(q),f)?;
    let h=polynomial(&xs,f);
    let mut t=h.into_iter().map(c).collect::<Vec<_>>();
    for i in (4..t.len()).rev(){
        let a=t[i].clone();
        for j in 0..5 {
            t[i-4+j]=sub(&t[i-4+j],&mul(&a,&m[j],f)?,f);
        }
    }
    Ok(t.iter().take(4).map(|x| {
        let poly:Poly=x.iter().map(|(e,&k)|(e.to_vec(),k)).collect();
        normalise(&poly,P,Ordering::Lex)
    }).collect())
}
fn poly_eval(poly:&Poly,z:[u64;3],f:Fp)->u64 {
    poly.iter().fold(0,|sum,(e,&a)|{
        let t=e.iter().enumerate().fold(a,|v,(i,&n)|
            f.mul(v,f.pow(z[i],n as u64)));
        f.add(sum,t)
    })
}
/// Return a (variable,dense coefficients) univariate consequence of a
/// completed F4 basis after substituting earlier roots. There is NO
/// enumeration of an unconstrained field coordinate.
fn find_uni(basis:&[Poly],a:&[Option<u64>;3],f:Fp)
    ->Result<Option<(usize,Vec<u64>)>,()>{
    let mut best:Option<(usize,Vec<u64>)>=None;
    for g in basis {
        let mut var=None;let mut multivar=false;
        let mut terms=BTreeMap::<usize,u64>::new();
        for (e,&coeff) in g {
            let mut t=coeff;let mut local=None;
            for i in 0..3 {
                if e[i]==0{continue;}
                if let Some(x)=a[i]{t=f.mul(t,f.pow(x,e[i] as u64));}
                else if local.is_none(){local=Some(i);}
                else if local!=Some(i){multivar=true;break;}
            }
            if multivar{break;}
            if let Some(i)=local {
                if var.is_some()&&var!=Some(i){multivar=true;break;}
                var=Some(i);
                let k=e[i] as usize;
                terms.insert(k,f.add(terms.get(&k).copied().unwrap_or(0),t));
            }else{
                terms.insert(0,f.add(terms.get(&0).copied().unwrap_or(0),t));
            }
        }
        if multivar{continue;}
        let mut d=vec![0u64;terms.keys().last().copied().unwrap_or(0)+1];
        for (k,t) in terms{d[k]=t;}
        while d.last()==Some(&0){d.pop();}
        if d.len()==1 && d[0]!=0{return Err(());}
        if let Some(i)=var {
            if d.len()>1 && best.as_ref().map_or(true,|(_,old)|d.len()<old.len()){
                best=Some((i,d));
            }
        }
    }Ok(best)
}
fn triangular(
    basis:&[Poly],input:&[Poly],a:&mut [Option<u64>;3],
    roots:&mut BTreeSet<[u64;3]>,muls:&mut u64,deadline:Instant
)->Result<(),String>{
    if Instant::now()>deadline{return Err("triangular deadline".into());}
    if a.iter().all(Option::is_some){
        let z=[a[0].unwrap(),a[1].unwrap(),a[2].unwrap()];
        if input.iter().all(|g|poly_eval(g,z,Fp::new(P))==0){roots.insert(z);}
        return Ok(());
    }
    let (i,g)=match find_uni(basis,a,Fp::new(P)) {
        Err(())=>return Ok(()),
        Ok(Some(t))=>t,
        Ok(None)=>return Err("no triangular polynomial: refuse field scan".into()),
    };
    let values=poly_roots_fp_counted_until(
        &g,P,0x52523546524F4D,&mut *muls,Some(deadline)
    ).ok_or("rootfactor deadline")?;
    if values.len()>10000{return Err("root branch cap".into());}
    for x in values {
        a[i]=Some(x);
        let r=triangular(basis,input,a,roots,muls,deadline);
        a[i]=None;
        r?;
    }
    Ok(())
}
fn extract(input:&[Poly],budget:Duration)->Result<(Vec<[u64;3]>,u64,u64),String>{
    let start=Instant::now();
    let opts=F4Options::new(Ordering::Lex,12).with_budget(budget);
    let rep=f4(input,3,P,&opts);
    if rep.timed_out||rep.pairs_above_bound>0{
        return Err(format!("F4 censored: timed_out={} pairs_above_bound={}",
            rep.timed_out,rep.pairs_above_bound));
    }
    if rep.inconsistent{return Ok((vec![],rep.field_ops,0));}
    let mut roots=BTreeSet::new();let mut a=[None;3];let mut ops=0;
    triangular(&rep.basis,input,&mut a,&mut roots,&mut ops,start+budget)?;
    Ok((roots.into_iter().collect(),rep.field_ops,ops))
}
/// Recover signed leaves from every completed algebraic coefficient solution.
fn signed_lift(q:PrimePoint,base:&[PrimePoint],params:[u64;3])
    ->Option<Vec<(usize,i8)>>{
    let curve=group();let f=Fp::new(P);let r=curve.neg(q);
    let m=quartic(r,f).ok()?;
    let coeff=m.iter().map(|p|ev(p,params,f)).collect::<Vec<_>>();
    if coeff[4]!=1{return None;}
    let mut ops=0;
    let roots=poly_roots_fp_counted_until(
        &coeff,P,0x464F55524C454146,&mut ops,None)?;
    if roots.len()!=4{return None;}
    let mut sum=PrimePoint::INFINITY;let mut steps=GroupOps::default();
    let mut row=Vec::new();
    for x in roots {
        let i=base.iter().position(|p|p.x==x)?;
        let den=f.add(x,params[2]);
        if den==0 || x==r.x {return None;} // degenerate charts censored
        let u=f.add(params[0],f.mul(params[1],x));
        let y=f.sub(r.y,f.mul(f.mul(f.sub(x,r.x),u),f.inv(den)));
        let p=PrimePoint::affine(x,y);
        if !curve.is_on_curve(p){return None;}
        let sign=if p==base[i]{1}
            else if p==curve.neg(base[i]){-1}
            else{return None};
        row.push((i,sign));
        sum=curve.add(&mut steps,sum,p);
    }
    if sum!=q{return None;}
    row.sort_unstable();Some(row)
}
fn main(){
    // Known-answer pilot exercises original prime-order curve only;
    // G multiples must not be used for production index-calculus timing.
    let base=base4();let c=group();let q=scalar(&c,G,10);
    let input=system(q,&base).expect("public system");
    match extract(&input,Duration::from_secs(5)){
        Ok((params,f4_mul,root_mul))=>{
            let rows=params.into_iter().filter_map(|p|signed_lift(q,&base,p))
                .collect::<BTreeSet<_>>();
            println!("PRIME31_RR5 p={P} B=4 algebraic_complete=true signed_rows={} f4_muls={f4_mul} root_muls={root_mul} (development, no cold ECDLP)",rows.len());
        }
        Err(reason)=>println!("PRIME31_RR5 p={P} B=4 status=undetermined reason={reason}"),
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn symbolic_quartic_has_target_and_witness_roots(){
        let curve=group();let f=Fp::new(P);
        let base=base4();let q=scalar(&curve,G,10);
        let r=curve.neg(q);
        let mut matrix=[[0u64;4];3];
        for (i,p) in base.iter().take(3).enumerate(){
            let dx=f.sub(p.x,r.x);
            let dy=f.sub(p.y,r.y);
            matrix[i]=[dx,f.mul(p.x,dx),dy,f.neg(f.mul(p.x,dy))];
        }
        for j in 0..3{
            let pivot=(j..3).find(|&k|matrix[k][j]!=0).unwrap();
            matrix.swap(j,pivot);
            let inv=f.inv(matrix[j][j]);
            for k in j..4{matrix[j][k]=f.mul(matrix[j][k],inv);}
            for i in 0..3{
                if i==j{continue;}
                let z=matrix[i][j];
                for k in j..4{matrix[i][k]=f.sub(matrix[i][k],f.mul(z,matrix[j][k]));}
            }
        }
        let z=[matrix[0][3],matrix[1][3],matrix[2][3]];
        let m=quartic(r,f).unwrap();
        let co=m.iter().map(|v|ev(v,z,f)).collect::<Vec<_>>();
        assert_eq!(co,polynomial(&base.iter().map(|v|v.x).collect::<Vec<_>>(),f));
        let s=system(q,&base).unwrap();
        assert!(s.iter().all(|p|poly_eval(p,z,f)==0));
        let row=signed_lift(q,&base,z).unwrap();
        assert_eq!(row,vec![(0,1),(1,1),(2,1),(3,1)]);
    }
    #[test]
    fn poisoning_changes_public_equations(){
        let curve=group();let base=base4();
        let q=scalar(&curve,G,10);
        let wrong=scalar(&curve,G,17);
        assert_ne!(system(q,&base).unwrap(),system(wrong,&base).unwrap());
        assert!(system(PrimePoint::INFINITY,&base).is_err());
    }
}

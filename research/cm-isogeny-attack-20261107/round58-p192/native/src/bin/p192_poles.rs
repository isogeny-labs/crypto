#[allow(dead_code,unused_imports)]
#[path="../big.rs"] mod big;
use big::{E,K,Pt};
use num_bigint::BigUint as Z;
use num_traits::{One,Zero};
use serde_json::{json,Value};
type Jet=Vec<K>;
fn zero(e:&E)->Jet {vec![e.c(0);6]}
fn one(e:&E)->Jet {let mut z=zero(e);z[0]=e.c(1);z}
fn plus(e:&E,a:&Jet,b:&Jet)->Jet {(0..6).map(|i|e.addk(&a[i],&b[i])).collect()}
fn minus(e:&E,a:&Jet,b:&Jet)->Jet {(0..6).map(|i|e.subk(&a[i],&b[i])).collect()}
fn times(e:&E,a:&Jet,b:&Jet)->Jet {
 let mut z=zero(e);for i in 0..6 {for j in 0..6 {let k=i%3+j%3;let s=i/3+j/3;if k<3&&s<2 {z[k+3*s]=e.addk(&z[k+3*s],&e.mulk(&a[i],&b[j]))}}}z
}
fn inverse(e:&E,a:&Jet)->Jet {
 let ci=e.inverse(&a[0]);let scaled:Jet=a.iter().map(|x|e.mulk(x,&ci)).collect();let v=minus(e,&scaled,&one(e));let vv=times(e,&v,&v);
 let inv=minus(e,&plus(e,&minus(e,&one(e),&v),&vv),&times(e,&vv,&v));let inv:Jet=inv.iter().map(|x|e.mulk(x,&ci)).collect();assert_eq!(times(e,a,&inv),one(e));inv
}
fn determinant(e:&E,mut a:Vec<Vec<Jet>>)->Jet {
 let n=a.len();let mut out=one(e);
 for k in 0..n {
  if k+1==n {return times(e,&out,&a[k][k])}
  let (i,j)=(k..n).flat_map(|i|(k..n).map(move|j|(i,j))).find(|&(i,j)|a[i][j][0]!=e.c(0)).expect("constant rank deficient by more than one");
  if i!=k {a.swap(i,k);out=minus(e,&zero(e),&out)}
  if j!=k {for row in &mut a {row.swap(j,k)}out=minus(e,&zero(e),&out)}
  let pivot=a[k].clone();out=times(e,&out,&pivot[k]);let inv=inverse(e,&pivot[k]);
  for row in a.iter_mut().skip(k+1) {let c=times(e,&row[k],&inv);for j in k..n {row[j]=minus(e,&row[j],&times(e,&c,&pivot[j]))}}
 }out
}
fn invmat(e:&E,a:&[Vec<K>])->Vec<Vec<K>> {
 let n=a.len();let mut mat:Vec<_>=a.iter().enumerate().map(|(i,row)|{let mut out=row.clone();out.extend((0..n).map(|j|e.c(u32::from(i==j))));out}).collect();
 for k in 0..n {let i=(k..n).find(|&i|mat[i][k]!=e.c(0)).unwrap();mat.swap(i,k);let inv=e.inverse(&mat[k][k]);for j in 0..2*n {mat[k][j]=e.mulk(&mat[k][j],&inv)}let pivot=mat[k].clone();for i in 0..n {if i==k {continue}let c=mat[i][k].clone();for j in 0..2*n {mat[i][j]=e.subk(&mat[i][j],&e.mulk(&c,&pivot[j]))}}}
 mat.into_iter().map(|row|row[n..].to_vec()).collect()
}
fn norm(e:&E,vals:&[Jet],v:&[Vec<K>],vi:&[Vec<K>],root:bool)->Jet {
 if root {return vals.iter().fold(one(e),|a,b|times(e,&a,b))}
 let n=v.len();let mut mat=vec![vec![zero(e);n];n];
 for i in 0..n {for j in 0..n {for k in 0..n {let c=e.mulk(&vi[i][k],&v[k][j]);for l in 0..6 {mat[i][j][l]=e.addk(&mat[i][j][l],&e.mulk(&c,&vals[k][l]))}}}}
 determinant(e,mat)
}
fn mul(e:&E,n:i64,p:&Pt)->Pt {if n<0 {return e.neg(&mul(e,-n,p))}let mut n=n as u64;let mut cur=p.clone();let mut z=None;while n>0 {if n&1==1 {z=e.add(&z,&cur)}cur=e.add(&cur,&cur);n>>=1}z}
fn cval(x:&K)->Value {json!([x.0.to_string(),x.1.to_string()])}
fn hnum(e:&E,t:&K,u:&K)->K {e.addk(&e.addk(&e.mulk(u,&e.mulk(t,t)),&e.mulk(&e.addk(&(e.a.clone(),Z::zero()),&e.mulk(u,u)),t)),&e.addk(&e.mulk(&(e.a.clone(),Z::zero()),u),&e.mulk(&e.c(2),&(e.b.clone(),Z::zero()))))}
fn square_root(e:&E,x:&K)->K {
 let exp=(&e.p+Z::one())/Z::from(4u32);
 if x.1.is_zero() {let a=x.0.modpow(&exp,&e.p);if &a*&a%&e.p==x.0 {return (a,Z::zero())}let b=((&e.p-&x.0)%&e.p).modpow(&exp,&e.p);assert_eq!(&b*&b%&e.p,(&e.p-&x.0)%&e.p);return (Z::zero(),b)}
 let norm=(&x.0*&x.0+&x.1*&x.1)%&e.p;let sn=norm.modpow(&exp,&e.p);assert_eq!(&sn*&sn%&e.p,norm);let half=(&e.p+Z::one())/Z::from(2u32);
 for n in [sn.clone(),(&e.p-&sn)%&e.p] {let v=(&x.0+n)*&half%&e.p;let a=v.modpow(&exp,&e.p);if a.is_zero()||&a*&a%&e.p!=v {continue}let b=&x.1*(Z::from(2u32)*&a).modpow(&(&e.p-Z::from(2u32)),&e.p)%&e.p;let out=(a,b);assert_eq!(e.mulk(&out,&out),*x);return out}panic!("non-square extension element")
}
fn decode(e:&E,x:&K,r0:&Pt)->Pt {
 let rhs=e.addk(&e.addk(&e.mulk(&e.mulk(x,x),x),&e.mulk(&(e.a.clone(),Z::zero()),x)),&(e.b.clone(),Z::zero()));let y=square_root(e,&rhs);let mut out=vec![];
 for y in [y.clone(),e.negk(&y)] {let p=e.add(&Some((x.clone(),y)),&e.neg(r0));if let Some((xx,yy))=&p {if xx.1.is_zero()&&yy.1.is_zero() {out.push(p)}}}
 assert_eq!(out.len(),1);out.remove(0)
}
fn main() {
 let args:Vec<_>=std::env::args().collect();let dir=std::path::Path::new(args.get(1).expect("usage: p192_poles ROUND58 [--root-product]"));let root=args.iter().any(|x|x=="--root-product");
 let input=big::load(dir.join("../round52-p192/input.json"));let shift=big::load(dir.join("../round54-p192/P192-shift.json"));let e=E{p:big::z(&input["p"]),a:big::z(&input["a"]),b:big::z(&input["b"])};let g=big::point(&input["G"]);let q=big::point(&input["Q"]);
 let r0=Some(((big::z(&shift["R_6S"][0][0]),big::z(&shift["R_6S"][0][1])),(big::z(&shift["R_6S"][1][0]),big::z(&shift["R_6S"][1][1]))));assert!(e.on(&r0));assert_ne!(e.frob(&e.add(&r0,&r0)),e.add(&r0,&r0));
 let js=[1i64,3,5,7];let babies:Vec<_>=js.iter().map(|&j|mul(&e,j,&g).unwrap()).collect();let bqs:Vec<_>=js.iter().map(|&j|mul(&e,j,&q).unwrap()).collect();
 let v:Vec<Vec<K>>=babies.iter().map(|(x,_)|{let mut row=vec![e.c(1)];for k in 1..4 {row.push(e.mulk(&row[k-1],x))}row}).collect();let vi=invmat(&e,&v);let mut rows=vec![];let mut origins=0;let mut regular=0;
 for i in [16i64,32] {for sg in [1i64,-1] {
  let ag=mul(&e,sg*i,&g);let aq=mul(&e,sg*i,&q);let tt=e.add(&r0,&aq).unwrap();
  let mut uindices=vec![];for &j in &js {for bs in [1i64,-1] {uindices.push(sg*i+bs*j)}}uindices.extend([sg*i,-sg*i]);
  for uindex in uindices {
   let u=mul(&e,uindex,&g);let pp=e.neg(&u);let Some(ss)=e.add(&pp,&ag) else {origins+=1;continue};let yp=&pp.as_ref().unwrap().1;let dx=e.quotient(&ss.1,yp);
   let mut vals=vec![];
   for ((x,y),(qx,qy)) in babies.iter().zip(&bqs) {
    let d=e.subk(x,&ss.0);let dsecond=e.subk(qx,&tt.0);let dsecond=e.mulk(&dsecond,&dsecond);
    let d0=e.mulk(&e.mulk(&d,&d),&dsecond);let d1=e.negk(&e.mulk(&e.c(2),&e.mulk(&dx,&e.mulk(&d,&dsecond))));let d2=e.mulk(&e.mulk(&dx,&dx),&dsecond);
    let n1=hnum(&e,x,&ss.0);let n2=hnum(&e,qx,&tt.0);let relative=e.mulk(&e.mulk(&ss.1,&tt.1),&e.mulk(y,qy));let n0=e.mulk(&e.c(2),&e.addk(&e.mulk(&n1,&n2),&e.mulk(&e.c(4),&relative)));
    vals.push(vec![d0,d1,d2,n0,e.c(0),e.c(0)]);
   }
   let d=norm(&e,&vals,&v,&vi,root);if d[0]!=e.c(0) {assert_eq!(pp,ag);regular+=1;continue}assert_eq!(d[1],e.c(0));assert_ne!(d[2],e.c(0));
   let raw=e.quotient(&d[3],&d[2]);let scale=e.mulk(&e.c(4),&e.mulk(yp,yp));let mark=e.quotient(&raw,&scale);let decoded=decode(&e,&mark,&r0);let wanted=mul(&e,uindex,&q);assert_eq!(decoded,wanted);
   assert_eq!(raw,e.mulk(&scale,&e.add(&r0,&wanted).unwrap().0));assert!(e.on(&decoded));
   rows.push(json!({"giant":i,"sign":sg,"U_index":uindex,"U":big::code(&u),"det":d.iter().map(cval).collect::<Vec<_>>(),"raw_coefficient":cval(&raw),"marked_x":cval(&mark),"recovered":big::code(&decoded)}));
  }
 }}
 assert_eq!(rows.len(),32);assert_eq!(origins,4);assert_eq!(regular,4);
 println!("{}",serde_json::to_string(&json!({"all_checks_pass":true,"p":input["p"],"r":input["r"],"G":input["G"],"Q":input["Q"],"R0":big::code(&r0),"baby_indices":js,"giants":[16,32],"pole_blocks":32,"origin_chart_skips":origins,"regular_doubling_blocks":regular,"rows":rows,"scope":"32 selected P192 pole blocks and point actions; no full trace; unsigned"})).unwrap());
}

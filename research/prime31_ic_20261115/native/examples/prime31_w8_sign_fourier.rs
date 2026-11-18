//! Classical 70-minor/8-bit Walsh transform for all signed 8x8 RR9 determinants
//! on supplied weighted Prime31 slot x coordinates. Not an unknown-x extractor.
use prime31_algebraic_ic::{check_curve, Arithmetic, Point, PRIME31};
use serde_json::{json,Value};
use std::collections::HashSet;
use std::time::Instant;
const P:u64=2_147_483_647;
const W:[u64;8]=[1,3,5,7,11,13,17,19];
const G:Point=Some((625_037_715,2_100_627_251));
#[derive(Clone,Copy,Default)]
struct Ops{mul:u64,inv:u64,add:u64}
fn plus(a:u64,b:u64,o:&mut Ops)->u64{o.add+=1;let s=a+b;if s>=P{s-P}else{s}}
fn minus(a:u64,b:u64,o:&mut Ops)->u64{o.add+=1;if a>=b{a-b}else{a+P-b}}
fn times(a:u64,b:u64,o:&mut Ops)->u64{o.mul+=1;((a as u128*b as u128)%P as u128) as u64}
fn power(mut a:u64,mut n:u64,o:&mut Ops)->u64{
 let mut b=1;
 while n>0{if n&1==1{b=times(b,a,o)} n>>=1;if n>0{a=times(a,a,o)}}
 b
}
fn invert(a:u64,o:&mut Ops)->u64{assert_ne!(a,0);o.inv+=1;power(a,P-2,o)}
fn det(mut m:[[u64;8];8],o:&mut Ops)->u64{
 let mut d=1;
 for k in 0..8{
  let Some(j)=(k..8).find(|&j|m[j][k]!=0)else{return 0};
  if j!=k{m.swap(j,k);d=minus(0,d,o)}
  let pivot=m[k][k];d=times(d,pivot,o);let pi=invert(pivot,o);
  for i in k+1..8{
   if m[i][k]==0{continue}
   let scale=times(m[i][k],pi,o);
   for h in k+1..8{let v=times(scale,m[k][h],o);m[i][h]=minus(m[i][h],v,o)}
   m[i][k]=0;
  }
 }
 d
}
fn direct(points:&[(u64,u64);8],r:(u64,u64))->([u64;256],Ops){
 let mut o=Ops::default();let mut ans=[0;256];
 for (signs,value) in ans.iter_mut().enumerate(){
  let mut m=[[0;8];8];
  for (i,&(x,y0)) in points.iter().enumerate(){
   let y=if signs&(1<<i)!=0{minus(0,y0,&mut o)}else{y0};
   let a=minus(x,r.0,&mut o);let b=plus(y,r.1,&mut o);
   let mut xp=1;
   for j in 0..4{
    m[i][j]=times(a,xp,&mut o);
    m[i][4+j]=times(b,xp,&mut o);
    xp=times(xp,x,&mut o);
   }
  }
  *value=det(m,&mut o)
 }
 (ans,o)
}
fn fourier(points:&[(u64,u64);8],r:(u64,u64))->([u64;256],Ops){
 let mut o=Ops::default();
 let xs:[u64;8]=std::array::from_fn(|i|points[i].0);
 let ys:[u64;8]=std::array::from_fn(|i|points[i].1);
 let mut c=[0u64;256];let mut nterms=0;
 for mask in 0usize..256{
  if mask.count_ones()!=4{continue}
  nterms+=1;
  let(mut l,mut h)=([0usize;4],[0usize;4]);
  let(mut nl,mut nh,mut parity)=(0,0,0);
  for i in 0..8{
   if mask&(1<<i)!=0{l[nl]=i;nl+=1;parity^=i&1}
   else{h[nh]=i;nh+=1}
  }
  let mut t=if parity==1{minus(0,1,&mut o)}else{1};
  for inds in [&l,&h]{
   for i in 0..4{for j in i+1..4{
    let diff=minus(xs[inds[j]],xs[inds[i]],&mut o);
    t=times(t,diff,&mut o)
   }}
  }
  for &i in &l{
   let diff=minus(xs[i],r.0,&mut o);t=times(t,diff,&mut o)
  }
  for small in 0usize..16{
   let mut mon=t;let mut support=0;
   for(j,&i)in h.iter().enumerate(){
    if small&(1<<j)!=0{support|=1<<i;mon=times(mon,ys[i],&mut o)}
    else{mon=times(mon,r.1,&mut o)}
   }
   c[support]=plus(c[support],mon,&mut o)
  }
 }
 assert_eq!(nterms,70);
 for bit in 0..8{
  for mask in 0usize..256{
   if mask&(1<<bit)!=0{continue}
   let other=mask|(1<<bit);let a=c[mask];let b=c[other];
   c[mask]=plus(a,b,&mut o);c[other]=minus(a,b,&mut o)
  }
 }
 (c,o)
}
fn base(ar:&mut Arithmetic)->Vec<(u64,u64)>{
 let mut v=Vec::new();
 for x in 0..10_000{
  if let Some(y)=ar.sqrt(ar.rhs(PRIME31,x),P){
   if y!=0{let q=(x,y.min(P-y));assert!(ar.on_curve(PRIME31,Some(q)));v.push(q)}
  }
  if v.len()==31{break}
 }
 assert_eq!(v.len(),31);v
}
fn slots(ar:&mut Arithmetic,b:&[(u64,u64)])->Vec<Vec<(u64,u64)>>{
 let mut seen=HashSet::new();
 let v:Vec<_>=W.iter().map(|&w|b.iter().map(|&q|{
  let z=ar.scalar(PRIME31,w,Some(q)).unwrap();
  assert!(seen.insert(z.0));z
 }).collect::<Vec<_>>()).collect();
 assert_eq!(seen.len(),248);v
}
fn negate(q:(u64,u64))->(u64,u64){(q.0,if q.1==0{0}else{P-q.1})}
fn sum(ar:&mut Arithmetic,pts:&[(u64,u64);8],mask:usize)->Point{
 pts.iter().enumerate().fold(None,|acc,(i,&q)|
  ar.add(PRIME31,acc,Some(if mask&(1<<i)!=0{negate(q)}else{q})))
}
fn checksum(a:&[u64;256])->u64{
 a.iter().enumerate().fold(0u64,|s,(i,&x)|{
  (s+(((i as u128+1)*x as u128)%P as u128)as u64)%P
 })
}
fn run()->Value{
 let mut ar=Arithmetic::default();check_curve(PRIME31,&mut ar).unwrap();
 let b=base(&mut ar);let slot=slots(&mut ar,&b);
 let mut rows=Vec::new();let(mut cd,mut cf)=(Ops::default(),Ops::default());
 let(mut td,mut tf)=(0u128,0u128);let mut nz=0u64;
 for t in 0usize..16{
  let pts:[(u64,u64);8]=std::array::from_fn(|j|slot[j][(3*t+7*j+j*j)%31]);
  let sign=(0..8).fold(0usize,|acc,j|acc|if(t+3*j)%4<2{0}else{1<<j});
  let r=sum(&mut ar,&pts,sign).unwrap();
  let shifted=ar.add(PRIME31,Some(r),G).unwrap();
  for (arm,target) in [("planted",r),("shifted_negative",shifted)]{
   let ((a,oa,ta),(z,oz,tz))=if t%2==0{
    let t0=Instant::now();let (a,oa)=direct(&pts,target);let ta=t0.elapsed().as_nanos();
    let t1=Instant::now();let (z,oz)=fourier(&pts,target);let tz=t1.elapsed().as_nanos();
    ((a,oa,ta),(z,oz,tz))
   }else{
    let t0=Instant::now();let (z,oz)=fourier(&pts,target);let tz=t0.elapsed().as_nanos();
    let t1=Instant::now();let (a,oa)=direct(&pts,target);let ta=t1.elapsed().as_nanos();
    ((a,oa,ta),(z,oz,tz))
   };
   assert_eq!(a,z,"256 exact determinant residues");
   let zeros:Vec<_>=(0..256).filter(|&i|z[i]==0).collect();
   assert_eq!(zeros,if arm=="planted"{vec![sign]}else{vec![]});
   for (i,&d)in z.iter().enumerate(){
    assert_eq!(sum(&mut ar,&pts,i)==Some(target),d==0)
   }
   cd.mul+=oa.mul;cd.inv+=oa.inv;cd.add+=oa.add;
   cf.mul+=oz.mul;cf.inv+=oz.inv;cf.add+=oz.add;
   td+=ta;tf+=tz;nz+=zeros.len()as u64;
   rows.push(json!({
    "fixture":t,"arm":arm,"points":pts,"target":target,
    "planted_sign_mask":if arm=="planted"{Some(sign)}else{None},
    "zero_sign_masks":zeros,"residue_checksum":checksum(&z),
    "sample_residues":[z[0],z[1],z[63],z[127],z[255]],
    "direct_fp_mul":oa.mul,"direct_fp_inv":oa.inv,"direct_fp_add":oa.add,
    "fourier_fp_mul":oz.mul,"fourier_fp_inv":oz.inv,"fourier_fp_add":oz.add,
    "direct_ns":ta.to_string(),"fourier_ns":tz.to_string()
   }));
  }
 }
 json!({
  "schema":"prime31.w8.sign-fourier.v1","p":P,"a":PRIME31.a,"b":PRIME31.b,
  "n":PRIME31.n,"cofactor":1,"weights":W,
  "base_x":b.iter().map(|p|p.0).collect::<Vec<_>>(),
  "given_abscissae":true,"generic_ec_tables_in_candidate":false,
  "unknown_x_index_extractions":0,"unknown_public_ecdlp_solves":0,
  "source_crate_sha":"1df714645aa683cf975bc76fcacb9842cb4780fd",
  "records":rows,"total_predicates_replayed":8192,"positive_sign_vectors":nz,
  "direct_total_fp_mul":cd.mul,"direct_total_fp_inv":cd.inv,"direct_total_fp_add":cd.add,
  "fourier_total_fp_mul":cf.mul,"fourier_total_fp_inv":cf.inv,"fourier_total_fp_add":cf.add,
  "direct_total_ns":td.to_string(),"fourier_total_ns":tf.to_string(),
  "independent_signed_pcc":"unverified"
 })
}
fn main(){println!("{}",serde_json::to_string_pretty(&run()).unwrap())}
#[cfg(test)]
mod tests{
 use super::*;
 #[test]fn all_8192_original_curve_signed_predicates(){
  let j=run();
  assert_eq!(j["total_predicates_replayed"],8192);
  assert_eq!(j["positive_sign_vectors"],16);
  assert_eq!(j["fourier_total_fp_inv"],0);
  assert_eq!(j["unknown_public_ecdlp_solves"],0);
 }
}

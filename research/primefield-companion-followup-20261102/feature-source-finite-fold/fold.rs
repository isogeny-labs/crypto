//! Finite-subgroup Ward reduction; no derivative seed or unknown scalar input.
#[path="prior.rs"]mod prior;
#[path="../feature-native-value-division/lib.rs"]mod division;
use std::collections::BTreeMap;
#[derive(Clone,Debug)]struct Fold {r:u64,q:u64,negative:bool,b_power:u64,terms:BTreeMap<u64,u64>}
impl Fold{
 fn new(r:u64,q:u64,negative:bool,terms:&[(u64,i128)])->Self {
  assert!(r>=7&&r%2==1&&r<=1_000_001&&(q==2||q==4));let mut z=Self{r,q,negative,b_power:0,terms:BTreeMap::new()};let(mut aa,mut bb)=(0i128,0i128);let r=r as i128;let h=(r-1)/2;
  for &(n,e) in terms{assert!(n<=2_000_002&&e>=0&&e<q as i128);let mut v=n as i128%r;assert_ne!(v,0);if v>h{v-=r}let k=(n as i128-v)/r;
   if v<0&&e%2==1{z.negative^=true}let j=v.unsigned_abs()as u64;let x=z.terms.entry(j).or_default();*x=(*x+e as u64)%q;
   aa=(aa+e*v*k).rem_euclid(q as i128);bb=(bb+e*k*k).rem_euclid(q as i128);
  }
  // r inverse modulo2 or4 equals r modulo that modulus, since r is odd.
  z.b_power=(bb+2*(r%(q as i128))*aa).rem_euclid(q as i128)as u64;z.terms.retain(|n,e|*n!=1&&*e!=0);z
 }
 fn compile(r:u64,q:u64)->Self{let(neg,t)=prior::program(r,q as i128);Self::new(r,q,neg,&t)}
 fn interval(&self)->Option<(u64,u64)>{let lo=*self.terms.keys().next()?;let hi=*self.terms.keys().next_back()?;if self.terms.len()as u64==hi-lo+1&&self.terms.values().all(|e|*e==1){Some((lo,hi))}else{None}}
 fn value(&self,p:u64,a:u64,b:u64,t:[u64;2])->u64{
  assert!(p>3&&p<=u64::MAX/2&&(p-1)%self.q==0&&((p-1)/self.q)%2==1&&p!=self.r);
  let mut d=division::ValueDivision::new(p,a,b,t[0],t[1]).unwrap();assert_eq!(d.value(self.r),0,"r torsion required");
  let mut z=if self.negative{p-1}else{1};for(&n,&e)in &self.terms{let w=d.value(n);assert_ne!(w,0);z=mul(z,pow(w,e,p),p)}
  if self.b_power!=0{let x=d.value(self.r+1);let y=d.value(self.r+2);let two=d.value(2);assert!(x!=0&&y!=0&&two!=0);let beta=mul(mul(mul(x,x,p),two,p),inv(y,p),p);z=mul(z,pow(beta,self.b_power,p),p)}
  pow(z,(p-1)/self.q,p)
 }
}
// Constant-size constructor: no exponent-map construction or subgroup enumeration.
#[derive(Debug)]struct Compact {r:u64,q:u64,start:u64,end:u64,b_power:u64}
impl Compact {
 fn new(r:u64,q:u64)->Self{assert!(r>=7&&r%2==1&&r<=1_000_001&&(q==2||q==4));let h=(r-1)/2;let sum=(h as u128)*(h as u128+1)*(2*h as u128+1)/6;Self{r,q,start:2,end:h,b_power:((q as u128-sum%(q as u128))%(q as u128))as u64}}
 fn value(&self,p:u64,a:u64,b:u64,t:[u64;2])->u64{
  assert!(p>3&&p<=u64::MAX/2&&(p-1)%self.q==0&&((p-1)/self.q)%2==1&&p!=self.r);
  let mut d=division::ValueDivision::new(p,a,b,t[0],t[1]).unwrap();assert_eq!(d.value(self.r),0);let mut z=1;
  for n in self.start..=self.end{let w=d.value(n);assert_ne!(w,0);z=mul(z,w,p)}
  if self.b_power!=0{let x=d.value(self.r+1);let y=d.value(self.r+2);let two=d.value(2);assert!(x!=0&&y!=0&&two!=0);z=mul(z,pow(mul(mul(mul(x,x,p),two,p),inv(y,p),p),self.b_power,p),p)}pow(z,(p-1)/self.q,p)
 }
}
fn mul(a:u64,b:u64,p:u64)->u64{((a as u128*b as u128)%p as u128)as u64}
fn pow(mut a:u64,mut n:u64,p:u64)->u64{let mut z=1;while n>0{if n&1==1{z=mul(z,a,p)}a=mul(a,a,p);n>>=1}z}
fn inv(a:u64,p:u64)->u64{assert_ne!(a,0);let z=pow(a,p-2,p);assert_eq!(mul(a,z,p),1);z}
fn main(){for(r,q)in[(7,4),(13,2),(479,4)]{let f=Fold::compile(r,q);println!("r={r} q={q} negative={} beta_power={} terms={} interval={:?}",f.negative,f.b_power,f.terms.len(),f.interval());}}
#[cfg(test)]#[path="../feature-native-source/lib.rs"]mod original;
#[cfg(test)]#[path="../feature-orbit-local-terms/local.rs"]mod ec;
#[cfg(test)]mod fold_tests{
 use super::*;
 #[test]fn source_and_corruptions(){let(mut sign_rejected,mut beta_rejected)=(0,0);
  for(p,a,b,r,base,q)in[(5,2,1,7,[0,1],4),(7,0,3,13,[1,2],2),(509,2,10,479,[3,68],4)]{
   let f=Fold::compile(r,q);assert_eq!(f.interval(),Some((2,(r-1)/2)));assert!(!f.negative);let c=ec::Curve::new(p,a,b);let base=ec::Point{x:base[0],y:base[1]};let source=original::Source::new(p,a,b,r,[base.x,base.y]).unwrap();let mut t=Some(base);
   for i in 1..r {let pt=t.unwrap();if r<20||[1,2,3,7,16,64,239,478].contains(&i){let pt=[pt.x,pt.y];let expected=pow(source.raw(pt).unwrap().value,(p-1)/q,p);assert_eq!(f.value(p,a,b,pt),expected);assert_eq!(Compact::new(r,q).value(p,a,b,pt),expected);let mut wrong=f.clone();wrong.negative^=true;if wrong.value(p,a,b,pt)!=expected{sign_rejected+=1}if f.b_power!=0{wrong=f.clone();wrong.b_power=0;if wrong.value(p,a,b,pt)!=expected{beta_rejected+=1}}}t=c.plus(t,Some(base));}
  }assert_eq!(sign_rejected,26);assert!(beta_rejected>0);println!("sign_corruptions_rejected={sign_rejected} beta_omissions_rejected={beta_rejected}");
 }
 #[test]fn ward_indices_including_characteristic(){
  for(p,a,b,r,t,q)in[(5,2,1,7,[0,1],4),(7,0,3,13,[1,2],2),(509,2,10,479,[3,68],4)]{
   let mut d=division::ValueDivision::new(p,a,b,t[0],t[1]).unwrap();for n in 1..=2*r+3{if n%r==0{continue}let f=Fold::new(r,q,false,&[(n,1)]);assert_eq!(f.value(p,a,b,t),pow(d.value(n),(p-1)/q,p),"p={p} n={n}");}
  }
 }
 #[test]fn direct_constructor_matches_expanded(){for r in (7..=101).step_by(2){for q in [2,4]{let f=Fold::compile(r,q);let c=Compact::new(r,q);assert_eq!(f.interval(),Some((c.start,c.end)));assert_eq!(f.b_power,c.b_power);assert!(!f.negative);}}}
 #[test]fn domain_guards(){assert!(std::panic::catch_unwind(||Fold::new(7,4,false,&[(7,1)])).is_err());assert!(std::panic::catch_unwind(||Fold::new(7,8,false,&[(1,1)])).is_err());let f=Fold::compile(7,4);assert!(std::panic::catch_unwind(||f.value(5,0,1,[0,1])).is_err());}
}

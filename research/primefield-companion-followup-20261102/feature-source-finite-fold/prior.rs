//! Exact signed division-polynomial expression for the existing geometry lift.
use std::collections::BTreeMap;
#[derive(Clone,Debug,PartialEq,Eq,Default)] struct M { negative:bool, exponents:BTreeMap<u64,i128> }
impl M {
 fn constant(negative:bool)->Self {Self{negative,..Self::default()}}
 fn psi(n:u64)->Self {assert!(n>0);Self{negative:false,exponents:BTreeMap::from([(n,1)])}}
 fn times(&self,b:&Self,k:i128)->Self {let mut z=self.clone();if k.rem_euclid(2)==1 {z.negative^=b.negative}for(&n,&e)in &b.exponents{let v=z.exponents.entry(n).or_default();*v=v.checked_add(e.checked_mul(k).unwrap()).unwrap();}z.exponents.retain(|_,v|*v!=0);z}
 fn power(&self,k:i128)->Self {Self::default().times(self,k)}
 // x([a]Q)-x([b]Q) = -psi_(a+b) psi_(a-b)/(psi_a² psi_b²).
 fn difference(a:u64,b:u64)->Self {assert!(a>0&&b>0&&a!=b);Self::constant(a>b).times(&Self::psi(a.checked_add(b).unwrap()),1).times(&Self::psi(a.abs_diff(b)),1).times(&Self::psi(a),-2).times(&Self::psi(b),-2)}
 fn residue(&self,q:i128)->Self {assert!(q==2||q==4);let mut z=self.clone();for e in z.exponents.values_mut(){*e=e.rem_euclid(q)}z.exponents.retain(|n,e|*n!=1&&*e!=0);z}
 // Divisor is sum e_n [n]^*O - (sum e_n n²)[O].
 fn pole_weight(&self)->i128 {self.exponents.iter().map(|(&n,&e)|e*(n as i128)*(n as i128)).sum()}
}
fn partition(r:u64)->[u64;4]{assert!(r>=7&&r%2==1&&r<=1_000_001);let h=(r-1)/2;let target=(h+1)/2;let mut w=1;while w*w<target{w+=1}w=w.min(h/2);let rem=h%(2*w);[w,h/(2*w),rem/2,rem%2]}
fn geometry(r:u64)->M{
 let[w,k,s,e]=partition(r);let n=r/2+1;
 let mut previous=M::constant(true);let mut current=M::default();let(mut bp,mut ap,mut prefix)=(M::default(),M::default(),M::default());let mut babies=Vec::new();let mut result=M::default();
 for j in 0..w {let t=(n+j)%r;babies.push(t);bp=bp.times(&current,1);if j<s{prefix=prefix.times(&current,1)}let next=M::difference(1,t).times(&current,2).times(&previous,-1);previous=current;current=next;}
 previous.negative^=true;
 for i in 0..k {let t=(n+w+2*w*i)%r;ap=ap.times(&current,1);for &b in &babies{result=result.times(&M::difference(t,b),1)}if i+1<k{let next=M::difference(2*w,t).times(&current,2).times(&previous,-1);previous=current;current=next;}}
 if s>0{for &b in &babies[..s as usize]{result=result.times(&M::difference(s+e,b),1)}}
 result.negative^=(w*k+s+e)%2==1;result.times(&ap,(2*w)as i128).times(&bp,(2*k)as i128).times(&prefix,2)
}
fn projected_program(r:u64,q:i128)->M{
 let[w,k,s,e]=partition(r);let n=r/2+1;let g=|x:u64|{let x=x as i128;x*(x-1)*(x+1)/3};
 geometry(r).times(&M::psi(n),(4*w*k+2*s+e)as i128).times(&M::psi(2*w),2*w as i128*g(k)).times(&M::psi(if s==0{1}else{s+e}),(2*s)as i128).residue(q)
}
#[path="../feature-native-value-division/lib.rs"]mod division;
fn mul(a:u64,b:u64,p:u64)->u64 {((a as u128*b as u128)%p as u128)as u64}
fn pow(mut a:u64,mut n:u64,p:u64)->u64 {let mut v=1;while n>0{if n&1==1{v=mul(v,a,p)}a=mul(a,a,p);n>>=1;}v}
fn evaluate(m:&M,p:u64,a:u64,b:u64,point:[u64;2],q:u64)->u64{
 assert!((q==2||q==4)&&(p-1)%q==0&&((p-1)/q)%2==1);
 let mut d=division::ValueDivision::new(p,a,b,point[0],point[1]).unwrap();let mut v=if m.negative{p-1}else{1};
 for(&n,&e)in &m.exponents{assert!(e>=0);let z=d.value(n);assert_ne!(z,0);v=mul(v,pow(z,e as u64,p),p)}pow(v,(p-1)/q,p)
}
fn prior_main(){for(r,q)in[(7,4),(13,2),(479,4)]{let m=projected_program(r,q);println!("r={r} q={q} negative={} terms={} exponents={:?}",m.negative,m.exponents.len(),m.exponents);}}
#[cfg(test)]#[path="../feature-native-source/lib.rs"]mod original;
#[cfg(test)]#[path="../feature-orbit-local-terms/local.rs"]mod ec;
#[cfg(test)]mod bridge_tests{
 use super::*;
 #[test]fn exact_order_seven_lift(){let g=geometry(7);assert_eq!(g,M{negative:true,exponents:BTreeMap::from([(1,-3),(3,2),(4,-6),(9,1)])});assert_eq!(g.pole_weight(),0);let m=projected_program(7,4);assert_eq!(m,M{negative:true,exponents:BTreeMap::from([(3,2),(4,3),(9,1)])});}
 #[test]fn actual_source_values(){for(p,a,b,r,base,q)in[(5,2,1,7,[0,1],4),(7,0,3,13,[1,2],2),(509,2,10,479,[3,68],4)]{let m=projected_program(r,q);let c=ec::Curve::new(p,a,b);let base=ec::Point{x:base[0],y:base[1]};let source=original::Source::new(p,a,b,r,[base.x,base.y]).unwrap();let mut t=Some(base);for i in 1..r{let v=t.unwrap();if r<20||[1,2,3,7,16,64,239,478].contains(&i){let raw=source.raw([v.x,v.y]).unwrap().value;assert_eq!(evaluate(&m,p,a,b,[v.x,v.y],q as u64),pow(raw,(p-1)/(q as u64),p),"r={r}, i={i}");}t=c.plus(t,Some(base));}assert_eq!(t,None);}}
 #[test]fn support_and_lift_scope(){let g=geometry(7);let at_order_nine:i128=g.exponents.iter().filter(|(n,_)|**n%9==0).map(|(_,e)|*e).sum();assert_eq!(at_order_nine,1);assert_ne!(M::difference(5,4),M::difference(2,3));for r in [7,13,19,31,43,73,479]{assert_eq!(geometry(r).pole_weight(),0)}}
}

pub fn program(r:u64,q:i128)->(bool,Vec<(u64,i128)>){let m=projected_program(r,q);(m.negative,m.exponents.into_iter().collect())}

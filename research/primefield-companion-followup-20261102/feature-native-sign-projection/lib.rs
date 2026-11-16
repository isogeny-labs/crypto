//! Multiplicative projection only. Prime modulus is a caller precondition.
//! A projected sign candidate does not certify that the original ratio was ±1.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Image { value: u64, p: u64 }
#[derive(Clone, Copy, Debug)]
pub struct Projection { p: u64, pub order: u64, pub odd_part: u64 }
fn mul(a:u64,b:u64,p:u64)->u64 { ((a as u128*b as u128)%p as u128) as u64 }
fn pow(mut a:u64,mut e:u64,p:u64)->u64 { let mut out=1; while e>0 {if e&1==1 {out=mul(out,a,p);} a=mul(a,a,p);e>>=1;} out }
impl Projection {
    /// Does not test primality. Only odd caller-certified primes are supported.
    pub fn new_prime(p:u64)->Option<Self> {
        if p<=3 || p&1==0 {return None;}
        let order=1u64<<(p-1).trailing_zeros();
        Some(Self{p,order,odd_part:(p-1)/order})
    }
    pub fn project(&self,a:u64)->Option<Image> {
        if a==0 || a>=self.p {return None;}
        Some(Image{value:pow(a,self.odd_part,self.p),p:self.p})
    }
    fn accepts(&self,a:Image)->bool {a.p==self.p}
    pub fn product(&self,a:Image,b:Image)->Option<Image> {
        if !self.accepts(a)||!self.accepts(b) {return None;}
        Some(Image{value:mul(a.value,b.value,self.p),p:self.p})
    }
    pub fn power(&self,a:Image,e:u64)->Option<Image> {
        if !self.accepts(a) {return None;}
        Some(Image{value:pow(a.value,e%self.order,self.p),p:self.p})
    }
    pub fn quotient(&self,a:Image,b:Image)->Option<Image> {
        self.product(a,self.power(b,self.order-1)?)
    }
    /// Image of delta * anchor^2 * current^2 / previous.
    pub fn step(&self,delta:Image,anchor:Image,current:Image,previous:Image)->Option<Image> {
        let square=self.power(self.product(anchor,current)?,2)?;
        self.quotient(self.product(delta,square)?,previous)
    }
    /// Correct sign only under an externally established original-ratio ±1 promise.
    pub fn sign_candidate(&self,a:Image)->Option<i8> {
        if !self.accepts(a) {return None;}
        if a.value==1 {Some(1)} else if a.value==self.p-1 {Some(-1)} else {None}
    }
}
#[cfg(test)]mod tests {
 use super::*;
 #[test]fn nonzero_domain_and_cross_field(){
  assert!(Projection::new_prime(2).is_none());assert!(Projection::new_prime(6).is_none());
  let a=Projection::new_prime(7).unwrap();let b=Projection::new_prime(13).unwrap();
  assert!(a.project(0).is_none());assert!(a.project(7).is_none());
  assert!(a.product(a.project(2).unwrap(),b.project(2).unwrap()).is_none());
 }
 #[test]fn homomorphism_and_signs(){
  for p in [5,7,13,17,509] {let c=Projection::new_prime(p).unwrap();
   assert_eq!(c.sign_candidate(c.project(1).unwrap()),Some(1));
   assert_eq!(c.sign_candidate(c.project(p-1).unwrap()),Some(-1));
   for a in 1..p {for b in 1..p {
    assert_eq!(c.product(c.project(a).unwrap(),c.project(b).unwrap()),c.project(mul(a,b,p)));
   }}
  }
 }
 #[test]fn recurrence_commutes(){
  for p in [5,7,13,17,509] {let c=Projection::new_prime(p).unwrap();
   for a in 1..p {for b in 1..p {
    let delta=1+(a+b)%(p-1);let anchor=1+(a*3+b)%(p-1);
    let raw=mul(mul(delta,pow(anchor,2,p),p),mul(pow(a,2,p),pow(b,p-2,p),p),p);
    assert_eq!(c.step(c.project(delta).unwrap(),c.project(anchor).unwrap(),c.project(a).unwrap(),c.project(b).unwrap()),c.project(raw));
   }}
  }
 }
 #[test]fn projection_is_not_a_validity_check(){
  let c=Projection::new_prime(7).unwrap();
  assert_ne!(2,1);assert_ne!(2,6);
  assert_eq!(c.sign_candidate(c.project(2).unwrap()),Some(1)); // 2^3=1
  let c=Projection::new_prime(13).unwrap();
  assert_eq!(pow(12,6,13),1); // quadratic character loses minus sign
  assert_eq!(c.sign_candidate(c.project(12).unwrap()),Some(-1));
 }
 #[test]fn minimal_power_quotient(){
  for p in [5,7,13,17,509] {let c=Projection::new_prime(p).unwrap();
   for d in 1..p {let preserves=pow(p-1,d,p)==p-1;
    let mut a=p-1;let mut b=d;while b!=0{let r=a%b;a=b;b=r;}
    let image_order=(p-1)/a;
    if preserves {assert!(image_order>=c.order);assert_eq!(image_order%c.order,0);}
   }
  }
 }
}

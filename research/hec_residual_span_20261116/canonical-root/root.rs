use hec_native_ic::{fp::Fp,instance::prime_factors};
fn gcd(mut a:u64,mut b:u64)->u64{while b!=0{let r=a%b;a=b;b=r;}a}
// Precondition: w has exact order n in the prime field. No global generator needed.
pub fn from_primitive(f:&Fp,n:u64,w:u64)->u64{assert!(n>1&&(f.p-1)%n==0);let mut x=1;let mut best=f.p;for e in 1..=n{x=f.mul(x,w);if gcd(e,n)==1{best=best.min(x);}}assert_eq!(x,1);assert!(best>1&&best<f.p);best}
pub fn power_map(f:&Fp,n:u64)->(u64,u64){assert!(n>1&&(f.p-1)%n==0);let factors=prime_factors(n);let mut trials=0;for a in 2..f.p{trials+=1;let w=f.pow(a,(f.p-1)/n);if factors.iter().all(|&l|f.pow(w,n/l)!=1){return(from_primitive(f,n,w),trials)}}unreachable!("prime field multiplicative group has a primitive n-th root")}
pub fn reference(f:&Fp,n:u64)->u64{let factors=prime_factors(n);(2..f.p).find(|&z|f.pow(z,n)==1&&factors.iter().all(|&l|f.pow(z,n/l)!=1)).unwrap()}

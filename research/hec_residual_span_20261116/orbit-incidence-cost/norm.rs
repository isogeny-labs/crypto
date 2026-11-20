use hec_native_ic::{fp::Fp,poly::Poly};
fn power(a:Poly,n:usize,f:&Fp)->Poly{let mut p=Poly::one();for _ in 0..n{p=p.mul(&a,f)}p}
pub fn norm(a:Poly,b:Poly,c:Poly,r:u64,f:&Fp)->Poly{let term=power(b,5,f).sub(&a.mul(&c,f).mul(&power(b,3,f),f).scale(5,f),f).add(&power(a,2,f).mul(&power(c,2,f),f).mul(&b,f).scale(5,f),f);power(c,5,f).add(&term.scale(r,f),f).add(&power(a,5,f).scale(f.mul(r,r),f),f)}

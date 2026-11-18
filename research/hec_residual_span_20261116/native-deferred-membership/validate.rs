use hec_native_ic::jac::{Curve,Div};
pub fn eligible(c:&Curve,d:&Div,t:&Div,ell:u64)->bool{
 if ell<=2||ell>=(1u64<<62){return false;}if !(2..).take_while(|q|q*q<=ell).all(|q|ell%q!=0){return false;}
 if !c.is_valid(d)||!c.is_valid(t)||c.is_zero(d)||!c.is_zero(&c.mul(d,ell)){return false;}
 let mut root=1u128;while root*root<c.fp.p as u128{root+=1;}let upper=(root+1).pow((2*c.g)as u32);(ell as u128)*(ell as u128)>upper
}

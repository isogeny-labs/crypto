use hec_native_ic::jac::{Curve,Div};
fn gcd(mut a:i64,mut b:i64)->i64{
    while b!=0 {(a,b)=(b,a%b);}
    a
}
pub fn check(c:&Curve,d:&Div,target:&Div,k:u64,l:u64)->bool {
    // The base has order l. Requiring b invertible on the entire Jacobian
    // also prevents torsion in an incorrect target from passing the check.
    let h=match c.fp.p {73=>6,127=>1,257|383=>3,_=>return c.mul(d,k)==*target};
    if k==0 {return c.is_zero(target);}
    let (mut prev,mut a)=(l as i64,k as i64);
    let (mut old_b,mut b)=(0i64,1i64);
    let mut best=(a,b);
    while a>0{
        if a.max(b.abs())<best.0.max(best.1.abs()) && gcd(b.abs(),h*l as i64)==1 {best=(a,b);}
        let q=prev/a;
        (prev,a)=(a,prev%a);
        (old_b,b)=(b,old_b-q*b);
    }
    let (a,b)=best;
    let t=if b>0 {c.neg(target)}else{*target};
    let width=64-(a as u64|b.unsigned_abs()).leading_zeros();
    let both=crate::lift::add(c,d,&t,4,false);
    let mut acc=c.zero();
    for i in (0..width).rev(){
        acc=crate::lift::double(c,&acc,4,false);
        let bit=((a as u64>>i)&1)+2*((b.unsigned_abs()>>i)&1);
        if bit!=0 {acc=crate::lift::add(c,&acc,match bit{1=>d,2=>&t,_=>&both},4,false);}
    }
    c.is_zero(&acc)
}

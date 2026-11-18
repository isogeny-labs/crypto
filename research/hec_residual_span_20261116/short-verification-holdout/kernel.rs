use hec_native_ic::{jac::{Curve,Div},poly::Poly};
pub fn evaluate(c:&Curve,d:&Div,z:u64,a:&[i128])->Div{
 let n=a.len();let mut powers=vec![1;n];for j in 1..n{powers[j]=c.fp.mul(powers[j-1],z);}
 let degree=d.u.deg() as usize;let mut terms=vec![];
 for(j,&v)in a.iter().enumerate(){if v==0{continue;}let mut q=*d;
 if j>0{let mut u=vec![0;degree+1];let mut vv=vec![0;d.v.n];for i in 0..degree{u[i]=c.fp.mul(d.u.c[i],powers[(j*(degree-i))%n]);}u[degree]=1;for i in 0..d.v.n{vv[i]=if i==0{d.v.c[i]}else{c.fp.mul(d.v.c[i],powers[(n-(j*i)%n)%n])};}q=Div{u:Poly::from_coeffs(&u),v:Poly::from_coeffs(&vv)};}
 if v<0{q=c.neg(&q);}terms.push((q,u64::try_from(v.abs()).unwrap()));}
 let bits=terms.iter().map(|q|64-q.1.leading_zeros()).max().unwrap_or(0);let mut out=c.zero();for bit in(0..bits).rev(){if !c.is_zero(&out){out=c.add(&out,&out);}for(q,v)in &terms{if(v>>bit)&1==1{out=if q.u.deg()==1{c.add_place(&out,c.fp.neg(q.u.c[0]),q.v.c[0])}else{c.add(&out,q)}}}}out
}

use crate::packet::{Packet,Kind};
use hec_native_ic::{fp::Fp,poly::Poly};
use crate::index::{fold,build};
fn source_points(p:&Packet)->Option<Vec<(u64,u64)>>{let c=&p.curve;let u=p.d.u;let mut xs=vec![];match u.deg(){1=>xs.push(c.fp.neg(u.c[0])),2=>{let a=u.c[1];let delta=c.fp.sub(c.fp.mul(a,a),c.fp.mul(4,u.c[0]));let s=c.fp.sqrt(delta)?;let iv=c.fp.inv(2);xs.push(c.fp.mul(c.fp.sub(c.fp.neg(a),s),iv));xs.push(c.fp.mul(c.fp.add(c.fp.neg(a),s),iv));},_=>return None}let mut rebuilt=Poly::one();let mut points=vec![];for x in xs{rebuilt=rebuilt.mul(&Poly::x_minus(x,&c.fp),&c.fp);points.push((x,p.d.v.eval(x,&c.fp)));}if rebuilt!=u{return None}Some(points)}
pub fn verify(p:&Packet)->bool{let c=&p.curve;let n=p.reps.len()+1;if n<2||p.sm==0||p.sm>=p.ell||p.k>=p.ell||p.lambda>=p.ell||p.z>=c.fp.p||p.rows.is_empty()||p.witness.len()>p.rows.len(){return false}if !c.is_valid(&p.d)||!c.is_valid(&p.target)||c.is_zero(&p.d){return false}
 // Fixed-instance wholeprime order is an explicit external premise, not trusted packet input.
 let expected=match(c.fp.p,c.f.c[0]){(101,2)=>7481,(251,18)=>67931,(601,7)=>411491,_=>return false};if p.ell!=expected||c.g!=2||c.f!=Poly::from_coeffs(&[c.f.c[0],0,0,0,0,1]){return false}if c.fp.pow(p.z,5)!=1||p.z==1{return false}
 for &(x,y)in &p.reps{if x>=c.fp.p||y>=c.fp.p||c.fp.mul(y,y)!=c.f.eval(x,&c.fp){return false}}
 let core=c.place(p.reps[0].0,p.reps[0].1);if c.is_zero(&core)||c.mul(&core,p.sm)!=p.d{return false}if !crate::action::verify(c,p.z,p.ell,p.lambda){return false}let fl=Fp::new(p.ell);crate::phase::tap("binding");let index=build(p,&fl);crate::phase::tap("orbit_index");
 let maxa=p.rows.iter().filter_map(|r|if let Kind::Pencil{a,..}=r.kind{Some(a)}else{None}).max().unwrap_or(0);if maxa>16{return false}let mut anchors=vec![p.target];for i in 0..maxa as usize{anchors.push(c.add(&anchors[i],&p.d));}
 for anchor in &anchors{if !c.is_valid(anchor){return false}}
 crate::phase::tap("anchors");let mut matrix=vec![];let mut rhs=vec![];for row in &p.rows{if row.coeff.len()!=n||row.coeff.iter().any(|&x|x>=p.ell)||row.rhs>=p.ell{return false}let (want,b)=match &row.kind{
  Kind::Public=>{let mut v=vec![0;n];v[0]=1;(v,fl.inv(p.sm))},
  Kind::Source=>{let Some(points)=source_points(p)else{return false};let Some(v)=fold(p,&points,&fl,&index)else{return false};(v,1)},
  Kind::Pencil{a,t,points}=>{if *a==0||*a>maxa||*t>=c.fp.p||points.len()!=3{return false}let anchor=anchors[*a as usize];if anchor.u.deg()!=2{return false}if !crate::geometry::roots(c,&anchor,*t,points){return false}let Some(mut v)=fold(p,points,&fl,&index)else{return false};v[n-1]=1;(v,fl.neg(*a%p.ell))}
 };if row.coeff!=want||row.rhs!=b{return false}matrix.push(want);rhs.push(b);}
 crate::phase::tap("relations");let mut combined=vec![0;n];let mut k=0;for(i,&w)in p.witness.iter().enumerate(){if w>=p.ell{return false}for j in 0..n{combined[j]=fl.add(combined[j],fl.mul(w,matrix[i][j]));}k=fl.add(k,fl.mul(w,rhs[i]));}let mut expected=vec![0;n];expected[n-1]=1;{let accepted=combined==expected&&k==p.k;crate::phase::tap("witness");accepted}
}

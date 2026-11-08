#![allow(dead_code,unused_imports)]
include!("helpers.rs");
// J = F[e,s]/(e^3,s^2), index e+3*s. No nilpotent is inverted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct J<const P:i64,const D:i64>([F<P,D>;6]);
impl<const P:i64,const D:i64> J<P,D> {
 fn zero()->Self { Self([F::c(0);6]) }
 fn one()->Self { let mut z=Self::zero();z.0[0]=F::c(1);z }
 fn from(c:F<P,D>)->Self { let mut z=Self::zero();z.0[0]=c;z }
 fn add(self,b:Self)->Self { let mut z=self;for i in 0..6 {z.0[i]=z.0[i]+b.0[i]}z }
 fn neg(self)->Self {let mut z=self;for x in &mut z.0 {*x=-*x}z}
 fn sub(self,b:Self)->Self {self.add(b.neg())}
 fn mul(self,b:Self)->Self {
  let mut z=Self::zero();for i in 0..6 {for j in 0..6 {
   let e=i%3+j%3;let s=i/3+j/3;
   if e<3 && s<2 {z.0[e+3*s]=z.0[e+3*s]+self.0[i]*b.0[j]}
  }}z
 }
 fn inv(self)->Self {
  assert_ne!(self.0[0],F::c(0));let c=Self::from(self.0[0].inv());
  let v=self.mul(c).sub(Self::one());let vv=v.mul(v);
  let z=Self::one().sub(v).add(vv).sub(vv.mul(v)).mul(c);
  assert_eq!(self.mul(z),Self::one());z
 }
 fn code(self)->Value {json!(self.0.iter().map(|x|x.code()).collect::<Vec<_>>())}
}
fn determinant<const P:i64,const D:i64>(mut a:Vec<Vec<J<P,D>>>)->J<P,D> {
 let n=a.len();let mut out=J::one();
 for k in 0..n {
  if k+1==n {return out.mul(a[k][k])}
  let pivot=(k..n).flat_map(|i|(k..n).map(move|j|(i,j))).find(|&(i,j)|a[i][j].0[0]!=F::c(0));
  let (i,j)=pivot.expect("constant rank lower than m-1: unsupported singularity");
  if i!=k {a.swap(i,k);out=out.neg()}
  if j!=k {for row in &mut a {row.swap(j,k)}out=out.neg()}
  let v=a[k][k];out=out.mul(v);let vi=v.inv();let line=a[k].clone();
  for row in a.iter_mut().skip(k+1) {let z=row[k].mul(vi);for c in k..n {row[c]=row[c].sub(z.mul(line[c]))}}
 }out
}
fn ring_ev<const P:i64,const D:i64>(cs:&[Vec<F<P,D>>;4],x:F<P,D>)->J<P,D> {
 let mut z=J::zero();for (i,which) in [0,1,2,3].iter().enumerate() {z.0[*which]=ev(&cs[i],x)}z
}
fn trace_det<const P:i64,const D:i64>(cs:&[Vec<F<P,D>>;4],h:&[F<P,D>])->J<P,D> {
 let n=h.len()-1;let mut matrix=vec![vec![J::zero();n];n];
 for (i,which) in [0,1,2,3].iter().enumerate() {
  let mm=multmatrix(&cs[i],h);for row in 0..n {for col in 0..n {matrix[row][col].0[*which]=mm[row][col]}}
 }determinant(matrix)
}
fn jproduct<const P:i64,const D:i64>(cs:&[Vec<F<P,D>>;4],xs:&[F<P,D>])->J<P,D> {
 xs.iter().fold(J::one(),|v,&x|v.mul(ring_ev(cs,x)))
}
fn panel<const P:i64,const D:i64>(a:i64,b:i64,r:u64,g:Pt<P,D>,r0:Pt<P,D>,labels:&[u64],ulabels:&[u64],verify:bool,mutate:bool)->Value {
 assert!(mul(r,g,a).is_none());assert_ne!(mul(2,r0,a).unwrap().1.1,0);
 let rational_count=1+(0..P).flat_map(|x|(0..P).map(move|y|(x,y))).filter(|&(x,y)|(y*y-x*x*x-a*x-b).rem_euclid(P)==0).count();assert_eq!(rational_count,r as usize);
 let mut m=1;while 4*(m+1)*(m+1)<=r-1 {m+=1}let n=(r-1)/(4*m);
 let ii:Vec<_>=(0..n).map(|i|2*m*(2*i+1)).collect();let jj:Vec<_>=(0..m).map(|j|2*j+1).collect();let tail:Vec<_>=((4*m*n+1)..r).step_by(2).collect();
 let babies:Vec<_>=jj.iter().map(|&j|mul(j,g,a).unwrap()).collect();let xs:Vec<_>=babies.iter().map(|&(x,_)|x).collect();
 let mut h=vec![F::c(1)];for &x in &xs {h=pm(&h,&[-x,F::c(1)])}
 let (mut actions,mut poles,mut tails,mut origins,mut tangents,mut genuine_tangents,mut regular,mut wrongsign)=(0,0,0,0,0,0,0,0);
 let mut rows=vec![];
 for &label in labels {
  let q=mul(label,g,a);let qs:Vec<_>=jj.iter().map(|&j|mul(j,q,a).unwrap()).collect();
  let av:Vec<_>=qs.iter().map(|&(x,_)|x).collect();let bv:Vec<_>=babies.iter().zip(&qs).map(|(&(_,y),&(_,z))|y*z).collect();
  let ap=interpolate(&xs,&av,&h);let bp=interpolate(&xs,&bv,&h);
  for &ulabel in ulabels {
   let u=mul(ulabel,g,a);let pp=neg(u);let (_,yp)=pp.unwrap();let mut total=F::c(0);let mut hits=0;let mut blocks=vec![];
   for &i in &ii {for sg in [1i64,-1] {
    let ig=mul(i,g,a);let iq=mul(i,q,a);let ag=if sg==1 {ig}else{neg(ig)};let aq=if sg==1 {iq}else{neg(iq)};
    let ss=add(pp,ag,a);let tt=add(r0,aq,a).unwrap();
    if ss.is_none() {origins+=1;continue}
    let (sx,sy)=ss.unwrap();let (tx,ty)=tt;let dx=sy/yp;
    if pp==ag {tangents+=1;
     let (px,py)=pp.unwrap();let y1=(F::c(3)*px*px+F::c(a))/(F::c(2)*py);let y2=(F::c(3)*px-y1*y1)/(F::c(2)*py);
     assert_eq!(dx,F::c(2)*y1*y2-F::c(1));
    } else {
     let (px,py)=pp.unwrap();let (ax,ay)=ag.unwrap();let y1=(F::c(3)*px*px+F::c(a))/(F::c(2)*py);
     let slope=(py-ay)/(px-ax);let slope1=(y1-slope)/(px-ax);assert_eq!(dx,F::c(2)*slope*slope1-F::c(1));
    }
    let t=vec![F::c(0),F::c(1)];let n1=hnum(&t,sx,a,b);let n2=hnum(&ap,tx,a,b);
    let dv=sum(&ap,&[tx],F::c(-1));let d2nd=pm(&dv,&dv);
    let d0=pm(&pm(&[-sx,F::c(1)],&[-sx,F::c(1)]),&d2nd);
    let d1=scaled(&pm(&[-sx,F::c(1)],&d2nd),-F::c(2)*dx);
    let d2=scaled(&d2nd,dx*dx);
    let n0=scaled(&sum(&pm(&n1,&n2),&bp,F::c(4)*sy*ty),F::c(2));
    let mut cs=[d0,d1,d2,n0];
    // Mutation changes only the determinant's numerator; direct reference stays fixed.
    if mutate && xs.contains(&sx) {cs[3][0]=cs[3][0]+F::c(1)}
    let determinant=if verify {jproduct(&cs,&xs)}else{trace_det(&cs,&h)};
    if determinant.0[0]!=F::c(0) {regular+=1;assert!(!xs.contains(&sx));continue}
    assert_eq!(determinant.0[1],F::c(0));assert_ne!(determinant.0[2],F::c(0));
    let residue=determinant.0[3]/determinant.0[2];let mut direct=F::c(0);let mut cross=F::c(0);let mut root_count=0;
    for k in 0..m as usize {
     if sx!=xs[k] {continue}root_count+=1;
     let by=babies[k].1;let bq=Some(qs[k]);
     let (correct,wrong)=if sy+by==F::c(0) {(add(Some(tt),bq,a),add(Some(tt),neg(bq),a))}else{assert_eq!(sy,by);(add(Some(tt),neg(bq),a),add(Some(tt),bq,a))};
     direct=F::c(4)*yp*yp*correct.unwrap().0;cross=F::c(4)*yp*yp*wrong.unwrap().0;
    }
    assert_eq!(root_count,1);assert_eq!(residue,direct);assert_ne!(residue,cross);wrongsign+=1;
    if pp==ag {genuine_tangents+=1}
    total=total+residue;hits+=1;poles+=1;
    blocks.push(json!({"i":i,"sign":sg,"det":determinant.code(),"residue":residue.code(),"wrong_sign_residue":cross.code(),"tangent":pp==ag}));
   }}
   for &k in &tail {for sg in [1i64,-1] {
    let kg=mul(k,g,a);let kq=mul(k,q,a);let ag=if sg==1 {kg}else{neg(kg)};let aq=if sg==1 {kq}else{neg(kq)};
    if add(pp,ag,a).is_none() {total=total+F::c(4)*yp*yp*add(r0,aq,a).unwrap().0;hits+=1;tails+=1}
   }}
   assert_eq!(hits,1);let marked_x=total/(F::c(4)*yp*yp);let out=lift(marked_x,r0,r,a,b);
   assert_eq!(out,mul(label,u,a));actions+=1;
   rows.push(json!({"Q_label":label,"U_label":ulabel,"U":pc(u),"raw_coefficient":total.code(),"marked_x":marked_x.code(),"recovered":pc(out),"blocks":blocks}));
  }
 }
 json!({"p":P,"D":D,"a":a,"b":b,"r":r,"G":pc(g),"R0":pc(r0),"m":m,"n":n,"actions":actions,"genuine_pole_blocks":poles,"explicit_tail_poles":tails,"origin_chart_skips":origins,"tangent_translations":tangents,"genuine_tangent_poles":genuine_tangents,"regular_blocks":regular,"wrong_relative_sign_rejections":wrongsign,"rows":rows})
}
pub fn run() {
 let args:Vec<_>=std::env::args().collect();let verify=args.iter().any(|s|s=="--root-product");let mutate=args.iter().any(|s|s=="--mutate");
 let sa=Some((F::<11,2>::c(0),F::new(0,3)));let sb=Some((F::<17,3>::c(1),F::new(0,8)));
 let rt=(1..211).find(|x|x*x%211==204).unwrap();let sc=Some((F::<211,210>::c(0),F::new(0,rt)));assert!(mul(3,sc,0).is_none());
 let a=panel::<11,2>(2,7,7,Some((F::c(6),F::c(2))),mul(6,sa,2),&(1..7).collect::<Vec<_>>(),&(1..7).collect::<Vec<_>>(),verify,mutate);
 let b=panel::<17,3>(2,2,19,Some((F::c(5),F::c(1))),mul(6,sb,2),&(1..19).collect::<Vec<_>>(),&(1..19).collect::<Vec<_>>(),verify,mutate);
 let c=panel::<211,210>(0,7,199,Some((F::c(3),F::c(33))),sc,&[1,2,3,5,7,11,17,31,63,97,128,198],&[1,2,3,7,31,98,101,198],verify,mutate);
 let out=json!({"all_checks_pass":true,"controls":[a,b,c],"standing":"unsigned exact native checks; determinant and root-product outputs must match"});
 if let Some(at)=args.iter().position(|s|s=="--check") {let expected:Value=serde_json::from_slice(&fs::read(&args[at+1]).unwrap()).unwrap();assert_eq!(out,expected);eprintln!("certificate exactly matches independent root-product replay");}
 println!("{}",serde_json::to_string(&out).unwrap());
}

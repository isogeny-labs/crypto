//! Prime-field local leading terms. Prime p and prime subgroup order r are caller-certified.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct Point { pub x:u64, pub y:u64 }
#[derive(Clone,Copy,Debug)]
pub struct Curve { p:u64, a:u64, b:u64 }
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct Local { pub order:i128, pub coefficient:u64 }
impl Curve {
 pub fn new(p:u64,a:u64,b:u64)->Self {assert!(p>3 && p<=u64::MAX/2 && a<p && b<p);let c=Self{p,a,b};assert_ne!(c.add(c.mul(4,c.pow(a,3)),c.mul(27%p,c.mul(b,b))),0);c}
 fn add(self,a:u64,b:u64)->u64 {(a+b)%self.p}
 fn neg(self,a:u64)->u64 {if a==0 {0}else{self.p-a}}
 fn sub(self,a:u64,b:u64)->u64 {self.add(a,self.neg(b))}
 fn mul(self,a:u64,b:u64)->u64 {((a as u128*b as u128)%self.p as u128)as u64}
 fn pow(self,mut a:u64,mut n:u64)->u64 {let mut z=1;while n>0 {if n&1==1 {z=self.mul(z,a)}a=self.mul(a,a);n>>=1;}z}
 fn inv(self,a:u64)->u64 {assert_ne!(a,0);let z=self.pow(a,self.p-2);assert_eq!(self.mul(a,z),1);z}
 fn div(self,a:u64,b:u64)->u64 {self.mul(a,self.inv(b))}
 fn valid(self,t:Point){assert!(t.x<self.p&&t.y<self.p);assert_eq!(self.mul(t.y,t.y),self.add(self.add(self.pow(t.x,3),self.mul(self.a,t.x)),self.b));}
 pub fn negative(self,t:Point)->Point {Point{x:t.x,y:self.neg(t.y)}}
 pub fn plus(self,a:Option<Point>,b:Option<Point>)->Option<Point> {
  let (a,b)=match(a,b){(None,b)=>return b,(a,None)=>return a,(Some(a),Some(b))=>(a,b)};
  if a.x==b.x && self.add(a.y,b.y)==0 {return None}
  let l=if a==b {self.div(self.add(self.mul(3,self.mul(a.x,a.x)),self.a),self.mul(2,a.y))}else{self.div(self.sub(b.y,a.y),self.sub(b.x,a.x))};
  let x=self.sub(self.sub(self.mul(l,l),a.x),b.x);Some(Point{x,y:self.sub(self.mul(l,self.sub(a.x,x)),a.y)})
 }
 fn combine(self,a:Local,b:Local,divide:bool)->Local {Local{order:if divide{a.order.checked_sub(b.order)}else{a.order.checked_add(b.order)}.unwrap(),coefficient:if divide{self.div(a.coefficient,b.coefficient)}else{self.mul(a.coefficient,b.coefficient)}}}
 fn vertical(self,x:u64,t:Option<Point>)->Local {
  match t {None=>Local{order:-2,coefficient:1},Some(t)=>{let d=self.sub(t.x,x);if d==0 {Local{order:1,coefficient:1}}else{Local{order:0,coefficient:d}}}}
 }
 // Finite expansion uses u=x-x(T), valid only for y(T)!=0.
 fn line(self,a:Point,l:u64,t:Option<Point>)->Local {
  let t=match t {None=>return Local{order:-3,coefficient:self.neg(1)},Some(t)=>t};
  let d=self.sub(self.sub(t.y,a.y),self.mul(l,self.sub(t.x,a.x)));
  if d!=0{return Local{order:0,coefficient:d}}
  let den=self.inv(self.mul(2,t.y));
  let y1=self.mul(self.add(self.mul(3,self.mul(t.x,t.x)),self.a),den);
  let y2=self.mul(self.sub(self.mul(3,t.x),self.mul(y1,y1)),den);
  let y3=self.mul(self.sub(1,self.mul(2,self.mul(y1,y2))),den);
  for (i,c) in [self.sub(y1,l),y2,y3].iter().enumerate(){if *c!=0{return Local{order:(i+1)as i128,coefficient:*c}}}
  panic!("line cannot have intersection multiplicity above three");
 }
 // g has divisor [A]+[B]-[A+B]-[O], leading coefficient 1 at O.
 fn g(self,a:Option<Point>,b:Option<Point>,t:Option<Point>)->Local {
  let(a,b)=match(a,b){(Some(a),Some(b))=>(a,b),_=>return Local{order:0,coefficient:1}};
  let sum=self.plus(Some(a),Some(b));
  if sum.is_none(){return self.vertical(a.x,t)}
  let l=if a==b{self.div(self.add(self.mul(3,self.mul(a.x,a.x)),self.a),self.mul(2,a.y))}else{self.div(self.sub(b.y,a.y),self.sub(b.x,a.x))};
  let mut z=self.combine(self.line(a,l,t),self.vertical(sum.unwrap().x,t),true);z.coefficient=self.neg(z.coefficient);z
 }
 /// Normalized f_{r,base}; finite parameter x-x(T), infinity parameter -x/y.
 /// Rejects finite 2-torsion evaluation points. No kernel or scalar labels are inputs.
 pub fn miller(self,base:Point,r:u64,t:Option<Point>)->Local {
  assert!(r>=3 && r%2==1 && r!=self.p && r<=u64::MAX/2);self.valid(base);assert_ne!(base.y,0);
  if let Some(t)=t {self.valid(t);assert_ne!(t.y,0)}
  let mut v=Some(base);let mut z=Local{order:0,coefficient:1};
  for bit in (0..63-r.leading_zeros()).rev(){z=self.combine(z,z,false);z=self.combine(z,self.g(v,v,t),false);v=self.plus(v,v);
   if(r>>bit)&1==1 {z=self.combine(z,self.g(v,Some(base),t),false);v=self.plus(v,Some(base));}
  }
  assert_eq!(v,None,"base must be r torsion");z
 }
 /// Convert the leading coefficient to invariant translated parameter t(Q-T).
 pub fn invariant(self,z:Local,t:Point)->Local {assert_ne!(t.y,0);let b=self.mul(2,t.y);let e=u64::try_from(z.order.unsigned_abs()).unwrap();let d=self.pow(b,e);Local{order:z.order,coefficient:if z.order<0{self.div(z.coefficient,d)}else{self.mul(z.coefficient,d)}}}
}
#[cfg(test)] mod tests {
 use super::*;
 // Reference enumerates the known subgroup only in tests and differentiates its monic kernel.
 fn subgroup(c:Curve,p:Point,r:u64)->Vec<Point>{let mut v=Vec::new();let mut t=Some(p);for _ in 1..r{v.push(t.unwrap());t=c.plus(t,Some(p));}assert_eq!(t,None);v}
 fn anchor(c:Curve,h:&[Point],t:Point)->u64 {let mut xs=Vec::new();for p in h{if !xs.contains(&p.x){xs.push(p.x)}}let mut d=1;for x in xs{if x!=t.x{d=c.mul(d,c.sub(t.x,x));}}c.mul(c.mul(2,t.y),d)}
 #[test]fn subgroup_local_identities(){
  for (p,a,b,px,py,r) in [(5,0,1,0,1,3),(5,2,1,0,1,7),(7,0,3,1,2,13),(509,2,10,3,68,479)] {
   let c=Curve::new(p,a,b);let h=subgroup(c,Point{x:px,y:py},r);
   let sources:Vec<_>=if r<20{h.clone()}else{[0,1,2,6,15,31,127,477].iter().map(|i|h[*i]).collect()};
   let targets:Vec<_>=if r<20{h.clone()}else{[0,1,2,3,4,7,15,31,63,127,255,477].iter().map(|i|h[*i]).collect()};
   for s in sources {let cs=anchor(c,&h,s);let base=c.negative(s);
    assert_eq!(c.miller(base,r,None),Local{order:-(r as i128),coefficient:1});
    let zero=c.invariant(c.miller(base,r,Some(base)),base);
    assert_eq!(zero,Local{order:r as i128,coefficient:c.neg(c.mul(cs,cs))});
    for &t in &targets {if t==base{continue}let ts=c.plus(Some(t),Some(s)).unwrap();let f=c.miller(base,r,Some(t));assert_eq!(f.order,0);
     assert_eq!(anchor(c,&h,ts),c.div(c.mul(cs,anchor(c,&h,t)),f.coefficient));
    }
   }
  }
 }
 #[test]fn explicit_five_field_cubic(){let c=Curve::new(5,0,1);let b=Point{x:0,y:4};
  for t in [Point{x:0,y:1},Point{x:2,y:2},Point{x:2,y:3}] {assert_eq!(c.miller(b,3,Some(t)),Local{order:0,coefficient:c.neg(c.add(t.y,1))});}
  assert_eq!(c.miller(b,3,Some(b)),Local{order:3,coefficient:3});
  // Tangent contact orders two and three are evaluated without division by zero.
  assert_eq!(c.line(b,0,Some(b)).order,3);
  let t=Point{x:2,y:2};let l=c.div(c.mul(3,c.mul(t.x,t.x)),c.mul(2,t.y));assert_eq!(c.line(t,l,Some(t)).order,2);
 }
 #[test]fn normalized_primitives_and_cancellation(){let c=Curve::new(7,0,3);let h=subgroup(c,Point{x:1,y:2},13);let mut cancelled=0;
  for &a in &h {for &b in &h {assert_eq!(c.g(Some(a),Some(b),None).coefficient,1);
   if let Some(s)=c.plus(Some(a),Some(b)){for &t in &h{let l=if a==b{c.div(c.mul(3,c.mul(a.x,a.x)),c.mul(2,a.y))}else{c.div(c.sub(b.y,a.y),c.sub(b.x,a.x))};let n=c.line(a,l,Some(t));let d=c.vertical(s.x,Some(t));if n.order>0&&d.order>0{cancelled+=1;let g=c.g(Some(a),Some(b),Some(t));assert_eq!(g.order,n.order-d.order);assert_ne!(g.coefficient,0);}}}
  }}assert!(cancelled>0);
 }
 #[test]fn invalid_domains(){let c=Curve::new(5,0,1);assert!(std::panic::catch_unwind(||c.miller(Point{x:0,y:1},3,Some(Point{x:4,y:0}))).is_err());assert!(std::panic::catch_unwind(||c.miller(Point{x:2,y:2},3,None)).is_err());}
}

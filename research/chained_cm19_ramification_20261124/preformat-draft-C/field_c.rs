pub type K=[u8;2];
pub fn k(c:u8)->K {assert!(c<7);[c,0]}
pub fn m(n:i32)->u8 {u8::try_from(n.rem_euclid(7)).unwrap()}
pub fn add(a:u8,b:u8)->u8 {m(i32::from(a)+i32::from(b))}
pub fn sub(a:u8,b:u8)->u8 {m(i32::from(a)-i32::from(b))}
pub fn mul(a:u8,b:u8)->u8 {m(i32::from(a)*i32::from(b))}
pub fn inv(a:u8)->u8 {(1..7).find(|b|mul(a,*b)==1).unwrap()}
#[derive(Clone,Copy,Debug)]
pub struct Quad {pub constant:u8,pub linear:u8}
impl Quad {
 pub fn new(modulus:[u8;3])->Self {
  assert_eq!(modulus[2],1);
  for a in 0..7u8 {assert_ne!(add(add(mul(a,a),mul(modulus[1],a)),modulus[0]),0);}
  Self{constant:sub(0,modulus[0]),linear:sub(0,modulus[1])}
 }
 pub fn add(self,a:K,b:K)->K {[add(a[0],b[0]),add(a[1],b[1])]}
 pub fn sub(self,a:K,b:K)->K {[sub(a[0],b[0]),sub(a[1],b[1])]}
 pub fn mul(self,a:K,b:K)->K {let bb=mul(a[1],b[1]);[add(mul(a[0],b[0]),mul(bb,self.constant)),add(add(mul(a[0],b[1]),mul(a[1],b[0])),mul(bb,self.linear))]}
 pub fn pow(self,a:K,n:usize)->K {(0..n).fold(k(1),|r,_|self.mul(r,a))}
 pub fn conjugate(self,a:K)->K {[add(a[0],mul(self.linear,a[1])),sub(0,a[1])]}
 pub fn inverse(self,a:K)->K {let c=self.conjugate(a);let n=self.mul(a,c);assert_eq!(n[1],0);assert_ne!(n[0],0);let scalar=inv(n[0]);[mul(c[0],scalar),mul(c[1],scalar)]}
 pub fn divide(self,a:K,b:K)->K {self.mul(a,self.inverse(b))}
}
pub type EPoint=Option<(K,K)>;
pub fn valid(f:Quad,p:EPoint)->bool {p.is_none_or(|(x,y)|f.pow(y,2)==f.add(f.add(f.pow(x,3),f.mul(k(2),x)),k(6)))}
pub fn eadd(f:Quad,a:EPoint,b:EPoint)->EPoint {
 assert!(valid(f,a)&&valid(f,b));let Some((x,y))=a else{return b;};let Some((xx,yy))=b else{return a;};
 if x==xx && f.add(y,yy)==k(0) {return None;}
 let slope=if x==xx {f.divide(f.add(f.mul(k(3),f.pow(x,2)),k(2)),f.mul(k(2),y))}else{f.divide(f.sub(yy,y),f.sub(xx,x))};
 let nx=f.sub(f.sub(f.pow(slope,2),x),xx);let ny=f.sub(f.mul(slope,f.sub(x,nx)),y);let p=Some((nx,ny));assert!(valid(f,p));p
}

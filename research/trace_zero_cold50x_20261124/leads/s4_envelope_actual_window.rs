use std::{env, fs};
type E = [u64;3];
fn add(a:E,b:E)->E {std::array::from_fn(|i|a[i]^b[i])}
struct Field {n:usize,tail:E}
impl Field {
 fn x(&self,a:E)->E {let top=(a[(self.n-1)/64]>>((self.n-1)%64))&1;let mut b=[a[0]<<1,(a[1]<<1)|(a[0]>>63),(a[2]<<1)|(a[1]>>63)];b[(self.n-1)/64]&=(1u64<<(self.n%64))-1;for i in 0..3 {b[i]^=self.tail[i]&0u64.wrapping_sub(top);}b}
 fn mul(&self,mut a:E,b:E)->E {let mut r=[0;3];for i in 0..self.n {if (b[i/64]>>(i%64))&1!=0 {r=add(r,a);}a=self.x(a);}r}
 fn sq(&self,a:E)->E {self.mul(a,a)}
 fn fourth(&self,a:E)->E {self.sq(self.sq(a))}
 fn monomial(&self,j:usize)->E {(0..j).fold([1,0,0],|a,_|self.x(a))}
 fn resultant(&self,x:E,y:E,z:E,t:E)->E {
  let a=self.sq(add(x,y));let b=self.mul(x,y);let c=add(self.sq(b),[1,0,0]);let d=self.sq(add(z,t));let e=self.mul(z,t);let f=add(self.sq(e),[1,0,0]);
  add(self.sq(add(self.mul(a,f),self.mul(d,c))),self.mul(add(self.mul(a,e),self.mul(d,b)),add(self.mul(b,f),self.mul(e,c))))
 }
 fn symmetric(&self,x:E,y:E,z:E,t:E)->E {
  let a=add(add(x,y),z);let b=add(add(self.mul(x,y),self.mul(x,z)),self.mul(y,z));let c=self.mul(self.mul(x,y),z);let a2=self.sq(a);let b2=self.sq(b);let c2=self.sq(c);let t2=self.sq(t);let t3=self.mul(t2,t);let t4=self.sq(t2);
  let mut r=self.mul(add(add(self.sq(b2),c2),[1,0,0]),t4);
  r=add(r,self.mul(self.mul(c,add(b2,[1,0,0])),t3));r=add(r,self.mul(add(b2,self.mul(a2,c2)),t2));r=add(r,self.mul(self.mul(c,add(a2,c2)),t));add(r,add(add(self.sq(a2),self.sq(c2)),c2))
 }
}
fn insert(p:&mut [E],mut a:E)->bool {for bit in (0..p.len()).rev() {if a[bit/64]>>(bit%64)&1==0 {continue;}if p[bit]==[0;3] {p[bit]=a;return true;}a=add(a,p[bit]);}false}
fn contains(p:&[E],mut a:E)->bool {for bit in (0..p.len()).rev(){if a[bit/64]>>(bit%64)&1!=0 {if p[bit]==[0;3] {return false;}a=add(a,p[bit]);}}a==[0;3]}
fn parse(s:&str)->E {let mut a=[0;3];for c in s.chars(){let d=c.to_digit(16).unwrap() as u64;a=[(a[0]<<4)|d,(a[1]<<4)|(a[0]>>60),(a[2]<<4)|(a[1]>>60)];}a}
fn main(){
 let f=Field{n:3,tail:[3,0,0]};let mut checked=0;
 for a in 0..8 {for b in 0..8 {for c in 0..8 {for t in 0..8 {assert_eq!(f.resultant([a,0,0],[b,0,0],[c,0,0],[t,0,0]),f.symmetric([a,0,0],[b,0,0],[c,0,0],[t,0,0]));checked+=1;}}}}
 println!("{{\"kind\":\"identity\",\"exhaustive_f8_tuples\":{checked},\"matches\":true}}");
 for line in fs::read_to_string(&env::args().nth(1).unwrap()).unwrap().lines() {
  let a:Vec<_>=line.split_whitespace().collect();let n:usize=a[1].parse().unwrap();let f=Field{n,tail:if n==83 {[(1u64<<45)|7,0,0]}else{[0x2007,0,0]}};let t=parse(a[2]);let t2=f.sq(t);let t3=f.mul(t2,t);let t4=f.sq(t2);
  for d in [2,4,6,8,10,12] {let k=d;let mut p=vec![[0;3];n];let mut columns=0;let mut rank=0;
   for family in 0..6 {let bound=[k,2*k,3*k,7*k,4*k,9*k][family];for j in 0..=bound {
    let col=match family {0=>f.monomial(4*j),1=>add(f.mul(t4,f.monomial(4*j)),f.mul(t2,f.monomial(2*j))),2=>add(add(f.monomial(4*j),f.mul(add(t4,[1,0,0]),f.monomial(2*j))),f.mul(t3,f.monomial(j))),3=>f.mul(t3,f.monomial(j)),4=>f.mul(t2,f.monomial(2*j)),_=>f.mul(t,f.monomial(j))};columns+=1;if insert(&mut p,col){rank+=1;}
   }}
   let consistent=contains(&p,t4);println!("{{\"kind\":\"envelope\",\"instance\":\"{}\",\"n\":{},\"d\":{},\"columns\":{},\"rank\":{},\"consistent\":{},\"full_rank\":{}}}",a[0],n,d,columns,rank,consistent,rank==n);
  }
 }
}

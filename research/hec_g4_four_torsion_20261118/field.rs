pub type E=[u64;9];pub type P=Vec<E>;
pub const Z:E=[0;9];
pub fn scalar(a:u64)->E{let mut v=Z;v[0]=a;v}
pub fn trim(mut a:P)->P{while a.last()==Some(&Z){a.pop();}a}
#[derive(Clone)]pub struct F{pub p:u64,pub n:usize,pub m:Vec<u64>,pub q:u64}
impl F{
 pub fn new(p:u64,m:Vec<u64>)->Self{let n=m.len()-1;assert!(n<=9&&m[n]==1);Self{p,n,m,q:p.pow(n as u32)}}
 pub fn add(&self,a:E,b:E)->E{let mut c=Z;for i in 0..self.n{c[i]=(a[i]+b[i])%self.p}c}
 pub fn sub(&self,a:E,b:E)->E{let mut c=Z;for i in 0..self.n{c[i]=(a[i]+self.p-b[i])%self.p}c}
 pub fn mul(&self,a:E,b:E)->E{let mut c=[0u64;17];for i in 0..self.n{for j in 0..self.n{c[i+j]=(c[i+j]+a[i]*b[j])%self.p}}for i in(self.n..2*self.n-1).rev(){let v=c[i];for j in 0..self.n{c[i-self.n+j]=(c[i-self.n+j]+self.p-(v*self.m[j])%self.p)%self.p}c[i]=0}let mut out=Z;out[..self.n].copy_from_slice(&c[..self.n]);out}
 pub fn pow(&self,mut a:E,mut n:u64)->E{let mut b=scalar(1);while n>0{if n&1!=0{b=self.mul(b,a)}a=self.mul(a,a);n>>=1}b}
 pub fn inv(&self,a:E)->E{assert_ne!(a,Z);if a==scalar(1){a}else{let b=self.pow(a,self.q-2);assert_eq!(self.mul(a,b),scalar(1));b}}
 pub fn pa(&self,a:&P,b:&P)->P{let mut c=vec![Z;a.len().max(b.len())];for(i,x)in c.iter_mut().enumerate(){*x=self.add(*a.get(i).unwrap_or(&Z),*b.get(i).unwrap_or(&Z))}trim(c)}
 pub fn ps(&self,a:&P,b:&P)->P{let mut c=vec![Z;a.len().max(b.len())];for(i,x)in c.iter_mut().enumerate(){*x=self.sub(*a.get(i).unwrap_or(&Z),*b.get(i).unwrap_or(&Z))}trim(c)}
 pub fn pm(&self,a:&P,b:&P)->P{if a.is_empty()||b.is_empty(){return vec![]}let mut c=vec![Z;a.len()+b.len()-1];for(i,&v)in a.iter().enumerate(){for(j,&w)in b.iter().enumerate(){c[i+j]=self.add(c[i+j],self.mul(v,w))}}trim(c)}
 pub fn div(&self,a:&P,b:&P)->(P,P){assert!(!b.is_empty());if a.len()<b.len(){return(vec![],a.clone())}let mut r=a.clone();let mut q=vec![Z;a.len()-b.len()+1];let iv=self.inv(*b.last().unwrap());while !r.is_empty()&&r.len()>=b.len(){let j=r.len()-b.len();let v=self.mul(*r.last().unwrap(),iv);q[j]=v;for(k,&w)in b.iter().enumerate(){r[j+k]=self.sub(r[j+k],self.mul(v,w))}r=trim(r)}(trim(q),r)}
 pub fn rem(&self,a:&P,b:&P)->P{self.div(a,b).1}
 pub fn monic(&self,a:&P)->P{if a.is_empty(){return a.clone()}let v=self.inv(*a.last().unwrap());a.iter().map(|&x|self.mul(x,v)).collect()}
 pub fn gcd(&self,mut a:P,mut b:P)->P{while !b.is_empty(){let r=self.rem(&a,&b);a=b;b=r}self.monic(&a)}
 pub fn ppow(&self,mut a:P,mut n:u64,m:&P)->P{let mut b=vec![scalar(1)];while n>0{if n&1!=0{b=self.rem(&self.pm(&b,&a),m)}a=self.rem(&self.pm(&a,&a),m);n>>=1}b}
 pub fn eval(&self,a:&P,x:E)->E{a.iter().rev().fold(Z,|s,&v|self.add(self.mul(s,x),v))}
 pub fn elements(&self,i:u64)->E{let mut r=Z;let mut i=i;for v in &mut r[..self.n]{*v=i%self.p;i/=self.p}assert_eq!(i,0);r}
}
pub fn poly(v:&[u64])->P{trim(v.iter().copied().map(scalar).collect())}
pub fn irreducible(p:u64,a:&[u64]){let f=F::new(p,vec![0,1]);let m=poly(a);let n=a.len()-1;let x=poly(&[0,1]);let mut y=x.clone();for i in 1..=n{y=f.ppow(y,p,&m);if i<n&&n%i==0{assert_eq!(f.gcd(m.clone(),f.ps(&y,&x)).len(),1)}}assert!(f.ps(&y,&x).is_empty())}

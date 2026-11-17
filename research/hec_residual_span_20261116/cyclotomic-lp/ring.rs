use hec_native_ic::{fp::Fp,poly::Poly};
pub type R=[u64;4];pub const ZERO:R=[0;4];pub const ONE:R=[1,0,0,0];
pub struct Ring{pub f:Fp,pub phi:Poly}
impl Ring{
 pub fn new(ell:u64)->Self{Self{f:Fp::new(ell),phi:Poly::from_coeffs(&[1;5])}}
 pub fn constant(&self,x:u64)->R{[x%self.f.p,0,0,0]}
 pub fn z(&self,e:usize)->R{let e=e%5;if e==4{[self.f.p-1;4]}else{let mut v=ZERO;v[e]=1;v}}
 pub fn add(&self,a:R,b:R)->R{std::array::from_fn(|j|self.f.add(a[j],b[j]))}
 pub fn sub(&self,a:R,b:R)->R{std::array::from_fn(|j|self.f.sub(a[j],b[j]))}
 pub fn neg(&self,a:R)->R{std::array::from_fn(|j|self.f.neg(a[j]))}
 fn product(&self,a:u64,b:u64)->u64{if a==0||b==0{0}else if a==1{b}else if b==1{a}else if a==self.f.p-1{self.f.neg(b)}else if b==self.f.p-1{self.f.neg(a)}else{self.f.mul(a,b)}}
 pub fn scale(&self,a:R,c:u64)->R{std::array::from_fn(|j|self.product(a[j],c))}
 pub fn mul(&self,a:R,b:R)->R{let mut c=[0;5];for i in 0..4{if a[i]==0{continue}for j in 0..4{if b[j]!=0{let k=(i+j)%5;c[k]=self.f.add(c[k],self.product(a[i],b[j]));}}}std::array::from_fn(|j|self.f.sub(c[j],c[4]))}
 pub fn poly(&self,a:R)->Poly{Poly::from_coeffs(&a)}
 pub fn element(&self,p:Poly)->R{let p=p.rem(&self.phi,&self.f);std::array::from_fn(|j|p.c[j])}
 pub fn eval(&self,a:R,x:u64)->u64{self.poly(a).eval(x,&self.f)}
 pub fn inv(&self,a:R)->R{let nz:Vec<_>=a.iter().enumerate().filter(|(_,x)|**x!=0).collect();if nz.len()==1{let(j,&c)=nz[0];let inv=if c==1{1}else if c==self.f.p-1{self.f.p-1}else{self.f.inv(c)};return self.scale(self.z((5-j)%5),inv)}if a[0]!=0&&a.iter().all(|x|*x==a[0]){let c=self.f.neg(a[0]);let inv=if c==1{1}else if c==self.f.p-1{self.f.p-1}else{self.f.inv(c)};return self.scale(self.z(1),inv)}let(g,s,_)=Poly::xgcd(&self.poly(a),&self.phi,&self.f);assert_eq!(g,Poly::one(),"nonunit LP pivot");let inv=self.element(s);assert_eq!(self.mul(a,inv),ONE);inv}
}
#[derive(Clone)]struct Branch{m:Poly,basis:[Option<[Poly;3]>;2]}
fn narrow(mut b:Branch,m:Poly,f:&Fp)->Branch{b.m=m;for p in b.basis.iter_mut().flatten(){for x in p{*x=x.rem(&m,f)}}b}
fn branch_insert(f:&Fp,mut b:Branch,mut row:[Poly;3])->Vec<Branch>{for x in &mut row{*x=x.rem(&b.m,f)}for j in 0..2{if row[j].is_zero(){continue}if let Some(p)=b.basis[j]{let a=row[j];for k in j..3{row[k]=row[k].sub(&a.mul(&p[k],f),f).rem(&b.m,f)}}else{let g=Poly::gcd(&row[j],&b.m,f);if g.deg()>0{assert!(g.deg()<b.m.deg());let other=b.m.div_exact(&g,f);let mut out=branch_insert(f,narrow(b.clone(),g,f),row);out.extend(branch_insert(f,narrow(b,other,f),row));return out}let(g,inv,_)=Poly::xgcd(&row[j],&b.m,f);assert_eq!(g,Poly::one());for x in &mut row{*x=x.mul(&inv,f).rem(&b.m,f)}b.basis[j]=Some(row);return vec![b]}}
if row[2].is_zero(){vec![b]}else{let g=Poly::gcd(&row[2],&b.m,f);if g.deg()==0{vec![]}else{vec![narrow(b,g,f)]}}}
struct Labels{branches:Vec<Branch>}
impl Labels{fn new(phi:Poly)->Self{Self{branches:vec![Branch{m:phi,basis:[None,None]}]}}fn insert(&mut self,ring:&Ring,v:[R;3]){let row=v.map(|x|ring.poly(x));self.branches=std::mem::take(&mut self.branches).into_iter().flat_map(|b|branch_insert(&ring.f,b,row)).collect();}fn status(&self,x:u64,f:&Fp)->Option<(usize,Option<u64>)>{let mut found=None;for b in &self.branches{if b.m.eval(x,f)==0{assert!(found.is_none());found=Some((b.basis.iter().filter(|x|x.is_some()).count(),b.basis[1].map(|r|r[2].eval(x,f))));}}found}fn unique(&self,f:&Fp)->Option<(u64,Option<u64>)>{if self.branches.len()!=1||self.branches[0].m.deg()!=1{return None}let b=&self.branches[0];Some((f.neg(b.m.c[0]),b.basis[1].map(|r|r[2].c[0])))}fn survivors(&self)->usize{self.branches.iter().map(|b|b.m.deg()as usize).sum()}}
pub struct Dsu{pub ring:Ring,n:usize,parent:Vec<usize>,size:Vec<usize>,weight:Vec<R>,shift:Vec<[R;3]>,labels:Labels,pub lp_rank:usize}
impl Dsu{
 pub fn new(n:usize,ell:u64)->Self{let ring=Ring::new(ell);let labels=Labels::new(ring.phi);Self{ring,n,parent:(0..=n).collect(),size:vec![1;n+1],weight:vec![ONE;n+1],shift:vec![[ZERO;3];n+1],labels,lp_rank:0}}
 fn find(&mut self,x:usize)->(usize,R,[R;3]){if self.parent[x]==x{return(x,ONE,[ZERO;3])}let(p,w,h)=(self.parent[x],self.weight[x],self.shift[x]);let(r,v,t)=self.find(p);let z=std::array::from_fn(|j|self.ring.add(h[j],self.ring.mul(w,t[j])));let w=self.ring.mul(w,v);self.parent[x]=r;self.weight[x]=w;self.shift[x]=z;(r,w,z)}
 pub fn insert(&mut self,v:&[R],rhs:R){assert_eq!(v.len(),self.n+2);let mut q=[v[self.n],v[self.n+1],self.ring.neg(rhs)];let mut roots=Vec::<(usize,R)>::new();assert!(v[..self.n].iter().filter(|x|**x!=ZERO).count()<=2);for(x,&a)in v[..self.n].iter().enumerate(){if a==ZERO{continue}let(r,w,h)=self.find(x);for j in 0..3{q[j]=self.ring.add(q[j],self.ring.mul(a,h[j]))}if r==self.n{continue}let b=self.ring.mul(a,w);if let Some(e)=roots.iter_mut().find(|e|e.0==r){e.1=self.ring.add(e.1,b)}else{roots.push((r,b));}}roots.retain(|e|e.1!=ZERO);if roots.is_empty(){self.labels.insert(&self.ring,[q[0],q[1],self.ring.neg(q[2])]);return}if roots.len()==2&&self.size[roots[0].0]>self.size[roots[1].0]{roots.swap(0,1)}let(root,a)=roots[0];let inv=self.ring.inv(a);let h=q.map(|x|self.ring.neg(self.ring.mul(x,inv)));if roots.len()==1{self.parent[root]=self.n;self.weight[root]=ZERO;self.shift[root]=h;}else{let(other,b)=roots[1];self.parent[root]=other;self.weight[root]=self.ring.neg(self.ring.mul(b,inv));self.shift[root]=h;self.size[other]+=self.size[root];}self.lp_rank+=1;
 }
 pub fn status(&self,x:u64)->Option<(usize,Option<u64>)>{self.labels.status(x,&self.ring.f).map(|(r,k)|(r+self.lp_rank,k))}
 pub fn unique(&self)->Option<(u64,Option<u64>)>{self.labels.unique(&self.ring.f)}
 pub fn survivors(&self)->usize{self.labels.survivors()}
}

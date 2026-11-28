use hec_native_ic::fp::Fp;
#[path="../orbit-core/sparse.rs"]mod small;
pub struct Affine {f:Fp,n:usize,parent:Vec<usize>,size:Vec<usize>,weight:Vec<u64>,shift:Vec<[u64;3]>,labels:small::Sparse,pub lp_rank:usize,pub hops:usize}
impl Affine{
 pub fn new(n:usize,ell:u64)->Self{Self{f:Fp::new(ell),n,parent:(0..=n).collect(),size:vec![1;n+1],weight:vec![1;n+1],shift:vec![[0;3];n+1],labels:small::Sparse::new(2,ell),lp_rank:0,hops:0}}
 fn product(&self,a:u64,b:u64)->u64{if a==0||b==0{0}else if a==1{b}else if b==1{a}else{self.f.mul(a,b)}}
 fn find(&mut self,x:usize)->(usize,u64,[u64;3]){if self.parent[x]==x{return(x,1,[0;3])}self.hops+=1;let(p,w,h)=(self.parent[x],self.weight[x],self.shift[x]);let(r,v,t)=self.find(p);let mut z=h;for j in 0..3{z[j]=self.f.add(z[j],self.product(w,t[j]));}let w=self.product(w,v);self.parent[x]=r;self.weight[x]=w;self.shift[x]=z;(r,w,z)}
 pub fn k(&self)->Option<u64>{self.labels.k()}
 pub fn rank(&self)->usize{self.lp_rank+self.labels.rank}
 pub fn insert(&mut self,v:&[u64],rhs:u64){assert_eq!(v.len(),self.n+2);let mut q=[v[self.n],v[self.n+1],self.f.neg(rhs)];let mut roots=Vec::<(usize,u64)>::new();assert!(v[..self.n].iter().filter(|x|**x!=0).count()<=2);for(x,&a)in v[..self.n].iter().enumerate(){if a==0{continue}let(r,w,h)=self.find(x);for j in 0..3{q[j]=self.f.add(q[j],self.product(a,h[j]));}if r==self.n{continue}let b=self.product(a,w);if let Some(e)=roots.iter_mut().find(|e|e.0==r){e.1=self.f.add(e.1,b)}else{roots.push((r,b));}}roots.retain(|x|x.1!=0);
 if roots.is_empty(){self.labels.insert(&q[..2],self.f.neg(q[2]));return}
 if roots.len()==2 && self.size[roots[0].0]>self.size[roots[1].0]{roots.swap(0,1)}
 let(root,a)=roots[0];let inv=self.f.inv(a);let mut h=[0;3];for j in 0..3{h[j]=self.f.neg(self.product(q[j],inv));}
 if roots.len()==1{self.parent[root]=self.n;self.weight[root]=0;self.shift[root]=h;}else{let(other,b)=roots[1];self.parent[root]=other;self.weight[root]=self.f.neg(self.product(b,inv));self.shift[root]=h;self.size[other]+=self.size[root];}
 self.lp_rank+=1;
 }
}

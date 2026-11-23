include!("interval.rs");
/// Descending coefficients c_(e+j*m) of (X-1)^h. Prime r is caller-certified.
struct CoeffStream{r:u64,h:u64,m:u64,index:u64,value:u64,left:usize,cap:usize,products:IntervalProducts}
impl CoeffStream{
 fn new(r:u64,m:usize,e:usize,cap:usize)->Result<Self,Error>{
  if r<7||r>u64::MAX/2||m<2||m as u64>=r||(r-1)%m as u64!=0||e>=m.min(3)||cap==0{return Err(Error::Domain)}
  let h=(r-1)/2;let left=((h-e as u64)/m as u64+1)as usize;let index=e as u64+(left as u64-1)*m as u64;let q=(h-index).min(index);
  let mut products=IntervalProducts::new(r,m as u64,cap)?;let num=products.product(h-q+1,q)?;let den=products.product(1,q)?;
  let mut value=if q==0{1}else{let inv=pow(den,r-2,r);if mul(den,inv,r)!=1{return Err(Error::Arithmetic)}mul(num,inv,r)};
  if (h-index)%2!=0{value=(r-value)%r}Ok(Self{r,h,m:m as u64,index,value,left,cap,products})
 }
 fn take_desc(&mut self,count:usize)->Result<Vec<u64>,Error>{
  if count==0||count>self.left||count>self.cap{return Err(Error::Limit)}
  let steps=count.min(self.left-1);let mut ns=Vec::with_capacity(steps);let mut ds=Vec::with_capacity(steps);let mut prefix=vec![1];let mut index=self.index;
  for _ in 0..steps{let mut num=self.products.product(index-self.m+1,self.m)?;let den=self.products.product(self.h-index+1,self.m)?;
   if self.m%2!=0{num=(self.r-num)%self.r}if den==0{return Err(Error::Arithmetic)}ns.push(num);ds.push(den);prefix.push(mul(*prefix.last().unwrap(),den,self.r));index-=self.m;
  }
  if steps>0{let mut inv=pow(prefix[steps],self.r-2,self.r);if mul(inv,prefix[steps],self.r)!=1{return Err(Error::Arithmetic)}
   for i in(0..steps).rev(){let den=ds[i];ds[i]=mul(inv,prefix[i],self.r);inv=mul(inv,den,self.r)}
  }
  let mut out=Vec::with_capacity(count);for i in 0..count{out.push(self.value);if i<steps{self.value=mul(mul(self.value,ns[i],self.r),ds[i],self.r);self.index-=self.m;}}
  self.left-=count;Ok(out)
 }
}
/// No coefficient vector or polynomial tree is stored here.
struct StreamModes{r:u64,m:usize,zeta:u64,block:usize}
impl StreamModes{
 fn new(r:u64,m:usize,zeta:u64,max_m:usize,max_coeff:usize,block:usize)->Result<Self,Error>{
  if r<7||r>u64::MAX/2||m<2||m as u64>=r||(r-1)%m as u64!=0||zeta==0||zeta>=r||block==0{return Err(Error::Domain)}
  if m>max_m||((r-1)/2/m as u64+1)as usize>max_coeff{return Err(Error::Limit)}
  let mut z=1;for j in 1..=m{z=mul(z,zeta,r);if z==1&&j!=m{return Err(Error::Domain)}}if z!=1{return Err(Error::Domain)}
  Ok(Self{r,m,zeta,block})
 }
 fn stream(&self,e:usize)->Result<CoeffStream,Error>{CoeffStream::new(self.r,self.m,e,self.block)}
 fn observed(&self,signs:&[u64])->Result<Vec<u64>,Error>{
  if signs.len()!=self.m||signs.iter().any(|&s|s!=0&&s!=1&&s!=self.r-1){return Err(Error::Domain)}
  let h=(self.r-1)/2;Ok((0..self.m.min(3)).map(|e|{let root=pow(self.zeta,(e as u64+self.m as u64-h%self.m as u64)%self.m as u64,self.r);let(mut w,mut sum)=(1,0);for &s in signs{sum=(sum+mul(s,w,self.r))%self.r;w=mul(w,root,self.r)}sum}).collect())
 }
 fn evaluate(&self,e:usize,x:u64)->Result<u64,Error>{
  if x>=self.r{return Err(Error::Domain)}let mut s=self.stream(e)?;let mut out=0;
  while s.left>0{for c in s.take_desc(s.left.min(self.block))?{out=(mul(out,x,self.r)+c)%self.r}}Ok(out)
 }
 fn phase(&self,v:u64,b:u64,f1:u64)->Result<Option<u64>,Error>{
  let r=self.r;if v>=r||b==0||b>=r||f1>=r{return Err(Error::Domain)}let a=mul(self.m as u64,pow(b,(r-1)/2,r),r);let den=mul(a,self.evaluate(1,v)?,r);if den==0{return Ok(None)}let x=mul(f1,pow(den,r-2,r),r);if pow(x,self.m as u64,r)!=v{return Err(Error::Arithmetic)}Ok(Some(x))
 }
 fn residue(&self,p:&mut polynomial::Poly,e:usize,t:u64,g:&[u64])->Result<Vec<u64>,Error>{
  let r=self.r;if g.len()<2||g.last()!=Some(&1)||g.iter().any(|&x|x>=r)||t==0||t>=r{return Err(Error::Domain)}
  let n=g.len()-1;let w=n.min(self.block);let mut powers=Vec::with_capacity(w);let mut tw=1;for _ in 0..w{powers.push(tw);tw=mul(tw,t,r)}
  let step=if w==n{g[..n].iter().map(|&x|mul((r-x)%r,tw,r)).collect::<Vec<_>>()}else{let mut v=vec![0;w+1];v[w]=tw;v};
  let mut s=self.stream(e)?;let mut acc=vec![0];let mut first=true;
  while s.left>0{
   let count=if first{(s.left-1)%w+1}else{w};let chunk=s.take_desc(count)?;
   if !first{let product=p.mul(&acc,&step);assert!(product.len()<=2*n-1);acc=p.divrem(&product,g).map_err(|_|Error::Arithmetic)?.1;}first=false;
   if acc.len()<count{acc.resize(count,0)}for (i,&c) in chunk.iter().rev().enumerate(){acc[i]=(acc[i]+mul(c,powers[i],r))%r}
   while acc.len()>1&&acc.last()==Some(&0){acc.pop();}
  }Ok(acc)
 }
 fn first_c0(&self,a:u64,t:u64,f0:u64)->Result<Vec<u64>,Error>{
  let mut s=self.stream(0)?;let mut out=vec![0;s.left];let mut at=out.len();
  while s.left>0{for c in s.take_desc(s.left.min(self.block))?{at-=1;out[at]=c;}}
  let mut scale=a;for c in &mut out{*c=mul(*c,scale,self.r);scale=mul(scale,t,self.r)}out[0]=(out[0]+self.r-f0)%self.r;Ok(out)
 }
 fn intersect_modes(&self,p:&mut polynomial::Poly,state:&mut ImageDomain,b:u64,f:&[u64])->Result<(),Error>{
  let r=self.r;if state.r!=r||state.m!=self.m||b==0||b>=r||f.len()!=self.m.min(3)||f.iter().any(|&x|x>=r){return Err(Error::Domain)}if state.degree()==0{return Ok(())}
  let a=mul(self.m as u64,pow(b,(r-1)/2,r),r);let t=pow(pow(b,self.m as u64,r),r-2,r);
  let c0=if let Some(g)=&state.dense{let mut c=self.residue(p,0,t,g)?;for x in &mut c{*x=mul(*x,a,r)}c[0]=(c[0]+r-f[0])%r;c}else{self.first_c0(a,t,f[0])?};
  state.intersect(p,&c0)?;drop(c0);if self.m<3||state.degree()==0{return Ok(())}
  let c2=if let Some(g)=&state.dense{let u=self.residue(p,1,t,g)?;let mut v=self.residue(p,2,t,g)?;let square=p.mul(&u,&u);assert!(square.len()<=2*(g.len()-1)-1);let mut square=p.divrem(&square,g).map_err(|_|Error::Arithmetic)?.1;
   let left=mul(f[1],f[1],r);let right=mul(a,f[2],r);for x in &mut v{*x=mul(*x,left,r)}for x in &mut square{*x=mul(*x,right,r)}p.sub(&v,&square)
  }else{if self.m as u64!=r-1{return Err(Error::Arithmetic)}let u=self.evaluate(1,0)?;let v=self.evaluate(2,0)?;vec![(mul(mul(f[1],f[1],r),v,r)+r-mul(mul(a,f[2],r),mul(u,u,r),r))%r]};state.intersect(p,&c2)
 }
}
include!("stream-checks.rs");

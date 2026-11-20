#[derive(Clone)]struct SplitValue{both:Option<Quad>,one:Option<Vec<u64>>}
struct SplitRing{r:u64,both:Option<QuadRing>,one:Vec<u64>,root:Vec<u64>,p:polynomial::Poly,reads:usize,pair_products:usize,scalar_products:usize}
impl SplitRing{
 fn new(s:&CanonicalFiber)->Result<Self,Error>{Ok(Self{r:s.r,both:if s.both.len()>1{Some(QuadRing::new(s.r,&s.both)?)}else{None},one:s.one.clone(),root:s.root.clone(),p:polynomial::Poly::new(s.r).map_err(|_|Error::Domain)?,reads:0,pair_products:0,scalar_products:0})}
 fn constant(&self,c:u64)->SplitValue{SplitValue{both:self.both.as_ref().map(|_|Quad([vec![c],vec![0]])),one:if self.one.len()>1{Some(vec![c])}else{None}}}
 fn base(&self,scale:u64)->SplitValue{SplitValue{both:self.both.as_ref().map(|_|Quad([vec![0],vec![scale]])),one:if self.one.len()>1{Some(self.root.iter().map(|&x|mul(x,scale,self.r)).collect())}else{None}}}
 fn scale(&self,v:&SplitValue,c:u64)->SplitValue{SplitValue{both:v.both.as_ref().map(|q|Quad(std::array::from_fn(|i|q.0[i].iter().map(|&x|mul(x,c,self.r)).collect()))),one:v.one.as_ref().map(|a|a.iter().map(|&x|mul(x,c,self.r)).collect())}}
 fn subtract_constant(&self,v:&mut SplitValue,c:u64){if let Some(q)=&mut v.both{q.0[0][0]=(q.0[0][0]+self.r-c)%self.r;}if let Some(a)=&mut v.one{a[0]=(a[0]+self.r-c)%self.r;}}
 fn add(&mut self,a:&SplitValue,b:&SplitValue)->SplitValue{SplitValue{both:a.both.as_ref().zip(b.both.as_ref()).map(|(a,b)|self.both.as_mut().unwrap().add(a,b)),one:a.one.as_ref().zip(b.one.as_ref()).map(|(a,b)|self.p.add(a,b))}}
 fn sub(&mut self,a:&SplitValue,b:&SplitValue)->SplitValue{SplitValue{both:a.both.as_ref().zip(b.both.as_ref()).map(|(a,b)|self.both.as_mut().unwrap().sub(a,b)),one:a.one.as_ref().zip(b.one.as_ref()).map(|(a,b)|self.p.sub(a,b))}}
 fn multiply(&mut self,a:&SplitValue,b:&SplitValue)->Result<SplitValue,Error>{
  let both=if let Some(ring)=&mut self.both{self.pair_products+=1;Some(ring.multiply(a.both.as_ref().ok_or(Error::Domain)?,b.both.as_ref().ok_or(Error::Domain)?)?)}else{None};
  let one=if self.one.len()>1{self.scalar_products+=1;let c=self.p.mul(a.one.as_ref().ok_or(Error::Domain)?,b.one.as_ref().ok_or(Error::Domain)?);Some(canonical_rem(&mut self.p,&c,&self.one)?)}else{None};Ok(SplitValue{both,one})
 }
 fn block(&mut self,reader:&mut CoefficientReader<'_>,length:usize,base:&SplitValue)->Result<(SplitValue,SplitValue),Error>{
  if length==0{return Err(Error::Domain)}if length==1{let c=reader.next()?;self.reads+=1;return Ok((self.constant(c),base.clone()))}
  let(a,ap)=self.block(reader,length/2,base)?;let(b,bp)=self.block(reader,length-length/2,base)?;let ab=self.multiply(&a,&bp)?;let value=self.add(&ab,&b);let power=self.multiply(&ap,&bp)?;Ok((value,power))
 }
 fn mode(&mut self,modes:&RecoveryModes,e:usize,scale:u64)->Result<SplitValue,Error>{if modes.r!=self.r||scale>=self.r{return Err(Error::Domain)}let stream=modes.stream(e)?;let length=stream.left;let mut reader=CoefficientReader{stream,buffer:Vec::new(),at:0,remaining:length};let base=self.base(scale);Ok(self.block(&mut reader,length,&base)?.0)}
}
impl CanonicalBranch{
 fn keep_split(&mut self,eq:SplitValue)->Result<(),Error>{
  let s=&mut self.canonical;let p=&mut self.ring.p;let r=s.r;
  if eq.both.is_some()!=(s.both.len()>1)||eq.one.is_some()!=(s.one.len()>1){return Err(Error::Domain)}
  let old_one=if let Some(v)=eq.one{p.gcd_monic(&s.one,&v,s.one.len()+v.len()+2).map_err(|_|Error::Arithmetic)?.polynomial}else{vec![1]};let old_root=canonical_rem(p,&s.root,&old_one)?;
  let mut fresh=CanonicalFiber{r,both:s.both.clone(),one:vec![1],root:vec![0],inverses:0,crt_merges:0};if let Some(q)=eq.both{fresh.keep(p,&q)?;}
  s.inverses+=fresh.inverses;s.crt_merges+=fresh.crt_merges;
  let(root,one)=if old_one.len()==1{(fresh.root,fresh.one)}else if fresh.one.len()==1{(old_root,old_one)}else{ s.inverses+=1;s.crt_merges+=1;let inv=canonical_inverse(p,&old_one,&fresh.one,r)?;let delta=p.sub(&fresh.root,&old_root);let product=p.mul(&delta,&inv);let correction=canonical_rem(p,&product,&fresh.one)?;let shifted=p.mul(&old_one,&correction);(p.add(&old_root,&shifted),p.mul(&old_one,&fresh.one))};
  s.both=fresh.both;s.one=one;s.root=root;self.ring.g=s.support(p);Ok(())
 }
}
include!("split-image.rs");
include!("split-adapter.rs");
include!("split-checks.rs");

include!("reused.rs");

std::thread_local!{static POWERED_EVAL:std::cell::Cell<(usize,usize,usize)>=const{std::cell::Cell::new((0,0,0))};}
impl QuadRing{
 fn section_product(&mut self,a:&[Quad],b:&[Quad],v:&Quad)->Result<Vec<Quad>,Error>{
  let m=a.len();if b.len()!=m||m==0{return Err(Error::Domain)}let mut out=vec![Quad([vec![0],vec![0]]);m];
  for i in 0..m{for j in 0..m{let mut c=self.multiply(&a[i],&b[j])?;let k=i+j;if k>=m{c=self.multiply(&c,v)?;}out[k%m]=self.add(&out[k%m],&c);}}Ok(out)
 }
 fn sections(&mut self,m:usize,v:&Quad,slot_cap:usize)->Result<Vec<Quad>,Error>{
  if m<2||m as u64>=self.r||self.g.len()<2||v.0.iter().any(|x|x.is_empty()||x.iter().any(|&a|a>=self.r)){return Err(Error::Domain)}
  let slots=m.checked_mul(2).and_then(|x|x.checked_mul(self.g.len()-1)).ok_or(Error::Limit)?;if slots>slot_cap{return Err(Error::Limit)}
  let v=Quad([self.rem(&v.0[0])?,self.rem(&v.0[1])?]);
  let mut acc=vec![Quad([vec![0],vec![0]]);m];acc[0]=Quad([vec![1],vec![0]]);
  let mut base=vec![Quad([vec![0],vec![0]]);m];base[0]=Quad([vec![self.r-1],vec![0]]);base[1]=Quad([vec![1],vec![0]]);
  let(mut n,mut products)=((self.r-1)/2,0);
  while n>0{if n&1!=0{acc=self.section_product(&acc,&base,&v)?;products+=1;}n>>=1;if n>0{base=self.section_product(&base,&base,&v)?;products+=1;}}
  POWERED_EVAL.with(|c|{let(a,b,d)=c.get();c.set((a+1,b+products,d))});Ok(acc)
 }
}
std::thread_local!{static POWERED_PHASE:std::cell::Cell<(usize,usize)>=const{std::cell::Cell::new((0,0))};}
impl RecoveryModes<'_>{
 fn phase_powered(&self,v:u64,b:u64,f1:u64,slots:usize)->Result<Option<u64>,Error>{
  let r=self.r;if v>=r||b==0||b>=r||f1>=r{return Err(Error::Domain)}
  // Constant embedding into a degree-one W quotient; no inverse of v is needed.
  let mut ring=QuadRing::new(r,&[0,1])?;
  let values=match ring.sections(self.m,&Quad([vec![v],vec![0]]),slots){Ok(x)=>x,Err(Error::Limit)=>{POWERED_PHASE.with(|c|{let(a,b)=c.get();c.set((a,b+1))});return self.phase(v,b,f1)},Err(e)=>return Err(e)};
  if values[1].0[0].len()!=1||values[1].0[1]!=[0]{return Err(Error::Arithmetic)}
  POWERED_PHASE.with(|c|{let(a,b)=c.get();c.set((a+1,b))});
  let a=mul(self.m as u64,pow(b,(r-1)/2,r),r);let den=mul(a,values[1].0[0][0],r);if den==0{return Ok(None)}
  let x=mul(f1,pow(den,r-2,r),r);if pow(x,self.m as u64,r)!=v{return Err(Error::Arithmetic)}Ok(Some(x))
 }
}

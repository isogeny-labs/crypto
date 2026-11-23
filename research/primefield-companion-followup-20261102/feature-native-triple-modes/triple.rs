struct TripleReader<'a>{anchor:SharedCoeffStream<'a>,buffer:Vec<[u64;3]>,at:usize,remaining:usize,reads:usize,batches:usize,transitions:usize,max_denominators:usize,max_buffer:usize}
impl<'a> TripleReader<'a>{
 fn new(modes:&'a RecoveryModes)->Result<Self,Error>{let h=(modes.r-1)/2;if modes.m<3||h%modes.m as u64!=0{return Err(Error::Domain)}let anchor=modes.stream(1)?;let remaining=anchor.left;Ok(Self{anchor,buffer:Vec::new(),at:0,remaining,reads:0,batches:0,transitions:0,max_denominators:0,max_buffer:0})}
 fn refill(&mut self)->Result<(),Error>{
  let count=self.remaining.min(self.anchor.cap);if count==0||self.at!=self.buffer.len()||count>self.anchor.left{return Err(Error::Limit)}
  let steps=count.min(self.anchor.left-1);let(r,h,m,start)=(self.anchor.r,self.anchor.h,self.anchor.m,self.anchor.index);
  let span=(count as u64-1).checked_mul(m).ok_or(Error::Domain)?;if start<=span||start>=h{return Err(Error::Domain)}
  let mut requests=Vec::with_capacity(2*steps);let mut index=start;for _ in 0..steps{requests.push((index-m+1,m));requests.push((h-index+1,m));index-=m;}
  let values=self.anchor.products.products(&requests)?;let mut ds=Vec::with_capacity(steps+2*count);
  for pair in values.chunks_exact(2){ds.push(pair[1]);}for j in 0..count{let n=start-j as u64*m;ds.push(h-n+1);ds.push(n+1);}
  if ds.iter().any(|&d|d==0||d>=r){return Err(Error::Arithmetic)}let mut prefix=vec![1];for &d in &ds{prefix.push(mul(*prefix.last().unwrap(),d,r));}
  let inv=pow(*prefix.last().unwrap(),r-2,r);if mul(inv,*prefix.last().unwrap(),r)!=1{return Err(Error::Arithmetic)}finish_batch_inverses(&mut ds,&prefix,inv,r)?;
  let mut out=Vec::with_capacity(count);let(mut value,mut n)=(self.anchor.value,start);
  for j in 0..count{let lower=mul(mul(value,n,r),ds[steps+2*j],r);let upper=mul(mul(value,h-n,r),ds[steps+2*j+1],r);out.push([(r-lower)%r,value,(r-upper)%r]);
   if j<steps{let numerator=if m%2==0{values[2*j]}else{(r-values[2*j])%r};value=mul(mul(value,numerator,r),ds[j],r);n-=m;}}
  self.anchor.value=value;self.anchor.index=n;self.anchor.left-=count;self.buffer=out;self.at=0;self.batches+=1;self.transitions+=steps;self.max_denominators=self.max_denominators.max(ds.len());self.max_buffer=self.max_buffer.max(count);Ok(())
 }
 fn next(&mut self)->Result<[u64;3],Error>{if self.remaining==0{return Err(Error::Limit)}if self.at==self.buffer.len(){self.refill()?;}let c=self.buffer[self.at];self.at+=1;self.remaining-=1;self.reads+=1;Ok(c)}
}
fn triple_block(rings:&mut[ReusedRing],reader:&mut TripleReader<'_>,length:usize)->Result<Vec<[SplitValue;3]>,Error>{
 if length==0{return Err(Error::Domain)}if length==1{let c=reader.next()?;return Ok(rings.iter_mut().map(|ring|{ring.inner.reads+=3;std::array::from_fn(|e|ring.constant(c[e]))}).collect())}
 let left=triple_block(rings,reader,length/2)?;let right=triple_block(rings,reader,length-length/2)?;let mut out=Vec::with_capacity(rings.len());
 for((ring,a),b)in rings.iter_mut().zip(left).zip(right){let power=ring.cached_power(length-length/2)?;let mut v=std::array::from_fn(|_|ring.constant(0));for e in 0..3{let product=ring.multiply(&a[e],&power)?;v[e]=ring.add(&product,&b[e]);ring.value_products+=1;}out.push(v);}Ok(out)
}
fn triple_modes(rings:&mut[ReusedRing],modes:&RecoveryModes,scale:u64)->Result<Vec<[SplitValue;3]>,Error>{
 if rings.len()>4||scale>=modes.r||rings.iter().any(|r|r.r!=modes.r){return Err(Error::Domain)}if rings.is_empty(){return Ok(Vec::new())}
 let mut reader=TripleReader::new(modes)?;let length=reader.remaining;for ring in rings.iter_mut(){ring.prepare(scale)?;}
 let mut out=triple_block(rings,&mut reader,length)?;if reader.reads!=length||reader.remaining!=0{return Err(Error::Arithmetic)}
 for(ring,v)in rings.iter_mut().zip(&mut out){let leading=ring.cached_power(length)?;v[0]=ring.add(&v[0],&leading);ring.inner.reads+=1;}Ok(out)
}
impl DirectImage{
 fn update_triple(&mut self,modes:&RecoveryModes,b:u64,f:&[u64])->Result<(),Error>{
  let r=self.r;if modes.r!=r||modes.m!=self.m||b==0||b>=r||f.len()!=3||f.iter().any(|&x|x>=r){return Err(Error::Domain)}if self.rounds>=4{return Err(Error::Limit)}
  if ((r-1)/2)%self.m as u64!=0{return self.update(modes,b,f)}
  let h=(r-1)/2;let a=mul(self.m as u64,pow(b,h,r),r);let t=pow(pow(b,self.m as u64,r),r-2,r);let scale=mul(t,self.inv_t0,r);let left=mul(f[1],f[1],r);let right=mul(a,f[2],r);let c1=r-h;let c2=mul(mul(h,h-1,r),(r+1)/2,r);
  self.zero=self.zero&&f[0]==a&&mul(left,c2,r)==mul(right,mul(c1,c1,r),r);
  let(indices,mut rings)=self.contexts()?;let values=triple_modes(&mut rings,modes,scale)?;
  for((j,ring),v)in indices.into_iter().zip(rings.iter_mut()).zip(values){let mut c0=ring.scale(&v[0],a);ring.subtract_constant(&mut c0,f[0]);self.keep(j,c0)?;
   if self.branches[j].g.len()>1{let square=ring.multiply(&v[1],&v[1])?;let lhs=ring.scale(&v[2],left);let rhs=ring.scale(&square,right);self.keep(j,ring.sub(&lhs,&rhs))?;}}
  self.rounds+=1;DIRECT_COUNTS.with(|c|{let(a,b,d,e)=c.get();c.set((a,b+1,d,e))});Ok(())
 }
}
include!("triple-adapter.rs");

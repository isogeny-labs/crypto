fn counted_fused_mul(a:u64,b:u64,r:u64,count:&mut usize)->u64{*count+=1;mul(a,b,r)}
fn fused_invert(r:u64,denominators:&[u64],count:&mut usize)->Result<Vec<u64>,Error>{
 if r<7||r>u64::MAX/2||denominators.is_empty()||denominators.iter().any(|&d|d==0||d>=r){return Err(Error::Domain)}
 let mut prefix=vec![1];for &d in denominators{prefix.push(counted_fused_mul(*prefix.last().unwrap(),d,r,count));}
 let mut inverse=pow(*prefix.last().unwrap(),r-2,r);if counted_fused_mul(inverse,*prefix.last().unwrap(),r,count)!=1{return Err(Error::Arithmetic)}
 let mut out=vec![0;denominators.len()];for j in(0..denominators.len()).rev(){out[j]=counted_fused_mul(inverse,prefix[j],r,count);inverse=counted_fused_mul(inverse,denominators[j],r,count);}Ok(out)
}
struct FusedReader<'a>{anchor:SharedCoeffStream<'a>,buffer:Vec<[u64;2]>,at:usize,remaining:usize,reads:usize,batches:usize,transitions:usize,max_pairs:usize,multiplications:usize,removed_inversions:usize,max_denominators:usize}
impl<'a> FusedReader<'a>{
 fn new(modes:&'a RecoveryModes)->Result<Self,Error>{let h=(modes.r-1)/2;if modes.m<3||h<2||(h-1)/modes.m as u64!=(h-2)/modes.m as u64{return Err(Error::Domain)}let anchor=modes.stream(1)?;let remaining=anchor.left;Ok(Self{anchor,buffer:Vec::new(),at:0,remaining,reads:0,batches:0,transitions:0,max_pairs:0,multiplications:0,removed_inversions:0,max_denominators:0})}
 fn refill(&mut self)->Result<(),Error>{
  let count=self.remaining.min(self.anchor.cap);if count==0||self.at!=self.buffer.len()||count>self.anchor.left{return Err(Error::Limit)}let steps=count.min(self.anchor.left-1);let(r,h,m,start)=(self.anchor.r,self.anchor.h,self.anchor.m,self.anchor.index);
  let span=(count as u64-1).checked_mul(m).ok_or(Error::Domain)?;if start<=span||start>=h{return Err(Error::Domain)}
  let mut requests=Vec::with_capacity(2*steps);let mut index=start;for _ in 0..steps{requests.push((index-m+1,m));requests.push((h-index+1,m));index-=m;}
  let values=self.anchor.products.products(&requests)?;let mut denominators=Vec::with_capacity(steps+count);for pair in values.chunks_exact(2){denominators.push(pair[1]);}for j in 0..count{denominators.push(start-j as u64*m+1);}
  let before=self.multiplications;let inverse=fused_invert(r,&denominators,&mut self.multiplications)?;let mut out=Vec::with_capacity(count);let(mut value,mut index)=(self.anchor.value,start);
  for j in 0..count{let ratio=counted_fused_mul(value,h-index,r,&mut self.multiplications);let next=counted_fused_mul(ratio,inverse[steps+j],r,&mut self.multiplications);out.push([value,(r-next)%r]);if j<steps{let num=if m%2==0{values[2*j]}else{(r-values[2*j])%r};value=counted_fused_mul(value,num,r,&mut self.multiplications);value=counted_fused_mul(value,inverse[j],r,&mut self.multiplications);index-=m;}}
  if self.multiplications-before!=5*(steps+count)+1{return Err(Error::Arithmetic)}self.anchor.value=value;self.anchor.index=index;self.anchor.left-=count;self.buffer=out;self.at=0;self.batches+=1;self.transitions+=steps;self.max_pairs=self.max_pairs.max(count);self.removed_inversions+=usize::from(steps>0);self.max_denominators=self.max_denominators.max(denominators.len());Ok(())
 }
 fn next(&mut self)->Result<[u64;2],Error>{if self.remaining==0{return Err(Error::Limit)}if self.at==self.buffer.len(){self.refill()?;}let out=self.buffer[self.at];self.at+=1;self.remaining-=1;self.reads+=1;Ok(out)}
}
include!("fused-block.rs");
// Successful paired evaluations: calls, coefficients, transitions, inversions,
// counted multiplications outside inverse exponentiation, removed inversions, max denominators.
std::thread_local!{static FUSED_WORK:std::cell::Cell<(usize,usize,usize,usize,usize,usize,usize)>=const{std::cell::Cell::new((0,0,0,0,0,0,0))};}
fn fused_modes(rings:&mut[ReusedRing],modes:&RecoveryModes,scale:u64)->Result<(Vec<SplitValue>,Vec<SplitValue>),Error>{
 if rings.len()>4||scale>=modes.r||modes.m<3||rings.iter().any(|r|r.r!=modes.r){return Err(Error::Domain)}if rings.is_empty(){return Ok((Vec::new(),Vec::new()))}let mut reader=FusedReader::new(modes)?;let length=reader.remaining;for ring in rings.iter_mut(){ring.prepare(scale)?;}
 let out=fused_block(rings,&mut reader,length)?;if reader.reads!=length||reader.remaining!=0{return Err(Error::Arithmetic)}FUSED_WORK.with(|c|{let(a,b,d,e,f,g,h)=c.get();c.set((a+1,b+reader.reads,d+reader.transitions,e+reader.batches,f+reader.multiplications,g+reader.removed_inversions,h.max(reader.max_denominators)))});Ok(out)
}
include!("fused-image.rs");
include!("fused-adapter.rs");
include!("fused-checks.rs");

include!("direct.rs");

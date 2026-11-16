fn adjacent_coefficients(r:u64,h:u64,m:u64,start:u64,anchor:&[u64])->Result<Vec<[u64;2]>,Error>{
 if r<7||r>u64::MAX/2||h!=(r-1)/2||m<3||m>=r||anchor.is_empty()||anchor.iter().any(|&x|x>=r){return Err(Error::Domain)}
 let span=(anchor.len()as u64-1).checked_mul(m).ok_or(Error::Domain)?;if start<=span||start>=h{return Err(Error::Domain)}
 let mut prefix=vec![1];for j in 0..anchor.len(){let n=start-j as u64*m;prefix.push(mul(*prefix.last().unwrap(),n+1,r));}
 let mut inverse=pow(*prefix.last().unwrap(),r-2,r);if mul(inverse,*prefix.last().unwrap(),r)!=1{return Err(Error::Arithmetic)}
 let mut out=vec![[0,0];anchor.len()];for j in(0..anchor.len()).rev(){let n=start-j as u64*m;let inv=mul(inverse,prefix[j],r);inverse=mul(inverse,n+1,r);let next=mul(mul(anchor[j],h-n,r),inv,r);out[j]=[anchor[j],(r-next)%r];}Ok(out)
}
struct NeighborReader<'a>{anchor:SharedCoeffStream<'a>,buffer:Vec<[u64;2]>,at:usize,remaining:usize,reads:usize,batches:usize,transitions:usize,max_pairs:usize}
impl<'a> NeighborReader<'a>{
 fn new(modes:&'a RecoveryModes)->Result<Self,Error>{let h=(modes.r-1)/2;if modes.m<3||h<2{return Err(Error::Domain)}let l1=(h-1)/modes.m as u64+1;let l2=(h-2)/modes.m as u64+1;if l1!=l2{return Err(Error::Domain)}let anchor=modes.stream(1)?;let remaining=anchor.left;Ok(Self{anchor,buffer:Vec::new(),at:0,remaining,reads:0,batches:0,transitions:0,max_pairs:0})}
 fn next(&mut self)->Result<[u64;2],Error>{if self.remaining==0{return Err(Error::Limit)}if self.at==self.buffer.len(){let n=self.remaining.min(self.anchor.cap);let start=self.anchor.index;let steps=n.min(self.anchor.left-1);let values=self.anchor.take_desc(n)?;self.buffer=adjacent_coefficients(self.anchor.r,self.anchor.h,self.anchor.m,start,&values)?;self.at=0;self.batches+=1;self.transitions+=steps;self.max_pairs=self.max_pairs.max(n);}
  let out=self.buffer[self.at];self.at+=1;self.remaining-=1;self.reads+=1;Ok(out)
 }
}
// Successful paired evaluations: calls, anchor coefficients, anchor stride transitions,
// adjacent batch inversions, adjacent multiplications outside exponentiation, max buffered pairs.
std::thread_local!{static NEIGHBOR_WORK:std::cell::Cell<(usize,usize,usize,usize,usize,usize)>=const{std::cell::Cell::new((0,0,0,0,0,0))};}
fn neighbor_block(rings:&mut[ReusedRing],reader:&mut NeighborReader<'_>,length:usize)->Result<(Vec<SplitValue>,Vec<SplitValue>),Error>{
 if length==0{return Err(Error::Domain)}if length==1{let c=reader.next()?;let mut a=Vec::new();let mut b=Vec::new();for ring in rings{ring.inner.reads+=2;a.push(ring.constant(c[0]));b.push(ring.constant(c[1]));}return Ok((a,b))}
 let(a1,a2)=neighbor_block(rings,reader,length/2)?;let(b1,b2)=neighbor_block(rings,reader,length-length/2)?;let mut out1=Vec::with_capacity(rings.len());let mut out2=Vec::with_capacity(rings.len());
 for j in 0..rings.len(){let ring=&mut rings[j];let power=ring.cached_power(length-length/2)?;let x=ring.multiply(&a1[j],&power)?;out1.push(ring.add(&x,&b1[j]));let y=ring.multiply(&a2[j],&power)?;out2.push(ring.add(&y,&b2[j]));ring.value_products+=2;}Ok((out1,out2))
}
fn neighbor_modes(rings:&mut[ReusedRing],modes:&RecoveryModes,scale:u64)->Result<(Vec<SplitValue>,Vec<SplitValue>),Error>{
 if rings.len()>4||scale>=modes.r||modes.m<3||rings.iter().any(|r|r.r!=modes.r){return Err(Error::Domain)}if rings.is_empty(){return Ok((Vec::new(),Vec::new()))}let mut reader=NeighborReader::new(modes)?;let length=reader.remaining;for ring in rings.iter_mut(){ring.prepare(scale)?;}
 let out=neighbor_block(rings,&mut reader,length)?;if reader.reads!=length||reader.remaining!=0{return Err(Error::Arithmetic)}
 NEIGHBOR_WORK.with(|c|{let(a,b,d,e,f,g)=c.get();c.set((a+1,b+reader.reads,d+reader.transitions,e+reader.batches,f+5*reader.reads+reader.batches,g.max(reader.max_pairs)))});Ok(out)
}
include!("neighbor-image.rs");
include!("neighbor-adapter.rs");
include!("neighbor-checks.rs");

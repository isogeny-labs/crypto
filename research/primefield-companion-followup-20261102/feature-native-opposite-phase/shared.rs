// Completed mode calls only: streams, reader-delivered coefficients, equivalent separate
// streams, equivalent separate coefficients, max branch count, summed cache slots.
std::thread_local!{static SHARED_STREAMS:std::cell::Cell<(usize,usize,usize,usize,usize,usize)>=const{std::cell::Cell::new((0,0,0,0,0,0))};}
fn shared_block(rings:&mut[ReusedRing],reader:&mut CoefficientReader<'_>,length:usize,reads:&mut usize)->Result<Vec<SplitValue>,Error>{
 if length==0{return Err(Error::Domain)}if length==1{let c=reader.next()?;*reads+=1;return Ok(rings.iter_mut().map(|ring|{ring.inner.reads+=1;ring.inner.constant(c)}).collect())}
 let left=shared_block(rings,reader,length/2,reads)?;let right=shared_block(rings,reader,length-length/2,reads)?;let mut out=Vec::with_capacity(rings.len());
 for((ring,a),b)in rings.iter_mut().zip(&left).zip(&right){let power=ring.cached_power(length-length/2)?;let product=ring.multiply(a,&power)?;ring.value_products+=1;out.push(ring.add(&product,b));}Ok(out)
}
fn shared_mode(rings:&mut[ReusedRing],modes:&RecoveryModes,e:usize,scale:u64)->Result<Vec<SplitValue>,Error>{
 if rings.len()>4||scale>=modes.r||rings.iter().any(|r|r.r!=modes.r){return Err(Error::Domain)}if rings.is_empty(){return Ok(Vec::new())}
 let stream=modes.stream(e)?;let length=stream.left;let mut reader=CoefficientReader{stream,buffer:Vec::new(),at:0,remaining:length};for ring in rings.iter_mut(){ring.prepare(scale)?;}
 let mut reads=0;let values=shared_block(rings,&mut reader,length,&mut reads)?;if reads!=length{return Err(Error::Arithmetic)}
 let cache=rings.iter().map(ReusedRing::cache_slots).sum();SHARED_STREAMS.with(|c|{let(a,b,d,f,g,h)=c.get();c.set((a+1,b+reads,d+rings.len(),f+reads*rings.len(),g.max(rings.len()),h.max(cache)))});Ok(values)
}
include!("shared-image.rs");
include!("shared-adapter.rs");
include!("shared-checks.rs");

include!("neighbor.rs");

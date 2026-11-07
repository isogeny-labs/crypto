fn fused_block(rings:&mut[ReusedRing],reader:&mut FusedReader<'_>,length:usize)->Result<(Vec<SplitValue>,Vec<SplitValue>),Error>{
 if length==0{return Err(Error::Domain)}if length==1{let c=reader.next()?;let mut a=Vec::new();let mut b=Vec::new();for ring in rings{ring.inner.reads+=2;a.push(ring.constant(c[0]));b.push(ring.constant(c[1]));}return Ok((a,b))}
 let(a1,a2)=fused_block(rings,reader,length/2)?;let(b1,b2)=fused_block(rings,reader,length-length/2)?;let mut out1=Vec::with_capacity(rings.len());let mut out2=Vec::with_capacity(rings.len());
 for j in 0..rings.len(){let ring=&mut rings[j];let power=ring.cached_power(length-length/2)?;let x=ring.multiply(&a1[j],&power)?;out1.push(ring.add(&x,&b1[j]));let y=ring.multiply(&a2[j],&power)?;out2.push(ring.add(&y,&b2[j]));ring.value_products+=2;}Ok((out1,out2))
}

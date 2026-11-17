struct ReusedRing{inner:SplitRing,powers:std::collections::BTreeMap<usize,SplitValue>,scale_key:Option<u64>,power_products:usize,value_products:usize,resets:usize}
impl std::ops::Deref for ReusedRing{type Target=SplitRing;fn deref(&self)->&SplitRing{&self.inner}}
impl std::ops::DerefMut for ReusedRing{fn deref_mut(&mut self)->&mut SplitRing{&mut self.inner}}
impl ReusedRing{
 fn new(s:&CanonicalFiber)->Result<Self,Error>{Ok(Self{inner:SplitRing::new(s)?,powers:std::collections::BTreeMap::new(),scale_key:None,power_products:0,value_products:0,resets:0})}
 fn prepare(&mut self,scale:u64)->Result<(),Error>{if scale>=self.r{return Err(Error::Domain)}if self.scale_key!=Some(scale){self.powers.clear();self.scale_key=Some(scale);self.resets+=1;self.powers.insert(0,self.inner.constant(1));self.powers.insert(1,self.inner.base(scale));}Ok(())}
 fn cached_power(&mut self,n:usize)->Result<SplitValue,Error>{
  if let Some(v)=self.powers.get(&n){return Ok(v.clone())}if self.scale_key.is_none(){return Err(Error::Domain)}
  let half=self.cached_power(n/2)?;let mut value=self.inner.multiply(&half,&half)?;self.power_products+=1;
  if n%2==1{let base=self.powers.get(&1).ok_or(Error::Arithmetic)?.clone();value=self.inner.multiply(&value,&base)?;self.power_products+=1;}
  self.powers.insert(n,value.clone());Ok(value)
 }
 fn value_block(&mut self,reader:&mut CoefficientReader<'_>,length:usize)->Result<SplitValue,Error>{
  if length==0{return Err(Error::Domain)}if length==1{let c=reader.next()?;self.inner.reads+=1;return Ok(self.inner.constant(c))}
  let left=self.value_block(reader,length/2)?;let right=self.value_block(reader,length-length/2)?;let power=self.cached_power(length-length/2)?;let product=self.inner.multiply(&left,&power)?;self.value_products+=1;Ok(self.inner.add(&product,&right))
 }
 fn mode(&mut self,modes:&RecoveryModes,e:usize,scale:u64)->Result<SplitValue,Error>{if modes.r!=self.r||scale>=self.r{return Err(Error::Domain)}let stream=modes.stream(e)?;let length=stream.left;let mut reader=CoefficientReader{stream,buffer:Vec::new(),at:0,remaining:length};self.prepare(scale)?;self.value_block(&mut reader,length)}
 fn cache_slots(&self)->usize{self.powers.values().map(|v|v.both.as_ref().map_or(0,|q|q.0[0].len()+q.0[1].len())+v.one.as_ref().map_or(0,Vec::len)).sum()}
}
include!("reused-image.rs");
include!("reused-adapter.rs");
include!("reused-checks.rs");

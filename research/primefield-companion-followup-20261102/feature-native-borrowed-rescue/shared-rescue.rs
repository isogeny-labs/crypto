// Distinct stream identities share one bounded interval-product engine.
std::thread_local!{static RESCUE_SHARES:std::cell::Cell<[usize;4]>=const{std::cell::Cell::new([0;4])};}
impl RecoveryProducts<'_>{
 fn for_doubled(&self,stream:&StreamModes)->Result<RecoveryProducts<'_>,Error>{
  if stream.r!=self.r||stream.block!=self.cap||self.m.checked_mul(2)!=Some(stream.m){return Err(Error::Domain)}
  let q=self.inner.try_borrow().map_err(|_|Error::Arithmetic)?;
  if q.r!=self.r||q.request_cap!=self.cap.saturating_mul(2)||q.width==0||q.width>self.cap{return Err(Error::Domain)}
  let warm=usize::from(q.polynomial.is_some());let width=q.width;drop(q);
  RESCUE_SHARES.with(|c|{let mut v=c.get();v[0]+=1;v[1]+=warm;v[2]+=1-warm;v[3]+=width;c.set(v)});
  Ok(RecoveryProducts{r:self.r,m:stream.m,cap:self.cap,inner:EngineStorage::Borrowed(&self.inner)})
 }
}
include!("shared-rescue-phase.rs");
include!("shared-rescue-adapter.rs");
include!("shared-rescue-checks.rs");

include!("borrowed-checks.rs");

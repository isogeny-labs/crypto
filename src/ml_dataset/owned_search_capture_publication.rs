//! Actual original-budget capture publication. A committed snapshot certifies
//! custody of a complete encoding, never completion of the search or dataset.
use super::{OwnedAttempt,Failure as OwnerFailure,Stop,Budget,capture};
use super::capture_encoding as encoding;
use std::{mem::size_of,time::Instant};
#[cfg(test)]
#[path="owned_search_publication_observer.rs"]
pub(in crate::ml_dataset::solvers) mod test_observer;
pub(in crate::ml_dataset::solvers) const ROLE:&str="boolean-original-owned-search-snapshot-chunk-v1";
#[derive(Default)]
pub(in crate::ml_dataset::solvers) struct Publication{
    pub snapshots:Vec<Snapshot>,pub failure:Option<Stop>,
}
pub(in crate::ml_dataset::solvers) struct Snapshot{
    pub index:u64,pub chunks:Vec<Chunk>,pub encoding:Option<encoding::Attempt>,
    pub complete:bool,pub failure:Option<Stop>,pub capture_error:Option<capture::CaptureError>,
}
pub(in crate::ml_dataset::solvers) struct Chunk{
    pub index:u64,pub path:String,pub bytes:u64,pub before:Option<capture::CommittedEventPosition>,
    pub ack:Option<capture::CommittedEventPosition>,pub exact:bool,
}
struct Bridge<'a>{budget:&'a mut Budget,sink:&'a mut capture::CaptureEmitter,
    limits:&'a capture::CaptureLimits,pattern:u32,snapshot:&'a mut Snapshot,
    expected:&'a capture::CommittedEventPosition,}
impl Bridge<'_>{
 fn fail<T>(&mut self,s:Stop)->Result<T,()>{self.snapshot.failure=Some(s);let _:Result<(),Stop>=self.budget.fail(s);Err(())}
 fn pay(&mut self,w:u64,b:u64)->Result<(),()>{if let Err(s)=self.budget.reserve_checking(w,b,Instant::now()){return self.fail(s)}Ok(())}
 fn publish_inner(&mut self,index:u64,bytes:&[u8])->Result<(),Stop>{
  if index!=self.snapshot.chunks.len() as u64{return Err(Stop::Phase)}
  let count=u64::try_from(bytes.len()).map_err(|_|Stop::Overflow)?;
  let quote=super::super::super::quote_parent_storage_length(count,false,self.limits)?;
  // Covers record capacity, two actual head snapshots, fixed bounded path and
  // capture error/metadata storage before any associated allocating operation.
  let entries=self.snapshot.chunks.len().checked_add(1).ok_or(Stop::Overflow)?;
  let growth=u64::try_from(entries.checked_mul(size_of::<Chunk>()).ok_or(Stop::Overflow)?).map_err(|_|Stop::Overflow)?;
  self.budget.reserve_checking(quote.work.checked_add(8192).and_then(|w|w.checked_add(entries as u64)).ok_or(Stop::Overflow)?,quote.bytes.checked_add(growth).and_then(|b|b.checked_add(65536)).ok_or(Stop::Overflow)?,Instant::now())?;
  self.budget.reserve_trace(quote.trace,Instant::now())?;
  self.snapshot.chunks.try_reserve_exact(1).map_err(|_|Stop::Allocation)?;
  let path=format!("boolean-owned-pattern-{}-snapshot-{}-chunk-{index}.bin.part",self.pattern,self.snapshot.index);
  self.snapshot.chunks.push(Chunk{index,path,bytes:count,before:None,ack:None,exact:false});
  let current=self.sink.committed_event_position();
  let expected=if index==0{self.expected}else{self.snapshot.chunks[index as usize-1].ack.as_ref().ok_or(Stop::Observer)?};
  let matches=&current==expected;
  self.snapshot.chunks[index as usize].before=Some(current);
  if !matches{return Err(Stop::Observer)}
  self.budget.check(Instant::now())?;
  let chunk=&mut self.snapshot.chunks[index as usize];
  if let Err(e)=self.sink.payload_borrowed(&chunk.path,ROLE,"owned-search-wire-v1",bytes){self.snapshot.capture_error=Some(e);return Err(Stop::Observer)}
  // Install acknowledgement before integrity/deadline checks can fail.
  chunk.ack=Some(self.sink.committed_event_position());
  let before=chunk.before.as_ref().ok_or(Stop::Observer)?;
  let after=chunk.ack.as_ref().ok_or(Stop::Observer)?;
  let exact=after.event_count==before.event_count.checked_add(1).ok_or(Stop::Overflow)?
   &&after.binding_sha256==before.binding_sha256&&after.byte_end>before.byte_end;
  chunk.exact=exact;if !exact{return Err(Stop::Observer)}
  // Test observers see an actual installed acknowledgement, never pending bytes.
  // This is custody evidence only; the original deadline check remains below.
  #[cfg(test)]
  test_observer::notify(test_observer::Boundary::ChunkAcknowledged {
   pattern:self.pattern,snapshot:self.snapshot.index,chunk:index,bytes:count,receipt:after,
  });
  self.budget.check(Instant::now())
 }
}
impl encoding::AdmissionSink for Bridge<'_>{
 fn reserve(&mut self,w:u64,b:u64)->Result<(),()>{self.pay(w,b)}
 fn check(&mut self)->Result<(),()>{if let Err(s)=self.budget.check(Instant::now()){self.fail(s)}else{Ok(())}}
 fn publish(&mut self,i:u64,b:&[u8])->Result<(),()>{match self.publish_inner(i,b){Ok(())=>Ok(()),Err(s)=>self.fail(s)}}
}
/// Borrowed publication uses only the authentic owner split and original sink.
pub(super) fn checkpoint_parts(source:&encoding::EncodingSource<'_>,budget:&mut Budget,sink:&mut capture::CaptureEmitter,publication:&mut Publication,limits:&capture::CaptureLimits)->Result<(),Stop>{
 if let Some(s)=publication.failure{return Err(s)}
 let result=(||->Result<(),Stop>{
   let entries=publication.snapshots.len().checked_add(1).ok_or(Stop::Overflow)?;
   let growth=u64::try_from(entries.checked_mul(size_of::<Snapshot>()).ok_or(Stop::Overflow)?).map_err(|_|Stop::Overflow)?;
   budget.reserve_checking(8192u64.checked_add(entries as u64).ok_or(Stop::Overflow)?,growth.checked_add(65536).ok_or(Stop::Overflow)?,Instant::now())?;
   let expected=match publication.snapshots.last(){
    Some(s)=>{if !s.complete{return Err(Stop::Phase)}s.chunks.last().and_then(|c|c.ack.as_ref()).ok_or(Stop::Observer)?},
    None=>source.map_ack().ok_or(Stop::Observer)?,
   }.clone();
   let pattern=source.pattern().ok_or(Stop::Phase)?;
   let index=u64::try_from(publication.snapshots.len()).map_err(|_|Stop::Overflow)?;
   publication.snapshots.try_reserve_exact(1).map_err(|_|Stop::Allocation)?;
   publication.snapshots.push(Snapshot{index,chunks:Vec::new(),encoding:None,complete:false,failure:None,capture_error:None});
   let snapshot=publication.snapshots.last_mut().unwrap();
   let work=budget.native_work_limit.checked_sub(budget.native.work).and_then(|v|v.checked_sub(budget.checking_work)).ok_or(Stop::Overflow)?;
   let chunk_bytes=usize::try_from(limits.chunk_bytes.min(limits.max_payload_bytes).min(16384)).map_err(|_|Stop::Overflow)?;
   let cap=encoding::Limits{chunk_bytes,total_bytes:limits.max_payload_bytes,chunks:limits.max_payload_count,work};
   let mut bridge=Bridge{budget,sink,limits:&limits,pattern,snapshot,expected:&expected};
   let encoded=encoding::encode(source,index,cap,&mut bridge);
   let complete=encoded.complete&&encoded.result.is_ok();
   let encoding_stop=match encoded.result{Ok(())=>Stop::Serialization,Err(encoding::Failure::Deadline)=>Stop::Deadline,Err(encoding::Failure::Overflow)=>Stop::Overflow,Err(encoding::Failure::Allocation)=>Stop::Allocation,Err(encoding::Failure::Format)=>Stop::Serialization,Err(_)=>Stop::NativeFailure};
   let acknowledged=encoded.acknowledged_chunks;
   let byte_summary=(encoded.pending.is_empty(),encoded.acknowledged_bytes,encoded.encoded_bytes);
   bridge.snapshot.encoding=Some(encoded);
   if !complete{return Err(bridge.snapshot.failure.unwrap_or(encoding_stop))}
   if acknowledged!=bridge.snapshot.chunks.len() as u64||bridge.snapshot.chunks.is_empty()
    ||bridge.snapshot.chunks.iter().any(|c|!c.exact||c.ack.is_none()){return Err(Stop::Observer)}
   let retained_bytes=bridge.snapshot.chunks.iter().try_fold(0u64,|n,c|n.checked_add(c.bytes)).ok_or(Stop::Overflow)?;
   if !byte_summary.0||byte_summary.1!=byte_summary.2||byte_summary.1!=retained_bytes{return Err(Stop::Observer)}
   bridge.budget.check(Instant::now())?;
   bridge.snapshot.complete=true;
   #[cfg(test)]
   test_observer::notify(test_observer::Boundary::SnapshotComplete {
    pattern,snapshot:index,chunks:acknowledged,bytes:retained_bytes,
    receipt:bridge.snapshot.chunks.last().and_then(|c|c.ack.as_ref()).expect("checked complete acknowledgement"),
   });
   // Test instrumentation cannot return a successful publication after consuming
   // the original deadline while observing its complete durable state.
   #[cfg(test)]
   bridge.budget.check(Instant::now())?;
   Ok(())
 })();
 if let Err(s)=result{publication.failure=Some(s);if let Some(last)=publication.snapshots.last_mut(){if !last.complete{last.failure.get_or_insert(s);}}let _:Result<(),Stop>=budget.fail(s);}
 result
}
impl OwnedAttempt{
 /// Actual checkpoint boundary. No stopped original budget receives new supply.
 /// Multiple successful snapshots have distinct indices and an exact head chain.
 pub(in crate::ml_dataset::solvers) fn checkpoint_capture(&mut self)->Result<(),Stop>{
  if let Some(s)=self.publication.failure{return Err(s)}
  let mut publication=std::mem::take(&mut self.publication);
  let limits=self.original.capture_limits.clone();
  // CaptureLimits is fixed-size numeric metadata; cloning it allocates nothing.
  let result=encoding::with_parts(self,|source,budget,sink|checkpoint_parts(source,budget,sink,&mut publication,&limits));
  self.publication=publication;
  if let Err(s)=result{if self.failure.is_none(){self.fail(OwnerFailure::Publication(s),s);}return Err(s)}
  Ok(())
 }
 pub(in crate::ml_dataset::solvers) fn publish_capture(mut self)->Self{let _=self.checkpoint_capture();self}
 pub(in crate::ml_dataset::solvers) fn capture_publication(&self)->&Publication{&self.publication}
}

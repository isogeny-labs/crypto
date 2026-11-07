//! Bounded native decoder for the actual retained-search wire format.
//! Parsing/shape checks confer no original-source or mathematical authority.
use std::{mem::size_of,time::Instant};
#[derive(Debug,PartialEq,Eq)]
pub enum Value{Null,Bool(bool),U64(u64),Bytes(Vec<u8>),Array(Vec<Value>),Object(Vec<(String,Value)>),Text(String)}
impl Value{
 pub fn as_array(&self)->Option<&[Value]>{if let Self::Array(v)=self{Some(v)}else{None}}
 pub fn as_object(&self)->Option<&[(String,Value)]>{if let Self::Object(v)=self{Some(v)}else{None}}
 pub fn get(&self,k:&str)->Option<&Value>{self.as_object()?.iter().find(|(s,_)|s==k).map(|(_,v)|v)}
 pub fn as_u64(&self)->Option<u64>{if let Self::U64(v)=self{Some(*v)}else{None}}
 pub fn as_bool(&self)->Option<bool>{if let Self::Bool(v)=self{Some(*v)}else{None}}
 pub fn as_str(&self)->Option<&str>{if let Self::Text(v)=self{Some(v)}else{None}}
 pub fn as_bytes(&self)->Option<&[u8]>{if let Self::Bytes(v)=self{Some(v)}else{None}}
}
pub trait Admission{fn reserve(&mut self,work:u64,bytes:u64)->Result<(),()>;fn check(&mut self)->Result<(),()>;}
#[derive(Clone,Copy,Debug)]
pub struct Limits{pub raw_bytes:usize,pub nodes:u64,pub depth:usize,pub list_items:usize,pub map_fields:usize,pub string_bytes:usize,pub bytes_value:usize,pub work:u64,pub allocation_bytes:u64}
impl Default for Limits{fn default()->Self{Self{raw_bytes:128*1024*1024,nodes:2_000_000,depth:64,list_items:1_000_000,map_fields:64,string_bytes:4096,bytes_value:128*1024*1024,work:32_000_000,allocation_bytes:256*1024*1024}}}
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]pub struct Charge{pub work:u64,pub bytes:u64}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum Failure{Limits,RawCap,Nodes,Depth,Container,StringCap,BytesCap,Work,Memory,Overflow,OwnerReserve,OwnerCheck,Deadline,Allocation,Truncated,Magic,Tag,Boolean,Utf8,DuplicateKey,KeyType,Trailing,Shape}
pub struct Attempt{pub raw:Vec<u8>,pub value:Option<Value>,pub result:Result<(),Failure>,pub consumed:usize,pub nodes:u64,pub charge:Charge}
struct Meter<'a,T:Admission>{owner:&'a mut T,limits:Limits,deadline:Instant,charge:Charge}
impl<T:Admission> Meter<'_,T>{
 fn pay(&mut self,w:u64,b:u64)->Result<(),Failure>{
  if Instant::now()>=self.deadline{return Err(Failure::Deadline)}
  let next=Charge{work:self.charge.work.checked_add(w).ok_or(Failure::Overflow)?,bytes:self.charge.bytes.checked_add(b).ok_or(Failure::Overflow)?};
  if next.work>self.limits.work{return Err(Failure::Work)}if next.bytes>self.limits.allocation_bytes{return Err(Failure::Memory)}
  self.owner.reserve(w,b).map_err(|_|Failure::OwnerReserve)?;self.charge=next;
  self.owner.check().map_err(|_|Failure::OwnerCheck)?;
  if Instant::now()>=self.deadline{return Err(Failure::Deadline)}Ok(())
 }
 fn vec<V>(&mut self,n:usize)->Result<Vec<V>,Failure>{
  let bytes=n.checked_mul(size_of::<V>()).and_then(|v|v.checked_add(64)).ok_or(Failure::Overflow)?;
  self.pay(1,u64::try_from(bytes).map_err(|_|Failure::Overflow)?)?;
  let mut v=Vec::new();v.try_reserve_exact(n).map_err(|_|Failure::Allocation)?;Ok(v)
 }
}
struct Reader<'a,'b,T:Admission>{raw:&'a[u8],offset:usize,nodes:u64,meter:Meter<'b,T>}
impl<T:Admission> Reader<'_,'_,T>{
 fn take(&mut self,n:usize)->Result<&[u8],Failure>{
  let end=self.offset.checked_add(n).ok_or(Failure::Overflow)?;if end>self.raw.len(){return Err(Failure::Truncated)}
  self.meter.pay(u64::try_from(n).map_err(|_|Failure::Overflow)?,0)?;
  let start=self.offset;self.offset=end;Ok(&self.raw[start..end])
 }
 fn number(&mut self)->Result<u64,Failure>{let b=self.take(8)?;Ok(u64::from_le_bytes(b.try_into().map_err(|_|Failure::Truncated)?))}
 fn count(&mut self)->Result<usize,Failure>{usize::try_from(self.number()?).map_err(|_|Failure::Overflow)}
 fn value(&mut self,depth:usize)->Result<Value,Failure>{
  if depth>self.meter.limits.depth{return Err(Failure::Depth)}
  if self.nodes>=self.meter.limits.nodes{return Err(Failure::Nodes)}
  self.meter.pay(1,0)?;self.nodes+=1;let tag=self.take(1)?[0];
  match tag{
   0=>Ok(Value::Null),1=>match self.take(1)?[0]{0=>Ok(Value::Bool(false)),1=>Ok(Value::Bool(true)),_=>Err(Failure::Boolean)},
   2=>Ok(Value::U64(self.number()?)),
   3=>{let n=self.count()?;if n>self.meter.limits.string_bytes{return Err(Failure::StringCap)}if n>self.raw.len()-self.offset{return Err(Failure::Truncated)}
    // Charge UTF8 validation/copy and allocation before either operation.
    self.meter.pay((n as u64).checked_mul(2).ok_or(Failure::Overflow)?,(n as u64).checked_add(64).ok_or(Failure::Overflow)?)?;
    let data=self.take(n)?;let text=std::str::from_utf8(data).map_err(|_|Failure::Utf8)?;
    let mut s=String::new();s.try_reserve_exact(n).map_err(|_|Failure::Allocation)?;s.push_str(text);Ok(Value::Text(s))},
   4=>{let n=self.count()?;if n>self.meter.limits.bytes_value{return Err(Failure::BytesCap)}if n>self.raw.len()-self.offset{return Err(Failure::Truncated)}
    let mut bytes=self.meter.vec::<u8>(n)?;bytes.extend_from_slice(self.take(n)?);Ok(Value::Bytes(bytes))},
   5=>{let n=self.count()?;if n>self.meter.limits.list_items{return Err(Failure::Container)}
    if n as u64>self.meter.limits.nodes-self.nodes{return Err(Failure::Nodes)}
    if n>self.raw.len()-self.offset{return Err(Failure::Truncated)}
    let mut a=self.meter.vec(n)?;for _ in 0..n{a.push(self.value(depth+1)?)}Ok(Value::Array(a))},
   6=>{let n=self.count()?;if n>self.meter.limits.map_fields{return Err(Failure::Container)}
    if (n as u64).checked_mul(2).ok_or(Failure::Overflow)?>self.meter.limits.nodes-self.nodes{return Err(Failure::Nodes)}
    if n.checked_mul(2).ok_or(Failure::Overflow)?>self.raw.len()-self.offset{return Err(Failure::Truncated)}
    let mut fields:Vec<(String,Value)>=self.meter.vec(n)?;
    for _ in 0..n{let key=match self.value(depth+1)?{Value::Text(k)=>k,_=>return Err(Failure::KeyType)};
     for(prev,_)in &fields{self.meter.pay((prev.len() as u64).checked_add(key.len() as u64).ok_or(Failure::Overflow)?,0)?;if prev==&key{return Err(Failure::DuplicateKey)}}
     fields.push((key,self.value(depth+1)?));}
    Ok(Value::Object(fields))},_=>Err(Failure::Tag)
  }
 }
}
enum Spec{Any,UInt,UInt32,Bool,Text,Bytes(Option<usize>),Maybe(&'static Spec),List(&'static Spec),Tuple(&'static[Spec]),Object(&'static[(&'static str,Spec)])}
const UINT:Spec=Spec::UInt;const BOOL:Spec=Spec::Bool;const TEXT:Spec=Spec::Text;
const MONO:Spec=Spec::List(&Spec::UInt32);const POLY:Spec=Spec::List(&MONO);const POLYS:Spec=Spec::List(&POLY);
const ROW:Spec=Spec::Object(&[("polynomial",POLY),("proof",Spec::List(&Spec::Tuple(&[UINT,MONO])))]);
const TASK:Spec=Spec::Object(&[("kind",TEXT),("degree",UINT),("serial",UINT)]);
const CHARGE:Spec=Spec::Tuple(&[UINT,UINT]);
const BATCH:Spec=Spec::Object(&[("basis_before",UINT),("degree",UINT),("tasks",Spec::List(&TASK)),("initial",Spec::List(&ROW)),("matrix",Spec::List(&ROW)),("reducers",Spec::List(&Spec::Object(&[("basis",UINT),("multiplier",MONO),("row",UINT)]))),("operations",Spec::List(&Spec::Tuple(&[UINT,UINT]))),("stage",TEXT),("row_cursor",UINT),("term_cursor",UINT),("basis_cursor",UINT),("order",Spec::List(&UINT)),("pivots",Spec::List(&UINT)),("staged_basis",Spec::List(&ROW)),("staged_tasks",Spec::List(&TASK)),("additions",Spec::List(&Spec::Tuple(&[UINT,UINT])))]);
const NATIVE:Spec=Spec::Object(&[("batches",Spec::List(&BATCH)),("result",TEXT),("complete",BOOL),("basis",Spec::List(&ROW)),("pending",Spec::List(&TASK)),("active",Spec::Maybe(&TASK)),("current",ROW),("charge",CHARGE),("completed",UINT),("row_xors",UINT),("symbolic_reducers",UINT),("field_tasks",UINT),("ordinary_tasks",UINT),("last_degree",UINT)]);
const NODE:Spec=Spec::Object(&[("parent",Spec::Maybe(&UINT)),("origins",Spec::List(&TEXT)),("equations",POLYS),("decisions",Spec::List(&Spec::Tuple(&[Spec::UInt32,BOOL]))),("status",TEXT),("native_disposition",Spec::Maybe(&TEXT)),("native",Spec::Maybe(&NATIVE)),("candidate",Spec::List(&BOOL)),("candidate_valid",Spec::Maybe(&BOOL)),("children",Spec::Tuple(&[Spec::Maybe(&UINT),Spec::Maybe(&UINT)])),("refutation_row",Spec::Maybe(&UINT))]);
const SEARCH:Spec=Spec::Object(&[("result",TEXT),("verdict",TEXT),("nodes",Spec::List(&NODE)),("queue",Spec::List(&UINT)),("active",Spec::Maybe(&UINT)),("witness_node",Spec::Maybe(&UINT)),("charge",CHARGE)]);
const PARTITION:Spec=Spec::Object(&[("parent",Spec::Maybe(&UINT)),("prefix",Spec::List(&BOOL)),("equations",POLYS),("status",TEXT),("search",Spec::Maybe(&SEARCH)),("audit",Spec::Any),("children",Spec::List(&UINT)),("exclusion_record",Spec::Maybe(&UINT))]);
const CONTINUATION:Spec=Spec::Object(&[("partitions",Spec::List(&PARTITION)),("queue",Spec::List(&UINT)),("active",Spec::Maybe(&UINT)),("outcome",TEXT),("failure",Spec::Maybe(&TEXT)),("charge",CHARGE)]);
const EXPANSION:Spec=Spec::Object(&[("variables",Spec::UInt32),("equations",POLYS),("status",TEXT),("owner_failure",Spec::Maybe(&TEXT)),("charge",CHARGE),("partial",POLY),("scratch",POLY),("stage",TEXT),("row",UINT),("literal",UINT)]);
const ACK:Spec=Spec::Object(&[("binding_sha256",TEXT),("event_count",UINT),("byte_end",UINT),("last_event_sha256",TEXT)]);
const IDENTITY:Spec=Spec::Object(&[("public_sha256",Spec::Bytes(Some(32))),("factor_sha256",Spec::Bytes(Some(32))),("launch_sha256",Spec::Bytes(Some(32))),("binding_sha256",Spec::Bytes(Some(32))),("map_bytes",Spec::Bytes(None)),("map_ack",Spec::Maybe(&ACK)),("publication_attempted",BOOL),("map_published",BOOL),("pattern",Spec::Maybe(&Spec::UInt32))]);
const ROOT:Spec=Spec::Object(&[("schema",TEXT),("snapshot_index",UINT),("identity",IDENTITY),("signed_domain",Spec::List(&Spec::Tuple(&[UINT,UINT]))),("signed_target",Spec::Maybe(&Spec::Tuple(&[UINT,UINT]))),("failure",Spec::Maybe(&TEXT)),("first_stop",Spec::Maybe(&TEXT)),("diagnostic",Spec::Maybe(&TEXT)),("normalization_cursor",Spec::Tuple(&[UINT,UINT,UINT])),("expansion",Spec::Maybe(&EXPANSION)),("normalized",POLYS),("continuation",Spec::Maybe(&CONTINUATION)),("lift_records",Spec::Any),("coverage",Spec::Any),("exclusion_audit",Spec::Any),("accepted_record",Spec::Maybe(&UINT)),("status",TEXT),("encoding_complete",BOOL)]);
fn shape<T:Admission>(v:&Value,s:&Spec,m:&mut Meter<'_,T>)->Result<(),Failure>{
 m.pay(1,0)?;
 match(s,v){
 (Spec::Any,_)|(Spec::UInt,Value::U64(_))|(Spec::Bool,Value::Bool(_))|(Spec::Text,Value::Text(_))=>Ok(()),
 (Spec::UInt32,Value::U64(n))if *n<=u32::MAX as u64=>Ok(()),
 (Spec::Bytes(n),Value::Bytes(v))if n.is_none_or(|n|n==v.len())=>Ok(()),
 (Spec::Maybe(_),Value::Null)=>Ok(()),(Spec::Maybe(s),v)=>shape(v,s,m),
 (Spec::List(s),Value::Array(a))=>{for v in a{shape(v,s,m)?}Ok(())},
 (Spec::Tuple(s),Value::Array(a))if s.len()==a.len()=>{for(v,s)in a.iter().zip(s.iter()){shape(v,s,m)?}Ok(())},
 (Spec::Object(s),Value::Object(a))if s.len()==a.len()=>{for((k,v),(expected,s))in a.iter().zip(s.iter()){m.pay((k.len() as u64).checked_add(expected.len() as u64).ok_or(Failure::Overflow)?,0)?;if k!=expected{return Err(Failure::Shape)}shape(v,s,m)?}Ok(())},
 _=>Err(Failure::Shape)
 }
}
fn valid_limits(l:Limits)->bool{
 l.raw_bytes>0&&l.raw_bytes<=1024*1024*1024&&l.nodes>0&&l.nodes<=4_000_000&&l.depth>0&&l.depth<=128&&l.list_items<=2_000_000&&l.map_fields<=256&&l.string_bytes<=1024*1024&&l.bytes_value<=1024*1024*1024&&l.work<=1_000_000_000&&l.allocation_bytes<=1024*1024*1024
}
/// Input Vec is already owned/read under the caller's admission. This decoder
/// charges all new owned copies/tree allocations. On any error the original raw
/// bytes, offset and accepted charges survive; no incomplete tree is exported.
pub fn decode(raw:Vec<u8>,limits:Limits,deadline:Instant,owner:&mut impl Admission)->Attempt{
 let mut value=None;
 let(mut consumed,mut nodes,mut charge)=(0,0,Charge::default());
 let result=if !valid_limits(limits){Err(Failure::Limits)}else if raw.len()>limits.raw_bytes{Err(Failure::RawCap)}else{
  let mut r=Reader{raw:&raw,offset:0,nodes:0,meter:Meter{owner,limits,deadline,charge:Charge::default()}};
  let result=(||{
   if r.take(10)?!=b"OWNSEARCH\x01"{return Err(Failure::Magic)}
   let parsed=r.value(0)?;
   if r.offset!=raw.len(){return Err(Failure::Trailing)}
   shape(&parsed,&ROOT,&mut r.meter)?;
   r.meter.pay(512,0)?;
   if parsed.get("schema").and_then(Value::as_str)!=Some("owned-search-retained-evidence/v1")||parsed.get("encoding_complete").and_then(Value::as_bool)!=Some(true){return Err(Failure::Shape)}
   if let Some(ack)=parsed.get("identity").and_then(|v|v.get("map_ack")){if !matches!(ack,Value::Null){for k in ["binding_sha256","last_event_sha256"]{let h=ack.get(k).and_then(Value::as_str).ok_or(Failure::Shape)?;if h.len()!=64||!h.bytes().all(|c|c.is_ascii_digit()||(b'a'..=b'f').contains(&c)){return Err(Failure::Shape)}}}}
   r.meter.pay(1,0)?;value=Some(parsed);Ok(())
  })();consumed=r.offset;nodes=r.nodes;charge=r.meter.charge;result
 };
 Attempt{raw,value,result,consumed,nodes,charge}
}

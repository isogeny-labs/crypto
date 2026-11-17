//! Structural test shim, NOT an authenticated CheckedMap constructor.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum Error{InvalidOriginal,Cap,Admission,Identity,Semantics,Formula}
pub struct Quote{pub work:u64,pub bytes:u64}
pub trait Admission{fn reserve(&mut self,q:Quote)->Result<(),Error>;fn check(&mut self)->Result<(),Error>;}
pub enum Constraint{Clause{literals:Vec<i32>},Xor{wires:Vec<u32>,rhs:bool}}
pub struct Identity{n:u32,m:usize,a:Vec<u32>,b:Vec<u32>,target:Option<Vec<u32>>,low_terms:Vec<u32>,codes:Vec<Vec<u32>>,pattern:u32}
pub struct Map{identity:Identity,variables:u32,formula:Vec<Constraint>,blocks:Vec<Vec<u32>>}
pub struct CheckedMap(Map);
impl CheckedMap{fn variables(&self)->u32{self.0.variables}}
#[path="checked_model.shim.rs"]pub mod checked_model;
mod signed_lift;mod sparse;mod scheduler;mod bridge;mod bqi;mod check;mod continuation;mod exclusions;


use sparse as equations;
mod provenance{pub use super::Error;}
#[derive(Debug,Clone,Copy)]pub enum Stop {
    InvalidLimits,
    Overflow,
    Deadline,
    Phase,
    Nonmonotone,
    Work,
    Bytes,
    Trace,
    Additions,
    Observer,
    NativeFailure,
    Allocation,
    Serialization,
    Events,
}
#[derive(Debug,Clone,Copy)]pub enum OwnerFailure{Admission(Stop),Expansion(equations::Failure),Search(scheduler::Failure),Audit(check::Failure),LiftStopped(provenance::Error),Continuation(continuation::Failure),Coverage(continuation::Failure),Exclusions(exclusions::Failure)}
pub struct LiftRecord {
    pub partition:usize,pub record:u64,pub model:Vec<bool>,
    pub progress:checked_model::Progress,
    pub callback_result:Option<Result<(),continuation::Failure>>,
    pub returned:Option<Result<signed_lift::Outcome,Stop>>,
}
mod capture{pub struct CommittedEventPosition {
    pub binding_sha256: String,
    pub event_count: u64,
    pub byte_end: u64,
    pub last_event_sha256: String,
}}
mod capture_projection{use super::capture;pub struct SourceIdentity<'a>{
 pub public_sha256:&'a [u8;32],pub factor_sha256:&'a [u8;32],
 pub launch_sha256:&'a [u8;32],pub binding_sha256:&'a [u8;32],
 pub map_bytes:&'a [u8],pub map_ack:Option<&'a capture::CommittedEventPosition>,
 pub publication_attempted:bool,pub map_published:bool,pub pattern:Option<u32>,
}
#[derive(Debug,Clone,Copy)]pub enum Status{
 Incomplete,
 AcceptedSignedRelation,
 ConditionalBooleanCoverage,
 /// Authentic owner's replay gate succeeded. Does not claim all patterns,
 /// generator keys, or curves exhausted, and is not a persisted receipt.
 RetainedModelExclusionsReplayed,
}}
#[path="encoding.test.rs"] mod encoding;

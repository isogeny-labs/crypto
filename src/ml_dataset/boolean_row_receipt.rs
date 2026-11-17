//! Versioned finite-unit-row wire. Identity fields are checked against actual
//! originals and capture receipts, never accepted merely because parsing succeeds.
use serde::{Serialize,Deserialize};
pub(in crate::ml_dataset::solvers) const SCHEMA:&str="crypto.ml-dataset.original-unit-boolean-row/v1";
pub(in crate::ml_dataset::solvers) const ROLE:&str="boolean-original-unit-row";
pub(in crate::ml_dataset::solvers) const META_CAP:u64=8192;
pub(in crate::ml_dataset::solvers) const REPORT_CAP:u64=4096;
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub(in crate::ml_dataset::solvers) struct Metadata {
    pub original_binding:[u8;32], pub launch_sha256:[u8;32],
    pub pattern:u32,pub row:u64,pub window:u8,
    pub variables:u32,pub generators:u64,
    pub map_sha256:[u8;32],pub map_bytes:u64,pub map_sequence:u64,pub map_event_sha256:[u8;32],
    pub wire_sha256:[u8;32],pub wire_bytes:u64,
    pub limits:[u64;4],pub totals:[u64;7],pub phase:u8,
    pub phase_native:[u64;2],pub phase_limits:[u64;2],
    pub first_stop:Option<super::super::super::Stop>,
    pub solver_completed:bool,pub full_span_search:bool,pub scalar_recovered:bool,
}
#[derive(Serialize)]
pub(in crate::ml_dataset::solvers) struct Wire<'a>{pub schema:&'static str,pub metadata:&'a Metadata,pub wire:&'a str}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::ml_dataset::solvers) struct Stored {pub schema:String,pub metadata:Metadata,pub wire:String}
pub(in crate::ml_dataset::solvers) fn path(pattern:u32,row:u64)->String{format!("boolean-original-pattern-{pattern}-row-{row}.bqi.json")}
/// All bytes of genuine BQI1 are digits, spaces, header letters and final LF.
/// Only LF expands by one JSON byte; metadata cap covers all remaining fields.
pub(in crate::ml_dataset::solvers) fn cap(raw:&[u8])->Option<u64>{
    if raw.iter().any(|b|!b.is_ascii_digit()&&!matches!(b,b' '|b'\n'|b'B'|b'Q'|b'I')){return None}
    let escaped=raw.iter().filter(|b|**b==b'\n').count() as u64;
    (raw.len() as u64).checked_add(escaped)?.checked_add(META_CAP)
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn maximum_fixed_metadata_and_real_ascii_wire_fit_admitted_cap(){
  let m=Metadata{original_binding:[255;32],launch_sha256:[255;32],pattern:u32::MAX,row:u64::MAX,window:u8::MAX,variables:u32::MAX,generators:u64::MAX,map_sha256:[255;32],map_bytes:u64::MAX,map_sequence:u64::MAX,map_event_sha256:[255;32],wire_sha256:[255;32],wire_bytes:u64::MAX,limits:[u64::MAX;4],totals:[u64::MAX;7],phase:u8::MAX,phase_native:[u64::MAX;2],phase_limits:[u64::MAX;2],first_stop:None,solver_completed:false,full_span_search:false,scalar_recovered:false};
  let raw="BQI1 2 1 1 1 0 1 1 0 1 0 0\n";
  let encoded=serde_json::to_vec(&Wire{schema:SCHEMA,metadata:&m,wire:raw}).unwrap();
  assert!((encoded.len() as u64)<=cap(raw.as_bytes()).unwrap());
  let decoded:Stored=serde_json::from_slice(&encoded).unwrap();assert_eq!(decoded.metadata,m);assert_eq!(decoded.wire,raw);
  let mut value:serde_json::Value=serde_json::from_slice(&encoded).unwrap();value["extra"]=true.into();assert!(serde_json::from_value::<Stored>(value).is_err());
 }
}

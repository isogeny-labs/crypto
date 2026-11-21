#![allow(dead_code)]
mod ml_dataset{
 #[path="/home/ec2-user/ml-dataset-resume-20261029/owned-search-disposition-v1-20261107/types.rs"]pub mod types;
 pub mod solvers{
 use super::types::CaseConfig;use serde_json::Value;
 mod koblitz_m3{pub mod sat_original_owner{
  pub mod search_worker{pub fn supported(_: &crate::ml_dataset::types::CaseConfig)->bool{false}}
  pub mod affine_worker{pub fn supported(_: &crate::ml_dataset::types::CaseConfig)->bool{false}}
  pub mod worker{#[derive(PartialEq)]pub enum Eligibility{Requested,Other}pub fn eligibility(_: &crate::ml_dataset::types::CaseConfig)->Eligibility{Eligibility::Other}}
  pub mod owned_search_worker{
   use crate::ml_dataset::types::{CaseConfig,OwnedBooleanSearchRequest,OwnedBooleanSearchPolicy};
   mod boolean_search{#[derive(Clone,Copy,PartialEq,Debug)]pub enum ProposedPolicy{ConservativeAffineEnvelopeV1}}
   include!("profile.rs");include!("selection.rs");
  }
 }}
 include!("disposition.rs");include!("legacy_disposition.rs");
 #[cfg(test)]mod tests{
 use super::*;use serde_json::json;use crate::ml_dataset::types::RequestedChannelDispositions;
 fn config()->CaseConfig{serde_json::from_value(json!({"family":"binary","n":23,"m":3,"twist":false,"curve_seed":0,"generator_index":0,"key_index":0,"seed":42,"factor_base_size":4,"degree_bound":3,"work_limit":10000,"solver":"f4","channels":{"schema":"crypto.ml-dataset.channel-requests/v5","owned_boolean_search":{"mode":"matrix_continuation","pattern":0,"policy":"conservative_affine_envelope_v1"}}})).unwrap()}
 #[test]fn supported_request_records_exact_request_without_evidence_claims(){let c=config();let v=requested_channel_disposition(&c).unwrap().unwrap();let d=&v["owned_boolean_search"];assert_eq!(d["status"],"requested");assert_eq!(d["reason"],"requires_actual_owned_search_replay");assert_eq!(d["request"],serde_json::to_value(c.channels.unwrap().owned_boolean_search.unwrap()).unwrap());assert_eq!(d["execution_verified"],false);assert_eq!(d["training_export_admitted"],false);let typed:RequestedChannelDispositions=serde_json::from_value(v.clone()).unwrap();assert_eq!(serde_json::to_value(typed).unwrap(),v);}
 #[test]fn actual_supported_profile_both_twists_and_all_sizes(){let mut c=config();for family in ["binary","koblitz"]{for n in (23..=31).chain(47..=61){for m in 2..=15{for twist in [false,true]{c.family=family.into();c.n=n;c.m=m;c.twist=twist;assert_eq!(requested_channel_disposition(&c).unwrap().unwrap()["owned_boolean_search"]["status"],"requested");}}}}}
 #[test]fn unsupported_profile_preserves_request_and_reason(){for (key,bad) in [("family",json!("prime")),("n",json!(32)),("m",json!(16)),("solver",json!("f5")),("factor_base_size",json!(0)),("degree_bound",json!(0)),("work_limit",json!(0))]{let mut v=serde_json::to_value(config()).unwrap();v[key]=bad;let c=serde_json::from_value(v).unwrap();let d=requested_channel_disposition(&c).unwrap().unwrap();assert_eq!(d["owned_boolean_search"]["status"],"unsupported");assert_eq!(d["owned_boolean_search"]["reason"],"outside_owned_search_profile");assert_eq!(d["owned_boolean_search"]["execution_verified"],false);}}
 #[test]fn pattern_boundary_uses_actual_worker_predicate(){let mut v=serde_json::to_value(config()).unwrap();v["channels"]["owned_boolean_search"]["pattern"]=json!(2);let c=serde_json::from_value(v).unwrap();assert_eq!(requested_channel_disposition(&c).unwrap().unwrap()["owned_boolean_search"]["status"],"unsupported");}
 #[test]fn v1_to_v4_bytes_unchanged_against_original_function(){for ch in [json!({"schema":"crypto.ml-dataset.channel-requests/v1","boolean_consequence":{"mode":"produce","row_index":0}}),json!({"schema":"crypto.ml-dataset.channel-requests/v2","sat_initial":{"mode":"capture_and_check_initial"}}),json!({"schema":"crypto.ml-dataset.channel-requests/v3","affine_boolean_row":{"mode":"affine_unit_row","pattern":0,"row_index":0}}),json!({"schema":"crypto.ml-dataset.channel-requests/v4","sat_search":{"mode":"capture_and_check_model","events":1,"conflicts":1}})]{let mut c=config();c.channels=Some(serde_json::from_value(ch).unwrap());let before=legacy_disposition(&c).unwrap().unwrap();let after=requested_channel_disposition(&c).unwrap().unwrap();assert_eq!(serde_json::to_string(&before).unwrap(),serde_json::to_string(&after).unwrap());assert!(after.get("owned_boolean_search").is_none());let typed:RequestedChannelDispositions=serde_json::from_value(before.clone()).unwrap();assert!(typed.owned_boolean_search.is_none());assert_eq!(serde_json::to_value(typed).unwrap(),before);}}
 #[test]fn missing_channels_stays_absent(){let mut c=config();c.channels=None;assert_eq!(requested_channel_disposition(&c).unwrap(),None);}
 #[test]fn invalid_mixed_v5_never_gets_disposition(){let mut c=config();c.channels.as_mut().unwrap().sat_initial=Some(crate::ml_dataset::types::SatRequest::CaptureAndCheckInitial{});assert!(requested_channel_disposition(&c).is_err());}
 }
 }
}

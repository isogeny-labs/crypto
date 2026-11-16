use super::*;
fn end()->Instant{Instant::now()+std::time::Duration::from_secs(10)}
fn sample()->&'static[u8]{b"BQI1 1 1 1 1 0 1 1 0 1 0 0\n"}
#[test]fn success_preserves_legacy_result_and_every_resource_counter(){
 let plain=verify_untrusted(sample(),Limits::software(),end());assert!(plain.result.is_ok());let mut actual=[0u64;9];
 let admitted=verify_untrusted_admitted(sample(),Limits::software(),end(),&mut|r,n|{actual[r as usize]+=n;Ok(())});
 assert_eq!(admitted.result,plain.result);assert_eq!(admitted.charges.used,plain.charges.used);assert_eq!(actual,admitted.charges.used);
}
#[test]fn refusal_sweep_retains_exact_request_and_prior_successful_admissions(){
 let mut full=Vec::new();let a=verify_untrusted_admitted(sample(),Limits::software(),end(),&mut|r,n|{full.push((r,n));Ok(())});assert!(a.result.is_ok());
 for refused in 0..full.len(){let mut index=0;let mut accepted=[0;9];let mut rejected=None;
  let a=verify_untrusted_admitted(sample(),Limits::software(),end(),&mut|r,n|{let i=index;index+=1;if i==refused{rejected=Some((r,n));return Err(())}accepted[r as usize]+=n;Ok(())});
  let(r,n)=full[refused];assert_eq!(a.result,Err(Failure::Unknown(Exhaustion::Admission{resource:r,amount:n})));assert_eq!(rejected,Some((r,n)));assert_eq!(a.charges.used,accepted);assert_eq!(index,refused+1);
 }
}
#[test]fn allocation_refusal_happens_before_vector_allocation_and_counter_commit(){
 let mut calls=0;let mut reject=|r,n|{calls+=1;assert_eq!(r,Resource::AllocationBytes);assert_eq!(n,4096);Err(())};
 // Direct Meter with already admitted probe to isolate allocation transaction.
 // Normal charge first requests a probe, so admit that and reject allocation.
 let mut hook=|r,n|if r==Resource::DeadlineProbes{Ok(())}else{reject(r,n)};
 let mut m=Meter{limits:Limits::software(),charges:Charges::default(),deadline:end(),admission:Some(&mut hook)};
 let v=m.vector::<u64>(512);assert!(matches!(v,Err(Failure::Unknown(Exhaustion::Admission{resource:Resource::AllocationBytes,amount:4096}))));assert_eq!(m.charges.used[Resource::AllocationBytes as usize],0);assert_eq!(m.charges.used[Resource::DeadlineProbes as usize],1);drop(m);assert_eq!(calls,1);
}
#[test]fn local_caps_preserve_refusal_without_requesting_disallowed_resource(){
 let mut limits=Limits::software();limits.caps[Resource::EncodedBytes as usize]=0;let mut seen=Vec::new();let a=verify_untrusted_admitted(sample(),limits,end(),&mut|r,n|{seen.push((r,n));Ok(())});assert_eq!(a.result,Err(Failure::Unknown(Exhaustion::Resource(Resource::EncodedBytes))));assert_eq!(seen,vec![(Resource::DeadlineProbes,1)]);
}
#[test]fn expired_deadline_preserved_and_no_algorithm_resource_is_admitted(){
 let mut seen=Vec::new();let a=verify_untrusted_admitted(sample(),Limits::software(),Instant::now(),&mut|r,n|{seen.push((r,n));Ok(())});assert_eq!(a.result,Err(Failure::Unknown(Exhaustion::Deadline)));assert_eq!(seen,vec![(Resource::DeadlineProbes,1)]);
}
#[test]fn invalid_input_and_equality_failure_remain_failures(){
 for bytes in [&b"bad"[..],&b"BQI1 1 1 1 1 0 1 0 1 0 0\n"[..]]{let plain=verify_untrusted(bytes,Limits::software(),end());let a=verify_untrusted_admitted(bytes,Limits::software(),end(),&mut|_,_|Ok(()));assert!(plain.result.is_err());assert_eq!(plain.result,a.result);assert_eq!(plain.charges.used,a.charges.used);}
}
#[test]fn post_reservation_refusal_is_not_reported_as_uncharged_parent(){
 let mut parent=0;let a=verify_untrusted_admitted(sample(),Limits::software(),end(),&mut|_,n|{parent+=n;Err(())});assert_eq!(parent,1);assert_eq!(a.charges.used,[0;9]);assert_eq!(a.result,Err(Failure::Unknown(Exhaustion::Admission{resource:Resource::DeadlineProbes,amount:1})));
}

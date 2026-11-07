#[path="../feature-native-recycled-powers/division.rs"]mod bundled;
fn liveness_checks(){use compiled::{Schedule,Step};let(mut checks,mut strict)=(0,0);
 let mut lists:Vec<Vec<u64>>=(0..4096).map(|n|vec![n]).collect();lists.extend([vec![],vec![0,1,2,3,4,2,0],(0..128).chain([u64::MAX-1,u64::MAX]).collect(),vec![479,480,2,481,240,22,3,480]]);
 for requests in lists{let s=Schedule::new(&requests,4096).unwrap();let n=s.stats().nodes;let mut last:Vec<_>=(0..n).map(|i|[i;3]).collect();
  for(i,step)in s.steps.iter().enumerate(){let uses=match *step{Step::Odd{a,b,c,d,..}=>vec![(a,0),(b,2),(c,0),(d,2)],Step::Even{a,b,c,d,e}=>vec![(a,0),(b,0),(c,1),(d,0),(e,1)]};for(j,k)in uses{last[j][k]=last[j][k].max(i+5)}}
  for &(j,_)in &s.outputs{last[j][0]=n}
  let mut owners:Vec<Option<(usize,usize)>>=vec![None;s.field_slots()];let mut delta=vec![0i64;n+2];
  for i in 0..n{for k in 0..3{let present=k==0||s.power_flags[i]&(1<<(k-1))!=0;if !present{assert_eq!(s.slots[i][k],usize::MAX);continue}let slot=s.slots[i][k];if let Some((old,kind))=owners[slot]{assert!(last[old][kind]<i,"reuse at {i} while {old}/{kind} live")}owners[slot]=Some((i,k));delta[i]+=1;delta[last[i][k]+1]-=1;}}
  let(mut live,mut peak)=(0,0);for d in delta{live+=d;peak=peak.max(live)}assert_eq!(s.field_slots(),peak as usize);
  let prior=bundled::Schedule::new(&requests,4096).unwrap();assert!(s.field_slots()<=3*prior.live_slots());assert_eq!(s.saved_multiplications(),prior.saved_multiplications());
  if n>5&&s.field_slots()<3*prior.live_slots(){strict+=1}checks+=1;
 }
 assert!(strict>0);println!("typed_liveness_plans={checks} no_live_alias=true exact_interval_peak=true field_slots_le_three_bundle_slots=true strict_nonbase_reduction_exists=true");
}

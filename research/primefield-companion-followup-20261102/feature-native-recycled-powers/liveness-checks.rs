fn liveness_checks(){use compiled::{Schedule,Step};let(mut checks,mut strict)=(0,0);
 let mut lists:Vec<Vec<u64>>=(0..4096).map(|n|vec![n]).collect();lists.extend([vec![],vec![0,1,2,3,4,2,0],(0..128).chain([u64::MAX-1,u64::MAX]).collect(),vec![479,480,2,481,240,22,3,480]]);
 for requests in lists{let s=Schedule::new(&requests,4096).unwrap();let n=s.stats().nodes;let mut last:Vec<_>=(0..n).collect();
  for(i,step)in s.steps.iter().enumerate(){let ids=match *step{Step::Odd{a,b,c,d,..}=>vec![a,b,c,d],Step::Even{a,b,c,d,e}=>vec![a,b,c,d,e]};for j in ids{last[j]=last[j].max(i+5)}}
  for &(j,_)in &s.outputs{last[j]=n}
  let mut owners=vec![None;s.live_slots()];let mut delta=vec![0i64;n+2];
  for i in 0..n{let slot=s.slots[i];if let Some(old)=owners[slot]{assert!(last[old]<i,"reuse at {i} while {old} live")}owners[slot]=Some(i);delta[i]+=1;delta[last[i]+1]-=1;}
  let(mut live,mut peak)=(0,0);for d in delta{live+=d;peak=peak.max(live)}assert_eq!(s.live_slots(),peak as usize);assert!(s.live_slots()<=n);
  if s.live_slots()<n{strict+=1}checks+=1;
 }
 assert!(strict>0);println!("liveness_plans={checks} no_live_alias=true exact_interval_peak=true strict_reuse_exists=true");
}

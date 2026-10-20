use std::{env,fs,collections::BTreeMap};
fn main(){
 let args:Vec<String>=env::args().collect();
 let original=fs::read_to_string(&args[1]).unwrap(); let replay=fs::read_to_string(&args[2]).unwrap();
 let a:Vec<_>=original.lines().collect(); let b:Vec<_>=replay.lines().collect(); assert_eq!(a.len(),2963); assert_eq!(a.len(),b.len());
 let(mut text_diff,mut numeric_diff,mut changed,mut recorded_d_mismatch,mut recorded_p_mismatch)=(0,0,0,0,0);
 let(mut max_replay,mut max_d,mut max_p)=(0f64,0f64,0f64); let mut crossings=[0usize;3]; let mut groups=BTreeMap::<String,usize>::new();
 for (left,right) in a.iter().zip(&b){
  let l:Vec<_>=left.split('\t').collect(); let r:Vec<_>=right.split('\t').collect();assert_eq!(l.len(),10);assert_eq!(r.len(),10);
  for i in [0,1,2,3,4,5,6,8]{assert_eq!(l[i],r[i],"non-pvalue field changed at {} {} field {}",l[0],l[1],i);}
  for i in [7,9]{let x:f64=l[i].parse().unwrap();let y:f64=r[i].parse().unwrap();assert!(x.is_finite()&&y.is_finite()); if l[i]!=r[i]{text_diff+=1;} if x!=y{numeric_diff+=1;}max_replay=max_replay.max((x-y).abs());}
  let rd:f64=r[4].parse().unwrap();let rp:f64=r[5].parse().unwrap();let bd:f64=r[6].parse().unwrap();let bp:f64=r[7].parse().unwrap();let fd:f64=r[8].parse().unwrap();let fp:f64=r[9].parse().unwrap();
  if rd!=bd{recorded_d_mismatch+=1;} if rp!=bp{recorded_p_mismatch+=1;}
  if bd!=fd || bp!=fp {changed+=1;*groups.entry(r[0].split('/').nth(1).unwrap().to_string()).or_default()+=1;}
  max_d=max_d.max((bd-fd).abs());max_p=max_p.max((bp-fp).abs());
  for (i,t) in [0.05,0.01,0.001].iter().enumerate(){if (bp<*t)!=(fp<*t){crossings[i]+=1;}}
 }
 assert_eq!(recorded_d_mismatch,0);assert_eq!(changed,49);assert_eq!(crossings,[0,0,0]);
 println!("rows={}\ntext_p_fields_changed={}\nnumeric_p_fields_changed={}\nmax_archive_replay_abs_p_difference={:.17e}\nrecorded_vs_buggy_d_mismatches={}\nrecorded_vs_buggy_p_mismatches={}\ntie_changes={}\ngroups={:?}\nmax_tie_d_difference={:.17e}\nmax_tie_p_difference={:.17e}\ncrossings={:?}",a.len(),text_diff,numeric_diff,max_replay,recorded_d_mismatch,recorded_p_mismatch,changed,groups,max_d,max_p,crossings);
}

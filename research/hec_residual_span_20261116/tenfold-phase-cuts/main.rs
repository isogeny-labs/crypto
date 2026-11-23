use std::{collections::BTreeMap,fs};
const NAMES:[&str;5]=["prepare","source","anchor","scan","verify"];
fn rows(path:&str)->Vec<BTreeMap<String,String>>{let s=fs::read_to_string(path).unwrap();let mut ls=s.lines();let header:Vec<_>=ls.next().unwrap().split('\t').map(str::to_owned).collect();ls.map(|l|{let v:Vec<_>=l.split('\t').collect();assert_eq!(v.len(),header.len());header.iter().cloned().zip(v.iter().map(|s|s.to_string())).collect()}).collect()}
fn key(r:&BTreeMap<String,String>)->String{format!("{}/{}/{}",r["p"],r["sm"],r["case"])}
fn minimal(valid:&[u32])->Vec<u32>{valid.iter().copied().filter(|m|!valid.iter().any(|q|q!=m&&(q&m)==*q)).collect()}
fn names(mask:u32)->String{NAMES.iter().enumerate().filter(|(i,_)|mask&(1<<i)!=0).map(|(_,s)|*s).collect::<Vec<_>>().join("+")}
fn main(){
 let b="research/hec_residual_span_20261116";let old=rows(&format!("{b}/source-phases/SUMMARY.tsv"));let short=rows(&format!("{b}/short-verification/SUMMARY.tsv"));let raw=fs::read_to_string(format!("{b}/short-verification-integrated/CONTROL.txt")).unwrap();let mut global_original=vec![true;32];let mut global_strong=vec![true;32];
 assert_eq!(old.len(),12);println!("p\tsm\tcase\tbudget_kind\treference_total\tcurrent_total\tinteger_budget\tphase_costs\tindividually_required\tminimal_changed_sets");
 for r in old{let new=short.iter().find(|x|key(x)==key(&r)).unwrap();let costs:[u64;5]=std::array::from_fn(|i|if i==4{new["new_verify"].parse().unwrap()}else{r[NAMES[i]].parse().unwrap()});let current=costs.iter().sum::<u64>();let original:u64=r["total"].parse().unwrap();assert_eq!(current,new["derived_new_total"].parse::<u64>().unwrap());assert_eq!(r["linear"],"0");
 let prefix=format!("COUNT p={} sm={} case={} rep=0 modulus={} vector=[",r["p"],r["sm"],r["case"],r["p"]);let line=raw.lines().find(|l|l.starts_with(&prefix)).unwrap();let vals:Vec<u64>=line.split("vector=[").nth(1).unwrap().trim_end_matches(']').split(',').map(|s|s.trim().parse().unwrap()).collect();assert_eq!(vals[8],current);
 for(kind,reference,global)in[("fixed_original",original,&mut global_original),("stronger_current",current,&mut global_strong)]{let budget=reference/10;let valid:Vec<u32>=(0u32..32).filter(|mask|costs.iter().enumerate().filter(|(i,_)|mask&(1<<i)==0).map(|(_,v)|*v).sum::<u64>()<=budget).collect();let required=(0..5).filter(|&i|costs[i]>budget).fold(0,|m,i|m|(1<<i));for mask in 0..32{global[mask]&=valid.contains(&(mask as u32));}
 println!("{}\t{}\t{}\t{kind}\t{reference}\t{current}\t{budget}\t{}\t{}\t{}",r["p"],r["sm"],r["case"],costs.iter().map(u64::to_string).collect::<Vec<_>>().join(","),names(required),minimal(&valid).iter().map(|m|names(*m)).collect::<Vec<_>>().join(" OR "));
 for i in 0..5{let unchanged=current-costs[i];println!("SINGLE key={} budget_kind={kind} phase={} remaining={unchanged} tenfold_possible_if_phase_free={}",key(&r),NAMES[i],10*unchanged<=reference);}
 }}
 for(kind,global)in[("fixed_original",global_original),("stronger_current",global_strong)]{let valid:Vec<_>=(0..32).filter(|&i|global[i]).map(|i|i as u32).collect();println!("GLOBAL budget_kind={kind} minimal_uniform_changed_sets={}",minimal(&valid).iter().map(|m|names(*m)).collect::<Vec<_>>().join(" OR "));}
 eprintln!("PASS twelve_integrated_totals_reconciled=true all32phase_subsets_checked=true fixed_original_target_preserved=true stronger_budget_separate=true optimistic_necessary_conditions_only=true no_new_measurement_or_speed_claim=true");
}

mod sparse;
mod scheduler;
#[cfg(test)]mod tests{
 use super::{scheduler::*,sparse::{Charge,Owner,Polynomial}};use std::time::{Instant,Duration};
 #[derive(Default)]struct Ledger{work:u64,bytes:u64,checks:u64,stop:Option<u64>}
 impl Owner for Ledger{fn reserve(&mut self,q:Charge)->Result<(),()>{self.work+=q.work;self.bytes+=q.bytes;Ok(())}fn check(&mut self)->Result<(),()>{self.checks+=1;if self.stop.is_some_and(|s|self.checks>=s){Err(())}else{Ok(())}}}
 fn limits()->super::scheduler::Limits{super::scheduler::Limits{variables:256,generators:64,basis:64,tasks:20000,terms:512,proof:4096,degree:256,work:20_000_000,bytes:200_000_000}}
 fn poly(bits:u64,n:u32)->Polynomial{let mut p=vec![];for mask in 0..1u32<<n{if bits>>mask&1!=0{p.push((0..n).filter(|v|mask>>v&1!=0).collect::<Vec<_>>())}}p.sort_by(|a,b|a.len().cmp(&b.len()).then(a.cmp(b)));p}
 fn bits(p:&Polynomial)->u64{p.iter().fold(0,|b,t|b^(1u64<<t.iter().fold(0,|m,v|m|(1u32<<v))))}
 fn mul(a:u64,m:u32,n:u32)->u64{let mut r=0;for t in 0..1u32<<n{if a>>t&1!=0{r^=1<< (t|m)}}r}
 fn span(mut rows:Vec<u64>)->Vec<u64>{let mut b=[0u64;64];for mut x in rows.drain(..){while x!=0{let k=63-x.leading_zeros()as usize;if b[k]==0{b[k]=x;break}x^=b[k]}}b.into_iter().filter(|x|*x!=0).collect()}
 fn in_span(mut x:u64,b:&[u64])->bool{for r in b.iter().rev(){let k=63-r.leading_zeros();if x>>k&1!=0{x^=r}}x==0}
 fn remainder(mut p:u64,b:&[Row],n:u32)->u64{for _ in 0..1000{if p==0{return 0}let leading=|x:u64|(0..1u32<<n).filter(|m|x>>m&1!=0).max_by_key(|m|(m.count_ones(),(0..n).filter(|v|m>>v&1!=0).collect::<Vec<_>>())).unwrap();let lm=leading(p);let mut hit=false;for r in b{let q=bits(&r.polynomial);let bl=leading(q);if bl&lm==bl{let scaled=mul(q,lm^bl,n);if scaled!=0&&leading(scaled)==lm{p^=scaled;hit=true;break}}}if !hit{return p}}panic!("nonterminating independent reduction")}
 fn proof(g:&[Polynomial],r:&Row)->bool{let mut out=std::collections::BTreeSet::new();for p in &r.proof{if p.generator>=g.len(){return false}for t in &g[p.generator]{let mut u=t.clone();u.extend(&p.multiplier);u.sort();u.dedup();if !out.insert(u.clone()){out.remove(&u);}}}out==r.polynomial.iter().cloned().collect()}
 fn check(g:&[Polynomial],a:&Attempt){for r in &a.basis{assert!(proof(g,r));}assert!(proof(g,&a.current));if let Some(t)=a.active{assert!(a.pending.iter().any(|q|q.serial==t.serial));}assert!(!a.complete||a.result.is_ok());}
 fn runit(n:u32,g:&[Polynomial])->Attempt{let mut owner=Ledger::default();let a=run(n,g,limits(),Instant::now()+Duration::from_secs(10),&mut owner);assert_eq!(owner.work,a.charge.work);assert_eq!(owner.bytes,a.charge.bytes);check(g,&a);a}
 #[test]fn exhaustive_two_variable_ideals_match_independent_vector_span(){for x in 0..16{for y in 0..16{let g=vec![poly(x,2),poly(y,2)];let a=runit(2,&g);assert_eq!(a.result,Ok(()));assert!(a.complete);let expected=span((0..4).flat_map(|m|[mul(x,m,2),mul(y,m,2)]).collect());let got=span(a.basis.iter().flat_map(|r|(0..1u32<<2).map(move|m|mul(bits(&r.polynomial),m,2))).collect());assert_eq!(expected.len(),got.len());for mask in 0..1usize<<expected.len(){let r=expected.iter().enumerate().filter(|(i,_)|mask>>i&1!=0).fold(0,|x,(_,r)|x^r);assert!(in_span(r,&got));assert_eq!(remainder(r,&a.basis,2),0);}}}}
 #[test]fn three_variable_ideals_and_derivations(){for seed in 1..33u64{let gs=[seed*7%256,seed*11%256,seed*19%256];let g:Vec<_>=gs.iter().map(|x|poly(*x,3)).collect();let a=runit(3,&g);assert!(a.complete,"{:?}",a.result);let expected=span((0..8).flat_map(|m|gs.map(|x|mul(x,m,3))).collect());let got=span(a.basis.iter().flat_map(|r|(0..1u32<<3).map(move|m|mul(bits(&r.polynomial),m,3))).collect());assert_eq!(expected.len(),got.len());for mask in 0..1usize<<expected.len(){let r=expected.iter().enumerate().filter(|(i,_)|mask>>i&1!=0).fold(0,|x,(_,r)|x^r);assert!(in_span(r,&got));assert_eq!(remainder(r,&a.basis,3),0);}}}
 #[test]fn field_pairs_are_required_for_one_generator(){// xy+x+y -> multiplying by x gives x, by y gives y. No ordinary initial pair exists.
 let g=vec![poly(0b1110,2)];let a=runit(2,&g);assert!(a.complete);assert!(a.field_tasks>0);let b=span(a.basis.iter().flat_map(|r|(0..4).map(move|m|mul(bits(&r.polynomial),m,2))).collect());assert!(in_span(0b0010,&b));assert!(in_span(0b0100,&b));assert_eq!(b.len(),3);}
 #[test]fn high_index_nontrivial_elimination_and_partial_retention(){let g=vec![vec![vec![128],vec![128,129]],vec![vec![129],vec![128,129]]];let mut l=limits();l.degree=2;let a=run(130,&g,l,Instant::now()+Duration::from_secs(10),&mut Ledger::default());check(&g,&a);assert!(!a.complete);assert_eq!(a.result,Err(super::scheduler::Failure::Cap));assert!(a.row_xors>0);assert!(a.basis.iter().any(|r|r.polynomial==vec![vec![128],vec![129]]));}
 #[test]fn owner_refusals_retain_every_committed_proof(){let g=vec![poly(14,2),poly(7,2)];for at in 1..1500{let mut owner=Ledger{stop:Some(at),..Default::default()};let a=run(2,&g,limits(),Instant::now()+Duration::from_secs(10),&mut owner);check(&g,&a);assert_eq!(a.charge.work,owner.work);assert_eq!(a.charge.bytes,owner.bytes);if !a.complete{assert_eq!(a.result,Err(super::scheduler::Failure::Owner));}}}
 #[test]fn cumulative_caps_deadline_and_malformed(){let g=vec![poly(14,2)];for cap in [0,1,100,1000,10000]{let mut l=limits();l.work=cap;let a=run(2,&g,l,Instant::now()+Duration::from_secs(10),&mut Ledger::default());check(&g,&a);assert!(a.charge.work<=cap);}let a=run(2,&g,limits(),Instant::now(),&mut Ledger::default());assert_eq!(a.result,Err(super::scheduler::Failure::Deadline));let bad=vec![vec![vec![1],vec![0]]];assert_eq!(run(2,&bad,limits(),Instant::now()+Duration::from_secs(1),&mut Ledger::default()).result,Err(super::scheduler::Failure::Malformed));}
 #[test]fn zero_generators_stop_during_validation_and_task_admission(){
  // Every empty generator used to bypass validate's internal term checks.
  // Initial check is #1; generator validation checks are #2..#65.
  let g=vec![vec![];64];let mut owner=Ledger{stop:Some(5),..Default::default()};
  let a=run(2,&g,limits(),Instant::now()+Duration::from_secs(10),&mut owner);
  assert_eq!(a.result,Err(super::scheduler::Failure::Owner));
  assert_eq!(owner.checks,5);assert_eq!(a.charge.work,5);
  assert!(a.pending.is_empty());assert!(a.basis.is_empty());assert_eq!(a.completed,0);
  // Two preallocated vectors add checks #66,#67; first three task admissions
  // are #68,#69,#70. Refusal on #70 must retain only the first two originals.
  let mut owner=Ledger{stop:Some(70),..Default::default()};
  let a=run(2,&g,limits(),Instant::now()+Duration::from_secs(10),&mut owner);
  assert_eq!(a.result,Err(super::scheduler::Failure::Owner));
  assert_eq!(owner.checks,70);assert_eq!(a.pending.len(),2);
  assert_eq!(a.pending[0].kind,Kind::Original(0));assert_eq!(a.pending[1].kind,Kind::Original(1));
  assert!(a.active.is_none());assert_eq!(a.completed,0);assert!(!a.complete);
  assert_eq!(a.charge.work,owner.work);assert_eq!(a.charge.bytes,owner.bytes);
  check(&g,&a);
 }

}

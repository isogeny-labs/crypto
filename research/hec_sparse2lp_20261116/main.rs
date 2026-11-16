#[path="../hec_native2lp_20261116/graph.rs"] mod graph;
#[path="../hec_native2lp_20261116/native.rs"] mod reference;
mod native;
use hec_native_ic::instance::{pick_instance,factor_base};
fn main(){
 let mut comparisons=0;let mut supplied=0;
 for(g,p)in[(2,101),(3,61),(4,31)]{
  let inst=pick_instance(p,g).unwrap();let m=factor_base(&inst.curve).len()/2;
  for seed in[20260916,20260917]{for lp in[1,2]{
   let target=inst.curve.mul(&inst.d1,inst.ell/3+7);
   let baseline=reference::run(&inst,m,lp,seed,20_000);
   let candidate=native::run(&inst,&target,m,lp,seed,20_000);
   assert_eq!(candidate.k,baseline.k);assert!(candidate.k.is_some());
   assert_eq!((candidate.trials,candidate.rows,candidate.relation_checks),(baseline.trials,baseline.rows,baseline.relation_checks));
   assert_eq!(candidate.solve.pivot_hash,baseline.solve.pivot_hash);
   println!("EQUIVALENCE genus={g} p={p} lp={lp} seed={seed} trials={} rows={} scalar_verified=true pivot_hash_equal=true",candidate.trials,candidate.rows);comparisons+=1;
  }}
  // Construct outside the solver; its API receives only the public divisor.
  let target=inst.curve.mul(&inst.d1,inst.ell/5+11);
  let answer=native::run(&inst,&target,m,2,20260918,20_000);
  let k=answer.k.expect("supplied target cap");assert_eq!(inst.curve.mul(&inst.d1,k),target);
  println!("SUPPLIED_TARGET genus={g} p={p} k={k} source_group_verified=true");supplied+=1;
 }
 assert_eq!((comparisons,supplied),(12,3));println!("PASS equivalence=12 supplied_targets=3 timing_claim=false");
}

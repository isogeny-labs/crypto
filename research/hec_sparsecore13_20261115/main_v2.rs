mod sparse;
mod setup {
 pub fn qf()->[u64;15] {std::fs::read_to_string(std::env::args().nth(1).expect("runtime quartic")).unwrap().split_whitespace().map(|s|s.parse().unwrap()).collect::<Vec<_>>().try_into().unwrap()}
}
#[allow(dead_code)] mod target {include!("../hec_alignment13_20261115/atlas/target.rs");include!("../hec_alignment13_20261115/atlas/target_api.rs");}
#[allow(dead_code)] mod lp_trace {include!("../hec_chartcold13_20261115/lp_trace.rs");}
mod candidate {include!("point_sparse_v2.rs");}
mod reference {include!("point_reference.rs");}
fn main(){
 let (p,a)=candidate::fresh();let(q,b)=reference::fresh();assert_eq!(p,q);
 let ell=1028893;let f=hec_native_ic::fp::Fp::new(ell);let i=a.iter().position(|&a|a!=0).unwrap();assert_ne!(b[i],0);let ratio=f.mul(a[i],f.inv(b[i]));assert!(a.iter().zip(&b).all(|(&a,&b)|a==f.mul(ratio,b)),"independent_dense_reference");
 for n in [47,128,512] {
  let planted:Vec<_>=(0..n).map(|i|i as u64+1).collect();
  let rows:Vec<sparse::Row>=(0..n-1).map(|i|vec![(i,planted[i+1]),(i+1,ell-planted[i])]).collect();
  for seed in 0..4 {
   let x=sparse::kernel(&rows,n,ell,seed,32).expect("planted_retry_cap");
   assert!(x.iter().zip(&planted).all(|(&a,&b)|a==f.mul(x[0],b)));
   let mut bad=rows.clone();bad[0][0].1=f.add(bad[0][0].1,1);assert!(!sparse::valid(&bad,&x,ell));assert!(!sparse::valid(&rows,&vec![0;n],ell));
  }
  println!("PLANTED_PASS columns={n} seeds=4 known_rank={} corrupted_original_and_zero_rejected=true",n-1);
 }
 println!("PASS sparse_reduced_core=true all129_point_logs_projectively_match=true planted_cases=12 timing_claim=false full_pipeline_integrated=false");
}

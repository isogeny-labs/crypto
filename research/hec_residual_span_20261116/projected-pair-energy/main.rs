use hec_native_ic::{fp::Fp,poly::Poly,jac::Curve,instance::factor_base};
use std::collections::HashMap;
fn main(){
 for (g,p,c,ell,h) in [(2usize,251u64,3u64,101u64,55u64),(2,1021,4,631,1655),(3,211,2,301267,29),(4,181,2,48781,4445),(4,271,2,397,112347)]{
  assert!((2..).take_while(|q|q*q<=ell).all(|q|ell%q!=0));assert_ne!(h%ell,0);
  let mut cs=vec![0;2*g+2];cs[0]=c;cs[2*g+1]=1;let curve=Curve::new(Fp::new(p),Poly::from_coeffs(&cs));
  let mut raw=vec![];for (x,y) in factor_base(&curve){let d=curve.place(x,y);raw.push(d);if y!=0{raw.push(curve.neg(&d));}}
  let m=raw.len() as u64;let mut images=HashMap::new();for d in &raw {let v=curve.mul(d,h);assert!(curve.is_valid(&v));assert!(curve.is_zero(&curve.mul(&v,ell)));*images.entry(v).or_insert(0u64)+=1;}
  let source=*images.keys().filter(|d|!curve.is_zero(d)).min_by_key(|d|format!("{d:?}")).unwrap();
  assert!(curve.is_zero(&curve.mul(&source,ell)));let mut labels=vec![0u64;ell as usize];let mut d=curve.zero();let mut found=0;
  /* Diagnostic exhaustive cyclic-group walk, not a decomposition algorithm or a free log table. */
  for k in 0..ell {if k>0{assert!(!curve.is_zero(&d));}if let Some(&weight)=images.get(&d){labels[k as usize]=weight;found+=1;}d=curve.add(&d,&source);}
  assert!(curve.is_zero(&d));assert_eq!(found,images.len());assert_eq!(labels.iter().sum::<u64>(),m);
  let support:Vec<_>=labels.iter().enumerate().filter(|(_,w)|**w>0).map(|(i,w)|(i,*w)).collect();
  let mut weighted=vec![0u64;ell as usize];let mut distinct=vec![0u64;ell as usize];
  for &(a,wa) in &support {for &(b,wb) in &support {let j=(a+b)%ell as usize;weighted[j]+=wa*wb;distinct[j]+=1;}}
  assert_eq!(weighted.iter().sum::<u64>(),m*m);assert_eq!(distinct.iter().sum::<u64>(),(support.len()*support.len()) as u64);
  for (i,&(a,_)) in support.iter().take(16).enumerate(){let b=support[(i*7+3)%support.len()].0;let da=curve.mul(&source,a as u64);let db=curve.mul(&source,b as u64);assert_eq!(curve.add(&da,&db),curve.mul(&source,((a+b)%ell as usize) as u64));}
  for i in 0..raw.len().min(16){let j=(i*7+3)%raw.len();assert_eq!(curve.mul(&curve.add(&raw[i],&raw[j]),h),curve.add(&curve.mul(&raw[i],h),&curve.mul(&raw[j],h)));}
  let energy=weighted.iter().map(|v|v*v).sum::<u64>();let set_energy=distinct.iter().map(|v|v*v).sum::<u64>();let coverage=weighted.iter().filter(|v|**v>0).count();
  assert_eq!(coverage,distinct.iter().filter(|v|**v>0).count());
  println!("CENSUS g={g} p={p} c={c} ell={ell} h={h} raw_signed={m} distinct_images={} image_zero_weight={} pair_support={coverage} weighted_energy={energy} set_energy={set_energy} weighted_zero_pairs={} set_zero_pairs={} max_weighted={} max_set={} coverage_fraction={coverage}/{ell}",support.len(),labels[0],weighted[0],distinct[0],weighted.iter().max().unwrap(),distinct.iter().max().unwrap());
  println!("EXACT mean_ordered_representations={}/{ell} weighted_energy_vs_uniform_ratio={}/{} deduplicated_mean={}/{ell}",m*m,energy*ell,m.pow(4),support.len()*support.len());
 }
 eprintln!("PASS all_five_proper_subgroups=true cyclic_membership_exhaustively_verified=true weighted_and_deduplicated_convolutions=true native_controls=true diagnostic_logs_not_solver_inputs=true no_speed_claim=true");
}

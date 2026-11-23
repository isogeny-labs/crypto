use hec_native_ic::{fp::Fp,poly::Poly,jac::Curve,instance::factor_base};
use std::collections::{HashMap,HashSet,BTreeMap};
fn main(){
 for (g,p,c) in [(2usize,251u64,18u64),(3,421,2),(4,271,2)] {
  let mut cs=vec![0;2*g+2];cs[0]=c;cs[2*g+1]=1;let curve=Curve::new(Fp::new(p),Poly::from_coeffs(&cs));
  let canonical=factor_base(&curve);let w=canonical.iter().filter(|q|q.1==0).count() as u64;
  let mut base=vec![];for &(x,y) in &canonical {let d=curve.place(x,y);base.push(d);if y!=0{base.push(curve.neg(&d));}}
  let m=base.len() as u64;assert_eq!(base.iter().copied().collect::<HashSet<_>>().len(),base.len());
  assert!(base.iter().all(|d|curve.is_valid(d)&&!curve.is_zero(d)));
  let mut sums=HashMap::new();for a in &base {for b in &base {let s=curve.add(a,b);assert!(curve.is_valid(&s));assert!(s.u.deg()<=2);*sums.entry(s).or_insert(0u64)+=1;}}
  assert_eq!(sums[&curve.zero()],m);let mut nonzero_hist=BTreeMap::new();let mut energy=0u64;let mut total=0u64;
  for (d,count) in &sums {total+=count;energy+=count*count;if !curve.is_zero(d){*nonzero_hist.entry(*count).or_insert(0u64)+=1;}}
  assert_eq!(total,m*m);assert_eq!(nonzero_hist.get(&1).copied().unwrap_or(0),m-w);
  assert_eq!(nonzero_hist.get(&2).copied().unwrap_or(0),(m*m-2*m+w)/2);assert!(nonzero_hist.keys().all(|k|*k==1||*k==2));
  assert_eq!(energy,3*m*m-3*m+w);assert_eq!(sums.len() as u64,1+(m*m-w)/2);
  println!("CENSUS genus={g} p={p} c={c} signed_base={m} affine_weierstrass={w} ordered_pairs={total} sumset={} zero_multiplicity={m} nonzero_hist={nonzero_hist:?} energy={energy} predicted_energy={}",sums.len(),3*m*m-3*m+w);
 }
 eprintln!("PASS all_three_genera=true exact_ordered_pair_census=true all_sums_valid=true reduced_uniqueness_histogram=true ambient_jacobians_only=true no_subgroup_order_or_speed_claim=true");
}

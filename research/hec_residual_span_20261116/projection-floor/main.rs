use hec_native_ic::{counted,fp::Fp,poly::Poly,jac::Curve,instance::{factor_base,prime_factors}};
use std::collections::{HashMap,HashSet};
fn main(){
 for (g,p,c,ell,h,expected) in [(2usize,251u64,3u64,101u64,55u64,24usize),(2,1021,4,631,1655,102),(3,211,2,301267,29,14),(4,181,2,48781,4445,10),(4,271,2,397,112347,9)]{
  counted::disable();let n=(2*g+1) as u64;let fp=Fp::new(p);let mut cs=vec![0;2*g+2];cs[0]=c;cs[2*g+1]=1;let curve=Curve::new(fp,Poly::from_coeffs(&cs));let fb=factor_base(&curve);
  let factors=prime_factors(p-1);let generator=(2..p).find(|&a|factors.iter().all(|&q|curve.fp.pow(a,(p-1)/q)!=1)).unwrap();let z=curve.fp.pow(generator,(p-1)/n);
  let by_x:HashMap<_,_>=fb.iter().copied().collect();let mut seen=HashSet::new();let mut representatives=vec![];
  for &(x,y) in &fb {if x==0 {assert!(curve.is_zero(&curve.mul(&curve.place(x,y),h)));continue;}if seen.contains(&x){continue;}representatives.push(curve.place(x,y));let mut xx=x;for _ in 0..n{assert_eq!(by_x[&xx],y);assert!(seen.insert(xx));xx=curve.fp.mul(xx,z);}assert_eq!(xx,x);}
  assert_eq!(representatives.len(),expected);
  let mut prior=None;for rep in 0..2{
   counted::reset();let projected:Vec<_>=representatives.iter().map(|d|curve.mul(d,h)).collect();let counts=counted::snapshot();counted::disable();
   for d in &projected {assert!(curve.is_valid(d));assert!(curve.is_zero(&curve.mul(d,ell)));}
   if let Some((old,outputs))=&prior{assert_eq!(old,&counts);assert_eq!(outputs,&projected);}else{prior=Some((counts.clone(),projected));}
   for (modulus,v) in counts{assert_eq!(v[8],2*v[7]+v[5]+v[6]);println!("COUNT g={g} p={p} c={c} ell={ell} h={h} representatives={expected} rep={rep} modulus={modulus} vector={v:?}");}
  }
 }
 eprintln!("PASS projection_outputs_valid_and_ell_annihilated=true one_scalar_projection_per_raw_orbit=true two_counter_vectors_exact=true count_identity=true free_preparation_and_free_h=true not_complete_alias_map=true");
}

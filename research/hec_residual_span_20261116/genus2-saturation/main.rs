use hec_native_ic::{fp::Fp, poly::Poly,jac::Curve, instance::factor_base,oracle::{split,OracleStats}};
fn main(){
 let fp=Fp::new(251); let curve=Curve::new(fp,Poly::from_coeffs(&[18,0,0,0,0,1]));
 let fb=factor_base(&curve);let w=fb.iter().filter(|p|p.1==0).count() as u64;let m=2*fb.len() as u64-w;
 let ell=67931u64;assert!((2..).take_while(|q|q*q<=ell).all(|q|ell%q!=0));
 let generator=curve.place(fb[0].0,fb[0].1);assert!(!curve.is_zero(&generator));assert!(curve.is_zero(&curve.mul(&generator,ell)));
 let mut d=curve.zero();let mut stats=OracleStats::default();let mut degrees=[0u64;3];let mut smooth=0;
 for k in 0..ell {
  assert!(curve.is_valid(&d));if k>0 {assert!(!curve.is_zero(&d));}
  let degree=d.u.deg() as usize;assert!(degree<=2);degrees[degree]+=1;
  if split(&d.u,&curve.fp,&mut stats).is_some(){smooth+=1;}
  d=curve.add(&d,&generator);
 }
 assert!(curve.is_zero(&d));let predicted=1+m+(m*m-w)/2;assert_eq!(smooth,predicted);assert_eq!(degrees[0],1);assert_eq!(degrees[1],m);
 println!("CENSUS p=251 c=18 subgroup_order={ell} canonical_places={} affine_points={m} affine_weierstrass={w} degree_counts={degrees:?} smooth={smooth} nonsmooth={} predicted_smooth={predicted}",fb.len(),ell-smooth);
 println!("CEILING uniform_full_subgroup_success_only={ell}/{smooth} decimal={:.12} tenfold_requires_smooth_at_most={}",ell as f64/smooth as f64,ell/10);
 assert!(10*smooth>ell);eprintln!("PASS full_prime_order_subgroup_enumerated=true all_valid=true closed_walk=true exact_smooth_count_identity=true tenfold_from_success_probability_alone_excluded_on_this_uniform_full_base_instance=true");
}

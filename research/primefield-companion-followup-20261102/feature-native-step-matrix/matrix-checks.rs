#[path="../feature-native-singleton-resultant/polynomial.rs"]mod baseline;
fn test_poly(p:u64,n:usize,seed:u64)->Vec<u64>{let mut a:Vec<_>=(0..=n).map(|i|((i as u64*17+seed*13+3)*(i as u64+seed+1))%p).collect();a[n]=seed%(p-1)+1;a}
pub fn matrix_checks(){
 let(mut local,mut full,mut nontrivial,mut zero_resultants)=(0,0,0,0);
 for p in [5u64,17,97,257]{let mut poly=Poly::new(p).unwrap();
  for n in [0usize,1,2,7,13,32]{for seed in 0..12u64{
   let mut m:Matrix=std::array::from_fn(|i|std::array::from_fn(|j|test_poly(p,(n+i+j)%34,seed+(2*i+j)as u64)));
   if seed%3==0{m[0][0]=vec![0]}if seed%4==0{m[1][1]=vec![0]}
   let q=if seed==0{vec![0]}else{test_poly(p,n/2,seed)};
   let expected_saved=m.iter().flatten().map(|c|if c==&[0]{0}else{c.len()}).sum::<usize>();
   let step=poly.step_matrix(&q);reset_audit();let expected=poly.matrix_mul(&step,&m);let before=audit();
   let ptrs=[m[1][0].as_ptr(),m[1][1].as_ptr()];let caps=[m[1][0].capacity(),m[1][1].capacity()];
   reset_audit();STEP_AUDIT.with(|a|a.set([0;2]));let actual=poly.step_left(&q,m);let after=audit();
   assert_eq!(actual,expected);assert_eq!(before[0],after[0]+expected_saved);assert_eq!(before[1],after[1]);
   assert_eq!([actual[0][0].as_ptr(),actual[0][1].as_ptr()],ptrs);assert_eq!([actual[0][0].capacity(),actual[0][1].capacity()],caps);
   assert_eq!(STEP_AUDIT.with(|a|a.get()),[1,expected_saved]);local+=1;
  }}
  for n in [9usize,10,16,17,31,32,48,65,96]{for seed in 0..12u64{
   let mut old=baseline::Poly::new(p).unwrap();let a=test_poly(p,n,seed);let b=if seed==0{a[..n].to_vec()}else if seed==1{poly.mul(&[1,1],&test_poly(p,n-3,seed))}else{test_poly(p,n-1-(seed as usize%3),seed+1)};
   let a=if seed==1{poly.mul(&[1,1],&test_poly(p,n-1,seed+3))}else{a};
   baseline::reset_audit();let expected=old.resultant_class(&a,&b).unwrap().project();let before=baseline::audit();
   reset_audit();STEP_AUDIT.with(|x|x.set([0;2]));let actual=poly.resultant_class(&a,&b).unwrap().project();let after=audit();let stats=STEP_AUDIT.with(|x|x.get());
   assert_eq!(actual,expected,"p={p} n={n} seed={seed}");assert_eq!(before[0],after[0]+stats[1],"full operation preservation");assert_eq!(before[1..],after[1..]);
   if stats[0]>0{assert!(stats[1]>=2*stats[0]);nontrivial+=1;}
   if actual==0{zero_resultants+=1;}full+=1;
  }}
 }
 assert!(nontrivial>0&&zero_resultants>0);
 println!("matrix_local_cases={local} row_buffers_reused=true exact_identity_product_saving=true resultant_cases={full} hgcd_active_cases={nontrivial} zero_resultants={zero_resultants} full_count_delta_matches=true");
}

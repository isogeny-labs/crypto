fn singleton_checks(){
 let(mut checks,mut zeros,mut orientation_controls,mut fallbacks)=(0usize,0usize,0usize,0usize);
 for p in [5u64,7,17,41,97,193,257,509]{
  for n in 1..=65usize{for seed in 0..8u64{for orientation in 0..2{
   let mut list:Vec<_>=(0..n).map(|i|match seed%4{0=>seed,1=>(i as u64*7+seed)%p,2=>p+(i as u64*i as u64+seed)%p,_=>u64::MAX-i as u64}).collect();
   let single=[(seed*13+1)%p];if seed==4{list[n/2]=single[0];}
   let(roots,points)=if orientation==0{(list.as_slice(),single.as_slice())}else{(single.as_slice(),list.as_slice())};
   let mut poly=polynomial::Poly::new(p).unwrap();
   let f=poly.root_product(roots).unwrap();let g=poly.root_product(points).unwrap();
   let expected=poly.resultant_class(&g,&f).unwrap().project();
   let mut direct=1;for &y in points{for &x in roots{direct=mul(direct,(y%p+p-x%p)%p,p)}}
   let q=1u64<<(p-1).trailing_zeros();assert_eq!(expected,pow(direct,(p-1)/q,p));
   polynomial::reset_audit();let actual=poly.root_resultant_class(roots,points).unwrap();
   assert_eq!(polynomial::audit(),[n-1,0,0],"p={p} n={n} orientation={orientation}");
   assert_eq!(actual.project(),expected);assert_eq!(poly.root_projected_resultant(roots,points).unwrap(),expected);
   if direct==0{zeros+=1}else if n%2==1 && expected!=pow((p-direct)%p,(p-1)/q,p){orientation_controls+=1;}
   checks+=1;
  }}}
  let mut poly=polynomial::Poly::new(p).unwrap();
  for(a,b)in [(vec![],vec![]),(vec![],vec![1]),(vec![1],vec![])]{
   polynomial::reset_audit();assert!(matches!(poly.root_resultant_class(&a,&b),Err(polynomial::Error::EmptyTree)));assert_eq!(polynomial::audit(),[0,0,0]);
  }
  for n in 2..=17{let roots:Vec<_>=(0..n).map(|i|(i*3+1)%p).collect();let points:Vec<_>=(0..n+1).map(|i|(i*7+2)%p).collect();let f=poly.root_product(&roots).unwrap();let g=poly.root_product(&points).unwrap();let expected=poly.resultant_class(&g,&f).unwrap().project();polynomial::reset_audit();assert_eq!(poly.root_resultant_class(&roots,&points).unwrap().project(),expected);assert!(polynomial::audit()[2]>0);fallbacks+=1;}
 }
 assert!(zeros>0&&orientation_controls>0);
 // Near-domain boundary exercises overflow-safe canonicalization at prime 2^61-1.
 let p=(1u64<<61)-1;let mut poly=polynomial::Poly::new(p).unwrap();let roots=[u64::MAX,p-1,p+1];let y=[u64::MAX-7];let direct=roots.iter().fold(1,|a,&x|mul(a,(y[0]%p+p-x%p)%p,p));polynomial::reset_audit();let actual=poly.root_resultant_class(&roots,&y).unwrap();assert_eq!(polynomial::audit(),[2,0,0]);assert_eq!(actual.project(),pow(direct,(p-1)/2,p));
 println!("singleton_cases={checks} zero_cases={zeros} orientation_controls={orientation_controls} fallback_cases={fallbacks} empty_controls=24 large_prime_controls=1 exact_products=N-1 inversions=0 root_constructions=0");
}

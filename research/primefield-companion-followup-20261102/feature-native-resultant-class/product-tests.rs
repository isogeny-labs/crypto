
#[test]fn product_matches_vector_and_direct(){
 let mut cases=0;
 for p in [5u64,7,13,509,65537]{for n in [1usize,2,3,12,13,27,65]{for m in [1usize,2,3,12,13,27,65]{
  let roots:Vec<_>=(0..n).map(|i|(i*i+11)as u64%(2*p)).collect();
  let points:Vec<_>=(0..m).map(|i|(i*7+3)as u64%(2*p)).collect();
  let mut c=Poly::new(p).unwrap();let mut r=reference::Poly::new(p).unwrap();
  let got=c.root_resultant(&roots,&points).unwrap();let expected=r.root_resultant(&roots,&points).unwrap();
  let direct=points.iter().fold(1u64,|v,&x|roots.iter().fold(v,|w,&y|((w as u128*((x%p+p-y%p)%p)as u128)%p as u128)as u64));
  assert_eq!(got,expected);assert_eq!(got,direct);
  assert_eq!((c.karatsuba_calls,c.newton_steps),(r.karatsuba_calls,r.newton_steps));
  let tree=c.product_tree(&points).unwrap();
  for a in [vec![],vec![0],vec![2],vec![p+1,3,7]]{
   let values=c.values(&a,&tree).unwrap();let vector=values.iter().fold(1u64,|v,&x|((v as u128*x as u128)%p as u128)as u64);
   assert_eq!(c.value_product(&a,&tree).unwrap(),vector);
  }
  cases+=1;
 }}}
 println!("resultant_cases={cases}; coefficient_cases={}",cases*4);
 let mut p=Poly::new(509).unwrap();assert_eq!(p.root_resultant(&[],&[1]),Err(Error::EmptyTree));assert_eq!(p.root_resultant(&[1],&[]),Err(Error::EmptyTree));
 let foreign=Poly::new(7).unwrap().product_tree(&[1]).unwrap();assert_eq!(p.value_product(&[1],&foreign),Err(Error::Domain));
}

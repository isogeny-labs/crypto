fn section_encoding_checks(){let(mut cases,mut pairs)=(0,0);
 for r in [17u64,19,41,97]{for m in [2,3,4,5,7]{for g in [vec![r-2,1],vec![2,1],vec![r-4,0,1],vec![1,0,1],vec![4,r-4,1],vec![0,0,0,1],vec![3,2,0,1,0,1]]{for scale in [1,2,r-1]{
  let cap=2*m*(g.len()-1);let mut ring=QuadRing::new(r,&g).unwrap();
  assert!(matches!(ring.univariate_sections(m,scale,cap-1),Err(Error::Limit)));
  let got=ring.univariate_sections(m,scale,cap).unwrap();let want=ring.sections(m,&Quad([vec![0],vec![scale]]),cap).unwrap();
  for (a,b) in got.iter().zip(&want){assert_eq!(a.0,b.0,"r={r} m={m} g={g:?} scale={scale}");pairs+=1;}cases+=1;
 }}}}
 let mut ring=QuadRing::new(17,&[15,1]).unwrap();assert_eq!(ring.section_modulus(3,2,6).unwrap(),vec![4,0,0,13,0,0,1]);
 for (m,s) in [(1,1),(3,0),(3,17),(17,1)]{assert!(matches!(ring.univariate_sections(m,s,100),Err(Error::Domain)));}
 println!("univariate_cases={cases} section_pair_equalities={pairs} endpoint_multiplicity=preserved capacity_boundary=checked invalid_inputs=rejected");
}

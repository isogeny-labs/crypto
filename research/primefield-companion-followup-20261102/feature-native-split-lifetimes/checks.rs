fn block_checks(){let(mut prefixes,mut certificates)=(0,0);
 for r in [7,11,101]{for n in 1..=16.min(r as usize-1){let roots:Vec<_>=(0..n as u64).collect();let g=schoolbook_roots(&roots,r);let mut p=polynomial::Poly::new(r).unwrap();
  for m in [2,3,5]{let b=power_mod(&mut p,&g,m).unwrap();let weights=trace_weights(&mut p,&g,r).unwrap();
   for length in 1..=2*n{let blocks=power_blocks(&mut p,&g,&b,length,4096).unwrap();let got=projected_prefix(&mut p,&g,&blocks,&weights,length,r).unwrap();let expected:Vec<_>=(0..length).map(|j|roots.iter().fold(0,|s,&x|(s+pow(x,(m*j)as u64,r))%r)).collect();assert_eq!(got,expected);prefixes+=1;
    for degree in [0,1,n,2*n+1]{let c:Vec<_>=(0..=degree).map(|i|((i*i+3*i+1)as u64)%r).collect();let reference=quotient_evaluate(&mut p,&g,&b,&c,r).unwrap();assert_eq!(blocked_certificate(&mut p,&g,&blocks,&c,r).unwrap(),reference);certificates+=1;}
    let used=blocks.width*n;assert!(matches!(power_blocks(&mut p,&g,&b,length,used-1),Err(Error::Limit)));assert!(power_blocks(&mut p,&g,&b,length,used).is_ok());
   }
  }
 }}
 let mut p=polynomial::Poly::new(7).unwrap();assert!(matches!(adaptive_image_blocked(&mut p,&[6,1],2,7,1,7,2,1),Err(Error::Limit)));
 assert!(matches!(adaptive_image_blocked(&mut p,&[1,5,1],2,7,2,7,4,16),Err(Error::Domain)));
 assert!(matches!(adaptive_image_blocked(&mut p,&[6,1],2,7,1,7,1,16),Err(Error::Limit)));
 println!("blocked_prefix_equalities={prefixes} blocked_certificate_equalities={certificates} table_cap_boundaries={prefixes}");}

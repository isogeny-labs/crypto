fn workspace_checks(){let(mut plans,mut vectors,mut capped_rejections)=(0,0,0);
 for n in 1..=32{for length in 1..=2*n{for rows in 1..=length{let plan=width_plan(n,length,n*rows).unwrap();assert!(plan.width<=rows&&plan.table_words<=n*rows);
  for q in 1..=rows{let objective=q+2*length.div_ceil(q)+(n.min(length)+1).div_ceil(q)-2;assert!(plan.call_bound<=objective as u128);if objective as u128==plan.call_bound{assert!(plan.width<=q)}}
  assert_eq!(plan.call_bound,(plan.quotient_products+plan.convolutions)as u128);plans+=1;
 }}}
 for r in [7,11,101]{for n in 1..=8.min(r as usize-1){let roots:Vec<_>=(0..n as u64).collect();let g=schoolbook_roots(&roots,r);let mut p=polynomial::Poly::new(r).unwrap();let b=power_mod(&mut p,&g,2).unwrap();let weights=trace_weights(&mut p,&g,r).unwrap();
  for length in 1..=2*n{for rows in 1..=length{let cap=n*rows;let plan=width_plan(n,length,cap).unwrap();let blocks=planned_blocks(&mut p,&g,&b,length,cap).unwrap();assert_eq!(blocks.width,plan.width);
   let moments=projected_prefix(&mut p,&g,&blocks,&weights,length,r).unwrap();let expected:Vec<_>=(0..length).map(|j|roots.iter().fold(0,|s,&x|(s+pow(x,2*j as u64,r))%r)).collect();assert_eq!(moments,expected);
   let c:Vec<_>=(0..=n.min(length)).map(|i|(i*i+1)as u64%r).collect();assert_eq!(blocked_certificate(&mut p,&g,&blocks,&c,r).unwrap(),quotient_evaluate(&mut p,&g,&b,&c,r).unwrap());vectors+=1;
  }}
  assert!(matches!(planned_blocks(&mut p,&g,&b,2,n-1),Err(Error::Limit)));capped_rejections+=1;
 }}
 let mut p=polynomial::Poly::new(7).unwrap();let g=schoolbook_roots(&[1,2,3,4,5,6],7);
 assert!(matches!(adaptive_image_blocked(&mut p,&g,6,7,6,7,12,6),Err(Error::Limit)));
 let got=adaptive_image_workspace(&mut p,&g,6,7,6,7,12,6).unwrap();assert_eq!(got.images,vec![6,1]);assert_eq!(got.moments.len(),2);
 assert!(matches!(width_plan(0,2,8),Err(Error::Domain)));assert!(matches!(width_plan(2,0,8),Err(Error::Domain)));
 println!("workspace_plans={plans} workspace_vectors={vectors} below_one_row_rejections={capped_rejections} previous_cap_failure_recovered=1");
}

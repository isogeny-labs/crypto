fn meter_checks(){
 meter::self_test();
 let mut q=IntervalProducts::new(97,12,7).unwrap();
 for len in [0,1,2,7,12]{
  let expected=(0..len).fold(1,|a,j|mul(a,3+j,97));
  let (got,s)=meter::capture(||q.product(3,len)).unwrap();assert_eq!(got.unwrap(),expected);
  assert_eq!(s.counts[0],len);assert_eq!(&s.counts[1..4],&[0,0,0]);assert!(!s.overflow);
 }
 let(_,outside)=meter::capture(||{mul(2,3,97);pow(2,95,97);}).unwrap();assert_eq!(outside.counts,[0;10]);
 let(_,s)=meter::capture(||{let _r=meter::Region::enter();let mut p=polynomial::Poly::new(97).unwrap();assert_eq!(p.mul(&[1,2],&[3,4,5]),vec![3,10,13,10]);}).unwrap();assert_eq!(s.counts[1],6);assert_eq!(s.counts[2],0);
 let(_,s)=meter::capture(||{let _r=meter::Region::enter();let mut p=polynomial::Poly::new(97).unwrap();assert_eq!(p.inv_series(&[2],1).unwrap(),vec![49]);assert_eq!(pow(2,95,97),49);}).unwrap();assert_eq!(s.counts[2],1);assert_eq!(s.counts[3],1);assert_eq!(s.counts[0],13);
 let(err,s)=meter::capture(||q.product(97,1)).unwrap();assert_eq!(err,Err(Error::Domain));assert_eq!(s.counts,[0;10]);
 // Two fresh identical blocked engines: A/A only, no width-policy comparison.
 let mut snapshots=Vec::new();
 for _ in 0..2{
  let mut engine=IntervalProducts::new(257,64,8).unwrap();
  let(vals,s)=meter::capture(||engine.products(&[(3,64),(100,33)])).unwrap();
  let values=vals.unwrap();for (i,(start,len)) in [(3,64),(100,33)].into_iter().enumerate(){assert_eq!(values[i],(0..len).fold(1,|v,j|mul(v,(start+j)%257,257)));}
  assert!(s.counts[0]>0&&s.counts[1]>0&&s.counts[7]>0);assert!(!s.overflow);
  // Retained polynomial/progression and returned Vec cross the boundary safely.
  assert!(engine.polynomial.is_some());snapshots.push(s);
 }
 assert_eq!(snapshots[0],snapshots[1]);
 println!("interval_meter_self_test=pass direct_lengths=5 naive_poly_products=6 eea_calls=1 fermat_calls=1 fermat_products=13 blocked_aa=pass allocator_trait=pass nesting_error_unwind_overflow=pass");
}
fn instrument_main(){if std::env::args().any(|x|x=="--inherited"){inherited_checks()}else{meter_checks()}}

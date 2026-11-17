#[path="../feature-native-step-matrix/polynomial.rs"]mod division_reference;
pub fn remainder_checks(){
 let(mut checked,mut constants,mut zero_remainders,mut fewer,mut equal,mut more,mut schoolbook_checks)=(0,0,0,0,0,0,0);
 for p in [5u64,17,97,257]{let mut new=Poly::new(p).unwrap();let mut old=division_reference::Poly::new(p).unwrap();
  for m in [0usize,1,2,3,7,8,12,13,24,32,63,96]{for d in [0usize,1,2,3,7,12,13,24,32]{for seed in 0..6u64{
   let mut a=test_poly(p,m,seed);let mut b=test_poly(p,d,seed+3);
   if seed==0{b=vec![0;d+1];b[d]=1;} // monomial; zero low part
   if seed==1{a=new.mul(&b,&test_poly(p,m/2,seed));} // exact divisibility
   if seed==2{a.push(0);b.push(0);} // trim boundary
   if seed==3{for v in a.iter_mut().chain(b.iter_mut()){*v+=p;}} // noncanonical
   division_reference::reset_audit();let expected=old.divrem(&a,&b).unwrap();let old_count=division_reference::audit();
   reset_audit();let actual=new.divrem(&a,&b).unwrap();let new_count=audit();assert_eq!(actual,expected,"p={p} m={m} d={d} seed={seed}");assert_eq!(old_count[1],new_count[1]);
   match new_count[0].cmp(&old_count[0]){std::cmp::Ordering::Less=>fewer+=1,std::cmp::Ordering::Equal=>equal+=1,std::cmp::Ordering::Greater=>more+=1}
   let(q,r)=actual;let aa=new.trim(&a);let bb=new.trim(&b);let qb=old.mul(&q,&bb);assert_eq!(old.add(&qb,&r),aa);let degree_b=bb.len()-1;
   assert!(r==[0]||r.len()<bb.len());
   for i in degree_b..aa.len().max(qb.len()){assert_eq!(aa.get(i).copied().unwrap_or(0),qb.get(i).copied().unwrap_or(0));}
   if degree_b==0{constants+=1;assert_eq!(r,vec![0]);}
   if r==[0]{zero_remainders+=1;}
   // Exact residual schoolbook pair count, independent of the quotient stage.
   if degree_b>0 && q!=[0] && q.len().min(bb.len())<=12{
    reset_audit();let low=new.mul_low(&q[..q.len().min(degree_b)],&bb[..degree_b],degree_b);let count=audit()[0];
    let qlow=new.trim(&q[..q.len().min(degree_b)]);let blow=new.trim(&bb[..degree_b]);
    let pairs=if qlow==[0]||blow==[0]{0}else{(0..qlow.len()).map(|i|blow.len().min(degree_b-i)).sum()};
    assert_eq!(count,pairs);assert!(count<=q.len()*bb.len());assert!(low.len()<=degree_b);schoolbook_checks+=1;
   }
   checked+=1;
  }}}
  for a in [vec![],vec![0],vec![1,2],vec![p,2*p]]{assert!(matches!(new.divrem(&a,&[]),Err(Error::ZeroDivisor)));assert!(matches!(new.divrem(&a,&[0,p]),Err(Error::ZeroDivisor)));}
  assert_eq!(new.divrem(&[],&[1,2]).unwrap(),(vec![0],vec![0]));
 }
 assert!(fewer>0&&constants>0&&zero_remainders>0);
 println!("division_cases={checked} constants={constants} zero_remainders={zero_remainders} exact_reconstruction_and_high_cancellation=true schoolbook_pair_controls={schoolbook_checks} lower_product_count={fewer} equal_product_count={equal} higher_product_count={more} zero_divisor_controls=32 empty_dividend_controls=4");
}

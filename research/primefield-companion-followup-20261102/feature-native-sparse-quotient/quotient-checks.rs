#[path="../feature-native-low-remainder/polynomial.rs"]mod quotient_reference;
fn old_quotient(old:&mut quotient_reference::Poly,a:&[u64],b:&[u64])->Vec<u64>{let n=a.len()-b.len()+1;let rev_a:Vec<_>=a.iter().rev().take(n).copied().collect();let rev_b:Vec<_>=b.iter().rev().copied().collect();let inv=old.inv_series(&rev_b,n).unwrap();let mut q=old.mul_low(&rev_a,&inv,n);q.truncate(n);q.resize(n,0);q.reverse();while q.len()>1&&q.last()==Some(&0){q.pop();}q}
fn one_quotient_check(p:u64,a:&[u64],b:&[u64],n:usize){
 let mut new=Poly::new(p).unwrap();let mut old=quotient_reference::Poly::new(p).unwrap();
 quotient_reference::reset_audit();let expected=old_quotient(&mut old,a,b);let previous=quotient_reference::audit();
 reset_audit();let actual=new.quotient_canonical(a,b).unwrap();let now=audit();assert_eq!(actual,expected);assert_eq!(previous[1],now[1]);
 let u=usize::from(*b.last().unwrap()!=1);let h=usize::from(b.len()>1&&b[b.len()-2]!=0);let ell=usize::from(a.len()>1&&a[a.len()-2]!=0);
 if n==1{assert_eq!(previous[0],1);assert_eq!(now[0],u);}else{assert_eq!(previous[0],3+3*h+ell);assert_eq!(now[0],2*u+h);assert!(now[0]<previous[0]);}
 assert_eq!(now[1],u);assert_eq!(new.newton_steps,0);assert_eq!(new.divrem(a,b).unwrap(),old.divrem(a,b).unwrap());
}
pub fn quotient_checks(){
 let(mut constant,mut linear,mut fallback,mut trees)=(0,0,0,0);
 for p in [3u64,5,7]{
  for lead_a in 1..p{for lead_b in 1..p{
   for d in [0usize,1,8,33]{let mut a=vec![1;d+1];let mut b=vec![2%p;d+1];a[d]=lead_a;b[d]=lead_b;one_quotient_check(p,&a,&b,1);constant+=1;}
   for next_a in 0..p{one_quotient_check(p,&[next_a,lead_a],&[lead_b],2);linear+=1;
    for next_b in 0..p{for d in [1usize,8]{let mut a=vec![1;d+2];let mut b=vec![2%p;d+1];a[d+1]=lead_a;a[d]=next_a;b[d]=lead_b;b[d-1]=next_b;one_quotient_check(p,&a,&b,2);linear+=1;}}
   }
  }}
 }
 for p in [5u64,17,97,257]{let mut new=Poly::new(p).unwrap();let mut old=quotient_reference::Poly::new(p).unwrap();
  for n in [3usize,4,8,13,32]{for d in [0usize,1,7,16]{for seed in 0..8u64{let a=test_poly(p,d+n-1,seed);let b=test_poly(p,d,seed+3);
   quotient_reference::reset_audit();let expected=old_quotient(&mut old,&a,&b);let before=quotient_reference::audit();reset_audit();let actual=new.quotient_canonical(&a,&b).unwrap();assert_eq!(actual,expected);if b.iter().rev().skip(1).take(n-1).filter(|&&x|x!=0).count()>1{assert_eq!(audit(),before);}else{assert_eq!(audit()[1],before[1]);}fallback+=1;
  }}}
  for n in 1..=16usize{for gap in [0usize,1,2,7]{let points:Vec<_>=(0..n).map(|i|(i as u64*3+1)%p).collect();let a=test_poly(p,n+gap,5);let nt=new.product_tree(&points).unwrap();let ot=old.product_tree(&points).unwrap();assert_eq!(new.values(&a,&nt).unwrap(),old.values(&a,&ot).unwrap());trees+=1;}}
  for b in [vec![1],vec![2]]{quotient_reference::reset_audit();let expected=old.divrem(&[0],&b).unwrap();let before=quotient_reference::audit();reset_audit();assert_eq!(new.divrem(&[0],&b).unwrap(),expected);assert_eq!(audit(),before);}
 }
 println!("constant_quotient_cases={constant} linear_quotient_cases={linear} exact_cost_formulas=true wider_quotient_cases={fallback} dense_fallback_counts_preserved=true tree_evaluation_cases={trees} zero_dividend_count_controls=8");
}

pub fn weighted_checks(){
 let(mut accepted,mut rejected,mut multi,mut boundary,mut max_support)=(0,0,0,0,0);
 for p in [5u64,17,97,257]{for n in [3usize,4,5,8,13,32,65,129]{for lead in [1u64,2]{for seed in 0..4u64{
  let d=n+2;let m=d+n-1;let a:Vec<_>=(0..=m).map(|i|if i==m{seed%(p-1)+1}else if seed%2==0&&i>d{0}else{(i as u64*13+seed*7)%p}).collect();
  let mut supports=vec![vec![],vec![1],vec![1,2],vec![n-2,n-1]];
  let mut tail=Vec::new();let mut w=0;for k in 1..n{if w+k>n-1{break;}w+=k;tail.push(n-k);}tail.sort();supports.push(tail);
  if n<=8{for mask in 0..(1usize<<(n-1)){supports.push((1..n).filter(|j|mask&(1<<(j-1))!=0).collect());}}
  for js in supports{
   let weight:usize=js.iter().map(|j|n-j).sum();let mut b=vec![0;d+1];b[d]=lead;b[0]=1;b[1]=2;b[2]=3;
   for &j in &js{b[d-j]=(seed+j as u64)%(p-1)+1;}
   let mut old=quotient_reference::Poly::new(p).unwrap();let mut new=Poly::new(p).unwrap();
   quotient_reference::reset_audit();let expected=old_quotient(&mut old,&a,&b);let previous=quotient_reference::audit();
   reset_audit();SPARSE_AUDIT.with(|x|x.set([0;3]));WEIGHT_AUDIT.with(|x|x.set([0;2]));let actual=new.quotient_canonical(&a,&b).unwrap();let counts=audit();let visits=WEIGHT_AUDIT.with(|x|x.get());assert_eq!(actual,expected);
   if weight<=n-1{
    accepted+=1;if js.len()>1{multi+=1;}if weight==n-1{boundary+=1;}max_support=max_support.max(js.len());
    assert_eq!(SPARSE_AUDIT.with(|x|x.get()),[n-1,1,0]);assert_eq!(visits[1],weight);assert!(visits[0]<=weight+n);assert_eq!(new.newton_steps,0);
    let scales=if lead==1{0}else{expected.iter().filter(|&&x|x!=0).count()};let cross:usize=js.iter().map(|&j|expected.iter().rev().take(n-j).filter(|&&x|x!=0).count()).sum();
    assert_eq!(counts[0],scales+cross);assert_eq!(counts[1],usize::from(lead!=1));assert!(counts[0]<=2*n-1);assert!(js.len()*(js.len()+1)/2<=n-1);
   }else{
    rejected+=1;assert_eq!(counts,previous);assert_eq!(visits,[0,0]);let v=SPARSE_AUDIT.with(|x|x.get());assert_eq!(&v[1..],&[0,1]);assert!(v[0]<=n-1);
   }
   assert_eq!(new.divrem(&a,&b).unwrap(),old.divrem(&a,&b).unwrap());
  }
 }}}}
 assert!(multi>0&&boundary>0&&max_support>=15);
 println!("weighted_accepted={accepted} rejected={rejected} multi_term={multi} exact_budget_boundary={boundary} max_support={max_support} exact_products=true linear_visits=true quotient_remainder_identity=true");
}

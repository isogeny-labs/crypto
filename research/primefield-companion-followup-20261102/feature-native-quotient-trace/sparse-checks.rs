pub fn sparse_checks(){
 let(mut cases,mut dense,mut low_controls,mut fewer,mut equal,mut more)=(0,0,0,0,0,0);
 for p in [5u64,17,97,257]{for n in [3usize,4,7,8,13,32,65,129]{for lead in [1u64,2]{for seed in 0..6u64{
  let d=n+2;let m=d+n-1;let a:Vec<_>=(0..=m).map(|i|if i==m{seed%(p-1)+1}else if seed%2==0&&i>d{0}else{(i as u64*13+seed*7)%p}).collect();
  for lag in [None,Some(1usize),Some(n/2),Some(n-1)]{
   let mut b=vec![0;d+1];b[d]=lead;if let Some(j)=lag{b[d-j]=seed%(p-1)+1;}b[0]=(seed+1)%p;b[1]=(seed+2)%p;b[2]=(seed+3)%p;
   let mut old=quotient_reference::Poly::new(p).unwrap();let mut new=Poly::new(p).unwrap();
   quotient_reference::reset_audit();let expected=old_quotient(&mut old,&a,&b);let before=quotient_reference::audit();
   reset_audit();SPARSE_AUDIT.with(|x|x.set([0;3]));let actual=new.quotient_canonical(&a,&b).unwrap();let after=audit();assert_eq!(actual,expected);assert_eq!(SPARSE_AUDIT.with(|x|x.get()),[n-1,1,0]);
   let scales=if lead==1{0}else{expected.iter().filter(|&&x|x!=0).count()};let cross=lag.map_or(0,|j|expected.iter().rev().take(n-j).filter(|&&x|x!=0).count());assert_eq!(after[0],scales+cross);assert_eq!(after[1],usize::from(lead!=1));assert_eq!(new.newton_steps,0);
   assert!(after[0]<=usize::from(lead!=1)*n+lag.map_or(0,|j|n-j));
   match after[0].cmp(&before[0]){std::cmp::Ordering::Less=>fewer+=1,std::cmp::Ordering::Equal=>equal+=1,std::cmp::Ordering::Greater=>more+=1}
   let qr=new.divrem(&a,&b).unwrap();assert_eq!(qr,old.divrem(&a,&b).unwrap());
   let mut changed=b.clone();changed[0]=(changed[0]+1)%p;let other=new.divrem(&a,&changed).unwrap();assert_eq!(other.0,qr.0);assert_ne!(other.1,qr.1);low_controls+=1;cases+=1;
  }
  let mut b=vec![0;d+1];b[d]=lead;b[d-1]=1;b[d-2]=1;
  let mut old=quotient_reference::Poly::new(p).unwrap();let mut new=Poly::new(p).unwrap();quotient_reference::reset_audit();let expected=old_quotient(&mut old,&a,&b);let before=quotient_reference::audit();reset_audit();SPARSE_AUDIT.with(|x|x.set([0;3]));assert_eq!(new.quotient_canonical(&a,&b).unwrap(),expected);assert_eq!(audit(),before);assert_eq!(SPARSE_AUDIT.with(|x|x.get()),[2,0,1]);dense+=1;
 }}}}
 println!("sparse_quotient_cases={cases} exact_nonzero_product_counts=true scan_bounds=true dense_fallback_cases={dense} lower_coefficient_controls={low_controls} lower_product_count={fewer} equal_product_count={equal} higher_product_count={more}");
}

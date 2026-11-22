fn signed_checks(){
 let(mut observations,mut reversals,mut quotients,mut supports,mut zeros)=(0,0,0,0,0);
 for(r,m)in[(7u64,3usize),(13,3),(17,4),(41,5),(97,3),(281,5),(281,35)]{
  let h=(r-1)/2;let l=h/m as u64;let sigma=if h%2==0{1}else{r-1};
  let zeta=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();
  // Independent fixture coefficients: repeated multiplication, no binomial recurrence.
  let mut dense=vec![1];for _ in 0..h{let mut next=vec![0;dense.len()+1];for(j,&c)in dense.iter().enumerate(){next[j]=(next[j]+r-c)%r;next[j+1]=(next[j+1]+c)%r;}dense=next;}
  let plus:Vec<_>=dense.iter().skip(1).step_by(m).copied().collect();
  let minus:Vec<_>=dense.iter().skip(m-1).step_by(m).copied().collect();
  assert_eq!(plus.len(),l as usize);assert_eq!(minus.len(),plus.len());
  assert_eq!(minus,plus.iter().rev().map(|&c|mul(sigma,c,r)).collect::<Vec<_>>());reversals+=1;
  // Independent Laurent product -> W polynomial via D_j=V^j+V^-j.
  let mut p=polynomial::Poly::new(r).unwrap();let mut norm=vec![plus.iter().fold(0,|a,&c|(a+mul(c,c,r))%r)];
  let(mut previous,mut current)=(vec![2],vec![0,1]);
  for gap in 1..plus.len(){let c=(gap..plus.len()).fold(0,|a,j|(a+mul(plus[j],plus[j-gap],r))%r);
   let term:Vec<_>=current.iter().map(|&x|mul(x,c,r)).collect();norm=p.add(&norm,&term);
   let shifted=p.mul(&[0,1],&current);let next=p.sub(&shifted,&previous);previous=current;current=next;
  }
  for cap in [1,2,7]{let stream=StreamModes::new(r,m,zeta,m,1000,cap).unwrap();let modes=RecoveryModes::from_modes(&stream).unwrap();
   for b in [1,2,r-1]{for x in 0..r{
    let mut z=1;let signs:Vec<_>=(0..m).map(|_|{let s=pow(mul(b,(x+r-z)%r,r),h,r);z=mul(z,zeta,r);s}).collect();
    let f=modes.signed_observed(&signs).unwrap();let a=mul(m as u64,pow(b,h,r),r);let v=pow(x,m as u64,r);
    assert_eq!(f,[mul(mul(a,x,r),eval(&plus,v,r),r),mul(mul(a,pow(x,(m-1)as u64,r),r),eval(&minus,v,r),r)]);observations+=1;
    if x==0{assert_eq!(f,[0,0]);continue}
    let w=(v+pow(v,r-2,r))%r;let epsilon=pow(x,h,r);let target=mul(mul(m as u64,m as u64,r),mul(sigma,epsilon,r),r);
    assert_eq!(mul(f[0],f[1],r),mul(target,eval(&norm,w,r),r));if f[0]==0||f[1]==0{zeros+=1;}
    // Both branch hypotheses: equality, not automatic rejection of the wrong branch when norm=0.
    for ep in [1,r-1]{let g=vec![(r-w)%r,1];let eq=modes.signed_norm_equation(&g,f,ep).unwrap();let factor=mul(mul(m as u64,m as u64,r),mul(sigma,ep,r),r);
     assert_eq!(eval(&eq,w,r),(mul(factor,eval(&norm,w,r),r)+r-mul(f[0],f[1],r))%r);quotients+=1;
    }
   }}
   for g in [vec![0,1],vec![1,0,1],vec![4,r-4,1],vec![0,0,0,1]]{for f in [[0,0],[1,1],[2,3],[r-1,1]]{for ep in [1,r-1]{
    let factor=mul(mul(m as u64,m as u64,r),mul(sigma,ep,r),r);let mut raw:Vec<_>=norm.iter().map(|&v|mul(factor,v,r)).collect();raw[0]=(raw[0]+r-mul(f[0],f[1],r))%r;
    let rem=p.divrem(&raw,&g).unwrap().1;let got=modes.signed_norm_equation(&g,f,ep).unwrap();assert_eq!(p.divrem(&got,&g).unwrap().1,rem);quotients+=1;
    let expected=p.gcd_monic(&g,&raw,g.len()+raw.len()+2).unwrap().polynomial;
    assert_eq!(modes.signed_norm_filter(&g,f,ep).unwrap(),expected);supports+=1;
   }}}
   assert!(modes.signed_observed(&[]).is_err());assert!(modes.signed_observed(&vec![2;m]).is_err());
   assert!(modes.signed_norm_equation(&[1,1],[0,0],0).is_err());assert!(modes.signed_norm_equation(&[1,1],[r,0],1).is_err());assert!(modes.signed_norm_equation(&[1],[0,0],1).is_err());
  }
 }
 println!("signed_reversals={reversals} observation_cases={observations} quotient_equalities={quotients} support_equalities={supports} zero_observation_cases={zeros} invalid_domains=rejected");
}

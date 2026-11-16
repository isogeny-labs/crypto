use super::*;
fn chi(a:u64,r:u64)->i8{if a==0{0}else{if pow(a,(r-1)/2,r)==1{1}else{-1}}}
fn eval(v:&[u64],x:u64,r:u64)->u64{v.iter().rev().fold(0,|a,&b|(mul(a,x,r)+b)%r)}
fn order_root(r:u64,m:usize)->u64{(1..r).find(|&z|(1..m).all(|i|pow(z,i as u64,r)!=1)&&pow(z,m as u64,r)==1).unwrap()}
fn roots(v:&[u64],r:u64)->Vec<u64>{(0..r).filter(|&x|eval(v,x,r)==0).collect()}
#[test]fn fold_identity_and_complete_states(){
 let(mut identities,mut states,mut fibers,mut ambiguous)=(0,0,0,0);
 for r in [7,13,17,31,41,97]{for m in 2..=8{if (r-1)%m as u64!=0{continue}let f=Fold::new(r,m,order_root(r,m),8,r as usize).unwrap();
  for k in 0..r{
   let mut state=f.initial().unwrap();let mut expected=roots(state.polynomial(),r);assert_eq!(expected.len(),f.d+1);
   for b in 1..=6.min(r-1){
    let prod=f.offsets(1,b).unwrap().iter().fold(1,|s,&a|s*chi((k+a)%r,r));let z=pow((k+1)%r,m as u64,r);assert_eq!(prod,chi((z+r-pow(b,m as u64,r))%r,r));identities+=1;
    expected.retain(|&v|chi((v+r-pow(b,m as u64,r))%r,r)==prod);f.observe(&mut state,b,prod).unwrap();assert_eq!(roots(state.polynomial(),r),expected);assert_eq!(state.moments().count as usize,expected.len());states+=1;
   }
   let images=[pow(k,m as u64,r),pow((k+1)%r,m as u64,r)];let g=f.intersect([0,1],images).unwrap();let expected:Vec<_>=(0..r).filter(|&t|pow(t,m as u64,r)==images[0]&&pow((t+1)%r,m as u64,r)==images[1]).collect();assert_eq!(roots(&g,r),expected);assert!(expected.contains(&k));if m==2{assert_eq!(g,vec![(r-k)%r,1])}if expected.len()>1{ambiguous+=1}fibers+=1;
  }
 }}assert!(ambiguous>0);println!("orbit_identities={identities} exact_states={states} fibers={fibers} ambiguous_fibers={ambiguous}");
}
#[test]fn public_point_recovery_and_limits(){
 let mut recovered=0;let mut unresolved=0;let mut endpoint=0;let mut gcd=0;let mut calls=0;
 for (r,p,a,b,base,m,zeta,rounds) in [(13,7,0,3,[1,2],3,3,6),(479,509,2,10,[3,68],2,478,20)]{
  let runner=PublicRecovery::new(p,a,b,r,base,m,zeta,8,300,64,[0,1],rounds,1000).unwrap();
  let targets:Vec<_>=if r==13{(0..r).map(|k|{let mut c=Counts::default();(k,PublicRecovery::pair(runner.multiple(k,&mut c)))}).collect()}else{include_str!("../feature-native-monomial-recovery/recovery.tsv").lines().skip(1).map(|s|{let v:Vec<_>=s.split('\t').collect();let k=v[0].parse::<u64>().unwrap();(k,if k==0{None}else{Some([v[1].parse().unwrap(),v[2].parse().unwrap()])})}).collect()};
  for(k,t)in targets{match runner.recover(t).unwrap(){Outcome::Recovered{value,route,counts}=>{assert_eq!(value,k);recovered+=1;calls+=counts.source_calls;if route=="fiber-gcd"{gcd+=1}else{endpoint+=1}},Outcome::Unresolved{counts,..}=>{unresolved+=1;calls+=counts.source_calls}}}
 }
 assert!(gcd>0);assert_eq!(recovered+unresolved,33);println!("public_cases=33 recovered={recovered} unresolved={unresolved} fiber_gcd={gcd} endpoint_or_identity={endpoint} source_calls={calls}");
 let f=Fold::new(13,3,3,3,5).unwrap();assert!(matches!(Fold::new(13,3,1,3,5),Err(Error::Domain)));assert!(matches!(Fold::new(13,3,3,2,5),Err(Error::Limit)));assert!(matches!(Fold::new(13,3,3,3,4),Err(Error::Limit)));assert!(f.offsets(0,0).is_err());assert!(f.intersect([1,1],[1,1]).is_err());
 let mut state=f.initial().unwrap();assert!(f.observe(&mut state,1,2).is_err());f.observe(&mut state,1,1).unwrap();f.observe(&mut state,1,-1).unwrap();assert_eq!(state.moments().count,0);
 let runner=PublicRecovery::new(509,2,10,479,[3,68],2,478,2,240,64,[0,1],1,1).unwrap();assert!(matches!(runner.recover(Some([0,0])),Err(Error::Source)));assert!(matches!(runner.recover(Some([509,68])),Err(Error::Source)));assert!(matches!(runner.recover(Some([3,68])),Ok(Outcome::Unresolved{..})));
}

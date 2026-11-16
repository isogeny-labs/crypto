// R_a(V)=V^(a.len()-1) a(V+V^-1), retaining the declared degree bound
// even if a's high coefficients are zero. Prime r is caller-certified.
struct HomogeneousLift{p:polynomial::Poly,powers:std::collections::BTreeMap<usize,Vec<u64>>}
impl HomogeneousLift{
 fn power(&mut self,k:usize)->Vec<u64>{
  if let Some(v)=self.powers.get(&k){return v.clone()}
  let v=if k==0{vec![1]}else if k==1{vec![1,0,1]}else{let half=self.power(k/2);let square=self.p.mul(&half,&half);if k%2==0{square}else{self.p.mul(&square,&[1,0,1])}};
  self.powers.insert(k,v.clone());v
 }
 fn transform(&mut self,a:&[u64])->Vec<u64>{
  if a.len()==1{return vec![a[0]]}
  let k=a.len()/2;let low=self.transform(&a[..k]);let high=self.transform(&a[k..]);let power=self.power(k);
  let right=self.p.mul(&power,&high);let mut left=vec![0;a.len()-k];left.extend_from_slice(&low);self.p.add(&left,&right)
 }
}
fn homogeneous_lift(r:u64,a:&[u64])->Result<Vec<u64>,Error>{
 if r<7||r>u64::MAX/2||a.is_empty()||a.iter().any(|&v|v>=r){return Err(Error::Domain)}
 let _degree=(a.len()-1).checked_mul(2).ok_or(Error::Limit)?;
 let mut h=HomogeneousLift{p:polynomial::Poly::new(r).map_err(|_|Error::Domain)?,powers:std::collections::BTreeMap::new()};Ok(h.transform(a))
}
fn homogeneous_checks(){let(mut coefficient_cases,mut point_cases)=(0,0);
 for r in [7u64,19,281,998244353]{for n in (1usize..=65).chain([127,128,129,255,256,257,1025]){for kind in 0..6{
  let mut a:Vec<_>=(0..n).map(|j|match kind{0=>0,1=>u64::from(j==0),2=>u64::from(j+1==n),3=>((j*j+7*j+3)as u64)%r,4=>if j%2==0{r-1}else{0},_=>((17*j+5)as u64)%r}).collect();if kind==5{a[n-1]=0;}
  let got=homogeneous_lift(r,&a).unwrap();let mut p=polynomial::Poly::new(r).unwrap();let d=n-1;let mut want=vec![a[d]];
  for j in 1..=d{want=p.mul(&want,&[1,0,1]);if want.len()<=j{want.resize(j+1,0)}want[j]=(want[j]+a[d-j])%r;}
  while want.len()>1&&want.last()==Some(&0){want.pop();}assert_eq!(got,want,"coeff r={r} n={n} kind={kind}");coefficient_cases+=1;
  for v in [1,2,3,r-1]{let w=(v+pow(v,r-2,r))%r;assert_eq!(eval(&got,v,r),mul(pow(v,d as u64,r),eval(&a,w,r),r));point_cases+=1;}
 }}}
 assert!(matches!(homogeneous_lift(19,&[]),Err(Error::Domain)));assert!(matches!(homogeneous_lift(19,&[19]),Err(Error::Domain)));assert!(matches!(homogeneous_lift(5,&[1]),Err(Error::Domain)));
 assert_eq!(homogeneous_lift(19,&[0,0,0]).unwrap(),vec![0]);
 println!("homogeneous_coefficient_equalities={coefficient_cases} homogeneous_evaluation_equalities={point_cases} max_input_coefficients=1025 zero_high_coefficients=covered invalid_inputs=rejected");
}

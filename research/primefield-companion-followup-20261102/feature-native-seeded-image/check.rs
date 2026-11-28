#[path="../feature-native-dyadic-source/lib.rs"]mod full;
mod polynomial;
include!("class-checks.rs");
#[path="lib.rs"]mod source;
#[path="../feature-native-standalone-source/lib.rs"]mod prior;
#[path="../feature-native-value-division/lib.rs"]mod division;
#[path="../feature-orbit-local-terms/local.rs"]mod ec;
fn mul(a:u64,b:u64,p:u64)->u64{(a as u128*b as u128%p as u128)as u64}
fn pow(mut a:u64,mut e:u64,p:u64)->u64{let mut v=1;while e>0{if e&1!=0{v=mul(v,a,p)}a=mul(a,a,p);e>>=1}v}
fn prime(n:u64)->bool{n>=2&&(2..).take_while(|d|d*d<=n).all(|d|n%d!=0)}
fn fixture(p:u64)->(u64,u64,u64,[u64;2]){for a in 0..=8{for b in 0..=8{if (4*a*a*a+27*b*b)%p==0{continue}let mut points=vec![];for x in 0..p{for y in 0..p{if y*y%p==(x*x%p*x+a*x+b)%p{points.push([x,y])}}}let r=points.len()as u64+1;if r>=7&&r!=p&&prime(r)&&(p-1)%r!=0{return(a,b,r,points[0])}}}panic!("fixture cap exhausted {p}")}
fn beta(p:u64,a:u64,b:u64,r:u64,t:[u64;2])->u64{let mut d=division::ValueDivision::new(p,a,b,t[0],t[1]).unwrap();mul(mul(mul(d.value(r+1),d.value(r+1),p),d.value(2),p),pow(d.value(r+2),p-2,p),p)}
include!("schedule-checks.rs");
include!("singleton-checks.rs");
fn main(){polynomial::weighted_checks();polynomial::sparse_checks();polynomial::quotient_checks();polynomial::remainder_checks();polynomial::matrix_checks();singleton_checks();schedule_checks();class_checks();let(mut signs,mut raws,mut covariance,mut windows,mut wrong_exponent)=(0,0,0,0,0);
 let mut fixtures=vec![(5,2,1,7,[0,1]),(7,0,3,13,[1,2]),(509,2,10,479,[3,68])];for p in [17,41,97,193,257]{let(a,b,r,base)=fixture(p);fixtures.push((p,a,b,r,base));}
 for(p,a,b,r,base)in fixtures{let q=1u64<<(p-1).trailing_zeros();let e=(1..q).find(|&e|mul(mul(r%q,r%q,q),e,q)==1).unwrap();
  println!("fixture p={p} a={a} b={b} r={r} P={base:?} q={q} inverse_r_squared={e}");
  let s=source::Source::new(p,a,b,r,base,64).unwrap();let selected=source::Source::new_with_costs(p,a,b,r,base,64,source::division::CostModel{multiplication:1,inversion:0},4096).unwrap();let reference=full::Source::new(p,a,b,r,base,64).unwrap();let c=ec::Curve::new(p,a,b);let bp=ec::Point{x:base[0],y:base[1]};let mut point=Some(bp);let beta0=beta(p,a,b,r,base);let c0=pow(pow(beta0,p-2,p),e*(p-1)/q,p);let mut d0=division::ValueDivision::new(p,a,b,base[0],base[1]).unwrap();assert_eq!(s.sign(None),Ok(0));
  for k in 1..r{let t=point.unwrap();let xy=[t.x,t.y];let expected=if pow(k,(r-1)/2,r)==1{1}else{-1};assert_eq!(s.sign(Some(xy)),Ok(expected),"p={p} k={k}");signs+=1;assert_eq!(selected.sign(Some(xy)),Ok(expected));assert_eq!(s.raw(xy).unwrap().value,reference.raw(xy).unwrap().value);
   let bt=beta(p,a,b,r,xy);let ct=pow(pow(bt,p-2,p),e*(p-1)/q,p);assert_eq!(ct,mul(pow(d0.value(k),(p-1)/q,p),pow(c0,k*k,p),p));covariance+=1;
   let mut direct=division::ValueDivision::new(p,a,b,t.x,t.y).unwrap();let seed=pow(pow(bt,p-2,p),e,p);let mut product=1;for j in 1..=(r-1)/2{product=mul(product,mul(pow(seed,j*j,p),direct.value(j),p),p)}assert_eq!(s.raw(xy).unwrap().value,pow(product,(p-1)/q,p));raws+=1;
   if q<=4{assert_eq!(prior::Source::new(p,a,b,r,base,64).unwrap().sign(Some(xy)),Ok(expected))}
   if e!=1{let wrong=pow(pow(bt,p-2,p),(p-1)/q,p);if wrong!=ct{wrong_exponent+=1}}
   point=c.plus(point,Some(bp));
  }assert_eq!(point,None);
  assert_eq!(s.preparation_builds(),0);assert_eq!(s.window(None,0),Ok(vec![]));assert_eq!(s.preparation_builds(),0);
  for(start,k0)in[(None,0),(Some(base),1),(Some([base[0],p-base[1]]),r-1)]{let got=s.window(start,3).unwrap();let want:Vec<_>=(0..3).map(|j|{let k=(k0+j)%r;if k==0{0}else if pow(k,(r-1)/2,r)==1{1}else{-1}}).collect();assert_eq!(got,want);windows+=1;}
  assert_eq!(s.preparation_builds(),1);assert_eq!(selected.preparation_builds(),0);std::thread::scope(|scope|{let hs:Vec<_>=(0..4).map(|_|scope.spawn(||selected.window(Some(base),3).unwrap())).collect();for h in hs{assert_eq!(h.join().unwrap(),s.window(Some(base),3).unwrap());}});assert_eq!(selected.preparation_builds(),1);std::thread::scope(|scope|{let handles:Vec<_>=(0..4).map(|_|scope.spawn(||s.window(Some(base),3).unwrap())).collect();for h in handles{assert_eq!(h.join().unwrap(),s.window(Some(base),3).unwrap());}});assert_eq!(s.preparation_builds(),1);
  assert!(source::Source::new(p,a,b,r,base,0).is_err());assert!(s.sign(Some([p,base[1]])).is_err());
 }println!("prepared_sources=8 lazy_zero_window_controls=8 repeated_and_concurrent_reuse=true");assert!(wrong_exponent>0);println!("dyadic_signs={signs} direct_halfproducts={raws} covariance={covariance} identity_crossing_windows={windows} wrong_exponent_detected={wrong_exponent}");
}

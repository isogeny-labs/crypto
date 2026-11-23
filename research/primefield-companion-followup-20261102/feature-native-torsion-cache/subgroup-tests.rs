#[path="../feature-native-division/lib.rs"]mod phase_reference;
#[test]fn subgroup_acceptance_matches_phase_and_affine(){
 let(mut cases,mut accepted,mut rejected)=(0,0,0);
 for p in [5u64,7,13,17,19,31]{for r in [3u64,5,7,11,13,17,19]{
  for x in 0..p{for y in 1..p{
   if mul(y,y,p)!=(mul(mul(x,x,p),x,p)+2*x+1)%p{continue;}
   let q=[x,y];let got=subgroup::validate(p,2,1,r,q).is_ok();
   let old=phase_reference::Division::new(p,2,1,x,y).unwrap().phase(r).is_ok();assert_eq!(got,old);
   let(mut a,mut b)=(r,p-1);while b!=0{let z=a%b;a=b;b=z;}
   let expected=r!=p&&a==1&&point(p,2,q,r).is_none();assert_eq!(got,expected);
   cases+=1;if got{accepted+=1;}else{rejected+=1;}
  }}
 }}assert!(accepted>0&&rejected>0);
 for(p,a,b,r,q)in[(3,0,1,7,[0,1]),(7,0,0,13,[0,1]),(7,0,3,13,[0,0]),(7,0,3,13,[7,2]),(7,0,3,2,[1,2]),(7,0,3,7,[1,2]),(7,0,3,3,[1,2])]{assert!(subgroup::validate(p,a,b,r,q).is_err());}
 println!("finite_point_order_cases={cases} accepted={accepted} rejected={rejected} malformed_cases=7");
}

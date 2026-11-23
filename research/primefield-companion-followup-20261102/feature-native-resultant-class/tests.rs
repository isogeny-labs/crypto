use super::*;
fn add(p:u64,a:u64,x:Option<[u64;2]>,y:Option<[u64;2]>)->Option<[u64;2]>{let(Some([x1,y1]),Some([x2,y2]))=(x,y)else{return x.or(y)};if x1==x2&&(y1+y2)%p==0{return None;}let(n,d)=if x1==x2{((3*mul(x1,x1,p)+a)%p,2*y1%p)}else{((y2+p-y1)%p,(x2+p-x1)%p)};let m=mul(n,pow(d,p-2,p),p);let xx=(mul(m,m,p)+2*p-x1-x2)%p;Some([xx,(mul(m,(x1+p-xx)%p,p)+p-y1)%p])}
fn point(p:u64,a:u64,base:[u64;2],mut k:u64)->Option<[u64;2]>{let(mut out,mut cur)=(None,Some(base));while k>0{if k&1!=0{out=add(p,a,out,cur);}cur=add(p,a,cur,cur);k>>=1;}out}

#[test]fn projected_split_windows(){
 let(mut splits,mut advances,mut skips)=(0,0,0);
 for(p,a,b,r,base)in[(5,2,1,7,[0,1]),(7,0,3,13,[1,2]),(509,2,10,479,[3,68])]{
  let s=Source::new(p,a,b,r,base,128,600).unwrap();let ready=s.prepare().unwrap();
  let full=full::Source::new(p,a,b,r,base,128,600).unwrap();let f=full.prepare().unwrap();
  let(whole,_)=ready.begin(None,r as usize,0).unwrap();assert_eq!(whole.signs,f.window(None,r as usize).unwrap().signs);
  let mut starts=vec![0,1,2,3,r-3,r-2,r-1];if r==479{starts.extend([25,173,292,440]);}starts.sort_unstable();starts.dedup();
  for start in starts{let q=point(p,a,base,start);let expected=f.window(q,17).unwrap();
   for length in [1,2,3,7]{let(initial,c)=ready.begin(q,length,17-length).unwrap();assert_eq!(initial.signs,expected.signs[..length]);let mut cursor=Some(c);
    for i in length..17{let skip=i%3==0;let c=cursor.take().unwrap();let(e,next)=if skip{c.skip_sign()}else{c.advance()}.unwrap();assert_eq!(e.point,expected.points[i]);
     if skip{assert_eq!(e.sign,None);assert_eq!(e.ratio,None);skips+=1;}else{assert_eq!(e.sign,Some(expected.signs[i]));}
     cursor=Some(next);advances+=1;
    }assert!(cursor.unwrap().advance().is_err());splits+=1;
   }
  }
 }assert_eq!((splits,advances,skips),(100,1375,450));println!("split_windows={splits} advances={advances} skipped_signs={skips} complete_subgroup_rows=499");
}
#[test]fn limits_and_rejection(){
 let s=Source::new(509,2,10,479,[3,68],128,16).unwrap();let r=s.prepare().unwrap();
 assert!(r.begin(None,0,1).is_err());assert!(r.begin(None,14,3).is_err());assert!(r.begin(None,1,usize::MAX).is_err());assert!(r.begin(Some([0,0]),2,1).is_err());
 assert!(!ratio_matches(7,3,Some([1,2]),[2,1],[1,1]));
 assert!(source::Source::new(17,2,1,7,[0,1],128,16).is_err());
}
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

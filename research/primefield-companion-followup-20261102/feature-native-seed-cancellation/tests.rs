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

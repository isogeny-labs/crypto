use super::*;
#[path="../feature-native-multiseed-factor/lib.rs"]mod reference;
fn same(p:u64,a:Pair,b:Pair)->bool{mul(a[0],b[1],p)==mul(b[0],a[1],p)}
#[test]fn combined_complete_subgroups(){
 let mut checked=0;
 for(p,a,b,r,base)in[(5,2,1,7,[0,1]),(7,0,3,13,[1,2]),(509,2,10,479,[3,68])]{
  let s=Source::new(p,a,b,r,base,100,600).unwrap();let old=reference::Source::new(p,a,b,r,base,100,600).unwrap();
  let got=s.prepare().unwrap().window(None,r as usize).unwrap();let want=old.prepare().unwrap().window(None,r as usize).unwrap();
  assert_eq!(got.points,want.points);assert_eq!(got.signs,want.signs);assert_eq!(got.direct_initializations,0);
  for(a,b)in got.ratios.iter().zip(&want.ratios){assert!(same(p,*a,*b));checked+=1;}
 }
 println!("complete_ratios={checked}");
}
#[test]fn combined_shifted_windows(){
 let s=Source::new(509,2,10,479,[3,68],100,600).unwrap();let old=reference::Source::new(509,2,10,479,[3,68],100,600).unwrap();
 let points=old.prepare().unwrap().window(None,479).unwrap().points;let mut checked=0;
 for k in [0,1,2,3,25,173,239,345,465,477,478]{
  let got=s.prepare().unwrap().window(points[k],16).unwrap();let want=old.prepare().unwrap().window(points[k],16).unwrap();
  assert_eq!(got.signs,want.signs);assert_eq!(got.points,want.points);assert_eq!(got.direct_initializations,want.direct_initializations);
  for(a,b)in got.ratios.iter().zip(&want.ratios){assert!(same(509,*a,*b));checked+=1;}
 }
 println!("shifted_ratios={checked}");
}
#[test]fn combined_nonodd_monomial_anchor(){
 // r=13 has even alpha: the -P monomial equals the P monomial.
 let s=Source::new(7,0,3,13,[1,2],100,32).unwrap();let t=s.transport.prepare().unwrap().window(None,13).unwrap();
 assert!(same(7,t.pairs[1][0],t.pairs[12][0]));
 assert_eq!(t.pairs.iter().map(Vec::len).collect::<Vec<_>>(),vec![1;13]);
}

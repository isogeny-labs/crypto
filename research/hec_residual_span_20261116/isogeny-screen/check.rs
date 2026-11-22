use std::collections::BTreeSet;
fn norm(x:u8)->u8 {if x&64!=0{x^127}else{x}}
fn shift(x:u8)->u8 {norm(((x<<1)&127)|(x>>6))}
fn action(x:u8, powers:&[usize])->u8 {let mut y=0;let mut v=x;for i in 0..7 {if powers.contains(&i){y^=v;}v=shift(v);}y}
fn pairing(x:u8,y:u8)->u32 {((x&y).count_ones()+x.count_ones()*y.count_ones())&1}
fn pow(mut a:u64,mut n:u64,p:u64)->u64 {let mut r=1;while n>0 {if n&1!=0{r=r*a%p;}a=a*a%p;n>>=1;}r}
fn main(){
 let mut subs=BTreeSet::new();
 for a in 1u8..64 {for b in a+1..64 {for c in b+1..64 {
 let mut v=vec![0,a,b,c,a^b,a^c,b^c,a^b^c];v.sort_unstable();v.dedup();if v.len()==8{subs.insert(v);}
 }}}
 assert_eq!(subs.len(),1395);
 let isotropic:Vec<_>=subs.iter().filter(|v|v.iter().all(|&x|v.iter().all(|&y|pairing(x,y)==0))).collect();
 assert_eq!(isotropic.len(),135);
 let stable:Vec<_>=subs.iter().filter(|v|v.iter().all(|&x|v.contains(&shift(x)))).collect();assert_eq!(stable.len(),2);
 let ka:Vec<_>=(0u8..64).filter(|&x|action(x,&[1,2,4])==0).collect();
 let kb:Vec<_>=(0u8..64).filter(|&x|action(x,&[3,5,6])==0).collect();
 assert_ne!(ka,kb);assert_eq!(ka.len(),8);assert_eq!(kb.len(),8);
 for v in &stable {assert!(**v==ka||**v==kb);assert!(isotropic.contains(v));}
 let mut product=[0u32;7];for a in [1,2,4]{for b in [3,5,6]{product[(a+b)%7]+=1;}}
 assert_eq!(product,[3,1,1,1,1,1,1]);
 // Independent carryless multiplication of the two cubic factors.
 let mut phi=0u32;for i in 0..4 {if 0b1011&(1<<i)!=0 {phi^=0b1101<<i;}}assert_eq!(phi,127);
 println!("subspaces\tisotropic\tstable\tstable_isotropic\tkernel_a\tkernel_b");
 println!("{}\t{}\t{}\t2\t{:?}\t{:?}",subs.len(),isotropic.len(),stable.len(),ka,kb);
 println!("p\tc\tnonseventh_power\troot_count\tirreducible");
 for (p,c) in [(113u64,3u64),(421,2)] {assert_eq!(p%7,1);let t=pow(p-c,(p-1)/7,p);let roots=(0..p).filter(|&x|pow(x,7,p)==p-c).count();assert_ne!(t,1);assert_eq!(roots,0);println!("{p}\t{c}\t{t}\t{roots}\ttrue");}
}

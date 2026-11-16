//! Prospective inputs and separate direct-product oracle; no backend calls here.
#[derive(Clone,Debug,PartialEq,Eq)]pub struct Case{pub id:usize,pub p:u64,pub kind:&'static str,pub roots:Vec<u64>,pub points:Vec<u64>}
pub const SEED:u64=20261107;
struct Rng(u64);
impl Rng{
 fn next(&mut self)->u64{let mut x=self.0;x^=x>>12;x^=x<<25;x^=x>>27;self.0=x;x.wrapping_mul(2685821657736338717)}
 fn below(&mut self,p:u64)->u64{let limit=u64::MAX-u64::MAX%p;loop{let x=self.next();if x<limit{return x%p;}}}
}
pub fn prime(p:u64)->bool{if p<2{return false;}let mut d=2;while d<=p/d{if p%d==0{return false;}d+=1;}true}
pub fn cases()->Vec<Case>{
 let mut rng=Rng(SEED);let mut cases=Vec::new();
 for p in [509,1000003]{assert!(prime(p));for n in [16,32,64,128]{for m in [n/2,n,2*n]{
  let mut sample=Vec::new();while sample.len()<n+m{let x=rng.below(p);if !sample.contains(&x){sample.push(x);}}
  let roots=sample[..n].to_vec();let points=sample[n..].to_vec();
  cases.push(Case{id:cases.len(),p,kind:"disjoint",roots:roots.clone(),points:points.clone()});
  let mut points=points;points[0]=roots[0];cases.push(Case{id:cases.len(),p,kind:"common_root",roots,points});
 }}}
 cases
}
fn mul(a:u64,b:u64,p:u64)->u64{((a as u128*b as u128)%p as u128)as u64}
/// Independent Cartesian product, with no polynomial engine, meter or backend projection.
pub fn oracle(c:&Case)->u64{
 let mut value=1;for &x in &c.points{for &r in &c.roots{value=mul(value,(x+c.p-r)%c.p,c.p);}}
 let mut exponent=(c.p-1)>>((c.p-1).trailing_zeros());let mut answer=1;
 while exponent>0{if exponent%2==1{answer=mul(answer,value,c.p);}exponent/=2;if exponent>0{value=mul(value,value,c.p);}}
 answer
}
fn csv(v:&[u64])->String{v.iter().map(u64::to_string).collect::<Vec<_>>().join(",")}
pub fn input_text(cases:&[Case])->String{
 let mut text=String::from("case_id\tp\tkind\tn\tm\troots\tpoints\texpected\n");
 for c in cases{text.push_str(&format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",c.id,c.p,c.kind,c.roots.len(),c.points.len(),csv(&c.roots),csv(&c.points),oracle(c)));}text
}
pub fn self_test(){
 assert!(prime(13));assert!(!prime(1));assert!(!prime(49));
 let mut c=Case{id:0,p:13,kind:"disjoint",roots:vec![1,2],points:vec![3,4]};assert_eq!(oracle(&c),12);
 c.points[0]=1;assert_eq!(oracle(&c),0);
 c.roots=vec![1];c.points=vec![0];assert_eq!(oracle(&c),12);
 let a=cases();assert_eq!(a,cases());assert_eq!(a.len(),48);
 for c in a{
  assert!(c.roots.iter().chain(&c.points).all(|&x|x<c.p));
  for v in [&c.roots,&c.points]{let mut sorted=v.clone();sorted.sort_unstable();sorted.dedup();assert_eq!(sorted.len(),v.len());}
  let common=c.points.iter().filter(|x|c.roots.contains(x)).count();assert_eq!(common,if c.kind=="common_root"{1}else{0});
 }
}

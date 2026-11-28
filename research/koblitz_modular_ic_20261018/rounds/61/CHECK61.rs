//! Complete source-bound binary certificate checker; no producer reruns.
use std::{collections::BTreeMap,env,fs,io::Write};
#[derive(Clone,PartialEq,Eq,Debug)]struct V{n:usize,w:Vec<u64>}
impl V{
 fn z(n:usize)->Self{Self{n,w:vec![0;(n+63)/64]}}
 fn e(n:usize,i:usize)->Self{let mut v=Self::z(n);v.set(i);v}
 fn get(&self,i:usize)->bool{assert!(i<self.n);self.w[i/64]>>(i%64)&1!=0}
 fn set(&mut self,i:usize){assert!(i<self.n);self.w[i/64]|=1<<(i%64);}
 fn xor(&mut self,b:&Self){assert!(self.n==b.n);for(x,y)in self.w.iter_mut().zip(&b.w){*x^=*y;}}
 fn zero(&self)->bool{self.w.iter().all(|x|*x==0)}
 fn top(&self)->Option<usize>{for(i,x)in self.w.iter().enumerate().rev(){if *x!=0{return Some(64*i+63-x.leading_zeros()as usize);}}None}
 fn ones(&self)->Vec<usize>{let mut out=vec![];for(i,&word)in self.w.iter().enumerate(){let mut x=word;while x!=0{let j=x.trailing_zeros()as usize;out.push(64*i+j);x&=x-1;}}out}
 fn slice(&self,start:usize,n:usize)->Self{let mut x=Self::z(n);for i in 0..n{if self.get(start+i){x.set(i);}}x}
 fn hex(&self)->String{self.w.iter().map(|x|format!("{x:016x}")).collect::<Vec<_>>().join(",")}
 fn fromhex(n:usize,s:&str)->Self{let w=s.split(',').filter(|x|!x.is_empty()).map(|x|u64::from_str_radix(x,16).unwrap()).collect::<Vec<_>>();assert!(w.len()==(n+63)/64);if n%64!=0{assert!(w.last().unwrap()>>(n%64)==0);}Self{n,w}}
}
#[derive(Clone,PartialEq,Eq,Debug)]struct M{r:usize,c:usize,v:Vec<V>}
impl M{
 fn z(r:usize,c:usize)->Self{Self{r,c,v:vec![V::z(r);c]}}
 fn cols(r:usize,v:Vec<V>)->Self{assert!(v.iter().all(|x|x.n==r));Self{r,c:v.len(),v}}
 fn id(n:usize)->Self{Self::cols(n,(0..n).map(|i|V::e(n,i)).collect())}
 fn mv(&self,x:&V)->V{assert!(x.n==self.c);let mut y=V::z(self.r);for j in x.ones(){y.xor(&self.v[j]);}y}
 fn mul(&self,b:&Self)->Self{assert!(self.c==b.r);Self::cols(self.r,b.v.iter().map(|x|self.mv(x)).collect())}
 fn add(&self,b:&Self)->Self{assert!(self.r==b.r&&self.c==b.c);let mut x=self.clone();for(i,y)in b.v.iter().enumerate(){x.v[i].xor(y);}x}
 fn zero(&self)->bool{self.v.iter().all(V::zero)}
 fn rows(&self)->Vec<V>{let mut rows=vec![V::z(self.c);self.r];for(j,x)in self.v.iter().enumerate(){for i in x.ones(){rows[i].set(j);}}rows}
 fn rowpart(&self,start:usize,n:usize)->Self{Self::cols(n,self.v.iter().map(|x|x.slice(start,n)).collect())}
 fn inverse(&self)->Self{assert!(self.r==self.c);let n=self.r;let left=self.rows();let mut rows=vec![V::z(2*n);n];for i in 0..n{for j in left[i].ones(){rows[i].set(j);}rows[i].set(n+i);}
  for k in 0..n{let p=(k..n).find(|&i|rows[i].get(k)).expect("singular inverse");rows.swap(k,p);let pivot=rows[k].clone();for i in 0..n{if i!=k&&rows[i].get(k){rows[i].xor(&pivot);}}}
  let mut out=Self::z(n,n);for i in 0..n{assert!((0..n).all(|j|rows[i].get(j)==(i==j)));for j in 0..n{if rows[i].get(n+j){out.v[j].set(i);}}}out
 }
}
// Independent product checker: row-dot-column parity, not producer column XOR.
fn dotmul(a:&M,b:&M)->M{assert!(a.c==b.r);let rows=a.rows();let mut out=M::z(a.r,b.c);for(j,col)in b.v.iter().enumerate(){for(i,row)in rows.iter().enumerate(){let p=row.w.iter().zip(&col.w).fold(0u32,|s,(x,y)|s^((x&y).count_ones()&1));if p!=0{out.v[j].set(i);}}}out}
fn product(a:&M,b:&M,c:&M){assert!(dotmul(a,b)==*c,"independent product mismatch");}
fn read_singular(path:&str)->BTreeMap<String,M>{
 let text=fs::read_to_string(path).expect("matrix input");let mut out:BTreeMap<String,M>=BTreeMap::new();let mut header=false;
 for line in text.lines(){let s=line.trim();if s.is_empty(){continue;}
  if s.starts_with("ring "){assert!(!header && s.ends_with("=2,(w),dp;short=0;"));header=true;continue;}
  assert!(header,"missing binary ring declaration");
  if let Some(t)=s.strip_prefix("matrix "){
   let t=t.strip_suffix(';').unwrap();let b=t.find('[').unwrap();let name=&t[..b];let dims=t[b..].replace('['," ").replace(']'," ");let nums=dims.split_whitespace().map(|x|x.parse::<usize>().unwrap()).collect::<Vec<_>>();assert_eq!(nums.len(),2);assert!(nums[0]>0&&nums[1]>0);assert!(out.insert(name.into(),M::z(nums[0],nums[1])).is_none(),"duplicate matrix");
  }else{
   let b=s.find('[').expect("unknown nonassignment line");let name=&s[..b];let rest=&s[b+1..];let e=rest.find(']').unwrap();assert_eq!(&rest[e+1..],"=1;","only literal binary entries allowed");let ij=rest[..e].split(',').map(|x|x.parse::<usize>().unwrap()).collect::<Vec<_>>();assert_eq!(ij.len(),2);let m=out.get_mut(name).expect("undeclared matrix");assert!(ij[0]>0&&ij[0]<=m.r&&ij[1]>0&&ij[1]<=m.c);assert!(!m.v[ij[1]-1].get(ij[0]-1),"duplicate entry");m.v[ij[1]-1].set(ij[0]-1);
  }
 }
 assert!(header&&!out.is_empty());out
}
fn rank(a:&M)->usize{let mut piv:Vec<Option<V>>=vec![None;a.r];let mut rank=0;for c in &a.v{let mut v=c.clone();while let Some(i)=v.top(){if let Some(p)=&piv[i]{v.xor(p);}else{piv[i]=Some(v);rank+=1;break;}}}rank}
fn plus_product(a:&M,b:&M,c:&M,d:&M,want:&M){assert_eq!(dotmul(a,b).add(&dotmul(c,d)),*want);}
fn main(){
 let args=env::args().collect::<Vec<_>>();assert_eq!(args.len(),4,"expected cubic, fixed, cohort matrix paths");
 let a=read_singular(&args[1]);let f=read_singular(&args[2]);let c=read_singular(&args[3]);
 let k=&a["kernel61"];let r=&a["recovery61"];let h=&a["homotopy61"];let conditions=&a["cubicC61"];let jets=&a["cubicJets61"];let profiles=&a["cubicProfiles61"];
 assert_eq!((k.r,k.c),(324,76));assert_eq!((conditions.r,conditions.c),(248,324));assert_eq!((profiles.r,profiles.c),(272,324));assert_eq!((jets.r,jets.c),(770,324));
 product(conditions,k,&M::z(248,76));product(r,k,&M::id(76));plus_product(k,r,h,conditions,&M::id(324));product(&a["ambientLeft61"],jets,&M::id(324));
 let s=&a["sigmaMixed61"];let n=&f["N61"];let fixed=&f["F61"];let fr=&f["R61"];let fh=&f["H61"];let q=&f["Q61"];
 product(&a["sigmaAmbient61"],&a["sigmaAmbient61"],&M::id(324));product(jets,&a["sigmaAmbient61"],&a["cubicSigmaJets61"]);assert_eq!(dotmul(&a["sigmaAmbient61"],k),dotmul(k,s));product(s,s,&M::id(76));assert_eq!(*n,s.add(&M::id(76)));
 assert_eq!((fixed.r,fixed.c),(76,38));product(n,fixed,&M::z(76,38));product(fr,fixed,&M::id(38));plus_product(fixed,fr,fh,n,&M::id(76));plus_product(n,q,q,n,&M::id(76));product(q,q,&M::z(76,76));product(n,&f["preimages61"],fixed);product(n,&f["odd61"],&a["oneMixed61"]);product(&f["J61"],&f["Jinv61"],&M::id(76));product(&f["Jinv61"],&f["J61"],&M::id(76));
 assert_eq!(c["fixed7661"],*fixed);assert_eq!(c["fixedOne61"],f["one38"]);product(fixed,&f["one38"],&a["oneMixed61"]);
 let ambient_fixed=dotmul(k,fixed);product(profiles,&ambient_fixed,&f["fixedProfiles61"]);product(jets,&ambient_fixed,&f["fixedRegular61"]);product(profiles,k,&a["profiles61"]);product(jets,k,&a["regular61"]);
 for name in ["fixedProfiles61","fixedRegular61"]{let m=&f[name];for col in &m.v{for i in (1..m.r).step_by(2){assert!(!col.get(i),"fixed imaginary coefficient");}}}
 println!("RUSTCHECK61_SPACES_CHECKED");
 let caps=[34i32,12,20,2];let offsets=[0usize,136,184,264];let mut count=0usize;let mut control_count=0;let mut last:Option<M>=None;
 for e1 in -6i32..=6{for e2 in -10i32..=10{for e3 in -1i32..=1{
  let e0=-e1-e2-e3;let signed=[e0,e1,e2,e3];let mut bound=M::z(136,324);let mut row=0;
  for p in 0..4{for order in -caps[p]..2*signed[p]{let src=offsets[p]+2*usize::try_from(order+caps[p]).unwrap();assert!(src+1<272&&row+1<136);for j in 0..324{if profiles.v[j].get(src){bound.v[j].set(row);}if profiles.v[j].get(src+1){bound.v[j].set(row+1);}}row+=2;}}
  assert_eq!(row,136);let reconstructed=dotmul(&bound,&ambient_fixed);
  if signed.iter().all(|e|*e==0){control_count+=1;assert_eq!(reconstructed,c["control61"]);assert_eq!(rank(&reconstructed),37);assert_eq!((c["controlK61"].r,c["controlK61"].c),(38,1));assert_eq!(rank(&c["controlK61"]),1);product(&reconstructed,&c["controlK61"],&M::z(136,1));product(&reconstructed,&c["fixedOne61"],&M::z(136,1));assert_eq!(rank(&c["fixedOne61"]),1);
  }else{
   count+=1;let actual=&c[&format!("Ccase{count}")];let inverse=&c[&format!("Hcase{count}")];assert_eq!(reconstructed,*actual,"ambient source rows case{count}");product(inverse,actual,&M::id(38));
   if count==1{let mut bad=inverse.clone();let j=(0..actual.c).find(|&j|!actual.v[j].zero()).unwrap();let r=actual.v[j].ones()[0];bad.v[r].w[0]^=1;assert_ne!(dotmul(&bad,actual),M::id(38),"changed inverse must fail");}
   last=Some(reconstructed);if count%100==0{println!("RUSTCHECK61_PROGRESS case={count}");}
  }
 }}}
 assert_eq!(count,818);assert_eq!(control_count,1);let last=last.unwrap();let mut changed=last.clone();changed.v[0].w[0]^=1;assert_ne!(last,changed,"changed source row must fail");
 println!("RUSTCHECK61_RESULT patterns=818 ambient_dimension=324 mixed_dimension=76 fixed_dimension=38 excluded_patterns=818 source_row_reconstructions=818 complete_left_inverses=818 regular_control_dimension=1 full_trace_contraction=1 changed_source_row_rejected=1 changed_inverse_rejected=1");
 println!("RUSTCHECK61_ALL_CHECKS_PASS");
}

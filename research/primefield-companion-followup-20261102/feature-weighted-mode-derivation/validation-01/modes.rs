//! Selected compensated modes. Caller certifies prime r and exact-order zeta.
#[path="../feature-native-hgcd-roots/polynomial.rs"] mod polynomial;
fn mul(a:u64,b:u64,r:u64)->u64{(a as u128*b as u128%r as u128)as u64}
fn pow(mut a:u64,mut n:u64,r:u64)->u64{let mut v=1;while n>0{if n&1!=0{v=mul(v,a,r)}a=mul(a,a,r);n>>=1}v}
fn eval(a:&[u64],x:u64,r:u64)->u64{a.iter().rev().fold(0,|v,&c|(mul(v,x,r)+c)%r)}
#[derive(Debug,PartialEq)] pub enum Error{Domain,Limit,Arithmetic}
pub struct Modes{r:u64,m:usize,zeta:u64,coeff:Vec<Vec<u64>>}
impl Modes{
 pub fn compile(r:u64,m:usize,zeta:u64,max_m:usize,max_coeff:usize)->Result<Self,Error>{
  if r<7||r>u64::MAX/2||m<2||m as u64>=r||(r-1)%(m as u64)!=0||zeta==0||zeta>=r{return Err(Error::Domain)}
  let h=(r-1)/2;let count=m.min(3);let max_len=(h/m as u64+1)as usize;
  if m>max_m||max_len>max_coeff{return Err(Error::Limit)}
  let mut z=1;for j in 1..=m{z=mul(z,zeta,r);if z==1&&j!=m{return Err(Error::Domain)}}if z!=1{return Err(Error::Domain)}
  let mut p=polynomial::Poly::new(r).map_err(|_|Error::Domain)?;
  // The two (-1)^m factors cancel: numerator roots are h-i.
  let nr:Vec<_>=(0..m).map(|i|(h+r-i as u64)%r).collect();let dr:Vec<_>=(0..m).map(|i|r-1-i as u64).collect();
  let num=p.product_tree(&nr).map_err(|_|Error::Arithmetic)?.polynomial().to_vec();let den=p.product_tree(&dr).map_err(|_|Error::Arithmetic)?.polynomial().to_vec();
  let mut coeff=Vec::new();let mut start=if h%2==0{1}else{r-1};
  for e in 0..count{if e>0{start=mul(mul(r-start,h-e as u64+1,r),pow(e as u64,r-2,r),r)}
   let steps=((h-e as u64)/m as u64)as usize;let mut out=vec![start];
   if steps>0{let points:Vec<_>=(0..steps).map(|j|e as u64+j as u64*m as u64).collect();let tree=p.product_tree(&points).map_err(|_|Error::Arithmetic)?;
    let ns=p.values(&num,&tree).map_err(|_|Error::Arithmetic)?;let ds=p.values(&den,&tree).map_err(|_|Error::Arithmetic)?;
    let mut prefix=vec![1;steps+1];for j in 0..steps{if ds[j]==0{return Err(Error::Arithmetic)}prefix[j+1]=mul(prefix[j],ds[j],r)}
    let mut inv=pow(prefix[steps],r-2,r);if mul(inv,prefix[steps],r)!=1{return Err(Error::Arithmetic)}let mut ids=vec![0;steps];
    for j in(0..steps).rev(){ids[j]=mul(inv,prefix[j],r);inv=mul(inv,ds[j],r)}
    for j in 0..steps{out.push(mul(mul(*out.last().unwrap(),ns[j],r),ids[j],r))}
   }coeff.push(out);
  }Ok(Self{r,m,zeta,coeff})
 }
 pub fn observed(&self,signs:&[u64])->Result<Vec<u64>,Error>{if signs.len()!=self.m||signs.iter().any(|&s|s!=0&&s!=1&&s!=self.r-1){return Err(Error::Domain)}
  let h=(self.r-1)/2;Ok((0..self.coeff.len()).map(|e|{let root=pow(self.zeta,(e as u64+self.m as u64-h%self.m as u64)%self.m as u64,self.r);let(mut w,mut sum)=(1,0);for &s in signs{sum=(sum+mul(s,w,self.r))%self.r;w=mul(w,root,self.r)}sum}).collect())
 }
 pub fn phase(&self,v:u64,b:u64,f1:u64)->Result<Option<u64>,Error>{if v>=self.r||b==0||b>=self.r||f1>=self.r{return Err(Error::Domain)}let a=mul(self.m as u64,pow(b,(self.r-1)/2,self.r),self.r);let den=mul(a,eval(&self.coeff[1],v,self.r),self.r);if den==0{return Ok(None)}let x=mul(f1,pow(den,self.r-2,self.r),self.r);if pow(x,self.m as u64,self.r)!=v{return Err(Error::Arithmetic)}Ok(Some(x))}
}
// This standalone checker enumerates reference scalars only; Modes never does.
fn main(){
 let(mut configs,mut values,mut phase_ok,mut phase_zero,mut odd,mut even,mut zero_modes,mut wrong_weight)=(0,0,0,0,0,0,0,0);
 for r in [7,11,13,17,19,23,31,41,59,97,101]{for m in 2..=16.min((r-1)as usize){if(r-1)%m as u64!=0{continue}let zeta=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let c=Modes::compile(r,m,zeta,16,60).unwrap();configs+=1;if((r-1)/m as u64)%2==0{even+=1}else{odd+=1}
  let h=(r-1)/2;let mut dense=vec![1];for _ in 0..h{let mut next=vec![0;dense.len()+1];for(j,&a)in dense.iter().enumerate(){next[j]=(next[j]+r-a)%r;next[j+1]=(next[j+1]+a)%r}dense=next;}
  for e in 0..m.min(3){assert_eq!(c.coeff[e],dense.iter().skip(e).step_by(m).copied().collect::<Vec<_>>())}
  for b in 1..=3{let a=mul(m as u64,pow(b,h,r),r);for x in 0..r{let v=pow(x,m as u64,r);let mut z=1;let signs:Vec<_>=(0..m).map(|_|{let s=pow(mul(b,(x+r-z)%r,r),h,r);z=mul(z,zeta,r);s}).collect();let fs=c.observed(&signs).unwrap();
   for e in 0..m.min(3){assert_eq!(fs[e],mul(mul(a,pow(x,e as u64,r),r),eval(&c.coeff[e],v,r),r));values+=1;if fs[e]==0{zero_modes+=1}}
   match c.phase(v,b,fs[1]).unwrap(){Some(recovered)=>{assert_eq!(recovered,x);phase_ok+=1},None=>{assert_eq!(eval(&c.coeff[1],v,r),0);phase_zero+=1}}
   if m>=3{assert_eq!(mul(mul(fs[1],fs[1],r),eval(&c.coeff[2],v,r),r),mul(mul(a,fs[2],r),pow(eval(&c.coeff[1],v,r),2,r),r))}
   let unweighted=signs.iter().fold(0,|s,&t|(s+t)%r);if unweighted!=fs[0]{wrong_weight+=1}
  }}
 }}
 for m in [64,128,256,512]{let r=65537;let c=Modes::compile(r,m,pow(3,(r-1)/m as u64,r),512,1024).unwrap();let h=(r-1)/2;let mut g=1;for n in 0..=h{if(n%m as u64)<3{assert_eq!(c.coeff[(n%m as u64)as usize][(n/m as u64)as usize],g)}if n<h{g=mul(mul(r-g,h-n,r),pow(n+1,r-2,r),r)}}}
 assert_eq!(Modes::compile(13,3,3,2,10).err(),Some(Error::Limit));assert_eq!(Modes::compile(13,3,1,3,10).err(),Some(Error::Domain));
 println!("configs={configs} even_d={even} odd_d={odd} mode_identities={values} phase_recovered={phase_ok} phase_zero_denominator={phase_zero} observed_zero_modes={zero_modes} wrong_uncompensated_weight_rejected={wrong_weight} large_compilers=4");
}

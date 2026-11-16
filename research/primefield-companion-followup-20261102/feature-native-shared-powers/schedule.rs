use std::collections::BTreeMap;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum Mode{Full,Normalized}
#[derive(Clone,Copy,Debug)]pub struct CostModel{pub multiplication:u64,pub inversion:u64}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub struct Stats{pub even:u64,pub odd:u64,pub even_reads:u64,pub nodes:usize}
impl Stats{
 pub fn choose(self,c:CostModel)->Mode{
  let delta=self.odd as i128-self.even as i128+self.even_reads as i128+1;
  let prefer=if delta<0{c.multiplication!=0||c.inversion!=0}else{(delta as u128).checked_mul(c.multiplication as u128).is_some_and(|extra|extra<c.inversion as u128)};
  if prefer{Mode::Normalized}else{Mode::Full}
 }
}
#[derive(Debug,PartialEq,Eq)]pub enum PlanError{Cap}
#[derive(Clone,Debug)]pub enum Step{Odd{a:usize,b:usize,c:usize,d:usize,half_even:bool},Even{a:usize,b:usize,c:usize,d:usize,e:usize}}
#[derive(Debug)]pub struct Schedule{pub(crate) steps:Vec<Step>,pub(crate) outputs:Vec<(usize,bool)>,stats:Stats,pub(crate) power_flags:Vec<u8>}
impl Schedule{
 pub fn new(requests:&[u64],cap:usize)->Result<Self,PlanError>{
  if cap<5{return Err(PlanError::Cap)}
  let mut ids:BTreeMap<u64,usize>=(0..5).map(|i|(i,i as usize)).collect();let mut steps=Vec::new();
  fn visit(n:u64,cap:usize,ids:&mut BTreeMap<u64,usize>,steps:&mut Vec<Step>)->Result<usize,PlanError>{
   if let Some(&i)=ids.get(&n){return Ok(i)}
   let m=n/2;
   let step=if n&1!=0{Step::Odd{a:visit(m+2,cap,ids,steps)?,b:visit(m,cap,ids,steps)?,c:visit(m-1,cap,ids,steps)?,d:visit(m+1,cap,ids,steps)?,half_even:m&1==0}}
    else{Step::Even{a:visit(m,cap,ids,steps)?,b:visit(m+2,cap,ids,steps)?,c:visit(m-1,cap,ids,steps)?,d:visit(m-2,cap,ids,steps)?,e:visit(m+1,cap,ids,steps)?}};
   if ids.len()>=cap{return Err(PlanError::Cap)}let i=ids.len();ids.insert(n,i);steps.push(step);Ok(i)
  }
  let mut outputs=Vec::with_capacity(requests.len());let mut even_reads=0;
  for &n in requests{outputs.push((visit(n,cap,&mut ids,&mut steps)?,n&1==0));if n&1==0{even_reads+=1}}
  let even=steps.iter().filter(|s|matches!(s,Step::Even{..})).count()as u64;let odd=steps.len()as u64-even;
  let stats=Stats{even,odd,even_reads,nodes:ids.len()};let mut power_flags=vec![0u8;ids.len()];
  for step in &steps{match *step{Step::Odd{b,d,..}=>{power_flags[b]|=3;power_flags[d]|=3},Step::Even{c,e,..}=>{power_flags[c]|=1;power_flags[e]|=1}}}
  Ok(Self{steps,outputs,stats,power_flags})
 }
 pub fn power_counts(&self)->(u64,u64){(self.power_flags.iter().filter(|f|**f&1!=0).count()as u64,self.power_flags.iter().filter(|f|**f&2!=0).count()as u64)}
 pub fn saved_multiplications(&self)->u128{let(s,c)=self.power_counts();2*self.stats.even as u128+4*self.stats.odd as u128-s as u128-c as u128}
 pub fn stats(&self)->Stats{self.stats}
}

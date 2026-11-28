use hec_native_ic::{fp::Fp,jac::Curve};
use std::cell::Cell;
#[derive(Debug)]pub struct RootTable{pub p:u64,pub roots:Vec<Option<u64>>,pub sqrt_lookups:Cell<u64>,pub legendre_lookups:Cell<u64>}
impl RootTable{
 pub fn build(fp:&Fp)->Self{assert!(fp.p<=65537,"bounded whole-field table");let mut roots=vec![None;fp.p as usize];let(mut square,mut delta)=(0,1);for y in 0..=(fp.p-1)/2{assert!(roots[square as usize].is_none());roots[square as usize]=Some(y);square=fp.add(square,delta);delta=fp.add(delta,2);}Self{p:fp.p,roots,sqrt_lookups:Cell::new(0),legendre_lookups:Cell::new(0)}}
 pub fn sqrt(&self,a:u64,fp:&Fp)->Option<u64>{assert_eq!(self.p,fp.p);assert!(a<fp.p);self.sqrt_lookups.set(self.sqrt_lookups.get()+1);self.roots[a as usize]}
 pub fn legendre(&self,a:u64,fp:&Fp)->i32{assert_eq!(self.p,fp.p);assert!(a<fp.p);self.legendre_lookups.set(self.legendre_lookups.get()+1);if a==0{0}else if self.roots[a as usize].is_some(){1}else{-1}}
 pub fn reset_lookups(&self){self.sqrt_lookups.set(0);self.legendre_lookups.set(0);}
 pub fn bytes(&self)->usize{self.roots.len()*std::mem::size_of::<Option<u64>>()}
}
#[derive(Debug)]pub struct Prepared{pub roots:RootTable,pub fb:Vec<(u64,u64)>}
pub fn prepare(c:&Curve,audit:bool)->Prepared{let fp=&c.fp;let roots=RootTable::build(fp);let degree=c.f.deg()as usize;let mut values:Vec<_>=(0..=degree).map(|x|c.f.eval(x as u64%fp.p,fp)).collect();let mut differences=vec![];while !values.is_empty(){differences.push(values[0]);values=values.windows(2).map(|w|fp.sub(w[1],w[0])).collect();}
 let mut fb=vec![];for x in 0..fp.p{let y2=differences[0];if audit{assert_eq!(y2,c.f.eval(x,fp));}if let Some(y)=roots.sqrt(y2,fp){fb.push((x,y));}for j in 0..degree{differences[j]=fp.add(differences[j],differences[j+1]);}}Prepared{roots,fb}}

//! Value-only predicate; caller certifies prime p and prime intended order r.
#[path="../feature-native-value-division/lib.rs"]mod division;
#[derive(Debug,PartialEq)]pub enum Error{Domain,Point,NonTorsion}
pub struct Validated{p:u64,a:u64,b:u64,r:u64,q:[u64;2],values:division::ValueDivision}
impl Validated{
 pub fn matches(&self,p:u64,a:u64,b:u64,r:u64,q:[u64;2])->bool{(self.p,self.a,self.b,self.r,self.q)==(p,a,b,r,q)}
 pub fn value(&mut self,n:u64)->u64{self.values.value(n)}
 #[cfg(test)]pub fn retained(&self)->usize{self.values.retained_indices()}
}
pub fn validate(p:u64,a:u64,b:u64,r:u64,q:[u64;2])->Result<(),Error>{validated(p,a,b,r,q).map(|_|())}
pub fn validated(p:u64,a:u64,b:u64,r:u64,q:[u64;2])->Result<Validated,Error>{
 if p<5||p>u64::MAX/2||r<3||r&1==0||r==p{return Err(Error::Domain);}
 let(mut x,mut y)=(r,p-1);while y!=0{let z=x%y;x=y;y=z;}if x!=1{return Err(Error::Domain);}
 #[cfg(test)]ENGINES.with(|n|n.set(n.get()+1));
 let mut values=division::ValueDivision::new(p,a,b,q[0],q[1]).map_err(|_|Error::Point)?;
 if values.value(r)!=0{return Err(Error::NonTorsion);}Ok(Validated{p,a,b,r,q,values})
}

#[cfg(test)]thread_local!{static ENGINES:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};}
#[cfg(test)]pub fn engines()->usize{ENGINES.with(|n|n.get())}

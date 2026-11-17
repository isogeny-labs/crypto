/// Multiplicative quotient representative, with zero adjoined. No additive API
/// and no raw-field accessor: this is not the full resultant.
#[derive(Debug)]
pub struct ResultantClass { p:u64, representative:u64 }
#[cfg(test)]std::thread_local!{static PROJECTIONS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};}
#[cfg(test)]pub fn reset_projections(){PROJECTIONS.with(|n|n.set(0));}
#[cfg(test)]pub fn projections()->usize{PROJECTIONS.with(|n|n.get())}
impl ResultantClass{
 pub fn combine(self,other:Self)->Result<Self,Error>{if self.p!=other.p{return Err(Error::Domain);}Ok(Self{p:self.p,representative:((self.representative as u128*other.representative as u128)%self.p as u128)as u64})}
 pub fn square(self)->Self{Self{p:self.p,representative:((self.representative as u128*self.representative as u128)%self.p as u128)as u64}}
 pub fn negate(self)->Self{Self{p:self.p,representative:(self.p-self.representative)%self.p}}
 pub fn is_zero(&self)->bool{self.representative==0}
 pub fn project(self)->u64{
  #[cfg(test)]PROJECTIONS.with(|n|n.set(n.get()+1));
  let order=1u64<<(self.p-1).trailing_zeros();let(mut a,mut e,mut v)=(self.representative,(self.p-1)/order,1u64);
  while e!=0{if e&1!=0{v=((v as u128*a as u128)%self.p as u128)as u64;}a=((a as u128*a as u128)%self.p as u128)as u64;e>>=1;}v
 }
}

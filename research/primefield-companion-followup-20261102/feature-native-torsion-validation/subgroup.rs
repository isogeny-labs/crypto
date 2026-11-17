//! Value-only predicate; caller certifies prime p and prime intended order r.
#[path="../feature-native-value-division/lib.rs"]mod division;
#[derive(Debug,PartialEq)]pub enum Error{Domain,Point,NonTorsion}
pub fn validate(p:u64,a:u64,b:u64,r:u64,q:[u64;2])->Result<(),Error>{
 if p<5||p>u64::MAX/2||r<3||r&1==0||r==p{return Err(Error::Domain);}
 let(mut x,mut y)=(r,p-1);while y!=0{let z=x%y;x=y;y=z;}if x!=1{return Err(Error::Domain);}
 let mut values=division::ValueDivision::new(p,a,b,q[0],q[1]).map_err(|_|Error::Point)?;
 if values.value(r)!=0{return Err(Error::NonTorsion);}Ok(())
}

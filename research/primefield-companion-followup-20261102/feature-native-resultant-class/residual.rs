//! Projected geometry residual. Additive polynomial work remains in F_p.
use super::{mul,Error,Pair};
#[path="polynomial.rs"]mod polynomial;
pub(crate) fn geometry_ratio(p:u64,part:[u64;4],q:[u64;2],grid:&[[u64;2]])->Result<Pair,Error>{
 if p<5||p>u64::MAX/2{return Err(Error::Domain);}
 let order=1u64<<(p-1).trailing_zeros();if order>4{return Err(Error::Domain);}
 let[w,k,s,e]=part;
 if grid.len()!=1+(w+k)as usize+usize::from(s!=0){return Err(Error::Geometry);}
 let(mut previous,mut current,mut square_class)=(p-1,1,1);
 let mut t=grid[0];let(mut babies,mut centers,mut prefix_x)=(Vec::new(),Vec::new(),Vec::new());
 for j in 0..w{
  babies.push(t[0]);if j<s{prefix_x.push(t[0]);}
  if order==4&&((k&1==1)^(j<s)){square_class=mul(square_class,current,p);}
  let delta=(q[0]+p-t[0])%p;if delta==0{return Err(Error::Degenerate);}
  if order==4{let next=mul(delta,previous,p);previous=current;current=next;}
  t=grid[2+j as usize];
 }
 previous=p-previous;
 for i in 0..k{
  centers.push(t[0]);if order==4&&w&1==1{square_class=mul(square_class,current,p);}
  if i+1<k{
   let delta=(grid[1][0]+p-t[0])%p;if delta==0{return Err(Error::Degenerate);}
   if order==4{let next=mul(delta,previous,p);previous=current;current=next;}
   t=grid[2+w as usize+i as usize];
  }
 }
 let mut poly=polynomial::Poly::new(p).map_err(|_|Error::Polynomial)?;
 let mut resultant=poly.root_resultant_class(&babies,&centers).map_err(|_|Error::Polynomial)?;
 if s!=0{let center=grid.last().ok_or(Error::Geometry)?;let tail=poly.root_resultant_class(&prefix_x,&[center[0]]).map_err(|_|Error::Polynomial)?;resultant=resultant.combine(tail).map_err(|_|Error::Polynomial)?;}
 if resultant.is_zero(){return Err(Error::Degenerate);}
 if order==4{let correction=poly.image_class(square_class).map_err(|_|Error::Polynomial)?.square();resultant=resultant.combine(correction).map_err(|_|Error::Polynomial)?;}
 let mut out=resultant.project();
 if ((w&1)*(k&1)+(s&1)+(e&1))&1==1{out=p-out;}
 Ok([out,1])
}

#[cfg(test)]pub(crate) fn reset_projection_count(){polynomial::reset_projections();}
#[cfg(test)]pub(crate) fn projection_count()->usize{polynomial::projections()}

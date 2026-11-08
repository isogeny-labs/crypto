enum Collection{Known{scalar:u64,calls:usize},Transcript{sign:u64,observations:Vec<Observation>,calls:usize},CallCap{calls:usize}}
fn collect_public(source:&source::Source,curve:group::Curve,base:group::Point,target:Option<group::Point>,r:u64,m:usize,zeta:u64,radii:&[u64],max_calls:usize)->Result<Collection,Error>{
 if r<7||r>u64::MAX/2||m<3||m>64||m as u64>=r||((r-1)/2)%m as u64!=0||zeta==0||zeta>=r||pow(zeta,m as u64,r)!=1||(1..m).any(|j|pow(zeta,j as u64,r)==1)||radii.is_empty()||radii.len()>4||radii.iter().any(|&b|b==0||b>=r)||max_calls==0{return Err(Error::Domain)}
 let encode=|s:i8|if s<0{r-1}else{s as u64};let sign=encode(source.sign(target.map(|p|[p.x,p.y])).map_err(|_|Error::Arithmetic)?);let mut calls=1;
 if sign==0{if target.is_some(){return Err(Error::Arithmetic)}return Ok(Collection::Known{scalar:0,calls})}
 let mut observations=vec![];
 for &b in radii{let mut z=1;let mut f=[0;3];for _ in 0..m{if calls>=max_calls{return Ok(Collection::CallCap{calls})}let off=mul(b,z,r);let point=curve.plus(target,multiple(curve,base,(r-off)%r));let s=encode(source.sign(point.map(|p|[p.x,p.y])).map_err(|_|Error::Arithmetic)?);calls+=1;
  if s==0{if multiple(curve,base,off)!=target{return Err(Error::Arithmetic)}return Ok(Collection::Known{scalar:off,calls})}
  f[0]=(f[0]+s)%r;f[1]=(f[1]+mul(z,s,r))%r;f[2]=(f[2]+mul(pow(z,r-2,r),s,r))%r;z=mul(z,zeta,r);
 }observations.push(Observation{b,f});}
 Ok(Collection::Transcript{sign,observations,calls})
}

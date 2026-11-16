fn blocked_moments(p:&mut polynomial::Poly,g:&[u64],b:&[u64],r:u64,max_words:usize)->Result<Vec<u64>,Error>{
 let n=g.len()-1;let mut width=1usize;while width.checked_mul(width).ok_or(Error::Limit)?<n+1{width+=1}
 if width.checked_mul(n).ok_or(Error::Limit)?>max_words{return Err(Error::Limit)}
 let precision=n.checked_mul(2).and_then(|v|v.checked_sub(1)).ok_or(Error::Limit)?;let q:Vec<_>=g.iter().rev().copied().collect();let inverse=p.inv_series(&q,precision).map_err(|_|Error::Arithmetic)?;let quotient=p.mul_low(&derivative(&q,r),&inverse,precision-1);
 let mut weights=vec![n as u64];for i in 0..precision-1{weights.push((r-quotient.get(i).copied().unwrap_or(0))%r)}
 let mut short=vec![vec![1]];for j in 1..width{let v=p.mul(&short[j-1],b);short.push(p.divrem(&v,g).map_err(|_|Error::Arithmetic)?.1)}
 let v=p.mul(short.last().unwrap(),b);let step=p.divrem(&v,g).map_err(|_|Error::Arithmetic)?.1;let blocks=(n+1+width-1)/width;
 let mut giant=vec![1];let mut moments=vec![0;n+1];
 for u in 0..blocks{let mut reversed=giant.clone();reversed.resize(n,0);reversed.reverse();let convolution=p.mul(&reversed,&weights);
  for v in 0..width{let j=u*width+v;if j>n{break}moments[j]=short[v].iter().enumerate().fold(0,|s,(i,&a)|(s+mul(a,convolution.get(n-1+i).copied().unwrap_or(0),r))%r);}
  if u+1<blocks{let v=p.mul(&giant,&step);giant=p.divrem(&v,g).map_err(|_|Error::Arithmetic)?.1;}
 }Ok(moments)
}
include!("blocked-image.rs");

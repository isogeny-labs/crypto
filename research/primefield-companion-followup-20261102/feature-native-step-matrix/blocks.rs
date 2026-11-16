// Polynomial power blocks: no group elements or scalar search table.
struct PowerBlocks{short:Vec<Vec<u64>>,step:Vec<u64>,width:usize}
fn power_blocks(p:&mut polynomial::Poly,g:&[u64],b:&[u64],length:usize,max_words:usize)->Result<PowerBlocks,Error>{
 if g.len()<2||length==0{return Err(Error::Domain)}let n=g.len()-1;let mut width=1usize;while width.checked_mul(width).ok_or(Error::Limit)?<length{width+=1}
 if width.checked_mul(n).ok_or(Error::Limit)?>max_words{return Err(Error::Limit)}
 let mut short=vec![vec![1]];for i in 1..width{let v=p.mul(&short[i-1],b);short.push(p.divrem(&v,g).map_err(|_|Error::Arithmetic)?.1)}
 let v=p.mul(short.last().unwrap(),b);let step=p.divrem(&v,g).map_err(|_|Error::Arithmetic)?.1;Ok(PowerBlocks{short,step,width})
}
fn blocked_certificate(p:&mut polynomial::Poly,g:&[u64],blocks:&PowerBlocks,c:&[u64],r:u64)->Result<Vec<u64>,Error>{
 let n=g.len()-1;let mut out=vec![0];let count=c.len().div_ceil(blocks.width);
 for j in (0..count).rev(){if j+1<count{let v=p.mul(&out,&blocks.step);out=p.divrem(&v,g).map_err(|_|Error::Arithmetic)?.1}
  out.resize(n,0);for i in 0..blocks.width{let at=j*blocks.width+i;if at>=c.len(){break}for(k,&v)in blocks.short[i].iter().enumerate(){out[k]=(out[k]+mul(c[at],v,r))%r}}
  while out.len()>1&&out.last()==Some(&0){out.pop();}
 }Ok(out)
}
fn trace_weights(p:&mut polynomial::Poly,g:&[u64],r:u64)->Result<Vec<u64>,Error>{
 let n=g.len()-1;let precision=n.checked_mul(2).and_then(|v|v.checked_sub(1)).ok_or(Error::Limit)?;
 let q:Vec<_>=g.iter().rev().copied().collect();let inverse=p.inv_series(&q,precision).map_err(|_|Error::Arithmetic)?;let logarithm=p.mul_low(&derivative(&q,r),&inverse,precision-1);
 let mut weights=vec![n as u64];for i in 0..precision-1{weights.push((r-logarithm.get(i).copied().unwrap_or(0))%r)}Ok(weights)
}
fn projected_prefix(p:&mut polynomial::Poly,g:&[u64],blocks:&PowerBlocks,weights:&[u64],length:usize,r:u64)->Result<Vec<u64>,Error>{
 let n=g.len()-1;let count=length.div_ceil(blocks.width);let mut giant=vec![1];let mut moments=vec![0;length];
 for u in 0..count{let mut reversed=giant.clone();reversed.resize(n,0);reversed.reverse();let convolution=p.mul(&reversed,weights);
  for v in 0..blocks.width{let j=u*blocks.width+v;if j>=length{break}moments[j]=blocks.short[v].iter().enumerate().fold(0,|s,(i,&a)|(s+mul(a,convolution.get(n-1+i).copied().unwrap_or(0),r))%r)}
  if u+1<count{let v=p.mul(&giant,&blocks.step);giant=p.divrem(&v,g).map_err(|_|Error::Arithmetic)?.1}
 }Ok(moments)
}
fn adaptive_image_blocked(p:&mut polynomial::Poly,g:&[u64],m:usize,r:u64,max_input:usize,max_output:usize,max_moments:usize,max_words:usize)->Result<AdaptiveImage,Error>{
 if r<7||r>u64::MAX/2||g.is_empty()||g.last()!=Some(&1)||g.iter().any(|&a|a>=r)||m<2||m as u64>=r||(r-1)%m as u64!=0{return Err(Error::Domain)}
 let n=g.len()-1;if n as u64>=r{return Err(Error::Domain)}if n>max_input{return Err(Error::Limit)}
 if n==0{return Ok(AdaptiveImage{images:vec![1],envelope:vec![1],moments:vec![],rejected:0})}
 if p.gcd_monic(g,&derivative(g,r),n).map_err(|_|Error::Arithmetic)?.polynomial!=[1]{return Err(Error::Domain)}
 let b=power_mod(p,g,m)?;let weights=trace_weights(p,g,r)?;let mut target=2usize;let mut rejected=0;
 loop{target=target.min(2*n);if target>max_moments{return Err(Error::Limit)}let blocks=power_blocks(p,g,&b,target,max_words)?;
  let moments=projected_prefix(p,g,&blocks,&weights,target,r)?;let c=shortest_recurrence(&moments,r);
  if blocked_certificate(p,g,&blocks,&c,r)?==[0]{let envelope=pullback(&c,m,r,max_output)?;return Ok(AdaptiveImage{images:c,envelope,moments,rejected})}
  rejected+=1;if target==2*n{return Err(Error::Arithmetic)}target=target.checked_mul(2).ok_or(Error::Limit)?;
 }
}

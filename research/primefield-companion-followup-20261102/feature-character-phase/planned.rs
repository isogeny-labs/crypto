fn planned_blocks(p:&mut polynomial::Poly,g:&[u64],b:&[u64],length:usize,max_words:usize)->Result<PowerBlocks,Error>{
 if g.len()<2{return Err(Error::Domain)}let width=width_plan(g.len()-1,length,max_words)?.width;
 let mut short=vec![vec![1]];for i in 1..width{let v=p.mul(&short[i-1],b);short.push(p.divrem(&v,g).map_err(|_|Error::Arithmetic)?.1)}
 let v=p.mul(short.last().unwrap(),b);let step=p.divrem(&v,g).map_err(|_|Error::Arithmetic)?.1;Ok(PowerBlocks{short,step,width})
}
fn adaptive_image_workspace(p:&mut polynomial::Poly,g:&[u64],m:usize,r:u64,max_input:usize,max_output:usize,max_moments:usize,max_words:usize)->Result<AdaptiveImage,Error>{
 if r<7||r>u64::MAX/2||g.is_empty()||g.last()!=Some(&1)||g.iter().any(|&a|a>=r)||m<2||m as u64>=r||(r-1)%m as u64!=0{return Err(Error::Domain)}
 let n=g.len()-1;if n as u64>=r{return Err(Error::Domain)}if n>max_input{return Err(Error::Limit)}
 if n==0{return Ok(AdaptiveImage{images:vec![1],envelope:vec![1],moments:vec![],rejected:0})}
 if p.gcd_monic(g,&derivative(g,r),n).map_err(|_|Error::Arithmetic)?.polynomial!=[1]{return Err(Error::Domain)}
 let b=power_mod(p,g,m)?;let weights=trace_weights(p,g,r)?;let mut target=2usize;let mut rejected=0;
 loop{target=target.min(2*n);if target>max_moments{return Err(Error::Limit)}let blocks=planned_blocks(p,g,&b,target,max_words)?;
  let moments=projected_prefix(p,g,&blocks,&weights,target,r)?;let c=shortest_recurrence(&moments,r);
  if blocked_certificate(p,g,&blocks,&c,r)?==[0]{let envelope=pullback(&c,m,r,max_output)?;return Ok(AdaptiveImage{images:c,envelope,moments,rejected})}
  rejected+=1;if target==2*n{return Err(Error::Arithmetic)}target=target.checked_mul(2).ok_or(Error::Limit)?;
 }
}

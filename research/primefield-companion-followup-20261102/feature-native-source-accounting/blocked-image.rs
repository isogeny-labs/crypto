fn trace_image_blocked(p:&mut polynomial::Poly,g:&[u64],m:usize,r:u64,max_input:usize,max_output:usize,max_workspace:usize)->Result<TraceImage,Error>{
 if r<7||g.is_empty()||g.last()!=Some(&1)||g.iter().any(|&a|a>=r)||m<2||m as u64>=r||(r-1)%m as u64!=0{return Err(Error::Domain)}
 let n=g.len()-1;if n as u64>=r{return Err(Error::Domain)}if n>max_input{return Err(Error::Limit)}
 if n==0{return Ok(TraceImage{characteristic:vec![1],images:vec![1],envelope:vec![1],moments:vec![0]})}
 if p.gcd_monic(g,&derivative(g,r),n).map_err(|_|Error::Arithmetic)?.polynomial!=[1]{return Err(Error::Domain)}
 let b=power_mod(p,g,m)?;let moments=blocked_moments(p,g,&b,r,max_workspace)?;
 let characteristic=converter::from_moments(r,&moments,max_input).map_err(|_|Error::Arithmetic)?.coefficients;
 let common=p.gcd_monic(&characteristic,&derivative(&characteristic,r),n).map_err(|_|Error::Arithmetic)?.polynomial;
 let(images,rem)=p.divrem(&characteristic,&common).map_err(|_|Error::Arithmetic)?;if rem!=[0]{return Err(Error::Arithmetic)}
 let envelope=pullback(&images,m,r,max_output)?;
 Ok(TraceImage{characteristic,images,envelope,moments})
}
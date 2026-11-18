// Coefficients in ascending order. A shortest finite-prefix recurrence is
// provisional until its polynomial annihilates X^m in the entire quotient.
fn shortest_recurrence(s:&[u64],r:u64)->Vec<u64>{
 let(mut c,mut old)=(vec![1u64],vec![1u64]);let(mut degree,mut shift,mut discrepancy)=(0usize,1usize,1u64);
 for k in 0..s.len(){let mut d=s[k];for i in 1..=degree{d=(d+mul(c.get(i).copied().unwrap_or(0),s[k-i],r))%r}
  if d==0{shift+=1;continue}let saved=c.clone();let factor=mul(d,pow(discrepancy,r-2,r),r);c.resize(c.len().max(old.len()+shift),0);
  for(i,&v)in old.iter().enumerate(){c[i+shift]=(c[i+shift]+r-mul(factor,v,r))%r}
  if 2*degree<=k{degree=k+1-degree;old=saved;discrepancy=d;shift=1}else{shift+=1}
 }
 c.resize(degree+1,0);c.reverse();c
}
fn quotient_evaluate(p:&mut polynomial::Poly,g:&[u64],b:&[u64],d:&[u64],r:u64)->Result<Vec<u64>,Error>{
 let mut v=vec![0];for &a in d.iter().rev(){let product=p.mul(&v,b);v=p.divrem(&product,g).map_err(|_|Error::Arithmetic)?.1;v[0]=(v[0]+a)%r;}Ok(v)
}
struct AdaptiveImage{images:Vec<u64>,envelope:Vec<u64>,moments:Vec<u64>,rejected:usize}
fn adaptive_image(p:&mut polynomial::Poly,g:&[u64],m:usize,r:u64,max_input:usize,max_output:usize,max_moments:usize)->Result<AdaptiveImage,Error>{
 if r<7||r>u64::MAX/2||g.is_empty()||g.last()!=Some(&1)||g.iter().any(|&a|a>=r)||m<2||m as u64>=r||(r-1)%m as u64!=0{return Err(Error::Domain)}
 let n=g.len()-1;if n as u64>=r{return Err(Error::Domain)}if n>max_input{return Err(Error::Limit)}
 if n==0{return Ok(AdaptiveImage{images:vec![1],envelope:vec![1],moments:vec![],rejected:0})}
 if p.gcd_monic(g,&derivative(g,r),n).map_err(|_|Error::Arithmetic)?.polynomial!=[1]{return Err(Error::Domain)}
 let q:Vec<_>=g.iter().rev().copied().collect();let inv=p.inv_series(&q,n).map_err(|_|Error::Arithmetic)?;let logarithm=p.mul_low(&derivative(&q,r),&inv,n-1);
 let mut weights=vec![n as u64];for i in 0..n-1{weights.push((r-logarithm.get(i).copied().unwrap_or(0))%r)}
 let b=power_mod(p,g,m)?;let mut current=vec![1];let mut moments=vec![];let mut target=2usize;let mut rejected=0;
 loop{target=target.min(2*n);if target>max_moments{return Err(Error::Limit)}
  while moments.len()<target{moments.push(current.iter().zip(&weights).fold(0,|s,(&a,&w)|(s+mul(a,w,r))%r));if moments.len()<target{let v=p.mul(&current,&b);current=p.divrem(&v,g).map_err(|_|Error::Arithmetic)?.1}}
  let d=shortest_recurrence(&moments,r);
  if quotient_evaluate(p,g,&b,&d,r)?==[0]{let envelope=pullback(&d,m,r,max_output)?;return Ok(AdaptiveImage{images:d,envelope,moments,rejected})}
  rejected+=1;if target==2*n{return Err(Error::Arithmetic)}let v=p.mul(&current,&b);current=p.divrem(&v,g).map_err(|_|Error::Arithmetic)?.1;target=target.checked_mul(2).ok_or(Error::Limit)?;
 }
}
fn adaptive_controls(){
 let r=7;let mut p=polynomial::Poly::new(r).unwrap();
 let g=schoolbook_roots(&[0,1,6],r);let b=power_mod(&mut p,&g,2).unwrap();
 let wrong=shortest_recurrence(&[2,2,2,2],r);assert_eq!(wrong,vec![6,1]);assert_ne!(quotient_evaluate(&mut p,&g,&b,&wrong,r).unwrap(),vec![0]);
 let got=adaptive_image(&mut p,&g,2,r,3,7,6).unwrap();assert_eq!(got.images,vec![0,6,1]);assert!(got.rejected>0);
 let g=schoolbook_roots(&[1,2,3,4,5,6],r);let got=adaptive_image(&mut p,&g,6,r,6,7,2).unwrap();assert_eq!(got.images,vec![6,1]);assert_eq!(got.moments.len(),2);
 let full=schoolbook_roots(&(0..r).collect::<Vec<_>>(),r);assert!(matches!(adaptive_image(&mut p,&full,2,r,7,7,14),Err(Error::Domain)));
 assert_eq!(adaptive_image(&mut p,&[1],2,r,0,0,0).unwrap().images,vec![1]);
 println!("adaptive_named_controls=zero_transient,single_image_two_moments,degree_r_rejection,empty_input");
}

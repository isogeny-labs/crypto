include!("blocks.rs");
include!("checks.rs");
include!("adaptive.rs");
#[path="../feature-native-fast-moment-roots/lib.rs"]mod converter;
struct TraceImage{characteristic:Vec<u64>,images:Vec<u64>,envelope:Vec<u64>,moments:Vec<u64>}
fn derivative(a:&[u64],r:u64)->Vec<u64>{if a.len()<2{return vec![0]}a.iter().enumerate().skip(1).map(|(i,&v)|mul(i as u64,v,r)).collect()}
fn power_mod(p:&mut polynomial::Poly,g:&[u64],mut exponent:usize)->Result<Vec<u64>,Error>{let mut base=p.divrem(&[0,1],g).map_err(|_|Error::Arithmetic)?.1;let mut out=vec![1];while exponent>0{if exponent&1!=0{let v=p.mul(&out,&base);out=p.divrem(&v,g).map_err(|_|Error::Arithmetic)?.1}exponent>>=1;if exponent>0{let v=p.mul(&base,&base);base=p.divrem(&v,g).map_err(|_|Error::Arithmetic)?.1}}Ok(out)}
fn trace_image(p:&mut polynomial::Poly,g:&[u64],m:usize,r:u64,max_input:usize,max_output:usize)->Result<TraceImage,Error>{
 if r<7||g.is_empty()||g.last()!=Some(&1)||g.iter().any(|&a|a>=r)||m<2||m as u64>=r||(r-1)%m as u64!=0{return Err(Error::Domain)}
 let n=g.len()-1;if n as u64>=r{return Err(Error::Domain)}if n>max_input{return Err(Error::Limit)}
 if n==0{return Ok(TraceImage{characteristic:vec![1],images:vec![1],envelope:vec![1],moments:vec![0]})}
 if p.gcd_monic(g,&derivative(g,r),n).map_err(|_|Error::Arithmetic)?.polynomial!=[1]{return Err(Error::Domain)}
 // -Q'/Q gives power sums 1..n-1, Q=T^n G(1/T), Q(0)=1.
 let q:Vec<_>=g.iter().rev().copied().collect();let inverse=p.inv_series(&q,n).map_err(|_|Error::Arithmetic)?;let quotient=p.mul_low(&derivative(&q,r),&inverse,n-1);
 let mut weights=vec![n as u64];for i in 0..n-1{weights.push((r-quotient.get(i).copied().unwrap_or(0))%r)}
 let b=power_mod(p,g,m)?;let mut current=vec![1];let mut moments=vec![n as u64];
 for _ in 1..=n{let v=p.mul(&current,&b);current=p.divrem(&v,g).map_err(|_|Error::Arithmetic)?.1;moments.push(current.iter().zip(&weights).fold(0,|s,(&a,&w)|(s+mul(a,w,r))%r));}
 let characteristic=converter::from_moments(r,&moments,max_input).map_err(|_|Error::Arithmetic)?.coefficients;
 let common=p.gcd_monic(&characteristic,&derivative(&characteristic,r),n).map_err(|_|Error::Arithmetic)?.polynomial;
 let(images,rem)=p.divrem(&characteristic,&common).map_err(|_|Error::Arithmetic)?;if rem!=[0]{return Err(Error::Arithmetic)}
 let envelope=pullback(&images,m,r,max_output)?;
 Ok(TraceImage{characteristic,images,envelope,moments})
}
include!("block-projection.rs");
include!("history.rs");include!("trace-adapter.rs");
fn schoolbook_roots(roots:&[u64],r:u64)->Vec<u64>{let mut f=vec![1];for &x in roots{let mut next=vec![0;f.len()+1];for(i,&a)in f.iter().enumerate(){next[i]=(next[i]+r-mul(x,a,r))%r;next[i+1]=(next[i+1]+a)%r}f=next}f}
fn trace_checks(){let(mut adaptive_rejections,mut early)=(0usize,0usize);let(mut subsets,mut collisions,mut zeros,mut rejected_full)=(0,0,0,0);
 for r in [7,11]{for m in 2..r as usize{if(r-1)%m as u64!=0{continue}let zeta=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let parent:Vec<_>=(0..=r).map(|i|if i==1{r-1}else if i==r{1}else{0}).collect();let mut p=polynomial::Poly::new(r).unwrap();
  for mask in 0..1usize<<r{let roots:Vec<_>=(0..r).filter(|&x|mask&(1usize<<x)!=0).collect();let g=schoolbook_roots(&roots,r);if roots.len()==r as usize{assert!(matches!(trace_image(&mut p,&g,m,r,r as usize,r as usize),Err(Error::Domain)));rejected_full+=1;continue}
   let got=trace_image(&mut p,&g,m,r,r as usize,r as usize).unwrap();let blocked=trace_image_blocked(&mut p,&g,m,r,r as usize,r as usize,4096).unwrap();assert_eq!((&got.characteristic,&got.images,&got.envelope,&got.moments),(&blocked.characteristic,&blocked.images,&blocked.envelope,&blocked.moments));let multiset:Vec<_>=roots.iter().map(|&x|pow(x,m as u64,r)).collect();let mut distinct=multiset.clone();distinct.sort();distinct.dedup();assert_eq!(got.characteristic,schoolbook_roots(&multiset,r));assert_eq!(got.images,schoolbook_roots(&distinct,r));
   for(j,&s)in got.moments.iter().enumerate(){assert_eq!(s,multiset.iter().fold(0,|a,&x|(a+pow(x,j as u64,r))%r))}
   let adaptive=adaptive_image(&mut p,&g,m,r,r as usize,r as usize,2*r as usize).unwrap();assert_eq!((&adaptive.images,&adaptive.envelope),(&got.images,&got.envelope));let candidate=adaptive_image_blocked(&mut p,&g,m,r,r as usize,4096,2*r as usize,4096).unwrap();assert_eq!((&candidate.images,&candidate.envelope,&candidate.moments,candidate.rejected),(&adaptive.images,&adaptive.envelope,&adaptive.moments,adaptive.rejected));assert!(adaptive.moments.len()<=4*distinct.len());
   if !roots.is_empty(){let b=power_mod(&mut p,&g,m).unwrap();let mut corrupt=adaptive.images.clone();corrupt[0]=(corrupt[0]+1)%r;assert_ne!(quotient_evaluate(&mut p,&g,&b,&corrupt,r).unwrap(),vec![0]);}
   adaptive_rejections+=adaptive.rejected;if !roots.is_empty()&&adaptive.moments.len()<roots.len()+1{early+=1;}
   let expected=stable_envelope(&mut p,&g,&parent,zeta,m,r,r as usize).unwrap().0;assert_eq!(got.envelope,expected);subsets+=1;if distinct.len()<roots.len(){collisions+=1}if roots.contains(&0){zeros+=1}
  }
 }}
 let mut larger=0;for (n,m) in [(8,128),(16,64),(32,32)]{let r=65537;let roots:Vec<_>=(0..n as u64).collect();let mut p=polynomial::Poly::new(r).unwrap();let g=schoolbook_roots(&roots,r);let got=trace_image(&mut p,&g,m,r,32,4096).unwrap();let blocked=trace_image_blocked(&mut p,&g,m,r,32,4096,4096).unwrap();assert_eq!((&got.characteristic,&got.images,&got.envelope,&got.moments),(&blocked.characteristic,&blocked.images,&blocked.envelope,&blocked.moments));let adaptive=adaptive_image(&mut p,&g,m,r,32,4096,64).unwrap();assert_eq!((&adaptive.images,&adaptive.envelope),(&got.images,&got.envelope));let candidate=adaptive_image_blocked(&mut p,&g,m,r,r as usize,4096,2*r as usize,4096).unwrap();assert_eq!((&candidate.images,&candidate.envelope,&candidate.moments,candidate.rejected),(&adaptive.images,&adaptive.envelope,&adaptive.moments,adaptive.rejected));let multiset:Vec<_>=roots.iter().map(|&x|pow(x,m as u64,r)).collect();let mut distinct=multiset.clone();distinct.sort();distinct.dedup();assert_eq!(got.characteristic,schoolbook_roots(&multiset,r));assert_eq!(got.images,schoolbook_roots(&distinct,r));assert_eq!(p.divrem(&got.envelope,&g).unwrap().1,vec![0]);larger+=1;}
 let mut p=polynomial::Poly::new(13).unwrap();assert!(matches!(trace_image(&mut p,&[1,11,1],3,13,3,13),Err(Error::Domain)));assert!(matches!(trace_image(&mut p,&[12,1],3,13,0,13),Err(Error::Limit)));assert!(matches!(trace_image(&mut p,&[12,1],3,13,1,2),Err(Error::Limit)));
 assert!(matches!(trace_image_blocked(&mut p,&[12,1],3,13,1,13,1),Err(Error::Limit)));
 assert!(adaptive_rejections>0);assert!(early>0);assert!(matches!(adaptive_image(&mut p,&[12,1],3,13,1,13,1),Err(Error::Limit)));assert!(matches!(adaptive_image(&mut p,&[1,11,1],3,13,3,13,6),Err(Error::Domain)));println!("adaptive_rejected_prefixes={adaptive_rejections} fewer_than_n_plus_one_moments={early}");
 println!("trace_subsets={subsets} collision_multisets={collisions} zero_subsets={zeros} degree_r_rejections={rejected_full} larger_cases={larger}");}
fn main(){block_checks();adaptive_controls();envelope_checks();trace_checks();let(mut cases,mut calls)=(0,0);
 for(p,a,b,r,base,m)in[(7,0,3,13,[1,2],3),(7,0,3,13,[1,2],6),(509,2,10,479,[3,68],2),(701,2,8,683,[2,106],11)]{let c=group::Curve::new(p,a,b);let bp=group::Point{x:base[0],y:base[1]};let source=source::Source::new(p,a,b,r,base,64).unwrap();let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=Modes::compile(r,m,z,16,256).unwrap();let scalars:Vec<u64>=if r==13{(0..r).collect()}else if r==479{include_str!("../feature-native-monomial-recovery/recovery.tsv").lines().skip(1).map(|l|l.split('\t').next().unwrap().parse().unwrap()).collect()}else{vec![0,1,2,3,7,11,17,31,47,83,127,191,257,341,509,681,682]};
  for k in scalars{let t=multiple(c,bp,k);let extra:Vec<_>=(0..17.min(r)).collect();let(old,os,route)=modes.recover_enveloped(&source,c,bp,t,&[1,2,3,4],64,&extra,128).unwrap();ENVELOPE_PROFILE.with(|n|n.borrow_mut().clear());let(new,ns,nroute)=modes.recover_traced(&source,c,bp,t,&[1,2,3,4],64,&extra,128).unwrap();assert_eq!(new,old);assert_eq!(new,Some(k));assert_eq!(route,nroute);assert_eq!((os.calls,os.replayed,os.extra,os.lifted,os.image_count),(ns.calls,ns.replayed,ns.extra,ns.lifted,ns.image_count));cases+=1;calls+=ns.calls;}
 }println!("trace_public_equalities={cases} source_calls={calls}");}

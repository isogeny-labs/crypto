// Caller supplies a monic divisor of Z(Z^d-1), maintained by image gcds.
fn pullback(f:&[u64],m:usize,r:u64,cap:usize)->Result<Vec<u64>,Error>{
 if m<2||m as u64>=r||(r-1)%m as u64!=0||f.is_empty()||f.last()!=Some(&1)||f.iter().any(|&v|v>=r){return Err(Error::Domain)}
 if f.len()==1{return Ok(vec![1])}
 let zero=f[0]==0;if zero&&f.get(1)==Some(&0){return Err(Error::Domain)}
 let degree=(f.len()-1).checked_mul(m).ok_or(Error::Limit)?.checked_sub(if zero{m-1}else{0}).ok_or(Error::Domain)?;
 if degree>cap{return Err(Error::Limit)}let mut out=vec![0;degree+1];
 for(i,&a)in f.iter().enumerate(){if zero&&i==0{continue}out[i*m-if zero{m-1}else{0}]=a}Ok(out)
}
fn replay(p:&mut polynomial::Poly,f:&[u64],offset:u64,sign:u64,r:u64,cap:usize)->Result<Vec<u64>,Error>{
 if offset>=r||sign!=0&&sign!=1&&sign!=r-1||f.len().saturating_sub(1)>cap{return Err(Error::Domain)}
 if f.len()==1{return Ok(f.to_vec())}
 let mut a=p.divrem(&[offset,1],f).map_err(|_|Error::Arithmetic)?.1;let mut n=(r-1)/2;let mut b=vec![1];
 while n>0{if n&1!=0{let v=p.mul(&b,&a);b=p.divrem(&v,f).map_err(|_|Error::Arithmetic)?.1}n>>=1;if n>0{let v=p.mul(&a,&a);a=p.divrem(&v,f).map_err(|_|Error::Arithmetic)?.1}}
 b[0]=(b[0]+r-sign)%r;Ok(p.gcd_monic(f,&b,cap).map_err(|_|Error::Arithmetic)?.polynomial)
}
#[derive(Default,Debug)]struct PullStats{calls:usize,replayed:usize,extra:usize,lifted:usize,image_count:usize}
impl Modes{
 fn recover_pulled(&self,source:&source::Source,curve:group::Curve,base:group::Point,target:Option<group::Point>,radii:&[u64],cap:usize,extra:&[u64],max_calls:usize)->Result<(Option<u64>,PullStats,&'static str),Error>{
  if radii.len()>4||radii.iter().any(|&b|b==0||b>=self.r)||extra.len()>32||extra.iter().any(|&e|e>=self.r)||max_calls==0{return Err(Error::Domain)}
  let r=self.r;let s=source.sign(target.map(|p|[p.x,p.y])).map_err(|_|Error::Domain)?;let mut stats=PullStats{calls:1,..Default::default()};
  let finish=|k,stats,route|{if multiple(curve,base,k)!=target{Err(Error::Arithmetic)}else{Ok((Some(k),stats,route))}};
  if s==0{return finish(0,stats,"identity")}
  let mut history=vec![(0,if s<0{r-1}else{s as u64})];let d=((r-1)/self.m as u64)as usize;let mut state=vec![0;d+2];state[1]=r-1;state[d+1]=1;let mut p=polynomial::Poly::new(r).unwrap();
  for &b in radii{let mut zeta=1;let mut signs=Vec::new();for _ in 0..self.m{if stats.calls>=max_calls{return Ok((None,stats,"call-cap"))}let offset=mul(b,zeta,r);let q=curve.plus(target,multiple(curve,base,(r-offset)%r));let s=source.sign(q.map(|p|[p.x,p.y])).map_err(|_|Error::Arithmetic)?;stats.calls+=1;
    if s==0{return finish(offset,stats,"endpoint")}let sign=if s<0{r-1}else{s as u64};signs.push(sign);history.push(((r-offset)%r,sign));zeta=mul(zeta,self.zeta,r);}
   let f=self.observed(&signs)?;for constraint in self.constraints(b,&f){state=p.gcd_monic(&state,&constraint,d+1).map_err(|_|Error::Arithmetic)?.polynomial;}
   if state.len()==1{return Err(Error::Arithmetic)}
   if state.len()==2{let image=(r-state[0])%r;let v=mul(image,pow(pow(b,self.m as u64,r),r-2,r),r);
    if let Some(x)=self.phase(v,b,f[1])?{return finish(mul(b,x,r),stats,"mode-phase")}
   }
   let mut fiber=match pullback(&state,self.m,r,cap){Ok(f)=>f,Err(Error::Limit)=>continue,Err(e)=>return Err(e)};
   stats.lifted=fiber.len()-1;stats.image_count=state.len()-1;
   for &(offset,sign)in &history{fiber=replay(&mut p,&fiber,offset,sign,r,cap)?;stats.replayed+=1;}
   drop(state);drop(history);
   if fiber.len()==1{return Err(Error::Arithmetic)}if fiber.len()==2{return finish((r-fiber[0])%r,stats,"retained-set")}
   for &offset in extra{if stats.calls>=max_calls{return Ok((None,stats,"call-cap"))}let q=curve.plus(target,multiple(curve,base,offset));let s=source.sign(q.map(|p|[p.x,p.y])).map_err(|_|Error::Arithmetic)?;stats.calls+=1;stats.extra+=1;
    if s==0{return finish((r-offset)%r,stats,"extra-endpoint")}fiber=replay(&mut p,&fiber,offset,if s<0{r-1}else{s as u64},r,cap)?;
    if fiber.len()==1{return Err(Error::Arithmetic)}if fiber.len()==2{return finish((r-fiber[0])%r,stats,"extra-sign")}
   }return Ok((None,stats,"fiber-exhausted"))
  }Ok((None,stats,"image-exhausted"))
 }
}
fn exact_sets(){let(mut sets,mut zero_sets,mut prefixes,mut limits)=(0,0,0,0);
 for r in [7,11,13,17,19,31]{for m in 2..r as usize{if(r-1)%m as u64!=0{continue}let mut images=Vec::new();for x in 0..r{let v=pow(x,m as u64,r);if !images.contains(&v){images.push(v)}}if images.len()>8{continue}
  let mut p=polynomial::Poly::new(r).unwrap();for mask in 0..1usize<<images.len(){let selected:Vec<_>=images.iter().enumerate().filter(|&(j,_)|mask&(1<<j)!=0).map(|(_,&v)|v).collect();let f=if selected.is_empty(){vec![1]}else{p.product_tree(&selected).unwrap().polynomial().to_vec()};let lifted=pullback(&f,m,r,r as usize).unwrap();sets+=1;if selected.contains(&0){zero_sets+=1}
   let expected:Vec<_>=(0..r).filter(|&x|selected.contains(&pow(x,m as u64,r))).collect();assert_eq!((0..r).filter(|&x|eval(&lifted,x,r)==0).collect::<Vec<_>>(),expected);assert_eq!(lifted.len()-1,expected.len());
   if !expected.is_empty(){assert_eq!(pullback(&f,m,r,expected.len()-1).err(),Some(Error::Limit));limits+=1;let target=expected[0];let mut state=lifted.clone();let mut surviving=expected;
    for a in 0..4.min(r){let sign=pow((target+a)%r,(r-1)/2,r);state=replay(&mut p,&state,a,sign,r,r as usize).unwrap();surviving.retain(|&x|pow((x+a)%r,(r-1)/2,r)==sign);assert_eq!((0..r).filter(|&x|eval(&state,x,r)==0).collect::<Vec<_>>(),surviving);assert_eq!(state.len()-1,surviving.len());prefixes+=1}
   }
  }
 }}assert_eq!(pullback(&[0,0,1],2,13,10).err(),Some(Error::Domain));println!("subset_checks={sets} zero_containing_sets={zero_sets} replay_prefixes={prefixes} cap_rejections={limits}");}
fn main(){previous_checks();exact_sets();let(mut cases,mut recovered,mut unresolved,mut calls,mut extra,mut replayed,mut max_lift,mut nonsingleton)=(0,0,0,0,0,0,0,0);
 for(p,a,b,r,base,m)in[(7,0,3,13,[1,2],3),(7,0,3,13,[1,2],6),(509,2,10,479,[3,68],2),(701,2,8,683,[2,106],11)]{
  let c=group::Curve::new(p,a,b);let bp=group::Point{x:base[0],y:base[1]};let source=source::Source::new(p,a,b,r,base,64).unwrap();let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=Modes::compile(r,m,z,16,256).unwrap();
  let scalars:Vec<u64>=if r==13{(0..r).collect()}else if r==479{include_str!("../feature-native-monomial-recovery/recovery.tsv").lines().skip(1).map(|l|l.split('\t').next().unwrap().parse().unwrap()).collect()}else{vec![0,1,2,3,7,11,17,31,47,83,127,191,257,341,509,681,682]};
  let before=(cases,recovered,unresolved);for k in scalars{let target=multiple(c,bp,k);let(out,s,route)=modes.recover_pulled(&source,c,bp,target,&[1,2,3,4],64,&(0..17.min(r)).collect::<Vec<_>>(),128).unwrap();cases+=1;calls+=s.calls;extra+=s.extra;replayed+=s.replayed;max_lift=max_lift.max(s.lifted);if s.image_count>1{nonsingleton+=1}match out{Some(v)=>{assert_eq!(v,k);recovered+=1},None=>unresolved+=1}println!("row r={r} m={m} k={k} recovered={} route={route} calls={} lifted={} images={} extra={}",out.is_some(),s.calls,s.lifted,s.image_count,s.extra);
   assert!(modes.recover_pulled(&source,c,bp,target,&[1],0,&[],1).unwrap().0.is_none()||k==0);
  }println!("pulled_fixture r={r} m={m} cases={} recovered={} unresolved={}",cases-before.0,recovered-before.1,unresolved-before.2);
 }println!("pulled_cases={cases} recovered={recovered} unresolved={unresolved} calls={calls} replayed={replayed} extra={extra} max_lifted_degree={max_lift} nonsingleton_transitions={nonsingleton}");}

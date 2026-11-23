// V_j = U*c_j modulo the current precision, with U a unit.
fn selected_coefficients<F:FnMut(u64,u64)>(r:u64,f:&[u64],wanted:&std::collections::BTreeSet<u64>,caps:StreamCaps,stats:&mut StreamStats,mut take:F)->Result<(),Error>{
 if r<7||r>i64::MAX as u64||f.is_empty()||f[0]==0||f.last()==Some(&0)||f.iter().any(|&x|x>=r){return Err(Error::Domain)}let d=f.len()-1;let h=(r-1)/2;let degree=(d as u64).checked_mul(h).ok_or(Error::Limit)?;if wanted.iter().any(|&k|k>degree){return Err(Error::Domain)}if wanted.is_empty(){return Ok(())}
 if degree>caps.steps.saturating_sub(stats.steps){stats.limit=Some("steps");return Err(Error::Limit)}let mut valuation=0;let mut n=degree;while n>0{n/=r;valuation+=n;}let mut modulus=r;for _ in 0..valuation{modulus=match modulus.checked_mul(r){Some(x)if x<=i64::MAX as u64=>x,_=>{stats.limit=Some("precision");return Err(Error::Limit)}}}
 stats.max_modulus=stats.max_modulus.max(modulus);let c0=stream_power(f[0],h,modulus,stats);if wanted.contains(&0){take(0,c0%r)}if d==0{return Ok(())}let mut window=std::collections::VecDeque::from([c0]);let mut denominator=1;stats.max_window=stats.max_window.max(window.len());
 for k in 1..=degree{stats.steps+=1;let mut numerator=0;for j in 1..=d.min(k as usize){let factor=(((h as i128+1)*j as i128-k as i128).rem_euclid(modulus as i128))as u64;let v=stream_mul(factor,f[j],modulus,stats);let v=stream_mul(v,window[j-1],modulus,stats);numerator=(numerator+v)%modulus;}
  let mut unit=k;let mut divisor=1;while unit%r==0{unit/=r;divisor*=r;stats.precision_drops+=1;}if numerator%divisor!=0{return Err(Error::Arithmetic)}modulus/=divisor;let q=stream_mul(unit%modulus,f[0],modulus,stats);denominator=stream_mul(denominator%modulus,q,modulus,stats);
  if window.len()==d{window.pop_back();}for v in &mut window{*v=stream_mul(*v%modulus,q,modulus,stats);}let scaled=numerator/divisor;window.push_front(scaled);stats.max_window=stats.max_window.max(window.len());
  if wanted.contains(&k){stats.inversions+=1;let inv=unit_inverse(denominator%r,r)?;let coefficient=stream_mul(scaled%r,inv,r,stats);take(k,coefficient);}
 }if modulus!=r{return Err(Error::Arithmetic)}Ok(())
}
fn selected_trace(r:u64,f:&[u64],epsilon:u64,m:usize,caps:StreamCaps,stats:&mut StreamStats)->Result<[u64;2],Error>{
 if r<7||f.is_empty()||(epsilon!=1&&epsilon!=r-1){return Err(Error::Domain)}let h=(r-1)/2;if m==0||m as u64>h{return Err(Error::Domain)}let d=f.len()-1;let degree=(d as u64).checked_mul(h).ok_or(Error::Limit)?;if degree>caps.steps.saturating_sub(stats.steps){stats.limit=Some("steps");return Err(Error::Limit)}let mut wanted=std::collections::BTreeSet::new();for q in 0..=d{wanted.insert(q as u64*h);}for q in 0..=d+1{if let Some(k)=(q as u64).checked_mul(h).and_then(|x|x.checked_sub(m as u64)){if k<=degree{wanted.insert(k);}}}
 let mut sums=[0,0];selected_coefficients(r,f,&wanted,caps,stats,|k,c|{for(i,w)in [0,m as u64].iter().enumerate(){let e=k+w;if e%h==0{let weight=if epsilon==1||e/h%2==0{1}else{r-1};sums[i]=(sums[i]+mul(c,weight,r))%r;}}})?;Ok([mul(h,sums[0],r),mul(h,sums[1],r)])
}

fn selected_word_moments(problem:&CorrectedProblem,zeta:u64,words:&[Word],caps:StreamCaps)->Result<([u64;2],StreamStats),(Error,StreamStats)>{let mut stats=StreamStats::default();let r=problem.inner.r;let m=problem.inner.m;
 let result=(||{let n=words.len().checked_mul(m).ok_or(Error::Limit)?;let subsets=match 1usize.checked_shl(n as u32){Some(x)if n<usize::BITS as usize&&x<=caps.subsets=>x,_=>{stats.limit=Some("subsets");return Err(Error::Limit)}};let mut offsets=vec![];for w in words{let mut z=1;for &s in &w.signs{offsets.push((mul(w.b,z,r),s));z=mul(z,zeta,r)}}let mut sums=[0,0];
  for mask in 0..subsets{stats.subsets+=1;let mut f=vec![1];let mut weight=1;for(i,&(a,s))in offsets.iter().enumerate(){if mask&(1<<i)==0{continue}weight=mul(weight,s,r);let mut next=vec![0;f.len()+1];for(j,&c)in f.iter().enumerate(){next[j]=(next[j]+mul(r-a,c,r))%r;next[j+1]=(next[j+1]+c)%r;stats.polynomial_pairs+=1;}f=next;}let tr=selected_trace(r,&f,problem.inner.epsilon,m,caps,&mut stats)?;for j in 0..2{sums[j]=(sums[j]+mul(weight,tr[j],r))%r;}}
  let scale=mul(pow(pow(2,r-2,r),n as u64,r),pow(problem.inner.stabilizer as u64,r-2,r),r);for j in 0..2{sums[j]=(mul(scale,sums[j],r)+r-problem.correction_moments[j])%r;}if sums[0]>problem.inner.l as u64{return Err(Error::Arithmetic)}Ok(sums)
 })();match result{Ok(v)=>Ok((v,stats)),Err(e)=>Err((e,stats))}
}

fn recover_selected_trace(source:&source::Source,curve:group::Curve,base:group::Point,target:Option<group::Point>,r:u64,m:usize,zeta:u64,radii:&[u64],max_calls:usize,node_cap:usize,caps:StreamCaps,correction_cap:usize)->Result<StreamRecovery,Error>{
 if !valid_domain(r,m,zeta)||radii.is_empty()||radii.len()>4||radii.iter().any(|&b|b==0||b>=r)||max_calls==0{return Err(Error::Domain)}
 let correction=std::cell::Cell::new(CorrectionStats::default());let stream=std::cell::Cell::new(StreamStats::default());
 let finish=|scalar,calls,route,pairs|{if let Some(k)=scalar{if multiple(curve,base,k)!=target{return Err(Error::Arithmetic)}}Ok(StreamRecovery{result:Recovery{scalar,calls,route,pairs},correction:correction.get(),stream:stream.get()})};
 let encode=|s:i8|if s<0{r-1}else{s as u64};let epsilon=encode(source.sign(target.map(|p|[p.x,p.y])).map_err(|_|Error::Arithmetic)?);let mut calls=1;if epsilon==0{return finish(Some(0),calls,"endpoint",0)}let mut words=vec![];
 for &b in radii{let mut z=1;let mut signs=vec![];for _ in 0..m{if calls>=max_calls{return finish(None,calls,"call-cap",0)}let off=mul(b,z,r);let pt=curve.plus(target,multiple(curve,base,(r-off)%r));let s=encode(source.sign(pt.map(|p|[p.x,p.y])).map_err(|_|Error::Arithmetic)?);calls+=1;if s==0{return finish(Some(off),calls,"endpoint",0)}signs.push(s);z=mul(z,zeta,r)}words.push(Word{b,signs})}
 let problem=match CorrectedProblem::build(r,m,zeta,epsilon,&words,node_cap,correction_cap){Ok(x)=>x,Err(Error::Limit)=>return finish(None,calls,"construction-cap",0),Err(e)=>return Err(e)};
 correction.set(problem.correction);
 let(moment,stats)=match selected_word_moments(&problem,zeta,&words,caps){Ok(x)=>x,Err((Error::Limit,s))=>{stream.set(s);return finish(None,calls,"stream-cap",0)},Err((e,_))=>return Err(e)};
 stream.set(stats);
 if moment[0]==0{return Err(Error::Arithmetic)}if moment[0]==1{if let Some(k)=problem.phase_at(moment[1])?{return finish(Some(k),calls,"selected-trace-singleton",0)}}finish(None,calls,"unresolved",0)
}

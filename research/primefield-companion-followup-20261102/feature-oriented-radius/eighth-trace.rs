std::thread_local!{static EIGHTH_SEARCH:std::cell::Cell<(usize,usize,usize)>=const{std::cell::Cell::new((0,0,0))};static NEGATIVE_SPLITS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};}
fn eighth_trace(r:u64,trials:usize)->Result<Option<u64>,Error>{
 if r<7||r>u64::MAX/2||trials>64{return Err(Error::Domain)}
 EIGHTH_SEARCH.with(|c|{let(a,b,d)=c.get();c.set((a+1,b,d))});
 if r%8!=1{return Ok(None)}
 for z in 2..r.min(2+trials as u64){EIGHTH_SEARCH.with(|c|{let(a,b,d)=c.get();c.set((a,b+1,d))});let eta=pow(z,(r-1)/8,r);let eta2=mul(eta,eta,r);if mul(eta2,eta2,r)!=r-1{continue}
  let eta3=mul(eta2,eta,r);let s=(eta+r-eta3)%r;if mul(s,s,r)!=2{return Err(Error::Arithmetic)}
  EIGHTH_SEARCH.with(|c|{let(a,b,d)=c.get();c.set((a,b,d+1))});return Ok(Some(s))
 }Ok(None)
}
fn split_negative_domains(r:u64,n:usize,i:u64,s:u64)->Result<Vec<(u64,Vec<u64>)>,Error>{
 if r<7||r>u64::MAX/2||n==0||n%2!=0||n as u64>=r||i>=r||mul(i,i,r)!=r-1||s>=r||mul(s,s,r)!=2{return Err(Error::Domain)}
 let mut p=polynomial::Poly::new(r).map_err(|_|Error::Domain)?;let dt=initial_dickson(&mut p,n/2);
 let mut minus=dt.clone();minus[0]=(minus[0]+r-s)%r;let mut plus=dt;plus[0]=(plus[0]+s)%r;
 NEGATIVE_SPLITS.with(|c|c.set(c.get()+1));Ok(vec![(i,minus.clone()),(i,plus.clone()),(r-i,minus),(r-i,plus)])
}

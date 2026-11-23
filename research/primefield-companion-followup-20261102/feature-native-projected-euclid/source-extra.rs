fn map_transport(e:transport::Error)->Error{match e{transport::Error::Limit=>Error::Limit,_=>Error::Transport}}
pub struct Cursor<'b,'a>{prepared:&'b Prepared<'a>,transport:transport::Cursor<'b,'a>,indices:Vec<u64>}
#[derive(Debug)]pub struct Extension{pub sign:Option<i8>,pub ratio:Option<Pair>,pub point:Point,pub method:&'static str,pub pairs:Vec<Pair>,pub converted_points:usize,pub lane_updates:usize,pub conversion_batches:usize}
impl Source{
 fn calibrated(&self,ratio:Pair)->Result<i8,Error>{let left=mul(ratio[0],self.calibration[1],self.p);let right=mul(ratio[1],self.calibration[0],self.p);if left==right{Ok(1)}else if left==self.p-right{Ok(-1)}else{Err(Error::Character)}}
}
impl<'a> Prepared<'a>{
 pub fn window(&self,q:Point,length:usize)->Result<Window,Error>{self.begin(q,length,0).map(|(w,_)|w)}
 pub fn begin<'b>(&'b self,q:Point,length:usize,max_extra:usize)->Result<(Window,Cursor<'b,'a>),Error>{
  let(w,cursor)=self.transport.begin(q,length,max_extra).map_err(map_transport)?;let indices=w.indices.clone();let initial=self.consume(w)?;
  Ok((initial,Cursor{prepared:self,transport:cursor,indices}))
 }
 fn consume(&self,w:transport::Window)->Result<Window,Error>{
  let s=self.source;let mut signs=Vec::with_capacity(w.points.len());let mut ratios=Vec::with_capacity(w.points.len());
  for(i,point)in w.points.iter().enumerate(){if let Some(point)=point{let ratio=s.ratio(*point,&w.grids[i],&w.indices,&w.pairs[i])?;signs.push(s.calibrated(ratio)?);ratios.push(ratio);}else{ratios.push([0,1]);signs.push(0);}}
  Ok(Window{points:w.points,signs,ratios,direct_initializations:w.direct_initializations,converted_points:w.converted_points})
 }
}
impl<'b,'a> Cursor<'b,'a>{
 pub fn advance(self)->Result<(Extension,Self),Error>{self.step(true)}
 pub fn skip_sign(self)->Result<(Extension,Self),Error>{self.step(false)}
 fn step(self,evaluate:bool)->Result<(Extension,Self),Error>{
  let(e,next)=self.transport.advance().map_err(map_transport)?;let s=self.prepared.source;
  let(ratio,sign)=if evaluate{if let Some(q)=e.point{let ratio=s.ratio(q,&e.grid,&self.indices,&e.pairs)?;(Some(ratio),Some(s.calibrated(ratio)?))}else{(Some([0,1]),Some(0))}}else{(None,None)};
  let extension=Extension{ratio,sign,point:e.point,method:e.method,pairs:e.pairs,converted_points:e.converted_points,lane_updates:e.lane_updates,conversion_batches:e.conversion_batches};
  Ok((extension,Self{prepared:self.prepared,transport:next,indices:self.indices}))
 }
}
#[cfg(test)]thread_local!{static SIGN_CALLS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};}
#[cfg(test)]pub fn calls()->(usize,usize,usize){let(a,b)=transport::calls();(SIGN_CALLS.with(|n|n.get()),a,b)}

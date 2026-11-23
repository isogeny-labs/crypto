pub struct Cursor<'b,'a>{prepared:&'b Prepared<'a>,tail:geometry::Tail<'b,'a>,count:usize,previous:Vec<Pair>,older:Vec<Pair>,middle_x:Vec<Option<u64>>,remaining:usize}
pub struct Extension{pub point:Point,pub grid:Vec<[u64;2]>,pub pairs:Vec<Pair>,pub method:&'static str,pub converted_points:usize,pub lane_updates:usize,pub conversion_batches:usize}
impl<'b,'a> Cursor<'b,'a>{
 pub fn advance(self)->Result<(Extension,Self),Error>{let _accounting_scope=crate::meter::granular(crate::meter::Stage::ExtraTransport);
  if self.remaining==0{return Err(Error::Limit);}let(next,tail)={let _accounting_scope=crate::meter::granular(crate::meter::Stage::ExtraGeometry);self.tail.advance().map_err(|_|Error::Geometry)?};
  let s=self.prepared.source;let point=next.point;
  let(pairs,method)=if let Some((_,v))=s.anchors.iter().find(|(p,_)|*p==point){(v.clone(),"anchor")}else if self.count<2{let q=point.ok_or(Error::Geometry)?;let c=s.phase(q)?;(s.values(q,c)?,"direct")}else{
   let mut pairs=Vec::with_capacity(s.indices.len());for j in 0..s.indices.len(){let x=self.middle_x[j].ok_or(Error::Geometry)?;pairs.push(step(s.p,(s.base_x[j]+s.p-x)%s.p,s.squares[j],self.previous[j],self.older[j])?);}(pairs,"recurrence")
  };
  let mut middle_x=vec![None;s.indices.len()];if let Some(q)=point{for(j,&lane)in s.lanes.iter().enumerate(){middle_x[j]=Some(if lane==usize::MAX{q[0]}else{next.grid.get(lane).ok_or(Error::Geometry)?[0]});}}
  let extension=Extension{point,grid:next.grid,pairs:pairs.clone(),method,converted_points:next.converted_points,lane_updates:next.lane_updates,conversion_batches:next.batches};
  Ok((extension,Self{prepared:self.prepared,tail,count:self.count.checked_add(1).ok_or(Error::Limit)?,previous:pairs,older:self.previous,middle_x,remaining:self.remaining-1}))
 }
}

//! Shared-geometry projected sign windows. Native prime-field arithmetic only.
include!("source.rs");
pub struct PreparedWindow<'a>{source:&'a Source,geometry:geometry::Prepared<'a>,max_window:usize}
#[derive(Debug)]pub struct Signs{pub values:Vec<i8>,pub shifts:Vec<Option<[u64;2]>>,pub conversion_batches:usize,pub converted_points:usize}
impl Source{
 /// Caps window length and flattened points before any window allocation.
 pub fn prepare_window(&self,max_window:usize,max_points:usize)->Result<PreparedWindow<'_>,Error>{
  let[w,k,s,e]=self.part;let lanes=1+w as u128+k as u128+u128::from(s!=0);
  if max_window==0||(max_window as u128)*(lanes+1)>max_points as u128{return Err(Error::Limit)}
  let geometry=self.curve.prepare(self.r,self.base,w,k,s,e).map_err(|_|Error::Geometry)?;
  Ok(PreparedWindow{source:self,geometry,max_window})
 }
}
impl PreparedWindow<'_>{
 pub fn signs(&self,target:Option<[u64;2]>,length:usize)->Result<Signs,Error>{
  if length==0||length>self.max_window{return Err(Error::Limit)}
  // Validate torsion before constructing any lanes from an untrusted target.
  if let Some([x,y])=target{let mut d=division::ValueDivision::new(self.source.p,self.source.a,self.source.b,x,y).map_err(|_|Error::Point)?;if d.value(self.source.r)!=0{return Err(Error::NonTorsion)}}
  let joint=self.geometry.joint_window(target,length).map_err(|_|Error::Geometry)?;
  let mut values=Vec::with_capacity(length);let mut cursor:usize=0;let lanes=joint.geometry.labels.len();
  for(offset,point)in joint.shifts.iter().enumerate(){
   let t=match point{None=>{values.push(0);continue},Some(t)=>*t};
   let end=cursor.checked_add(lanes).ok_or(Error::Geometry)?;
   let slots=joint.geometry.slots.get(cursor..end).ok_or(Error::Geometry)?;
   let mut affine=Vec::with_capacity(lanes);
   for(lane,slot)in slots.iter().enumerate(){if slot.offset!=offset||slot.lane!=lane||slot.label!=joint.geometry.labels[lane]{return Err(Error::Geometry)}affine.push(slot.affine)}
   cursor=end;let raw=self.source.raw_on_grid(t,&affine)?;
   values.push(if raw==self.source.calibration{1}else if raw==self.source.p-self.source.calibration{-1}else{return Err(Error::Character)});
  }
  if cursor!=joint.geometry.slots.len(){return Err(Error::Geometry)}
  Ok(Signs{values,shifts:joint.shifts,conversion_batches:joint.geometry.conversion_batches,converted_points:joint.converted_points})
 }
}
#[cfg(test)]#[path="../feature-orbit-local-terms/local.rs"]mod ec;
#[cfg(test)]mod window_tests{
 use super::*;
 #[test]fn complete_small_and_frozen_windows(){
  let mut windows=0;let mut signs=0;
  for(p,a,b,r,base)in[(5,2,1,7,[0,1]),(7,0,3,13,[1,2]),(509,2,10,479,[3,68])]{
   let src=Source::new(p,a,b,r,base,64).unwrap();let prepared=src.prepare_window(16,1024).unwrap();let curve=ec::Curve::new(p,a,b);let bp=Some(ec::Point{x:base[0],y:base[1]});
   let starts:Vec<u64>=if r<20{(0..r).collect()}else{vec![0,1,2,478,85,381,26,174,47,419,217,249,25,173,293,441,121,345,292,440]};
   for k in starts{let mut t=None;for _ in 0..k{t=curve.plus(t,bp)}let pair=|v:Option<ec::Point>|v.map(|p|[p.x,p.y]);
    let out=prepared.signs(pair(t),16).unwrap();let mut finite=0;
    for j in 0..16{assert_eq!(out.shifts[j],pair(t));assert_eq!(out.values[j],src.sign(pair(t)).unwrap());let n=(k+j as u64)%r;let want=if n==0{0}else if pow(n,(r-1)/2,r)==1{1}else{-1};assert_eq!(out.values[j],want);finite+=usize::from(t.is_some());t=curve.plus(t,bp);signs+=1}
    assert_eq!(out.conversion_batches,1);assert_eq!(out.converted_points,finite*(1+out_geometry_lanes(&src)));windows+=1;
   }
  }assert_eq!((windows,signs),(40,640));
 }
 fn out_geometry_lanes(s:&Source)->usize{let[w,k,t,_]=s.part;(1+w+k+u64::from(t!=0))as usize}
 #[test]fn limits_and_identity(){let s=Source::new(509,2,10,479,[3,68],64).unwrap();assert!(matches!(s.prepare_window(16,10),Err(Error::Limit)));assert!(matches!(s.prepare_window(0,1024),Err(Error::Limit)));let p=s.prepare_window(16,1024).unwrap();assert!(matches!(p.signs(None,17),Err(Error::Limit)));assert!(matches!(p.signs(None,0),Err(Error::Limit)));assert!(matches!(p.signs(Some([509,68]),1),Err(Error::Point)));assert!(matches!(p.signs(Some([0,0]),1),Err(Error::Point)));let z=p.signs(None,1).unwrap();assert_eq!(z.values,vec![0]);assert_eq!(z.conversion_batches,0);assert_eq!(z.converted_points,0);}
}

pub mod recovery;

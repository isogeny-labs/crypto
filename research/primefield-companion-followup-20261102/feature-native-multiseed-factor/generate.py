"""Static source adaptation; no field execution. Run from repository root."""
from pathlib import Path
import hashlib,json,sys
r=Path('research/primefield-companion-followup-20261102');d=r/'feature-native-multiseed-factor'
transport=r/'feature-native-normalized-transport/lib.rs';source=r/'feature-native-quartic-recovery/routed/source.rs'
def emit(name,content):
 if '--check' in sys.argv:
  assert (d/name).read_text()==content,name
 else:
  (d/name).write_text(content)
def once(s,a,b):
 assert s.count(a)==1,(a,s.count(a))
 return s.replace(a,b)
s=transport.read_text().split('#[cfg(test)]mod tests{')[0]
s=once(s,'let base_x=lanes.iter().map(|&i|grid.affine[i][0]).collect();','let mut base_x:Vec<u64>=lanes.iter().map(|&i|grid.affine[i][0]).collect();indices.push(1);lanes.push(usize::MAX);base_x.push(base[0]);')
s=once(s,'pub struct Window{pub points:','pub struct Window{pub grids:Vec<Vec<[u64;2]>>,pub points:')
s=once(s,'let mut out=Window{points:joint.shifts,','let lanes=joint.geometry.labels.len();let mut grids=vec![vec![[0,0];lanes];length];let mut seen=vec![0usize;length];for slot in &joint.geometry.slots{grids[slot.offset][slot.lane]=slot.affine;seen[slot.offset]+=1;}for i in 0..length{if let Some(point)=joint.shifts[i]{if seen[i]!=lanes{return Err(Error::Geometry);}x[i][s.indices.len()-1]=Some(point[0]);}}\n  let mut out=Window{grids,points:joint.shifts,')
s=once(s,'#[cfg(test)]fn calls()','#[cfg(test)]pub fn calls()')
emit('transport.rs',s)
s=source.read_text();start=s.index('  let update=',s.index(' fn raw_grid_seed_parts('));end=s.index('\n }\n fn unit_grid_parts',start);body=s[start:end]
body=once(body,'let c=(p-scaled(s+e))%p;','let c=(p-tail)%p;')
body=once(body,'  Ok(RawParts{seed,ratio:[numerator,denominator],resultant,recurrence_updates:updates,partial_value})','  Ok([numerator,denominator])')
body=body.replace('let mut updates=0;','').replace('updates+=1;','').replace('let mut partial_value=None;','').replace('partial_value=Some(v);','')
head='  let [w,k,s,e]=part;\n  if grid.len()!=1+(w+k)as usize+usize::from(s!=0){return Err(Error::Geometry);}\n'
geo='pub(crate) fn geometry_ratio(p:u64,part:[u64;4],q:[u64;2],grid:&[[u64;2]])->Result<Pair,Error>{\n'+head+'  let(u,seed_sq,step_sq,tail)=(1,1,1,1);\n'+body+'\n}\n'
ref='#[cfg(test)] pub(crate) fn reference_ratio(p:u64,part:[u64;4],q:[u64;2],grid:&[[u64;2]],inputs:[u64;4])->Result<Pair,Error>{\n'+head+'  let [seed,u,step,tail]=inputs;let seed_sq=mul(seed,seed,p);let step_sq=mul(step,step,p);\n'+body+'\n}\n'
emit('residual.rs','use super::{mul,pow,Error,Pair};\n#[path="../feature-native-low-product/polynomial.rs"]mod polynomial;\n'+geo+ref)
emit('generation.json',json.dumps(dict(inputs={str(p.relative_to(r)):hashlib.sha256(p.read_bytes()).hexdigest()for p in[transport,source]},outputs={name:hashlib.sha256((d/name).read_bytes()).hexdigest()for name in['transport.rs','residual.rs']},changes=['retain production transport; append seed n=1 using existing public shifts; retain full grids from same geometry window','extract frozen scalar residual body; specialize four inputs to one for production; scalar-input reference is test-only']),indent=2)+'\n')

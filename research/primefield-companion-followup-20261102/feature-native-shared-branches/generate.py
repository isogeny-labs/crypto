from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-reused-powers';pins={};outs=[]
for f in sorted(src.iterdir()):
 if f.suffix!='.rs' and f.name!='validate-recovery.py':continue
 t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if f.name=='reused.rs':t+='\ninclude!("shared.rs");\n'
 if f.name=='new-recovery.rs':t=t.replace('fn main(){','fn main(){shared_checks();',1)
 (p/f.name).write_text(t);outs.append(f.name)
t=(src/'reused-image.rs').read_text().replace('ReusedImage','SharedImage').replace('REUSED_COUNTS','SHARED_COUNTS')
a=t.index('  for branch in &mut self.branches{if branch.ring.g.len()==1{continue}');b=t.index('self.rounds+=1;Ok(())',a)
t=t[:a]+"""  if !first{
   let indices:Vec<_>=self.branches.iter().enumerate().filter(|(_,b)|b.ring.g.len()>1).map(|(j,_)|j).collect();let mut rings:Vec<_>=indices.iter().map(|&j|ReusedRing::new(&self.branches[j].canonical)).collect::<Result<_,_>>()?;
   let values=shared_mode(&mut rings,modes,0,scale)?;for ((j,ring),value) in indices.into_iter().zip(rings.iter()).zip(values){let mut eq=ring.scale(&value,a);ring.subtract_constant(&mut eq,f[0]);self.branches[j].keep_split(eq)?;}
  }
  let indices:Vec<_>=self.branches.iter().enumerate().filter(|(_,b)|b.ring.g.len()>1).map(|(j,_)|j).collect();let mut rings:Vec<_>=indices.iter().map(|&j|ReusedRing::new(&self.branches[j].canonical)).collect::<Result<_,_>>()?;
  let f1=shared_mode(&mut rings,modes,1,scale)?;let f2=shared_mode(&mut rings,modes,2,scale)?;
  for (((j,ring),v1),v2) in indices.into_iter().zip(rings.iter_mut()).zip(f1).zip(f2){let sq=ring.multiply(&v1,&v1)?;let l=ring.scale(&v2,left);let rr=ring.scale(&sq,right);self.branches[j].keep_split(ring.sub(&l,&rr))?;}
  """+t[b:]
(p/'shared-image.rs').write_text(t);outs.append('shared-image.rs')
t=(src/'reused-adapter.rs').read_text().replace('recover_reused','recover_shared').replace('ReusedImage','SharedImage').replace('REUSED_COUNTS','SHARED_COUNTS');(p/'shared-adapter.rs').write_text(t);outs.append('shared-adapter.rs')
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')

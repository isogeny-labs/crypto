from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-shared-branches';pins={};outs=[]
for f in sorted(src.iterdir()):
 if f.suffix!='.rs' and f.name!='validate-recovery.py':continue
 t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if f.name=='shared.rs':t+='\ninclude!("neighbor.rs");\n'
 if f.name=='new-recovery.rs':t=t.replace('fn main(){','fn main(){neighbor_checks();',1)
 (p/f.name).write_text(t);outs.append(f.name)
t=(src/'shared-image.rs').read_text().replace('SharedImage','NeighborImage').replace('SHARED_COUNTS','NEIGHBOR_COUNTS')
old='let f1=shared_mode(&mut rings,modes,1,scale)?;let f2=shared_mode(&mut rings,modes,2,scale)?;'
assert old in t;t=t.replace(old,'let(f1,f2)=neighbor_modes(&mut rings,modes,scale)?;')
(p/'neighbor-image.rs').write_text(t);outs.append('neighbor-image.rs')
t=(src/'shared-adapter.rs').read_text().replace('recover_shared','recover_neighbor').replace('SharedImage','NeighborImage').replace('SHARED_COUNTS','NEIGHBOR_COUNTS');(p/'neighbor-adapter.rs').write_text(t);outs.append('neighbor-adapter.rs')
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')

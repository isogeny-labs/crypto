from pathlib import Path
import json,hashlib
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-implicit-domain';pins={};outs=[]
for f in sorted(src.iterdir()):
 if f.suffix!='.rs' and f.name!='validate-recovery.py':continue
 t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if f.name=='trace-adapter.rs':
  old='for constraint in self.constraints(b,&f){state.intersect(&mut p,&constraint)?;}'
  assert old in t;t=t.replace(old,'self.intersect_modes(&mut p,&mut state,b,&f)?;')
  t='include!("residue-constraints.rs");\n'+t
 if f.name=='new-recovery.rs':t=t.replace('fn main(){implicit_checks();','fn main(){residue_constraint_checks();implicit_checks();')
 (p/f.name).write_text(t);outs.append(f.name)
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')

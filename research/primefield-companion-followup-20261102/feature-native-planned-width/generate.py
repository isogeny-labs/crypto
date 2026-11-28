from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-borrowed-rescue';pins={};outs=[]
for f in sorted(src.iterdir()):
 if (f.suffix!='.rs' and f.name!='validate-recovery.py') or f.name.startswith('validation01-'):continue
 t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if f.name=='shared-rescue.rs':t+='\ninclude!("width-plan.rs");\n'
 if f.name=='new-recovery.rs':t=t.replace('fn main(){','fn main(){width_plan_checks();',1)
 (p/f.name).write_text(t);outs.append(f.name)
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')

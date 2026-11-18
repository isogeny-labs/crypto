from pathlib import Path
import json,hashlib
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-residue-constraints';pins={};outs=[]
for f in sorted(src.iterdir()):
 if f.suffix!='.rs' and f.name!='validate-recovery.py':continue
 t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if f.name=='trace-adapter.rs':
  adapter=t[t.index('impl Modes{'):].replace('impl Modes{','impl StreamModes{',1)
  (p/'stream-adapter.rs').write_text(adapter);outs.append('stream-adapter.rs')
  t+='\ninclude!("streamed.rs");\ninclude!("stream-adapter.rs");\n'
 if f.name=='new-recovery.rs':
  t=t.replace('fn main(){','fn main(){stream_checks();',1)
  t=t.replace('let extra:Vec<_>=','let streamed=StreamModes::new(r,m,z,16,256,7).unwrap();let extra:Vec<_>=')
  t=t.replace('let(new,ns,nroute)=modes.recover_traced','let(new,ns,nroute)=streamed.recover_traced')
  t=t.replace('dyadic_recovery_cases=','streamed_recovery_cases=')
 (p/f.name).write_text(t);outs.append(f.name)
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')

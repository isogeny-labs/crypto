from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-fused-inverses';pins={};outs=[]
for f in sorted(src.iterdir()):
 if f.suffix!='.rs' and f.name!='validate-recovery.py':continue
 t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if f.name=='fused.rs':t+='\ninclude!("direct.rs");\n'
 if f.name=='new-recovery.rs':t=t.replace('fn main(){','fn main(){direct_checks();',1)
 (p/f.name).write_text(t);outs.append(f.name)
t=(src/'fused-adapter.rs').read_text().replace('recover_fused','recover_direct').replace('FusedImage','DirectImage').replace('FUSED_COUNTS','DIRECT_COUNTS');(p/'direct-adapter.rs').write_text(t);outs.append('direct-adapter.rs')
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')

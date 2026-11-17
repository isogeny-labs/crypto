from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-split-modes';pins={};outs=[]
for f in sorted(src.iterdir()):
 if f.suffix!='.rs' and f.name!='validate-recovery.py':continue
 t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if f.name=='split.rs':t+='\ninclude!("reused.rs");\n'
 if f.name=='new-recovery.rs':t=t.replace('fn main(){','fn main(){reused_checks();',1)
 (p/f.name).write_text(t);outs.append(f.name)
for name in ['image','adapter']:
 t=(src/('split-'+name+'.rs')).read_text().replace('SplitImage','ReusedImage').replace('SPLIT_COUNTS','REUSED_COUNTS').replace('SplitRing','ReusedRing').replace('recover_split','recover_reused')
 dest='reused-'+name+'.rs';(p/dest).write_text(t);outs.append(dest)
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')

from pathlib import Path
import json,hashlib
p=Path(__file__).resolve().parent;src=p.parent/'feature-workspace-adaptive-projection';pins={}
for f in sorted(src.glob('*.rs')):
 s=f.read_text();pins[f.name]=hashlib.sha256(s.encode()).hexdigest()
 if f.name=='modes.rs':s=s.replace('../feature-native-standalone-source/lib.rs','lib.rs')
 if f.name=='trace.rs':s=s.replace('fn main(){','fn previous_main(){')+'\ninclude!("new-recovery.rs");\n'
 (p/f.name).write_text(s)
(p/'recovery-generation.json').write_text(json.dumps({'source':str(src),'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in pins}},indent=2)+'\n')

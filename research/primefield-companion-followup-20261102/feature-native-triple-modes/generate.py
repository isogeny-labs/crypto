from pathlib import Path
import hashlib,json
D=Path(__file__).resolve().parent;S=D.parent/'feature-native-terminal-arithmetic';rows=[]
for f in sorted(S.glob('*.rs')):
 old=f.read_text();t=old;edits=[]
 if f.name=='terminal-checks.rs':
  a='fn main(){';b='fn terminal_predecessor_main(){';assert t.count(a)==1;t=t.replace(a,b);edits.append([a,b]);suffix='\ninclude!("triple.rs");\ninclude!("triple-checks.rs");\n';t+=suffix;edits.append(['',suffix])
 (D/f.name).write_text(t);rows.append({'source':'../feature-native-terminal-arithmetic/'+f.name,'output':f.name,'source_sha256':hashlib.sha256(old.encode()).hexdigest(),'sha256':hashlib.sha256(t.encode()).hexdigest(),'edits':edits})
f=S/'direct-adapter.rs';old=f.read_text();t=old.replace('recover_direct','recover_triple').replace('image.update(self,b,&f)?','image.update_triple(self,b,&f)?');assert t!=old
(D/'triple-adapter.rs').write_text(t);rows.append({'source':'../feature-native-terminal-arithmetic/direct-adapter.rs','output':'triple-adapter.rs','source_sha256':hashlib.sha256(old.encode()).hexdigest(),'sha256':hashlib.sha256(t.encode()).hexdigest(),'edits':[['recover_direct','recover_triple'],['image.update(self,b,&f)?','image.update_triple(self,b,&f)?']]})
(D/'generation.json').write_text(json.dumps(rows,indent=2)+'\n')

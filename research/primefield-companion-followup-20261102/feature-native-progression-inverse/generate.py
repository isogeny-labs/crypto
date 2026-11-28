from pathlib import Path
import json,hashlib
D=Path(__file__).resolve().parent;S=D.parent/'feature-native-work-accounting';rows=[]
for f in sorted(S.glob('*.rs')):
 t=f.read_text();old=t;edits=[]
 def sub(a,b):
  global t
  assert t.count(a)==1,(f.name,a,t.count(a));t=t.replace(a,b);edits.append([a,b])
 if f.name=='progression.rs':
  sub('let d=f.len()-1;let inv_step=pow(step,r-2,r);','let d=f.len()-1;let mut factorial=1;for i in 1..=d{factorial=mul(factorial,i as u64,r)}\n  let joint=mul(step,factorial,r);let inverse_joint=pow(joint,r-2,r);let inv_step=mul(factorial,inverse_joint,r);')
  sub('let mut factorial=1;for i in 1..=d{factorial=mul(factorial,i as u64,r)}let mut invfact=vec![0;d+1];invfact[d]=pow(factorial,r-2,r);','let mut invfact=vec![0;d+1];invfact[d]=mul(step,inverse_joint,r);')
 if f.name=='meter-checks.rs':
  sub('fn main(){','fn predecessor_main(){')
  t+='\ninclude!("inverse-checks.rs");\n';edits.append(['','\ninclude!("inverse-checks.rs");\n'])
 (D/f.name).write_text(t);rows.append({'source':'../feature-native-work-accounting/'+f.name,'output':f.name,'source_sha256':hashlib.sha256(old.encode()).hexdigest(),'sha256':hashlib.sha256(t.encode()).hexdigest(),'edits':edits})
(D/'generation.json').write_text(json.dumps(rows,indent=2)+'\n')

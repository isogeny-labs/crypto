from pathlib import Path
import hashlib,json,re
D=Path(__file__).resolve().parent;S=D.parent/'feature-native-work-accounting';rows=[]
for name,out in [('modes.rs','main.rs'),('meter-checks.rs','instrument-checks.rs')]:
 original=(S/name).read_text();t=original;edits=[]
 def sub(a,b):
  global t
  assert t.count(a)==1,(a,t.count(a));t=t.replace(a,b);edits.append([a,b])
 if name=='modes.rs':
  for x in ['metered-polynomial.rs','lib.rs']:sub('path="'+x+'"','path="../feature-native-work-accounting/'+x+'"')
  sub('mod meter;','#[path="../feature-native-work-accounting/meter.rs"] mod meter;')
  sub('include!("meter-checks.rs");','include!("instrument-checks.rs");')
  for x in ['reference.rs','orbit.rs']:sub('include!("'+x+'");','include!("../feature-native-work-accounting/'+x+'");')
  t+='\ninclude!("panel.rs");\n';edits.append(['','\ninclude!("panel.rs");\n'])
 else:sub('fn main(){','fn instrument_main(){')
 (D/out).write_text(t);rows.append({'source':'../feature-native-work-accounting/'+name,'output':out,'source_sha256':hashlib.sha256(original.encode()).hexdigest(),'sha256':hashlib.sha256(t.encode()).hexdigest(),'edits':edits})
(D/'generation.json').write_text(json.dumps(rows,indent=2)+'\n')

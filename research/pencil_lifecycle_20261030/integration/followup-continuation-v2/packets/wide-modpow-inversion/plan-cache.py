from pathlib import Path
import re,json
p=Path(__file__).parent;base=Path('/var/tmp/pencil-sameu-global-smooth-20261031-v1/dependency-union');src=Path('/home/ec2-user/crypto-pr2304-persist/research/projective_law_followup_20261022/formal/.lake/packages/mathlib');cache=src/'.lake/build/lib/lean';seen=set();rows=[]
def clean(s):
 out=[];i=0;depth=0
 while i<len(s):
  if s[i:i+2]=='/-':depth+=1;i+=2;continue
  if depth and s[i:i+2]=='-/':depth-=1;i+=2;continue
  if not depth and s[i:i+2]=='--':
   j=s.find('\n',i);i=len(s) if j<0 else j;continue
  if not depth or s[i]=='\n':out.append(s[i])
  i+=1
 return ''.join(out)
def visit(mod):
 rel=Path(*mod.split('.'))
 if (base/rel.with_suffix('.olean')).exists() or mod in seen:return
 seen.add(mod);f=src/rel.with_suffix('.lean');s=clean(f.read_text())
 for dep in re.findall(r'^(?:public |private )?import (\S+)',s,re.M):visit(dep)
 fs=[f]+list((cache/rel.parent).glob(rel.name+'.*'));assert any(q.suffix=='.olean' for q in fs),mod
 for q in fs:rows.append({'source':str(q),'bytes':q.stat().st_size,'module':mod,'source_text':q==f})
visit('Mathlib.FieldTheory.Finite.Basic');j={'operation':'copy existing cached modules into private union; no download or dependency build','modules':sorted(seen),'files':rows,'bytes':sum(r['bytes'] for r in rows)};(p/'cache-plan.json').write_text(json.dumps(j,indent=2)+'\n');print(len(seen),len(rows),j['bytes'])

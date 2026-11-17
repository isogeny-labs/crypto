from pathlib import Path
import re,json,hashlib,shutil
D=Path(__file__).parent;src=Path('/home/ec2-user/crypto-pr2304-persist/research/projective_law_followup_20261022/formal/.lake/packages/mathlib');cache=src/'.lake/build/lib/lean';base=Path('/var/tmp/pencil-sameu-global-smooth-20261031-v1/dependency-union');out=D/'dependency-union';seen=set();rows=[]
def visit(mod):
 rel=Path(*mod.split('.'))
 if (base/rel.with_suffix('.olean')).exists() or mod in seen:return
 seen.add(mod);f=src/rel.with_suffix('.lean');s=f.read_text()
 for dep in re.findall(r'^(?:public |private )?import (\S+)',s,re.M):visit(dep)
 files=[f]+list((cache/rel.parent).glob(rel.name+'.*'));assert any(p.suffix=='.olean' for p in files)
 for p in files:
  target=(D/'cache-sources'/p.relative_to(src)) if p==f else out/p.relative_to(cache)
  target.parent.mkdir(parents=True,exist_ok=True);data=p.read_bytes();target.write_bytes(data);rows.append({'source':str(p),'copied':str(target.relative_to(D)),'bytes':len(data),'sha256':hashlib.sha256(data).hexdigest()})
visit('Mathlib.Algebra.Polynomial.Degree.Lemmas')
(D/'cache-custody.json').write_text(json.dumps({'operation':'copy existing pinned dependency cache, no build or download','modules':sorted(seen),'files':rows},indent=2)+'\n');print(len(seen),len(rows))

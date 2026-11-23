from pathlib import Path
import json,re,sys
D=Path(__file__).parent;R=D/'invocation-3v1otqmz';bad=D/'invocation-f4vt0baj'
r=json.loads((R/'receipt.json').read_text());f=json.loads((bad/'receipt.json').read_text())
assert r['gate_exit']==0 and r['bindings_unchanged'] and [m['module'] for m in r['modules']]==['Find','Cycles','Union','Forest','Composition','PriorControls','ForestControls','Readback'] and all(m['native_exit']==0 for m in r['modules'])
expected=re.findall(r'#print axioms (\S+)',(R/'snapshot/Readback.lean').read_text())
found=re.findall(r"'([^']+)' depends on axioms: \[([^\]]*)\]",(R/'Readback.stdout').read_text())
assert [n for n,a in found]==expected and len(found)==19
assert all(set(a.split(', ')) <= {'propext','Classical.choice','Quot.sound'} for n,a in found)
assert f['gate_exit']==1 and f['modules'][0]['native_exit']==1
out={'accepted':True,'main':10,'controls':9,'readbacks':19,'native_success':str(R),'known_false':str(bad),'allowed_axioms_only':True,'source_bindings_unchanged':True}
(D/'acceptance.json').write_text(json.dumps(out,indent=2));print(json.dumps(out))

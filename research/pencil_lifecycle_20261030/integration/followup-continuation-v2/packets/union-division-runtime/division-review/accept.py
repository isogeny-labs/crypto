from pathlib import Path
import json,re
D=Path(__file__).parent;R=D/'invocation-wmxfcxdv';F=D/'invocation-m6rlssmu'
r=json.loads((R/'receipt.json').read_text());f=json.loads((F/'receipt.json').read_text())
expected_modules=['QuotientWrite','DivisionLoop','ExactDivision','DivisionInterface','DivisionControls','DivisionReadback','ReviewerControls','ReviewerReadback']
assert r['gate_exit']==0 and r['bindings_unchanged'] and [m['module'] for m in r['modules']]==expected_modules and all(m['native_exit']==0 for m in r['modules'])
allfound=[]
for n in ['DivisionReadback','ReviewerReadback']:
 expected=re.findall(r'#print axioms (\S+)',(R/'snapshot'/(n+'.lean')).read_text());found=re.findall(r"'([^']+)' depends on axioms: \[([^\]]*)\]",(R/(n+'.stdout')).read_text());assert [a for a,b in found]==expected
 assert all(set(b.split(', ')) <= {'propext','Classical.choice','Quot.sound'} for a,b in found);allfound+=found
assert len(allfound)==32 and f['gate_exit']==1 and f['modules'][0]['native_exit']==1
(D/'acceptance.json').write_text(json.dumps({'accepted':True,'main':17,'producer_controls':9,'reviewer_controls':6,'readbacks':32,'axioms':allfound,'successful_invocation':str(R),'known_false':str(F)},indent=2));print('accepted 32 readbacks')

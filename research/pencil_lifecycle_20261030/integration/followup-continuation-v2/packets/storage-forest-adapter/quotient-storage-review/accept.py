from pathlib import Path
import json,re
D=Path(__file__).parent;R=D/'invocation-pyojnbw9';F=D/'invocation-mrx_tv3s'
r=json.loads((R/'receipt.json').read_text());f=json.loads((F/'receipt.json').read_text())
expected_modules=['QuotientStorage','StoredLoop','StoredDivision','StorageEndpoint','StorageControls','StorageReadback','ReviewerControls','ReviewerReadback']
assert r['gate_exit']==0 and r['bindings_unchanged'] and [m['module'] for m in r['modules']]==expected_modules and all(m['native_exit']==0 for m in r['modules'])
allfound=[]
for n in ['StorageReadback','ReviewerReadback']:
 expected=re.findall(r'#print axioms (\S+)',(R/'snapshot'/(n+'.lean')).read_text());found=re.findall(r"'([^']+)' depends on axioms: \[([^\]]*)\]",(R/(n+'.stdout')).read_text());assert [a for a,b in found]==expected
 assert all(set(b.split(', ')) <= {'propext','Classical.choice','Quot.sound'} for a,b in found);allfound+=found
assert len(allfound)==36 and f['gate_exit']==1 and f['modules'][0]['native_exit']==1
(D/'acceptance.json').write_text(json.dumps({'accepted':True,'main':15,'producer_controls':13,'reviewer_controls':8,'readbacks':36,'axioms':allfound,'successful_invocation':str(R),'known_false':str(F)},indent=2));print('accepted 36 readbacks')

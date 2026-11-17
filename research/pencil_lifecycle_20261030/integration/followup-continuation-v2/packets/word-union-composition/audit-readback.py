from pathlib import Path
import json,re
p=Path(__file__).parent
names=json.loads((p/'readback-names.json').read_text())
def valid(s):
 rows=re.findall(r"'([^']+)' depends on axioms: \[([^\]]*)\]",s)
 return len(rows)==len(names) and [n for n,a in rows]==names and all(set(a.replace(' ','').split(',')) <= {'propext','Classical.choice','Quot.sound'} for n,a in rows)
s=(p/'CompositionReadback-attempt-xvP3ZWUh/stdout').read_text()
assert valid(s)
controls={'empty':not valid(''),'wrong_name':not valid(s.replace(names[0],'Wrong.name')),'sorryAx':not valid(s.replace('propext','sorryAx',1))}
assert all(controls.values())
(p/'readback-audit.json').write_text(json.dumps({'passed':True,'names':names,'negative_controls_rejected':controls},indent=2)+'\n')
print('17 named readbacks checked; three malformed inventories rejected')

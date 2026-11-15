from pathlib import Path
import json,re
p=Path(__file__).parent
names=json.loads((p/'readback-names.json').read_text())
def valid(s):
 rows=[]
 for line in s.splitlines():
  m=re.fullmatch(r"'([^']+)' depends on axioms: \[([^\]]*)\]",line)
  z=re.fullmatch(r"'([^']+)' does not depend on any axioms",line)
  if m:
   axioms=set(m[2].replace(' ','').split(','))
   if not axioms <= {'propext','Classical.choice','Quot.sound'}:return False
   rows.append(m[1])
  elif z:rows.append(z[1])
  elif line.strip():return False
 return rows==names
s=(p/'NodeReadback-attempt-dmjxt4XX/stdout').read_text();assert valid(s)
controls={'empty':not valid(''),'wrong_name':not valid(s.replace(names[0],'Wrong.name')),'sorryAx':not valid(s.replace('propext','sorryAx',1))};assert all(controls.values())
(p/'readback-audit.json').write_text(json.dumps({'passed':True,'names':names,'negative_controls_rejected':controls},indent=2)+'\n')
print('17 readbacks checked; three malformed inventories rejected')

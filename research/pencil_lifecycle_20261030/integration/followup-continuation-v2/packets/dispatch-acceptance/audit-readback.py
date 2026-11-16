import re,json
from pathlib import Path
p=Path(__file__).parent
names=['DispatchAcceptance.every_execution_accepted','DispatchAcceptance.accepted_execution_exists','AcceptanceControls.invalid_identity_output','AcceptanceControls.acceptance_not_canonical','AcceptanceControls.degree_fallback_accepted','AcceptanceControls.norm_fallback_accepted','AcceptanceControls.slope_fallback_accepted']
def valid(s):
 rows=re.findall(r"'([^']+)' depends on axioms: \[([^\]]*)\]",s)
 return len(rows)==len(names) and [n for n,a in rows]==names and all(set(a.replace(' ','').split(',')) <= {'propext','Classical.choice','Quot.sound'} for n,a in rows)
s=(p/'AcceptanceReadback-attempt-6fn9Fors/stdout').read_text();assert valid(s)
controls={'empty':not valid(''),'wrong_name':not valid(s.replace(names[0],'Incorrect.name')),'sorryAx':not valid(s.replace('propext','sorryAx',1))};assert all(controls.values())
(p/'readback-audit.json').write_text(json.dumps({'named_readbacks':names,'passed':True,'negative_controls_rejected':controls},indent=2)+'\n')
print('Seven named readbacks passed; three malformed-inventory controls rejected')

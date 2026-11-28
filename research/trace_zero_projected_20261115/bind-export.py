"""Fill canonical export metadata from observed retained files; no results computed."""
import hashlib,json,sys
from pathlib import Path
p=Path(sys.argv[1]);e=json.loads((p/'export.json').read_text());q=json.loads((p/'protocol.json').read_text())
def entry(rel,kind):
 raw=(p/rel).read_bytes();return {'path':rel,'sha256':hashlib.sha256(raw).hexdigest(),'bytes':len(raw),'kind':kind}
for arm in e['arms']:
 inst=next(a['instrument'] for a in q['arms'] if a['id']==arm['id'])
 arm['commit']=inst['commit'];arm['binary_sha256']=inst['binary_sha256']
e['source']['commit']=q['arms'][0]['instrument']['commit'];e['source']['dirty']=False
analysis=q['analysis_plan']['script'];e['analysis']['script']=entry(analysis['path'],'analysis')
assert e['analysis']['script']['sha256']==analysis['sha256']
e['analysis']['outputs']=[entry('summary.json','summary')]
e['files'].extend([entry(q['extensions']['workload_path'],'raw'),entry('preflight.json','raw'),entry('preflight-gate.json','receipt')])
(p/'export.json').write_text(json.dumps(e,indent=2)+'\n')

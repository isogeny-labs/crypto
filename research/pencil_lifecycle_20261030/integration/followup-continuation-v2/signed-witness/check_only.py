"""Fresh certificate/source readback. Does not import or call elimination."""
from pathlib import Path
import json,hashlib,tarfile,copy
from collections import Counter
from check_certificate import verify
P=Path(__file__).resolve().parent;prior=P.parent/'signed-native-restarts'
h=lambda b:hashlib.sha256(b).hexdigest()
def bound_rows(artifact,raw):
 records=[json.loads(line) for line in raw.splitlines()]
 assert all(r['sequence']==i for i,r in enumerate(records))
 events=[r['event'] for r in records];p=events[0]['r'];n=events[-1]['node_count']-1
 assert artifact['schema']=='signed.target-witness/v1' and artifact['modulus']==p and artifact['nuisance_columns']==n and artifact['source_sha256']==h(raw)
 candidates={e['attempt']:e for e in events if e['kind']=='candidate'};expected=[];origins=[]
 for sequence,e in enumerate(events):
  if e['kind']=='edge':
   row=[sum(sign for node,sign in zip(e['nodes'],e['signs']) if node==j)%p for j in range(1,n+1)]
   row.extend([(-e['b'])%p,(-e['a'])%p])
  elif e['kind']=='identity':
   c=candidates[e['attempt']];row=[0]*n+[(-c['b'])%p,(-c['a'])%p]
  else:continue
  expected.append(row);origins.append({'sequence':sequence,'kind':e['kind'],'attempt':e['attempt']})
 assert artifact['rows']==expected and artifact['row_origins']==origins and artifact['retained_scalar']==events[-1]['recovered']
 assert verify(p,n,expected,artifact['certificate'])
 return True
source_manifest={r['file']:r for r in json.loads((prior/'stream-manifest.json').read_text())['members']}
cert_manifest=json.loads((P/'certificate-manifest.json').read_text())['members'];results=[];examples={};kinds=Counter()
with tarfile.open(prior/'native-streams.tar') as streams,tarfile.open(P/'certificates.tar.gz') as certificates:
 for entry in cert_manifest:
  blob=certificates.extractfile(entry['file']).read();assert h(blob)==entry['sha256'] and len(blob)==entry['bytes'];artifact=json.loads(blob);name=artifact['source_stream']
  raw=streams.extractfile(name).read();assert h(raw)==source_manifest[name]['sha256'];assert bound_rows(artifact,raw)
  kinds.update(r['kind'] for r in artifact['row_origins']);results.append(name);examples.setdefault(artifact['certificate']['kind'],(artifact,raw))
assert len(results)==64
poisons=[]
for mutation in ['scalar','weight','point','null_direction','source_row']:
 original,raw=examples['undetermined' if mutation=='null_direction' else 'determined'];a=copy.deepcopy(original);p=a['modulus'];c=a['certificate']
 if mutation=='scalar':c['scalar']=(c['scalar']+1)%p
 elif mutation=='weight':
  j=next(i for i,row in enumerate(a['rows']) if any(row));c['weights'][j]=(c['weights'][j]+1)%p
 elif mutation=='point':
  j=next(j for j in range(a['nuisance_columns']) if any(row[j] for row in a['rows']));c['point'][j]=(c['point'][j]+1)%p
 elif mutation=='null_direction':c['direction'][-1]=0
 else:a['rows'][0][-1]=(a['rows'][0][-1]+1)%p
 rejected=False
 try:bound_rows(a,raw)
 except AssertionError:rejected=True
 assert rejected;poisons.append({'mutation':mutation,'rejected':True})
assert not verify(7,1,[[0,1,5],[0,0,1]],{'kind':'determined','weights':[1,0],'scalar':2,'point':[0,2]})
poisons.append({'mutation':'valid_isolator_on_inconsistent_system','rejected':True})
print(json.dumps({'schema':'signed.target-witness.readback/v1','certificates_verified':len(results),'row_kinds':dict(kinds),'negative_controls':poisons,'elimination_invoked':False,'collector_invoked':False,'independent_reviewer':False},indent=2))

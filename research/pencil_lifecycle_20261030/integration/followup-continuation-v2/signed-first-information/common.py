from pathlib import Path
import hashlib,json,tarfile,sys
P=Path(__file__).resolve().parent
W=P.parent/'signed-witness';N=P.parent/'signed-native-restarts'
sys.path.insert(0,str(W))
from check_certificate import verify
h=lambda raw:hashlib.sha256(raw).hexdigest()
def load_inputs():
 result={};bindings={}
 for root,name in [(W,'certificates.tar.gz'),(N,'native-streams.tar')]:
  raw=(root/name).read_bytes();b=json.loads((root/'.artifacts.json').read_text())['files'][name]
  assert len(raw)==b['bytes'] and h(raw)==b['sha256'];bindings[str(root.name+'/'+name)]=b
 expected={f'{i:02d}-{arm}.jsonl' for i in range(32) for arm in ['affine','target_rank']}
 cm=json.loads((W/'certificate-manifest.json').read_text())['members'];sm=json.loads((N/'stream-manifest.json').read_text())['members']
 assert len(cm)==64 and len({x['file'] for x in cm})==64
 assert {x['file'].replace('.certificate.json','.jsonl') for x in cm}==expected
 smap={x['file']:x for x in sm};assert len(smap)==len(sm)==192
 with tarfile.open(W/'certificates.tar.gz') as ct,tarfile.open(N/'native-streams.tar') as st:
  assert len(ct.getmembers())==64 and set(ct.getnames())=={x['file'] for x in cm}
  assert len(st.getmembers())==192 and set(st.getnames())==set(smap)
  for member in cm:
   raw=ct.extractfile(member['file']).read();assert len(raw)==member['bytes'] and h(raw)==member['sha256'];d=json.loads(raw);name=d['source_stream']
   assert name in expected and name not in result
   src=st.extractfile(name).read();b=smap[name];assert len(src)==b['bytes'] and h(src)==b['sha256']==d['source_sha256']
   records=[json.loads(line) for line in src.splitlines()];assert [r['sequence'] for r in records]==list(range(len(records)))
   events=[r['event'] for r in records];terminal=events[-1];assert terminal['kind']=='terminal'
   p=events[0]['r'];n=terminal['node_count']-1;assert d['modulus']==p and d['nuisance_columns']==n
   candidates={e['attempt']:e for e in events if e['kind']=='candidate'};rows=[];origins=[]
   for seq,e in enumerate(events):
    if e['kind'] not in ('edge','identity'):continue
    row=[0]*(n+2)
    if e['kind']=='edge':
     for node,sign in zip(e['nodes'],e['signs']):
      assert 0<=node<=n and sign in (-1,1)
      if node:row[node-1]=(row[node-1]+sign)%p
     a,b=e['a'],e['b']
    else:a,b=candidates[e['attempt']]['a'],candidates[e['attempt']]['b']
    row[n:]=[(-b)%p,(-a)%p];rows.append(row);origins.append({'sequence':seq,'kind':e['kind'],'attempt':e['attempt']})
   assert d['rows']==rows and d['row_origins']==origins and d['retained_scalar']==terminal['recovered']
   assert verify(p,n,rows,d['certificate'])
   assert (d['certificate']['kind']=='undetermined' if terminal['recovered'] is None else d['certificate']['kind']=='determined' and d['certificate']['scalar']==terminal['recovered'])
   result[name]={'system':d,'terminal':terminal,'witness_sha256':h(raw)}
 assert set(result)==expected
 return result,bindings

def check_boundary(p,n,rows,first,cert,previous):
 if first is None:return cert['kind']=='undetermined' and previous is None and verify(p,n,rows,cert)
 return (type(first)is int and 1<=first<=len(rows) and cert['kind']=='determined' and previous['kind']=='undetermined' and verify(p,n,rows[:first],cert) and verify(p,n,rows[:first-1],previous))

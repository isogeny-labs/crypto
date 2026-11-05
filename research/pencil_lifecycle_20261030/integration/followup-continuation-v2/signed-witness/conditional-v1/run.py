from pathlib import Path
import json,hashlib,tarfile,tempfile,copy
from collections import Counter
from certificates import solve
from check_certificate import verify
P=Path(__file__).resolve().parent;prior=P.parent/'signed-native-restarts'
h=lambda raw:hashlib.sha256(raw).hexdigest()
sources={str(f):h(f.read_bytes()) for f in [P/'run.py',P/'certificates.py',P/'check_certificate.py',P/'PLAN.md']}
archive=prior/'native-streams.tar';manifest=json.loads((prior/'stream-manifest.json').read_text());expected={r['file']:r for r in manifest['members']}
binding=json.loads((prior/'.artifacts.json').read_text())['files']['native-streams.tar'];assert h(archive.read_bytes())==binding['sha256']
out=Path(tempfile.mkdtemp(prefix='check-',dir=P));(P/'latest-check.json').write_text(json.dumps({'path':str(out)})+'\n');certdir=out/'certificates';certdir.mkdir()
synthetic=[]
for name,rows,n,kind in [('determined',[[1,0,3],[1,1,1]],1,'determined'),('undetermined',[[1,1,3]],1,'undetermined'),('inconsistent',[[0,0,1]],1,'inconsistent'),('empty',[],0,'undetermined')]:
 cert=solve(7,n,rows);assert cert['kind']==kind and verify(7,n,rows,cert)
 bad=copy.deepcopy(cert)
 if kind=='determined':bad['scalar']=(bad['scalar']+1)%7
 elif kind=='undetermined':bad['direction'][-1]=0
 else:bad['weights']=[0]*len(rows)
 assert not verify(7,n,rows,bad);synthetic.append({'name':name,'valid_accepted':True,'corrupted_rejected':True})
records=[]
with tarfile.open(archive) as tar:
 for name in sorted(n for n in expected if n.endswith('.jsonl')):
  raw=tar.extractfile(name).read();assert len(raw)==expected[name]['bytes'] and h(raw)==expected[name]['sha256']
  events=[json.loads(line)['event'] for line in raw.splitlines()];header=events[0];terminal=events[-1];assert terminal['kind']=='terminal';r=header['r'];n=terminal['node_count']-1
  candidates={e['attempt']:e for e in events if e['kind']=='candidate'};rows=[];origins=[]
  for seq,e in enumerate(events):
   if e['kind'] not in ('edge','identity'):continue
   row=[0]*(n+2)
   if e['kind']=='edge':
    for node,sign in zip(e['nodes'],e['signs']):
     assert 0<=node<=n and sign in (-1,1)
     if node:row[node-1]=(row[node-1]+sign)%r
    a,b=e['a'],e['b']
   else:a,b=candidates[e['attempt']]['a'],candidates[e['attempt']]['b']
   row[n]=(-b)%r;row[n+1]=(-a)%r;rows.append(row);origins.append({'sequence':seq,'kind':e['kind'],'attempt':e['attempt']})
  cert=solve(r,n,rows);accepted=verify(r,n,rows,cert);assert accepted
  retained=terminal['recovered'];matches=(cert['kind']=='undetermined' if retained is None else cert['kind']=='determined' and cert['scalar']==retained)
  artifact={'schema':'signed.target-witness/v1','source_stream':name,'source_sha256':h(raw),'modulus':r,'nuisance_columns':n,'convention':'U*x-b*k-a=0','rows':rows,'row_origins':origins,'certificate':cert,'direct_check_pass':accepted,'retained_scalar':retained,'matches_retained':matches}
  (certdir/(name[:-6]+'.certificate.json')).write_text(json.dumps(artifact,separators=(',',':'))+'\n')
  record={'stream':name,'rows':len(rows),'nuisance_columns':n,'kind':cert['kind'],'scalar':cert.get('scalar'),'retained_scalar':retained,'matches_retained':matches,'direct_check_pass':accepted};records.append(record);print(json.dumps(record),flush=True)
assert len(records)==64
summary={'schema':'signed.target-witness.summary/v1','records':records,'counts':dict(Counter(x['kind'] for x in records)),'all_direct_checks_pass':all(x['direct_check_pass'] for x in records),'all_match_retained':all(x['matches_retained'] for x in records),'synthetic_controls':synthetic,'source_hashes':sources,'sources_unchanged':all(h(Path(f).read_bytes())==d for f,d in sources.items()),'input_archive_sha256':binding['sha256'],'collector_invocations':0,'independent_review':'pending'}
(out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n');assert summary['sources_unchanged'] and summary['all_match_retained']

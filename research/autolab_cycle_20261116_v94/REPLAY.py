"""Portable hash, certificate, root-set and raw-evidence replay (no new benchmark)."""
from pathlib import Path
import sys,json,hashlib,tarfile,io,lzma,base64
C=Path(__file__).resolve().parent;ROOT=C.parents[1];sys.path.insert(0,str(ROOT))
index=json.loads((C/'RAW_INDEX.json').read_text());members={}
with tarfile.open(fileobj=io.BytesIO(lzma.decompress(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_text()))))as tf:
 assert set(tf.getnames())==set(index)
 for name,wanted in index.items():
  raw=tf.extractfile(name).read();assert hashlib.sha256(raw).hexdigest()==wanted,name;members[name]=raw
def data(name):return json.loads(members[name])
for name,wanted in json.loads((C/'EVIDENCE_SHA256.json').read_text()).items():
 p=ROOT/name
 if name=='vendor/chain_native_v90/chain-native-baseline'and not p.exists():continue
 assert hashlib.sha256(p.read_bytes()).hexdigest()==wanted,name
from research.degree_block_20261116_v94.verifier import verify,evaluate
from research.source_guards_20261116_v93.kernel import polynomial
from research.twist_auxiliary_20261116_v86.algebra import Field,s3
P='research/degree_block_20261116_v94';D=json.loads((ROOT/P/'DATA.json').read_text());a=data(P+'/oracles_raw.json');assert a['error']is None;a=a['out'];assert len(a['tiny'])==120 and sum(x['root_count']for x in a['tiny'])==188;assert len(a['random_mq'])==80 and a['tamper_rejections']==3
for c in D['cases']:
 record=next(r for r in a['actual']if r['id']==c['id']);poly=polynomial(c['n'],c['mod'],c['d'],0,c['u'],0,0,0,c['B']);rows=[sorted(m for m,v in poly.items()if v>>j&1)for j in range(c['n'])];r=record['certificate'];assert verify(rows,2*c['d'],r)
 if record['roots']is not None:
  expected=[]
  if not r['unsat']:
   for z in range(1<<r['n']):
    if not any(evaluate(p,z)for p in r['polys']):expected.append(sum(evaluate(q,z)<<j for j,q in enumerate(r['maps'])))
  assert sorted(expected)==record['roots']
b=data(P+'/cold_raw.json');assert b['error']is None;b=b['out'];assert len(b['runs'])==36 and b['complete'];assert sum(r['parsed']['native_engine_called']for r in b['runs'])==6
expected={}
for r in b['runs']:
 v=r['parsed'];assert v['complete']and v['error']is None and len(v['cpu_affinity'])==1;expected.setdefault(r['id'],v['roots']);assert expected[r['id']]==v['roots'];assert v['exact_root_count']==len(v['roots'])
O='research/odd_cached_quadratic_20261116_v94b';d=json.loads((ROOT/O/'DATA.json').read_text());o=data(O+'/raw.json');assert o['error']is None;o=o['out'];assert len(o['tiny'])==10 and sum(r['root_count']for r in o['tiny'])==1758
from research.odd_cached_quadratic_20261116_v94b.kernel import solve
for spec in d['tiny']:
 f=Field(spec['p'],spec['mod']);literal=[(x,y,z)for x in spec['domain']for y in spec['domain']for z in range(f.q)if s3(x,y,z,spec['A'],spec['B'],f)==0];assert solve(spec,'cached')['roots']==literal
for spec,row in zip(d['nist'],o['nist']):
 f=Field(spec['p'],spec['mod']);assert spec['id']==row['id'];assert len(row['roots'])==len(set(map(tuple,row['roots'])));expected_hash=hashlib.sha256(json.dumps(row['roots'],separators=(',',':')).encode()).hexdigest()
 for x,y,z in row['roots']:assert x in spec['domain']and y in spec['domain']and 0<=z<f.q and s3(x,y,z,spec['A'],spec['B'],f)==0
 assert len(row['runs'])==6 and all(r['root_sha256']==expected_hash for r in row['runs'])
assert json.loads((C/'LEDGER_VALIDATION.txt').read_text())['integrity']=='passed'
print('C94: hashes,157 raw files,9 actual certificates,36 cold root reports,10 exhaustive odd oracles and all5NIST output hashes verified. No qualified GB speedup.')

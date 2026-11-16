from pathlib import Path
import json,hashlib,sys,io,tarfile,lzma,base64
C=Path(__file__).resolve().parent;ROOT=C.parents[1];sys.path.insert(0,str(ROOT));index=json.loads((C/'RAW_INDEX.json').read_text());members={}
with tarfile.open(fileobj=io.BytesIO(lzma.decompress(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_text()))))as tf:
 assert set(tf.getnames())==set(index)
 for name,h in index.items():
  raw=tf.extractfile(name).read();assert hashlib.sha256(raw).hexdigest()==h,name;members[name]=raw
for name,h in json.loads((C/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==h,name
def data(name):return json.loads(members[name])
P='research/binary_rebracket_20261116_v96';D=json.loads((ROOT/P/'DATA.json').read_text());a=data(P+'/oracles_raw.json');assert a['error']is None;a=a['out'];assert len(a['tiny'])==672 and sum(r['root_count']for r in a['tiny'])==834 and a['zero_unique_total']==12 and a['infinity_unique_total']==166
from research.twist_auxiliary_20261116_v86.algebra import Field,s3
from research.binary_rebracket_20261116_v96.kernel import solve
for row in a['tiny']:
 n=row['n'];mod=next(v[1]for v in D['tiny']if v[0]==n);dim=row['d'];f=Field(2,mod);expected=[(x,y,z)for x in range(1<<dim)for y in range(1<<dim)for z in range(1<<n)if s3(row['a'],x,z,0,row['B'],f)==s3(z,y,row['b'],0,row['B'],f)==0];spec={'n':n,'mod':mod,'d':dim,'a':row['a'],'b':row['b'],'B':row['B'],'pair_cap_bits':12,'source_cap_bits':12,'diagonal_cap_bits':12};assert solve(spec,'rebracket')['roots']==expected
for spec,row in zip(D['actual'],a['actual']):
 assert spec['id']==row['id'];f=Field(2,spec['mod']);assert row['exact_root_count']==(len(row['roots'])if row['complete']else None)
 for x,y,z in row['roots']:assert x<1<<spec['d']and y<1<<spec['d']and s3(spec['a'],x,z,0,spec['B'],f)==s3(z,y,spec['b'],0,spec['B'],f)==0
 if 'planted'in spec:assert spec['planted']in row['roots']
assert not a['actual'][-1]['complete']and a['actual'][-1]['exact_root_count']is None
for folder,count in[(P,12),('research/binary_rebracket_scale_20261116_v96b',6)]:
 d=json.loads((ROOT/folder/'DATA.json').read_text());b=data(folder+'/cold_raw.json');assert b['error']is None;b=b['out'];assert b['complete']and len(b['runs'])==count;expected={}
 for r in b['runs']:
  v=r['parsed'];assert v['complete']and v['exact_root_count']==len(v['roots'])and len(v['cpu_affinity'])==1;expected.setdefault(r['id'],v['roots']);assert expected[r['id']]==v['roots'];spec=next(c for c in d['cold']if c['id']==r['id']);f=Field(2,spec['mod'])
  for x,y,z in v['roots']:assert s3(spec['a'],x,z,0,spec['B'],f)==s3(z,y,spec['b'],0,spec['B'],f)==0
assert (ROOT/P/'kernel.py').read_bytes()==(ROOT/'research/binary_rebracket_scale_20261116_v96b/kernel.py').read_bytes()
assert json.loads((C/'LEDGER_VALIDATION.txt').read_text())['integrity']=='passed';print('C96 hashes/archive,672independentliteralcontrols,zero/infinitycoverage,actualcapsemantics,and18coldcompleteoriginalrootsets verified. Not a wholechain/GB release.')

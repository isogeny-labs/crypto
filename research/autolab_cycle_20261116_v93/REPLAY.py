from pathlib import Path
import json,hashlib,sys,tarfile,io,lzma,base64
C=Path(__file__).resolve().parent;ROOT=C.parents[1];sys.path.insert(0,str(ROOT));binary='vendor/chain_native_v90/chain-native-baseline'
for n,h in json.loads((C/'EVIDENCE_SHA256.json').read_text()).items():
 p=ROOT/n
 if n==binary and not p.exists():continue
 assert hashlib.sha256(p.read_bytes()).hexdigest()==h,n
G=ROOT/'research/source_guards_20261116_v93';P=ROOT/'research/symmetric_pair_20261116_v93b';a=json.loads((G/'raw.json').read_text());b=json.loads((P/'oracles_raw.json').read_text());d=json.loads((P/'cold_raw.json').read_text());assert a['error']is None and b['error']is None and d['error']is not None;assert len(a['out']['tiny'])==132 and sum(x['guards']for x in a['out']['tiny'])==2292;assert sum(x['empty_guards']for x in a['out']['actual'])==73 and not any(x['entire_cell_empty']for x in a['out']['actual']);assert len(b['out']['tiny'])==120 and sum(x['root_count']for x in b['out']['tiny'])==188;assert len(b['out']['actual'])==9 and all(x['complete']for x in b['out']['actual']);assert len(d['out']['runs'])==1
from research.twist_auxiliary_20261116_v86.algebra import Field,s3
D=json.loads((P/'DATA.json').read_text());actual={x['id']:x for x in b['out']['actual']}
for c in D['cases']:
 v=actual[c['id']];assert v['exact_root_count']==len(v['roots']);f=Field(2,c['mod'])
 for x,y in v['roots']:assert x<1<<c['d']and y<1<<c['d']and s3(x,y,c['u'],0,c['B'],f)==0
idx=json.loads((C/'CHILD_INDEX.json').read_text());raw=lzma.decompress(base64.b64decode((C/'CHILD_EVIDENCE.tar.xz.b64').read_text()));timeout=False
with tarfile.open(fileobj=io.BytesIO(raw))as t:
 assert set(t.getnames())==set(idx)
 for n,h in idx.items():
  data=t.extractfile(n).read();assert hashlib.sha256(data).hexdigest()==h,n
  if n.endswith('_F4.stderr'):timeout=b'TimeoutExpired'in data and b'3 seconds'in data
  if n.endswith('_symmetric.json'):assert json.loads(data)['parsed']['roots']==[]
assert timeout and not json.loads((C/'LEDGER_VALIDATION.txt').read_text())['problems'];print('C93 hashes,guard evidence,actual source-pair membership and retained F4timeout verified;no qualified speed ratio')

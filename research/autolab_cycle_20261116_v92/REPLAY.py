from pathlib import Path
import json,hashlib,sys,tarfile,io,base64,lzma
C=Path(__file__).resolve().parent;ROOT=C.parents[1];sys.path.insert(0,str(ROOT))
for n,h in json.loads((C/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256((ROOT/n).read_bytes()).hexdigest()==h,n
P=ROOT/'research/disjoint_separator_20261116_v92';Q=ROOT/'research/product_span_cells_20261116_v92c';a=json.loads((P/'oracles_raw.json').read_text());b=json.loads((P/'cold_raw.json').read_text());d=json.loads((Q/'raw.json').read_text());assert all(v['error']is None and v['out']['complete']for v in[a,b,d]);assert len(a['out']['tiny'])==48 and sum(len(r['roots'])for r in a['out']['tiny'])==306;assert len(b['out']['runs'])==54 and len(d['out']['runs'])==6 and d['out']['tiny_checks']==240 and d['out']['large_checks']==160
from research.disjoint_separator_20261116_v92.kernel import Prefix
from research.twist_auxiliary_20261116_v86.algebra import s3
D=json.loads((P/'DATA.json').read_text());idx=json.loads((C/'CHILD_INDEX.json').read_text());assert len(idx)==180;raw=lzma.decompress(base64.b64decode((C/'CHILD_EVIDENCE.tar.xz.b64').read_text()));expected={};count=0;certcount=0
with tarfile.open(fileobj=io.BytesIO(raw))as t:
 assert set(t.getnames())==set(idx)
 for n,h in idx.items():
  data=t.extractfile(n).read();assert hashlib.sha256(data).hexdigest()==h,n
  if not n.endswith('.json'):continue
  v=json.loads(data)['parsed'];assert v['complete']
  if 'product_span_cells'in n:
   want=[{k:r[k]for k in['id','kind','k','u0','y','empty','rank','dual']}for r in d['out']['certificates']];assert v['certificates']==want;certcount+=1;continue
  i=int(Path(n).stem.split('_')[1]);c=D['cases'][i];S=Prefix(c['n'],c['mod'],c['d']);expected.setdefault(i,v['roots']);assert expected[i]==v['roots']and len(v['roots'])==v['exact_root_count'];count+=1
  for x,u in v['roots']:assert x<1<<S.d and s3(x,c['y'],u,0,c['B'],S.f)==0
  if v['stats']['nodes']:
   visited={u for x,u in v['roots']}
   for cell in v['stats']['pruned']:
    points=set(range(cell['u0'],cell['u0']+(1<<cell['k'])));assert not points&visited;visited|=points
   assert len(visited)==1<<S.n
assert count==54 and certcount==6 and not json.loads((C/'LEDGER_VALIDATION.txt').read_text())['problems'];print('C92 hashes,180rawfiles,60coldoutputs and completeconditionalcover accounting verified;noqualifiedglobalgain')

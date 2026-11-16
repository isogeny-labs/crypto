from pathlib import Path
import json,hashlib,sys,tarfile,io,lzma,base64
C=Path(__file__).resolve().parent;ROOT=C.parents[1];sys.path.insert(0,str(ROOT))
for n,h in json.loads((C/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256((ROOT/n).read_bytes()).hexdigest()==h,n
P=ROOT/'research/separator_images_20261116_v91';W=ROOT/'research/separator_cells_20261116_v91c';a=json.loads((P/'oracles_raw.json').read_text());b=json.loads((P/'cold_raw.json').read_text());d=json.loads((W/'raw.json').read_text());assert all(v['error']is None and v['out']['complete']for v in[a,b,d]);assert sum(x['linear_queries']+x['S3_queries']for x in a['out']['small'])==5008 and len(b['out']['runs'])==36
idx=json.loads((C/'CHILD_INDEX.json').read_text());assert len(idx)==108;raw=lzma.decompress(base64.b64decode((C/'CHILD_EVIDENCE.tar.xz.b64').read_text()));expect={i:[{'id':r['id'],'kind':r['kind'],'roots':r['roots']}for r in panel['queries']]for i,panel in enumerate(a['out']['large'])};count=0
with tarfile.open(fileobj=io.BytesIO(raw))as t:
 assert set(t.getnames())==set(idx)
 for n,h in idx.items():
  data=t.extractfile(n).read();assert hashlib.sha256(data).hexdigest()==h,n
  if n.endswith('.json'):
   v=json.loads(data)['parsed'];i=int(Path(n).stem.split('_')[1]);assert v['complete']and v['exact_root_count']==12 and v['roots']==expect[i];count+=1
assert count==36
from research.separator_images_20261116_v91.kernel import Solver
D=json.loads((W/'DATA.json').read_text())
for spec,results in zip(D['panels'],d['out']['panels']):
 S=Solver(spec['n'],spec['mod'],spec['d']);f=S.f;qs={q['id']:q for q in spec['queries']}
 for r in results['cells']:
  if not r['certified_empty']:continue
  q=qs[r['query_id']];y=r['y'];u=r['u_base'];a,b,c=S.coefficients(y,u,q['B']);cols=S.columns(a,b,True)
  for i in range(r['dimension']):
   h=1<<i;v=f.mul(y,h);cols.extend(S.columns(f.sq(h),v,True));cols.append(f.sq(v))
  w=r['dual_witness'];assert all((w&v).bit_count()%2==0 for v in cols)and(w&c).bit_count()%2==1 and r['pointwise_root_count']==0
assert d['out']['panels'][-1]['excluded']==24 and not json.loads((C/'LEDGER_VALIDATION.txt').read_text())['problems'];print('C91hashes,108rawfiles,36conditionaloutputs and everyempty-cell dualwitness verified;no global root/gainclaim')

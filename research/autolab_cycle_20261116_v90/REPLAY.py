from pathlib import Path
import json,hashlib,sys,tarfile,io,base64,lzma
C=Path(__file__).resolve().parent;ROOT=C.parents[1];sys.path.insert(0,str(ROOT));binary='vendor/chain_native_v90/chain-native-baseline'
for n,h in json.loads((C/'EVIDENCE_SHA256.json').read_text()).items():
 p=ROOT/n
 if n==binary and not p.exists():continue # Binary excluded from publication;exact receipt retained.
 assert hashlib.sha256(p.read_bytes()).hexdigest()==h,n
P=ROOT/'research/polynomial_bridge_20261116_v90b';L=ROOT/'research/linearized_bridge_20261116_v90c';a=json.loads((P/'oracles_raw.json').read_text());b=json.loads((P/'cold_raw.json').read_text());d=json.loads((L/'raw.json').read_text());assert all(v['error']is None and v['out']['complete']for v in[a,b,d]);assert len(a['out']['small'])==9 and sum(v['root_count']for v in a['out']['small'])==37;assert len(b['out']['runs'])==27;assert d['out']['actual']['linear_map_rank']==65 and not d['out']['actual']['solve_performed']and d['out']['actual']['exact_root_count']is None
from research.polynomial_bridge_20261116_v90b.codec import encode,field,evaluate
from research.polynomial_bridge_20261116_v90b.oracle import roots
from research.polynomial_bridge_20261116_v90b.binary import anf,decode
D=json.loads((P/'DATA.json').read_text());cases=[c for c in D['cases']if c['id']in D['binary_baseline_ids']];want={i:sorted(roots(c))for i,c in enumerate(cases)};codes={i:encode(c)for i,c in enumerate(cases)};anfs={i:anf(codes[i])for i in range(len(cases))};idx=json.loads((C/'CHILD_INDEX.json').read_text());assert len(idx)==99;raw=lzma.decompress(base64.b64decode((C/'CHILD_EVIDENCE.tar.xz.b64').read_text()));children=0
with tarfile.open(fileobj=io.BytesIO(raw))as t:
 assert set(t.getnames())==set(idx)
 for n,h in idx.items():
  data=t.extractfile(n).read();assert hashlib.sha256(data).hexdigest()==h,n
  if n.endswith('.json'):
   i=int(Path(n).stem.split('_')[1]);v=json.loads(data)
   if n.endswith('_neutral.json'):assert v==anfs[i];continue
   r=v['parsed'];assert r['complete']and r['exact_root_count']==3;assert list(map(tuple,r['roots']))==want[i];children+=1
   for row in r['roots']:assert all(evaluate(e,row,field(cases[i]))==0 for e in codes[i]['equations'])
   if r['method']in ['F4','F5']:
    native=json.loads(r['native']['stdout']);assert native['complete']and native['algorithm']==r['method']and native['source_revision']==D['baseline_revision'];decoded=sorted(decode(int(x,16),field(cases[i]).n,len(codes[i]['variables']))for x in native['roots_hex']);assert decoded==want[i]
assert children==27 and not json.loads((C/'LEDGER_VALIDATION.txt').read_text())['problems'];print('C90 source/input hashes,99raw files,27matched genuineF4/F5/blockoutputs and originalroots verified;GF83profileonly;noqualifiedgain')

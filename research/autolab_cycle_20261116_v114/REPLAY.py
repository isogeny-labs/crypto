from pathlib import Path
import sys,json,hashlib,base64,tarfile,io,random
C=Path(__file__).resolve().parent;R=C.parents[1];sys.path.insert(0,str(R));saved={}
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for name,h in json.loads((C/'RAW_INDEX.json').read_text()).items():
  b=t.extractfile(name).read();assert hashlib.sha256(b).hexdigest()==h,name;saved[name]=b
for name,h in json.loads((C/'SOURCE_SHA256.json').read_text()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
from research.local_split_circuit_20261116_v100.kernel import Circuit
D=json.loads((R/'research/xor_cut_20261116_v114b/DATA.json').read_text());rng=random.Random(D['seed']);prior=json.loads(saved['research/xor_cut_20261116_v114b/oracles_raw.json']);assert prior['error']is None
for case in range(32):
 n=3+case%5;G=Circuit(n)
 for _ in range(18):
  kind=rng.choice(['xor','and','not']);G.op(kind,rng.randrange(len(G.nodes)),None if kind=='not'else rng.randrange(len(G.nodes)))
 outputs=[rng.randrange(len(G.nodes))for _ in range(2)];roots=[]
 for theta in range(1<<n):
  a=[0,1]+[(theta>>i)&1 for i in range(n)]
  for kind,u,*rest in G.nodes[n+2:]:a.append(a[u]&a[rest[0]]if kind=='and'else a[u]^a[rest[0]]if kind=='xor'else 1-a[u])
  if not any(a[w]for w in outputs):roots.append(theta)
 assert roots==prior['out']['root_sets'][case]
for name in ['research/parity_union_search_20261116_v113c/search_raw.json','research/xor_cut_20261116_v114b/search_raw.json']:
 v=json.loads(saved[name]);assert v['error']=='nativeunknown'and not v['out']['models']and v['out']['original_complete']is False
cut=json.loads(saved['research/xor_cut_20261116_v114b/search_raw.json'])['out'];assert cut['backend_configuration_readback']['anf']=='false'and int(cut['statistics']['sat-cut.xors'])==32184
print('C114immutable evidence/source bytes and all32complete tiny rootsets independently verified;both hard runs UNKNOWN;native search not rerun and no speed ratio certified.')

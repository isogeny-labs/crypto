from pathlib import Path
import sys,json,hashlib,base64,tarfile,io,itertools,random,os
C=Path(__file__).resolve().parent;R=C.parents[1];os.chdir(R);sys.path.insert(0,str(R));saved={}
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for name,h in json.loads((C/'RAW_INDEX.json').read_text()).items():
  b=t.extractfile(name).read();assert hashlib.sha256(b).hexdigest()==h,name;saved[name]=b
for name,h in json.loads((C/'SOURCE_SHA256.json').read_text()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
from research.prefix_translation_20261116_v115.kernel import product,translate,independent_compose,bounded,degree_equations
from research.frontier_certificate_20261116_v111b.kernel import solve
from research.guarded_local_search_20261116_v101.kernel import matrix
D=json.loads((R/'research/prefix_translation_20261116_v115/DATA.json').read_text());rng=random.Random(D['seed']);cases=[]
for d in[2,3]:cases.extend((d,xs)for xs in itertools.combinations_with_replacement(range(1<<d),5))
for _ in range(128):cases.append((4,tuple(sorted(rng.randrange(16)for _ in range(5)))))
for _ in range(128):cases.append((4,tuple(sorted(8+rng.randrange(8)for _ in range(5)))))
for d,xs in cases:
 c=1<<(d-1);H=product(xs);shift=translate(H,c);assert shift==independent_compose(H,c)==product([x^c for x in xs])
 if(H[4]>>(d-1))&1:assert bool((H[1]>>(4*(d-1)))&1)==(sum(bool(x&c)for x in xs)==5)==bounded(shift,d)
payload=json.loads(saved['research/prefix_translation_20261116_v115/contracted_chart.json']);cfg=json.loads((R/'research/chart_compiler_20261116_v112/DATA.json').read_text());rows=matrix(cfg['matrix']);eq,labels=degree_equations(rows[0],rows[1:]+[[cfg['modulus']<<j,0,0,0,0,0]for j in range(3)],18);assert json.loads(json.dumps(eq))==payload['degree_equations'];new=solve(payload['old_chart1']['rows']+eq,76);assert json.loads(json.dumps(new))==payload['translated_chart1']and new['dimension']==57
from research.chart_compiler_20261116_v112.kernel import source_graph
from research.local_split_circuit_20261116_v100.kernel import Circuit,cnf
from research.parity_chart_20261116_v113d.kernel import optimized_import
source,outputs=source_graph(cfg);chart=payload['translated_chart1'];G=Circuit(57);forms=[((chart['particular']>>i)&1)|sum(((v>>i)&1)<<(j+1)for j,v in enumerate(chart['columns']))for i in range(76)];G,oo,stats=optimized_import(source,outputs,G,forms,list(range(2,59)),256,8000000);b=json.dumps({'nodes':G.nodes,'outputs':oo},separators=(',',':')).encode();assert all(hashlib.sha256(b).hexdigest()==h for h in json.loads((C/'DERIVED_GRAPH_RECEIPT.json').read_text())['omitted_public_duplicate_files'].values());assert len(G.nodes)-2== 46330 and len(cnf(G,oo))==169845
native=json.loads(saved['research/translated_branch_search_20261116_v115b/search_raw.json']);assert native['error']=='nativeunknown'and not native['out']['models']
print('C115immutable bytes,1104multiset identities, exact10row contraction and57inputgraph replayed;conditional native remainsUNKNOWN;otherbranch notsolved.')

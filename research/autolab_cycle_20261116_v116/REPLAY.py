from pathlib import Path
import sys,json,hashlib,base64,tarfile,io,itertools,random,os
C=Path(__file__).resolve().parent;R=C.parents[1];os.chdir(R);sys.path.insert(0,str(R));saved={}
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for name,h in json.loads((C/'RAW_INDEX.json').read_text()).items():
  b=t.extractfile(name).read();assert hashlib.sha256(b).hexdigest()==h,name;saved[name]=b
for name,h in json.loads((C/'SOURCE_SHA256.json').read_text()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
from research.leading_cover_20261116_v116.kernel import limits,fits,equations
from research.prefix_translation_20261116_v115.kernel import product,translate
from research.frontier_certificate_20261116_v111b.kernel import solve
from research.guarded_local_search_20261116_v101.kernel import matrix
D=json.loads((R/'research/leading_cover_20261116_v116/DATA.json').read_text());rng=random.Random(D['seed']);cases=[]
for size in[2,3]:cases.extend((size,xs)for xs in itertools.combinations_with_replacement(range(1<<size),5))
for _ in range(256):cases.append((4,tuple(sorted(rng.randrange(16)for _ in range(5)))))
for size,xs in cases:
 c=1<<(size-1);k=sum(bool(x&c)for x in xs);H=product(xs);assert fits(H,limits(size,k))and fits(translate(H,c),limits(size,5-k))
payload=json.loads(saved['research/leading_cover_20261116_v116/cover.json']);cfg=json.loads((R/'research/chart_compiler_20261116_v112/DATA.json').read_text());rows=matrix(cfg['matrix']);dirs=rows[1:]+[[cfg['modulus']<<j,0,0,0,0,0]for j in range(3)];mu=sum(((v[3]>>34)&1)<<i for i,v in enumerate(dirs));bu=(rows[0][3]>>34)&1
for rec in payload['records']:
 k=rec['k'];parent=payload['old_charts'][1 if k==5 else 0];eq=list(parent['rows'])
 if k!=5:eq.append((mu,bu^(k==3)))
 a,_=equations(rows[0],dirs,18,k,False);b,_=equations(rows[0],dirs,18,k,True);assert json.loads(json.dumps(solve(eq+a+b,76)))==rec['chart']
from research.cover_compiler_20261116_v116b.kernel import mapping,compile_map
from research.chart_compiler_20261116_v112.kernel import source_graph
from research.local_split_circuit_20261116_v100.kernel import cnf
sourcepath=R/cfg['source_graph']
if not sourcepath.exists():
 p=R/'research/trace_zero_predicate_20261116_v105/templates.json'
 if not p.exists():
  a=R/'research/autolab_cycle_20261116_v105'
  with tarfile.open(fileobj=io.BytesIO(base64.b64decode((a/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:data=t.extractfile(str(p.relative_to(R))).read()
  assert hashlib.sha256(data).hexdigest()==json.loads((a/'RAW_INDEX.json').read_text())[str(p.relative_to(R))]
  with p.open('xb')as f:f.write(data)
 from research.predicate_dispatch_20261116_v105b.kernel import compile_all as old_compile
 old,oldout,*_=old_compile(json.loads((R/'research/predicate_dispatch_20261116_v105b/DATA.json').read_text()));data=json.dumps({'nodes':old.nodes,'outputs':oldout},separators=(',',':')).encode();assert hashlib.sha256(data).hexdigest()==cfg['source_graph_sha256']
 with sourcepath.open('xb')as f:f.write(data)
m=mapping([r['chart']for r in payload['records']],76);assert m==json.loads(saved['research/cover_compiler_20261116_v116b/mapping.json']);source,outputs=source_graph(cfg);G,oo,stats=compile_map(source,outputs,m,256,8000000);b=json.dumps({'nodes':G.nodes,'outputs':oo},separators=(',',':')).encode();assert all(hashlib.sha256(b).hexdigest()==h for h in json.loads((C/'DERIVED_GRAPH_RECEIPT.json').read_text())['omitted_public_duplicate_files'].values());assert G.n==65 and len(G.nodes)-2==48128 and len(cnf(G,oo))==177023
print('C116immutable bytes,allleadingweightedbounds,exact61/63/57maps and canonical65inputgraph replayed;prepared native search not executed;no speed ratio certified.')

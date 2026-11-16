from pathlib import Path
import os,sys,json,hashlib,base64,tarfile,io
C=Path(__file__).resolve().parent;R=C.parents[1];os.chdir(R);sys.path.insert(0,str(R))
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for n,h in json.loads((C/'RAW_INDEX.json').read_text()).items():assert hashlib.sha256(t.extractfile(n).read()).hexdigest()==h,n
for n,h in json.loads((C/'SOURCE_SHA256.json').read_text()).items():assert hashlib.sha256((R/n).read_bytes()).hexdigest()==h,n
from research.cover_compiler_20261116_v116b.kernel import mapping
from research.guarded_high_root_20261116_v118.kernel import compile_all,raw_family
from research.chart_compiler_20261116_v112.kernel import source_graph
from research.guarded_local_search_20261116_v101.kernel import matrix
from research.local_split_circuit_20261116_v100.kernel import cnf
cfg=json.loads((R/'research/chart_compiler_20261116_v112/DATA.json').read_text());records=json.loads((R/'research/leading_cover_20261116_v116/cover.json').read_text())['records'];m=mapping([r['chart']for r in records],76);source,outputs=source_graph(cfg);G,oo,stats,extra=compile_all(source,outputs,m,raw_family(matrix(cfg['matrix']),cfg['modulus']),18);b=json.dumps({'nodes':G.nodes,'outputs':oo,'recovery':extra},separators=(',',':')).encode();assert hashlib.sha256(b).hexdigest()=='5994243f2689e89811f8ea4f937e92d686bdaa125b82ba8519ad37f286c97d1d';assert G.n==65 and len(G.nodes)-2==62650 and len(cnf(G,oo))==231270
print('C118immutable source/raw and exactguarded65inputgraph replayed;searchUNKNOWN;C119factorprototype remains unexecuted;no qualifiedspeedratio.')

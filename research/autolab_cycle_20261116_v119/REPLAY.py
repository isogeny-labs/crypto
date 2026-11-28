from pathlib import Path
import os,sys,json,hashlib,base64,tarfile,io
C=Path(__file__).resolve().parent;R=C.parents[1];os.chdir(R);sys.path.insert(0,str(R))
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for n,h in json.loads((C/'RAW_INDEX.json').read_text()).items():assert hashlib.sha256(t.extractfile(n).read()).hexdigest()==h,n
for n,h in json.loads((C/'SOURCE_SHA256.json').read_text()).items():assert hashlib.sha256((R/n).read_bytes()).hexdigest()==h,n
from research.coprime_factor_checks_20261116_v119b.circuit import lift,normalized_affine
from research.guarded_high_root_20261116_v118.kernel import raw_family
from research.guarded_local_search_20261116_v101.kernel import matrix
from research.local_split_circuit_20261116_v100.kernel import Circuit,cnf
cfg=json.loads((R/'research/chart_compiler_20261116_v112/DATA.json').read_text());chart=json.loads((R/'research/leading_cover_20261116_v116/cover.json').read_text())['records'][1]['chart'];G=Circuit(chart['dimension']);F=normalized_affine(raw_family(matrix(cfg['matrix']),cfg['modulus']),chart['particular'],chart['columns'],18,G);A,B,checks=lift(F,18,G);outputs=[{'bounded_coprime_factorization':True,'wires':[checks]}];b=json.dumps({'nodes':G.nodes,'outputs':outputs,'A':A,'B':B},separators=(',',':')).encode();assert hashlib.sha256(b).hexdigest()=='ac5a98921c9ce82f7fae048e98f0ab1327da8797377cea764deebf8038868b20';assert G.n==63 and len(G.nodes)-2==34322 and len(cnf(G,outputs))==123820
print('C119immutable source/raw and exact63inputcoprimefactorizationgraph replayed;factor supportstillunexecuted;no qualifiedspeedratio.')

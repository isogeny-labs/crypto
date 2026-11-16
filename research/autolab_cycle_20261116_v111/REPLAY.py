from pathlib import Path
import json,sys,hashlib,base64,tarfile,io,itertools,collections
C=Path(__file__).resolve().parent;R=C.parents[1];sys.path.insert(0,str(R));saved={}
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for name,h in json.loads((C/'RAW_INDEX.json').read_text()).items():
  b=t.extractfile(name).read();assert hashlib.sha256(b).hexdigest()==h,name;saved[name]=b
for name,h in json.loads((C/'SOURCE_SHA256.json').read_text()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
from research.residue_quotient_20261116_v99b.problem import build
from research.residue_quotient_20261116_v99b.kernel import code,rem,apply
from research.residue_tables_20261116_v102b.kernel import multiplication,allowed
from research.guarded_local_search_20261116_v101.kernel import matrix,decode,valid_guard
from research.syndrome_frontier_20261116_v111.kernel import build_code,Widths
from research.frontier_certificate_20261116_v111b.kernel import solve,guarded_charts
config=json.loads((R/'research/systematic_residue_20261116_v109/DATA.json').read_text());rows=matrix(str(R/config['matrix']));E,eb,widths,first,F,fb,last,details=build(R/'research/residue_quotient_20261116_v99b');tables=first+[last]
for s,m in config['fields'][8:]:tables.append(allowed(s,rem(rows[0][4],m),multiplication(s,m)))
cols=[0]*76;offset=0;affine=0;widths=[]
for s,m in config['fields']:
 directions=rows[1:]+[[config['modulus']<<j,0,0,0,0,0]for j in range(3)]
 for j,v in enumerate(directions):cols[j]^=code(v,s,m)<<offset
 affine^=code(rows[0],s,m)<<offset;widths.append(4*s);offset+=4*s
result=build_code(cols,affine,widths,tables,76);b=json.dumps(result,separators=(',',':')).encode();receipt=json.loads((C/'DERIVED_CODE_RECEIPT.json').read_text());assert all(hashlib.sha256(b).hexdigest()==h for h in receipt['omitted_public_duplicate_files'].values())
W=Widths(result['blocks']);hist=collections.Counter()
for tail in itertools.combinations(range(1,14),6):hist[W.cut(1|sum(1<<i for i in tail))]+=1
prior=json.loads(saved['research/frontier_certificate_20261116_v111b/profile_raw.json'])['out'];assert dict(hist)=={int(k):v for k,v in prior['balanced_width_histogram'].items()}and min(hist)==67
charts=json.loads(saved['research/frontier_certificate_20261116_v111b/charts.json']);equations=charts['local_hull_equations'];linear=solve(equations,76);assert linear==charts['linear']
def flag(i,bit):return sum(((v[i]>>bit)&1)<<j for j,v in enumerate(rows[1:])),(rows[0][i]>>bit)&1
rebuilt=guarded_charts(equations,76,flag(3,34),flag(2,51),flag(1,68),(1<<74,0),(1<<75,0));assert json.loads(json.dumps(rebuilt))==charts['charts']and all(c['dimension']==67 for c in rebuilt)
for c in rebuilt:
 for z in [0]+[1<<i for i in range(67)]:
  theta=c['particular']^apply(c['columns'],z);assert valid_guard(decode(rows,theta),theta>>73)
print('C111 raw/source hashes, exact code bytes, all1716balanced cuts and two67-bit guarded charts replayed;no hard search or original-system speedup certified.')

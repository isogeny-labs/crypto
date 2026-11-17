from pathlib import Path
import os,sys,json,hashlib,tarfile,io,base64
C=Path(__file__).resolve().parent;R=C.parents[1];os.chdir(R);sys.path.insert(0,str(R));saved={}
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for name,h in json.loads((C/'RAW_INDEX.json').read_bytes()).items():
  b=t.extractfile(name).read();assert hashlib.sha256(b).hexdigest()==h,name;saved[name]=b
for name,h in json.loads((C/'SOURCE_SHA256.json').read_bytes()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
from research.residue_quotient_20261116_v99b.kernel import basis,apply
cols=[5,6];piv,K=basis(cols)
for image,witness in piv.values():assert apply(cols,witness)==image
c=json.loads(saved['research/typed_cell_planes_20261117_v145b/census_raw.json']);assert c['error']is None and c['out']['complete_early_census'];o=c['out'];assert sum(o['rank_distribution'].values())==1<<24;assert sum(v*(1<<(39-int(r)))for r,v in o['rank_distribution'].items())==o['conditional_parameter_upper_bound']
g=json.loads(saved['research/singular_strata_20261117_v145c/guards_raw.json']);assert g['error']is None and g['out']['complete_sparse_guards']
for rec in g['out']['strata']:
 nodes=rec['nodes'];memo={}
 def expand(v,k):
  if v==0:return 0
  if v==1:return (1<<(1<<k))-1
  if (v,k)in memo:return memo[v,k]
  j,lo,hi=nodes[v];a=expand(lo,j)|(expand(hi,j)<<(1<<j));width=1<<(j+1)
  while width<1<<k:a|=a<<width;width*=2
  memo[v,k]=a;return a
 mask=expand(rec['root'],24);assert mask.bit_count()==rec['states'];assert hashlib.sha256(mask.to_bytes((1<<24)//8,'little')).hexdigest()==rec['truth_sha256']
print('C145 source/raw/child-ledger hashes,typed basis contract,complete census arithmetic and full2^24-stateBDDguard truth fingerprints replayed;nooriginalrootcountorqualifiedspeedclaim.')

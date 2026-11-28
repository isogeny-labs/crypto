from pathlib import Path
import os,sys,json,base64,tarfile,io,hashlib
C=Path(__file__).resolve().parent;R=C.parents[1];os.chdir(R);sys.path.insert(0,str(R))
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for n,h in json.loads((C/'RAW_INDEX.json').read_text()).items():assert hashlib.sha256(t.extractfile(n).read()).hexdigest()==h,n
for n,h in json.loads((C/'SOURCE_SHA256.json').read_text()).items():assert hashlib.sha256((R/n).read_bytes()).hexdigest()==h,n
from research.simple_prefix_lift_20261116_v117b.kernel import high_root,normalize,lift_circuit,evaluate
from research.prefix_translation_20261116_v115.kernel import product
from research.guarded_local_search_20261116_v101.kernel import matrix,decode
from research.residue_quotient_20261116_v99b.kernel import pmul
from research.local_split_circuit_20261116_v100.kernel import Circuit
for d in [2,3,4]:
 for n in range(1<<(d-1)):
  H=product([n,n,0,0,(1<<(d-1))|n]);root,tail,r=high_root(H,d);assert tail==0 and evaluate(H,root)==0
cfg=json.loads((R/'research/chart_compiler_20261116_v112/DATA.json').read_text());chart=json.loads((R/'research/leading_cover_20261116_v116/cover.json').read_text())['records'][0]['chart'];rows=matrix(cfg['matrix']);G=Circuit(chart['dimension'])
def raw(theta):
 H=decode(rows,theta);H[0]^=pmul(theta>>73,cfg['modulus']);return H
base=normalize(raw(chart['particular']),18);origin=raw(0);directions=[normalize([a^b for a,b in zip(raw(v),origin)],18)for v in chart['columns']];coeff=[]
for j,a in enumerate(base):
 bits=[]
 for bit in range((5-j)*17+1):
  w=(a>>bit)&1
  for i,v in enumerate(directions):
   if v[j]>>bit&1:w=G.xor(w,2+i)
  bits.append(w)
 coeff.append(bits)
root,tail,low=lift_circuit(coeff,18,G);b=json.dumps({'nodes':G.nodes,'outputs':[{'unique_high_root_tail':True,'wires':[tail]}],'root_wires':root,'low_residual_wires':low},separators=(',',':')).encode();assert hashlib.sha256(b).hexdigest()=='ed5ba88edb70a957719793207ee3fb1e7773f2c01c3dcb89b34a3377ec2d95b9'
print('C117 immutable sources/raw and exact61-input high-root graph replayed; no global speed ratio certified.')

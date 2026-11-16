from pathlib import Path
import json,hashlib,time
from rootfinder.binaryfield import BinaryField
from research.certified_support_20261115_v44 import solver as support
from research.root_conditioned_fibers_20261115_v45 import solver as s
P=Path(__file__).resolve().parent;R=P.parents[1];start=time.monotonic();raws=[]
for name,digest in json.loads((P/'EVIDENCE_SHA256.json').read_text()).items():
 data=(R/name).read_bytes();assert hashlib.sha256(data).hexdigest()==digest;x=json.loads(data);assert x['error']is None
 if 'native'in x['out']:raws.append(x['out'])
a,bb=raws;assert a['norm_family_complete']and bb['norm_family_complete']
assert a['native']['rank_histogram']==bb['native']['rank_histogram']=={'72':496,'73':261647}
key=lambda x:{(r,int(t,16))for r,t in x['native']['candidates']}
assert key(a)==key(bb)and len(key(a))==249 and a['supported']==bb['supported']==[]
text=(R/'research/root_conditioned_native_20261115_v45/matrix.txt').read_text().split();m=int(text[0]);values=[int(v,16)for v in text[1:]];base=values[:6];cols=[values[i:i+6]for i in range(6,len(values),6)];assert len(cols)==m==73
f=BinaryField(83,support.MOD);lam=support.compile_basis(f,[1<<i for i in range(18)])[0]
for root,t in key(a):
 H=[b^s.b.xor(c[j]for i,c in enumerate(cols)if t>>i&1)for j,b in enumerate(base)];assert s.evaluate(f,H,root)==0;assert not support.generic(f,tuple(H),lam)
print(json.dumps(dict(full_rank_histograms_match=True,complete_candidates_rechecked=249,supported_norms=0,scope='one fixed h k0 only',seconds=time.monotonic()-start)))

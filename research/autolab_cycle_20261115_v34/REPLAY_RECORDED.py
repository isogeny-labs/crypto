"""Independent all-h census replay via batch inversion, not per-h Euclid."""
from pathlib import Path
import json,hashlib,runpy,time
R=Path(__file__).resolve().parents[2];START=time.monotonic();runpy.run_path(str(R/'research/autolab_cycle_20261115_v33/REPLAY_RECORDED.py'))
def load(path,sha):
 data=(R/path).read_bytes();assert hashlib.sha256(data).hexdigest()==sha;v=json.loads(data);assert v['error']is None;return v['out']
prior=load('research/centroid_cover_20261115_v33/raw.json','1cae9d46d578598c9ed9302d453e7eebc0e4976706cab3bee4bff7d5bc832723')['actual'];out=load('research/as_global_census_20261115_v34/raw.json','763eb083ddb602efb0ecd9e3dc1b353e87ddd6c67234fee70eb8e148887d5f9b');a=out['actual'];mod=a['modulus'];n=83
assert a['basis']==prior['domain']['basis'] and  a['particular']==prior['domain']['particular'] and  a['complete']
def reduce(v):
 while v.bit_length()>n:v^=mod<<(v.bit_length()-n-1)
 return v
def mul(a,b):
 v=0
 while b:
  bit=b&-b;v^=a<<(bit.bit_length()-1);b^=bit
 return reduce(v)
def square(a):return reduce(sum(1<<(2*i)for i in range(n)if a>>i&1))
def trace(a):
 v=0
 for _ in range(n):v^=a;a=square(a)
 assert v in(0,1);return v
def inverse(a):
 u,v,x,y=a,mod,1,0
 while u!=1:
  k=u.bit_length()-v.bit_length()
  if k<0:u,v,x,y,k=v,u,y,x,-k
  u^=v<<k;x^=y<<k
 return reduce(x)
assert trace(a['target_z'])==a['trace_rhs']==0
powers=[];v=1
for _ in range(n+35-1):powers.append(trace(v));v=mul(v,2)
masks=[sum(powers[i+j]<<i for i in range(n))for j in range(35)];assert masks==a['masks']
N=1<<len(a['basis']);assert N==a['seen']==a['total']==262144;H=[];prefix=[];product=1;h=a['particular']
for i in range(N):
 if i:h^=a['basis'][(i&-i).bit_length()-1]
 assert h;H.append(h);prefix.append(product);product=mul(product,h)
assert product==prior['domain']['total_product']==a['total_product']
invproduct=inverse(product);assert mul(product,invproduct)==1;invs=[0]*N
for i in range(N-1,-1,-1):invs[i]=mul(invproduct,prefix[i]);invproduct=mul(invproduct,H[i])
assert invproduct==1
hist=[0]*35;degenerate=0;inverse_sum=0;digest=hashlib.sha256()
for i,(h,inv)in enumerate(zip(H,invs)):
 assert mul(h,inv)==1;v=square(inv);pivot=next((j for j,m in enumerate(masks)if(m&v).bit_count()%2),None)
 if pivot is None:degenerate+=1
 else:hist[pivot]+=1
 inverse_sum^=inv;digest.update(h.to_bytes(11,'little')+inv.to_bytes(11,'little'))
 if i%4096==0:assert time.monotonic()-START<85
assert hist==a['pivot_histogram'] and  degenerate==a['degenerate']==0 and  inverse_sum==a['total_inverse_sum']==prior['domain']['total_inverse_sum'] and  digest.hexdigest()==a['sequence_sha256']
assert sum(hist[17:])==0
print('VERIFIED: all262144inverse/rank witnesses via independent batch inversion; histograms, full sequence digest, product and reciprocal sums; no E2rankdrop for this target.')

"""Replay recorded ANF hashes using generic Newton recursion and other arithmetic."""
from pathlib import Path
import gzip,hashlib,json,time
from research.gold_projection_20261115_v14 import worker as field_oracle
from research.joint_cubic_20261115_v15 import worker as subject

start=time.monotonic();root=Path(__file__).resolve().parents[2]
rawpath=root/'research/joint_cubic_20261115_v15/raw.json.gz'
raw=json.loads(gzip.decompress(rawpath.read_bytes()))['out']['actual']
class Field:
    def mul(self,a,b):return field_oracle.multiply(a,b,83,0x800000000200000000007)
    def square(self,a):return self.mul(a,a)
f=Field();R=subject.Ring(f);cert=raw['family_certificate'];H=[]
for j in range(6):
    p={0:cert['base'][j]}if cert['base'][j]else{}
    for i,column in enumerate(cert['columns']):
        if column[j]:p[1<<i]=column[j]
    H.append(p)
# Generic Newton sums, with even powers replaced by their Frobenius squares.
# This does NOT use the optimizedg9straight-line identity being audited.
powers=[{0:1}]
for k in range(1,10):
    if not k%2:value=R.square(powers[k//2])
    else:
        value=R.add(*(R.mul(H[5-j],powers[k-j])for j in range(1,min(5,k-1)+1)))
        if k<=5:value=R.add(value,H[5-k])
    powers.append(value)
a=cert['base'][4];ap={0:a};moments=[]
for k in range(1,10):
    if k in[3,5,9]:moments.append(R.add(powers[k],ap))
    ap=R.scale(ap,a)
for p,expected in zip(moments,raw['moment_polynomials']):
    assert subject.info(p)==expected,'complete Newton ANF hash mismatch'
assert len(field_oracle.pivots(raw['nonlinear_span']))==61
assert len(field_oracle.pivots(raw['cubic_span']))==38
assert all(all(not((lam&v).bit_count()&1)for v in raw['cubic_span'])for lam in raw['quadratic_duals'])
assert len(field_oracle.pivots(raw['quadratic_duals']))==58
assert len(field_oracle.pivots(raw['affine_duals']))==35
assert time.monotonic()-start<60,'replay budget'
report=dict(verified=True,method='Generic Newton recurrence with independent coefficient-convolution field arithmetic; shared Boolean ring helper explicitly disclosed; complete g3/g5/g9ANF hashes and dual ranks replayed',
    same_full_moment_polynomials=True,nonzero_affine_consequences=0,nonzero_quadratic_consequences=23,remaining_cubic_dimension=38,
    seconds=time.monotonic()-start,field_product_pairs=subject.PAIRS,external_reviewer=False,no_new_root_or_point_search=True,
    raw_sha256=hashlib.sha256(rawpath.read_bytes()).hexdigest())
with Path(__file__).with_name('NEWTON_REPLAY.json').open('x')as out:json.dump(report,out,indent=2)
print(json.dumps(report,sort_keys=True))

#!/usr/bin/env python3
"""Independent n35 base and column-major minimum-pair index audit."""
from sage.all import *
import hashlib,json,resource,time,signal
from pathlib import Path
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[3]
P=json.loads((HERE/'protocol.json').read_text())
O_PATH=HERE/'result.json'
O=json.loads(O_PATH.read_text())
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def digest(obj):return hashlib.sha256(json.dumps(obj,sort_keys=True,separators=(',',':')).encode()).hexdigest()
assert O['status']=='complete' and len(O['targets'])==32
assert sha(HERE/'protocol.json')==O['protocol_sha256']
assert sha(HERE/'run.py')==O['runner_sha256']
assert sha(ROOT/P['screen_result'])==P['screen_result_sha256']==O['screen_result_sha256']
assert sha(ROOT/P['source'])==P['source_sha256']
S=json.loads((ROOT/P['screen_result']).read_text())
selected=next((x for x in S['instances'] if x['largest_prime_divisor']>=P['revised_prime_threshold']),None)
assert selected is not None and selected['index']==1 and S['selected_index'] is None
n=35;L=12;r=520604191;h=66
assert selected['group_order']==O['group_order']==r*h
F2=GF(2);PR=PolynomialRing(F2,'z');z=PR.gen()
poly=selected['field_modulus'];mod=sum(F2((poly>>i)&1)*z**i for i in range(n+1))
K=GF(1<<n,'z',modulus=mod)
def elt(v):return K([(v>>i)&1 for i in range(n)])
E=EllipticCurve(K,[1,elt(selected['curve_a2']),0,0,elt(selected['curve_a6'])])
assert int(E.cardinality())==r*h
def enc(e):return int(e.to_integer())
def slow(e):return sum(int(c)<<i for i,c in enumerate(e.polynomial().list()))
def rec(Q):return [enc(Q[0]),enc(Q[1])]
zero=E(0)
def key(Q):
    if Q==zero:return 1<<(2*n)
    return enc(Q[0])|(enc(Q[1])<<n)
raw=[]
for bits in range(1<<L):
    x=0
    for j,v in enumerate(selected['subspace_basis']):
        if (bits>>j)&1:x^=v
    raw.extend(E.lift_x(elt(x),all=True))
assert len(raw)==O['raw_count'] and len(raw)==len(set(raw))
assert sum(r*Q==zero for Q in raw)==O['direct_subgroup_points']
cleared=[h*Q for Q in raw]
base=sorted(set(cleared)-{zero},key=rec)
assert len(base)==O['base_count']==4096
assert cleared.count(zero)==O['cleared_identity_images']
assert len(cleared)-len(set(cleared))==O['cleared_duplicate_images']
assert digest([rec(Q) for Q in base])==O['base_sha256']
assert all(r*Q==zero for Q in base)
G=next(Q for Q in cleared if Q!=zero)
assert rec(G)==O['generator']
assert all(enc(Q[0])==slow(Q[0]) and enc(Q[1])==slow(Q[1]) for Q in base)
# Column-major order with explicit lexicographic minimum, unlike runner's row-major setdefault.
B=len(base);first={};start=time.perf_counter_ns()
for j,C in enumerate(base):
    for i,A in enumerate(base):
        k=key(A+C);v=i*B+j
        old=first.get(k)
        if old is None or v<old:first[k]=v
    if j%512==0 and resource.getrusage(resource.RUSAGE_SELF).ru_maxrss>8*1024**3:
        raise MemoryError('audit observed RSS exceeded 8 GiB')
assert len(first)==O['distinct_pair_sums']==8321091
for row in O['targets']:
    i=row['index'];assert 0<=i<32
    label=f'h4-n35-instance1-target-{i}'
    scalar=int.from_bytes(hashlib.sha256(label.encode()).digest(),'big')%(r-1)+1
    Q=scalar*G
    assert row['label']==label and row['fixture_scalar']==scalar and row['target']==rec(Q)
    witness=None;attempts=0
    for k,C in enumerate(base):
        attempts+=1
        hit=first.get(key(Q-C))
        if hit is not None:
            a,b=divmod(hit,B)
            assert base[a]+base[b]+C==Q
            witness=[a,b,k];break
    assert row['status']==('verified' if witness is not None else 'exhausted')
    assert row['third_point_attempts']==attempts
    assert row['witness_indices']==witness
receipt={'schema':'h4-n35-packed-pair-audit-v1','status':'passed',
         'result_sha256':sha(O_PATH),'screen_result_sha256':P['screen_result_sha256'],
         'base_count':B,'distinct_pair_sums':len(first),
         'target_outcomes_checked':len(O['targets']),
         'verified_targets':sum(x['status']=='verified' for x in O['targets']),
         'audit_index_ns':time.perf_counter_ns()-start,
         'peak_rss_bytes':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss}
(HERE/'audit.json').write_text(json.dumps(receipt,indent=2,sort_keys=True)+'\n')
print(json.dumps(receipt,indent=2,sort_keys=True))

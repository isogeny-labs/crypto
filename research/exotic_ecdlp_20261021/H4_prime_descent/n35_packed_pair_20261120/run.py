#!/usr/bin/env python3
"""Frozen n35 cleared-subgroup direct-integer pair index and 32 target lookups."""
from sage.all import *
import hashlib, json, resource, sys, time, signal
from pathlib import Path
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[3]
P=json.loads((HERE/'protocol.json').read_text())
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
SCREEN_PATH=ROOT/P['screen_result']
assert sha(SCREEN_PATH)==P['screen_result_sha256']
assert sha(ROOT/P['source'])==P['source_sha256']
SCREEN=json.loads(SCREEN_PATH.read_text())
assert SCREEN['status']=='complete' and SCREEN['selected_index'] is None
selected=next((x for x in SCREEN['instances'] if x['largest_prime_divisor']>=P['revised_prime_threshold']),None)
assert selected is not None and selected['index']==P['selected_instance_index']==1
assert selected['group_order']==P['group_order']==34359876606
assert selected['largest_prime_divisor']==P['prime_subgroup_order']==520604191
assert selected['cofactor']==P['cofactor']==66
N=P['field_degree'];L=P['subspace_dimension'];R=P['prime_subgroup_order'];H=P['cofactor']
assert N==35 and L==12 and R*H==P['group_order'] and is_prime(R)
def enc(e):return int(e.to_integer())
def rec(Q):return [enc(Q[0]),enc(Q[1])]
def digest(obj):return hashlib.sha256(json.dumps(obj,sort_keys=True,separators=(',',':')).encode()).hexdigest()
result={'schema':'h4-n35-packed-pair-result-v1','status':'running',
        'protocol_sha256':sha(HERE/'protocol.json'),'runner_sha256':sha(HERE/'run.py'),
        'screen_result_sha256':P['screen_result_sha256'],'stages':{},'targets':[]}
def save(): (HERE/'result.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
signal.signal(signal.SIGALRM,lambda *_:(_ for _ in ()).throw(TimeoutError('frozen 600-second process cap')))
signal.alarm(P['process_cap_seconds'])
save();start=time.perf_counter_ns()
try:
    t=time.perf_counter_ns()
    poly=selected['field_modulus'];F2=GF(2);PR=PolynomialRing(F2,'z');z=PR.gen()
    mod=sum(F2((poly>>i)&1)*z**i for i in range(N+1))
    K=GF(1<<N,'z',modulus=mod)
    def elt(v):return K([(v>>i)&1 for i in range(N)])
    E=EllipticCurve(K,[1,elt(selected['curve_a2']),0,0,elt(selected['curve_a6'])])
    assert int(E.cardinality())==P['group_order']
    basis=selected['subspace_basis'];assert len(basis)==L
    result.update(field_modulus=poly,curve_a2=selected['curve_a2'],curve_a6=selected['curve_a6'],
                  subspace_basis=basis,group_order=P['group_order'],subgroup_order=R,cofactor=H)
    result['stages']['instance_ns']=time.perf_counter_ns()-t;save()
    t=time.perf_counter_ns();raw=[]
    for bits in range(1<<L):
        x=0
        for j,v in enumerate(basis):
            if (bits>>j)&1:x^=v
        raw.extend(E.lift_x(elt(x),all=True))
    zero=E(0);assert len(raw)==len(set(raw))
    direct=sum(R*Q==zero for Q in raw)
    cleared=[H*Q for Q in raw]
    base=sorted(set(cleared)-{zero},key=rec)
    assert all(R*Q==zero for Q in base)
    G=next(Q for Q in cleared if Q!=zero)
    result.update(raw_count=len(raw),direct_subgroup_points=direct,
                  cleared_identity_images=cleared.count(zero),
                  cleared_duplicate_images=len(cleared)-len(set(cleared)),
                  base_count=len(base),base_sha256=digest([rec(Q) for Q in base]),generator=rec(G))
    result['stages']['base_ns']=time.perf_counter_ns()-t;save()
    def key(Q):
        if Q==zero:return 1<<(2*N)
        return enc(Q[0])|(enc(Q[1])<<N)
    assert len({key(Q) for Q in base})==len(base)
    t=time.perf_counter_ns();first={};B=len(base)
    for i,A in enumerate(base):
        for j,C in enumerate(base):
            first.setdefault(key(A+C),i*B+j)
        if i%64==0 and resource.getrusage(resource.RUSAGE_SELF).ru_maxrss>P['observed_peak_rss_bytes']:
            raise MemoryError('frozen 8 GiB observed RSS threshold')
        if i%128==0:
            result['index_progress_outer_i']=i
            result['peak_rss_so_far_bytes']=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
            save()
    result.update(pair_attempts=B*B,distinct_pair_sums=len(first),dict_shallow_bytes=sys.getsizeof(first))
    result['stages']['index_ns']=time.perf_counter_ns()-t
    result['peak_rss_after_index_bytes']=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    if result['peak_rss_after_index_bytes']>P['observed_peak_rss_bytes']:
        raise MemoryError('frozen 8 GiB observed RSS threshold')
    save()
    for i in range(P['target_count']):
        label=f'h4-n35-instance1-target-{i}'
        scalar=int.from_bytes(hashlib.sha256(label.encode()).digest(),'big')%(R-1)+1
        Q=scalar*G
        t=time.perf_counter_ns();witness=None;attempts=0
        for k,C in enumerate(base):
            attempts+=1
            hit=first.get(key(Q-C))
            if hit is not None:
                a,b=divmod(hit,B)
                assert base[a]+base[b]+C==Q
                witness=[a,b,k];break
        result['targets'].append({'index':i,'label':label,'fixture_scalar':scalar,'target':rec(Q),
                                  'status':'verified' if witness is not None else 'exhausted',
                                  'third_point_attempts':attempts,'witness_indices':witness,
                                  'lookup_ns':time.perf_counter_ns()-t})
        save()
    result['status']='complete'
except TimeoutError as exc:result.update(status='censored_timeout',error=str(exc))
except MemoryError as exc:result.update(status='censored_memory_limit',error=str(exc))
except Exception as exc:result.update(status='error',error=repr(exc));raise
finally:
    signal.alarm(0)
    result['total_ns']=time.perf_counter_ns()-start
    result['peak_rss_bytes']=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    save()
print(json.dumps({k:result.get(k) for k in ('status','error','base_count','pair_attempts','distinct_pair_sums','peak_rss_bytes')},sort_keys=True))

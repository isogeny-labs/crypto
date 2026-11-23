#!/usr/bin/env python3
"""Frozen n31 point-key versus injective coordinate-integer pair indexes."""
from sage.all import *
import hashlib, json, resource, signal, sys, time
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
SPEC = json.loads((HERE / 'protocol.json').read_text())
assert len(sys.argv) == 3 and sys.argv[1] in ('point', 'packed')
variant, repetition = sys.argv[1], int(sys.argv[2])
assert 1 <= repetition <= 2
assert [variant, repetition] in SPEC['run_order']
def sha(path): return hashlib.sha256(path.read_bytes()).hexdigest()
C_PATH = ROOT / SPEC['candidate']
REF_PATH = ROOT / SPEC['reference_pair_result']
assert sha(C_PATH) == SPEC['candidate_sha256']
assert sha(REF_PATH) == SPEC['reference_pair_sha256']
C = json.loads(C_PATH.read_text())['record']
REF = json.loads(REF_PATH.read_text())
assert REF['status'] == 'complete' and len(REF['targets']) == 32
N = C['field']['n']
R = C['curve']['r']
assert N == 31 and R == 34635389
signal.signal(signal.SIGALRM, lambda *_: (_ for _ in ()).throw(TimeoutError('frozen 180-second arm cap')))
signal.alarm(SPEC['per_arm_cap_seconds'])
result = {'schema':'h4-n31-packed-index-arm-v1','variant':variant,'repetition':repetition,
          'status':'running','protocol_sha256':sha(HERE/'protocol.json'),
          'runner_sha256':sha(HERE/'run.py'),'candidate_sha256':SPEC['candidate_sha256'],
          'reference_pair_sha256':SPEC['reference_pair_sha256'],'targets':[]}
OUT = HERE / f'{variant}-{repetition}.json'
def save(): OUT.write_text(json.dumps(result,sort_keys=True,indent=2)+'\n')
save()
start = time.perf_counter_ns()
try:
    poly = C['field']['defining_polynomial_int']
    F2 = GF(2); PR = PolynomialRing(F2,'z'); z = PR.gen()
    mod = sum(F2((poly >> i) & 1)*z**i for i in range(N+1))
    K = GF(1 << N,'z',modulus=mod)
    def elt(v): return K([(v >> i)&1 for i in range(N)])
    coeff = C['curve']['coefficients']
    E = EllipticCurve(K,[1,elt(coeff['a2']),0,0,elt(coeff['a6'])])
    def point(pair): return E(elt(pair[0]),elt(pair[1]))
    def encode(e): return sum(int(c)<<i for i,c in enumerate(e.polynomial().list()))
    def record(P): return [encode(P[0]),encode(P[1])]
    def key(P):
        if P == E(0): return 1 << (2*N)
        return encode(P[0]) | (encode(P[1]) << N)
    base = [point(pair) for pair in C['factor_base']['encoded_points']]
    assert len(base) == REF['base_count'] == 2090
    assert [record(P) for P in base] == C['factor_base']['encoded_points']
    assert C['curve']['G'] == REF['generator']
    assert len({key(P) for P in base}) == len(base)
    targets = [point(row['target']) for row in REF['targets']]
    result['setup_ns'] = time.perf_counter_ns()-start
    save()
    t = time.perf_counter_ns()
    index = {}
    for i,A in enumerate(base):
        for j,B in enumerate(base):
            S = A+B
            if variant == 'point':
                index.setdefault(S,(i,j))
            else:
                index.setdefault(key(S),i*len(base)+j)
    result['index_ns'] = time.perf_counter_ns()-t
    result['pair_attempts'] = len(base)**2
    result['stored_sums'] = len(index)
    result['dict_shallow_bytes'] = sys.getsizeof(index)
    result['peak_rss_after_build_bytes'] = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    if result['peak_rss_after_build_bytes'] > SPEC['memory_cap_observed_bytes']:
        raise MemoryError('frozen 8 GiB observed RSS cap')
    assert len(index) == REF['distinct_pair_sums'] == 2116611
    save()
    for row,Q in zip(REF['targets'],targets):
        t = time.perf_counter_ns(); witness = None; attempts = 0
        for k,P in enumerate(base):
            attempts += 1
            residual = Q-P
            hit = index.get(residual if variant == 'point' else key(residual))
            if hit is not None:
                if variant == 'point': a,b = hit
                else: a,b = divmod(hit,len(base))
                assert base[a]+base[b]+P == Q
                witness = [a,b,k]
                break
        out = {'index':row['index'],'target':row['target'],'attempts':attempts,
               'witness_indices':witness,'lookup_ns':time.perf_counter_ns()-t,
               'status':'verified' if witness is not None else 'exhausted'}
        assert out['status'] == row['status']
        assert out['attempts'] == row['attempts']
        assert out['witness_indices'] == row['witness_indices']
        result['targets'].append(out)
        save()
    result['status'] = 'complete'
except TimeoutError as exc:
    result.update(status='censored_timeout',error=str(exc))
except MemoryError as exc:
    result.update(status='censored_memory_limit',error=str(exc))
except Exception as exc:
    result.update(status='error',error=repr(exc))
    raise
finally:
    signal.alarm(0)
    result['total_ns'] = time.perf_counter_ns()-start
    result['peak_rss_bytes'] = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    save()
print(json.dumps({k:result.get(k) for k in ('variant','repetition','status','error','index_ns','stored_sums','peak_rss_bytes')},sort_keys=True))

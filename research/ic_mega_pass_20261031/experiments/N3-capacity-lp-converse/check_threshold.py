# post-hoc: apply the corrected rule (excess iff npts<=4*Mmax) to the 240 pre-registered m=2 cells; m3 l=1 extra p
import json, random, itertools, sys
from n3lib import *
A = json.load(open('raw/armA_m2.json')); cache = {}
for r in A:
    k = (r['seed'], r['p'])
    if k not in cache:
        rng = random.Random(1000*r['seed']+r['p']); f, _ = fpoly(2, r['p'], rng); p = r['p']
        cache[k] = sum(1 for x in range(p) for y in range(p) if evalf(f, [x, y], p) == 0)
    r['npts'] = cache[k]
viol = [(r['seed'],r['p'],r['l'],r['M'],r['npts'],r['gain']-r['ksum']) for r in A if r['M'].startswith('box') and r['npts'] > 4*r['Mmax'] and r['status'] != 'equal']
pred_ok = [(r['seed'],r['p'],r['l'],r['M'],r['npts'],r['gain']-r['ksum']) for r in A if r['M'].startswith('box') and r['npts'] <= 4*r['Mmax'] and r['status'] == 'equal']
print("npts per (seed,p):", cache)
print("box cells npts>4Mmax but not equal (rule violations):", viol)
print("box cells npts<=4Mmax but equal:", pred_ok)
json.dump(dict(npts={str(k): v for k, v in cache.items()}, violations=viol, low_npts_equal=pred_ok), open('raw/check_threshold_m2.json','w'), indent=1)
# m3 l=1 box4 extra cells
out = []
M = list(itertools.product(range(5), repeat=3))
for p, seed in ((13, 1), (13, 2), (19, 0), (23, 0), (29, 0)):
    rng = random.Random(777*seed + p); f, _ = fpoly(3, p, rng)
    npts = sum(1 for x in range(p) for y in range(p) for z in range(p) if evalf(f, [x, y, z], p) == 0)
    hist = []
    for N in (750, 1500, 3000):
        hist.append(gain(eval_matrix(M, sample_U(f, 3, p, 1, N, rng), p), p, 1))
    out.append(dict(p=p, seed=seed, npts=npts, gains=hist, ksum=ksum(M, 4, 1))); print(out[-1], flush=True)
json.dump(out, open('raw/armA_m3_extra.json','w'), indent=1)

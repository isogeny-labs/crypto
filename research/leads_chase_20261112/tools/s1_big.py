#!/usr/bin/env python3
"""S1 crossing, large cells on the big closure engine (CPU, --nosplit): one or two inputs.
usage: s1_big.py --csd <ml-cryptanalysis/csd> --out rec.json --p P --a A --b B --t T --k K --D D [--sigma M] [--h-seed S] [--x1g] [--threads N]
The second input is sigma_M(g) (exponents times M mod 2^k), a random swap-symmetric h on {0,1,2}^2 (seeded), or x1*g."""
import sys, json, argparse, hashlib, time, os
import numpy as np
ap = argparse.ArgumentParser()
for n in ('csd', 'out'): ap.add_argument('--' + n, required=True)
for n in ('p', 'a', 'b', 't', 'k', 'D'): ap.add_argument('--' + n, type=int, required=True)
ap.add_argument('--sigma', type=int); ap.add_argument('--h-seed', type=int); ap.add_argument('--x1g', action='store_true')
ap.add_argument('--threads', type=int, default=2); ap.add_argument('--work-gb', type=float, default=3.0)
a = ap.parse_args()
sys.path.insert(0, os.path.dirname(a.csd)); sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from csd import ic_bounds_big as B
from unit_test import s3_coeffs
N = 1 << a.k
def chain_gen(p, aa, bb, t, k):
    g = {}
    for (e1, e2), v in s3_coeffs(p, aa, bb, t).items():
        key = (e1 % N, e2 % N); g[key] = (g.get(key, 0) + v) % p
    return {e: v for e, v in g.items() if v}
g = chain_gen(a.p, a.a, a.b, a.t, a.k)
extra, label = [], ['g']
if a.sigma:
    s = {}
    for (e1, e2), v in g.items():
        key = ((e1 * a.sigma) % N, (e2 * a.sigma) % N); s[key] = (s.get(key, 0) + v) % a.p
    extra.append({e: v for e, v in s.items() if v}); label.append(f'sigma_{a.sigma} g')
if a.h_seed is not None:
    rng = np.random.default_rng(a.h_seed); h = {}
    for e1 in range(3):
        for e2 in range(e1, 3):
            v = int(rng.integers(1, a.p)); h[(e1, e2)] = v; h[(e2, e1)] = v
    extra.append(h); label.append(f'h(seed {a.h_seed})')
if a.x1g:
    extra.append({((e1 + 1) % N, e2): v for (e1, e2), v in g.items()}); label.append('x1 g')
wt = lambda poly: max(bin(e1).count('1') + bin(e2).count('1') for e1, e2 in poly)
class MultiEngine(B.Engine):
    def do_generators(self):
        base_g, base_cols = self.g, self.gen_cols
        for poly in [base_g] + extra:
            self.g = poly
            self.gen_cols = np.flatnonzero(self.L.w <= self.D - wt(poly))
            self.gen_next = 0
            super().do_generators()
        self.g, self.gen_cols = base_g, base_cols
t0 = time.time()
eng = MultiEngine(g, a.k, a.p, a.D, device='cpu', split=False, gens='all', threads=a.threads, work_gb=a.work_gb,
                  log=False, t_random=6, source_id=hashlib.sha256(json.dumps([a.p, a.a, a.b, a.t, a.k, a.D, label]).encode()).hexdigest())
status = eng.run()
rec = eng.record(); rec = {k_: v for k_, v in rec.items() if k_ not in ('basis_sha256', 'part_profiles')}
rec.update(status=status, inputs=label, input_weights=[wt(x) for x in [g] + extra], p=a.p, a=a.a, b=a.b, t=a.t, k=a.k, D=a.D,
           law=(a.k + 14) // 4, one_in_V=bool(rec.get('profile', [0])[0] == 1) or status == 'one', seconds=round(time.time() - t0, 1),
           engine_sha256=B.ENGINE_SHA256)
json.dump(rec, open(a.out, 'w'), indent=1); print(json.dumps({k_: rec[k_] for k_ in ('p', 'k', 'D', 'inputs', 'rank', 'profile', 'one_in_V', 'status', 'seconds') if k_ in rec}))

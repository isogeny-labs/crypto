#!/usr/bin/env python3
"""Build balanced (n=2k-1) random-base ordinary-curve unit instances (|Z|=0) with the deep chain's builder.
usage: build_balanced.py K COUNT [OUTDIR]   seeds tried: 20261114+t, t=0.. (name fixed per (k,j))."""
import sys, json, time, os
sys.path.insert(0, '/home/user/ml-cryptanalysis/scripts')
import ic_bounds_bin_big_run as R
k, count = int(sys.argv[1]), int(sys.argv[2])
out = sys.argv[3] if len(sys.argv) > 3 else '.'
SEED = 20261114
log = []
t = 0
for j in range(count):
    while True:
        spec = dict(name='B5_k%d_%d' % (k, j), n=2 * k - 1, k=k, base='random', curve='ordinary',
                    target='random-rational', seed=SEED + t, index=0)
        t0 = time.time()
        inst = R.build_instance(spec)
        rec = dict(k=k, j=j, seed=SEED + t, n=inst['n'], Z=inst['n_points'], cls=inst['class'], unit=inst['unit_class'],
                   e=inst['structure']['e'], const=inst['structure']['const'], system_hash=inst['system_hash'], sec=round(time.time() - t0, 1))
        print(json.dumps(rec), flush=True)
        log.append(rec)
        if inst['unit_class']:
            with open(os.path.join(out, spec['name'] + '.json'), 'w') as f:
                json.dump(inst, f, separators=(',', ':'))
            t += 1
            break
        t += 1
        if t > 40:
            sys.exit('no unit instance found')

# smoke, not data: timing and memory probe for the cost estimate (k = 20, 22 tables; N = 290, 579 base evals).
import sys, time, resource, numpy as np, random
sys.path.insert(0, '.'); import l1lib as L
for k in (20, 22):
    t = time.time(); cv = L.Curve(k); tb = time.time() - t
    t = time.time(); o, h = cv.hash_order('key1', 300000); th = time.time() - t
    print(f'k={k} p={cv.p} n/2={cv.n//2} table {tb:.1f}s  hash pass over {cv.M} valid u: {th:.1f}s', flush=True)
for N in (145, 290, 579):
    rng = np.random.default_rng(1); j = rng.integers(1, cv.r, size=5000)
    t = time.time()
    for _ in range(5): cv.eval_base(cv.b_rand(N, rng), j, 1250, want_t0=True)
    print(f'N={N} (on k=22 table): eval_base {(time.time()-t)/5*1000:.0f} ms/base', flush=True)
print('maxrss MB', resource.getrusage(resource.RUSAGE_SELF).ru_maxrss/1024)

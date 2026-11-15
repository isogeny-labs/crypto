#!/usr/bin/env python3
"""Option 3, check 2: can a choice of trial offsets recover BSGS's factor 2 for a table whose logs are known
but random (IC after phase 1)?  Expected hit time for a uniform target x when trial k covers T + w_k:
E = sum_{k>=0} (1 - |U_k| / n), U_k = union of the first k translates.  Strategies: random offsets; greedy
(each step the best of R random candidate offsets by new coverage); the tiled interval table (BSGS).
Pure combinatorics on Z_n (no curve arithmetic); plain sets (the x-only +- symmetry halves n for every
strategy alike)."""
import numpy as np, sys, json
def hit_time(n, T, choose, rng, max_steps):
    U = np.zeros(n, dtype=bool); E = 0.0; covered = 0
    for _ in range(max_steps):
        E += 1 - covered / n
        w = choose(U, T, rng)
        idx = (T + w) % n
        covered += int((~U[idx]).sum()); U[idx] = True
        if covered == n: break
    return E
def run(n, K, R, seed):
    rng = np.random.default_rng(seed)
    T = rng.choice(n, size=K, replace=False)
    rand = lambda U, T, rng: int(rng.integers(n))
    def greedy(U, T, rng):
        ws = rng.integers(n, size=R)
        return int(ws[np.argmin([U[(T + w) % n].sum() for w in ws])])
    steps = 40 * n // K
    e_rand = hit_time(n, T, rand, rng, steps)
    e_greedy = hit_time(n, T, greedy, rng, steps)
    Tint = np.arange(K); k = [0]
    def tile(U, T, rng):
        k[0] += 1; return (k[0] - 1) * K
    e_tile = hit_time(n, Tint, tile, rng, steps)
    return dict(n=n, K=K, R=R, seed=seed, E_random=e_rand, E_greedy=e_greedy, E_tiled=e_tile,
                model_random=n / K, model_tiled=(n / K + 1) / 2)
if __name__ == "__main__":
    rows = []
    for K in (256, 1024, 4096):
        for R in (16, 256):
            for seed in (1, 2, 3):
                r = run(1 << 18, K, R, seed); rows.append(r)
                print(json.dumps({k: (round(v, 2) if isinstance(v, float) else v) for k, v in r.items()}), flush=True)
    json.dump(rows, open("translate_sim.json", "w"), indent=1)

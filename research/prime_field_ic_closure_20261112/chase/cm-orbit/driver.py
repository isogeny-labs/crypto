#!/usr/bin/env python3
"""Driver: curves at 16/18/20/22 bits, IC arms x oracles x F grid, rho arms. Raw rows -> raw.jsonl.
[pcc:unverified]"""
import json, math, random, sys, time, os
from multiprocessing import Pool
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from cmorbit import *

OUT = os.path.dirname(os.path.abspath(__file__))
BITS = [16, 18, 20, 22]
CURVES_PER = 2
GRID = [0.5, 0.71, 1.0, 1.41, 2.0]
IC_SEEDS = 16
SEM_SEEDS = {16: 10, 18: 8, 20: 6, 22: 4}
RHO_WALKS = 120
ARMS = ["fold", "foldAA", "cmorb", "foldcm"]

def curves():
    out = []
    for bits in BITS:
        for ci in range(CURVES_PER):
            rng = random.Random(1000*bits + ci)
            E = find_curve(bits, rng)
            cs = setup(E, rng)
            out.append((bits, ci, E.p, E.b, E.n, cs))
    return out

def job(args):
    kind, bits, ci, p, b, n, cs, arm, oracle, F, seed, fullrank = args
    E = Curve(p, b, n)
    t = time.time()
    if kind == "ic":
        real_arm = "fold" if arm == "foldAA" else arm
        # A/A arm: same arm, independent seed stream
        sd = seed + (7919 if arm == "foldAA" else 0) + 104729*ci + 1299709*bits + (0 if oracle == "table" else 15485863) + 31*F
        r = run_ic(E, real_arm, oracle, F, random.Random(sd), cs, fullrank=fullrank)
        r["arm"] = arm
    else:
        r = run_rho(E, arm, random.Random(seed + 104729*ci + 1299709*bits + 17), cs)
        r["arm"] = arm
    r.update(kind=kind, bits=bits, curve=ci, p=p, b=b, n=n, oracle=oracle, Fgrid=F, seed=seed,
             cI=E.cost_I, cL=E.cost_L, cR=E.cost_R, wall=round(time.time()-t, 3))
    return r

def main():
    t0 = time.time()
    cv = curves()
    meta = [dict(bits=b, curve=ci, p=p, b=bb, n=n, s=cs[2], lam=cs[3], P=cs[0], Q=cs[1]) for (b, ci, p, bb, n, cs) in cv]
    json.dump(meta, open(os.path.join(OUT, "curves.json"), "w"), indent=1)
    jobs = []
    for (bits, ci, p, b, n, cs) in cv:
        F0 = (2*n) ** (1/3)
        for g in GRID:
            F = max(12, int(round(F0*g/6))*6)
            for arm in ARMS:
                for sd in range(IC_SEEDS):
                    jobs.append(("ic", bits, ci, p, b, n, cs, arm, "table", F, sd, g == 1.0))
        F = max(12, int(round(F0/6))*6)
        for arm in ARMS:
            for sd in range(SEM_SEEDS[bits]):
                jobs.append(("ic", bits, ci, p, b, n, cs, arm, "semaev", F, sd, False))
        for cls in ("plain", "neg", "aut6"):
            for sd in range(RHO_WALKS):
                jobs.append(("rho", bits, ci, p, b, n, cs, cls, None, None, sd, False))
    # big jobs first
    jobs.sort(key=lambda j: (-(j[1]), j[8] != "semaev"))
    with Pool(4) as pool, open(os.path.join(OUT, "raw.jsonl"), "w") as f:
        for i, r in enumerate(pool.imap_unordered(job, jobs, chunksize=4)):
            f.write(json.dumps(r) + "\n")
            if i % 500 == 0: print(i, len(jobs), round(time.time()-t0, 1), flush=True)
    print("done", len(jobs), "jobs", round(time.time()-t0, 1), "s")

if __name__ == "__main__":
    main()

#!/usr/bin/env python3
"""Supplemental rho walks (neg, aut6, plain) on the same curves, independent seeds, more walks,
to tighten the rho ratio.  Appends to raw_rho_more.jsonl.  [pcc:unverified]"""
import json, random, sys, os, time
from multiprocessing import Pool
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from cmorbit import *
OUT = os.path.dirname(os.path.abspath(__file__))
WALKS = int(sys.argv[1]) if len(sys.argv) > 1 else 600

def job(a):
    m, cls, sd = a
    E = Curve(m["p"], m["b"], m["n"])
    cs = (tuple(m["P"]), tuple(m["Q"]), m["s"], m["lam"])
    r = run_rho(E, cls, random.Random(10**7 + sd + 104729*m["curve"] + 1299709*m["bits"]), cs)
    r.update(kind="rho", arm=cls, bits=m["bits"], curve=m["curve"], p=m["p"], b=m["b"], n=m["n"],
             oracle=None, Fgrid=None, seed=10**7+sd, cI=E.cost_I, cL=E.cost_L, cR=E.cost_R)
    return r

if __name__ == "__main__":
    meta = json.load(open(os.path.join(OUT, "curves.json")))
    jobs = [(m, cls, sd) for m in meta for cls in ("plain", "neg", "aut6") for sd in range(WALKS)]
    t = time.time()
    with Pool(4) as pool, open(os.path.join(OUT, "raw_rho_more.jsonl"), "w") as f:
        for r in pool.imap_unordered(job, jobs, chunksize=16):
            f.write(json.dumps(r) + "\n")
    print("done", len(jobs), round(time.time()-t, 1))

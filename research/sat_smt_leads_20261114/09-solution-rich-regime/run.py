#!/usr/bin/env python3
"""usage: run.py n targets_per_cell out.jsonl l1 l2 ...   (one core)"""
import sys, time, json, random
from pysat.solvers import Solver
from pdp import Field, Curve, Truth, Enc, lift_ok

CAP = 400_000
KS = [2, 4, 8]
n = int(sys.argv[1]); NT = int(sys.argv[2]); outp = sys.argv[3]; LS = [int(a) for a in sys.argv[4:]]
F = Field(n); E = Curve(F)

def solve(name, clauses, enc, R):
    t = time.time()
    with Solver(name=name, bootstrap_with=clauses) as s:
        if name == "glucose4":
            s.conf_budget(CAP)
            res = s.solve_limited()
        else:
            res = s.solve()
        st = s.accum_stats()
        lifted = None
        if res:
            lifted = lift_ok(E, enc.xs_from_model(s.get_model()), R)
    return dict(result=("SAT" if res else "UNSAT" if res is False else "CENSORED"), conflicts=st["conflicts"],
                decisions=st["decisions"], propagations=st["propagations"], restarts=st["restarts"],
                lifted=lifted, wall=round(time.time() - t, 3))

with open(outp, "a") as fo:
    for l in LS:
        tr = Truth(E, l)
        for i in range(NT):
            seed = 1000003 * n + 1009 * l + i
            rng = random.Random(seed)
            R = E.random_point(rng)
            while R[0] == 0:
                R = E.random_point(rng)
            enc = Enc(F, l, R[0])
            base = dict(n=n, l=l, target=i, seed=seed, xR=R[0], nvars=enc.nv, nclauses=len(enc.clauses), **tr.query(R))
            rows = [dict(base, arm="a_glucose", k=0, **solve("glucose4", enc.clauses, enc, R))]
            if l <= n // 3 + 1:
                rows.append(dict(base, arm="a_cadical", k=0, **solve("cadical153", enc.clauses, enc, R)))
            for k in KS:
                extra = enc.streamline(k, random.Random(seed * 31 + k))
                rows.append(dict(base, arm="c_streamline", k=k, **solve("glucose4", enc.clauses + extra, enc, R)))
            for r in rows:
                fo.write(json.dumps(r) + "\n")
            fo.flush()

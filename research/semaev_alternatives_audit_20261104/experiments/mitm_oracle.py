"""Baseline m=4 (S_5) decomposition oracle: 2+2 meet-in-the-middle.

A 4-sum over a generic set of size B costs O(B^2) group operations: tabulate
all pair sums, then for every pair (k,l) look up x(R - P_k - P_l).  The
full-scan cost is measured (no early exit), so the fitted exponent is not
truncated by the match position.

s = 2 at m = 4.  Corrected win gate: s < m-3 (s < 1 at m = 4).  The document's
stated gate "e < 1 at m = 4" is passed by this generic oracle (e = 2/5 at
B = p^(1/5)), which is why the gate has to be fixed.

All group operations go through lib.ec.Curve, so the run is priced with the
same counters as the rest of the audit.
"""
import json
import math
import random
import sys
import time

import sympy as sp

from lib.accounting import Record, write_jsonl
from lib.ec import Curve, Counter


def mitm_full(E, base, R):
    tab = {}
    ops = 0
    n = len(base)
    for i in range(n):
        Pi = base[i]
        for j in range(i + 1, n):
            S = E.add(Pi, base[j]); ops += 1
            tab.setdefault(S[0], S[1])
    found = False
    for k in range(n):
        Rk = E.add(R, E.neg(base[k]))
        for l in range(k + 1, n):
            T = E.add(Rk, E.neg(base[l])); ops += 1
            if T is None:
                continue
            y = tab.get(T[0])
            if y is not None and (y == T[1] or y == (-T[1]) % E.p):
                found = True
    return found, ops


def run(pbits=40, deltas=(0.10, 0.14, 0.16, 0.18, 0.20, 0.22, 0.24), trials=3, seed=11):
    rng = random.Random(seed)
    p = int(sp.nextprime(2**pbits))
    counter = Counter()
    E = Curve(p, 0, 7, counter)
    rows, recs = [], []
    for d in deltas:
        B = int(round(p ** d))
        base = E.points_with_x_upto(B, rng)
        if len(base) < 8:
            continue
        best = None
        for _ in range(trials):
            Ps = rng.sample(base, 4)
            R = None
            for P in Ps:
                R = E.add(R, P)
            t0 = time.time()
            ok, ops = mitm_full(E, base, R)
            dt = time.time() - t0
            if best is None or ops > best[1]:
                best = (dt, ops, ok)
        rows.append({"delta": d, "B": B, "base": len(base), "found": best[2],
                     "ops": best[1], "seconds": round(best[0], 4)})
        recs.append(Record(
            experiment="mitm_m4",
            config={"p_bits": pbits, "delta": d, "trials": trials},
            counts={"group_add": best[1] * trials},
            metrics={"B": B, "base": len(base), "found": best[2], "ops": best[1]},
            seconds=best[0],
        ))
        print(rows[-1], flush=True)
    return p, rows, recs


def fit(rows, key):
    xs = [math.log(r["B"]) for r in rows]
    ys = [math.log(max(r[key], 1e-9)) for r in rows]
    n = len(xs); mx = sum(xs)/n; my = sum(ys)/n
    return sum((x-mx)*(y-my) for x, y in zip(xs, ys)) / sum((x-mx)**2 for x in xs)


if __name__ == "__main__":
    p, rows, recs = run()
    out = {"p_bits": 40, "rows": rows,
           "fitted_s_ops": round(fit(rows, "ops"), 3),
           "fitted_s_time": round(fit(rows, "seconds"), 3),
           "gate_old": "e<1 at m=4 -> e at B=p^(1/5) is 2/5 = 0.4 (passes)",
           "gate_new": "s < m-3 = 1 (fails)"}
    json.dump(out, open(sys.argv[1] if len(sys.argv) > 1 else "mitm_m4.json", "w"), indent=1)
    write_jsonl("mitm_m4.jsonl", recs)
    print("fitted s (ops) =", out["fitted_s_ops"])

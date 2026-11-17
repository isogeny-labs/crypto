#!/usr/bin/env sage -python
"""Matched-source Floyd rho controls for the 2027-subgroup target."""
import hashlib
import json
import random
import statistics
import time
from pathlib import Path

from sage.all import EllipticCurve

from nonsplit import L, code, dec

HERE = Path(__file__).resolve().parent
source = json.loads((HERE/"nonsplit_4054.json").read_text())
K = L.subfield(12, "w")
cv = lambda n: K(dec(n))
E = EllipticCurve(K, [K(1), cv(source["source_a"]), K(0), K(0),
                      cv(source["source_b"])])
G = E(*map(cv, source["source_G"]))
Q = E(*map(cv, source["source_Q"]))
r = source["subgroup_prime"]
assert r*G == E(0) and Q == 7*G


def step(state):
    P, a, b = state
    h = 0 if P == E(0) else code(L(P[0])) % 3
    if h == 0:
        return P+G, (a+1)%r, b
    if h == 1:
        return P+Q, a, (b+1)%r
    return 2*P, (2*a)%r, (2*b)%r


def solve(seed):
    rng = random.Random(seed)
    a, b = rng.randrange(r), rng.randrange(r)
    initial = (a*G+b*Q, a, b)
    tortoise = step(initial)
    hare = step(step(initial))
    calls = 3
    while tortoise[0] != hare[0]:
        tortoise = step(tortoise)
        hare = step(step(hare))
        calls += 3
        assert calls < 50*r
    denominator = (hare[2]-tortoise[2])%r
    if denominator == 0:
        return {"seed": seed, "step_calls": calls, "collision": "singular"}
    answer = ((tortoise[1]-hare[1])*pow(denominator,-1,r))%r
    assert answer*G == Q and answer == 7
    return {"seed": seed, "step_calls": calls, "collision": "solved",
            "answer": answer}


start = time.perf_counter()
runs = [solve(seed) for seed in range(100)]
seconds = time.perf_counter()-start
solved = [row["step_calls"] for row in runs if row["collision"] == "solved"]
result = {"schema": "ghs-nonsplit-matched-rho-v1",
          "script_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
          "source_sha256": hashlib.sha256((HERE/"nonsplit_4054.json").read_bytes()).hexdigest(),
          "subgroup_prime": r, "target": source["source_Q"],
          "runs": runs, "solved_runs": len(solved),
          "median_group_step_calls": statistics.median(solved),
          "mean_group_step_calls": statistics.mean(solved),
          "total_seconds": seconds}
(HERE/"rho_4054.json").write_text(json.dumps(result,indent=2)+"\n")
print(json.dumps({k:result[k] for k in
                  ("solved_runs","median_group_step_calls",
                   "mean_group_step_calls","total_seconds")},indent=2))

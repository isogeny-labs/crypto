#!/usr/bin/env python3
"""Replay and certificate sample, drawn from a fresh nonce (os.urandom) recorded in out/nonce.txt
before any replay runs.  Every draw is random.Random(sha256(nonce + label)).  Writes out/plan.json.
  A:   one (curve, k) cell per cc in {1.4, 2.0, 2.8}, all four SGE settings, seeds 1-2 -> 8 rows per cell
  B:   per prime and arm (x: cc 2.0; x*: cc 1.4; SGE 4,1.5), 2 distinct curves, 3 consecutive targets
       from k0, seeds 1-2 -> 12 rows per (prime, arm)
  Q:   one of the five p = 16411 plain-rho rows
  S:   two p = 32771 search chunks (files with >= 16 seeds), plus the chunk that admitted 2374 (fixed)
  ord: 8 irreducible seeds per prime, below the fifth chosen seed, rejected as order_not_near_prime
"""
import hashlib, json, os, random, sys, datetime
sys.path.insert(0, os.path.dirname(__file__))
from common import OUT, dump

nf = os.path.join(OUT, "nonce.txt")
if not os.path.exists(nf):
    with open(nf, "w") as fh:
        fh.write(os.urandom(16).hex() + " " + datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ") + "\n")
nonce = open(nf).read().split()[0]
R = lambda label: random.Random(int(hashlib.sha256((nonce + "|" + label).encode()).hexdigest(), 16))

curves = {16411: [373, 455, 548, 752, 875], 32771: [355, 882, 1461, 1623, 2374]}
plan = {"nonce": nonce, "A": [], "B": [], "Q": None, "S": [], "ord": {}}
for cc in ("1.4", "2.0", "2.8"):
    r = R(f"A{cc}")
    plan["A"].append({"cc": cc, "cs": r.choice([172, 180]), "k": r.randrange(3)})
for p in (16411, 32771):
    for arm, cc in (("x", "2.0"), ("xstar", "1.4")):
        r = R(f"B{p}{arm}")
        for cs in r.sample(curves[p], 2):
            plan["B"].append({"p": p, "arm": arm, "cc": cc, "cs": cs, "k0": r.randrange(4), "targets": 3})
plan["Q"] = {"p": 16411, "cs": R("Q").choice(curves[16411]), "k": 0, "seed": 1}
sel = json.load(open(os.path.join(OUT, "select_check.json")))
chunks = sorted(n for n, e in sel["files"].items() if n.startswith("search_p32771_r") and e["n_list"] >= 16)
plan["S"] = R("S").sample(chunks, 2)
fixed = "search_p32771_r2201-2600-all_w1_c28.jsonl.gz"
if fixed not in plan["S"]:
    plan["S"].append(fixed)
red = json.load(open(os.path.join(OUT, "seed_reducibility.json")))
for p in (16411, 32771):
    fifth = curves[p][-1]
    adm = set(sel["primes"][str(p)]["admitted"])
    pool = [s for s in range(1, fifth) if red[f"{p}:{s}"] == 0 and s not in adm]
    plan["ord"][str(p)] = {"pool": len(pool), "drawn": sorted(R(f"ord{p}").sample(pool, 8))}
dump("plan.json", plan)
print(json.dumps(plan, indent=1))

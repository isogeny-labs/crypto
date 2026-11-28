#!/usr/bin/env python3
"""C2 (implementation): differential test of the library's f_irreducible (val_po 'firr') against the validator's
DDF on monic degree-9 f at p = 16411 and 32771 (validator RNG 'val4po-diff', independent of the curve seeds):
  - 150 irreducible f per prime (rejection-sampled; the harmful direction: an irreducible f called reducible
    would skip a possible prime-order seed);
  - factor patterns built from random irreducibles: (3,3,3) (x^(p^9) = x holds, only the gcd with
    x^(p^3) - x can see it), (1,8), (2,7), (4,5), (3,6), (2,2,5), (1,1,7), (1^9), and non-squarefree g^2 h;
  - 300 uniformly random f per prime.
Writes out/prefilter_diff.json."""
import json, os, random, subprocess, sys
sys.path.insert(0, os.path.dirname(__file__))
from common import OUT, dump
from jac import pmul, trim
from polyfp import reducible_certificate, squarefree

H = "/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/val4po/bin/val_po"
rnd = random.Random("val4po-diff")


def rand_monic(p, d):
    return [rnd.randrange(p) for _ in range(d)] + [1]


def irreducible(p, d):
    while True:
        f = rand_monic(p, d)
        if d == 1 or (squarefree(f, p) and reducible_certificate_deg(f, p) is None):
            return f


def reducible_certificate_deg(f, p):
    from polyfp import ddf
    parts = ddf(f, p)
    return None if (len(parts) == 1 and parts[0][0] == len(f) - 1) else parts


def product(p, degs):
    while True:
        fs = [tuple(irreducible(p, d)) for d in degs]
        if len(set(fs)) == len(fs):
            out = (1,)
            for g in fs:
                out = pmul(out, g, p)
            return list(out)


cases = []
for p in (16411, 32771):
    for _ in range(150):
        cases.append(("irreducible", p, irreducible(p, 9), True))
    for degs in ((3, 3, 3), (1, 8), (2, 7), (4, 5), (3, 6), (2, 2, 5), (1, 1, 7), (1,) * 9):
        for _ in range(12):
            cases.append(("x".join(map(str, degs)), p, product(p, degs), False))
    for _ in range(12):
        g = irreducible(p, 2)
        h = irreducible(p, 5)
        cases.append(("nonsquarefree g^2 h", p, list(pmul(pmul(g, g, p), h, p)), False))
    for _ in range(300):
        f = rand_monic(p, 9)
        cases.append(("random", p, f, None))
inp = "".join(f"firr {p} {' '.join(map(str, f))}\n" for _, p, f, _ in cases)
outp = subprocess.run([H], input=inp, capture_output=True, text=True, check=True).stdout.splitlines()
assert len(outp) == len(cases)
res = {"cases": len(cases), "by_kind": {}, "disagree": []}
for (kind, p, f, truth), line in zip(cases, outp):
    r = json.loads(line)
    assert r["f"] == f and r["p"] == p
    mine = reducible_certificate(f, p, rnd) is None if squarefree(f, p) else False
    if truth is not None and mine != truth:
        res["disagree"].append({"kind": kind, "p": p, "f": f, "why": "validator DDF disagrees with construction"})
    b = res["by_kind"].setdefault(kind, {"n": 0, "rust_irr": 0, "validator_irr": 0, "agree": 0})
    b["n"] += 1
    b["rust_irr"] += r["irr"]
    b["validator_irr"] += mine
    b["agree"] += r["irr"] == mine
    if r["irr"] != mine:
        res["disagree"].append({"kind": kind, "p": p, "f": f, "rust": r["irr"], "validator": mine})
res["ok"] = not res["disagree"]
dump("prefilter_diff.json", res)
print(json.dumps(res["by_kind"], indent=0), "ok", res["ok"])

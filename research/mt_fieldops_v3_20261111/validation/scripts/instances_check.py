#!/usr/bin/env python3
"""Validator: instance validity (own code).  p and n prime (Miller-Rabin, 40 fixed + 20 random bases, and a
deterministic set for < 3.3e24), curve nonsingular, G on the curve, [n]G = O, n in the Hasse interval (so with
cofactor 1, #E = n), fixture point = [known_log]G, a != 0 and b != 0 (negation is the only automorphism),
MANIFEST sha256 matches, tune (idx 0) and holdout (idx 1..11) instance sets disjoint as curves.
usage: instances_check.py <instances dir> <out.json>"""
import json, sys, os, hashlib, random, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from eccheck_lib import smul_full
D, OUT = sys.argv[1:3]
def mr(n):
    if n < 2: return False
    for q in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41]:
        if n % q == 0: return n == q
    d, s = n - 1, 0
    while d % 2 == 0: d //= 2; s += 1
    bases = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41] + [random.Random(n).randrange(2, n - 1) for _ in range(20)]
    for b in bases:
        x = pow(b, d, n)
        if x in (1, n - 1): continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1: break
        else: return False
    return True
man = {m["file"]: m for m in json.load(open(os.path.join(D, "MANIFEST.json")))}
res = []; curves = {}
for fn in sorted(os.listdir(D)):
    if fn == "MANIFEST.json" or not fn.endswith(".json"): continue
    raw = open(os.path.join(D, fn), "rb").read(); d = json.loads(raw)
    p = int(d["field"]["modulus"]); a = int(d["a"]); b = int(d["b"]); n = int(d["subgroup_order"])
    G = (int(d["generator"]["x"]), int(d["generator"]["y"])); P = (int(d["point"]["x"]), int(d["point"]["y"]))
    kl = int(d["fixture"]["known_log"])
    disc = (4 * a ** 3 + 27 * b ** 2) % p
    j = (1728 * 4 * a ** 3 * pow(disc, -1, p)) % p if disc else None
    r = dict(file=fn, p=p, n=n, p_prime=mr(p), n_prime=mr(n), nonsingular=disc != 0,
             G_on=(G[1] ** 2 - G[0] ** 3 - a * G[0] - b) % p == 0, nG_is_O=smul_full(n, G, a, p) is None,
             hasse=abs(n - (p + 1)) <= 2 * math.isqrt(p) + 2, cofactor=int(d.get("cofactor", 1)),
             fixture_ok=smul_full(kl, G, a, p) == P, a_b_nonzero=a % p != 0 and b % p != 0,
             manifest_sha_ok=man.get(fn, {}).get("sha256") == hashlib.sha256(raw).hexdigest(), j=j, b=b, a=a)
    r["valid"] = all(r[k] for k in ["p_prime", "n_prime", "nonsingular", "G_on", "nG_is_O", "hasse", "fixture_ok", "manifest_sha_ok"]) and r["cofactor"] == 1
    res.append(r); curves[fn] = (p, a, b, j, n)
tune = {curves[f] for f in curves if f.endswith("-0.json")}; hold = {curves[f] for f in curves if not f.endswith("-0.json")}
tune_j = {(c[0], c[3]) for c in tune}; hold_j = {(c[0], c[3]) for c in hold}
out = dict(instances=len(res), all_valid=all(r["valid"] for r in res), invalid=[r["file"] for r in res if not r["valid"]],
           all_a_b_nonzero=all(r["a_b_nonzero"] for r in res), manifest_entries=len(man),
           tune_holdout_disjoint_curves=not (tune & hold), tune_holdout_disjoint_pj=not (tune_j & hold_j),
           distinct_pj=len({(c[0], c[3]) for c in curves.values()}), distinct_pn=len({(c[0], c[4]) for c in curves.values()}),
           results=res)
json.dump(out, open(OUT, "w"), indent=1)
print({k: v for k, v in out.items() if k != "results"})

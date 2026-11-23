#!/usr/bin/env python3
"""Validator check of the 33 v4 instances: hashes vs MANIFEST, p prime, curve nonsingular, G and the
fixture point on the curve, n prime, n G = O, cofactor 1 (n inside the Hasse interval and the only
multiple of n there), fixture known_log, freshness: (p, a, b) and (p, n) against every other curve JSON
under research/ (and docs/curves), counting the files scanned.
usage: instances_check.py <repo> <frozen study dir> <out.json>"""
import json, sys, os, glob, hashlib, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from ecv import Curve, is_prime, load
repo, study, out = sys.argv[1:4]
inst = os.path.join(study, "instances")
man = json.load(open(os.path.join(inst, "MANIFEST.json")))
res = []; keys = set(); keys_pn = set()
for m in man:
    path = os.path.join(inst, m["file"]); raw = open(path, "rb").read()
    d, C, G, n = load(path); p = C.p
    r = dict(file=m["file"], sha_ok=hashlib.sha256(raw).hexdigest() == m["sha256"], p=p, n=n,
             bits_n=n.bit_length(), p_prime=is_prime(p), n_prime=is_prime(n),
             nonsingular=(4 * C.a ** 3 + 27 * C.b ** 2) % p != 0, G_on=C.on(G),
             nG_is_O=C.mul(n, G) is None, cofactor=int(d["cofactor"]), group_order_eq_n=int(d["group_order"]) == n)
    lo, hi = p + 1 - 2 * math.isqrt(p) - 2, p + 1 + 2 * math.isqrt(p) + 2
    r["hasse"] = lo <= n <= hi
    r["only_multiple_in_hasse"] = sum(1 for k in range(1, 4) if lo <= k * n <= hi) == 1 and (2 * n > hi)
    P = (int(d["point"]["x"]), int(d["point"]["y"]))
    r["fixture_ok"] = C.on(P) and C.mul(int(d["fixture"]["known_log"]), G) == P
    res.append(r); keys.add((p, C.a, C.b)); keys_pn.add((p, n))
# freshness scan
scanned = 0; overlap = []
for f in glob.glob(os.path.join(repo, "research", "**", "*.json"), recursive=True) + glob.glob(os.path.join(repo, "docs/curves/*.json")):
    if "/mt_fieldops_v4_20261112/" in f: continue
    try:
        if os.path.getsize(f) > 2_000_000: continue
        d = json.load(open(f))
    except Exception:
        continue
    items = d if isinstance(d, list) else [d]
    for it in items:
        if not isinstance(it, dict) or "a" not in it or "b" not in it: continue
        try:
            fld = it.get("field"); p = int(fld["modulus"]) if isinstance(fld, dict) else int(it.get("p"))
            a, b = int(it["a"]) % p, int(it["b"]) % p
        except Exception:
            continue
        scanned += 1
        if (p, a, b) in keys: overlap.append(f)
summ = dict(instances=len(res), all_ok=all(all(v for k, v in r.items() if isinstance(v, bool)) and r["cofactor"] == 1 for r in res),
            bits=sorted({r["bits_n"] for r in res}), n_range={b: [min(r["n"] for r in res if r["bits_n"] == b), max(r["n"] for r in res if r["bits_n"] == b)] for b in sorted({r["bits_n"] for r in res})},
            p_values=sorted({r["p"] for r in res}), other_curve_records_scanned=scanned, overlap_files=overlap)
json.dump(dict(summary=summ, instances=res), open(out, "w"), indent=1)
print(json.dumps(summ, indent=1))

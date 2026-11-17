#!/usr/bin/env python3
"""VALIDATOR check of the 33 v5 instances: sha256 vs MANIFEST; p, n prime; nonsingular; G and fixture point on the
curve; n G = O; n the only multiple of n in the Hasse interval (so #E = n, cofactor 1); fixture known_log; and
freshness: (p, a mod p, b mod p) against every other curve-like JSON record under research/ and docs/curves.
usage: instances_check.py <repo> <frozen study dir> <out.json>"""
import json, sys, os, glob, hashlib, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from ec import Curve, is_prime, load
repo, study, out = sys.argv[1:4]
inst = os.path.join(study, "instances"); man = json.load(open(os.path.join(inst, "MANIFEST.json")))
res = []; keys = set(); keys_pn = set()
for m in man:
    path = os.path.join(inst, m["file"]); raw = open(path, "rb").read()
    d, C, G, n = load(path); p = C.p
    lo, hi = p + 1 - 2 * math.isqrt(p) - 2, p + 1 + 2 * math.isqrt(p) + 2
    P = (int(d["point"]["x"]), int(d["point"]["y"]))
    r = dict(file=m["file"], sha_ok=hashlib.sha256(raw).hexdigest() == m["sha256"], p=p, n=n, bits_n=n.bit_length(),
             p_prime=is_prime(p), n_prime=is_prime(n), nonsingular=(4 * C.a ** 3 + 27 * C.b ** 2) % p != 0,
             G_on=C.on(G), nG_is_O=C.mul(n, G) is None, cofactor_field_1=int(d["cofactor"]) == 1,
             group_order_eq_n=int(d["group_order"]) == n, hasse=lo <= n <= hi, unique_multiple=2 * n > hi,
             fixture_ok=C.on(P) and C.mul(int(d["fixture"]["known_log"]), G) == P,
             fixture_seed=d["fixture"]["seed"])
    res.append(r); keys.add((p, C.a, C.b)); keys_pn.add((p, n))
scanned = 0; files = 0; overlap = []; overlap_pn = []
for f in glob.glob(os.path.join(repo, "research", "**", "*.json"), recursive=True) + glob.glob(os.path.join(repo, "docs/curves/*.json")):
    if "/mt_fieldops_v5_20261112/" in f: continue
    try:
        if os.path.getsize(f) > 4_000_000: continue
        d = json.load(open(f))
    except Exception:
        continue
    files += 1
    stack = [d]
    while stack:
        it = stack.pop()
        if isinstance(it, list): stack.extend(x for x in it if isinstance(x, (list, dict))); continue
        if not isinstance(it, dict): continue
        for v in it.values():
            if isinstance(v, (list, dict)): stack.append(v)
        if "a" not in it or "b" not in it: continue
        try:
            fld = it.get("field"); p = int(fld["modulus"]) if isinstance(fld, dict) else int(it.get("p") or it.get("modulus"))
            a, b = int(it["a"]) % p, int(it["b"]) % p
        except Exception:
            continue
        scanned += 1
        if (p, a, b) in keys: overlap.append(f)
        for nk in ("subgroup_order", "n", "order"):
            try:
                if (p, int(it[nk])) in keys_pn: overlap_pn.append(f)
            except Exception:
                pass
summ = dict(instances=len(res), all_ok=all(all(v for k, v in r.items() if isinstance(v, bool)) for r in res),
            bits=sorted({r["bits_n"] for r in res}),
            n_bits_by_file={r["file"]: r["bits_n"] for r in res},
            p_values=sorted({r["p"] for r in res}), json_files_scanned=files, curve_records_scanned=scanned,
            overlap_files_pab=sorted(set(overlap)), overlap_files_pn=sorted(set(overlap_pn)))
json.dump(dict(summary=summ, instances=res), open(out, "w"), indent=1)
print(json.dumps({k: v for k, v in summ.items() if k != "n_bits_by_file"}, indent=1))

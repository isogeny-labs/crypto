#!/usr/bin/env python3
"""Check W5-SYN-3 (solving degree vs largest prime factor of d) against the recorded exponent-study cells.
Reads only committed cell files; prints one row per complete sym cell and the preregistered k = 22 check."""
import glob, json, os
R = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..")
def lpf(n):
    f, b = 2, 1
    while n > 1:
        while n % f == 0:
            b, n = f, n // f
        f += 1
    return b
rows = []
for f in sorted(glob.glob(os.path.join(R, "curve25519_exponent_study_20261108/cells/k2*_m[23]_sym_d*_g0.json"))) + \
         sorted(glob.glob(os.path.join(R, "curve25519_exponent_study_ext_20261110/cells/*.json"))):
    j = json.load(open(f))
    if j["capped"] or not j.get("per_target"):
        continue
    d = j["d_or_t"]; sd = max(r["solving_degree_max"] for r in j["per_target"])
    rows.append((j["k"], j["m"], d, lpf(d), sd))
for r in rows:
    print("k=%d m=%d d=%d lpf=%d SD=%d SD-lpf=%d" % (r + (r[4] - r[3],)))
prime_ok = all(sd == d + 1 for k, m, d, e, sd in rows if e == d)
print("prime d: SD = d + 1 in all", sum(1 for r in rows if r[3] == r[2]), "cells:", prime_ok)
prereg = {3: (4, 4), 5: (6, 6), 11: (12, 12), 31: (32, 32), 41: (42, 42), 15: (8, 10), 25: (8, 10), 33: (14, 16)}
got = {d: sd for k, m, d, e, sd in rows if k == 22 and m == 3}
hits = [(d, prereg[d], got.get(d), prereg[d][0] <= got.get(d, -1) <= prereg[d][1]) for d in sorted(prereg)]
for h in hits:
    print("k22 m3 d=%d prereg %s recorded %s ok=%s" % h)
print("preregistered k22 m3:", sum(h[3] for h in hits), "of", len(hits))

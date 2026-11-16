#!/usr/bin/env python3
"""L1: faithful-member cells against the smooth-family records at the same (m, D) (any k; binary dc2c30a4),
and the predictions P1-P4 of PROTOCOL.md."""
import glob, json, os
H = os.path.dirname(os.path.abspath(__file__)); R = os.path.join(H, "..", "..")
rec = {}
for f in glob.glob(os.path.join(R, "curve25519_exponent_study_20261108/cells/k*_m[23]_sym_d*_g0.json")) + \
         glob.glob(os.path.join(R, "curve25519_exponent_study_ext_20261110/cells/k*_m[23]_sym_d*_g0.json")):
    j = json.load(open(f))
    if not j["capped"] and j.get("per_target"):
        rec.setdefault((j["m"], j["d_or_t"]), []).append((j["k"], j["mean_field_ops"], max(t["solving_degree_max"] for t in j["per_target"])))
cells = {}
for f in sorted(glob.glob(os.path.join(H, "cells", "*.json"))):
    j = json.load(open(f)); cells[(j["m"], j["d_or_t"])] = j
out = []
for (m, d), j in sorted(cells.items()):
    if j.get("capped"):
        print(f"m={m} d={d}: censored ({j.get('failure')}, {j['ext']['wall_s']} s)"); out.append((m, d, None, None, None)); continue
    T = j["mean_field_ops"]; sd = max(t["solving_degree_max"] for t in j["per_target"])
    comps = rec.get((m, d), [])
    txt = ", ".join(f"k{k}: {t:.4g} (x{T / t:.3f})" for k, t, s in comps) or "no record at this (m, D)"
    print(f"m={m} d={d:3d}: faithful T={T:.4g} SD={sd} | smooth-family records: {txt}")
    out.append((m, d, T, sd, comps))
def within2(m, d):
    c = [t for k, t, s in rec.get((m, d), [])]
    T = cells[(m, d)]["mean_field_ops"] if (m, d) in cells and not cells[(m, d)].get("capped") else None
    return T is not None and c and all(0.5 <= T / t <= 2 for t in c)
print("\nP1 (D = 3, 5 within 2x at both m):", all(within2(m, d) for m in (2, 3) for d in (3, 5)))
print("P2 (SD = D + 1 at D = 3, 5):", all((m, d) in cells and not cells[(m, d)].get("capped") and max(t["solving_degree_max"] for t in cells[(m, d)]["per_target"]) == d + 1 for m in (2, 3) for d in (3, 5)))
m3 = [(d, j) for (m, d), j in cells.items() if m == 3]
cens = [d for d, j in m3 if j.get("capped")]
done9 = sorted(d for d, j in m3 if not j.get("capped") and d >= 9)
print("P3 (m = 3 chain censors at or before 60; estimator inapplicable):", "censored at", cens, "| complete d>=9:", done9, "| span", (done9[-1] / done9[0] if done9 else 0), "| applicable:", len(done9) >= 4 and done9 and done9[-1] / done9[0] >= 4)
print("P4 (m = 3, D = 12 within 2x of 7.75e8):", within2(3, 12), cells.get((3, 12), {}).get("mean_field_ops"))
json.dump({"rows": out}, open(os.path.join(H, "analysis.json"), "w"), indent=1, default=str)

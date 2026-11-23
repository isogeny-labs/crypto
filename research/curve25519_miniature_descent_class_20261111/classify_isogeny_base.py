#!/usr/bin/env python3
"""See PROTOCOL_isogeny.md. Classify tower_dumps/v_t*_seed*.json (V = x(R'+<Q>) from the real
isogeny_tower code, p = 1048573) by the 2-descent class on the miniature (A = 926).
Run: python3 -I classify_isogeny_base.py > isogeny_classes.out.txt"""
import glob, json, os
p, A = 1048573, 926
inv = lambda v: pow(v, p - 2, p)
leg = lambda v: 0 if v % p == 0 else (1 if pow(v % p, (p - 1) // 2, p) == 1 else -1)
shift = A * inv(3) % p
rhs = lambda xm: (xm * xm * xm + A * xm * xm + xm) % p
rows = []
for f in sorted(glob.glob(os.path.join(os.path.dirname(os.path.abspath(__file__)), "tower_dumps", "v_t*_seed*.json"))):
    d = json.load(open(f))
    on = []
    for xw in d["v"]:
        xm = (xw - shift) % p
        if leg(rhs(xm)) >= 0: on.append(xm)
    plus = sum(1 for xm in on if (leg(xm) if xm else leg(A * A - 4)) == 1)
    minus = len(on) - plus
    rows.append({"t": d["t"], "seed": d["seed"], "V": len(d["v"]), "on_curve": len(on), "plus": plus, "minus": minus})
rows.sort(key=lambda r: (r["t"], r["seed"]))
print("t seed |V| on_curve delta=+1 delta=-1 minority")
for r in rows:
    n = r["on_curve"]; m = min(r["plus"], r["minus"]) / n if n else float("nan")
    print("%d %4d %4d %8d %8d %8d  %.2f" % (r["t"], r["seed"], r["V"], n, r["plus"], r["minus"], m))
for t in (5, 6):
    rs = [r for r in rows if r["t"] == t and r["seed"] >= 1]
    P = sum(r["plus"] for r in rs); M = sum(r["minus"] for r in rs)
    print("t=%d seeds 1-%d pooled: delta=+1 %d, delta=-1 %d (%.1f%% +)" % (t, len(rs), P, M, 100 * P / (P + M)))
json.dump(rows, open("isogeny_classes.json", "w"), indent=1)

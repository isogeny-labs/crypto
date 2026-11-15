#!/usr/bin/env python3
"""Target 2: compare the numbers printed in TOWER_LARGE.md (results table, three
ratio tables) and the bench-record metrics with the validator's recompute
(recompute_tower_large.py output) at the printed precision.
Usage: check_tower_md.py TOWER_LARGE.md recompute.json tower_large.json record.json"""
import json, sys, re
md = open(sys.argv[1]).read().splitlines()
rc = json.load(open(sys.argv[2])); tl = json.load(open(sys.argv[3])); rec = json.load(open(sys.argv[4]))
rows = {r["p"]: r for r in rc["rows"]}; trow = {r["p"]: r for r in tl["rows"]}
def tables():
    out = []; cur = None
    for l in md:
        if l.startswith("|"):
            cells = [c.strip() for c in l.strip().strip("|").split("|")]
            if cur is None: cur = [cells]
            else: cur.append(cells)
        else:
            if cur: out.append(cur)
            cur = None
    if cur: out.append(cur)
    return out
T = tables(); errs = []; n = 0
def cmp(a, s, what):
    global n
    n += 1
    s = s.replace(",", "")
    dec = len(s.split(".")[1]) if "." in s else 0
    if abs(round(a, dec) - float(s)) > 10 ** (-dec) * 0.51: errs.append((what, a, s))
# table 1: results
t1 = next(t for t in T if t[0][:3] == ["p", "inst", "r (mean)"])
for c in t1[2:]:
    p = int(c[0]); r = rows[p]; tr = trow[p]
    cmp(tr["r_mean"], c[2], f"p={p} r"); cmp(r["rho_ops"], c[3], f"p={p} rho"); cmp(r["rho_steps"], c[4], f"p={p} steps")
    cmp(r["ic2_ops"], c[5], f"p={p} IC2"); cmp(r["ic3_ops"], c[6], f"p={p} IC3"); cmp(r["ic3_over_ic2"], c[7], f"p={p} IC3/IC2")
    cmp(r["la2"], c[8], f"p={p} LA2"); cmp(r["la3"], c[9], f"p={p} LA3")
ratio_tables = [t for t in T if t[0][:2] == ["p", "g=2 @0"]]
assert len(ratio_tables) == 3, len(ratio_tables)
for t, key in zip(ratio_tables, ["ratio_la", "ratio_walk", "ratio_all"]):
    for c in t[2:]:
        p = int(c[0]); r = rows[p]
        for g, off in ((2, 1), (3, 4)):
            for j in range(3): cmp(r[f"{key}_g{g}"][j], c[off + j], f"{key} p={p} g={g} w{j}")
# record metrics
mm = []
for m in rec["metrics"]:
    mt = re.match(r"(ic2|ic3|rho_e)_(group_ops|linalg_mulmods)_p(\d+)$", m["name"])
    if not mt: mm.append((m["name"], "unparsed")); continue
    kind, what, p = mt.group(1), mt.group(2), int(mt.group(3)); r = trow[p]
    want = r["rho_ops"] if kind == "rho_e" else r["ic"][0 if kind == "ic2" else 1]["ops" if what == "group_ops" else "la_mulmods"]
    if abs(m["value"] - want) > 1e-9 * max(1, abs(want)): mm.append((m["name"], m["value"], want))
    if m.get("reference") is not None and abs(m["reference"] - r["rho_ops"]) > 1e-9 * r["rho_ops"]: mm.append((m["name"], "ref", m["reference"]))
print(f"TOWER_LARGE.md numbers compared {n}, mismatches {len(errs)}; record metrics {len(rec['metrics'])}, mismatches {len(mm)}")
for e in errs[:20]: print("  ", e)
for e in mm[:20]: print("  ", e)

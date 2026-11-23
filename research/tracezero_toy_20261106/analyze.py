#!/usr/bin/env python3
"""Summarise rows.jsonl per PROTOCOL.md (A1, A2). Stdlib only."""
import hashlib, json, random, sys
from collections import defaultdict

rows_path, out_path = sys.argv[1], sys.argv[2]
raw = open(rows_path, "rb").read()
rows = [json.loads(l) for l in raw.decode().splitlines()]
METRICS = {"sub_cov2": ("sub", 1), "sub_cov3": ("sub", 2), "sub_size": ("sub", 0),
           "cov2": ("full", 1), "cov3": ("full", 2), "size": ("full", 0)}


def get(r, arm, metric):
    kind, i = METRICS[metric]
    v = r.get(("sub_" if kind == "sub" else "") + arm)
    if v is None:
        return None
    return (v[0] if kind == "sub" else v)[i]


def boot(xs, rnd, B=2000):
    n = len(xs)
    m = sum(xs) / n
    ms = sorted(sum(xs[rnd.randrange(n)] for _ in range(n)) / n for _ in range(B))
    return m, ms[int(0.025 * B)], ms[int(0.975 * B) - 1]


COMPS = {"trace_vs_rand": ("trace", "rand"), "aa": ("trace", "aa"), "pos_vs_rand": ("pos", "rand")}
groups = defaultdict(list)
for r in rows:
    groups[(r["k"], r["d"], "ge17" if r["p"] >= 17 else "lt17")].append(r)
    groups[(r["k"], r["d"], "p%d" % r["p"])].append(r)
out = {"rows_sha256": hashlib.sha256(raw).hexdigest(), "n_rows": len(rows), "cells": {}}
rnd = random.Random(20261106)
for key, rs in sorted(groups.items()):
    cell = {"n_curves": len(rs)}
    for cname, (a, b) in COMPS.items():
        for m in METRICS:
            xs = [get(r, a, m) - get(r, b, m) for r in rs
                  if get(r, a, m) is not None and get(r, b, m) is not None]
            if len(xs) >= 5:
                mean, lo, hi = boot(xs, rnd)
                cell[f"{cname}.{m}"] = {"n": len(xs), "mean": mean, "lo": lo, "hi": hi}
    rc = [get(r, "rand", "cov2") for r in rs]
    cell["rand_full_cov2_mean"] = sum(rc) / len(rc)
    out["cells"]["k%d_d%d_%s" % key] = cell

dec = {}
for m in ("sub_cov2", "sub_cov3"):
    hits, cells = [], {}
    for k in (2, 3):
        for d in (1, 2):
            c = out["cells"].get(f"k{k}_d{d}_ge17", {})
            t, aa, pos = (c.get(f"{x}.{m}") for x in ("trace_vs_rand", "aa", "pos_vs_rand"))
            if not (t and aa and pos):
                continue
            sign = 1 if t["lo"] > 0 else -1 if t["hi"] < 0 else 0
            cells[f"k{k}_d{d}"] = {"trace_excludes0_sign": sign,
                                   "aa_halfwidth": (aa["hi"] - aa["lo"]) / 2,
                                   "pos_excludes0": pos["lo"] > 0 or pos["hi"] < 0}
            hits.append(sign)
    n_pos = sum(1 for s in hits if s == 1)
    n_neg = sum(1 for s in hits if s == -1)
    pos_ok = all(v["pos_excludes0"] for v in cells.values())
    aa_ok = all(v["aa_halfwidth"] <= 0.05 for v in cells.values())
    if not (pos_ok and aa_ok):
        verdict = "inconclusive (positive control or A/A band failed)"
    elif max(n_pos, n_neg) >= 3:
        verdict = "structure found"
    else:
        verdict = "no structure detected (H0 stands in this regime)"
    dec[m] = {"cells": cells, "n_pos": n_pos, "n_neg": n_neg, "pos_ok": pos_ok,
              "aa_ok": aa_ok, "verdict": verdict}
out["decision"] = dec
json.dump(out, open(out_path, "w"), indent=1, sort_keys=True)
for m, v in dec.items():
    print(m, v["verdict"], "pos_ok", v["pos_ok"], "aa_ok", v["aa_ok"], v["n_pos"], v["n_neg"])

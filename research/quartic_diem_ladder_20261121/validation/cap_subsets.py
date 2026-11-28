#!/usr/bin/env python3
"""Validator: the 131101 cells under the frozen 110-minute search cap (PROTOCOL.md section 3).

Usage: python3 -I cap_subsets.py REPO SHA OUT.json

The even 131101 process searched 144 seeds in search_wall_s (cell row); curve 288 is
the 144th even seed and 228 the 114th.  Under the cap the process would have been
stopped at 110 min and rerun with the curves admitted so far, so 288 is excluded and
228 is borderline.  This script reports, for both arms and w = 0, 0.1, 1:
  all five curves (as reported), {53,177,193,228} and {53,177,193};
with exact cluster-bootstrap intervals, the frozen verdict, the ln(IC/rho) slope over
all primes with the 131101 set replaced, and the paired FPM-mid pooled ratio.
"""
import itertools
import json
import math
import subprocess
import sys
from collections import Counter, defaultdict

import numpy as np

STUDY = "research/quartic_diem_ladder_20261121"
FILES = ["raw/run.jsonl", "raw/run131101_odd.jsonl", "raw/run131101_even.jsonl"]
NEW, SFB = "Q_diem_2lp_new", "Q_diem_2lp_sfb"
NATIVE = ["rel_mul", "rel_add", "la_mul", "la_add", "idx_lookups", "idx_inserts"]
WS = [0.0, 0.1, 1.0]
SUBSETS = {"reported_5": [53, 177, 193, 228, 288], "cap_with_228": [53, 177, 193, 228],
           "cap_without_228": [53, 177, 193]}


def git_rows(repo, sha, path):
    out = subprocess.run(["git", "-C", repo, "show", f"{sha}:{STUDY}/{path}"],
                         check=True, capture_output=True).stdout
    return [json.loads(l) for l in out.decode().splitlines() if l.strip()]


def gm(v):
    return math.exp(sum(math.log(x) for x in v) / len(v))


def exact_ci(vals):
    n = len(vals)
    lg = [math.log(v) for v in vals]
    dist = []
    for combo in itertools.combinations_with_replacement(range(n), n):
        w = math.factorial(n)
        for k in Counter(combo).values():
            w //= math.factorial(k)
        dist.append((sum(lg[i] for i in combo) / n, w / n ** n))
    dist.sort()
    out, acc, j = [], 0.0, 0
    for x, w in dist:
        acc += w
        while j < 2 and acc >= [0.025, 0.975][j] - 1e-15:
            out.append(math.exp(x))
            j += 1
    return out


def main():
    repo, sha, outp = sys.argv[1:4]
    rows = [r for fn in FILES for r in git_rows(repo, sha, fn)]
    cell = [r for r in rows if r["kind"] == "cell" and r["family"] == "quartic" and r["p"] == 131101]
    ic = [r for r in rows if r["kind"] == "run" and r["arm"] in (NEW, SFB)]
    by = defaultdict(dict)
    for r in ic:
        by[(r["arm"], r["p"], r["curve_seed"])][r["seed"]] = r
    curves = defaultdict(set)
    for r in ic:
        curves[r["p"]].add(r["curve_seed"])
    ideal = {(r["p"], r["curve_seed"]): math.sqrt(math.pi * r["r"] / 4) for r in ic}

    def cval(arm, p, c, w):
        return gm([x["group_ops"] + w * sum(x[k] for k in NATIVE) for x in by[(arm, p, c)].values()]) / ideal[(p, c)]

    out = {"sha": sha, "cells_131101": [{k: c[k] for k in ("tried", "admitted", "search_wall_s")} for c in cell]}
    even = [c for c in cell if c["tried"] == 144][0]
    per_seed = even["search_wall_s"] / even["tried"]
    out["even_process_seconds_per_seed"] = per_seed
    out["estimated_admission_minute"] = {"228": 114 * per_seed / 60, "288": 144 * per_seed / 60}
    res = {}
    for name, sub in SUBSETS.items():
        for arm in (NEW, SFB):
            cells = {}
            for w in WS:
                v = [cval(arm, 131101, c, w) for c in sub]
                cells[str(w)] = {"gm": gm(v), "exact_ci95": exact_ci(v)}
            ups = [cells[str(w)]["exact_ci95"][1] for w in WS]
            los = [cells[str(w)]["exact_ci95"][0] for w in WS]
            verdict = ("BELOW-RHO CANDIDATE" if all(u < 1 for u in ups) else
                       "ABOVE RHO" if all(l > 1 for l in los) else "UNRESOLVED")
            slopes = {}
            for w in WS:
                xs, ys = [], []
                for p in sorted(curves):
                    cs = sub if p == 131101 else sorted(curves[p])
                    for c in cs:
                        xs.append(math.log(p))
                        ys.append(math.log(cval(arm, p, c, w)))
                x, y = np.array(xs), np.array(ys)
                slopes[str(w)] = float(((x - x.mean()) * (y - y.mean())).sum() / ((x - x.mean()) ** 2).sum())
            res[f"{name}|{arm}"] = {"cells": cells, "verdict": verdict, "slope_ln_ratio_point": slopes}
        fpm = lambda x: x["cantor_mul"] + 10 * x["cantor_inv"] + x["rel_mul"] + 9 * x["la_mul"]
        allv = []
        for p in sorted(curves):
            cs = sub if p == 131101 else sorted(curves[p])
            for c in cs:
                allv.append(gm([fpm(by[(SFB, p, c)][s]) / fpm(by[(NEW, p, c)][s]) for s in (1, 2)]))
        res[f"{name}|paired_fpm_pooled_gm"] = gm(allv)
    out["subsets"] = res
    json.dump(out, open(outp, "w"), indent=1)
    for k, v in res.items():
        if isinstance(v, dict):
            print(k, v["verdict"], {w: (round(c["gm"], 4), [round(x, 4) for x in c["exact_ci95"]]) for w, c in v["cells"].items()},
                  {w: round(s, 4) for w, s in v["slope_ln_ratio_point"].items()})
        else:
            print(k, round(v, 4))
    print("est. admission minute", out["estimated_admission_minute"])


if __name__ == "__main__":
    main()

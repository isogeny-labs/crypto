#!/usr/bin/env python3
"""r7/m8 round 2 analysis: readings A and B under the frozen kill rule (PRESET.txt s5, PRESET_R2.txt s2).
Inputs: parent analysis.json (msolve exact rungs), r2/i4/results.jsonl (I4 truncated bases), r2/runs/*.I1.res.json.
Output: JSON on stdout."""
import glob
import json
import math
import os
import re
import sys

R = os.path.dirname(os.path.abspath(__file__)) + "/.."
PARENT = "/home/user/crypto/research/pkm_m8_runs_20261115/analysis.json"
ALPHA = {8: (0.1561, 0.0563), 9: (0.1740, 0.0615), 10: (0.1893, 0.0658)}
NAME = re.compile(r"L_p2013265921_m(\d+)_N(\d+)_s1k\d+_r(\d)")

cells = {}  # (m, N) -> {draw: {"L": .., "U": .., "src": [...]}}


def slot(m, N, r):
    return cells.setdefault((m, N), {}).setdefault(r, {"L": None, "U": None, "Lsrc": None, "Usrc": None, "A": None})


par = json.load(open(PARENT))
for m in (8, 9, 10):
    for rung in par["ladders"][str(m)]["rungs"]:
        for r, d in rung["draws"].items():
            if not d:
                continue
            s = slot(m, rung["N"], int(r))
            if d.get("stop") is None and d.get("D") is not None and d.get("unit"):
                s["U"], s["Usrc"], s["A"] = d["D"], "parent msolve exact", d["D"]

for ln in open(R + "/r2/i4/results.jsonl"):
    x = json.loads(ln)
    mm = NAME.match(x["cell"])
    m, N, r = int(mm[1]), int(mm[2]), int(mm[3])
    c = int(x["cell"].rsplit(".c", 1)[1])
    s = slot(m, N, r)
    if x.get("status") != "ok":
        continue
    if x["constant_in_basis"]:
        if s["U"] is None or c < s["U"]:
            s["U"], s["Usrc"] = c, f"I4 c={c} found 1"
        continue
    chk = x.get("singular_check", "")
    g = re.search(r"generators_nonzero_NF (\d+) of (\d+)", chk)
    p = re.search(r"pairs_checked (\d+) nonzero_NF (\d+) mode (\w+)", chk)
    if not (g and p) or g[1] != "0" or p[2] != "0" or x["delta_c"] <= 0:
        s.setdefault("I4_fail", []).append(x["cell"])
        continue
    level = "FULL" if p[3] == "all" else "SUPPORTED"
    if s["L"] is None or c + 1 > s["L"]:
        s["L"], s["Lsrc"], s["delta"] = c + 1, f"I4 c={c} {level} delta_{c}={x['delta_c']}", x["delta_c"]

for f in sorted(glob.glob(R + "/r2/runs/*.I1.res.json")) + sorted(glob.glob(R + "/r2/runs/*.I1m5000.res.json")):
    variant = "I1m5000" if "I1m5000" in f else "I1"
    mm = NAME.search(f)
    m, N, r = int(mm[1]), int(mm[2]), int(mm[3])
    res = json.load(open(f))
    log = f.replace(".res.json", ".log")
    degs = [int(z.split()[0]) for z in open(log, errors="replace") if re.match(r"^\s+\d+\s+\d+\s+\d+\s+\d+ x \d+", z)]
    out = f.replace(".res.json", ".out")
    unit = os.path.exists(out) and "[1]" in open(out).read()
    s = slot(m, N, r)
    s[variant] = dict(stop=res["stop"], wall=res["wall_s"], rss_kB=res["ru_maxrss_kB"], maxdeg=max(degs) if degs else None, unit=unit)
    # reading A: the low-memory variant counts only if no default run of the cell finished (PRESET.txt s3)
    if res["stop"] is None and unit and (variant == "I1" or s["A"] is None):
        s["A"] = max(degs)
        s["A_src"] = variant
        if s["U"] is None or max(degs) < s["U"]:
            s["U"], s["Usrc"] = max(degs), f"r2 {variant} msolve exact"


def ols(xs, ys):
    xb = sum(xs) / len(xs)
    yb = sum(ys) / len(ys)
    sxx = sum((x - xb) ** 2 for x in xs)
    return sum((x - xb) * (y - yb) for x, y in zip(xs, ys)) / sxx


def verdict_point(m, s):
    a1, a2 = ALPHA[m]
    return "KILL" if s >= a1 else ("WEAK-KILL" if s >= a2 else "OPEN-LOW")


out = {"cells": {}, "readingA": {}, "readingB": {}}
for (m, N), dr in sorted(cells.items()):
    if m not in ALPHA:
        continue
    Ls = [dr.get(r, {}).get("L") for r in (0, 1)]
    Us = [dr.get(r, {}).get("U") for r in (0, 1)]
    As = [dr.get(r, {}).get("A") for r in (0, 1)]
    Lc = min(Ls) if all(v is not None for v in Ls) else None
    Uc = min([u for u in Us if u is not None], default=None)
    Ac = min(As) if all(v is not None for v in As) else None
    out["cells"][f"m{m}_N{N}"] = dict(m=m, N=N, L=Lc, U=Uc, D_A=Ac, draws={str(r): dr[r] for r in dr})

for m in (8, 9, 10):
    need = math.ceil(m / 2)
    # reading A: exact rungs (both draws finished) in the contiguous ladder from N=m
    xs, ys = [], []
    N = m
    while f"m{m}_N{N}" in out["cells"] and out["cells"][f"m{m}_N{N}"]["D_A"] is not None:
        xs.append(N); ys.append(out["cells"][f"m{m}_N{N}"]["D_A"]); N += 1
    if len(xs) >= 3 and xs[-1] - xs[0] >= need:
        sA = ols(xs, ys); vA = verdict_point(m, sA)
    else:
        sA, vA = None, "INCONCLUSIVE"
    out["readingA"][m] = dict(N=xs, D=ys, sigma_hat=sA, verdict=vA)
    # reading B: largest contiguous prefix N=m..Nmax with finite needed bounds
    best = None
    cand = sorted(int(k.split("_N")[1]) for k in out["cells"] if k.startswith(f"m{m}_"))
    for Nmax in range(m, max(cand, default=m) + 1):
        xs = list(range(m, Nmax + 1))
        if any(f"m{m}_N{n}" not in out["cells"] for n in xs):
            break
        xb = sum(xs) / len(xs)
        C = [out["cells"][f"m{m}_N{n}"] for n in xs]
        lo_y = [c["L"] if n >= xb else c["U"] for n, c in zip(xs, C)]
        hi_y = [c["U"] if n >= xb else c["L"] for n, c in zip(xs, C)]
        if len(xs) >= 3 and xs[-1] - xs[0] >= need and all(v is not None for v in lo_y):
            slo = ols(xs, lo_y)
            shi = ols(xs, hi_y) if all(v is not None for v in hi_y) else None
            best = dict(N=xs, y_lo=lo_y, y_hi=hi_y, sigma_lo=slo, sigma_hi=shi)
    if best is None:
        vB = "INCONCLUSIVE"
    else:
        a1, a2 = ALPHA[m]
        slo, shi = best["sigma_lo"], best["sigma_hi"]
        if slo >= a1:
            vB = "KILL"
        elif shi is not None and shi < a2:
            vB = "OPEN-LOW"
        elif slo >= a2 and shi is not None and shi < a1:
            vB = "WEAK-KILL"
        else:
            vB = "INCONCLUSIVE"
    out["readingB"][m] = dict(best_prefix=best, verdict=vB, alpha1=ALPHA[m][0], alpha2=ALPHA[m][1])


def item(vs):
    if all(v in ("KILL", "WEAK-KILL") for v in vs) and "KILL" in vs:
        return "CLOSED-AT-TOY-SCALE"
    if "OPEN-LOW" in vs:
        return "OPEN"
    return "INCONCLUSIVE"


out["item_A"] = item([out["readingA"][m]["verdict"] for m in (8, 9, 10)])
out["item_B"] = item([out["readingB"][m]["verdict"] for m in (8, 9, 10)])
print(json.dumps(out, indent=1))

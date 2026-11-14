"""Merge raw rows into results.json and print tables + slopes."""
import json, glob, os, math, random
import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
rows = []
for f in sorted(glob.glob(os.path.join(HERE, "*.jsonl"))):
    if os.path.basename(f).startswith("T3"):
        continue
    for line in open(f):
        r = json.loads(line)
        r["source_file"] = os.path.basename(f)
        rows.append(r)
seen = set()
ded = []
for r in rows:
    k = (r["series"], r["n"], r["l"], r["solver"], r["target_index"])
    if k in seen:
        continue
    seen.add(k)
    if "core_lemmas_T2" in r and len(r.get("trim_rounds", [])) < 2:
        r["core_lemmas_T2"] = None  # T2 (iterated trim) not run for this row (ROUNDS=1)
        r["t2_not_run"] = True
    ded.append(r)
rows = ded
# series B reuses the series A (17,5) cell
rows += [dict(r, series="B", reused_from="A") for r in rows if r["series"] == "A" and r["n"] == 17 and r["l"] == 5]
t3 = [json.loads(x) for f in glob.glob(os.path.join(HERE, "T3*.jsonl")) for x in open(f)]


def gm(xs):
    xs = [max(x, 1) for x in xs]
    return 2 ** (sum(math.log2(x) for x in xs) / len(xs)) if xs else float("nan")


def fit(pts):
    ls = np.array([p[0] for p in pts], float)
    ys = np.log2(np.maximum(np.array([p[1] for p in pts], float), 1))
    return float(np.polyfit(ls, ys, 1)[0])


def slope(pts, B=2000):
    """pts = [(l, value)]; LS slope of log2 value on l, bootstrap within cells."""
    if len(set(p[0] for p in pts)) < 2:
        return None
    s = fit(pts)
    rng = random.Random(1)
    cells = {}
    for p in pts:
        cells.setdefault(p[0], []).append(p)
    bs = []
    for _ in range(B):
        samp = []
        for c in cells.values():
            samp += [rng.choice(c) for _ in c]
        bs.append(fit(samp))
    bs.sort()
    return dict(slope=round(s, 3), lo95=round(bs[int(0.025 * B)], 3), hi95=round(bs[int(0.975 * B)], 3),
                n_rows=len(pts), ls=sorted(cells))


summary = {"cells": [], "slopes": [], "t3": []}
print("| series | n | l | solver | truth | rows | gm conflicts | gm decisions | gm proof lemmas | gm core T1 | gm core T2 | core T1/conflicts | 2^(2l) | 2^(3l)/6 | 2^l |")
print("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|")
keys = sorted(set((r["series"], r["n"], r["l"], r["solver"], r["truth"]) for r in rows))
for k in keys:
    rs = [r for r in rows if (r["series"], r["n"], r["l"], r["solver"], r["truth"]) == k and not r.get("trim_failed")]
    l = k[2]
    c = dict(series=k[0], n=k[1], l=l, solver=k[3], truth=k[4], rows=len(rs),
             gm_conflicts=round(gm([r["conflicts"] for r in rs]), 1),
             gm_decisions=round(gm([r["decisions"] for r in rs]), 1))
    if k[4] == "UNSAT":
        c["gm_proof_adds"] = round(gm([r["proof_adds"] for r in rs]), 1)
        c["gm_core_T1"] = round(gm([r["core_lemmas_T1"] for r in rs]), 1)
        t2 = [r["core_lemmas_T2"] for r in rs if r["core_lemmas_T2"] is not None]
        c["gm_core_T2"] = round(gm(t2), 1) if t2 else "-"
        c["rows_T2"] = len(t2)
        c["gm_ratio_T1_over_conflicts"] = round(gm([r["core_lemmas_T1"] / max(r["conflicts"], 1) * 1e6 for r in rs]) / 1e6, 3)
        c["gm_core_lits_T1"] = round(gm([r["core_lits_T1"] for r in rs]), 1)
    summary["cells"].append(c)
    print("| %s | %d | %d | %s | %s | %d | %s | %s | %s | %s | %s | %s | %d | %.0f | %d |" % (
        k[0], k[1], l, k[3], k[4], len(rs), c["gm_conflicts"], c["gm_decisions"], c.get("gm_proof_adds", "-"),
        c.get("gm_core_T1", "-"), c.get("gm_core_T2", "-"), c.get("gm_ratio_T1_over_conflicts", "-"),
        4 ** l, 8 ** l / 6, 2 ** l))
print()
print("| series | solver | truth | metric | l range | rows | slope bits/l | 95% bootstrap |")
print("|---|---|---|---|---|---|---|---|")
for series in ("A", "B"):
    for sv in ("cd15", "g4"):
        for truth in ("UNSAT", "SAT"):
            for lmin in (2, 3):
                base = [r for r in rows if r["series"] == series and r["solver"] == sv and r["truth"] == truth
                        and r["l"] >= lmin and not r.get("trim_failed")]
                metrics = ["conflicts", "decisions"] + (["proof_adds", "core_lemmas_T1", "core_lemmas_T2", "core_lits_T1"] if truth == "UNSAT" else [])
                for m in metrics:
                    s = slope([(r["l"], r[m]) for r in base if r.get(m) is not None])
                    if not s or len(s["ls"]) < 2 or (lmin == 3 and min(s["ls"]) != 3) or (lmin == 2 and min(s["ls"]) != 2):
                        continue
                    s.update(series=series, solver=sv, truth=truth, metric=m)
                    summary["slopes"].append(s)
                    print("| %s | %s | %s | %s | %s | %d | %.2f | [%.2f, %.2f] |" % (
                        series, sv, truth, m, "%d..%d" % (s["ls"][0], s["ls"][-1]), s["n_rows"], s["slope"], s["lo95"], s["hi95"]))
print()
for r in t3:
    start = {k: v["core_T2"] for k, v in r["start"].items()}
    print("T3 n=%d l=%d target=%d start cores %s -> best %d (rounds %d)" % (r["n"], r["l"], r["target_index"], start, r["best_core_lemmas_T3"], len(r["rounds"])))
summary["t3"] = t3
json.dump(dict(note="pilot, unverified [pcc:unverified]", summary=summary, rows=rows), open(os.path.join(HERE, "results.json"), "w"), indent=1)

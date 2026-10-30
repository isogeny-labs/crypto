"""Fit the elimination exponent w of the dense MutantXL closure from recorded counted word ops.

log2 word_ops = a + w * log2 columns  (OLS with intercept), per (m, degree, kind),
bootstrap CI clustered by cell (m, ell, N).  Also a quadratic (curvature) fit and the local
slope at the largest recorded size.  Input: data/recorded_closure_rows.json (harvest of every
committed closure scan row).  Counted metrics only.
"""
import json, math, random, sys, collections
import numpy as np
random.seed(20261030); np.random.seed(20261030)
R = json.load(open("data/recorded_closure_rows.json"))
# drop specialised (--fix) rows; they are different systems
R = [r for r in R if not r["fixed"] and r["degree"] >= 3 and r["closure_word_ops"] > 0]

def cells(rows):
    g = collections.defaultdict(list)
    for r in rows: g[(r["m"], r["ell"], r["N"])].append(r)
    return g

def fit(xs, ys, deg=1):
    A = np.vander(xs, deg + 1, increasing=True)
    coef, *_ = np.linalg.lstsq(A, ys, rcond=None)
    return coef

def boot(rows, deg=1, B=4000, at=None):
    g = cells(rows); keys = list(g)
    out = []
    for _ in range(B):
        samp = [random.choice(keys) for _ in keys]
        pts = [r for k in samp for r in g[k]]
        xs = np.array([math.log2(r["columns"]) for r in pts]); ys = np.array([math.log2(r["closure_word_ops"]) for r in pts])
        if len(set(xs.round(6))) < deg + 1: continue
        c = fit(xs, ys, deg)
        if deg == 1: out.append((c[0], c[1]))
        else:
            x0 = at; out.append((c[0], c[1] + 2 * c[2] * x0, c[2]))
    return np.array(out)

def ci(a): return np.percentile(a, [2.5, 97.5])

res = []
def block(name, rows, quad=False):
    if len(rows) < 4: return
    xs = np.array([math.log2(r["columns"]) for r in rows]); ys = np.array([math.log2(r["closure_word_ops"]) for r in rows])
    nc = len(cells(rows))
    c = fit(xs, ys); b = boot(rows)
    ratio = float(np.mean(ys / xs))
    d = dict(name=name, rows=len(rows), cells=nc, log2cols=[float(xs.min()), float(xs.max())], a=float(c[0]), w=float(c[1]),
             a_ci=ci(b[:, 0]).tolist(), w_ci=ci(b[:, 1]).tolist(), mean_ratio=ratio)
    if quad and nc >= 6:
        x0 = float(xs.max()); cq = fit(xs, ys, 2); bq = boot(rows, 2, at=x0)
        d.update(local_w_top=float(cq[1] + 2 * cq[2] * x0), local_w_top_ci=ci(bq[:, 1]).tolist(), curvature=float(cq[2]), curvature_ci=ci(bq[:, 2]).tolist())
    res.append(d)

for m in (3, 4):
    for deg in (3, 4, 5):
        clean = [r for r in R if r["m"] == m and r["degree"] == deg and not r["closure_refuted"] and r["saturated"]]
        block(f"m{m} deg{deg} saturated-not-refuted (sat+unsat draws)", clean, quad=True)
        ref = [r for r in R if r["m"] == m and r["degree"] == deg and r["closure_refuted"]]
        block(f"m{m} deg{deg} refuted (ops truncated at refutation)", ref, quad=True)
for deg in (3, 4, 5):
    clean = [r for r in R if r["degree"] == deg and not r["closure_refuted"] and r["saturated"]]
    block(f"m3+m4 deg{deg} saturated-not-refuted", clean, quad=True)
# pooled across degrees WITH a degree indicator (confound control)
clean = [r for r in R if not r["closure_refuted"] and r["saturated"]]
g = cells(clean); keys = list(g)
def pooled(pts):
    xs = np.array([math.log2(r["columns"]) for r in pts]); ys = np.array([math.log2(r["closure_word_ops"]) for r in pts])
    A = np.column_stack([np.ones(len(pts)), xs, [r["degree"] == 4 for r in pts], [r["degree"] == 5 for r in pts]])
    return np.linalg.lstsq(A, ys, rcond=None)[0]
c = pooled(clean); bs = []
for _ in range(2000):
    samp = [random.choice(keys) for _ in keys]; pts = [r for k in samp for r in g[(k)]]
    try: bs.append(pooled(pts))
    except Exception: pass
bs = np.array(bs)
res.append(dict(name="pooled deg3/4/5 with degree intercepts (m3+m4)", rows=len(clean), cells=len(keys), w=float(c[1]), w_ci=ci(bs[:, 1]).tolist(), deg4_shift=float(c[2]), deg5_shift=float(c[3])))
json.dump(res, open("data/fit_w.json", "w"), indent=1)
for d in res:
    print(d["name"], "| rows", d["rows"], "cells", d["cells"], "| w=%.3f [%.2f, %.2f]" % (d["w"], *d["w_ci"]),
          ("| a=%.2f [%.1f,%.1f] ratio=%.3f cols 2^%.1f..2^%.1f" % (d["a"], *d["a_ci"], d["mean_ratio"], *d["log2cols"])) if "a" in d else "",
          ("| local w@top=%.2f [%.2f,%.2f] curv=%.3f [%.3f,%.3f]" % (d["local_w_top"], *d["local_w_top_ci"], d["curvature"], *d["curvature_ci"])) if "local_w_top" in d else "")

"""Reprice the ECC2K-130 m=4 D=5 row (descent-degree note, section 7) at the fitted oracle cost.

Same model as examples/m4_reprice.rs (ported, with its positive controls asserted):
  targets = l + max(log2(m!) + n - m*l, 0), n = 131;  N = (m-2)n + m l
  oracle  = a + w * log2 C(N, <=D) + conv        (a = 0 reproduces the existing table)
  LA      = log2(m) + 2 l  (never converted)
  total   = log2(2^(targets+oracle) + 2^LA), minimised over integer l in 4..69; rho = 2^60.81.
conv (word-ops -> rho steps) = log2(Ir per word-op / Ir per n=83 rho step), research/k_conversion_20261018/raw.jsonl.
CIs: joint bootstrap of (a, w), clustered by cell, as in fit_w.py.
"""
import json, math, random, collections
import numpy as np
random.seed(7); np.random.seed(7)
N_FIELD, RHO = 131, 60.81
def l2add(a, b): return max(a, b) + math.log2(1 + 2 ** (-abs(a - b)))
def log2C(n, d):
    tot, b = 0.0, 1.0
    for i in range(d + 1):
        if i > n: break
        tot += b; b = b * (n - i) / (i + 1)
    return math.log2(tot)
def lf(m): return sum(math.log2(k) for k in range(1, m + 1))
def cost(m, l, D, a, w, conv):
    N = (m - 2) * N_FIELD + m * l
    t = l + max(lf(m) + N_FIELD - m * l, 0)
    o = a + w * log2C(N, D) + conv
    return l2add(t + o, math.log2(m) + 2 * l), o
def best(m, D, a, w, conv):
    return min(((cost(m, l, D, a, w, conv)[0], l) for l in range(4, 70)))
# positive controls: existing table rows (m=4 D=5: w=1 +10.32 A, w=2 +45.71 A)
assert abs(best(4, 5, 0, 1, 0)[0] - 71.13) < .005 and abs(best(4, 5, 0, 2, 0)[0] - 106.52) < .005
# conversion
raw = [json.loads(l) for l in open("../k_conversion_20261018/raw.jsonl") if l.strip().startswith("{")]
cl = [r for r in raw if r.get("kind") == "closure" and r.get("word_ops")]
irw = [r["ir"] / r["word_ops"] for r in cl]
step = 6664.8
conv = {k: math.log2(v / step) for k, v in dict(med=float(np.median(irw)), lo=min(irw), hi=max(irw)).items()}
print("conv bits (median, low, high):", {k: round(v, 2) for k, v in conv.items()})
CM = conv["med"]
out = []
def row(label, a, w, ci=None, m=4, D=5):
    tA, _ = best(m, D, a, w, 0.0); tB, lB = best(m, D, a, w, CM)
    d = dict(label=label, a=a, w=w, A_bits=tA - RHO, B_bits=tB - RHO, l=lB)
    if ci: d.update(ci)
    out.append(d)
    print(f"{label:58s} a={a:6.2f} w={w:5.2f}  A {tA-RHO:+7.2f}  B {tB-RHO:+7.2f} (l={lB})" + (f"  B 95% [{ci['B_lo']:+.1f}, {ci['B_hi']:+.1f}]" if ci else ""))
for w in (1.0, 1.6, 1.7, 1.76, 1.8, 2.0, 2.25, 2.33): row("origin (a=0), grid", 0.0, w)
fits = json.load(open("data/fit_w.json"))
R = [r for r in json.load(open("data/recorded_closure_rows.json")) if not r["fixed"] and r["degree"] >= 3 and r["closure_word_ops"] > 0]
def boot_rows(rows, B=2000):
    g = collections.defaultdict(list)
    for r in rows: g[(r["m"], r["ell"], r["N"])].append(r)
    keys = list(g); res = []
    for _ in range(B):
        pts = [r for k in [random.choice(keys) for _ in keys] for r in g[k]]
        xs = np.array([math.log2(r["columns"]) for r in pts]); ys = np.array([math.log2(r["closure_word_ops"]) for r in pts])
        if len(set(xs.round(6))) < 2: continue
        A = np.vander(xs, 2, increasing=True); c = np.linalg.lstsq(A, ys, rcond=None)[0]; res.append(c)
    return np.array(res)
for name, deg, mm, lab in [("m3+m4 deg5", 5, (3, 4), "deg-5 block fit (m3+m4), intercept"), ("m4 deg4", 4, (4,), "deg-4 block fit (m4), intercept; D=4 proxy at D=5"), ("m3+m4 deg4", 4, (3, 4), "deg-4 block fit (m3+m4), intercept; proxy at D=5")]:
    rows = [r for r in R if r["degree"] == deg and r["m"] in mm and not r["closure_refuted"] and r["saturated"]]
    f = [x for x in fits if x["name"].startswith(name + " saturated")][0]
    bs = boot_rows(rows)
    vals = [best(4, 5, c[0], c[1], CM)[0] - RHO for c in bs]
    row(lab, f["a"], f["w"], dict(B_lo=float(np.percentile(vals, 2.5)), B_hi=float(np.percentile(vals, 97.5))))
# local slope at the top of the measured range as an optimistic-for-dense-cubic stress: w = 3 with the deg-5 intercept at 2^16
row("dense-cubic stress: w=3, a so that ops match deg-5 fit at 2^16 cols", (-5.87 + (2.829 - 3) * 16.0), 3.0)
json.dump(dict(conv=conv, rows=out), open("data/reprice.json", "w"), indent=1)

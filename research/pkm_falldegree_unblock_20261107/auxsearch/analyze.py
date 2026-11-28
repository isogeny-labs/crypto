#!/usr/bin/env python3
"""Analysis of the auxiliary-curve search sweep (PROTOCOL.md). Pure Python.
Reads sweep.out.* and replicate.jsonl; writes summary.json and fig1.svg."""
import glob, json, math, random, statistics as st
rows = [json.loads(l) for f in sorted(glob.glob("sweep.out.*")) for l in open(f) if l.strip()]
rep = [json.loads(l) for l in open("replicate.jsonl")]
def ops(d): return d["adds"] + d["doubles"]
def q(xs, a):
    xs = sorted(xs); k = (len(xs) - 1) * a; i = int(k)
    return xs[i] if i + 1 >= len(xs) else xs[i] + (xs[i + 1] - xs[i]) * (k - i)
def interval(t, P, n=200, reps=20000, seed=7):
    """99% interval of mean(tries)/2^t for n Geometric(P) draws (simulation)."""
    rng = random.Random(seed ^ t); lp = math.log1p(-P); out = []
    for _ in range(reps):
        s = sum(1 + int(math.log(1.0 - rng.random()) / lp) for _ in range(n))
        out.append(s / n / 2 ** t)
    return q(out, 0.005), q(out, 0.995)
def ks_exp(xs):
    xs = sorted(xs); n = len(xs)
    D = max(max((i + 1) / n - (1 - math.exp(-x)), (1 - math.exp(-x)) - i / n) for i, x in enumerate(xs))
    lam = (math.sqrt(n) + 0.12 + 0.11 / math.sqrt(n)) * D
    pval = max(0.0, min(1.0, 2 * sum((-1) ** (k - 1) * math.exp(-2 * k * k * lam * lam) for k in range(1, 101))))
    return D, pval
import predict
summary = {"schema": "auxsearch.summary/v1", "p": 2013265921, "rows": []}
for t in range(8, 15):
    R = [r for r in rows if r["t"] == t]
    tries = [r["info"]["aux_curves_tried"] for r in R]
    PH = predict.count(t) / predict.order_sl2(t)
    M = 1 << t
    PD = sum(predict.N(-((a - 1) ** 2), t) for a in range(M)) / predict.order_sl2(t)
    lo, hi = interval(t, PH); dlo, dhi = interval(t, PD)
    r_t = st.mean(tries) / 2 ** t
    D, pv = ks_exp([x / 2 ** t for x in tries])
    s_ops = [ops(r["info"]["aux_ops_search"]) for r in R]
    c_ops = [ops(r["info"]["aux_ops_chain"]) for r in R]
    per_curve = sum(s_ops) / sum(tries)
    repl = next(r for r in rep if r["t"] == t)["info"]["aux_curves_tried"]
    rank = sum(x < repl for x in tries) / len(tries)
    summary["rows"].append({
        "t": t, "n": len(R), "mean_tries": st.mean(tries), "median_tries": st.median(tries),
        "sd_tries": st.stdev(tries), "r_t": r_t, "H_interval99": [lo, hi], "D_mean_over_2t": 1 / PD / 2 ** t,
        "D_interval99": [dlo, dhi], "H_holds": lo <= r_t <= hi, "D_rejected": r_t > dhi,
        "ks_D": D, "ks_p": pv, "search_ops_mean": st.mean(s_ops), "search_ops_median": st.median(s_ops),
        "search_ops_per_curve": per_curve, "chain_ops_mean": st.mean(c_ops),
        "ms_mean": st.mean(r["ms"] for r in R), "replicated_tries": repl, "replicated_quantile": rank})
json.dump(summary, open("summary.json", "w"), indent=1)
# Figure: r_t with H and D intervals; right panel search ops vs 2^t.
W, H, pad = 860, 360, 60
def sx(t, x0, w): return x0 + (t - 7.5) / 7 * w
svg = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" font-family="sans-serif" font-size="12">',
       '<rect width="100%" height="100%" fill="white"/>']
x0, w, y0, h = pad, 340, 30, 270
def sy(v): return y0 + h - (v / 1.6) * h
svg.append(f'<text x="{x0}" y="18" font-weight="bold">mean curves tried / 2^t (200 searches each)</text>')
for v in (0, 0.5, 1.0, 1.5):
    svg.append(f'<line x1="{x0}" x2="{x0+w}" y1="{sy(v)}" y2="{sy(v)}" stroke="#ddd"/><text x="{x0-8}" y="{sy(v)+4}" text-anchor="end">{v}</text>')
for r in summary["rows"]:
    X = sx(r["t"], x0, w)
    for (a, b), col in ((r["H_interval99"], "#4c78a8"), (r["D_interval99"], "#e45756")):
        svg.append(f'<rect x="{X-9}" y="{sy(b)}" width="18" height="{sy(a)-sy(b)}" fill="{col}" opacity="0.25"/>')
    svg.append(f'<circle cx="{X}" cy="{sy(r["r_t"])}" r="4.5" fill="black"/>')
    svg.append(f'<text x="{X}" y="{y0+h+16}" text-anchor="middle">{r["t"]}</text>')
svg.append(f'<text x="{x0+w/2}" y="{y0+h+34}" text-anchor="middle">t</text>')
svg.append(f'<rect x="{x0+150}" y="{y0+8}" width="12" height="10" fill="#4c78a8" opacity="0.4"/><text x="{x0+166}" y="{y0+17}">model H (cyclic 2-Sylow): 99%</text>')
svg.append(f'<rect x="{x0+150}" y="{y0+24}" width="12" height="10" fill="#e45756" opacity="0.4"/><text x="{x0+166}" y="{y0+33}">model D (2^t | #E only): 99%</text>')
x1 = x0 + w + 110
lo_l, hi_l = 4, 9
def sy2(v): return y0 + h - (math.log10(v) - lo_l) / (hi_l - lo_l) * h
svg.append(f'<text x="{x1}" y="18" font-weight="bold">group operations per search (log)</text>')
for e in range(lo_l, hi_l + 1):
    svg.append(f'<line x1="{x1}" x2="{x1+w}" y1="{sy2(10**e)}" y2="{sy2(10**e)}" stroke="#ddd"/><text x="{x1-8}" y="{sy2(10**e)+4}" text-anchor="end">1e{e}</text>')
for r in summary["rows"]:
    X = sx(r["t"], x1, w)
    svg.append(f'<circle cx="{X}" cy="{sy2(r["search_ops_mean"])}" r="4.5" fill="#4c78a8"/>')
    svg.append(f'<circle cx="{X}" cy="{sy2(max(r["chain_ops_mean"],1e4))}" r="4.5" fill="#f58518"/>')
    svg.append(f'<text x="{X}" y="{y0+h+16}" text-anchor="middle">{r["t"]}</text>')
svg.append(f'<text x="{x1+w/2}" y="{y0+h+34}" text-anchor="middle">t</text>')
svg.append(f'<circle cx="{x1+16}" cy="{y0+12}" r="4.5" fill="#4c78a8"/><text x="{x1+26}" y="{y0+16}">search (mean)</text>')
svg.append(f'<circle cx="{x1+16}" cy="{y0+28}" r="4.5" fill="#f58518"/><text x="{x1+26}" y="{y0+32}">Vélu chain (mean, floored at 1e4)</text>')
svg.append('</svg>')
open("fig1.svg", "w").write("\n".join(svg))
for r in summary["rows"]:
    print(r["t"], round(r["r_t"], 3), [round(x, 3) for x in r["H_interval99"]], r["H_holds"], r["D_rejected"],
          round(r["ks_p"], 3), round(r["search_ops_per_curve"]), round(r["search_ops_mean"]), round(r["chain_ops_mean"]), round(r["replicated_quantile"], 2))

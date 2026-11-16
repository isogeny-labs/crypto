import csv, json, math, sys, statistics as st
ROOT = sys.argv[1] if len(sys.argv) > 1 else "results"; BASEMODE = sys.argv[2] if len(sys.argv) > 2 else "smallx"
from collections import defaultdict
from scipy import stats
rows = list(csv.DictReader(open(f"{ROOT}/per_scalar.csv")))
meta = json.load(open(f"{ROOT}/meta.json"))
N = meta["N"]
g = defaultdict(list)
for r in rows: g[(int(r["m"]), r["curve"], r["fbmode"])].append(r)
def ci(xs):
    m = st.mean(xs); s = st.stdev(xs); n = len(xs); t = stats.t.ppf(0.975, n-1)
    return m, m - t*s/math.sqrt(n), m + t*s/math.sqrt(n)
def wilson(k, n, z=1.96):
    p = k/n; d = 1 + z*z/n; c = (p + z*z/(2*n))/d; w = z*math.sqrt(p*(1-p)/n + z*z/(4*n*n))/d
    return c - w, c + w
base = {m: {int(r["scalar_idx"]): float(r["yield_"]) for r in g[(m, "E0", BASEMODE)]} for m in (2, 3, 4)}
out = ["| m | curve | deg | FB mode | B | |FB| | |T_m|/#E (pred) | 1/m! | yield avg | 95% CI | min | max | pooled Wilson CI | yield / pred | trials avg (min-max) | paired Δ vs E0 (95% CI) | rec |",
       "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|"]
summ = []
for (m, c, mode), rs in sorted(g.items(), key=lambda kv: (kv[0][0], kv[0][2] != BASEMODE, int(kv[1][0]["deg"]))):
    y = [float(r["yield_"]) for r in rs]; tr = [int(r["trials"]) for r in rs]
    rel = sum(int(r["relations"]) for r in rs); ttot = sum(tr)
    mu, lo, hi = ci(y); wl, wh = wilson(rel, ttot)
    pred = int(rs[0]["table"]) / N
    d = [float(r["yield_"]) - base[m][int(r["scalar_idx"])] for r in rs]
    dm, dl, dh = ci(d) if any(d) else (0, 0, 0)
    rec = sum(int(r["recovered"]) for r in rs)
    out.append(f"| {m} | {c} | {rs[0]['deg']} | {mode} | {rs[0]['B']} | {rs[0]['fb']} | {pred:.4f} | {1/math.factorial(m):.4f} | {mu:.4f} | [{lo:.4f}, {hi:.4f}] | {min(y):.4f} | {max(y):.4f} | [{wl:.4f}, {wh:.4f}] | {mu/pred:.3f} | {st.mean(tr):.0f} ({min(tr)}-{max(tr)}) | {dm:+.4f} [{dl:+.4f}, {dh:+.4f}] | {rec}/{len(rs)} |")
    summ.append(dict(m=m, curve=c, deg=int(rs[0]["deg"]), mode=mode, fb=int(rs[0]["fb"]), pred=pred, mean=mu, lo=lo, hi=hi, ratio=mu/pred))
# does yield track |FB| (luck) rather than isogeny degree?  regress log yield on log|FB| within m, smallx only
out.append("")
for m in (2, 3, 4):
    S = [s for s in summ if s["m"] == m and s["mode"] == BASEMODE]
    if len({s["fb"] for s in S}) == 1:
        Y = [[float(r["yield_"]) for r in g[(m, s["curve"], BASEMODE)]] for s in S]
        an = stats.f_oneway(*Y); kw = stats.kruskal(*Y)
        means = [s["mean"] for s in S]
        out.append(f"- m={m}: all |FB|={S[0]['fb']}; curve means {min(means):.4f}-{max(means):.4f} "
                   f"(spread {100*(max(means)-min(means))/st.mean(means):.1f}%); ANOVA F={an.statistic:.2f} p={an.pvalue:.3f}; "
                   f"Kruskal-Wallis p={kw.pvalue:.3f}; yield/(|T_m|/#E) {min(s['ratio'] for s in S):.3f}-{max(s['ratio'] for s in S):.3f}")
        continue
    lr = stats.linregress([math.log(s["fb"]) for s in S], [math.log(s["mean"]) for s in S])
    rd = stats.spearmanr([math.log(s["deg"]) for s in S], [s["ratio"] for s in S])
    out.append(f"- m={m}: log(yield) vs log|FB| slope = {lr.slope:.2f} (theory m={m}), r^2={lr.rvalue**2:.3f}; "
               f"Spearman(log deg, yield/pred) = {rd.statistic:+.2f} (p={rd.pvalue:.2f}); yield/pred range "
               f"{min(s['ratio'] for s in S):.3f}-{max(s['ratio'] for s in S):.3f}")
out.append("\n### Paired per-trial decomposability vs E0 (same (a,b) stream, R' = phi(R))\n")
out.append("| m | curve | FB mode | trials | P(E0) | P(curve) | P(both) | P(E0)P(curve) | phi coef |")
out.append("|---|---|---|---|---|---|---|---|---|")
for r in csv.DictReader(open(f"{ROOT}/paired.csv")):
    if r["curve"] == "E0" and r["fbmode"] == BASEMODE: continue
    pe, pc = float(r["p_E0"]), float(r["p_curve"])
    out.append(f"| {r['m']} | {r['curve']} | {r['fbmode']} | {r['n_trials']} | {pe:.4f} | {pc:.4f} | {float(r['both']):.4f} | {pe*pc:.4f} | {float(r['phi']):+.3f} |")
for x in meta["meta"]:
    if "semaev" in x: out.append(f"\nSemaev cross-check (E0): {x['semaev']}")
open(f"{ROOT}/summary.md", "w").write("\n".join(out) + "\n"); print("\n".join(out))

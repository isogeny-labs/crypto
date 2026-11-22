#!/usr/bin/env python3
"""Fit and plot for SCALING_20261018.md. Reads results/run-20261018-scaling-*.jsonl
(only recorded values; nothing recomputed except the rho pricing formula stated below).
usage: analyze.py <results_dir> <out_dir>"""
import json, glob, math, sys, collections
res, out = sys.argv[1], sys.argv[2]
RHO_STEP_NEG = 108 + 4 + 4   # K affine add (2 kmul + kinv = 18+90) + lookup 4 + canonicalization 4
RHO_STEP_PLAIN = 108 + 4     # no negation
inst, cnt, trials, rho = {}, collections.defaultdict(list), collections.defaultdict(list), collections.defaultdict(list)
for f in sorted(glob.glob(res + "/run-20261018-scaling-*.jsonl")):
    for ln in open(f):
        try: r = json.loads(ln)
        except Exception: continue
        q = r.get("q")
        if r.get("rec") == "instance": inst[q] = r
        elif r.get("rec") == "counters": cnt[q].append(r)
        elif r.get("rec") == "trial" and r.get("variant") == "full": trials[q].append(r)
        elif r.get("rec") == "trial" and r.get("variant") == "rho": rho[q].append(r)
def ols(x, y):
    n = len(x); mx = sum(x)/n; my = sum(y)/n
    sxx = sum((a-mx)**2 for a in x); b = sum((a-mx)*(c-my) for a, c in zip(x, y))/sxx
    a0 = my - b*mx; rss = sum((c-(a0+b*a))**2 for a, c in zip(x, y))
    se = math.sqrt(rss/(n-2)/sxx) if n > 2 else float('nan')
    return b, se, a0
rows = []
for q in sorted(cnt):
    c = cnt[q][0]; t = trials[q]
    ell = c["ell"]
    n_ok = sum(1 for r in t if r.get("correct"))
    steps = [r["rho_steps"] for r in rho.get(q, [])]
    rows.append(dict(q=q, ell=ell, ord=c["ord"], nC=c["n_Cfq"], pairs=c["pairs"], rel=c["relations"], cols=c["la_cols"],
        rank=c["la_rank_mod_ell"], laops=c["la_row_ops"], ic=c["ops_total"], enum=c["ops_enum_CK_run"]+c["ops_enum_CK_in_ic"],
        relops=c["ops_relations"], tab=c["ops_E0_phi_table"], tgt=c["ops_target"], ntr=c["n_valid_transfer"], ncls=c["n_deg6_classes"],
        coll=c["n_deg6_key_collisions"], trials=len(t), ok=n_ok, ops_distinct=len({r["ops"] for r in t}),
        rho_an=math.sqrt(math.pi*ell/4), rho_an_cost=math.sqrt(math.pi*ell/4)*RHO_STEP_NEG,
        rho_ms=(sum(steps)/len(steps) if steps else float('nan')), rho_n=len(steps)))
    rows[-1]["rho_meas_cost"] = rows[-1]["rho_ms"]*RHO_STEP_PLAIN
    rows[-1]["ratio"] = rows[-1]["ic"]/rows[-1]["rho_an_cost"]
fits = {}
x = [math.log2(r["ell"]) for r in rows]
for name, key in [("IC total (ops_total)", "ic"), ("C(K) enumeration (2x)", "enum"), ("phi(nG) table phase", "tab"),
                  ("line relations phase", "relops"), ("matched rho analytic sqrt(pi l/4)*116", "rho_an_cost"),
                  ("rho measured steps*112", "rho_meas_cost")]:
    y = [math.log2(r[key]) for r in rows]
    b, se, a0 = ols(x, y); fits[name] = (b, se, a0, len(x))
json.dump(dict(rows=rows, fits={k: dict(slope=v[0], se=v[1], intercept=v[2], n=v[3]) for k, v in fits.items()}),
          open(out + "/scaling.json", "w"), indent=1)
print("q ell ic rho_an ratio")
for r in rows: print(r["q"], r["ell"], r["ic"], round(r["rho_an_cost"]), round(r["ratio"], 1), r["trials"], r["ok"])
for k, v in fits.items(): print("%-45s slope %.3f +- %.3f (n=%d)" % (k, v[0], v[1], v[3]))
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
fig, ax = plt.subplots(figsize=(7, 4.6))
ax.plot(x, [math.log2(r["ic"]) for r in rows], "o-", color="#1f5fbf", label="quartic_ic total counted ops (slope %.2f)" % fits["IC total (ops_total)"][0])
ax.plot(x, [math.log2(r["tab"]) for r in rows], "s--", color="#7a7a7a", label="  phi(nG) table phase (slope %.2f)" % fits["phi(nG) table phase"][0])
ax.plot(x, [math.log2(r["enum"]) for r in rows], "^--", color="#a05a00", label="  C(K) enumeration (slope %.2f)" % fits["C(K) enumeration (2x)"][0])
ax.plot(x, [math.log2(r["relops"]) for r in rows], "v--", color="#2a8a4a", label="  line relations (slope %.2f)" % fits["line relations phase"][0])
ax.plot(x, [math.log2(r["rho_an_cost"]) for r in rows], "D-", color="#c0392b", label="matched rho sqrt(pi l/4)*116 (slope %.2f)" % fits["matched rho analytic sqrt(pi l/4)*116"][0])
ax.set_xlabel("log2 l  (l = prime order of E0(F_{q^3}) subgroup)"); ax.set_ylabel("log2 counted cost (PROTOCOL pinned units)")
ax.set_title("quartic_ic vs matched rho, q = " + ",".join(str(r["q"]) for r in rows), fontsize=10)
ax.grid(alpha=.3); ax.legend(fontsize=7.5, loc="upper left")
fig.tight_layout(); fig.savefig(out + "/cost_vs_log2ell.svg")

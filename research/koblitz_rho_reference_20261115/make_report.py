#!/usr/bin/env python3
"""Renders figures/, README.md and REPORT.pdf from results.json (recorded values and `ecbench compare/band` output only)."""
import json, os, math, textwrap
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages
HERE = os.path.dirname(os.path.abspath(__file__)); R = json.load(open(os.path.join(HERE, "results.json")))
os.makedirs(os.path.join(HERE, "figures"), exist_ok=True)
W = ["0", "0.1", "1"]; C = R["curves"]; ns = sorted(C, key=lambda n: C[n]["log2_r"])
f = lambda p: f"{p['ratio']:.3f} [{p['lo']:.3f}, {p['hi']:.3f}]"
def tri(v): return " / ".join(f"{v[w]['ratio']:.3f}" for w in W)
def verdict(p): return "below" if p["hi"] < 1 else ("above" if p["lo"] > 1 else "unresolved")
def across(v):
    s = {verdict(v[w]) for w in W}; return s.pop() + " at all three" if len(s) == 1 else "order flips: " + "/".join(verdict(v[w]) for w in W)
best = {"17": "gen-c1", "23": "gen-c4", "29": "gen-c1", "31": "gen-c2", "37": "gen-c8"}
# ---- figure
fig, ax = plt.subplots(1, 2, figsize=(11, 4.2))
col = {"0": "#1b6ca8", "0.1": "#d98324", "1": "#3a8a3a"}
for w in W:
    x = [C[n]["log2_r"] for n in ns]; y = [C[n]["ratios"]["rho-v5/rho-strong"][w]["ratio"] for n in ns]
    lo = [C[n]["ratios"]["rho-v5/rho-strong"][w]["lo"] for n in ns]; hi = [C[n]["ratios"]["rho-v5/rho-strong"][w]["hi"] for n in ns]
    ax[0].errorbar(x, y, yerr=[[a - b for a, b in zip(y, lo)], [b - a for a, b in zip(y, hi)]], marker="o", ms=4, capsize=2, color=col[w], label=f"v5 / strong, w = {w}")
x = [C[n]["log2_r"] for n in ns]
ax[0].plot(x, [C[n]["ratios"]["rho-strong-v2/rho-strong"]["0.1"]["ratio"] for n in ns], "--", color="grey", label="strong_v2 / strong, w = 0.1")
ax[0].plot(x, [C[n]["v5_hash_counted_over_strong_derived"]["1"] for n in ns], ":", color=col["1"], label="v5 with jump index counted / strong, w = 1 (derived)")
ax[0].axhline(1, color="k", lw=0.6); ax[0].set_yscale("log"); ax[0].set_xlabel("log2 r"); ax[0].set_ylabel("paired cost ratio (S)"); ax[0].set_title("rho.signed_frobenius_v5 / strong"); ax[0].legend(fontsize=6.5)
for ref, c, m in (("rho-strong", "#1b6ca8", "o"), ("rho-v5", "#c0392b", "s")):
    xs, ys, el, eh = [], [], [], []
    for n in ns:
        if n in best:
            p = C[n]["ratios"][f"{best[n]}/{ref}"]["0.1"]; xs.append(C[n]["log2_r"]); ys.append(p["ratio"]); el.append(p["ratio"] - p["lo"]); eh.append(p["hi"] - p["ratio"])
    ax[1].errorbar(xs, ys, yerr=[el, eh], marker=m, capsize=2, color=c, label=f"best IC arm / {ref}")
ax[1].axhline(1, color="k", lw=0.6); ax[1].set_yscale("log"); ax[1].set_xlabel("log2 r"); ax[1].set_title("Distance to rho, strongest validated IC arm, w = 0.1"); ax[1].legend(fontsize=7)
plt.tight_layout(); fig.savefig(os.path.join(HERE, "figures/fig1-reference-ratio.png"), dpi=150); fig.savefig(os.path.join(HERE, "figures/fig1-reference-ratio.svg"))
# ---- tables
T = []
T.append("| curve | log2 r | strong S (w=0/0.1/1) | v5 S | strong_v2 S | rho-neg S | strong set-up share of charged | v5 set-up share | median steps/ideal strong, v5 |\n|:--|--:|:--|:--|:--|:--|--:|--:|:--|")
for n in ns:
    c = C[n]; a = c["arms"]; S = lambda k: " / ".join(f"{a[k]['mean_S'][w]:.3f}" for w in W)
    T.append(f"| n{n} `{c['slug']}` | {c['log2_r']:.2f} | {S('rho-strong')} | {S('rho-v5')} | {S('rho-strong-v2')} | {S('rho-neg')} | {a['rho-strong']['setup_share_of_charged']:.2f} | {a['rho-v5']['setup_share_of_charged']:.2f} | {a['rho-strong']['steps_over_ideal_median']:.2f}, {a['rho-v5']['steps_over_ideal_median']:.2f} |")
T1 = "\n".join(T)
T = ["| curve | log2 r | v5 / strong at w = 0 | w = 0.1 | w = 1 | v5 with jump index counted / strong (derived, point) | strong_v2 / strong (w = 0.1) | A/A strong, A/A v5 |\n|:--|--:|:--|:--|:--|:--|:--|:--|"]
for n in ns:
    c = C[n]; q = c["ratios"]["rho-v5/rho-strong"]; d = c["v5_hash_counted_over_strong_derived"]
    T.append(f"| n{n} | {c['log2_r']:.2f} | {f(q['0'])} | {f(q['0.1'])} | {f(q['1'])} | {d['0']:.3f} / {d['0.1']:.3f} / {d['1']:.3f} | {f(c['ratios']['rho-strong-v2/rho-strong']['0.1'])} | {c['ratios']['rho-strong-aa/rho-strong']['0.1']['ratio']:.3f}, {c['ratios']['rho-v5-aa/rho-v5']['0.1']['ratio']:.3f} |")
T2 = "\n".join(T)
fit = R["fit_ln_ratio_v5_over_strong_vs_log2r"]
T = ["| curve | log2 r | IC arm | S(w=0/0.1/1) | IC / strong | IC / v5 | order vs strong | order vs v5 |\n|:--|--:|:--|:--|:--|:--|:--|:--|"]
for n in ns:
    for a in C[n]["arms"]:
        if a.startswith(("gen-", "k1-")):
            c = C[n]; ss = ' / '.join('%.2f' % c['arms'][a]['mean_S'][w] for w in W); qs = c["ratios"][f"{a}/rho-strong"]; qv = c["ratios"][f"{a}/rho-v5"]
            T.append(f"| n{n} | {c['log2_r']:.2f} | {a} | {ss} | {f(qs['0'])}; {f(qs['0.1'])}; {f(qs['1'])} | {f(qv['0'])}; {f(qv['0.1'])}; {f(qv['1'])} | {across(qs)} | {across(qv)} |")
T3 = "\n".join(T)
SUP = R["post_hoc_supplement"]
T = ["| curve | rho-frob S (host-priced, flat in w) | rho-frob / strong (w=0, 0.1, 1) | rho-frob / v5 (w=0, 0.1, 1) | rho.signed_frobenius (tuned, plain) / strong (w=0) |\n|:--|--:|:--|:--|:--|"]
for n in sorted(SUP, key=int):
    s = SUP[n]; rq = s["ratios"]
    if "rho-frob/rho-strong" in rq:
        T.append(f"| n{n} | {s['arms']['rho-frob']['mean_S']['0']:.3f} | {tri(rq['rho-frob/rho-strong'])} | {tri(rq['rho-frob/rho-v5'])} | {rq['rho-frob-plain/rho-strong']['0']['ratio']:.3f} |")
    else:
        T.append(f"| n{n} | no price row for this curve in full_prices.json (arm errors, 192/192) | n/a | n/a | {rq['rho-frob-plain/rho-strong']['0']['ratio']:.3f} |")
T4 = "\n".join(T)
def interp(x, w):
    pts = sorted((C[n]["log2_r"], math.log(C[n]["ratios"]["rho-v5/rho-strong"][w]["ratio"])) for n in C)
    if x < pts[0][0] - 1e-9 or x > pts[-1][0] + 1e-9: return None
    for (x0, y0), (x1, y1) in zip(pts, pts[1:]):
        if x0 - 1e-9 <= x <= x1 + 1e-9: return math.exp(y0 + (y1 - y0) * (x - x0) / (x1 - x0))
ROWS = [("K1 pair-table sized for n (validated)", "icv1-f2m23-tm5197-1f85e9e1 (a=1 n23)", 22.00, "strong"), ("K1 pair-table sized for n (validated)", "icv1-f2m29-tm40309-30c52b96", 15.37, "strong"),
        ("gen-orbit (validated), lazy, two-columns, Frobenius-orbit rows", "icv1-f2m13-t181-515ee569", 11.0, "rho-frob / strong_full"), ("same", "icv1-f2m17-tm101-00378d4e", 16.0, "same"),
        ("same", "icv1-f2m19-t797-b6cf2467", 17.0, "same"), ("same", "icv1-f2m31-tm90707-c95f16f5", 20.5, "same"), ("same", "icv1-f2m23-t5197-69e76b73 (a=0 n23)", 21.0, "same"),
        ("same", "icv1-f2m37-tm534059-32aad96b", 27.8, "same"), ("lazy orbit; n41/n53 panel; F4/F5/F6 ladder", "icv1-f2m41-tm2308219-7f48b14a", 39.0, "strong"), ("n41/n53 panel", "icv1-f2m53-tm56619371-dac20a85", 44.26, "strong")]
T = ["| leaderboard section | curve | log2 r | v5 / strong at this r (w=0; w=1), interpolated in ln ratio | distance under v5 = distance under strong x |\n|:--|:--|--:|:--|:--|"]
for sec, cu, x, ref in ROWS:
    a0, a1 = interp(x, "0"), interp(x, "1")
    T.append(f"| {sec} | {cu} | {x:.2f} | " + (f"{a0:.3f}; {a1:.3f} | {1/a0:.2f}; {1/a1:.2f}" if a0 else "outside the measured range (2^15.4 to 2^44.3); not extrapolated | -") + " |")
T5 = "\n".join(T)
fits = "; ".join(f"w = {w}: {fit[w]['alpha']:.2f} + {fit[w]['beta']:.3f} log2 r (residual sd {fit[w]['resid_sd']:.2f})" for w in W)
open(os.path.join(HERE, "tables/tables.md"), "w").write("\n\n".join([T1, T2, T3, T4, T5]) + "\n")
json.dump({"T1": T1, "T2": T2, "T3": T3, "T4": T4, "T5": T5, "fits": fits}, open(os.path.join(HERE, "tables/tables.json"), "w"))
# ---- README
body = open(os.path.join(HERE, "README.body.md")).read()
for k, v in (("@T1@", T1), ("@T2@", T2), ("@T3@", T3), ("@T4@", T4), ("@T5@", T5), ("@FITS@", fits), ("@SHA@", R["binary_sha256"])): body = body.replace(k, v)
open(os.path.join(HERE, "README.md"), "w").write(body)
# ---- PDF
with PdfPages(os.path.join(HERE, "REPORT.pdf")) as pdf:
    txt = body.replace("`", "").replace("**", "")
    lines = []
    for p in txt.split("\n"):
        if p.startswith("|"): continue
        lines += textwrap.wrap(p, 118) or [""]
    pages = [lines[i:i + 62] for i in range(0, len(lines), 62)]
    for pg in pages:
        fg = plt.figure(figsize=(8.27, 11.69)); fg.text(0.05, 0.97, "\n".join(pg), va="top", family="monospace", fontsize=6.4); pdf.savefig(fg); plt.close(fg)
    pdf.savefig(fig)
    for title, t in (("Scaling of the single-target references", T2), ("Arms, S and set-up share", T1), ("Distance to rho under each reference", T3), ("Post hoc supplement: the tuned rho-frob", T4), ("Leaderboard rows: interpolated multiplier", T5)):
        fg = plt.figure(figsize=(11.69, 8.27)); fg.text(0.03, 0.97, title, va="top", fontsize=10, weight="bold")
        rows = [r.strip("|").split("|") for r in t.split("\n") if r.startswith("|") and not r.startswith("|:")]
        rows = [[c.strip() for c in r] for r in rows]
        ax_ = fg.add_axes([0.02, 0.05, 0.96, 0.88]); ax_.axis("off")
        tb = ax_.table(cellText=rows[1:], colLabels=rows[0], loc="upper center", cellLoc="left"); tb.auto_set_font_size(False); tb.set_fontsize(5.2); tb.auto_set_column_width(list(range(len(rows[0]))))
        pdf.savefig(fg); plt.close(fg)
print("ok")

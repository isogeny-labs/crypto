"""Lay out the recorded campaign rows (results/campaign.jsonl) as tables and
figures. Paired ratios use the same (q, seed) instance; no new measurement."""
import json, os
from collections import defaultdict
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = os.path.dirname(os.path.abspath(__file__))
rows = [json.loads(l) for l in open(os.path.join(HERE, "results", "campaign.jsonl"))]
F = os.path.join(HERE, "figures"); os.makedirs(F, exist_ok=True)
plt.rcParams.update({"font.size": 9, "axes.grid": True, "grid.alpha": 0.3})

def med(v):
    v = sorted(v); n = len(v)
    return v[n // 2] if n % 2 else 0.5 * (v[n // 2 - 1] + v[n // 2])

inst = defaultdict(dict)
for d in rows:
    inst[(d["p"], d["seed"])][d["tag"]] = d
ps = sorted({p for p, _ in inst})
out = []
P = lambda *a: out.append(" ".join(str(x) for x in a))
gates = all(d["solved"] and d["pushforward_8P"] and d["T_P_nonzero"] and d["iota_pairs_constant"] and d["x_inf_eq_4c"] for d in rows)
P("runs: %d; all correctness gates pass: %s" % (len(rows), gates))
P("")
P("| q | seeds | t_ic baseline s | A/A ratio median [min, max] | lp:0.5 ratio median [min, max] | lp:0.5 faster | lp:0.35 ratio median [min, max] | lp:0.35 faster | unknowns base / lp:0.5 | IC/rho-exp base | IC/rho-exp lp:0.5 |")
P("|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|")
summ = {}
for p in ps:
    ks = [k for k in inst if k[0] == p and all(t in inst[k] for t in ("baseline", "aa", "lp50", "lp35"))]
    if not ks:
        continue
    def ratios(tag):
        return [inst[k]["baseline"]["t_ic_total"] / inst[k][tag]["t_ic_total"] for k in ks]
    def rexp(k, tag):
        b = inst[k]["baseline"]
        return inst[k][tag]["t_ic_total"] / (b["rho_expected_steps"] * b["rho_seconds"] / b["rho_steps"])
    aa, l5, l3 = ratios("aa"), ratios("lp50"), ratios("lp35")
    summ[p] = (aa, l5, l3, [rexp(k, "baseline") for k in ks], [rexp(k, "lp50") for k in ks])
    P("| %d | %d | %.2f | %.3f [%.3f, %.3f] | %.2f [%.2f, %.2f] | %d/%d | %.2f [%.2f, %.2f] | %d/%d | %d / %d | %.2f | %.2f |" % (
        p, len(ks), med([inst[k]["baseline"]["t_ic_total"] for k in ks]),
        med(aa), min(aa), max(aa), med(l5), min(l5), max(l5), sum(x > 1 for x in l5), len(ks),
        med(l3), min(l3), max(l3), sum(x > 1 for x in l3), len(ks),
        med([inst[k]["baseline"]["unknowns"] for k in ks]), med([inst[k]["lp50"]["unknowns"] for k in ks]),
        med(summ[p][3]), med(summ[p][4])))
P("")
P("| q | arm | t_rel s | t_LA s | t_descent s | candidates | partials | combined | dropped (>=2 LP) |")
P("|--:|:--|--:|--:|--:|--:|--:|--:|--:|")
for p in ps:
    for tag in ("baseline", "lp50", "lp35"):
        v = [inst[k][tag] for k in inst if k[0] == p and tag in inst[k]]
        if v:
            P("| %d | %s | %.2f | %.2f | %.2f | %d | %d | %d | %d |" % (p, tag, med([d["t_relations"] for d in v]), med([d["t_linear_algebra"] for d in v]),
              med([d["t_descent"] for d in v]), med([d["rel_candidates"] + d["quad_candidates"] for d in v]),
              med([d["lp_partials"] for d in v]), med([d["lp_combined"] for d in v]), med([d["lp_dropped"] for d in v])))
open(os.path.join(HERE, "results", "summary.md"), "w").write("\n".join(out) + "\n")
print("\n".join(out))

def save(fig, name):
    fig.tight_layout(); fig.savefig(os.path.join(F, name + ".svg")); fig.savefig(os.path.join(F, name + ".png"), dpi=150); plt.close(fig)

qs = sorted(summ)
fig, ax = plt.subplots(figsize=(6.4, 3.8))
for i, (lab, col, idx) in enumerate([("A/A (iota rerun)", "#7a7a7a", 0), ("lp:0.5", "#2a6fb0", 1), ("lp:0.35", "#d9822b", 2)]):
    for q in qs:
        ax.scatter([q * (1 + 0.03 * (i - 1))] * len(summ[q][idx]), summ[q][idx], s=10, color=col, alpha=0.5)
    ax.plot(qs, [med(summ[q][idx]) for q in qs], color=col, marker="o", label=lab + " (median)")
ax.axhline(1, color="black", lw=0.8, ls="--")
ax.set_xscale("log"); ax.set_xlabel("q"); ax.set_ylabel("t_ic(iota) / t_ic(arm), same instance")
ax.set_title("Single large prime vs iota baseline (>1: arm faster), 5 seeds per q")
ax.legend(fontsize=7)
save(fig, "lp_speedup")

fig, ax = plt.subplots(figsize=(6.4, 3.6))
ax.plot(qs, [med(summ[q][3]) for q in qs], color="#7a7a7a", marker="o", label="iota baseline")
ax.plot(qs, [med(summ[q][4]) for q in qs], color="#2a6fb0", marker="o", label="lp:0.5")
ax.axhline(1, color="black", lw=0.8, ls="--")
ax.set_xscale("log"); ax.set_yscale("log"); ax.set_xlabel("q"); ax.set_ylabel("IC time / expected-steps rho")
ax.set_title("Distance remaining to matched rho (medians)")
ax.legend(fontsize=7)
save(fig, "lp_gap_to_rho")

with open(os.path.join(HERE, "results", "summary_v1.csv"), "w") as f:
    f.write("q,aa_med,aa_lo,aa_hi,lp50_med,lp50_lo,lp50_hi,lp50_faster,lp35_med,lp35_lo,lp35_hi,lp35_faster,n\n")
    for q in qs:
        aa, l5, l3 = summ[q][0], summ[q][1], summ[q][2]
        f.write("%d,%.3f,%.3f,%.3f,%.2f,%.2f,%.2f,%d,%.2f,%.2f,%.2f,%d,%d\n" % (q, med(aa), min(aa), max(aa), med(l5), min(l5), max(l5), sum(x > 1 for x in l5),
                med(l3), min(l3), max(l3), sum(x > 1 for x in l3), len(l5)))

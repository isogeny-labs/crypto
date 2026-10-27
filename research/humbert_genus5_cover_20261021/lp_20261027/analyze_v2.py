"""Lay out the recorded PROTOCOL_v2 rows (results/v2.jsonl): paired per-instance
ratios on the same (q, seed); no new measurement."""
import json, os
from collections import defaultdict
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = os.path.dirname(os.path.abspath(__file__))
rows = [json.loads(l) for l in open(os.path.join(HERE, "results", "v2.jsonl"))]
F = os.path.join(HERE, "figures"); os.makedirs(F, exist_ok=True)
plt.rcParams.update({"font.size": 9, "axes.grid": True, "grid.alpha": 0.3})

def med(v):
    v = sorted(v); n = len(v)
    return v[n // 2] if n % 2 else 0.5 * (v[n // 2 - 1] + v[n // 2])

inst = defaultdict(dict)
for d in rows:
    inst[(d["p"], d["seed"])][d["tag"]] = d
TAGS = ("v1", "pre", "pre_aa", "lp50pre")
full = {k: v for k, v in inst.items() if all(t in v for t in TAGS)}
ps = sorted({p for p, _ in full})
gates = all(d["solved"] and d["pushforward_8P"] and d["T_P_nonzero"] and d["iota_pairs_constant"] and d["x_inf_eq_4c"] for d in rows)
out = ["runs: %d; complete instances: %d; all correctness gates pass: %s" % (len(rows), len(full), gates), ""]

def rexp(k, tag):
    b = full[k]["v1"]
    return full[k][tag]["t_ic_total"] / (b["rho_expected_steps"] * b["rho_seconds"] / b["rho_steps"])

out.append("Per instance (t_ic in s; rounds = LA rounds):")
out.append("")
out.append("| q | seed | v1 t_ic (rounds) | pre t_ic (rounds, ext.) | M1 v1/pre | A/A pre/pre_aa | lp50pre t_ic (rounds) | M2 pre/lp50pre |")
out.append("|--:|--:|--:|--:|--:|--:|--:|--:|")
summ = {}
for (p, s) in sorted(full):
    a = full[(p, s)]
    m1 = a["v1"]["t_ic_total"] / a["pre"]["t_ic_total"]
    aa = a["pre"]["t_ic_total"] / a["pre_aa"]["t_ic_total"]
    m2 = a["pre"]["t_ic_total"] / a["lp50pre"]["t_ic_total"]
    summ.setdefault(p, []).append((m1, aa, m2, a["v1"]["la_rounds"], rexp((p, s), "v1"), rexp((p, s), "pre"), rexp((p, s), "lp50pre")))
    out.append("| %d | %d | %.2f (%d) | %.2f (%d, %d) | %.2f | %.3f | %.2f (%d) | %.2f |" % (
        p, s, a["v1"]["t_ic_total"], a["v1"]["la_rounds"], a["pre"]["t_ic_total"], a["pre"]["la_rounds"], a["pre"]["precheck_extensions"],
        m1, aa, a["lp50pre"]["t_ic_total"], a["lp50pre"]["la_rounds"], m2))
out.append("")
out.append("| q | M1 median [min, max] | multi-round v1 instances: pre faster | A/A median [min, max] | M2 median [min, max] | lp50pre faster | IC/rho-exp v1 | pre | lp50pre |")
out.append("|--:|--:|--:|--:|--:|--:|--:|--:|--:|")
for p in ps:
    v = summ[p]
    m1 = [x[0] for x in v]; aa = [x[1] for x in v]; m2 = [x[2] for x in v]
    multi = [x for x in v if x[3] > 1]
    out.append("| %d | %.2f [%.2f, %.2f] | %d/%d | %.3f [%.3f, %.3f] | %.2f [%.2f, %.2f] | %d/%d | %.2f | %.2f | %.2f |" % (
        p, med(m1), min(m1), max(m1), sum(x[0] > 1 for x in multi), len(multi), med(aa), min(aa), max(aa),
        med(m2), min(m2), max(m2), sum(x > 1 for x in m2), len(m2), med([x[4] for x in v]), med([x[5] for x in v]), med([x[6] for x in v])))
open(os.path.join(HERE, "results", "summary_v2.md"), "w").write("\n".join(out) + "\n")
print("\n".join(out))

def save(fig, name):
    fig.tight_layout(); fig.savefig(os.path.join(F, name + ".svg")); fig.savefig(os.path.join(F, name + ".png"), dpi=150); plt.close(fig)

fig, ax = plt.subplots(figsize=(6.4, 3.8))
for i, (lab, col, idx) in enumerate([("A/A (pre rerun)", "#7a7a7a", 1), ("M1: precheck vs v1 retry", "#3a9a5b", 0), ("M2: lp:0.5 vs fixed baseline", "#2a6fb0", 2)]):
    for q in ps:
        ax.scatter([q * (1 + 0.03 * (i - 1))] * len(summ[q]), [x[idx] for x in summ[q]], s=10, color=col, alpha=0.5)
    ax.plot(ps, [med([x[idx] for x in summ[q]]) for q in ps], color=col, marker="o", label=lab + " (median)")
ax.axhline(1, color="black", lw=0.8, ls="--")
ax.set_xscale("log"); ax.set_yscale("log"); ax.set_xlabel("q"); ax.set_ylabel("time ratio, same instance (>1: second arm faster)")
ax.set_title("Protocol v2: LA precheck and large primes, 5 seeds per q")
ax.legend(fontsize=7)
save(fig, "v2_ratios")

fig, ax = plt.subplots(figsize=(6.4, 3.6))
for lab, col, idx in [("iota, v1 retry", "#7a7a7a", 4), ("iota + precheck", "#3a9a5b", 5), ("lp:0.5 + precheck", "#2a6fb0", 6)]:
    ax.plot(ps, [med([x[idx] for x in summ[q]]) for q in ps], color=col, marker="o", label=lab)
ax.axhline(1, color="black", lw=0.8, ls="--")
ax.set_xscale("log"); ax.set_yscale("log"); ax.set_xlabel("q"); ax.set_ylabel("IC time / expected-steps rho")
ax.set_title("Distance remaining to matched rho (medians)")
ax.legend(fontsize=7)
save(fig, "v2_gap_to_rho")

with open(os.path.join(HERE, "results", "summary_v2.csv"), "w") as f:
    f.write("q,m1_med,m1_lo,m1_hi,multi_faster,multi_n,aa_med,aa_lo,aa_hi,m2_med,m2_lo,m2_hi,lp_faster,n,gap_v1,gap_pre,gap_lp\n")
    for p in ps:
        v = summ[p]
        m1 = [x[0] for x in v]; aa = [x[1] for x in v]; m2 = [x[2] for x in v]; multi = [x for x in v if x[3] > 1]
        f.write("%d,%.2f,%.2f,%.2f,%d,%d,%.3f,%.3f,%.3f,%.2f,%.2f,%.2f,%d,%d,%.2f,%.2f,%.2f\n" % (
            p, med(m1), min(m1), max(m1), sum(x[0] > 1 for x in multi), len(multi), med(aa), min(aa), max(aa),
            med(m2), min(m2), max(m2), sum(x > 1 for x in m2), len(m2), med([x[4] for x in v]), med([x[5] for x in v]), med([x[6] for x in v])))

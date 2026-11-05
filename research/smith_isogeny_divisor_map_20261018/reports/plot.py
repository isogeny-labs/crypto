"""Lay out recorded values of the W3b study as SVG (+150 dpi PNG).

Reads data/summary.json and data/cost_fit.json (both written by the Rust
example `smith_isogeny summarize` / `cost-fit`); computes nothing new.
"""
import json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = os.path.dirname(os.path.abspath(__file__))
D = os.path.join(HERE, "..", "data")
S = json.load(open(os.path.join(D, "summary.json")))
CF = json.load(open(os.path.join(D, "cost_fit.json")))
plt.rcParams.update({"font.size": 8, "svg.fonttype": "path"})
C_PASS, C_SKIP, C_FAIL = "#2a6fbb", "#b9cde6", "#d1495b"


def save(fig, name, rect=None):
    fig.tight_layout(rect=rect)
    fig.savefig(os.path.join(HERE, name + ".svg"))
    fig.savefig(os.path.join(HERE, name + ".png"), dpi=150)
    plt.close(fig)


# Figure 1: T2-T6 tallies summed over q, per engine
checks = ["t2_well_defined", "t3_homomorphism", "t4_kernel_s", "t4_outside_nonzero",
          "t5_order_nonzero", "t5_order_annihilated", "t6_group_order"]
labels = ["T2 well-defined", "T3 homomorphism", "T4 kernel ⊇ S(F_q)", "T4 nonzero off S",
          "T5 Ψ(G) ≠ 0", "T5 ℓ·Ψ(G) ~ 0", "T6 #J·Ψ(x) ~ 0"]
fig, axes = plt.subplots(1, 2, figsize=(7.4, 2.9), sharey=True)
for ax, eng, title in zip(axes, ["w1", "deg6"], ["W1 canonical keys (reduce_deg6)", "degree-6 adjoint engine (branch Pic)"]):
    ps = [sum(v["engines"][eng][c]["pass"] for v in S["verify"]) for c in checks]
    sk = [sum(v["engines"][eng][c]["skip"] for v in S["verify"]) for c in checks]
    fl = [sum(v["engines"][eng][c]["fail"] for v in S["verify"]) for c in checks]
    y = range(len(checks))
    ax.barh(y, ps, color=C_PASS, label="pass")
    ax.barh(y, sk, left=ps, color=C_SKIP, label="skip (engine undecided)")
    ax.barh(y, fl, left=[a + b for a, b in zip(ps, sk)], color=C_FAIL, label="fail")
    for i in y:
        ax.text(ps[i] + sk[i] + fl[i] + 8, i, f"{ps[i]}/{fl[i]}/{sk[i]}", va="center", fontsize=6.5)
    ax.set_title(title, fontsize=8)
    ax.set_xlabel("check instances (labels: pass/fail/skip)")
    ax.invert_yaxis()
axes[0].set_yticks(range(len(checks)), labels)
h, l = axes[0].get_legend_handles_labels()
fig.legend(h, l, loc="lower center", ncol=3, fontsize=6.5, frameon=False)
fig.subplots_adjust(bottom=0.2)
fig.suptitle("T2–T6 summed over q ∈ {7, 11, 13, 17, 19, 23}, 6 curves per q (32 with a plane quartic)", fontsize=8)
save(fig, "verify_tallies", rect=(0, 0.07, 1, 1))

# Figure 2: Route S funnel per q
R = S["route_s"]
fig, ax = plt.subplots(figsize=(7.4, 2.8))
stages = [("filter_instances", "instances (prime ℓ ≥ 5 | #E0)"), ("prop4_pass", "Props. 2 and 4 pass"),
          ("quartic_t1_pass", "X a smooth plane quartic (T1)"), ("verified_on_e", "k recovered, kG = Q on E")]
w = 0.2
for j, (k, lab) in enumerate(stages):
    xs = [i + (j - 1.5) * w for i in range(len(R))]
    vals = [r[k] for r in R]
    ax.bar(xs, vals, w, label=lab, color=["#9aa5b1", "#5b8cc4", "#2a6fbb", "#1b4f8a"][j])
    for x, v in zip(xs, vals):
        ax.text(x, v, str(v), ha="center", va="bottom", fontsize=5.5, rotation=90)
ax.set_xticks(range(len(R)), [f"q = {r['q']}" for r in R])
ax.set_yscale("log")
ax.set_ylabel("instances (log)")
ax.legend(fontsize=6.5, ncol=2, loc="upper left")
ax.set_title("T8 Route S: E0/F_{q^3} → H/F_q (JV m=3, r=5, split arm) → Ψ → J_X(F_q) dlog", fontsize=8)
save(fig, "route_s_funnel")

# Figure 3: T7 cost per psi(D) against q
pts = CF["points"]
qs = [p["q"] for p in pts]
fig, ax = plt.subplots(1, 2, figsize=(7.4, 2.7))
for a, key, lab in [(ax[0], "mul", "median F_q multiplications per ψ(D)"), (ax[1], "wall_ns", "median wall time per ψ(D) [ns]")]:
    a.loglog(qs, [p[key] for p in pts], "o", color=C_PASS, ms=4, base=2)
    f = CF["mul" if key == "mul" else "wall"]
    xx = [min(qs), max(qs)]
    import math
    a.loglog(xx, [2 ** (f["intercept"] + f["slope"] * math.log2(x)) for x in xx], "-", color="#555", lw=0.8, base=2,
             label=f"OLS slope {f['slope']:.3f} [{f['slope_lo']:.3f}, {f['slope_hi']:.3f}] (95%)")
    a.set_xlabel("q")
    a.set_ylabel(lab)
    a.legend(fontsize=6.5)
fig.suptitle("T7: cost of ψ on one generic degree-3 divisor (200 per q, median); log–log", fontsize=8)
save(fig, "cost_vs_q")
print("ok")

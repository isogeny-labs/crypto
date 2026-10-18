"""Lays out recorded values only: summary.json (means over the sessions' verified
records), the saved ecbench comparisons (ratios and 95 % intervals) and the
ecbench tables (mean S and its interval). Computes no result."""
import json, re, pathlib
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parent
S = json.loads((HERE / "summary.json").read_text())

def cmp(sess, a, b):
    d = json.loads((ROOT / "sessions" / sess / "comparisons" / f"{a}__{b}.json").read_text())["ops"]
    return d["ratio_b_over_a"], d["ci95"][0], d["ci95"][1], d["pairs"]

def table(sess):
    out = {}
    for line in (ROOT / "tables" / f"table-{sess}.md").read_text().splitlines():
        c = [x.strip() for x in line.split("|")]
        if len(c) > 8 and c[1].startswith("ECBS"):
            lo, hi = re.findall(r"[\d.]+", c[8])
            out[c[2]] = (float(c[7]), float(lo), float(hi))
    return out

rows = []
for r in S:
    sess = r["session"]
    base = next(k for k in r["arms"] if re.fullmatch(r"ic-orbit-c\d+-renew", k))
    lazy = base + "-lazy"
    rows.append(dict(sess=sess, n=r["n"], a=sess[1], log2r=r["log2r"], base=base, lazy=lazy,
                     imp=cmp(sess, lazy, base), vs_strong=cmp(sess, "rho-strong", lazy),
                     tab=table(sess), phases={k: r["arms"][k]["phases"] for k in (base, lazy)}))
rows.sort(key=lambda x: x["log2r"])
lab = [f"n={x['n']}, a={x['a']}\nlog2 r={x['log2r']:.1f}" for x in rows]
C = dict(base="#6b7280", lazy="#0f766e", strong="#b45309")

# Figure 1: improvement over the frozen baseline, per curve
fig, ax = plt.subplots(figsize=(9, 4.4))
xs = range(len(rows))
ax.errorbar(xs, [x["imp"][0] for x in rows],
            yerr=[[x["imp"][0] - x["imp"][1] for x in rows], [x["imp"][2] - x["imp"][0] for x in rows]],
            fmt="o", color=C["lazy"], capsize=4, ms=7)
ax.axhline(1, color="k", lw=0.8, ls="--")
ax.set_xticks(list(xs)); ax.set_xticklabels(lab, fontsize=8)
ax.set_ylabel("S(eager) / S(lazy)  (>1: lazy is cheaper)")
ax.set_title("Lazy orbit-member index vs frozen baseline: paired cost ratio per curve\n"
             "ecbench.gae, paired by target, cluster-bootstrap 95 % CI; 96 pairs (n41: 16, n37: 48)", fontsize=10)
ax.set_ylim(0.95, 1.55); ax.grid(alpha=0.3)
fig.tight_layout(); fig.savefig(HERE / "fig1-improvement.svg"); fig.savefig(HERE / "fig1-improvement.png", dpi=150); plt.close(fig)

# Figure 2: mean S per arm, against log2 r (the distance to strong rho)
fig, ax = plt.subplots(figsize=(9, 4.6))
for key, name, col, mk in (("base", "IC eager (frozen baseline)", C["base"], "s"),
                           ("lazy", "IC lazy (this change)", C["lazy"], "o"),
                           ("strong", "rho-strong (matched reference)", C["strong"], "^")):
    ys, lo, hi = [], [], []
    for x in rows:
        arm = {"base": x["base"], "lazy": x["lazy"], "strong": "rho-strong"}[key]
        m, l, h = x["tab"][arm]; ys.append(m); lo.append(m - l); hi.append(h - m)
    ax.errorbar([x["log2r"] for x in rows], ys, yerr=[lo, hi], fmt=mk + "-", color=col, capsize=3, label=name, ms=6)
ax.set_yscale("log"); ax.set_xlabel("log2 r"); ax.set_ylabel("mean S (GAE per sqrt r)")
ax.set_title("Mean S per arm, fully priced (*_full methods), 95 % interval from ecbench table", fontsize=10)
ax.grid(alpha=0.3, which="both"); ax.legend(fontsize=8)
fig.tight_layout(); fig.savefig(HERE / "fig2-S-by-curve.svg"); fig.savefig(HERE / "fig2-S-by-curve.png", dpi=150); plt.close(fig)

# Figure 3: where the saving is (mean GAE per phase, eager vs lazy)
fig, axes = plt.subplots(1, len(rows), figsize=(14, 4), sharey=False)
pcol = {"factor_base": "#94a3b8", "oracle_setup": "#dc2626", "relations": "#2563eb", "linear_algebra": "#a855f7", "verify": "#16a34a"}
for ax, x in zip(axes, rows):
    for i, arm in enumerate((x["base"], x["lazy"])):
        bottom = 0
        for ph in ("factor_base", "oracle_setup", "relations", "linear_algebra", "verify"):
            v = x["phases"][arm].get(ph, 0)
            ax.bar(i, v, bottom=bottom, color=pcol[ph], label=ph if ax is axes[0] and i == 0 else None)
            bottom += v
    ax.set_xticks([0, 1]); ax.set_xticklabels(["eager", "lazy"], fontsize=8)
    ax.set_title(f"n={x['n']}", fontsize=9)
axes[0].set_ylabel("mean GAE per solve (derived: mean of record phases)")
h, l = axes[0].get_legend_handles_labels()
fig.legend(h, l, loc="lower center", ncol=5, fontsize=8)
fig.suptitle("Mean priced work per phase: oracle_setup falls, relations rises by the resolution cost", fontsize=10)
fig.tight_layout(rect=(0, 0.06, 1, 1)); fig.savefig(HERE / "fig3-phases.svg"); fig.savefig(HERE / "fig3-phases.png", dpi=150); plt.close(fig)
print("ok")

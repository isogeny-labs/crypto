"""Lays out recorded values only: figures/summary.json (the ecbench band rows,
the saved paired comparisons and per-arm phase means, collected by
make_summary.sh). Computes no result."""
import json, pathlib, re
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = pathlib.Path(__file__).resolve().parent
S = json.loads((HERE / "summary.json").read_text())["curves"]


def arm(c, prefix):
    return next(a["arm"] for a in c["arms"] if re.match(prefix, a["arm"]))


def cmp(c, a, b):
    m = next(x for x in c["comparisons"] if x["a"] == a and x["b"] == b)
    return m["ratio"], m["ci95"][0], m["ci95"][1]


def band(c, name):
    return next(r["mean_s"] for r in c["band"] if r["arm"] == name)


lab = [f"n={c['n']}, a={c['a']}\nlog2 r={c['log2r']:.1f}" for c in S]
C = dict(lazy="#6b7280", layout="#0f766e", gen="#7c3aed", rho="#b45309")
xs = list(range(len(S)))

# Figure 1: improvements over the lazy baseline, per curve (paired ratios)
fig, ax = plt.subplots(figsize=(10, 4.6))
series = [
    ("lazy ÷ layout (H2: orbit-layout table)", "layout", lambda c: cmp(c, arm(c, "ic-layout"), arm(c, "ic-lazy")), -0.2),
    ("layout ÷ gen-layout (H3: generator orbit + stride)", "gen", lambda c: cmp(c, arm(c, "ic-gen-layout"), arm(c, "ic-layout")), 0.0),
    ("lazy ÷ gen-layout (H4: net)", "rho", lambda c: cmp(c, arm(c, "ic-gen-layout"), arm(c, "ic-lazy")), 0.2),
]
for name, col, f, dx in series:
    v = [f(c) for c in S]
    ax.errorbar([x + dx for x in xs], [r[0] for r in v],
                yerr=[[r[0] - r[1] for r in v], [r[2] - r[0] for r in v]],
                fmt="o", color=C[col], capsize=3, ms=6, label=name)
ax.axhline(1, color="k", lw=0.8, ls="--")
ax.set_yscale("log")
ax.set_xticks(xs); ax.set_xticklabels(lab, fontsize=8)
ax.set_ylabel("paired cost ratio (>1: second arm cheaper)")
ax.set_title("Improvement over the lazy orbit IC baseline, per curve\n"
             "ecbench.gae, paired by target, cluster-bootstrap 95 % CI (ecbench compare)", fontsize=10)
ax.grid(alpha=0.3, which="both"); ax.legend(fontsize=8, loc="upper right")
fig.tight_layout(); fig.savefig(HERE / "fig1-improvement.svg"); fig.savefig(HERE / "fig1-improvement.png", dpi=150); plt.close(fig)

# Figure 2: the §5a band (S at w = 0, 0.1, 1) per arm against log2 r
fig, ax = plt.subplots(figsize=(10, 4.8))
lr = [c["log2r"] for c in S]
for key, prefix, name, mk in (("lazy", "ic-lazy", "IC lazy (baseline, #2158)", "s"),
                              ("gen", "ic-gen-layout", "IC gen-layout (this study)", "o")):
    b = [band(c, arm(c, prefix)) for c in S]
    ax.plot(lr, [v[0] for v in b], mk + "-", color=C[key], label=f"{name}, w=0", ms=5)
    ax.plot(lr, [v[2] for v in b], mk + ":", color=C[key], label=f"{name}, w=1", ms=5)
    ax.fill_between(lr, [v[0] for v in b], [v[2] for v in b], color=C[key], alpha=0.12)
ax.plot(lr, [band(c, "rho-frob")[0] for c in S], "^-", color=C["rho"], label="rho-frob (matched reference; band-invariant)", ms=6)
for c in S:
    if c["n"] == 19:
        ax.annotate("n19: no pinned calibration,\nIC natives not re-priced", (c["log2r"], band(c, arm(c, "ic-lazy"))[0]),
                    textcoords="offset points", xytext=(8, 18), fontsize=7)
ax.set_yscale("log"); ax.set_xlabel("log2 r"); ax.set_ylabel("mean S (GAE per sqrt r)")
ax.set_title("§5a band: native work at 0 and 1 GAE per operation (shaded), rho-frob for reference", fontsize=10)
ax.grid(alpha=0.3, which="both"); ax.legend(fontsize=7)
fig.tight_layout(); fig.savefig(HERE / "fig2-band-by-curve.svg"); fig.savefig(HERE / "fig2-band-by-curve.png", dpi=150); plt.close(fig)

# Figure 3: n = 29, where the work is (mean GAE per phase, calibration prices)
c = next(c for c in S if c["n"] == 29)
fig, ax = plt.subplots(figsize=(9.5, 4.6))
top = 0
pcol = {"factor_base": "#94a3b8", "oracle_setup": "#dc2626", "relations": "#2563eb", "linear_algebra": "#a855f7",
        "verify": "#16a34a", "setup": "#f59e0b", "search": "#0ea5e9", "natives": "#64748b", "internal_verification": "#16a34a"}
names = [arm(c, "ic-lazy"), arm(c, "ic-layout"), arm(c, "ic-gen-layout"), "rho-frob"]
seen = set()
for i, nm in enumerate(names):
    a = next(x for x in c["arms"] if x["arm"] == nm)
    bottom = 0
    for ph, v in a["phases"].items():
        ax.bar(i, v, bottom=bottom, color=pcol.get(ph, "#999"), label=None if ph in seen else ph)
        seen.add(ph); bottom += v
    tr = f"\n{a['trials']:.1f} trials" if a["trials"] else ""
    ax.text(i, bottom, f"{a['gae']:.0f} GAE{tr}", ha="center", va="bottom", fontsize=8)
    top = max(top, bottom)
ax.set_xticks(range(len(names))); ax.set_xticklabels(names, fontsize=8)
ax.set_ylabel("mean GAE per solve (calibration prices)")
ax.set_title("n = 29 (log2 r = 15.4): mean priced work per phase, 192 runs per arm", fontsize=10)
ax.set_ylim(0, top * 1.15)
ax.legend(fontsize=7, loc="upper left", bbox_to_anchor=(1.01, 1)); ax.grid(alpha=0.3, axis="y")
fig.tight_layout(); fig.savefig(HERE / "fig3-n29-phases.svg"); fig.savefig(HERE / "fig3-n29-phases.png", dpi=150); plt.close(fig)
print("figures written")

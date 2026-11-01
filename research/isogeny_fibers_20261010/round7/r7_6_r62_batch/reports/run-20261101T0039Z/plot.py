"""Plot recorded R7.6 values. No result is computed here: the counts,
intervals and fits come from the native example (`examples/r7_6_r62_batch.rs`)
and from ecbench; this script only reads them and draws (AGENTS.md
"Implementation language: no Python"). Run from anywhere:

    python3 plot.py            # writes the SVG/PNG files beside this script
"""
import glob, json, math, pathlib
import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = pathlib.Path(__file__).resolve().parent
study = here.parent.parent
runs = study / "runs"
sess = study / "sessions" / "pooled-vs-rho-l0" / "comparisons"

COL = {24: "#2a6fdb", 28: "#d9822b", 32: "#2a9d5c", 36: "#a23bb5"}


def save(fig, name):
    svg = here / (name + ".svg")
    fig.savefig(svg)
    t = svg.read_text(encoding="utf-8").splitlines()
    svg.write_text("\n".join(l.rstrip() for l in t) + "\n", encoding="utf-8")
    fig.savefig(here / (name + ".png"), dpi=150)


def load_batches():
    out = []
    for f in sorted(glob.glob(str(runs / "batch_r*_sym*_v1.json"))):
        d = json.load(open(f))
        a = d["args"]
        rb = int(a[a.index("--rbits") + 1])
        sym = int(a[a.index("--sym") + 1])
        out.append((rb, sym, d))
    return out


batches = load_batches()

# ---- Fig 1: gain against L, per r. ------------------------------------------
fig, axs = plt.subplots(1, 2, figsize=(11.5, 4.6))
ax = axs[0]
for rb, sym, d in batches:
    Ls = [s["L"] for s in d["summary"]]
    g = [s["gain_separate_over_pooled"] for s in d["summary"]]
    est = [x["est"] for x in g]
    lo = [x["est"] - x["lo"] for x in g]
    hi = [x["hi"] - x["est"] for x in g]
    nt = len(d["trials"])
    ax.errorbar(Ls, est, yerr=[lo, hi], marker="o" if sym else "s", ms=3.5, lw=1.2 if sym else 0.8,
                ls="-" if sym else "--", color=COL[rb], alpha=1 if sym else .6, capsize=2,
                label=f"r=2^{d['base_curve']['rbits_actual']:.1f}, {'p=±1 mod 5,7' if sym else 'p unrestricted'}, {nt} trials")
Ls = [s["L"] for s in batches[0][2]["summary"]]
ax.plot(Ls, [math.sqrt(l) for l in Ls], "k:", lw=1.2, label="√L (strict prediction)")
ax.plot(Ls, [1 / s["ks_prediction_ratio"] for s in batches[0][2]["summary"]], color="#888", lw=1.2,
        label="ideal Kuhn–Struik gain (closed form)")
ax.set_xscale("log", base=2); ax.set_yscale("log", base=2)
ax.set_xlabel("pool size L = class size C (targets, one per curve)")
ax.set_ylabel("gain = Σ separate / Σ pooled (counted group additions)")
ax.set_title("Pooled gain over separate runs, paired on keys and seeds (95% bootstrap over trials)", fontsize=8)
ax.legend(fontsize=6.2, loc="upper left"); ax.grid(alpha=.3, which="both")

ax = axs[1]
for rb, sym, d in batches:
    Ls = [s["L"] for s in d["summary"]]
    g = [s["gain_over_sqrt_L"] for s in d["summary"]]
    ax.errorbar(Ls, [x["est"] for x in g], yerr=[[x["est"] - x["lo"] for x in g], [x["hi"] - x["est"] for x in g]],
                marker="o" if sym else "s", ms=3, lw=1.2 if sym else 0.8, ls="-" if sym else "--", color=COL[rb],
                alpha=1 if sym else .55, capsize=2, label=f"2^{d['base_curve']['rbits_actual']:.1f}" + ("" if sym else " (p free)"))
ax.axhline(1.0, color="k", ls=":", lw=1.2, label="strict √L")
ks = [1 / s["ks_prediction_ratio"] / math.sqrt(s["L"]) for s in batches[0][2]["summary"]]
ax.plot([s["L"] for s in batches[0][2]["summary"]], ks, color="#888", lw=1.2, label="ideal Kuhn–Struik")
ax.set_xscale("log", base=2); ax.set_ylim(0.5, 1.15)
ax.set_xlabel("pool size L = class size C"); ax.set_ylabel("gain / √L")
ax.set_title("Same data divided by √L: the constant, and its approach to the ideal as r grows", fontsize=8)
ax.legend(fontsize=6.2, loc="lower left", ncol=2); ax.grid(alpha=.3, which="both")
fig.suptitle("R7.6 batch half (examples/r7_6_r62_batch.rs, runs/batch_r*_sym*_v1.json)", fontsize=9)
fig.tight_layout(); save(fig, "fig1_batch_gain_vs_L")

# ---- Fig 2: exponents over L; B/C. -------------------------------------------
fig, axs = plt.subplots(1, 2, figsize=(11.5, 4.4))
ax = axs[0]
xs = []
for i, (rb, sym, d) in enumerate(batches):
    e = d["exponents_over_L"]["gain_ln_on_ln_L"]
    x = d["base_curve"]["rbits_actual"] + (0.15 if sym else -0.15)
    ax.errorbar([x], [e["est"]], yerr=[[e["est"] - e["lo"]], [e["hi"] - e["est"]]], marker="o" if sym else "s",
                color=COL[rb], capsize=3, ls="none", alpha=1 if sym else .6)
ax.axhline(0.5, color="k", ls=":", label="√ prediction 0.5")
ks_slope = None
fitf = runs / "scaling_fit_v1.json"
if fitf.exists():
    ks_slope = json.load(open(fitf))["ks_ideal_gain_exponent_L2_to_256"]
    ax.axhline(ks_slope, color="#888", label=f"ideal Kuhn–Struik over L=2..256 ({ks_slope:.3f})")
ax.set_xlabel("log2 r of the base curve (circle: p=±1 mod 5,7; square: p unrestricted)")
ax.set_ylabel("exponent of the gain in L (log–log fit over L=2..256)")
ax.set_title("Exponent of the pooled gain in L with 95% bootstrap interval", fontsize=8)
ax.legend(fontsize=7); ax.grid(alpha=.3)

ax = axs[1]
for rb, sym, d in batches:
    Ls = [s["L"] for s in d["summary"]]
    g = [s["ratio_B_over_C"] for s in d["summary"]]
    off = 1 + 0.03 * (list(COL).index(rb)) + (0.0 if sym else 0.015)
    ax.errorbar([l * off for l in Ls], [x["est"] for x in g],
                yerr=[[x["est"] - x["lo"] for x in g], [x["hi"] - x["est"] for x in g]], marker="o" if sym else "s",
                ms=3, lw=0.8, ls="none", color=COL[rb], alpha=1 if sym else .55, capsize=2,
                label=f"2^{d['base_curve']['rbits_actual']:.1f}" + ("" if sym else " (p free)"))
ax.axhspan(0.95, 1.05, color="#cfe8cf", alpha=.6, label="[0.95, 1.05]")
ax.axhline(1.0, color="k", lw=.8)
ax.set_xscale("log", base=2); ax.set_ylim(0.75, 1.7)
ax.set_xlabel("pool size L"); ax.set_ylabel("B / C (transported over same-curve pooled)")
ax.set_title("Transport control: pooled cost of class targets over same-curve targets", fontsize=8)
ax.legend(fontsize=6.2, ncol=2, loc="upper right"); ax.grid(alpha=.3, which="both")
fig.suptitle("R7.6 batch half: exponent and control (runs/batch_r*_sym*_v1.json)", fontsize=9)
fig.tight_layout(); save(fig, "fig2_batch_exponent_and_control")

# ---- Fig 3: ecbench same-curve pooled gain. -----------------------------------
fig, ax = plt.subplots(figsize=(6.6, 4.5))
Ls = [4, 16, 64, 256]
cmp_ = {L: json.load(open(sess / f"rho-neg__pooled-L{L}.json")) for L in Ls}
est = [L / cmp_[L]["ops"]["ratio_b_over_a"] for L in Ls]
lo = [L / cmp_[L]["ops"]["ci95_bootstrap"][1] for L in Ls]
hi = [L / cmp_[L]["ops"]["ci95_bootstrap"][0] for L in Ls]
ax.errorbar(Ls, est, yerr=[[e - l for e, l in zip(est, lo)], [h - e for e, h in zip(est, hi)]], marker="o", color="k",
            capsize=3, label="all four curves (64 workloads, 128 pairs)")
bits = {}
for L in Ls:
    for c in cmp_[L]["curves"]:
        bits.setdefault(round(c["log2_r"]), []).append((L, c))
cols = {19: "#2a6fdb", 24: "#d9822b", 28: "#2a9d5c", 32: "#a23bb5"}
for b, rows in sorted(bits.items()):
    ax.errorbar([L * (1 + .02 * list(bits).index(b)) for L, _ in rows], [L / c["ratio_b_over_a"] for L, c in rows],
                yerr=[[L / c["ratio_b_over_a"] - L / c["ci95_bootstrap"][1] for L, c in rows],
                      [L / c["ci95_bootstrap"][0] - L / c["ratio_b_over_a"] for L, c in rows]],
                marker="s", ms=3, lw=.8, color=cols[b], alpha=.8, capsize=2, label=f"r≈2^{b} (16 workloads)")
ax.plot(Ls, [math.sqrt(L) for L in Ls], "k:", label="√L")
ax.set_xscale("log", base=2); ax.set_yscale("log", base=2)
ax.set_xlabel("batch L (record target + L−1 companions on the same curve)")
ax.set_ylabel("gain = L × S(rho.negation) / S(pooled)")
ax.set_title("ecbench session ECBS1h4b0bd52ebc3a (L0, counts): pooled vs single, ratio of totals\n"
             "gain = L / ecbench ratio (derived); 95% cluster bootstrap over workloads", fontsize=8)
ax.legend(fontsize=6.5, loc="upper left"); ax.grid(alpha=.3, which="both")
fig.tight_layout(); save(fig, "fig3_ecbench_pooled_gain")

# ---- Fig 4: path-finding cost against component size. -------------------------
sc = [json.load(open(f)) for f in sorted(glob.glob(str(runs / "scaling_v[0-9].json")))]
cls = [c for d in sc for c in d["classes"]]
fit = json.load(open(runs / "scaling_fit_v1.json")) if (runs / "scaling_fit_v1.json").exists() else None
fig, axs = plt.subplots(1, 2, figsize=(11.5, 4.6))
ax = axs[0]
xs = [c["component_size"] for c in cls]
ys = [c["mean_total"] for c in cls]
ys2 = [c["mean_search"] for c in cls]
ax.scatter(xs, ys, s=16, color="#2a6fdb", label="mean total per pair (search + certification + validation)")
ax.scatter(xs, ys2, s=12, marker="x", color="#d9822b", label="search enumerations only")
if fit:
    f = [x for x in fit["fits"] if x["subset"] == "all" and x["cost"] == "total"][0]
    gx = math.exp(sum(math.log(x) for x in xs) / len(xs))
    gy = math.exp(sum(math.log(y) for y in ys) / len(ys))
    xx = [min(xs), max(xs)]
    ax.plot(xx, [gy * (x / gx) ** f["exponent"] for x in xx], color="#2a6fdb", lw=1,
            label=f"fit: exponent {f['exponent']:.2f} [{f['lo']:.2f}, {f['hi']:.2f}], {f['classes']} classes")
    ax.plot(xx, [gy * (x / gx) ** 0.5 for x in xx], "k:", lw=1.2, label="slope 0.5 (√C) through the same centre")
    ax.plot(xx, [gy * (x / gx) ** 1.0 for x in xx], color="#888", ls="--", lw=1, label="slope 1 through the same centre")
ax.set_xscale("log"); ax.set_yscale("log")
ax.set_xlabel("component size C (curves, certified edges)"); ax.set_ylabel("counted enumerations per pair")
ax.set_title("Corrected bidirectional search: cost per pair against component size", fontsize=8)
ax.legend(fontsize=6.2, loc="upper left"); ax.grid(alpha=.3, which="both")
ax = axs[1]
ax.scatter(xs, [y / x for x, y in zip(xs, ys)], s=16, color="#2a6fdb", label="total / C")
ax.scatter(xs, [y / math.sqrt(x) for x, y in zip(xs, ys)], s=16, marker="^", color="#a23bb5", label="total / √C")
ax.set_xscale("log"); ax.set_yscale("log")
ax.set_xlabel("component size C"); ax.set_ylabel("ratio (derived from recorded means)")
ax.set_title("A flat total/C would be linear; a flat total/√C would be the hypothesis", fontsize=8)
ax.legend(fontsize=7); ax.grid(alpha=.3, which="both")
fig.suptitle(f"R7.6 path half: {len(cls)} classes, p from 12 to 34 bits (runs/scaling_v*.json, scaling_fit_v1.json)", fontsize=9)
fig.tight_layout(); save(fig, "fig4_path_cost_vs_component")
print("wrote figures in", here)

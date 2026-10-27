"""Lay out recorded values of e5a.json and e5b.json (written by the Rust
examples generator_effect_e5a and generator_effect_e5b). No result is
computed here; the only arithmetic is log10 of recorded p-values and the
log2 of the recorded expectation line."""
import json, math, os
os.environ.setdefault("MPLCONFIGDIR", os.environ.get("TMPDIR", "/tmp"))
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
a = json.load(open(os.path.join(here, "e5a.json")))
b = json.load(open(os.path.join(here, "e5b.json")))

setcol = {"gmul": "#c0392b", "rscalar": "#4c78a8", "rx": "#6b8e23"}
setlab = {"gmul": "[i]G (standard G)", "rscalar": "[s]G, s uniform", "rx": "uniform x on curve"}

fig = plt.figure(figsize=(17, 10.5), constrained_layout=True)
gs = fig.add_gridspec(2, 3)

# A, B: running minimum of x against index, per curve (record lows at B = 2^24).
for k, cv in enumerate(a["curves"]):
    ax = fig.add_subplot(gs[0, k])
    big = [x for x in cv["by_B"] if x["log2_B"] == 24][0]
    for s in ("rscalar", "rx", "gmul"):
        rec = big["sets"][s]["records"]
        xs = [math.log2(r["index"]) for r in rec]
        ys = [r["log2_x"] for r in rec]
        # step plot: the running minimum holds until the next record
        xs2, ys2 = [], []
        for i in range(len(xs)):
            xs2 += [xs[i], xs[i + 1] if i + 1 < len(xs) else 24.0]
            ys2 += [ys[i], ys[i]]
        ax.plot(xs2, ys2, color=setcol[s], lw=1.6 if s == "gmul" else 1.0,
                label=setlab[s], zorder=3 if s == "gmul" else 2)
    ell = cv["ell"]
    lp = math.log2(int(cv["p"], 16))
    ii = [i / 4 for i in range(0, 97)]
    ax.plot(ii, [lp - math.log2(2 ** i + 1) for i in ii], ls="--", color="black", lw=0.8,
            label="E[min] = p/(i+1)")
    for lb in (20, 24):
        mt = [x for x in cv["by_B"] if x["log2_B"] == lb][0]["sets"]["gmul"]["min_test"]
        ax.annotate(f"B=2^{lb}: log2(min/E) = {mt['log2_xmin_over_expectation']:+.2f}, p = {mt['p']:.2f}",
                    xy=(0.03, 0.06 + 0.07 * (lb == 20)), xycoords="axes fraction", fontsize=8.5)
    ax.set_xlabel("log2 i (index into the stream)")
    ax.set_ylabel("log2 of the smallest x so far")
    ax.set_title(f"{'AB'[k]}. {cv['curve']}: running minimum of x over 2^24 points ({ell}-bit p)")
    ax.set_xlim(0, 24)
    ax.grid(alpha=0.3)
    ax.legend(fontsize=8, loc="upper right")

# C: the family of 20 tests, -log10 p against the Holm threshold.
ax = fig.add_subplot(gs[0, 2])
h = a["holm"]
order = sorted(h, key=lambda t: t["id"])
ys = list(range(len(order)))
ax.barh(ys, [-math.log10(max(t["p"], 1e-300)) for t in order],
        color=["#c0392b" if t["reject"] else "#7f7f7f" for t in order])
ax.plot([-math.log10(t["holm_threshold"]) for t in order], ys, "k|", ms=9, label="Holm threshold for that test's rank")
ax.axvline(-math.log10(a["alpha"] / a["family_size"]), color="black", ls=":", lw=0.8, label="0.05/20 (first Holm step)")
ax.set_yticks(ys)
ax.set_yticklabels([t["id"].replace("gmul-vs-", "vs ") for t in order], fontsize=7)
ax.invert_yaxis()
ax.set_xlabel("-log10 p")
ax.set_title(f"C. E5a family: 20 tests, Holm at 0.05; rejections = {a['rejections']}")
ax.legend(fontsize=8, loc="lower right")
ax.grid(alpha=0.3, axis="x")

# D, E: E5b model, distance to rho per key against keys T, each arm at its optimal size.
series = [("knownlog_random_to_rho_ks", "known-log random vs shared rho (Kuhn-Struik)", "#e0a030", "-"),
          ("knownlog_random_to_rho_indep", "known-log random vs independent rho", "#e0a030", ":"),
          ("abscissa_to_rho_ks", "abscissa vs shared rho", "#6b8e23", "-"),
          ("bsgs_opt_to_rho_ks", "BSGS vs shared rho", "#b07aa1", "-")]
for k, cv in enumerate(b["curves"]):
    ax = fig.add_subplot(gs[1, k])
    for w, mk in ((0.0, "o"), (1.0, "s")):
        bw = [x for x in cv["by_w"] if x["w"] == w][0]
        T = [o["log2_T"] for o in bw["optimal_B"]]
        for key, lab, c, ls in series:
            ax.plot(T, [o["log2_distance"][key] for o in bw["optimal_B"]], ls=ls, marker=mk, ms=3.5,
                    color=c, alpha=1.0 if w == 0 else 0.55, label=f"{lab}, w={w:g}")
    ax.axhline(0, color="black", lw=0.8)
    ax.axvline(cv["log2_r"] / 4, color="grey", ls="--", lw=0.7)
    ax.text(cv["log2_r"] / 4 + 0.5, 20, "T = r^(1/4):\nKuhn-Struik\nregime ends", fontsize=7.5, color="grey")
    ax.set_xlabel("log2 T (keys under one G)")
    ax.set_ylabel("log2 (S per key / rho's S per key)")
    ax.set_title(f"{'DE'[k]}. MODEL, {cv['curve']}: distance to rho, optimal sizes")
    ax.grid(alpha=0.3)
    if k == 0:
        ax.legend(fontsize=6.8, loc="lower left")

# F: E5b at the plan's equal B = r^(1/3): distance to rho at T = 1, and break-even T.
ax = fig.add_subplot(gs[1, 2])
rows = []
for cv in b["curves"]:
    for bw in cv["by_w"]:
        eq = bw["equal_B"]
        rows.append((f"{cv['curve']} w={bw['w']:g}",
                     eq["log2_distance_to_rho_T1"]["knownlog_random"],
                     eq["log2_distance_to_rho_T1"]["abscissa"],
                     eq["log2_break_even_T"]["knownlog_random_vs_rho_indep"]))
ys = list(range(len(rows)))
ax.barh([y - 0.2 for y in ys], [r[1] for r in rows], height=0.4, color="#e0a030", label="known-log random: log2(S / S_rho) at T=1")
ax.barh([y + 0.2 for y in ys], [r[2] for r in rows], height=0.4, color="#6b8e23", label="abscissa: log2(S / S_rho) at T=1")
ax.plot([r[3] for r in rows], ys, "kD", ms=5, label="log2 T to break even with independent rho (known-log)")
ax.set_yticks(ys)
ax.set_yticklabels([r[0] for r in rows], fontsize=8)
ax.invert_yaxis()
ax.set_xlabel("log2")
ax.set_title("F. MODEL at the plan's B = ceil(r^(1/3)): distance to rho")
ax.legend(fontsize=7.5, loc="upper center", bbox_to_anchor=(0.5, -0.12), ncol=1)
ax.grid(alpha=0.3, axis="x")

fig.suptitle("E5: P-192 and P-224. Top: E5a x-statistics of the first 2^24 multiples of G (measured, "
             "seed 20261019). Bottom: E5b cost model from dry run 2 constants (MODEL, not a feasibility claim)",
             fontsize=11)
fig.savefig(os.path.join(here, "e5.svg"))
fig.savefig(os.path.join(here, "e5.png"), dpi=150)

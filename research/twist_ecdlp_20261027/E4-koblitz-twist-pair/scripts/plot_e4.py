# Plots recorded values only (out/stage2_m*.csv). No computation of new results.
import csv, glob, statistics as st
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

rows = []
for f in sorted(glob.glob("out/stage2_m*.csv")):
    names = None
    with open(f) as fh:
        for line in fh:
            line = line.strip()
            if not line:
                continue
            if line.startswith("# fields: "):
                names = line[len("# fields: "):].split(",")
                continue
            if line.startswith("#"):
                continue
            rows.append(dict(zip(names, [float(x) for x in line.split(",")])))

ms = sorted({int(r["m"]) for r in rows})
col = {11: "#1f77b4", 13: "#2ca02c", 17: "#d62728", 19: "#9467bd"}
fig, ax = plt.subplots(1, 3, figsize=(15, 4.2))
for m in ms:
    rr = [r for r in rows if int(r["m"]) == m and int(r["pop"]) == 1]
    ax[0].scatter([r["l"] + 0.04 * (ms.index(m) - 1.5) for r in rr], [r["S_solves0"] / r["E_solves0"] for r in rr], s=10, color=col[m], label=f"m={m}")
    ax[1].scatter([r["l"] + 0.04 * (ms.index(m) - 1.5) for r in rr],
                  [st.median([r["tS%d" % k] / r["tE%d" % k] for k in (1, 2, 3) if r["tE%d" % k] > 0] or [float("nan")]) for r in rr], s=10, color=col[m], label=f"m={m}")
    ls = sorted({int(r["l"]) for r in rr if r["pc1_ok"] == 1})
    ax[2].plot(ls, [sum(r["P_rel0"] for r in rr if int(r["l"]) == l) / max(1, sum(r["E_rel0"] for r in rr if int(r["l"]) == l)) for l in ls], "o-", color=col[m], label=f"m={m}")
ax[0].set_title("Counted solves, shared / E-only (per V draw)"); ax[0].set_xlabel("l"); ax[0].set_ylabel("ratio"); ax[0].axhline(1, color="k", lw=0.5)
ax[1].set_title("CPU-time ratio S/E (median of 3 repeats)"); ax[1].set_xlabel("l"); ax[1].axhline(1, color="k", lw=0.5)
ax[2].set_title("PC1: relations (l+1)/l, E-only base"); ax[2].set_xlabel("l"); ax[2].axhspan(3.0, 5.0, color="0.9"); ax[2].axhline(4, color="k", lw=0.5)
for a in ax:
    a.legend(fontsize=7)
fig.suptitle("E4 Koblitz K0/K1 twist pair: shared base gives identical K0 relations (Delta_rel=0) at ~2x counted cost")
fig.tight_layout()
fig.savefig("out/e4_summary.png", dpi=130)
fig.savefig("out/e4_summary.svg")
print("wrote out/e4_summary.png and out/e4_summary.svg")

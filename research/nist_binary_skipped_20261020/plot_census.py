"""Lay out the recorded census. Plotting only: every value comes from
census/summary.json, census/koblitz_sweep.txt (this study) or
../nist_seed_provenance_20261016/census/summary.json (the prime runs);
nothing is computed here beyond reading those files."""
import json, pathlib
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = pathlib.Path(__file__).parent
binary = json.loads((HERE / "census" / "summary.json").read_text())["runs"]
prime = json.loads((HERE.parent / "nist_seed_provenance_20261016" / "census" / "summary.json").read_text())
PRIME_PUB = {"863711343": [2185, 2320, 3048], "863966734": [139, 5228, 6013], "871002724": [1136],
             "871696354": [1842], "872952608": [2688], "891607927": [7026]}
PRIME_NAME = {"863711343": "p239 (3 curves)", "863966734": "P-192 + 2", "871002724": "P-521",
              "871696354": "P-256", "872952608": "P-384", "891607927": "P-224"}
sweep, levels = [], []
for line in (HERE / "census" / "koblitz_sweep.txt").read_text().splitlines():
    w = line.split()
    if line.startswith("# level"):
        mode = "lv"; continue
    if line.startswith("#"):
        mode = "sw"; continue
    (sweep if mode == "sw" else levels).append(w)

fig, (ax, bx) = plt.subplots(1, 2, figsize=(13, 5.2), gridspec_kw={"width_ratios": [1.15, 1]})

# (a) rank of the published seed among accepted candidates, per run
rows = []
for r in prime:
    t = r["run"][8:-1]; pub = PRIME_PUB[t]; acc = sorted(r["accepted"])
    rows.append((f"{PRIME_NAME[t]}  srand48({t})", acc, pub))
for r in binary:
    rows.append((f"{r['curve']}  lrand48 run", sorted(r["accepted"]), [r["published"]]))
for y, (label, acc, pub) in enumerate(reversed(rows)):
    for k, i in enumerate(acc, start=1):
        if i in pub:
            ax.scatter(k, y, s=80, color="#1f4e9c", zorder=3)
        else:
            ax.scatter(k, y, s=80, facecolors="white", edgecolors="#c0392b", linewidths=1.6, zorder=3)
    ax.hlines(y, 1, max(len(acc), 1), color="#c9ccd3", lw=1, zorder=0)
ax.scatter([], [], s=80, color="#1f4e9c", label="published")
ax.scatter([], [], s=80, facecolors="white", edgecolors="#c0392b", linewidths=1.6, label="accepted, not published")
ax.axhline(3.5, color="#888", lw=0.8, ls="--")
ax.text(8.3, 1.5, "binary runs\n(this study)", fontsize=8, ha="right", va="center", color="#444")
ax.text(8.3, 6.5, "prime runs\n(2026-10-16 census)", fontsize=8, ha="right", va="center", color="#444")
ax.set_yticks(range(len(rows)))
ax.set_yticklabels([r[0] for r in reversed(rows)], fontsize=8)
ax.set_xticks(range(1, 9))
ax.set_xlabel("k-th accepted candidate of the run (prime order, MOV degree >= 2^10)")
ax.set_title("(a) Accepted candidates up to the last published seed", fontsize=10)
ax.legend(fontsize=8, loc="center right", bbox_to_anchor=(1.0, 0.43))

# (b) Koblitz degree sweep: bits(n) against m with the 2s thresholds
ms = [int(w[0]) for w in sweep]; bits = [int(w[3]) for w in sweep]
fips = [w[5] == "fips" for w in sweep]
bx.scatter([m for m, f in zip(ms, fips) if not f], [b for b, f in zip(bits, fips) if not f],
           s=30, facecolors="white", edgecolors="#555", label="almost-prime K_a, not chosen")
bx.scatter([m for m, f in zip(ms, fips) if f], [b for b, f in zip(bits, fips) if f],
           s=50, color="#1f4e9c", label="FIPS 186-2 K-curve")
for lv in levels:
    need = int(lv[1]); bx.axhline(need, color="#c9ccd3", lw=0.8)
    bx.text(105, need + 6, f"{lv[0]}-bit level: n >= 2^{need}", fontsize=7, color="#666")
    if lv[2] != lv[5]:
        m0, b0 = int(lv[2]), int(lv[4])
        bx.scatter([m0], [b0], s=140, facecolors="none", edgecolors="#c0392b", linewidths=2,
                   label=f"first qualifying degree, skipped (m = {m0})")
for m, b, f in zip(ms, bits, fips):
    if f or m in (131, 239, 277):
        off = {277: (-20, 2), 239: (-6, 7), 283: (6, -4), 233: (6, -9)}.get(m, (4, -10))
        bx.annotate(str(m), (m, b), xytext=off, textcoords="offset points", fontsize=7)
bx.set_xlabel("extension degree m (prime)")
bx.set_ylabel("bits of the prime subgroup order n")
bx.set_title("(b) Almost-prime Koblitz curves, prime m in [100, 600]", fontsize=10)
bx.legend(fontsize=7.5, loc="lower right")
fig.suptitle("NIST binary curves: skipped candidates (research/nist_binary_skipped_20261020, census.sh)", fontsize=11)
fig.tight_layout()
fig.savefig(HERE / "census.svg")
fig.savefig(HERE / "census.png", dpi=150)

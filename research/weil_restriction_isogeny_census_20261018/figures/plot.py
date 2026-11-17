"""Lay out recorded values from a run's CSVs. Computes nothing new.

Usage: python plot.py <run-dir> <out-dir>
Reads kernels.csv, summary.csv and by_class.csv written by
examples/weil_restriction_isogeny_census.rs and draws:
  affected.svg    fraction of curves over F_{q^3} with a Tian kernel, by q and ℓ
  kernel_map.svg  n = 3: Tian kernels by (ℓ, q mod ℓ, class of φ)
  n2n4.svg        n = 2 and n = 4: classes with and without a Tian kernel
Each SVG also gets a 150 dpi PNG copy from the same figure.
"""

import csv
import sys
from collections import defaultdict
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402
from matplotlib.patches import Rectangle  # noqa: E402

BLUE, ORANGE, AQUA, PURPLE, RED = "#2a78d6", "#eb6834", "#1baf7a", "#8e5bd6", "#c0392b"
INK, INK2, GRID, PALE = "#0b0b0b", "#52514e", "#d9d8d4", "#f2f1ee"

run, out = Path(sys.argv[1]), Path(sys.argv[2])
out.mkdir(parents=True, exist_ok=True)
STAMP = run.name


def read(name):
    with open(run / name, newline="") as f:
        return list(csv.DictReader(f))


def save(fig, name):
    fig.savefig(out / f"{name}.svg", bbox_inches="tight")
    fig.savefig(out / f"{name}.png", dpi=150, bbox_inches="tight")
    plt.close(fig)


def style(ax):
    ax.spines["top"].set_visible(False)
    ax.spines["right"].set_visible(False)
    ax.grid(axis="y", color=GRID, lw=0.6)
    ax.set_axisbelow(True)
    ax.tick_params(colors=INK2, labelsize=8)


# --- affected.svg --------------------------------------------------------
summary = read("summary.csv")
qs = sorted({int(r["q"]) for r in summary})
per_q = {}
for r in summary:
    q = int(r["q"])
    e = per_q.setdefault(
        q,
        {
            "classes": int(r["classes"]),
            "prime": int(r["prime_order"]),
            "cof2": int(r["cofactor_2"]),
            "prime_any": int(r["prime_any_odd_ell"]),
            "all_any": int(r["all_any_odd_ell"]),
            "ell": {},
        },
    )
    e["ell"][int(r["ell"])] = (
        int(r["with_tian"]),
        int(r["prime_with_tian"]),
        int(r["cof2_with_tian"]),
    )

fig, axes = plt.subplots(1, 2, figsize=(10.5, 3.8))
ax = axes[0]
x = range(len(qs))
w = 0.38
ax.bar(
    [i - w / 2 for i in x],
    [per_q[q]["prime_any"] / per_q[q]["prime"] for q in qs],
    w,
    color=RED,
    label="prime-order curves, kernel at some ℓ ∈ {3,5,7}",
)
ax.bar(
    [i + w / 2 for i in x],
    [per_q[q]["all_any"] / per_q[q]["classes"] for q in qs],
    w,
    color=BLUE,
    label="all curves, kernel at some ℓ ∈ {3,5,7}",
)
for i, q in enumerate(qs):
    ax.text(
        i - w / 2,
        per_q[q]["prime_any"] / per_q[q]["prime"] + 0.01,
        f"{per_q[q]['prime_any']}/{per_q[q]['prime']}",
        ha="center",
        fontsize=7,
        color=INK2,
    )
    ax.text(
        i + w / 2,
        per_q[q]["all_any"] / per_q[q]["classes"] + 0.01,
        f"{per_q[q]['all_any']}/{per_q[q]['classes']}",
        ha="center",
        fontsize=7,
        color=INK2,
    )
ax.set_xticks(list(x))
ax.set_xticklabels([f"q = {q}" for q in qs])
ax.set_ylim(0, 1)
ax.set_ylabel("fraction of F_{q^3}-isomorphism classes", fontsize=8)
ax.set_title("Curves over F_{q^3} with an F_q-rational Tian kernel (ℓ ≠ p)", fontsize=9)
ax.legend(fontsize=7, frameon=False, loc="upper left")
style(ax)

ax = axes[1]
ells = [2, 3, 5, 7]
cols = {2: INK2, 3: AQUA, 5: ORANGE, 7: PURPLE}
w = 0.2
for k, l in enumerate(ells):
    vals = []
    for q in qs:
        v = per_q[q]["ell"].get(l)
        vals.append(v[1] / per_q[q]["prime"] if v else 0.0)
    ax.bar([i + (k - 1.5) * w for i in x], vals, w, color=cols[l], label=f"ℓ = {l}")
    for i, q in enumerate(qs):
        if l == q:
            ax.text(i + (k - 1.5) * w, 0.01, "ℓ=p", ha="center", fontsize=6, color=INK2, rotation=90)
ax.set_xticks(list(x))
ax.set_xticklabels([f"q = {q}" for q in qs])
ax.set_ylim(0, 0.6)
ax.set_ylabel("fraction of prime-order curves", fontsize=8)
ax.set_title("Prime-order curves with a Tian kernel, by ℓ (none at ℓ = 2: check 8)", fontsize=9)
ax.legend(fontsize=7, frameon=False)
style(ax)
fig.text(0.01, -0.02, f"source: {STAMP}/summary.csv (exact counts)", fontsize=7, color=INK2)
save(fig, "affected")

# --- kernel_map.svg -------------------------------------------------------
kern = [r for r in read("kernels.csv") if int(r["n"]) == 3]
fig, axes = plt.subplots(1, 4, figsize=(12.5, 4.6), gridspec_kw={"width_ratios": [1, 1.4, 2.2, 3]})
order = {"scalar": 0, "jordan": 1, "split": 2, "nonsplit": 3}
for ax, l in zip(axes, ells):
    rows = [r for r in kern if int(r["ell"]) == l]
    qbars = sorted({int(r["qbar"]) for r in rows})
    labels = sorted(
        {(order[r["class"]], r["class"], int(r["tr"]), int(r["det"])) for r in rows}
    )
    ylab = {lab: i for i, lab in enumerate(labels)}
    for r in rows:
        lab = (order[r["class"]], r["class"], int(r["tr"]), int(r["det"]))
        xi = qbars.index(int(r["qbar"]))
        yi = ylab[lab]
        t = int(r["tian_kernels"])
        eig1 = r["eigenvalue_one"] == "true"
        face = (RED if not eig1 else ORANGE) if t > 0 else PALE
        ax.add_patch(Rectangle((xi, yi), 1, 1, facecolor=face, edgecolor="white", lw=1.2))
        ax.text(
            xi + 0.5,
            yi + 0.5,
            str(t) if t > 0 else "·",
            ha="center",
            va="center",
            fontsize=7.5,
            color="white" if t > 0 else INK2,
        )
    ax.set_xlim(0, len(qbars))
    ax.set_ylim(len(labels), 0)
    ax.set_xticks([i + 0.5 for i in range(len(qbars))])
    ax.set_xticklabels([str(q) for q in qbars], fontsize=7)
    ax.set_yticks([i + 0.5 for i in range(len(labels))])
    ax.set_yticklabels([f"{c} tr{tr} det{d}" for _, c, tr, d in labels], fontsize=6.5)
    ax.set_xlabel("q mod ℓ", fontsize=8)
    ax.set_title(f"ℓ = {l}", fontsize=9)
    for s in ax.spines.values():
        s.set_visible(False)
    ax.tick_params(length=0)
fig.suptitle(
    "n = 3: number of Tian kernels by (ℓ, q mod ℓ, class of φ = π_{q^3}|E[ℓ]); red: no eigenvalue 1 (prime order possible), orange: eigenvalue 1, dot: none",
    fontsize=8.5,
)
fig.text(0.01, -0.02, f"source: {STAMP}/kernels.csv (exact; one row per conjugacy class)", fontsize=7, color=INK2)
fig.tight_layout()
save(fig, "kernel_map")

# --- n2n4.svg -------------------------------------------------------------
kall = read("kernels.csv")
fig, ax = plt.subplots(figsize=(8, 3.4))
cells = sorted({(int(r["n"]), int(r["ell"])) for r in kall})
tot = defaultdict(int)
withk = defaultdict(int)
withk_noeig = defaultdict(int)
for r in kall:
    key = (int(r["n"]), int(r["ell"]))
    tot[key] += 1
    if int(r["tian_kernels"]) > 0:
        withk[key] += 1
        if r["eigenvalue_one"] != "true":
            withk_noeig[key] += 1
x = range(len(cells))
w = 0.27
ax.bar([i - w for i in x], [tot[c] for c in cells], w, color=GRID, label="conjugacy classes (det = q̄^n)")
ax.bar([i for i in x], [withk[c] for c in cells], w, color=BLUE, label="with a Tian kernel")
ax.bar([i + w for i in x], [withk_noeig[c] for c in cells], w, color=RED, label="with a Tian kernel and no eigenvalue 1")
for i, c in enumerate(cells):
    ax.text(i, withk[c] + 0.6, str(withk[c]), ha="center", fontsize=7, color=INK2)
    ax.text(i - w, tot[c] + 0.6, str(tot[c]), ha="center", fontsize=7, color=INK2)
ax.set_xticks(list(x))
ax.set_xticklabels([f"n={n}\nℓ={l}" for n, l in cells], fontsize=7)
ax.set_ylabel("classes [φ] over all q mod ℓ", fontsize=8)
ax.set_title("Kernel census by (n, ℓ): classes of φ with an F_q-rational Tian kernel (n = 4: Jacobian question open)", fontsize=9)
ax.legend(fontsize=7, frameon=False)
style(ax)
fig.text(0.01, -0.04, f"source: {STAMP}/kernels.csv", fontsize=7, color=INK2)
save(fig, "n2n4")
print("wrote", sorted(p.name for p in out.iterdir()))

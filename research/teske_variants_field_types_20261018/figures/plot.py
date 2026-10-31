"""Lay out recorded values from a run's CSVs. Computes nothing new.

Usage: python plot.py <run-dir> <out-dir> [census-run-dir]
Reads binary_summary.csv, binary_bits.csv, oddchar_types.csv,
oddchar_classes.csv and census_summary.csv written by
examples/teske_variants_field_types.rs and draws:
  margins.svg        binary and odd-characteristic margins a = delta - n/2 by n
  oddchar_counts.svg exact weak-j counts against q^{r-3} by (n, type)
  oddchar_classes.svg share of curves in isogeny classes with a weak member
  census.svg         Res-shaped genus-3 hyperelliptic classes by signature
Each SVG also gets a 150 dpi PNG copy from the same figure.
"""

import csv
import sys
from collections import defaultdict
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

BLUE, ORANGE, AQUA, PURPLE, RED = "#2a78d6", "#eb6834", "#1baf7a", "#8e5bd6", "#c0392b"
INK, INK2, GRID = "#0b0b0b", "#52514e", "#d9d8d4"

run, out = Path(sys.argv[1]), Path(sys.argv[2])
census_run = Path(sys.argv[3]) if len(sys.argv) > 3 else run
out.mkdir(parents=True, exist_ok=True)
STAMP = run.name if census_run == run else f"{run.name} (census: {census_run.name})"


def rows(name):
    src = census_run if name.startswith("census") else run
    with open(src / name, newline="") as f:
        return list(csv.DictReader(f))


def save(fig, name):
    fig.savefig(out / f"{name}.svg", bbox_inches="tight")
    fig.savefig(out / f"{name}.png", dpi=150, bbox_inches="tight")
    plt.close(fig)


plt.rcParams.update({"font.size": 9, "axes.edgecolor": INK2, "axes.labelcolor": INK,
                     "xtick.color": INK2, "ytick.color": INK2, "axes.titlesize": 9.5})

# ---------------------------------------------------------------- margins
bs = rows("binary_summary.csv")
ot = rows("oddchar_types.csv")
fig, (ax, bx) = plt.subplots(1, 2, figsize=(10.4, 3.9), gridspec_kw={"width_ratios": [1.5, 1]})
ns = [int(r["n"]) for r in bs if int(r["n"]) <= 63]
for key, col, lab in [("margin_typeI", BLUE, "GHS type I (genus 2^{m-1}-1)"),
                      ("margin_typeII", ORANGE, "GHS type II (genus 2^{m-1})"),
                      ("margin_gghs", AQUA, "gGHS (Hess), smallest genus")]:
    ys = [float(r[key]) if r[key] not in ("", "NaN") else float("nan") for r in bs if int(r["n"]) <= 63]
    ax.plot(ns, ys, "o-", ms=3.5, lw=1, color=col, label=lab)
ax.axhline(0, color=INK2, lw=0.8, ls="--")
ax.set_xlabel("odd extension degree n (binary field F_{2^{nl}})")
ax.set_ylabel("margin a = δ − n/2  (units of log₂ q = l bits)")
ax.set_title("Binary: margin of the smallest-genus descent, exact from the cosets of 2 mod n")
ax.grid(color=GRID, lw=0.6)
ax.legend(fontsize=7.5, loc="upper left")
for n0, txt, dy in [(7, "n=7: type I +0.5, type II −0.5", -14), (15, "n=15", 6), (31, "n=31", 6)]:
    r = next(r for r in bs if int(r["n"]) == n0)
    ax.annotate(txt, (n0, float(r["margin_typeI"])), textcoords="offset points", xytext=(6, dy), fontsize=7, color=INK2)
# odd characteristic: one point per (n, smallest-genus type), delta = n + 3 - r
best = {}
for r in ot:
    n, g, rr = int(r["n"]), int(r["genus"]), int(r["r"])
    if n not in best or g < best[n][0]:
        best[n] = (g, rr)
xs = sorted(best)
ys = [(n + 3 - best[n][1]) - n / 2 for n in xs]
bx.bar(xs, ys, color=[AQUA if y > 0 else ORANGE for y in ys], width=0.6)
bx.axhline(0, color=INK2, lw=0.8, ls="--")
for n in xs:
    bx.annotate(f"g={best[n][0]}", (n, ys[xs.index(n)]), textcoords="offset points",
                xytext=(0, 5 if ys[xs.index(n)] >= 0 else -11), ha="center", fontsize=7.5, color=INK2)
bx.set_xlabel("extension degree n (odd characteristic F_{q^n})")
bx.set_ylabel("margin a = δ − n/2, δ = n + 3 − r")
bx.set_title("Odd characteristic: smallest-genus type per n")
bx.grid(color=GRID, lw=0.6, axis="y")
fig.suptitle(f"Where a hidden member is sparser than the isogeny class (a > 0). Run {STAMP}; exponents, constants dropped.", fontsize=8.5, color=INK2, y=1.02)
save(fig, "margins")

# ---------------------------------------------------------------- odd-char counts
by = defaultdict(list)
for r in ot:
    by[(int(r["n"]), r["orbit_sizes"], int(r["m"]), int(r["r"]), int(r["genus"]))].append((int(r["q"]), int(r["weak_j"]), float(r["ratio_weak_j_over_q_pow"])))
fig, axes = plt.subplots(2, 3, figsize=(11, 6.4))
for ax, n in zip(axes.flat, [2, 3, 4, 5, 6, 7]):
    for (nn, sizes, m, rr, g), pts in sorted(by.items()):
        if nn != n:
            continue
        pts.sort()
        ax.plot([p[0] for p in pts], [p[2] for p in pts], "o-", ms=3.5, lw=1, label=f"{sizes} (m={m}, r={rr}, g={g})")
    ax.set_xscale("log")
    ax.set_yscale("log")
    ax.set_xticks([3, 5, 7, 11, 13])
    ax.set_xticklabels(["3", "5", "7", "11", "13"])
    ax.minorticks_off()
    ax.set_title(f"n = {n}: weak j-invariants / q^(r−3)")
    ax.set_xlabel("q")
    ax.grid(color=GRID, lw=0.6, which="both")
    ax.legend(fontsize=6.3, loc="best")
fig.suptitle(f"Exact count of weak j-invariants per branch type, divided by the predicted q^(r−3) (H_B). Run {STAMP}.", fontsize=9, color=INK2)
fig.tight_layout()
save(fig, "oddchar_counts")

# ---------------------------------------------------------------- odd-char classes
oc = rows("oddchar_classes.csv")
byc = defaultdict(list)
for r in oc:
    byc[(int(r["n"]), r["orbit_sizes"], int(r["m"]), int(r["genus"]))].append((int(r["q"]), float(r["share_curves_in_covered"]), float(r["mean_weak_per_covered_class"]), float(r["mean_walk_length"])))
fig, (ax, bx) = plt.subplots(1, 2, figsize=(10.4, 4))
markers = {2: "o", 3: "s", 4: "^", 5: "D", 6: "v", 7: "P"}
for (n, sizes, m, g), pts in sorted(byc.items()):
    pts.sort()
    ax.plot([p[0] for p in pts], [p[1] for p in pts], marker=markers[n], ms=4, lw=1, label=f"n={n} {sizes} g={g}")
    bx.plot([p[0] for p in pts], [p[2] for p in pts], marker=markers[n], ms=4, lw=1, label=f"n={n} {sizes} g={g}")
for a in (ax, bx):
    a.set_xscale("log")
    a.set_xticks([3, 5, 7, 11, 13])
    a.set_xticklabels(["3", "5", "7", "11", "13"])
    a.minorticks_off()
    a.set_xlabel("q")
    a.grid(color=GRID, lw=0.6, which="both")
ax.set_ylabel("share of all curves over F_{q^n} in a class with a weak member")
ax.set_title("Isogeny-class coverage (all-j trace table, exact)")
bx.set_yscale("log")
bx.set_ylabel("mean number of weak curves per covered class")
bx.set_title("Weak members per covered class")
ax.legend(fontsize=6, ncol=2)
fig.suptitle(f"Exact class statistics over toy fields. Run {STAMP}.", fontsize=9, color=INK2)
fig.tight_layout()
save(fig, "oddchar_classes")

# ---------------------------------------------------------------- census
cs = rows("census_summary.csv")
fig, ax = plt.subplots(figsize=(6.4, 3.8))
qs = [int(r["q"]) for r in cs]
sig = [int(r["classes_signature"]) for r in cs]
nosig = [int(r["classes_no_signature"]) for r in cs]
na = [int(r["classes_roots_outside_Fq6"]) for r in cs]
x = range(len(qs))
ax.bar(x, sig, color=AQUA, label="GHS compositum signature")
ax.bar(x, nosig, bottom=sig, color=ORANGE, label="no signature (other isogeny)")
ax.bar(x, na, bottom=[a + b for a, b in zip(sig, nosig)], color=PURPLE, label="roots outside F_{q^6}")
ax.set_xticks(list(x))
ax.set_xticklabels([str(q) for q in qs])
ax.set_yscale("log")
ax.set_ylim(1, max(a + b + c for a, b, c in zip(sig, nosig, na)) * 2.5)
ax.set_xlabel("q")
ax.set_ylabel("classes of genus-3 hyperelliptic C/F_q with Jac(C) ~ Res E")
ax.set_title("Weil-restriction-shaped genus-3 hyperelliptic Jacobians over F_q")
for i, (s, ns) in enumerate(zip(sig, nosig)):
    ax.annotate(f"{s} / {s + ns}", (i, s + ns), textcoords="offset points", xytext=(0, 3), ha="center", fontsize=7.5)
ax.legend(fontsize=7.5)
ax.grid(color=GRID, lw=0.6, axis="y")
fig.suptitle(f"Exact census (every squarefree octic and septic over F_q). Run {STAMP}.", fontsize=8.5, color=INK2, y=1.0)
save(fig, "census")
print("figures written to", out)

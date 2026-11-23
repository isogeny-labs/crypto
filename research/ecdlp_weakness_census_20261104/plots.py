#!/usr/bin/env python3
"""Lay out the recorded census values as SVG figures. Plots only what the
Rust censuses wrote to results/*.json; computes nothing new."""
import json
import math
import os
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
FIG = os.path.join(HERE, "figures")
os.makedirs(FIG, exist_ok=True)


def save(fig, name):
    fig.savefig(os.path.join(FIG, name + ".svg"))
    png_dir = os.environ.get("PNG_DIR")
    if png_dir:
        fig.savefig(os.path.join(png_dir, name + ".png"), dpi=80)


def load(name):
    with open(os.path.join(HERE, name, "results", f"{name}.json")) as f:
        return json.load(f)


def pq2():
    path = os.path.join(HERE, "pq2_rank", "results", "pq2_rank.json")
    if not os.path.exists(path):
        print("pq2_rank results missing; skipped")
        return
    rows = json.load(open(path))["rows"]
    fig, ax = plt.subplots(figsize=(8, 4.2))
    for key, label, marker in (("low3", "one-shot degree 3", "s"), ("low4", "one-shot degree 4 (n <= 20)", "^"),
                               ("iter3", "iterated degree 3 (F4-style closure)", "D"),
                               ("iter4", "iterated degree 4 (n <= 16)", "P")):
        xs = [r["n"] for r in rows if r[key] is not None]
        ys = [r[key] for r in rows if r[key] is not None]
        ax.scatter(xs, ys, s=18, marker=marker, label=label, alpha=0.8)
    ax.scatter([r["n"] for r in rows], [r["k"] for r in rows], s=18, marker="o", facecolors="none",
               edgecolors="k", label="k = N - affdim(Z) (linear consequences)")
    ax.set_xlabel("n (field F_{2^n}); 3 instances per n")
    ax.set_ylabel("dimension")
    ax.set_title("m = 2 descended S3: linear polynomials reachable at Macaulay degree d vs k")
    ax.legend(fontsize=8, loc="upper left")
    ax.grid(alpha=0.3)
    fig.tight_layout()
    save(fig, "pq2_low_vs_k")
    # d_lin summary
    fig, ax = plt.subplots(figsize=(8, 3.4))
    order = {"2": 2, "3": 3, "4": 4, ">3": 3.5, ">4": 4.5, "?": float("nan")}
    xs = [r["n"] for r in rows if r["d_iter"] != "?"]
    ys = [order[r["d_iter"]] for r in rows if r["d_iter"] != "?"]
    ax.scatter(xs, ys, s=22, label="d_iter: lower bound on F4's max step degree")
    for sub in ("iter4_n17_20", "iter4_n21_24", "iter4_n25", "iter4_n26", "iter4_n27"):
        p = os.path.join(HERE, "pq2_rank", "results", sub, "pq2_rank.json")
        if os.path.exists(p):
            ext = json.load(open(p))["rows"]
            ax.scatter([r["n"] for r in ext], [order[r["d_iter"]] - 0.08 for r in ext], s=22, marker="v",
                       color="C3", label="d_iter, degree-4 extension sweep (fresh instances)" if sub.endswith("20") else None)
    ms = os.path.join(HERE, "pq2_rank", "f4", "msolve_all.json")
    if os.path.exists(ms):
        mr = json.load(open(ms))["rows"]
        ax.scatter([r["n"] + 0.25 for r in mr], [r["max_step_degree"] for r in mr], s=26, marker="*", color="C2",
                   label="msolve F4: actual max step degree (same instances, n <= 34)")
    ax.scatter([r["n"] for r in rows], [order[r["d_lin"]] + 0.08 for r in rows], s=14, marker="x",
               color="gray", label="d_lin: lower bound on the one-shot solving degree")
    ax.set_yticks([2, 3, 3.5, 4, 4.5])
    ax.set_yticklabels(["2", "3", ">3 (deg 4 not run)", "4", ">4"])
    ax.set_xlabel("n")
    ax.set_ylabel("degree bound")
    ax.set_title("m = 2 descended S3: degree per instance (first fall degree = 2)")
    ax.legend(fontsize=8, loc="lower right")
    ax.grid(alpha=0.3)
    fig.tight_layout()
    save(fig, "pq2_dlin")


def pf_yield():
    rows = load("pf_yield")["rows"]
    fig, axes = plt.subplots(1, 2, figsize=(10, 4))
    for base, marker in (("small_x", "o"), ("random", "s"), ("random_negclosed", "^")):
        rs = [r for r in rows if r["base"] == base]
        u = [r["unordered_U"] for r in rs]
        axes[0].scatter([r["p"] for r in rs], [x["variance"] / x["mean"] for x in u], s=16, marker=marker,
                        label=base, alpha=0.8)
        axes[1].scatter([r["p"] for r in rs],
                        [x["decomposable_fraction"] / (1 - math.exp(-x["mean"])) for x in u], s=16,
                        marker=marker, label=base, alpha=0.8)
    for ax in axes:
        ax.set_xscale("log")
        ax.set_xlabel("p")
        ax.grid(alpha=0.3)
    axes[0].axhline(1, color="k", lw=0.8)
    axes[0].set_ylabel("var(U) / mean(U)")
    axes[0].set_title("unordered pair counts U(R): dispersion")
    axes[1].axhline(1, color="k", lw=0.8)
    axes[1].set_ylabel("P(U >= 1) / (1 - exp(-mean U))")
    axes[1].set_title("decomposable fraction vs Poisson prediction")
    axes[1].legend(fontsize=8)
    fig.tight_layout()
    save(fig, "pf_yield")


def rank_deficit():
    rows = load("rank_deficit")
    fig, ax = plt.subplots(figsize=(8, 4))
    deltas = sorted({r["delta"] for r in rows if r["shape"] == "A"})
    for p in sorted({r["p"] for r in rows}):
        ys = []
        for d in deltas:
            vals = [r["merged_deficit"] for r in rows if r["shape"] == "A" and r["p"] == p and r["delta"] == d]
            ys.append(max(vals) if vals else float("nan"))
        ax.plot(deltas, ys, marker="o", ms=4, label=f"p = {p}")
    ax.set_xlabel("surplus relations Delta")
    ax.set_ylabel("max rank deficit over samples (negation pairs merged)")
    ax.set_title("right-kernel relation matrices: deficit vs surplus (PKM: Delta = 10)")
    ax.axvline(10, color="k", lw=0.8, ls="--")
    ax.legend(fontsize=8)
    ax.grid(alpha=0.3)
    fig.tight_layout()
    save(fig, "rank_deficit")


def trace_pullback():
    curves = load("trace_pullback")["curves"]
    fig, ax = plt.subplots(figsize=(8, 4))
    for c in curves:
        vals = sorted(v["T_over_n"] for v in c["V"])
        ax.plot(range(len(vals)), vals, lw=1, label=f"p = {c['p']}, #E = {c['n_Fp']}")
    ax.set_xlabel("one-dimensional subspaces V of F_{p^2}, sorted by |Tr(F'_V)|")
    ax.set_ylabel("|Tr(F'_V)| / #E(F_p)")
    ax.set_title("trace image of subspace factor bases: never below 0.32 of the group")
    ax.legend(fontsize=7, ncol=2)
    ax.grid(alpha=0.3)
    fig.tight_layout()
    save(fig, "trace_pullback")


def ext_decomp():
    rows = load("ext_decomp")["rows"]
    fig, ax = plt.subplots(figsize=(8, 4))
    for flag, marker, label in ((False, "o", "a, b in F_{q^n} (heuristic's setting)"),
                                (True, "x", "a, b in F_q (control: curve over the subfield)")):
        rs = [r for r in rows if r["curve_over_subfield"] == flag]
        ax.scatter([r["n"] + (0.1 if flag else -0.1) for r in rs], [r["ratio"] for r in rs], marker=marker,
                   label=label, alpha=0.8)
    ax.set_yscale("log")
    ax.axhline(1, color="k", lw=0.8)
    ax.set_xticks([2, 3, 4])
    ax.set_xlabel("n (E over F_{q^n}, factor base x in F_q)")
    ax.set_ylabel("decomposable fraction / (1/n!)")
    ax.set_title("n-fold decomposition over F_{q^n}: measured fraction against 1/n!")
    ax.legend(fontsize=8)
    ax.grid(alpha=0.3, which="both")
    fig.tight_layout()
    save(fig, "ext_decomp")


if __name__ == "__main__":
    which = sys.argv[1:] or ["pq2", "pf_yield", "rank_deficit", "trace_pullback", "ext_decomp"]
    for w in which:
        globals()[w]()
        print("plotted", w)

#!/usr/bin/env python3
"""Draw the three protocol.json reporting graphs from table.tsv and ANALYSIS.json only.

Usage: python3 make_figures.py   (stdlib + matplotlib; run from anywhere)
Plots recorded values only; nothing is recomputed. Every seconds figure is
diagnostic (launch/README.md deviation 1: no L2 admission for command Jobs).
Outputs (SVG, and PNG at 2x of the 100 dpi base = 200 dpi) beside this script.
"""
import csv
import json
import os

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.lines import Line2D
from matplotlib.patches import Patch

HERE = os.path.dirname(os.path.abspath(__file__))
STUDY = os.path.dirname(HERE)
RUN = "arity-koblitz-m56-20261104-a5 (attempt 5; attempt 4 prefix resumed)"
PARITY = None
ENGINES = ["chain-s3-orbit", "chain-s3"]
ECOL = {"chain-s3-orbit": "#1b6ca8", "chain-s3": "#c2571a"}
CLASS_COL = {
    "decided_sat": "#2a9d4a", "decided_unsat": "#8ccf6a", "undecided": "#f2c14e",
    "capped": "#d1495b", "skipped_stopping_rule": "#c8c8c8",
}
SUBTITLE = ("Run %s. All seconds figures are diagnostic: no L2 admission for command Jobs "
            "(launch/README.md deviation 1). Source: table.tsv, ANALYSIS.json." % RUN)


def load():
    with open(os.path.join(STUDY, "table.tsv"), newline="") as f:
        rows = list(csv.DictReader(f, delimiter="\t"))
    with open(os.path.join(STUDY, "ANALYSIS.json")) as f:
        ana = json.load(f)
    # cross-check: table.tsv and ANALYSIS.json describe the same cells and classes
    key = lambda r: (r["family"], r["curve"], int(r["m"]), int(r["S"]), r["engine"], r["probes"])
    a = {key(c): c["class"] for c in ana["cells"]}
    t = {key(r): r["class"] for r in rows}
    assert a == t, "table.tsv and ANALYSIS.json disagree"
    return rows, ana


def num(s):
    return None if s in ("-", "") else float(s)


def save(fig, name):
    fig.savefig(os.path.join(HERE, name + ".svg"))
    fig.savefig(os.path.join(HERE, name + ".png"), dpi=200)  # 2x of the 100 dpi base
    plt.close(fig)


def fig1(rows, parity):
    sel = [r for r in rows if r["probes"] == "public" and r["family"] in ("koblitzA", "liftB")]
    fig, axes = plt.subplots(2, 2, figsize=(11, 8), sharex=True, sharey=True)
    ns = [23, 29, 31, 37, 41, 83]
    pos = {n: i for i, n in enumerate(ns)}
    for ax, (m, S) in zip(axes.flat, [(5, 2), (5, 3), (6, 2), (6, 3)]):
        for eng in ENGINES:
            pts = sorted((r for r in sel if int(r["m"]) == m and int(r["S"]) == S and r["engine"] == eng),
                         key=lambda r: int(r["n"]))
            dx = -0.06 if eng == "chain-s3-orbit" else 0.06
            dec = [(pos[int(r["n"])] + dx, num(r["sec/query"])) for r in pts if num(r["sec/query"]) is not None]
            if dec:
                ax.plot(*zip(*dec), "o", color=ECOL[eng], ms=7)
            cap = [(pos[int(r["n"])] + dx, num(r["cap_s"])) for r in pts if r["class"] == "capped"]
            if cap:
                ax.plot(*zip(*cap), "^", color=ECOL[eng], mfc="none", ms=9, mew=1.6)
            und = [(pos[int(r["n"])] + dx, 0.0035) for r in pts if r["class"] == "undecided"]
            if und:
                ax.plot(*zip(*und), "x", color=ECOL[eng], ms=8, mew=1.8)
        ax.axhline(parity, color="k", ls="--", lw=1)
        ax.text(-0.4, parity * 1.15, "%g s parity (m = 4 root table)" % parity, fontsize=8)
        ax.set_yscale("log")
        ax.set_ylim(0.002, 6000)
        ax.set_xticks(range(len(ns)))
        ax.set_xticklabels([str(n) for n in ns])
        ax.set_xlim(-0.5, len(ns) - 0.5)
        ax.set_title("m = %d, union:%d" % (m, S))
        ax.grid(alpha=0.25, which="both")
    for ax in axes[1]:
        ax.set_xlabel("n (field degree)")
    for ax in axes[:, 0]:
        ax.set_ylabel("seconds per decided query (diagnostic)")
    handles = [Line2D([], [], marker="o", ls="", color=ECOL[e], label=e + ": decided, s/query") for e in ENGINES]
    handles += [Line2D([], [], marker="^", ls="", mfc="none", mew=1.6, color="gray",
                       label="capped: censored at the process cap (600 s Part A, 1800 s Part B)"),
                Line2D([], [], marker="x", ls="", color="gray", mew=1.8,
                       label="undecided (conflict budget; no seconds figure), drawn at the bottom"),
                Line2D([], [], color="k", ls="--", label="188 s parity line")]
    fig.legend(handles=handles, loc="lower center", bbox_to_anchor=(0.5, 0.02), ncol=2, fontsize=8, frameon=False)
    fig.suptitle("Seconds per decided query against n, public cells (Part A n = 23..41, Part B n = 83)", fontsize=12)
    fig.text(0.5, 0.004, SUBTITLE, ha="center", fontsize=6.5)
    fig.text(0.5, 0.945, "A capped cell is a censored observation, not a cost; a Part A process cap bounds four queries together "
             "(a decided Part A cell is at most 150 s per query).", ha="center", fontsize=7.5)
    fig.tight_layout(rect=(0, 0.13, 1, 0.94))
    save(fig, "fig1_seconds_vs_n")


def fig2(rows):
    classes = ["decided_sat", "decided_unsat", "undecided", "capped", "skipped_stopping_rule"]
    fig, (axa, axb) = plt.subplots(1, 2, figsize=(12, 5.6), gridspec_kw={"width_ratios": [5, 1.3]})
    cfg = [(m, S, e) for e in ENGINES for (m, S) in [(5, 2), (5, 3), (6, 2), (6, 3)]]
    labels = ["%s  m=%d  S=%d" % (e, m, S) for (m, S, e) in cfg]
    for ax, fam, ns, title in [(axa, "koblitzA", [23, 29, 31, 37, 41], "Part A (600 s process cap, 4 queries)"),
                               (axb, "liftB", [83], "Part B (1800 s, 1 query)")]:
        for i, (m, S, e) in enumerate(cfg):
            for j, n in enumerate(ns):
                r = [x for x in rows if x["family"] == fam and x["probes"] == "public" and int(x["n"]) == n
                     and int(x["m"]) == m and int(x["S"]) == S and x["engine"] == e]
                assert len(r) == 1
                r = r[0]
                ax.add_patch(plt.Rectangle((j - 0.5, i - 0.5), 1, 1, fc=CLASS_COL[r["class"]], ec="white", lw=2))
                s = num(r["sec/query"])
                if s is not None:
                    ax.text(j, i, "%.2g s" % s, ha="center", va="center", fontsize=8)
        ax.set_xlim(-0.5, len(ns) - 0.5)
        ax.set_ylim(len(cfg) - 0.5, -0.5)
        ax.set_xticks(range(len(ns)))
        ax.set_xticklabels(["n=%d" % n for n in ns])
        ax.set_title(title, fontsize=10)
        if ax is axa:
            ax.set_yticks(range(len(cfg)))
            ax.set_yticklabels(labels, fontsize=8)
        else:
            ax.set_yticks([])
    fig.legend(handles=[Patch(fc=CLASS_COL[c], label=c) for c in classes], loc="lower center", ncol=5, fontsize=8, frameon=False)
    fig.suptitle("Outcome grid: public cells by (engine, m, S) and n, coloured by cell class", fontsize=12)
    fig.text(0.5, 0.004, SUBTITLE, ha="center", fontsize=6.5)
    fig.tight_layout(rect=(0, 0.07, 1, 0.95))
    save(fig, "fig2_outcome_grid")


def fig3(rows):
    fig, axes = plt.subplots(1, 3, figsize=(15, 5.4), gridspec_kw={"width_ratios": [1.1, 1.2, 1.1]})
    # (a) negative control m = 2
    ax = axes[0]
    neg = [r for r in rows if r["family"] == "negative"]
    curves = ["k0n23", "k1n29", "k0n31", "k0n37", "k0n41", "n83k1"]
    for k, e in enumerate(ENGINES):
        for j, c in enumerate(curves):
            r = [x for x in neg if x["curve"] == c and x["engine"] == e][0]
            x = j + (-0.2 if k == 0 else 0.2)
            s = num(r["sec/query"])
            if s is not None:
                ax.bar(x, max(s, 1e-4), width=0.38, color=ECOL[e])
                if s < 1e-3:
                    ax.text(x, 1.5e-4, "%.3f" % s, ha="center", fontsize=7, rotation=90, color="k")
            else:
                ax.plot([x], [1e-3], "x", color=ECOL[e], ms=9, mew=2)
    ax.set_yscale("log")
    ax.set_ylim(1e-4, 100)
    ax.set_xticks(range(len(curves)))
    ax.set_xticklabels(curves, fontsize=8)
    ax.set_ylabel("seconds per decided query (diagnostic)")
    ax.set_title("(a) Negative control, m = 2 union:2\nbar = decided_unsat; x = undecided", fontsize=9)
    ax.legend(handles=[Patch(fc=ECOL[e], label=e) for e in ENGINES], fontsize=8)
    ax.grid(alpha=0.25, which="both")
    # (b) A/A: public cell vs its A/A repeat, decided pairs, and class agreement
    ax = axes[1]
    main = {(r["curve"], r["m"], r["S"], r["engine"]): r for r in rows if r["family"] == "koblitzA" and r["probes"] == "public"}
    aa = [r for r in rows if r["family"] == "aa"]
    cnt = {"decided pairs": 0}
    xs, ys, lab = [], [], []
    agree = disagree = 0
    for r in aa:
        m = main[(r["curve"], r["m"], r["S"], r["engine"])]
        if r["class"] == m["class"]:
            agree += 1
        else:
            disagree += 1
        a, b = num(m["sec/query"]), num(r["sec/query"])
        if a is not None and b is not None:
            xs.append(a); ys.append(b); lab.append(r["engine"])
    ax.plot([0.005, 1], [0.005, 1], "k--", lw=1, label="y = x")
    ax.plot(xs, ys, "o", color=ECOL["chain-s3"], ms=8, label="decided pairs (n = %d)" % len(xs))
    ax.set_xscale("log"); ax.set_yscale("log")
    ax.set_xlabel("measurement cell, s/query"); ax.set_ylabel("A/A repeat cell, s/query")
    ax.set_title("(b) A/A: repeat vs measurement, public cells\nclass agreement %d of %d cell pairs (A/A skipped pairs included)"
                 % (agree, agree + disagree), fontsize=9)
    ax.legend(fontsize=8)
    ax.grid(alpha=0.25, which="both")
    # (c) positive control: planted cells
    ax = axes[2]
    pl = [r for r in rows if r["probes"] == "planted"]
    cats = ["verified 4/4\nor 1/1", "completed, not\nall verified", "capped", "skipped"]
    for k, e in enumerate(ENGINES):
        v = [0, 0, 0, 0]
        for r in pl:
            if r["engine"] != e:
                continue
            if r["class"] == "skipped_stopping_rule":
                v[3] += 1
            elif r["class"] == "capped":
                v[2] += 1
            elif int(r["verified"]) == int(r["trials"]) and r["class"] == "decided_sat":
                v[0] += 1
            else:
                v[1] += 1
        ax.bar([i + (-0.2 if k == 0 else 0.2) for i in range(4)], v, width=0.38, color=ECOL[e], label=e)
        for i, h in enumerate(v):
            ax.text(i + (-0.2 if k == 0 else 0.2), h + 0.3, str(h), ha="center", fontsize=8)
    ax.set_xticks(range(4)); ax.set_xticklabels(cats, fontsize=8)
    ax.set_ylabel("planted cells (count, Part A + Part B)")
    ax.set_title("(c) Positive control: planted cells by outcome", fontsize=9)
    ax.legend(fontsize=8)
    fig.suptitle("Controls: negative (m = 2), A/A and positive (planted)", fontsize=12)
    fig.text(0.5, 0.004, SUBTITLE, ha="center", fontsize=6.5)
    fig.tight_layout(rect=(0, 0.03, 1, 0.94))
    save(fig, "fig3_controls")


def main():
    rows, ana = load()
    parity = float(ana["parity_seconds"])
    fig1(rows, parity)
    fig2(rows)
    fig3(rows)


if __name__ == "__main__":
    main()

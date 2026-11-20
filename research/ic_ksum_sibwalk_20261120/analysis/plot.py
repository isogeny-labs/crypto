#!/usr/bin/env python3
"""summary.json + pilot/pilot_summary.json + pilot/model_pilot.json -> ratios.svg, gap.svg, model.svg (plots recorded values only). cwd = study dir."""
import json
import matplotlib
matplotlib.use("Agg")
matplotlib.rcParams["svg.fonttype"] = "none"
matplotlib.rcParams["svg.hashsalt"] = "ic_ksum_sibwalk_20261120"
import matplotlib.pyplot as plt
S = json.load(open("summary.json")); rows = list(S["rows"]); W = ["0", "0.1", "1"]
COL = {"0": "#2f6db3", "0.1": "#d07a1f", "1": "#4a9a5b"}


def ratios(fig):
    ax = fig.add_subplot(111)
    for i, w in enumerate(W):
        xs = [j + (i - 1) * 0.24 for j in range(len(rows))]
        ys = [S["rows"][r]["weights"][w]["ratio"] for r in rows]
        lo = [y - S["rows"][r]["weights"][w]["ci95"][0] for r, y in zip(rows, ys)]
        hi = [S["rows"][r]["weights"][w]["upper_used"] - y for r, y in zip(rows, ys)]
        ax.errorbar(xs, ys, yerr=[lo, hi], fmt="o", ms=5, color=COL[w], label=f"measured, w = {w}", capsize=0)
        ax.scatter(xs, [S["rows"][r]["weights"][w]["predicted"] for r in rows], marker="_", s=120, color="#222", zorder=3,
                   label="registered prediction" if i == 0 else None)
    ax.axhline(1, color="#888", lw=0.8)
    ax.set_xticks(range(len(rows))); ax.set_xticklabels([f"{r}\nm={S['rows'][r]['m']}" for r in rows])
    ax.set_ylabel("cost(cand) / cost(inc), pooled; bar to the registered upper end")
    ax.set_title("walk_sib=m against the incumbent best, Table B, 432 pairs a row [pcc:unverified]", fontsize=10)
    ax.legend(fontsize=8, loc="upper left")


def gap(fig):
    ax = fig.add_subplot(111)
    for i, w in enumerate(W):
        xs = [j + (i - 1) * 0.26 for j in range(len(rows))]
        ax.bar(xs, [S["rows"][r]["weights"][w]["gap"]["inc"] for r in rows], width=0.25, color=["#d4d4d4", "#b0b0b0", "#8c8c8c"][i],
               label=f"incumbent best, w = {w}")
        ax.scatter(xs, [S["rows"][r]["weights"][w]["gap"]["cand"] for r in rows], color=COL[w], s=22, zorder=3, label=f"walk_sib, w = {w}")
    ax.axhline(1, color="#333", lw=0.8); ax.set_xticks(range(len(rows))); ax.set_xticklabels(rows)
    ax.set_ylabel("S(arm) / S(rho-neg)  (distance remaining)")
    ax.set_title("Distance remaining to matched rho-neg, this study's targets [pcc:unverified]", fontsize=10); ax.legend(fontsize=7, ncol=2)


def model(fig):
    P = json.load(open("pilot/pilot_summary.json")); Mo = json.load(open("pilot/model_pilot.json"))
    ax = fig.add_subplot(111); xs, ys, cs = [], [], []
    for r in rows:
        for m in ("4", "8", "16"):
            for w in W:
                xs.append(Mo[r]["pred"][m][str(float(w))]); ys.append(P[r]["sib" + m][w]["ratio"]); cs.append(COL[w])
    for r in rows:
        for w in W:
            xs.append(S["rows"][r]["weights"][w]["predicted"]); ys.append(S["rows"][r]["weights"][w]["ratio"]); cs.append("#b03030")
    ax.scatter(xs, ys, c=cs, s=14)
    ax.plot([0.65, 1.15], [0.65, 1.15], color="#888", lw=0.8)
    ax.set_xlabel("count-model prediction (from incumbent records only)"); ax.set_ylabel("measured cost(walk_sib) / cost(inc)")
    ax.set_title("Count model against measurement: pilot m = 4/8/16 (blue/orange/green by w), confirmatory (red)", fontsize=9)


for name, fn, size in [("ratios.svg", ratios, (10, 4.6)), ("gap.svg", gap, (9, 4.4)), ("model.svg", model, (6, 5.5))]:
    fig = plt.figure(figsize=size); fn(fig); fig.tight_layout(); fig.savefig(name, metadata={"Date": None}); plt.close(fig)

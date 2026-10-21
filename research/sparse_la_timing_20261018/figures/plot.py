#!/usr/bin/env python3
"""Plot recorded values from analysis/*-summary.json (layout only).

Writes figures/{scaling_row_ops,scaling_end_to_end,fp4_phases}.{svg,png}.
The least-squares slopes printed in the legend of the row_ops panel are a
descriptive fit of six recorded medians, drawn on the graph; they are not a
result of this study.
"""
import json
import math
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
STYLE = {
    "sg": ("structured-gauss (incumbent)", "#1f4e79", "o", "-"),
    "ig": ("incremental-gauss (baseline cfg)", "#5b9bd5", "s", "--"),
    "igfr": ("incremental-gauss-full-rank (pos. ctrl)", "#7f7f7f", "D", ":"),
    "sparse-cado": ("sparse-cado = filter+wiedemann", "#c55a11", "^", "-"),
    "filter-lanczos": ("filter+lanczos", "#ed7d31", "v", "--"),
    "wiedemann": ("wiedemann (no filter)", "#70ad47", "P", "-"),
    "lanczos": ("lanczos (no filter)", "#a9d18e", "X", "--"),
}
SESSION_ID = "ECBS1h0bf2b60b4cc4"


def save(fig, name):
    for ext, kw in (("svg", {}), ("png", {"dpi": 150})):
        fig.savefig(HERE / f"{name}.{ext}", bbox_inches="tight", **kw)
    plt.close(fig)


def slope(xs, ys):
    lx = [math.log(x) for x in xs]
    ly = [math.log(y) for y in ys]
    mx, my = sum(lx) / len(lx), sum(ly) / len(ly)
    return sum((a - mx) * (b - my) for a, b in zip(lx, ly)) / sum((a - mx) ** 2 for a in lx)


def scaling():
    s = json.load(open(ROOT / "analysis/prime-scaling-summary.json"))
    rows = s["rows"]
    fig, ax = plt.subplots(1, 2, figsize=(12, 4.6))
    for arm, (lab, col, mk, ls) in STYLE.items():
        pts = sorted((r["columns_param"], r) for r in rows if r["linalg_arm"] == arm)
        xs = [p[0] for p in pts]
        ys = [p[1]["median_la_row_ops"] for p in pts]
        b = slope(xs, ys)
        ax[0].plot(xs, ys, marker=mk, ls=ls, color=col, label=f"{lab}  (fit slope {b:.2f})")
        ax[1].plot(xs, [p[1]["s_band"]["0.0"] for p in pts], marker=mk, ls=ls, color=col, label=lab)
    for a in ax:
        a.set_xscale("log", base=2)
        a.set_yscale("log")
        a.set_xlabel("factor-base size (prime-abscissa:size)")
        a.grid(True, which="both", alpha=0.25)
    ax[0].set_ylabel("linear-algebra row_ops (median of 6 targets)")
    ax[0].set_title("LA stage alone: counted row_ops vs matrix size")
    ax[1].axhline(s["rho_s_band"][0], color="black", lw=1)
    ax[1].text(128, s["rho_s_band"][0] * 1.15, "rho-neg (matched reference)", fontsize=8)
    ax[1].set_ylabel("end-to-end S (mean of 6 targets, row_ops at 0 GAE)")
    ax[1].set_title("End to end: S vs matrix size")
    ax[0].legend(fontsize=7, loc="upper left")
    fig.suptitle(f"icv1-fp30-t31711-b403411a, session {SESSION_ID}, 6 targets per point, all 330 runs verified", fontsize=9)
    save(fig, "scaling")

    # Band: S at w = 0, 0.1, 1 for each arm at each size.
    fig, ax = plt.subplots(1, 3, figsize=(14, 4.2), sharey=True)
    for i, w in enumerate(["0.0", "0.1", "1.0"]):
        for arm, (lab, col, mk, ls) in STYLE.items():
            pts = sorted((r["columns_param"], r["s_band"][w]) for r in rows if r["linalg_arm"] == arm)
            ax[i].plot([p[0] for p in pts], [p[1] for p in pts], marker=mk, ls=ls, color=col, label=lab)
        ax[i].axhline(s["rho_s_band"][i], color="black", lw=1)
        ax[i].set_xscale("log", base=2)
        ax[i].set_yscale("log")
        ax[i].set_title(f"native work priced at {w} GAE/op")
        ax[i].set_xlabel("factor-base size")
        ax[i].grid(True, which="both", alpha=0.25)
    ax[0].set_ylabel("end-to-end S (mean of 6 targets)")
    ax[0].legend(fontsize=7)
    fig.suptitle(f"AGENTS.md §5a band, session {SESSION_ID}; black line = rho-neg at the same weight", fontsize=9)
    save(fig, "band")


def fp4():
    p = ROOT / "analysis/fp4-p29-summary.json"
    if not p.exists():
        return
    s = json.load(open(p))
    arms = [r for r in s["rows"]]
    fig, ax = plt.subplots(1, 2, figsize=(12, 4.4))
    names = [r["linalg_arm"] for r in arms]
    ax[0].bar(names, [r["s_band"]["0.0"] for r in arms], color=[STYLE.get(n, ("", "#bbbbbb"))[1] for n in names])
    ax[0].set_ylabel("end-to-end S (mean of 2 targets)")
    ax[0].set_title("F(p^4) k=4 ladder, p=29: end-to-end S")
    ax[1].bar(names, [max(r["median_la_row_ops"], 1) for r in arms], color=[STYLE.get(n, ("", "#bbbbbb"))[1] for n in names])
    ax[1].set_yscale("log")
    ax[1].set_ylabel("linear-algebra row_ops (median of 2 targets)")
    ax[1].set_title("F(p^4): LA stage alone")
    for a in ax:
        a.tick_params(axis="x", rotation=60, labelsize=8)
    fig.suptitle(f"icv1-fp5k4-tm926-7bc5b359, session {s.get('session_id', '')}, every run verified", fontsize=9)
    save(fig, "fp4")


if __name__ == "__main__":
    scaling()
    fp4()

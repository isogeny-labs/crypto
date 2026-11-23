"""Graphs for the S3 recorded run (record 20261115T0426Z-hec-s3-inversion-hygiene).

Layout only: every plotted number is a field of ../../s3-20261114{,-rerun}/summary.jsonl
(summarise_s3.py) or of the raw stream lines ../../s3-20261114{,-rerun}/stream*-*.txt.
Writes <name>.svg and a 150 dpi <name>.png.

    python3 plots.py
"""
import glob
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = Path(__file__).resolve().parent
SETS = [("first set", HERE / "../../s3-20261114"), ("rerun", HERE / "../../s3-20261114-rerun")]
DATA = SETS[0][1]
ROWS = [json.loads(l) for l in (DATA / "summary.jsonl").read_text().splitlines() if l.strip()]
LABELS = [f"g{r['genus']}\np={r['p']}" for r in ROWS]
X = list(range(len(ROWS)))
RUN = ("record 20261115T0426Z-hec-s3-inversion-hygiene; t2_stream, 100,000 trials, 7 x A-B-A' per set; "
       "arm64 macOS, 1 thread, shared host: load average 52-401 (first set), 50-153 (rerun)")
C_OLD, C_NEW, C_AA = "#2f6fb0", "#3a9a5b", "#999"


def stream(rep, arm, data=DATA):
    path = data / f"stream{rep}-{arm}.txt"
    return [json.loads(l[5:]) for l in path.read_text().splitlines() if l.startswith("JSON ")]


REPS = sorted({Path(p).name.split("-")[0][len("stream"):] for p in glob.glob(str(DATA / "stream*-*.txt"))},
              key=int)


def save(fig, name):
    fig.tight_layout()
    fig.savefig(HERE / f"{name}.svg")
    fig.savefig(HERE / f"{name}.png", dpi=150)
    plt.close(fig)


def ratios():
    fig, axs = plt.subplots(2, 1, figsize=(10, 8), sharey=True)
    for ax, (name, data) in zip(axs, SETS):
        rows = [json.loads(l) for l in (data / "summary.jsonl").read_text().splitlines() if l.strip()]
        for rep in REPS:
            a, b, a2 = stream(rep, "A", data), stream(rep, "B", data), stream(rep, "A2", data)
            for x in X:
                r = (a[x]["per_trial_ns"] + a2[x]["per_trial_ns"]) / 2 / b[x]["per_trial_ns"]
                aa = a[x]["per_trial_ns"] / a2[x]["per_trial_ns"]
                ax.scatter([x - 0.12], [r], s=10, color=C_NEW, alpha=0.6, zorder=2)
                ax.scatter([x + 0.12], [aa], s=10, color=C_AA, alpha=0.6, zorder=2)
        for x, r in zip(X, rows):
            ax.plot([x - 0.32, x + 0.08], [r["ratio_med"]] * 2, color=C_NEW, lw=2.5, zorder=3)
            ax.plot([x - 0.08, x + 0.32], [r["aa_med"]] * 2, color="#555", lw=1.5, zorder=3)
        ax.axhspan(0.95, 1.05, color="#ddd", alpha=0.6, zorder=1)
        ax.axhline(1.25, color="#b00", lw=1, ls="--")
        ax.axhline(1.10, color="#d80", lw=1, ls=":")
        ax.set_yscale("log")
        ax.set_xticks(X, LABELS, fontsize=8)
        ax.set_ylabel("per-trial time ratio (log)")
        ax.set_title(f"{name}: old/new (green) and A/A old/old′ (grey); bars = medians; "
                     "band = A/A validity [0.95, 1.05]", fontsize=9)
    axs[0].text(5.6, 1.3, "E10 bar 1.25×", color="#b00", fontsize=7)
    axs[0].text(-0.4, 1.13, "E11 bar 1.10×", color="#d80", fontsize=7)
    axs[1].text(0, -0.2, RUN, transform=axs[1].transAxes, fontsize=7, color="#555")
    save(fig, "ratios")


def inversions():
    fig, axs = plt.subplots(1, 2, figsize=(12, 4.6), sharey=False)
    short = [f"g{r['genus']}\n{r['p']}" for r in ROWS]
    w = 0.38
    for ax, key, title in ((axs[0], "walk_inv", "per walk step"), (axs[1], "oracle_inv", "per oracle call")):
        ax.bar([x - w / 2 for x in X], [r[f"old_{key}"] for r in ROWS], w, color=C_OLD, label="old (S2b engine)")
        ax.bar([x + w / 2 for x in X], [r[f"new_{key}"] for r in ROWS], w, color=C_NEW, label="new (S3)")
        for x, r in zip(X, ROWS):
            ax.text(x + w / 2, r[f"new_{key}"] + 0.15, f"{r[f'new_{key}']:.1f}", ha="center", fontsize=6)
        ax.set_xticks(X, short, fontsize=7)
        ax.set_title(f"Fp::inv calls {title}", fontsize=10)
    axs[1].axhline(5, color="#b00", lw=1, ls="--")
    axs[1].text(-0.4, 5.3, "E9 bar ≤ 5 (genus 3, 4)", color="#b00", fontsize=7)
    axs[0].legend(fontsize=8)
    axs[0].set_ylabel("count (count-inv builds, deterministic)")
    fig.tight_layout(rect=(0, 0.05, 1, 1))
    fig.text(0.01, 0.01, RUN, fontsize=7, color="#555")
    fig.savefig(HERE / "inversions.svg")
    fig.savefig(HERE / "inversions.png", dpi=150)
    plt.close(fig)


ratios()
inversions()

"""Lay out what the atlas records (data/atlas_tally.json, read from the
lead files) and the `--match` hits of the candidate mechanisms
(data/candidate_matches.json).  Plotting only; nothing is computed here."""

import json
import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = Path(__file__).resolve().parent
OUT = HERE / (sys.argv[1] if len(sys.argv) > 1 else "figures")
OUT.mkdir(parents=True, exist_ok=True)

DISP = [
    ("closed_obstructed", "#7a1f1f"),
    ("closed_known", "#b5463a"),
    ("closed_dominated", "#d98b7a"),
    ("exhausted_at_scale", "#8c8c8c"),
    ("open_blocked", "#c9a227"),
    ("open_next_step", "#2f6db3"),
    ("open_unexamined", "#9ecae1"),
]
FAMILIES = ["ic-prime", "ic-binary", "ic-engine", "generic", "transfer", "structure", "exotic"]


def figure_tally():
    tally = json.loads((HERE / "data" / "atlas_tally.json").read_text())["tally"]
    fig, ax = plt.subplots(figsize=(8.6, 3.9))
    left = [0] * len(FAMILIES)
    for disp, colour in DISP:
        vals = [tally.get(f, {}).get(disp, 0) for f in FAMILIES]
        ax.barh(FAMILIES, vals, left=left, color=colour, label=disp.replace("_", " "))
        for i, (l, v) in enumerate(zip(left, vals)):
            if v:
                ax.text(l + v / 2, i, str(v), ha="center", va="center", fontsize=7,
                        color="white" if disp.startswith("closed") else "black")
        left = [l + v for l, v in zip(left, vals)]
    ax.invert_yaxis()
    ax.set_xlabel("leads (154 in the atlas, reviewed 2026-10-30)", fontsize=8)
    ax.tick_params(labelsize=8)
    ax.legend(fontsize=7, ncol=4, loc="upper center", bbox_to_anchor=(0.5, -0.16), frameon=False)
    ax.set_title("ECDLP lead atlas by family and disposition (crypto-autoresearcher main cfc1dae2, read 2026-11-01)",
                 fontsize=8.5)
    fig.tight_layout()
    for ext, kw in (("svg", {}), ("png", {"dpi": 150})):
        fig.savefig(OUT / f"atlas_tally.{ext}", bbox_inches="tight", **kw)


def figure_matches():
    rows = json.loads((HERE / "data" / "candidate_matches.json").read_text())
    colour = dict(DISP)
    fig, ax = plt.subplots(figsize=(10.5, 6.2))
    labels, xs, cols, texts = [], [], [], []
    for i, r in enumerate(rows):
        top = r["hits"][0]
        labels.append(r["candidate"])
        xs.append(top["score"])
        cols.append(colour[top["disposition"]])
        texts.append(f'{top["lead"][5:]}  ({top["disposition"].replace("_", " ")})')
    y = range(len(rows))
    ax.barh(list(y), xs, color=cols, alpha=0.9)
    for i, t in enumerate(texts):
        ax.text(xs[i] + 0.3, i, t, va="center", fontsize=6.8)
    ax.set_yticks(list(y))
    ax.set_yticklabels(labels, fontsize=7)
    ax.invert_yaxis()
    ax.set_xlim(0, max(xs) * 2.6)
    ax.set_xlabel("`tools/lead_atlas.py --match` score of the best-matching lead (word overlap; higher = closer)", fontsize=7.5)
    ax.set_title("Sixteen candidate mechanisms for a sub-exponential prime-field ECDLP, matched against the atlas\n"
                 "(every one lands on an existing lead; bar colour = that lead's disposition)", fontsize=9)
    handles = [plt.Rectangle((0, 0), 1, 1, color=c) for d, c in DISP]
    ax.legend(handles, [d.replace("_", " ") for d, _ in DISP], fontsize=7, ncol=4, loc="upper center",
              bbox_to_anchor=(0.5, -0.10), frameon=False)
    fig.tight_layout()
    for ext, kw in (("svg", {}), ("png", {"dpi": 150})):
        fig.savefig(OUT / f"candidate_matches.{ext}", bbox_inches="tight", **kw)


figure_tally()
figure_matches()

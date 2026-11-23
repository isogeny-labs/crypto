#!/usr/bin/env python3
"""Figures for research/tm1514_recheck_20261106/REPORT.md.

Plots recorded values only (research-visuals skill): the frozen analysis
output analysis/result.json (sha256 c7c473de...d236) and, for the earlier
n = 24 markers, the per-curve band means of the 4 November multi-curve
session (research/curve25519_autolab_20261104/multicurve/band-k24.txt).
The earlier ratios are inc S / rho-neg S from that table (derived, labelled).
No statistic is recomputed here.

Usage (from the repository root):
  python3 research/tm1514_recheck_20261106/figures/make_figures.py
Writes fig-ratio-per-curve.{svg,png,pdf} and fig-d-band.{svg,png,pdf}
beside this script (PNG at 150 dpi, same plotting source as the SVG).
"""
import hashlib
import json
import pathlib
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = pathlib.Path(__file__).resolve().parent
STUDY = HERE.parent
ROOT = STUDY.parent.parent
RESULT = STUDY / "analysis" / "result.json"
RESULT_SHA = "c7c473deaf89495857ddb63b37ae3a4e3851e9a77acec167df34feb35650d236"
BAND = ROOT / "research/curve25519_autolab_20261104/multicurve/band-k24.txt"
WEIGHTS = [("0", 0.0), ("0.1", 0.1), ("1", 1.0)]

plt.rcParams.update({"svg.hashsalt": "tm1514", "font.size": 9,
                     "axes.spines.top": False, "axes.spines.right": False})
META = {"Creator": "make_figures.py", "CreationDate": None}


def short(curve):
    return curve.split("-")[2]  # icv1-fp24-tm1514-dce9313c -> tm1514


def earlier_ratios(path):
    """inc S / rho-neg S per curve and weight from band-k24.txt (derived)."""
    s = {}
    for line in path.read_text().splitlines():
        cells = [c.strip() for c in line.strip().strip("|").split("|")]
        if len(cells) == 8 and cells[2] in ("inc", "rho-neg"):
            s[(cells[0], cells[2])] = [float(x) for x in cells[4:7]]
    out = {}
    for (curve, arm), vals in s.items():
        if arm == "inc":
            rho = s[(curve, "rho-neg")]
            out[curve] = [a / b for a, b in zip(vals, rho)]
    return out


def save(fig, stem):
    for ext in ("svg", "pdf"):
        fig.savefig(HERE / f"{stem}.{ext}", bbox_inches="tight",
                    metadata=META if ext == "pdf" else {"Date": None})
    png = HERE / f"{stem}.png"
    fig.savefig(png, dpi=150, bbox_inches="tight", metadata={"Software": None})
    plt.close(fig)
    # Palette PNG keeps the 150 dpi render under the artifact store's 64 KiB
    # data-file threshold (AGENTS.md section 13); pixels are unchanged in layout.
    from PIL import Image
    Image.open(png).convert("RGB").quantize(colors=64, method=Image.Quantize.MEDIANCUT).save(
        png, optimize=True)


def main():
    raw = RESULT.read_bytes()
    got = hashlib.sha256(raw).hexdigest()
    if got != RESULT_SHA:
        sys.exit(f"result.json sha256 {got} != frozen {RESULT_SHA}")
    r = json.loads(raw)
    sid = ", ".join(f"{i['replicate']} {i['session_id']}" for i in r["inputs"])
    early = earlier_ratios(BAND) if BAND.exists() else {}

    # Figure 1: per-curve inc / rho-neg ratio at the three weights.
    curves = r["per_curve"]
    colors = {"0": "#2a6fb0", "0.1": "#d08a1e", "1": "#3a9a5b"}
    offs = {"0": -0.22, "0.1": 0.0, "1": 0.22}
    fig, ax = plt.subplots(figsize=(7.0, 3.6))
    for i, pc in enumerate(curves):
        for key, w in WEIGHTS:
            q = pc["ratio_inc_over_rho_neg"][f"w={key}"]
            lo, hi = q["interval"]
            x = i + offs[key]
            ax.errorbar(x, q["point"], yerr=[[q["point"] - lo], [hi - q["point"]]],
                        fmt="o", ms=4.5, color=colors[key], capsize=2.5, lw=1.2,
                        label=f"re-measured, w = {key} (98.33 % interval)" if i == 0 else None)
            if pc["curve"] in early:
                ev = early[pc["curve"]][[k for k, _ in WEIGHTS].index(key)]
                ax.plot(x, ev, marker="x", ms=6, color=colors[key], mew=1.4, ls="none",
                        label=f"earlier n = 24, w = {key} (derived)" if i == 0 else None)
    ax.axhline(1.0, ls="--", color="0.3", lw=1)
    ax.text(-0.55, 1.005, "parity", va="bottom", ha="left", fontsize=8, color="0.3")
    ax.set_xticks(range(len(curves)))
    ax.set_xticklabels([short(pc["curve"]) + (" (subject)" if pc["role"] == "subject" else "")
                        for pc in curves])
    ax.set_xlim(-0.6, len(curves) - 0.4)
    ax.set_ylabel("inc / rho-neg, ratio of mean S")
    ax.set_title("k = 24 Curve25519-rule curves: ledger incumbent over matched rho-neg", fontsize=9.5)
    ax.set_ylim(0.85, 2.2)
    ax.legend(fontsize=7, loc="upper left", bbox_to_anchor=(1.0, 1.0), frameon=False)
    n = int(curves[0]["inc_rho_neg_pairs"])
    fig.text(0.0, -0.06, f"{n:,} inc/rho-neg pairs per curve (96 targets x 6 rounds x 2 replicates). "
             f"Sessions {sid}. Counted operations; native work priced at w GAE/op.\n"
             "x: earlier 4 Nov session (24 solves per arm), inc S / rho-neg S from band-k24.txt. "
             "Above 1: rho-neg cheaper. Source: analysis/result.json.", fontsize=7, ha="left")
    save(fig, "fig-ratio-per-curve")

    # Figure 2: D(w) = ln R_tm1514 - ln R_siblings with the A/A half-width.
    fig, ax = plt.subplots(figsize=(5.2, 3.2))
    for j, c in enumerate(r["C1"]):
        d = c["D"]
        lo, hi = d["interval"]
        h = c["aa_half_width"]
        ax.fill_between([j - 0.3, j + 0.3], [-h, -h], [h, h], color="0.88", lw=0,
                        label="A/A half-width (+/-)" if j == 0 else None)
        ax.errorbar(j, d["point"], yerr=[[d["point"] - lo], [hi - d["point"]]], fmt="o",
                    color="#2a6fb0", capsize=3, ms=5,
                    label="D(w), 98.33 % interval" if j == 0 else None)
        ax.annotate(f"{d['point']:.4f}\n[{lo:.4f}, {hi:.4f}]", (j + 0.08, hi), fontsize=7,
                    va="bottom")
    ax.axhline(0, ls="--", color="0.3", lw=1)
    ax.set_xticks(range(len(r["C1"])))
    ax.set_xticklabels([f"w = {c['w']:g}" for c in r["C1"]])
    ax.set_xlim(-0.5, len(r["C1"]) - 0.3)
    ax.set_ylim(-0.12, 0.14)
    ax.set_ylabel("D = ln R_tm1514 - ln R_siblings")
    ax.set_title("C1: tm1514 against its four siblings", fontsize=9.5)
    ax.legend(fontsize=7, loc="lower right", frameon=False)
    fig.text(0.0, -0.08, f"Verdict at every w: {r['C1'][0]['verdict']}. Paired bootstrap over targets, "
             "B = 10,000,\nstratified by curve. Sessions " + sid + ". Source: analysis/result.json.",
             fontsize=7, ha="left")
    save(fig, "fig-d-band")


if __name__ == "__main__":
    main()

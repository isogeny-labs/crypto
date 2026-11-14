"""Lay out the prime-field index-calculus lead heat map from leads.json.

Plots recorded verdicts only; computes nothing. Run with any Python that has
matplotlib:  python3 plot.py   (writes lead_cells.svg/.png and
verdict_matrix.svg/.png next to this file, reading ../leads.json).
"""
import json
import pathlib
from collections import Counter, defaultdict

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402
from matplotlib.patches import FancyBboxPatch, Rectangle  # noqa: E402

HERE = pathlib.Path(__file__).resolve().parent
LEADS = json.loads((HERE.parent / "leads.json").read_text())
UPDATED = LEADS.get("updated", "2026-11-14")
ROWS = [r for r in LEADS["leads"] if r["verdict"] != "out-of-scope"]

VERDICT_COLOR = {
    "refuted": "#d1495b",
    "engineering": "#edae49",
    "open": "#00798c",
    "blocked": "#4f6d7a",
    "unexamined": "#b8b8b8",
}
VERDICT_LABEL = {
    "refuted": "refuted / negative (obstruction, measured or toy with wide margin)",
    "engineering": "engineering / constant factor only",
    "open": "open / inconclusive (decisive test exists, not done)",
    "blocked": "open but blocked on a resource (host, memory, records, paper)",
    "unexamined": "unexamined (nothing run or derived)",
}
VERDICT_ORDER = ["open", "blocked", "unexamined", "engineering", "refuted"]
STRENGTH_ORDER = ["proof", "measured", "toy", "desk", "none"]
STRENGTH_DOTS = {"proof": 4, "measured": 3, "toy": 2, "desk": 1, "none": 0}
COLUMNS = [
    ("FB", "Factor-base families"),
    ("ORACLE", "Decomposition / solver oracles"),
    ("LIFT", "Lifting and p-adic"),
    ("TRANSFER", "Transfers, covers, structure"),
    ("BARRIER", "Barriers, spectra, controls"),
    ("ENG", "Engineering and accounting"),
]
SOURCE_GLYPH = {"atlas": "▲", "idea-pr": "●", "crypto-study": "■"}
plt.rcParams.update({"font.family": "DejaVu Sans", "font.size": 9, "svg.fonttype": "none"})


def sort_key(r):
    return (0 if r.get("flagship") else 1, VERDICT_ORDER.index(r["verdict"]),
            STRENGTH_ORDER.index(r["evidence_strength"]), r["id"])


def lead_cells():
    bycol = defaultdict(list)
    for r in ROWS:
        bycol[r["column"]].append(r)
    for c in bycol:
        bycol[c].sort(key=sort_key)
    nrows = max(len(v) for v in bycol.values())
    ncol = len(COLUMNS)
    cell_h = 0.62
    fig_h = 2.2 + nrows * cell_h * 0.42 + 1.9
    fig, ax = plt.subplots(figsize=(3.05 * ncol, fig_h))
    ax.set_xlim(0, ncol)
    ax.set_ylim(-(nrows * cell_h) - 2.4, 1.1)
    ax.axis("off")
    for ci, (key, title) in enumerate(COLUMNS):
        items = bycol.get(key, [])
        ax.text(ci + 0.5, 0.78, f"{title}\n(n = {len(items)})", ha="center", va="center",
                fontsize=9.5, fontweight="bold")
        for ri, r in enumerate(items):
            y = -ri * cell_h
            x = ci + 0.025
            flag = bool(r.get("flagship"))
            box = FancyBboxPatch((x, y - cell_h * 0.46), 0.95, cell_h * 0.9,
                                 boxstyle="round,pad=0.01,rounding_size=0.05",
                                 fc=VERDICT_COLOR[r["verdict"]], ec="black" if flag else "white",
                                 lw=2.2 if flag else 0.5, alpha=0.92)
            ax.add_patch(box)
            dots = STRENGTH_DOTS[r["evidence_strength"]]
            strength = "●" * dots + "○" * (4 - dots)
            label = f"{SOURCE_GLYPH.get(r['source_kind'], '?')} {r['label'][:38]}"
            dark = r["verdict"] in ("refuted", "open", "blocked")
            ax.text(x + 0.03, y, label, ha="left", va="center", fontsize=6.6,
                    color="white" if dark else "black", fontweight="bold" if flag else "normal")
            ax.text(x + 0.935, y, strength, ha="right", va="center", fontsize=5.2,
                    color="white" if dark else "black")
    handles = [Rectangle((0, 0), 1, 1, fc=VERDICT_COLOR[k]) for k in VERDICT_ORDER]
    ax.legend(handles, [VERDICT_LABEL[k] for k in VERDICT_ORDER], loc="lower center",
              bbox_to_anchor=(0.5, 0.0), fontsize=8, frameon=False, ncol=2,
              title="verdict (black border = live exponent-level question; dots = evidence strength: "
                    "●●●● proof, ●●● measured, ●● toy, ● desk; "
                    "▲ atlas lead, ● idea-campaign record, ■ crypto study)")
    n_oos = sum(1 for r in LEADS["leads"] if r["verdict"] == "out-of-scope")
    ax.set_title(f"Prime-field E(F_p) index-calculus leads chased, by mechanism\n"
                 f"(verdicts as recorded by each source, re-read {UPDATED}; {len(ROWS)} leads shown, "
                 f"{n_oos} out-of-scope records omitted; every cell [pcc:unverified])", fontsize=12)
    fig.tight_layout()
    fig.savefig(HERE / "lead_cells.svg")
    fig.savefig(HERE / "lead_cells.png", dpi=110)
    plt.close(fig)


PAGES = [("p1", ["FB", "ORACLE"]), ("p2", ["LIFT", "TRANSFER"]), ("p3", ["BARRIER", "ENG"])]
WRAP = 40


def lead_cells_paged():
    """Same cells, split over three A4-portrait pages; a column longer than WRAP rows wraps."""
    bycol = defaultdict(list)
    for r in ROWS:
        bycol[r["column"]].append(r)
    for c in bycol:
        bycol[c].sort(key=sort_key)
    titles = dict(COLUMNS)
    for name, keys in PAGES:
        subcols = []
        for k in keys:
            items = bycol.get(k, [])
            parts = [items[i:i + WRAP] for i in range(0, max(len(items), 1), WRAP)] or [[]]
            for pi, part in enumerate(parts):
                subcols.append((k, pi, len(parts), part, len(items)))
        n = len(subcols)
        nrows = max(len(sc[3]) for sc in subcols)
        cell_h = 0.62
        fig, ax = plt.subplots(figsize=(2.75 * n, 1.6 + nrows * cell_h * 0.42))
        ax.set_xlim(0, n)
        ax.set_ylim(-(nrows * cell_h) - 0.3, 1.1)
        ax.axis("off")
        for ci, (k, pi, nparts, items, total) in enumerate(subcols):
            cont = f" ({pi + 1}/{nparts})" if nparts > 1 else ""
            ax.text(ci + 0.5, 0.78, f"{titles[k]}{cont}\n(n = {total})", ha="center", va="center",
                    fontsize=9, fontweight="bold")
            for ri, r in enumerate(items):
                y = -ri * cell_h
                x = ci + 0.025
                flag = bool(r.get("flagship"))
                ax.add_patch(FancyBboxPatch((x, y - cell_h * 0.46), 0.95, cell_h * 0.9,
                                            boxstyle="round,pad=0.01,rounding_size=0.05",
                                            fc=VERDICT_COLOR[r["verdict"]], ec="black" if flag else "white",
                                            lw=2.2 if flag else 0.5, alpha=0.92))
                dots = STRENGTH_DOTS[r["evidence_strength"]]
                dark = r["verdict"] in ("refuted", "open", "blocked")
                ax.text(x + 0.03, y, f"{SOURCE_GLYPH.get(r['source_kind'], '?')} {r['label'][:38]}", ha="left",
                        va="center", fontsize=6.4, color="white" if dark else "black",
                        fontweight="bold" if flag else "normal")
                ax.text(x + 0.935, y, "\u25cf" * dots + "\u25cb" * (4 - dots), ha="right", va="center",
                        fontsize=5.0, color="white" if dark else "black")
        fig.tight_layout()
        fig.savefig(HERE / f"lead_cells_{name}.svg")
        plt.close(fig)


def verdict_matrix():
    counts = Counter((r["column"], r["verdict"]) for r in ROWS)
    scounts = Counter((r["verdict"], r["evidence_strength"]) for r in ROWS)
    fig, axes = plt.subplots(1, 2, figsize=(13, 4.6), gridspec_kw={"width_ratios": [6, 5]})
    ax = axes[0]
    keys = [k for k, _ in COLUMNS]
    mat = [[counts[(k, v)] for k in keys] for v in VERDICT_ORDER]
    im = ax.imshow(mat, cmap="YlOrRd", aspect="auto")
    ax.set_xticks(range(len(keys)))
    ax.set_xticklabels([t for _, t in COLUMNS], rotation=25, ha="right", fontsize=8)
    ax.set_yticks(range(len(VERDICT_ORDER)))
    ax.set_yticklabels(VERDICT_ORDER)
    for i, row in enumerate(mat):
        for j, v in enumerate(row):
            ax.text(j, i, str(v), ha="center", va="center", fontsize=9,
                    color="white" if v > max(max(mat)) * 0.6 else "black")
    ax.set_title(f"Leads by mechanism and verdict (n = {len(ROWS)})", fontsize=10)
    fig.colorbar(im, ax=ax, fraction=0.03, pad=0.02, label="leads")
    ax = axes[1]
    mat2 = [[scounts[(v, s)] for s in STRENGTH_ORDER] for v in VERDICT_ORDER]
    im2 = ax.imshow(mat2, cmap="YlOrRd", aspect="auto")
    ax.set_xticks(range(len(STRENGTH_ORDER)))
    ax.set_xticklabels(STRENGTH_ORDER)
    ax.set_yticks(range(len(VERDICT_ORDER)))
    ax.set_yticklabels(VERDICT_ORDER)
    for i, row in enumerate(mat2):
        for j, v in enumerate(row):
            ax.text(j, i, str(v), ha="center", va="center", fontsize=9,
                    color="white" if v > max(max(mat2)) * 0.6 else "black")
    ax.set_title("Verdict by evidence strength", fontsize=10)
    fig.colorbar(im2, ax=ax, fraction=0.03, pad=0.02, label="leads")
    fig.suptitle(f"Prime-field IC lead chase, {UPDATED}: verdict heat maps (counts of recorded verdicts; [pcc:unverified])",
                 fontsize=11)
    fig.tight_layout()
    fig.savefig(HERE / "verdict_matrix.svg")
    fig.savefig(HERE / "verdict_matrix.png", dpi=120)
    plt.close(fig)


if __name__ == "__main__":
    lead_cells()
    lead_cells_paged()
    verdict_matrix()
    print("wrote", HERE / "lead_cells.svg", HERE / "verdict_matrix.svg")

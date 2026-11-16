"""Present the native target-lift audit; no research computation in Python."""
from pathlib import Path
import json
import subprocess
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import Patch

study = Path(__file__).resolve().parent.parent
data = json.loads((study / "pipeline-receipts/fresh-target-relevance-20261025/receipt.json").read_text())
assert data["status"] == "all_target_roots_and_trace_criteria_agree"
folder = study / "reports/run-fresh-target-relevance-20261025"
folder.mkdir(exist_ok=True)
families = ["geometric", "geomtraceu", "random", "prefix"]
categories = [
    ("sat_liftable", "Boolean SAT, E0 lift", "#167d8d", None),
    ("sat_nonliftable", "Boolean SAT, no E0 lift", "#ba6844", "//"),
    ("unsat_liftable", "Boolean UNSAT, E0 lift", "#9bc7d2", None),
    ("unsat_nonliftable", "Boolean UNSAT, no E0 lift", "#dfc9bc", "//"),
]
plt.rcParams.update({"font.size": 9, "svg.fonttype": "none"})
fig, axes = plt.subplots(1, 4, figsize=(10.6, 3.6), sharey=True)
for ax, family in zip(axes, families):
    rows = sorted((f for f in data["files"] if f["family"] == family),
                  key=lambda f: (f["m"], f["n"], f["ell"]))
    assert len(rows) == 9
    for y, row in enumerate(rows):
        offset = 0
        for key, _, color, hatch in categories:
            ax.barh(y, row[key], left=offset, color=color, hatch=hatch,
                    edgecolor="white", linewidth=.4, height=.72)
            offset += row[key]  # Plot position only; counts are native receipt fields.
        assert offset == row["draws"]
    ax.set_title("trace-zero GP" if family == "geomtraceu" else family)
    ax.set_xlim(0, 32)
    ax.set_xticks([0, 16, 32])
    ax.set_xlabel("targets (32 per cell)")
    ax.set_yticks(range(9), [f"m={r['m']}, n={r['n']}, ell={r['ell']}" for r in rows])
    ax.grid(axis="x", alpha=.15)
    ax.spines[["top", "right"]].set_visible(False)
axes[0].invert_yaxis()
fig.legend(handles=[Patch(facecolor=c, hatch=h, label=l) for _, l, c, h in categories],
           loc="lower center", ncol=2, frameon=False, bbox_to_anchor=(.55, -.01))
fig.tight_layout(rect=(0, .12, 1, 1))
for ext in ("svg", "png", "pdf"):
    p = folder / ("target-lift-classes." + ext)
    fig.savefig(p, bbox_inches="tight", dpi=150)
    if ext == "svg":
        p.write_text("\n".join(line.rstrip() for line in p.read_text().splitlines()) + "\n")
plt.close(fig)
t = data["totals"]
text = rf"""\documentclass[10pt]{{article}}
\usepackage[margin=17mm]{{geometry}}
\usepackage{{graphicx,booktabs,hyperref}}
\usepackage[strings]{{underscore}}
\setlength{{\parindent}}{{0pt}}\setlength{{\parskip}}{{5pt}}
\begin{{document}}
\section*{{Fresh fixtures: target lifts on the E0 reference curve}}
Native correctness/relevance run, 2026-10-25. All 1,152 frozen targets
across 36 files, nine parameter cells and four families are retained.
E0 is $y^2+xy=x^3+1$ over each input's exact GF($2^n$) representation.
The Boolean solution count is copied from the frozen fixture, not recomputed
by this audit. Complete y-root solving, substitution and an independent
Artin--Schreier trace criterion agree for every target.

\begin{{center}}\begin{{tabular}}{{lrr}}\toprule
Frozen Boolean class & E0 lift & No E0 lift\\\midrule
SAT & {t['sat_liftable']} & {t['sat_nonliftable']}\\
UNSAT & {t['unsat_liftable']} & {t['unsat_nonliftable']}\\
\bottomrule\end{{tabular}}\end{{center}}
\begin{{center}}\includegraphics[width=\linewidth]{{target-lift-classes.pdf}}\end{{center}}

\textbf{{Scope.}} These are x-only Boolean fixtures. They do not specify a
full curve coefficient a, group order or subgroup. The b=1 S3 equation is
independent of a. This audit explicitly chooses a=0 as its reference model;
``no E0 lift'' is not invalidity for every b=1 curve or twist. A positive E0
lift establishes neither subgroup membership nor a signed factor-base point
decomposition. The uniform field-x target law is not an ordinary subgroup
query law. Preserve all four classes; do not silently filter them or
retrofit a different experiment.

\textbf{{Consequence.}} The closure panels and guess branches remain finite
algebraic evidence. Their relevance to ordinary IC relation generation
needs separate curve/subgroup-qualified inputs, complete lifting and
relation verification. No degree bound, security exponent, speed or cost
improvement follows from this diagnostic.

Frozen audit source: c3b2bd27d, \path{{examples/ic_fresh_relevance.rs}}.
The command is \path{{ic_fresh_relevance inputs/fresh}}; protocol:
\path{{../../FRESH_RELEVANCE_PROTOCOL.md}}. Local shared x86-64 host, L0,
12-GiB address-space limit and 900-second process limit; exit 0. The complete
JSON binds all 36 input files by SHA-256 and retains every target and y root.
A second native execution is byte-identical. This is a deterministic rerun,
with independent trace checking inside the auditor, not an external
large-cell solver replay or hardware measurement.

Receipt: \path{{../../pipeline-receipts/fresh-target-relevance-20261025/receipt.json}}.
Source/binary hashes, status, times and replay comparison accompany it.
\end{{document}}
"""
(folder / "report.tex").write_text(text)
with (folder / "render.log").open("w") as log:
    subprocess.run(["pdflatex", "-interaction=nonstopmode", "-halt-on-error", "report.tex"],
                   cwd=folder, stdout=log, stderr=subprocess.STDOUT, check=True)
print("Rendered native fresh-target reference-curve classes.")

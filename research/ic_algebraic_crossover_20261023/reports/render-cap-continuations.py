"""Render native fixed-point rows and one completed zero-guess pilot."""
from pathlib import Path
import json
import subprocess
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent.parent
pre = r"""\documentclass[10pt]{article}
\usepackage[margin=17mm]{geometry}
\usepackage{graphicx,booktabs,hyperref}
\usepackage[strings]{underscore}
\setlength{\parindent}{0pt}\setlength{\parskip}{5pt}
\begin{document}
"""

def pdf(folder, title, body):
    (folder / "report.tex").write_text(pre + "\\section*{" + title + "}\n" + body + "\n\\end{document}\n")
    with (folder / "render.log").open("w") as log:
        subprocess.run(["pdflatex", "-interaction=nonstopmode", "-halt-on-error", "report.tex"],
                       cwd=folder, stdout=log, stderr=subprocess.STDOUT, check=True)

data = json.loads((root / "pipeline-receipts/hky-probes-20261025/receipt.json").read_text())
folder = root / "reports/run-hky-probes-20261025"
folder.mkdir(parents=True, exist_ok=True)
plt.rcParams.update({"font.size": 10, "svg.fonttype": "none"})
fig, ax = plt.subplots(figsize=(7.3, 3.1))
for cell, color in zip(data["cells"], ["#147d92", "#ac6146", "#654b8c"]):
    rows = cell["degrees"]
    ax.plot([r["degree"] for r in rows], [int(r["columns"]) for r in rows], marker="o",
            color=color, label=f"N={cell['variables']}")
ax.set_xticks([2, 3, 4])
ax.set_yscale("log")
ax.set_xlabel("closure cap; degree 4 remains unknown in every cell")
ax.set_ylabel("monomial universe columns")
ax.legend(frameon=False)
ax.grid(axis="y", alpha=.2)
fig.tight_layout()
for ext in ("svg", "png", "pdf"):
    p = folder / ("monomial-universes." + ext)
    fig.savefig(p, bbox_inches="tight", dpi=150)
    if ext == "svg":
        p.write_text("\n".join(l.rstrip() for l in p.read_text().splitlines()) + "\n")
plt.close(fig)
table = "\n".join(f"{c['variables']} & {c['fixture']['m']} & {c['fixture']['n']} & {c['fixture']['ell']} & " +
                  f"{c['degrees'][0]['closure_dimension']:,} & {c['degrees'][1]['closure_dimension']:,} & " +
                  ("entry cap" if c['variables'] == 100 else "allocation failed") + r" \\" for c in data["cells"])
body = rf"""Three registered larger-variable Boolean probes, 2026-10-25.
Each uses the frozen first random-family UNSAT draw named by its input
manifest: N=58 draw 2, N=60 draw 0, N=100 draw 0. N counts Boolean variables,
not field or subgroup bits. Parameters and terminal dimensions:

\begin{{center}}\begin{{tabular}}{{rrrrrrl}}\toprule
N & m & field n & ell & dim at D2 & dim at D3 & D4 result\\\midrule
{table}
\bottomrule\end{{tabular}}\end{{center}}
\begin{{center}}\includegraphics[width=.98\linewidth]{{monomial-universes.pdf}}\end{{center}}

All six degree-two/three closures saturate without refuting. Dense N=58 and
N=60 begin degree four and abort with allocation-failure diagnostics under
the 12-GiB address-space limit (exit 134). Sparse N=100 begins degree four
with an implicit 4,087,976-column universe, then reaches the 25-million-entry
cap (process exit 0, scientific verdict incomplete). An ordinary process exit
does not establish a fixed point. No degree-four refutation or saturation is
claimed for any cell. The sparse universe is implicit, not a dense allocation.

Limits: 900 seconds per command, 12 GiB address space, dense outer cap
600,000 columns; sparse storage has its separate entry cap. Four Rayon
threads, HKY_CHUNK=256, shared x86-64 host, L0. The u128/sparse instrument
does not remove the production library's 64-variable boundary. Dimensions
are native observations; no independent large fixed-point certificate is
provided. Exact frozen Boolean solution counts do not establish ordinary
curve/subgroup-query relevance, which remains unknown here. No bounded-degree
law, complete DLP cost, speedup or hardware result follows.

The native collector checks every input/row hash, declared variable count,
process failure and terminal status, and reproduces byte for byte.
Source: \path{{examples/ic_hky_probe_summary.rs}}.
Receipt: \path{{../../pipeline-receipts/hky-probes-20261025/receipt.json}}.
Raw archive under evidence/: \path{{runs-local-followup-20261025-01.tar.xz}}.
It contains all three probes and only the separate completed draw-6 pilot;
the remaining live branch campaign is not represented as finished.
Protocols: PROTOCOL.md and GUESS_CAP200K_PROTOCOL.md. The complete remote
continuation and same-input external large-cell checks remain outstanding.
"""
pdf(folder, "N=58/60/100: fixed points and retained caps", body)

folder = root / "reports/run-guess-zero-draw6-20261025"
folder.mkdir(parents=True, exist_ok=True)
p = root / "runs/guess-cap200k-20261025-01/cell-0000-d6-k0-b0/rows.jsonl"
row = json.loads(p.read_text())
assert row["guessed_variables"] == [] and row["status"] == "refuted"
dot = rf'''digraph caps {{
  graph [rankdir=LR, bgcolor="white", margin=0.1];
  node [shape=box, style="rounded,filled", fillcolor="#edf3f5", fontname="Helvetica", fontsize=11];
  edge [color="#536879"];
  original [label="Original draw 6\n46 Boolean variables\nno guessed bits"];
  low [label="Earlier 140,000-column cap\nrefused before closure\nverdict unknown", fillcolor="#f4e8e0"];
  high [label="200,000-column continuation\n179,447 columns admitted\ndegree-4 refutation observed", fillcolor="#dcefed"];
  original -> low;
  original -> high;
}}'''
(folder / "cap-diagram.dot").write_text(dot)
for ext in ("svg", "pdf", "png"):
    subprocess.run(["dot", "-T" + ext, "-Gdpi=150", "cap-diagram.dot", "-o", "cap-diagram." + ext], cwd=folder, check=True)
body = rf"""First registered 200k-cap pilot, 2026-10-25; protocol and sequential
runner committed at f83ed00d8 before execution. Original ell=8 UNSAT draw 6,
(m,n,ell)=(3,22,8), frozen input unchanged, no guessed variables. Degree-four
dense closure returns refuted. This is an observed zero-guess refutation
for this input; independent solver/cost replay is still outstanding.

\begin{{center}}\includegraphics[width=.98\linewidth]{{cap-diagram.pdf}}\end{{center}}

Native recorded columns: {row['columns']:,}; terminal rank {row['rank']}.
Charged closure products: {row['closure_products']:,}; solver-word operations:
{row['closure_word_ops']:,}. Every branch of this zero-guess cell is charged
(one branch). These are solver-stage counts, not complete decomposition or
DLP costs. A single comparable cell gets a resource-gate diagram instead
of a fitted cost curve. The prior cap refusal is retained, not a failed
refutation or zero-cost comparison. Raising the cap is not an algorithmic gain.

The result shows that the upstream all-branches four-guess observation is
not evidence that four guesses are necessary on every fixture. Draw 0 still
has eight nonrefuting balanced k=3 branches; minima for other inputs and
variable sets remain unresolved. No universal degree law or scaling claim
is made. The 115 remaining registered attempts were ongoing when this pilot
snapshot was made, with individual caps/failures retained.

The independent fixture audit counts zero Boolean solutions and finds this
target abscissa liftable on E0 over GF($2^{{22}}$). Target/base subgroup
membership remains unknown. A curve lift does not make this a verified
subgroup-decomposition query or a full IC relation.

Resources: unchanged 12-GiB address-space and 900-second limits, four Rayon
threads, shared host L0; diagnostic time is in the raw row and resource receipt.
Frozen instrument SHA-256 starts ab544a41; build source d67f6085, with the later
source-only parity lint change excluded. Full hashes and command are archived.

Raw cell: \path{{../../runs/guess-cap200k-20261025-01/cell-0000-d6-k0-b0/}}.
Stable pilot snapshot under evidence/: \path{{runs-local-followup-20261025-01.tar.xz}}.
Original input: \path{{../../inputs/guess-ell8.jsonl}}.
Protocol: \path{{../../GUESS_CAP200K_PROTOCOL.md}}.
The original 33-run archive and all earlier reports stay unchanged.
"""
pdf(folder, "ell=8 draw 6: zero-guess degree-four refutation", body)
print("Rendered capped large-variable probes and the completed zero-guess pilot.")

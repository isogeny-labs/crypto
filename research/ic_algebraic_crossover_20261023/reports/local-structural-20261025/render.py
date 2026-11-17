"""Render recorded native report-data.json values only. No experiment or inference."""
from pathlib import Path
import json
import subprocess
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = Path(__file__).resolve().parent
STUDY = HERE.parent.parent
DATA = json.loads((STUDY / "report-data.json").read_text())
COLORS = {2: "#147d92", 3: "#bd7c17", 4: "#a6445e"}
plt.rcParams.update({"font.size": 10, "axes.spines.top": False,
                     "axes.spines.right": False, "svg.fonttype": "none"})

def save(fig, folder, name):
    folder.mkdir(parents=True, exist_ok=True)
    for suffix in ("svg", "pdf", "png"):
        fig.savefig(folder / (name + "." + suffix), bbox_inches="tight", dpi=150)
        if suffix == "svg":
            path = folder / (name + ".svg")
            path.write_text("\n".join(line.rstrip() for line in path.read_text().splitlines()) + "\n")
    plt.close(fig)

def ratio_plot(panels, folder, name):
    fig, ax = plt.subplots(figsize=(7.0, 3.5))
    for panel in panels:
        degree = panel["degree"]
        # Horizontal offsets distinguish overprinted records, not field values.
        x = [p["n"] + {2: -.22, 3: 0, 4: .22}[degree]
             + (p["ell"] - 3) * .035 for p in panel["points"]]
        y = [p["ratio"] for p in panel["points"]]
        ax.scatter(x, y, s=13, alpha=.46, color=COLORS[degree],
                   label=f"closure degree {degree} (304 pairs)", rasterized=False)
    ax.axhline(1, color="#34444d", lw=1, ls="--", label="equal recorded solver work")
    ax.set_yscale("log")
    ax.set_ylim(.03, 30000)
    ax.set_xticks([13, 19, 23, 31])
    ax.set_xlabel("binary field degree n (small offsets separate records)")
    ax.set_ylabel("symmetric / direct solver word operations")
    ax.grid(axis="y", alpha=.18)
    ax.legend(fontsize=8, loc="lower center", bbox_to_anchor=(.5, 1.12),
              ncol=2, frameon=False)
    ax.set_title("Finite matched queries; construction and splitting unpriced", fontsize=11)
    save(fig, folder, name)

PREAMBLE = r"""\documentclass[10pt]{article}
\usepackage[margin=18mm]{geometry}
\usepackage{graphicx,booktabs,array,hyperref}
\hypersetup{colorlinks=true,urlcolor=blue}
\setlength{\parindent}{0pt}\setlength{\parskip}{5pt}
\begin{document}
"""

def pdf(folder, title, body):
    tex = PREAMBLE + r"\section*{" + title + "}\n" + body + "\n\\end{document}\n"
    (folder / "report.tex").write_text(tex)
    with (folder / "render.log").open("w") as log:
        subprocess.run(["pdflatex", "-interaction=nonstopmode", "-halt-on-error", "report.tex"],
                       cwd=folder, stdout=log, stderr=subprocess.STDOUT, check=True)

ratio_plot(DATA["panels"], HERE, "closure-ratios")

for panel in DATA["panels"]:
    degree = panel["degree"]
    folder = STUDY / "reports" / f"run-structural-d{degree}-20261023"
    ratio_plot([panel], folder, "solver-words")
    run_label = panel["run"].replace("_", r"\_")
    body = rf"""Run: \texttt{{{run_label}}}. Experiments 2026-10-23; report 2026-10-25.

The frozen panel has 96 commands and {panel['raw_rows']} rows: $n=13,19,23,31$,
$\ell=2,3,4$, prefix/geometric/trace-zero geometric/random bases, two recorded
base replicates and four public target draws. Prefix replicate 1 is a duplicate;
{panel['duplicate_prefix_rows']} rows are excluded from the primary table.
Three formulations run: chained S3, direct S4, symmetric product-space S4.

Limits: degree-{degree} closure, 30,000 columns, 12 GiB address space, 900 seconds
per command. This shared host earns L0: timings are diagnostic only. The hybrid
extraction has an input-degree floor, which reaches six for direct S4. These
are not equal-degree or whole-query speed measurements.

There are {panel['paired_queries']} completed direct/symmetric pairs among
336 distinct input queries. The remaining 32 are unpaired, not zero-cost.
Symmetric solver work is lower in {panel['symmetric_lower_words']} pairs,
higher in {panel['symmetric_higher_words']}, equal in {panel['equal_words']}.
Recorded ratios range {panel['ratio_min']:.4g} to {panel['ratio_max']:.4g}.
Independent direct point enumeration checks every completed answer; the audit
reports {panel['complete_extractions']} complete extractions and zero failures.

\begin{{center}}\includegraphics[width=.96\linewidth]{{solver-words.pdf}}\end{{center}}

For the completed symmetric queries, {panel['symmetric_algebraic_roots']} algebraic
roots produced {panel['symmetric_accepted_multisets']} verified x-multisets;
{panel['symmetric_rejected_roots']} roots failed splitting or signed lifting.
Construction, splitting and physical conversion are not priced in solver words.
No end-to-end DLP cost, rho ratio, scaling law or external-CAS speed claim is made.

Raw rows: \path{{../../runs/{panel['run']}/cell-*/rows.jsonl}}.
The native recomputation is \path{{examples/ic_campaign_report_data.rs}};
plot values are \path{{../../report-data.json}}. The frozen protocol, controls,
input/binary hashes, cap refusals and setup failures remain in the study.
"""
    pdf(folder, f"Structural closure panel: degree {degree}", body)

guess = DATA["balanced_guess_draw0"]
folder = STUDY / "reports" / "run-guess-k3-draw0-20261023"
fig, ax = plt.subplots(figsize=(7, 2.8))
ax.bar([r["branch"] for r in guess["branches"]],
       [r["rank"] for r in guess["branches"]], color="#147d92")
ax.axhline(136698, color="#a6445e", ls="--", label="136,698 matrix columns")
ax.set_xticks(list(range(8)))
ax.set_xlabel("assignment of guessed variables 7, 15, 23")
ax.set_ylabel("saturated closure rank")
ax.set_ylim(0, 150000)
ax.set_title("Original draw 0: all eight branches saturate without refuting")
ax.legend(frameon=False, fontsize=8, loc="lower right")
save(fig, folder, "branch-ranks")
body = rf"""Original fixture from PR 2429: $(m,n,\ell)=(3,22,8)$, draw 0,
46 declared variables, guesses $[7,15,23]$, degree four, all eight assignments.
Each process has a 140,000-column, 12 GiB address-space and 900-second cap.
The internal library caps were explicitly raised to match the instrument;
eight earlier immediate engine-cap refusals are retained unscored.

All eight measurement processes exited zero. None refutes. Branches 0--3
saturate at rank 129,221, branches 4--7 at 129,473, against 136,698 columns.
The branch-0 rank agrees exactly with the upstream result; the other seven
branches were unmeasured there. Every branch is charged: the native total is
{guess['total_word_ops']:,} solver word operations and {guess['total_closure_products']:,}
closure products. Elapsed time is diagnostic at L0.

\begin{{center}}\includegraphics[width=.96\linewidth]{{branch-ranks.pdf}}\end{{center}}

This disproves sufficiency of this three-set on this draw. It does not establish
a minimum over all subsets, an ell=9 result, a complete decomposition algorithm,
or an IC improvement. Other draws and alternative three-sets remain open.
The original upstream four-set is an unmerged upper bound, not a new result here.

Raw paths: \path{{../../runs/guess-balanced-k3-d0-b*-20261023-02/}}.
The native report-data recomputation checks all eight rows, caps and statuses.
"""
pdf(folder, "Three-guess independent replay: complete draw 0", body)

table = "\n".join(f"{p['degree']} & {p['raw_rows']} & {p['complete_extractions']} & "
                  f"{p['symmetric_lower_words']}/304 & {p['ratio_min']:.4g}--{p['ratio_max']:.4g} \\\\"
                  for p in DATA["panels"])
body = rf"""Report date: 2026-10-25. Status: finite stage evidence; the wider campaign is partial.

Degree-two symmetric preprocessing has fewer recorded solver words in
263 of 304 completed paired queries. Degrees three and four have more in
every completed pair. These are native count observations, not full-query
or DLP speedups. The direct hybrid's input-degree floor may reach six.

\begin{{center}}\begin{{tabular}}{{rrrrl}}\toprule
Closure & raw rows & complete extractions & symmetric lower & ratio range\\\midrule
{table}
\bottomrule\end{{tabular}}\end{{center}}

\begin{{center}}\includegraphics[width=.96\linewidth]{{closure-ratios.pdf}}\end{{center}}

All three panels use the same public targets and seeded bases at $n=13,19,23,31$,
$\ell=2,3,4$. Prefix duplicates are retained in raw data and excluded from the
primary comparison. There are 336 distinct inputs, 304 completed matched pairs;
32 incomplete pairs are excluded and remain unknown. Direct group enumeration
found no answer mismatch. The 8,100 symmetric algebraic roots in the completed
queries yielded only four verified x-multisets; 8,096 roots failed split/lift.

\newpage
\section*{{Guessing, controls, and what remains}}
For the balanced three-set $[7,15,23]$ at the original ell=8 draw 0,
all eight branches now saturate degree four without refuting. The rank is
129,221 on branches 0--3 and 129,473 on 4--7. This independent replay agrees
with the upstream branch-0 result and fills seven previously unmeasured
branches. It establishes failure of this set on this draw, not a global minimum.

The exact fixture oracle independently reproduces all seven original counts;
all three SAT controls have checked Boolean witnesses. Small controls cover
2,304 direct assignments, every corresponding small symmetric root, repeated
roots and 2,880 signed-lift cases. The u128 builder matches the library's actual
equations at m=3--6; sparse and parallel fixed points match the dense controls.

The first-two-summand exchange is checked on all 308 coordinate equations of
the seven original fixtures. Certificate \texttt{{IDC1ha27a77e72057f9cb}} also
checks S3's ordinary integer-polynomial symmetry. Its formal-proof pointer is
unset. The permutation equates degree verdicts for $[7,23,24]$ and $[15,23,24]$,
not their costs.

Original-setup failures and internal-cap refusals are retained. The first
structural outer shell ended with a syntax error after all 96 native commands
had succeeded; its amended-on-disk runner caused the tail error. Subsequent
panels execute frozen runner copies. No failed attempt is assigned zero cost.

No full-price IC gain or rho crossover is established. Complete DLP smoke,
remaining guess draws, ell=9, actual N=58/60 continuations, sparse N=100,
quartic replay/scaling, wide arithmetic and external-CAS/hardware controls
remain separate obligations. The current quartic implementation explicitly
refuses m=16--18. Source-pinned jobs and blocked backlog cards preserve them.

The owner's total additional cloud-compute ceiling is USD 100. No paid job
had been launched when this report was prepared. A read-only CI access probe
is separate from a compute allocation.

\section*{{Evidence and scope}}
Native recomputation: \path{{examples/ic_campaign_report_data.rs}}.
Raw values and audit pointers: \path{{../../report-data.json}}. Per-panel
reports are siblings under \path{{../}}. Frozen requirements and amendments:
\path{{../../PROTOCOL.md}}. Upstream references:
\href{{https://github.com/aburan28/crypto/pull/2214}}{{fall-degree PR 2214}};
\href{{https://github.com/aburan28/crypto/pull/2429}}{{guessing PR 2429}};
\href{{https://github.com/aburan28/cryptanalysis/tree/claude/cryptanalysis-repo-setup-p59kek/experiments/pdp-degree-heuristics}}{{cryptanalysis structured-base study}}.
Solver construction, cubic splitting, lifting, physical conversion and the
complete IC/rho pipeline are not collapsed into the plotted word ratios.
"""
pdf(HERE, "Native algebraic IC follow-up: measured stage results", body)
print("Rendered three structural run reports, one guessing report, and the stage report.")

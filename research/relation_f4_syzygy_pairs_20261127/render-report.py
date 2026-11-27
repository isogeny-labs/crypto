"""Render Rust-produced counts and tables, without mathematical analysis."""
from pathlib import Path
import json
import subprocess

s = Path(__file__).resolve().parent
data = json.loads((s / "REPORT_DATA.json").read_text())
rows = []
for row in data["rows"]:
    if not row["original"]["f4_supported"]:
        continue
    a, b, c = (row[k] for k in ["original", "full", "first_pair_only"])
    rows.append(" & ".join([
        str(row["n"]),
        " / ".join(str(r["productive_degree"]) for r in [a, b, c]),
        " / ".join(str(r["max_cols"]) for r in [a, b, c]),
        " / ".join(str(r["row_reduction_multiplications_only"]) for r in [a, b, c]),
    ]) + r"\\")
tex = r"""\documentclass[10pt,a4paper]{article}
\usepackage[margin=18mm]{geometry}
\usepackage{booktabs,hyperref}
\pagestyle{empty}
\begin{document}
\noindent{\Large\bfseries Two syzygies retain degree 4 but fail the space gate}\\[4pt]
{\small Exploratory Rust panel, 27 November 2026. Draft PR \#5251. No application timing.}
\paragraph{Result.} All 14 tested distinct-edge syzygy pairs retain highest
productive regular-F4 step degree 4 on the existing 31-bit fixtures. All 41
supported calls pass independent ideal-equality and every Buchberger pair check.
Fresh original A/A matches exactly at all three dimensions; original/full
algebraic observations reproduce the prior study. This supports the fixed-input
pair hypothesis, not intrinsic regularity or a speedup. [pcc:unverified]
\begin{center}\begin{tabular}{rrrr}\toprule
$n$ & Productive degree & Maximum columns & Row multiplications\\\midrule
ROWS
\bottomrule\end{tabular}\end{center}
\noindent Each entry is original / full cubic pool / first distinct-edge pair.
The first pair is selected lexicographically, not by performance. Multiplications
cover row reduction only. Full construction, proof checks, other arithmetic,
transport and strongest structured controls remain outside these counters.
Internal engine clocks/deadlines remain; no elapsed fields are retained or
compared. No full-call gain or 15$\times$ admission. [pcc:unverified]
\paragraph{Pruning gate.} All 76 proper-subset arms lose declared D3 information
at their original prime; only the six full-pool arms preserve all declared
closed D2/3/4 spaces. There are 29 degree 4 proper-subset arms at 31 bits, all
rejected by the space gate. The pair-only D3 rank is 2, versus full 4/6/10 at
$n=3/4/6$. Exact snapshot derivations/membership are replayed; censoring never
passes. Other caps are uncomputed after an obstruction, not silently passed.
Equal observed solving degree therefore cannot authorize deleting these
relations. No P/Q labels exist, so the scalar gate is unassessed. [pcc:unverified]
\paragraph{Prospective panel.} Six unchanged input hashes, primes 2147483647
and 9007199254740881, seed7 LAST-star, $n=3,4,6$. Original A/A, full pool, all
differences, all syzygies and every distinct-edge pair with/without differences
are fixed before execution. Exact subset duplicates are removed except A/A.
82 declared arms comprise 41 actual 31-bit F4 calls and 41 unsupported 53-bit solver arms
deliberately not entered; their 41 polynomial gates still execute at the original
field. Existing 53-bit engine panics remain in the prior study. No new seeds,
substitute field, selection reruns or cap escalation. [pcc:unverified]
\paragraph{Proof and resources.} All originals remain in order. 80 auxiliary
coefficient/proof mutations are rejected. Six fresh packed references match
prior wires exactly and pass public native/independent wire checks; 24 reference
and checker canaries reject mutations/exhaustion. All 82 space-gate arms,
18 baseline snapshots and 41 completed F4 bases are replayed. 10 harness units pass.
CPU0/thread1; unchanged Root0 opt0/LTOoff/CG16/debug0 build, no production edit.
Grevlex degree 8; 100000 pairs, 10000 symbolic monomials, 1000000 matrix entries,
10 second cooperative deadlines; AS6GiB/file128MiB/per-case600seconds. Zero
cloud compute. Seven end-to-end replay corruptions are rejected with exact
diagnostics and no summary: gate flag, missing cap, rank, snapshot/basis
coefficients, A/A counter and fabricated unsupported observation. Original
data hashes stay unchanged; these checks add no F4/reference calls. No
whole-root or optimized compiler failure is relabeled. [pcc:unverified]
\paragraph{Source and standing.} The panel is frozen before main 99b949b383b,
which changes two root dependency files. It does not qualify a merged new-main
build. Prior source/evidence remains byte-exact. KB tools are unavailable;
the exact restored wrapper exits 127 for missing executable and the atlas helper
is absent. Actual source/prior executable review is the bounded fallback,
without novelty claim. This is unsigned producer evidence. [pcc:unverified]
\paragraph{Next lead.} Lower auxiliary cardinality cannot enter the current
space-preserving pruner. Investigate sparse changes of auxiliary basis with
all declared spaces replayed, or retain the full collection while scheduling
only selected helper equations for F4. Neither has been run or integrated here;
both still need comparison against strongest structured controls at the agreed
full-call boundary. Original 15$\times$ remains active and unachieved.
Conductor/checkpoint access is waived; the separate timing-document choice
remains unanswered. [pcc:unverified]
\end{document}
""".replace("ROWS", "\n".join(rows))
(s / "report.tex").write_text(tex)
subprocess.run([
    "xelatex", "-interaction=nonstopmode", "-halt-on-error",
    "-output-directory", str(s), str(s / "report.tex")
], cwd=s, check=True, stdout=(s / "raw/report-build.stdout").open("x"),
    stderr=(s / "raw/report-build.stderr").open("x"))

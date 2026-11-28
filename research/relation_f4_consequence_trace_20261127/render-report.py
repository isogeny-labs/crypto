"""Render recorded Rust output only; no mathematical/statistical analysis."""
from pathlib import Path
import json
import subprocess

s = Path(__file__).resolve().parent
v = json.loads((s / "OUTCOME.json").read_text())
rows = []
for c in v["cases"]:
    if c["status"] != "observed":
        continue
    a = next(r for r in c["observations"] if r["arm"]["id"] == "original-a")
    b = next(r for r in c["observations"] if r["arm"]["id"] == "full")
    rows.append(" & ".join([
        c["case"].split("-n")[1],
        f'{a["productive_degree"]} / {b["productive_degree"]}',
        f'{a["max_rows"]} / {b["max_rows"]}',
        f'{a["max_cols"]} / {b["max_cols"]}',
        f'{a["row_reduction_multiplications_only"]} / {b["row_reduction_multiplications_only"]}'
    ]) + r"\\")
tex = r"""\documentclass[10pt,a4paper]{article}
\usepackage[margin=18mm]{geometry}
\usepackage{booktabs,hyperref}
\pagestyle{empty}
\begin{document}
\noindent{\Large\bfseries Certified cubics change actual F4 step degree}\\[4pt]
{\small Rust diagnostic, 27 November 2026. Draft PR \#5251. No application timing.}
\paragraph{Observed result.} On the existing $p=2147483647$, $n=3,4,6$, seed7
LAST-star fixtures, full cubic augmentation lowers the highest productive
regular-F4 step degree from 5 to 4. All 49 completed arms pass independent full
ideal-equality and Buchberger checks; three original A/A pairs match exactly.
Of 43 candidate arms, 21 lower degree, 22 retain degree, 0 raise it. Every singleton
stays at 5. No completed 31-bit observation is censored. [pcc:unverified]
\begin{center}\begin{tabular}{rrrrr}\toprule
$n$ & Degree & Maximum rows & Maximum columns & Row multiplications\\
\midrule
ROWS
\bottomrule\end{tabular}\end{center}
\noindent Each entry shows original / full pool. Multiplications cover row
reduction only, not construction, other arithmetic, checking, transport or total
application cost. Matrix maxima and step degrees describe these runs; intrinsic
regularity remains unknown. The strongest existing structured controls are
retained and are not measured in this table. [pcc:unverified]
\paragraph{Fixed design.} All 98 arms were declared before F4 execution: originals
A/A, full pool, every singleton and every leave-one-out on six unchanged input
hashes at both fixed primes. Every original quartic f3 equation remains in
order; auxiliary cubics have original-ideal certificates. Grevlex, degree 8,
100000 pairs, 10000 symbolic monomials, 1000000 matrix-workspace entries and a 10 second
cooperative deadline are identical for all arms. CPU0, one numerical thread,
opt0 correctness build, AS6GiB, no new cloud compute. No new selection seeds.
\paragraph{53-bit failure retained.} Actual regular F4 requires $p<2^{32}$ in
its Barrett reducer. Each first original arm at $p=9007199254740881$ panics with
exit101; 46 later arms never execute. 52 F4 entries occurred: 49 completed, 3 panicked.
All 49planned53-bit observations remain unavailable, with no substituted field
or solver. All six reference proofs/native rechecks passed before these
panics. The complete-panel analyzer's missing-output failure stays retained.
Additive Rust analysis requires exact field, exit, panic and absence of output
before this classification. [pcc:unverified]
\paragraph{Proof and trace.} Existing packed reference construction is
verification-only setup, not free candidate work. Independent wire replay,
native public reverify, 80 auxiliary mutation assertions and 24 reference/checker
canaries pass in the final execution. Separate sparse u128 modular division
checks both ideal inclusions and all S-pairs, then rechecks all 49 completed
bases in additive analysis. Budget exhaustion fails closed. Eight distinct
unit tests execute 21 times across three harness contexts. Read-only progress
records degree/rows/columns/counters. Internal engine clocks/deadlines remain,
but elapsed fields are neither read nor serialized. Incidental compiler/test
durations are not application measurements. [pcc:unverified]
\paragraph{Setup correction.} Attempt-v1 retains six export-order comparison
failures before any F4 entry. Sorting both term collections repairs equality
without changing polynomials. Its source, raw logs and prepared inputs remain.
Twelve reference constructions include these six failures. Production code,
prior algorithms, evidence, tests, keys and gates are unchanged.
\paragraph{Next hypothesis and limit.} Two distinct-edge syzygies may suffice;
the pair-only panel has not run. This is exploratory on known fixtures. A
degree-preserving subset must still meet the existing stricter bounded-space
and optional augmented-scalar-span preservation gates; prior D2/3/4 pruning
retained every cubic. P/Q scalar labels are absent here; no logarithm recovery.
Original 15$\times$ remains unachieved. It still needs repeated full-call
measurements against strongest matched controls, all fixed cases, A/A and every
proof/failure. Conductor/S3 are waived; the separate timing-document choice
remains unanswered. [pcc:unverified]
\end{document}
""".replace("ROWS", "\n".join(rows))
(s / "report.tex").write_text(tex)
subprocess.run([
    "xelatex", "-interaction=nonstopmode", "-halt-on-error",
    "-output-directory", str(s), str(s / "report.tex")
], cwd=s, check=True, stdout=(s / "raw/report-build.stdout").open("x"),
    stderr=(s / "raw/report-build.stderr").open("x"))

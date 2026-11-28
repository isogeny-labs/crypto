"""Render compiler/resource records only; not application/statistical analysis."""
from pathlib import Path
import json
import subprocess
s = Path(__file__).resolve().parent
v = json.loads((s / "OUTCOME.json").read_text())
rows = "\n".join(f'{c["root_codegen_units"]} & {c["exit"]} & {c["resource_refusal"]["cgroup_memory_bytes"]} & pass '+r"\\" for c in v["contexts"])
tex = r"""\documentclass[10pt,a4paper]{article}
\usepackage[margin=18mm]{geometry}\usepackage{booktabs,hyperref}\pagestyle{empty}
\begin{document}
\noindent{\Large\bfseries Fresh main99 root build: two memory refusals}\\[4pt]
{\small 27 November 2026. Draft PR \#5251. No application timing or native pass.}
\paragraph{Outcome.} Neither bounded compiler layout produces a fresh root
worker. Both are stopped by their own14GiB shared-cgroup refusal, exit 143.
Every source postcheck passes and cgroup OOM-kill stays 0. The new36original
native cases,108matched pairs,1116mutation assertions,6unsupported refusals
and144separate proof audits/7canaries remain unexecuted. Older frozen workers
are not substituted. These are resource outcomes, not mathematical failures
or passes. Compiler trials end at the declared two contexts. [pcc:unverified]
\begin{center}\begin{tabular}{rrrr}\toprule
Root CG units & Exit & Cgroup bytes at refusal & Source check\\\midrule
ROWS
\bottomrule\end{tabular}\end{center}
\paragraph{Exact target.} Source3ddaa65f includes normal main 99b949b383b and
the existing PR implementation. Cargo compiles the full non-test production
crypto library as the unchanged native integration harness dependency. No
global root libtest or previously failed opt3configuration is repeated. The
second layout changes only crypto package codegen256; all dependencies and
harness stay 16. Its actual compiler metadata confirms opt0,256units and no
libtest flag. There is no algorithm, test, key or gate edit. [pcc:unverified]
\paragraph{Fixed budget.} CPU0/job1/thread1, zero cloud compute. Both layouts
use release optimization 0, LTO off, debug information 0, incremental compilation
off and default features disabled. Limits are 12 GiB address space, 1 GiB per
file, 900 second deadlines, 14 GiB shared-cgroup refusal, 512 MiB tmpfs floor and
2.5GiB target growth. Samples are taken every 2 seconds. Memory includes cache pages
and other processes, not exclusive compiler RSS. Samples may overshoot the
threshold before the next probe. No cap escalation or identical-layout retry.
The later native/auditor limits are declared but those calls never start.
\paragraph{Reversible cache handling.} Exactly two premerge non-executable,
single-link library cache files are checked against the existing archive,
inventory, selected member hashes and own-user live references before release.
Their byte-exact backup remains. Exactly three failed-context cache files are
separately backed up, member-verified and released before layout2. Every
protected frozen worker and every source byte remains unchanged. These are
RAM-local rebuildable cache records, not distributed mathematical evidence.
Unqualified partial codegen256outputs remain intact. [pcc:unverified]
\paragraph{Retained setup limitation.} An early layout 2 launcher is invoked
before cache validation finishes; its required receipt is absent, so it
returns before Cargo or any mathematical call. The actual compiler starts
only after helper success. This failure and both actual refusals are retained.
Old and new raw/preflight/source records are separate. Initial missing KB
wrapper and exact restored-wrapper status 127 are recorded; the KB executable
is unavailable. Source review is a bounded fallback, with no novelty claim.
\paragraph{Standing and next work.} Native source qualification is still
uncomputed, with all original fixed primes, $n=3,4,6$, seed7 LAST-star, proof and failure
gates retained. An isolated heap proof-checker prototype is independently
testable and cannot replace this root result. Future compiler working-set
recovery or new-source integration requires a separately declared scope;
there is no third trial here. Historical global/optimized/CI failures remain
historical. Conductor/checkpoint access is waived and no credentials are
requested. The separate timing-document choice remains unanswered.
Original15$\times$ remains active and unachieved. [pcc:unverified]
\end{document}
""".replace("ROWS", rows)
replacements = {"own14GiB":"own 14GiB", "exit 143":"exit 143", "stays 0":"stays 0",
 "new36original":"new 36 original", "cases,108matched":"cases, 108 matched",
 "pairs,1116mutation":"pairs, 1116 mutation", "assertions,6unsupported":"assertions, 6 unsupported",
 "refusals\nand144separate":"refusals\nand 144 separate", "audits/7canaries":"audits/7 canaries",
 "Source3ddaa65f":"Source 3ddaa65f", "opt3configuration":"opt3 configuration",
 "codegen256":"codegen256", "opt0,256units":"opt0, 256 units", "with2second":"with 2 second",
 "layout2":"layout 2", "codegen256outputs":"codegen256 outputs", "Original15":"Original 15"}
for a,b in replacements.items():tex=tex.replace(a,b)
(s / "report.tex").write_text(tex)
subprocess.run(["xelatex","-interaction=nonstopmode","-halt-on-error","-output-directory",str(s),str(s/"report.tex")],cwd=s,check=True,stdout=(s/"raw/report-build.stdout").open("x"),stderr=(s/"raw/report-build.stderr").open("x"))

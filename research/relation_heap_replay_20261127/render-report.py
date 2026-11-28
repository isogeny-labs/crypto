"""Render Rust-computed proof replay results, without computing statistics."""
from pathlib import Path
import json
import subprocess
s = Path(__file__).resolve().parent
v = json.loads((s / "OUTCOME.json").read_text())
tex = r"""\documentclass[10pt,a4paper]{article}
\usepackage[margin=18mm]{geometry}\usepackage{booktabs,hyperref}\pagestyle{empty}
\begin{document}
\noindent{\Large\bfseries Heap proof replay: exact equivalence}\\[4pt]
{\small Isolated Rust prototype, 27 November 2026. Draft PR \#5251. No timing.}
\paragraph{Recorded result.} All20unit functions pass:14inherited tests and6new
heap/order/cancellation/budget/DAG tests. All144frozen proof records pass the
original production checker, the copied original API and the opt-in heap API:
432complete positive replays, with identical basis and every Stats field,
also matching native bases. Seven mutations per record give1008mutation cases
and3024exact-diagnostic refusals. Input, proof and basis bytes stay unchanged.
No current merged-root or full-call speedup is established. [pcc:unverified]
\begin{center}\begin{tabular}{lr}\toprule
Identity accumulator observation & Recorded total\\\midrule
Original tree-map updates & OLD\\
Heap product pushes (all original products retained) & OLD\\
Heap final nonzero result-map entries & FINAL\\\bottomrule
\end{tabular}\end{center}
\noindent The heap also pops each product and stores iterator heads. Parsing,
other maps, construction, native checks, proof writes and transport remain
costs. These structural counts are neither a timing ratio nor frozen cost-model
credit. The144records cover six existing systems in repeated wire/kernel
encodings; they are not144independent instances or fresh timing samples.
Original15$\times$ remains unachieved. [pcc:unverified]
\paragraph{Mechanism.} Every generator is ordered by packed grevlex. A fixed
checked shift adds the same degree and packed bits without carry or overflow,
preserving order. A heap merges one iterator head per cofactor. Equal monomials
are summed exactly, then final nonzero coefficients enter the result map.
Every scalar product is still computed. The old consumer/API remains as a
control; no producer arithmetic/data structure is imported into the checker.
\paragraph{Unchanged obligations.} All byte, schema, shape, term, coefficient,
reference, duplicate, monomial, work and coverage budgets remain. Every acyclic
node identity, original-input reduction, monic basis and S-pair is checked.
The negative roster mutates basis/cofactor coefficients, future references,
pair coverage, duplicate cofactors and original constants. It also supplies an
algebraically equal S-pair representation whose leading product attains the
LCM: all three paths reject it as nonstandard. Added units cover degree 64,
dimension 16, exact cancellation, both fields and512-node DAG boundaries.
[pcc:unverified]
\paragraph{Source and resource scope.} Fixed $p=2147483647,9007199254740881$,
$n=3,4,6$, seed7LAST-star;36original plus108paired proof records from the prior
guarded-scan native source. No new points/seeds or basis construction. The
candidate is an isolated checker copy; production source/default APIs remain
unchanged. Corpus paths use exact current-root registry entries including
serde\_json 1.0.151 for all three paths. Unit checks have their separately pinned
inherited lock, serde\_json 1.0.154. No cross-context performance claim.
CPU0/thread1, opt0, locked offline builds, AS2GiB/file128MiB/600second bounds
per stage and zero cloud compute. Incidental compiler/test durations are not
application observations. [pcc:unverified]
\paragraph{Limits and next step.} Both separate fresh merged-root compiler
layouts hit their declared14GiB memory refusal; native root qualification
remains uncomputed. This component study cannot replace it or the original
full-call comparison against strongest existing controls. KB service status 127
and bounded actual-source fallback are retained; no novelty claim. Conductor
and checkpoint access are waived. Timing-document choice remains pending.
Future production integration needs complete source/native/failure qualification
and prospective repeated full-call/A-A/strongest-control evidence. No new PCC
ID, ledger promotion, scalar recovery or intrinsic-regularity claim.
[pcc:unverified]
\end{document}
""".replace("OLD",str(v["old_identity_map_updates"])).replace("FINAL",str(v["heap_final_map_entries"]))
for a,b in {"All20unit":"All 20 unit", "pass:14inherited":"pass: 14 inherited", "and6new":"and 6 new",
 "All144frozen":"All 144 frozen", "432complete":"432 complete", "give1008mutation":"give 1008 mutation",
 "and3024exact":"and 3024 exact", "The144records":"The 144 records", "not144independent":"not 144 independent",
 "Original15":"Original 15", "and512-node":"and 512-node", "plus108paired":"plus 108 paired",
 "seed7LAST-star;36original":"seed7 LAST-star; 36 original", "declared14GiB":"declared 14GiB"}.items():tex=tex.replace(a,b)
(s / "report.tex").write_text(tex)
subprocess.run(["xelatex","-interaction=nonstopmode","-halt-on-error","-output-directory",str(s),str(s/"report.tex")],cwd=s,check=True,stdout=(s/"raw/report-build.stdout").open("x"),stderr=(s/"raw/report-build.stderr").open("x"))

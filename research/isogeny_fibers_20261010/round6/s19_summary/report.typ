#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set table(stroke: 0.4pt + luma(180), inset: 4pt)

= §19 of the isogeny-fibers note: rounds 5 and 6, verdicts and corrections

*Date* 2026-10-24, status updated 2026-10-31 (branch `research/note-s19-r6-2-correction`, base `origin/main` `9a026c1459`). *Type* documentation of existing results; no experiment was run for this report and no number below is newly computed unless marked _derived_. The first revision (`research/note-s19-rounds-5-6`, base `5cb553810e`) was written while five pull requests were open; the 2026-10-31 update relabels them and records the R6.2 correction. Toy and synthetic instances only; no sub-rho claim and no claim about a deployed curve.

*Question.* What did the fifteen leads of rounds 5 and 6 (R5.1–R5.4, R6.1–R6.11) find, against their preregistered hypotheses and falsifiers, and which earlier statements of `RESEARCH_ISOGENY_FIBERS_ECDLP.md` must change in light of them and of the R6.10 literature check?

*Method.* Read each lead's `RESULTS.md`; on 2026-10-24 five were read from open pull-request branches (R5.3 \#2192, R6.1 \#2511, R6.4 \#2240, R6.8 \#2196, R6.9 \#2502; heads `e8e0bf1296`, `ff6fd1dc6c`, `325c8345b8`, `7d6d50d268`, `6ac9a3877e`) and R6.2 from its run report and raw records. On 2026-10-31 the pull-request records show all five merged (R6.4, R6.9, R6.1 on 10-26; R5.3 on 10-29; R6.8 on 10-30), and each was compared with its merged `RESULTS.md` on `main`: verdicts and numbers are unchanged. R6.2's correction \#2543 (merged 10-25) added its `RESULTS.md`, which is now the source. R7.1 \#2542 and \#2524 are recorded in section 19.19 of the note. Verdicts and headline numbers are transcribed; intervals are the sources'.

The verdict figure (Figure 1) is on the last page (landscape).

== Findings, including negative and unresolved ones

- *Falsifier fired.* R6.5: the A-pair correction does not explain the recorded single-base value 1.02 (mean residual −0.030 before, −0.085 after correction); the correction itself is confirmed in a 200-base ensemble (1.130, 1.059, 1.033 at $|F| = 36, 72, 144$). R5.3: "exponent drops toward 0.5–0.6" refuted, 0.798 [0.775, 0.820]. R6.4: ascending against descending differs for the $psi$ route (0.501 [0.457, 0.548]) and the $Phi$ route (0.360 [0.347, 0.373]). R6.9: relation rate off the $1\/g!$ heuristic by more than 10× (the source says 6 of 34 runs; its own table has 7).
- *Not fired.* R5.1, R5.4, R6.3, R6.6, R6.7 (read against the PROGRAM falsifier), R6.11, R6.1 and the R6.8 census.
- *Open.* R5.2 as a cost statement; R6.2 (no paired interval, batch half not re-run; the two defects, planner applicability and an asymmetric-edge path search, were corrected in \#2543, merged: 360 of 360 corrected routes validate forward, the pre-fix search returned a non-executable route for 19 of 180 fresh and 4 of 180 replay pairs (lower bounds), and the cost per pair still tracks component size, so the O(√(class size)) hypothesis is still not shown; verdict stays open); R6.8's exact-simulation theorem (proof claim withdrawn in the pull request); the generic-kernel exponent of R6.4 (1.919 [1.724, 2.115], insufficient evidence either way, restricted cost model).
- *Prior work.* R6.10: the kernel-saturation penalty (the $mu^(m-2)$ clause of Prop. 15.2) is anticipated by Faugère–Huot–Joux–Renault–Vitse, EUROCRYPT 2014, §3.2.
- *Source defect found while reading.* R6.7's merged `RESULTS.md` says 12 of 18 per-curve intervals contain 1; its records give 11, 9 and 11 of 18 (unpriced, priced as run, priced essential). The note uses the record counts. The merged PDF was not regenerated here (frozen evidence).

== What changed in the note (research note, not a result)

2026-10-31 update: R6.2 relabelled from "pending correction" to corrected in \#2543 (merged), verdict open; R5.3, R6.1, R6.4, R6.8, R6.9 relabelled merged; the R5.2 `audit_curve` sentence annotated with \#2524; R7.1 (\#2542) added as section 19.19; sections 19.16 and 19.17 updated. First revision:

New §19 (one subsection per lead, synthesis table, settled/open list); header status line; §0 items 3, 5, 8, 9; §6 H6 and §13.4 genus-2 attribution; FHJRV citation in §14.1, §15.2, §17.4, §17.7; §15.2 erratum from R6.5; §15.3 xedni and Koshelev wording; §17.2 patent sentence; §17.7 walk qualifier; §18 replaced by the R6.10 list. Measured values in earlier sections are unchanged.

== Graphs checked and left unchanged

`docs/index-calculus-scoreboard.html`, `docs/ic/progress-timeline.json`, `docs/performance-gains/`, `docs/ic/BOUNDARY_TARGETS.md` and `figures/` carried no isogeny-fibers round 5–6 row when searched on 2026-10-24 (only generated yield-record paths in `docs/browser/data.json`); that search was not repeated on 2026-10-31. On `main` the merged pull requests have since added browsing bundles `benchmarks/20261018-r6-pooled-rho/` (refreshed by \#2543 with the paths-v2 graphs and a PDF), `benchmarks/20261019-r5-3-ntt-sqrt-velu/` and `benchmarks/20261019-r6-4-vertical-margin-2/`; this documentation-only change edits none of them. No scoreboard or ledger value changes; the study-local graphs of each lead are unchanged and are linked from §19.

== Limits and unresolved checks

Verdict classes compress mixed outcomes into one column; the wording is in §19.16. Merge status is as read on 2026-10-31. The R6.8 proof gap remains open, and the validation holds recorded in the descriptions of \#2192, \#2240 and \#2196 before they merged were not re-checked here. `conductor check` was not available in this environment (no `conductor` executable), so no task was reserved; edits are confined to the note and this directory. The curve-name linter could not be run against the sparse checkout; no retired curve name is used, and the only curve slug in §19 is one already present in the note.

== Commands

`venv/bin/python plot.py` (reads `verdicts.json`, writes `verdicts.svg` and `verdicts.png`); `typst.compile('report.typ')` for this PDF.

#page(flipped: true, margin: 1.5cm)[
#figure(image("verdicts.svg", width: 100%), caption: [Verdict class of each lead (transcribed; data `verdicts.json`, script `plot.py`). Filled markers are merged (status read 2026-10-31); R5.3, R6.1, R6.4, R6.8 and R6.9 were hollow (unmerged) on 2026-10-24. R6.2 carries the merged correction \#2543 and its verdict stays open.])
]


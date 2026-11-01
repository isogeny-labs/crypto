#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set table(stroke: 0.4pt + luma(180), inset: 4pt)

= §20 of the isogeny-fibers note: round 7, verdicts and corrections

*Date* 2026-11-01 (branch `research/note-s20-round-7`; the revised §19 from \#3072 is on `main`). *Type* documentation of existing results; no experiment was run for this report and no number below is newly computed unless marked _derived_. Toy and synthetic instances only; no sub-rho claim and no claim about a deployed curve.

*Question.* What did the eight round-7 leads (R7.1–R7.8) find against their hypotheses and falsifiers, and which statements of §19 must change in light of them?

*Method.* Read each lead's `RESULTS.md` from the branch of its open pull request (R7.5: the two `ERRATUM_v2.md` files; R7.8: the pull-request description, because it has no `RESULTS.md`), checked the branch heads against the pull-request records (all eight match), and transcribed hypothesis, falsifier, verdict and numbers with the sources' intervals. Four of the eight pull requests merged on 2026-11-01 (R7.1 \#3107, R7.3 \#3105, R7.5 \#3109, R7.7 \#3113); the other four are *open and unmerged* on 2026-11-01: R7.2 \#3124, R7.4 \#3092, R7.6 \#3128, R7.8 \#3125. The §19 update \#3072 merged on 2026-11-01; \#3074 remains open. One check was re-run for this report: both `recount_v2.py` scripts of R7.5 on the committed records of `main`; their output equals the committed `recount_v2.txt`.

The verdict figure (Figure 1) is on the last page (landscape).

== Findings, including negative and unresolved ones

- *Falsifier fired / refuted.* R7.2: no toy instance is weak; index calculus costs 1.91 [1.72, 2.11] to 131 [123, 141] times the best generic method on 30 instances (N = 12 to 28), every interval above 1. R7.4: the binary-Edwards pair-table falsifier as worded fires at n ≤ 12 (39 of 40 cells exclude 1) and no gain is claimed. R7.6: the path-finding exponent is 1.13 [1.06, 1.21], so $O(sqrt(C))$ is refuted for breadth-first bidirectional search. R7.7: the a = 1 premise is false ($E\/4E tilde.equiv ZZ\/2$).
- *Mixed.* R7.1: the ordering falsifier fires in one accounting (cold, ascending: ℓ = 89, 109, 131, near ties of 1.6%, 0.3%, 0.4%), the exponent falsifier does not; the exponent stays 1.919, insufficient to show a value below 2. R7.3: null in iterations and yield after the $|"Aut"|$ baseline; one inferior algorithm costs 1.46× [1.44, 1.48] more on the floor in multiplications. R7.6: pooling gain exponent 0.43 to 0.52 against ideal 0.489, but below $sqrt(C)$ by more than its interval from C = 32. R7.7: the 4E criterion and odd-coset labels hold for every (a, b) with Tr(a) = 0, constancy of the second bit is measured not proved, and the constant-time bar is still missed there.
- *Not evaluated or unread.* King §5 and the Eagle–Galbraith–Ong cofactor-4 test: access blocked, still unread (R7.4). No Gröbner/F4 oracle on Edwards summation polynomials. The extrapolated crossing at N ≈ 32 (R7.2) was not built.
- *Corrections.* R6.7: 11 / 9 / 11 of 18 intervals contain 1 (not 12), and the m = 3 pricing shift is 0.0151 (not 0.011); R6.9: 7 of 34 runs are more than 10× from 1/g! (not 6). No verdict changes (R7.5).
- *No result.* R7.8: `chained-x` registered in `ic bench` and an end-to-end test of ecbench verdict procedure 2; the `chained-x` numbers are not bit-identical to R6.1's because the base differs.

== What changed in the note (research note, not a result)

New §20 (one subsection per lead, synthesis table, settled and open lists, next leads, edit list). Edits to earlier sections: header and §0 item 10; §19 status paragraph; §19.5, §19.11 (the 0.011 sentence and the unscoped inversion share), §19.13, §19.16 rows, §19.17 open lines (each now names its round-7 result and its pull-request status), §19.18, §19.19. The "awaits correction" lines were kept in substance because the correcting pull requests for R7.2, R7.4, R7.6 and R7.8 are open; the R7.5 errata (\#3109) and \#3072 merged. §17.3 and all frozen results are untouched; measured values of earlier sections are unchanged.

== Graphs checked and left unchanged

`round6/s19_summary/` covers rounds 5 and 6 and is left as it is (its content is the other half of the verdict matrix); each round-7 lead's own graphs are in its pull request and are linked from §20. The scoreboard, ledger and `benchmarks/` graphs were not searched on 2026-11-01 (the 2026-10-24 search of §19 was not repeated); this change edits none of them and no ledger or scoreboard value changes. No PNG render is committed (the repository's artifact policy sends a PNG over 64 KiB to a store this session cannot write).

== Limits and unresolved checks

Verdict classes compress mixed outcomes into one column; the wording is in §20.9. Merge status is as read on 2026-11-01: R7.1, R7.3, R7.5 and R7.7 merged; the rest may change. CI state of the round-7 pull requests was not read. `conductor` is not installed in this environment, so no task was reserved; edits are confined to the note and this directory. The curve-name linter was not run (docs only; the note names no curve slug that §19 did not already carry).

== Commands

`venv/bin/python plot.py` (reads `verdicts.json`, writes `verdicts.svg`); `typst.compile('report.typ')` for this PDF.

#page(flipped: true, margin: 1.5cm)[
#figure(image("verdicts.svg", width: 100%), caption: [Verdict class of each round-7 lead (transcribed; data `verdicts.json`, script `plot.py`). Hollow markers: pull request open and unmerged on 2026-11-01; filled: merged (R7.1, R7.3, R7.5, R7.7). R7.1: ordering falsifier fires in one accounting, exponent falsifier does not. R7.8 changes no result.])
]

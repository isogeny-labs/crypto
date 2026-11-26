---
title: "H116 formal-scalar out-of-sample run report"
date: "2026-11-18"
geometry: margin=0.72in
fontsize: 10pt
---

# Outcome

The unchanged eleven-coefficient formal-scalar candidate matched full
degree-197, 257, 397 and 547 Vélu maps on four deterministically selected
different-j curve models. One precommitted p=107 cell failed selection after
all 11,449 models, so the frozen all-five prediction is false even though every
constructed cell passed. [pcc:unverified]

| p | r | selected model | candidate/full-map result |
| ---: | ---: | --- | --- |
| 107 | 127 | none | retained selection failure |
| 211 | 197 | `(A,B,j)=(2,24,98)` | coefficients 0--10 equal |
| 241 | 257 | `(A,B,j)=(2,4,61)` | coefficients 0--10 equal |
| 431 | 397 | `(A,B,j)=(1,225,126)` | coefficients 0--10 equal |
| 587 | 547 | `(A,B,j)=(1,342,424)` | coefficients 0--10 equal |

The independent verifier repeated selection, quotient division, the recurrence,
the transformed first-order identity, the full cleared map identity, codomain
scaling and map mutations on retained JSON. It returned four matches in each
completed category and rejected a separate candidate-coefficient mutation.
[pcc:unverified]

![Recorded operation counts for the four completed cells. Candidate and
full-map oracle have different proof obligations, so the vertical separation
is not a speed ratio.](operations.png){width=92%}

The graph source is `plot_operations.py`; exact plotted values are in
`operations.csv`; `operations.svg` is the vector render.

# Protocol and execution

| Field | Value |
| --- | --- |
| protocol | `H116-FORMAL-SCALAR-OOS-001` |
| candidate SHA-256 prefix | `914c1909ffdd5e6b` |
| oracle v1 disposition | p=107 selection failure retained; no map built |
| oracle v2 source SHA-256 prefix | `f75b7073f7b0d93f` |
| comparison source SHA-256 prefix | `a84761b550a4118c` |
| verifier SHA-256 prefix | `8823c6af2e9f1af4` |
| requested/completed maps | 5 / 4; one selection failure |
| timing status | no timing or speed claim |

The protocol, v1 oracle, first failure, v2 resume source, comparison source and
independent verifier were committed in that order. The v2 source did not rerun
or replace the failed cell; it changed only early-termination behavior so the
remaining frozen cells were retained.

# Exact construction counts

| r | candidate add | candidate mul | candidate inv | oracle base add | oracle base mul |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 197 | 64,426 | 67,584 | 16 | 3,560,372 | 3,130,760 |
| 257 | 64,426 | 67,584 | 16 | 5,946,452 | 5,216,360 |
| 397 | 64,426 | 67,584 | 16 | 7,799,826 | 6,495,417 |
| 547 | 64,426 | 67,584 | 16 | 14,693,223 | 12,219,039 |

These are deterministic counts from different implementations and stages.
They show fixed work at the frozen candidate precision and growing work for the
full-map oracle, but do not establish an end-to-end advantage because generic
semantic certification is still missing. [pcc:unverified]

# Evidence boundary

The same candidate now matches five full maps including the earlier p=31
fixture, over degrees 37 through 547. Finite matches do not replace the generic
elliptic bridge. The remaining proof must instantiate `V+pi=[t]`, exact local
Frobenius, formal subtraction, and the one-degree loss from parameter agreement
modulo `z^p` to normalized x-ratio agreement modulo `z^(p-1)`.
[pcc:unverified]

Retained files are `../../FORMAL_SCALAR_OOS_ORACLE.json`,
`../../FORMAL_SCALAR_OOS_RESULT.json`, the protocol and three source-gate notes.
The full experiment narrative and hashes are in
`../../FORMAL_SCALAR_OOS_EXPERIMENT.md`.

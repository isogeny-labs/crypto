# Registry optimization diagnostics

Status: replay-verified L0 development evidence; not a leaderboard result.

## Coverage and integrity

- 35 complete admitted sessions, 1,005 records.
- 952 verified outcomes, 20 exhausted, 23 timed out, and 10 explicit errors.
- All 35 admitted receipts passed; 782/782 eligible deterministic measured replays
  reproduced exactly; zero audit problems.
- 72/72 runnable native top-level methods executed. The six unexecuted registry
  entries are external CM wrappers awaiting a real backend and digest.
- 11/11 system solvers, 9/9 PDP3 engines, and 4/4 PKM tower engines executed.
- The campaign-structure audit separately admits 111 specs with 78/78 methods
  and 24/24 nested algorithms.
- A post-merge audit with current `main` reproduced the same 742/742 eligible
  runs across all 33 admitted sessions with zero problems. Its binary-bound
  receipts and summary are in [`current-head-replay/`](current-head-replay/).
- The new PKM panel passed 24/24 measured replays. The first PDP3 pair-index
  session exposed cross-record cache charging (13/16 replays); it is retained
  as a failed negative control. After resetting the cache once per independent
  pipeline invocation, the corrected session passed 16/16.
- Those three post-merge raw JSONL payloads are local pending artifact-store
  delivery; tracked receipts bind their hashes and results. They are not a
  remotely hydrated evidence bundle and cannot support promotion.

Every timing here is L0 and descriptive. The completion and nested panels use
one target, one warm-up, and four measured rounds. Deterministic work can select
a holdout candidate, but one target cannot establish a curve-family frontier.

## Current diagnostic boundary

| lane | candidate/reference work ratio | result at this boundary |
|:--|--:|:--|
| Koblitz n=29 full-price generator-orbit pipeline / signed-Frobenius rho | 0.4405, 95% within-target interval [0.363, 0.534] | selected for holdout; wins at pricing weights 0, 0.1, and 1, but only one L0 development target |
| Koblitz n=13 pair-index F6-IC / inherited F6-IC | 0.2792 [0.246, 0.317] | same solver count; pair-index closure cuts charged group work and wins at all recorded weights, but L0 method wall is 0.922 [0.828, 1.027] |
| Koblitz n=13 pair-index F6-IC / strong rho | 139.3034 [85.480, 227.016] | rho leads at every pricing weight |
| prime p9 PKM signature F5 / negation rho | 4.6179 [4.140, 5.151] | rho leads at every pricing weight; F5 uses 0.2253× F4's native multiply-adds, but the unit is unpriced |
| prime p9 PKM F4 / negation rho | 4.6179 [4.140, 5.151] | rho leads at every pricing weight |
| prime p9 PKM F6 / negation rho | 4.7313 [4.235, 5.285] | rho leads at every pricing weight |
| prime p9 PKM enumeration / negation rho | 5.0328 [4.024, 6.295] | rho leads at every pricing weight |
| Koblitz n=17 best matched system-solver lane (FES-wide) / strong rho | 4.2817 [3.580, 5.121] | rho leads at every pricing weight |
| Koblitz n=13 rootfind-negfold / strong rho | 7.6894 [5.604, 10.551] | rho leads at every pricing weight |
| Koblitz n=13 rootfind / strong rho | 16.2245 [13.424, 19.609] | rho leads at every pricing weight |
| Koblitz n=13 window-negfold / signed word-RAM rho | 16.2069 [15.789, 16.636] | rho leads at every pricing weight |
| Koblitz n=13 window rootfind / signed rho | 19.6895 [19.282, 20.106] | rho leads at every pricing weight |
| random-binary n=13 matrix F5 / negation rho | 5.1750 [3.393, 7.893] | rho leads at every pricing weight |
| random-binary n=13 matrix F4 / negation rho | 5.8916 [5.194, 6.682] | incomplete lower bound; one measured timeout; rho still leads at every weight |
| Koblitz n=13 inherited F4 / strong rho | 209.8054 [193.206, 227.831] | rho leads at every pricing weight |
| Koblitz n=13 F6-IC / strong rho | 522.5885 [472.687, 577.758] | rho leads at every pricing weight |

The Koblitz n=29 result is the only new candidate selection. Its comparison is
[`smoke-completion-generator-orbit-a1-n29-t0/comparison-rho-frob-full__ic-gen-layout-c1.json`](smoke-completion-generator-orbit-a1-n29-t0/comparison-rho-frob-full__ic-gen-layout-c1.json).
The matched solver comparisons and raw records are under
[`smoke-nested-solver-backends-k17-matched/`](smoke-nested-solver-backends-k17-matched/).
The post-merge receipts and comparisons are under
[`post-merge-pdp3-pair-fixed/`](post-merge-pdp3-pair-fixed/) and
[`post-merge-pkm-tower/`](post-merge-pkm-tower/).

## Matched n=17 system-solver panel

All backends below used the same curve, public target, factor base, relation
stream, four measured rounds, and 17-second cap. Cross-backend native counters
are not pooled; the table uses complete enclosing-pipeline `S` and status.

| backend | measured outcome | mean S | ratio to strong rho |
|:--|:--|--:|--:|
| FES-wide / Gray-code | 4 verified | 3.3982 (lower bound) | 4.2817 |
| Crossbred | 4 verified | 3.4021 (lower bound) | 4.2867 |
| exhaustive | 4 verified | 3.4177 (lower bound) | 4.3063 |
| SAT-CDCL | 4 verified | 3.4596 (lower bound) | 4.3590 |
| inherited F4 | 4 verified | 74.4425 | 93.7969 |
| FES | 4 verified | 96.5978 | 121.7124 |
| matrix F5 | 4 verified | 1126.5628 | 1419.4593 |
| matrix F4 | 4 verified | 1126.5628 | 1419.4593 |
| F4-F2 | 4 verified | 12291.2094 | 15486.8166 |
| Buchberger-F2 | 4 timeouts | — | incomplete |
| XL-F2 | 4 explicit refusals | — | empty |

Strong rho's mean `S` was 0.7937 in the same panel. The lower-bound solver
rows remain worse than rho at every recorded pricing-band weight. XL correctly
refused the 18-unknown/17-equation system; Buchberger hit the common cap.

## Other retained bottlenecks

- Both dense and Wiedemann F_((2^10)^4) pipeline arms timed out in all five
  attempts at 17 seconds; both rho controls completed.
- The random-binary PDP3 matrix-F4 arm timed out twice; matrix F5 completed.
  Their direct F5/F4 ordering is unresolved because of censoring and pricing.
- F6 versus inherited F4 is unresolved across the pricing band: a solver-local
  count advantage does not survive every end-to-end price.
- WDSAT is a historical external path, not one of the 11 live native
  `SystemSolver` backends. No hash-bound WDSAT binary was available here, and
  the retained false-UNSAT regression described in the comprehensive report
  still blocks promotion; it is recorded as unmeasured, not folded into SAT.
- APS exact, k4 Gaudry, and the tiny Jacobian-image strong-rho controls retain
  20 total exhausted outcomes rather than being converted into losses or
  discarded.
- A deliberately retained pre-fix three-leads session has 10 runtime errors
  from pairing decomposition methods with a Scholten fixture. Planning now
  rejects that mismatch before execution; corrected Scholten and subfield
  sessions verify and replay exactly.

## Rho engineering diagnostics

- F_((2^10)^4) strong negation rho used 0.5303x the legacy work point estimate,
  but the one-target interval [0.087, 3.215] and incomplete pricing do not admit
  a gain.
- Cubic-v2 / legacy cubic was 1.0063 [0.938, 1.080]: no observed work gain.
- Quartic-v2 / legacy quartic was 0.8499 [0.501, 1.441]: unresolved.
- Strong-escape / strong-v2 on binary n=11 was 5.9510 [5.597, 6.327], worse at
  every weight on this tiny workload.
- Prime p20 negation BSGS / negation rho was 0.6908 [0.379, 1.257], unresolved
  and incompletely priced.

No rho implementation or competing method is promoted from these diagnostics.
The next admissible step is a disjoint multi-target L2 panel with exact A/A
controls and full symmetric pricing.

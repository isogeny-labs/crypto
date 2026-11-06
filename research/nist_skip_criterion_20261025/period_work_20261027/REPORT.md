# Recorded ECPP work does not explain every skipped curve

27 October 2026. All 21 saved order-proof transcripts were analyzed natively,
retaining ten published and eleven omitted choices. All 210 predefined work
counts are known. None of the 40 declared single-feature cutoff comparisons
separates the choices. Even field-specific combined upper limits retain two
omissions: p239 stream indices 1562 and 2682. **The historical selection
criterion remains unresolved.** No vulnerability or measured DLP is established.

![Ten declared upper work envelopes](reports/work-limits.png)

Each bar is the number of omissions still accepted by the tightest per-field
upper bound on that feature which keeps every published control. These are
exact finite counts from the [native comparison](results/summary.json), with
visible labels and no statistical interval or held-out prediction. Counts and
stream indices are decimal. The [complete table](results/profile.tsv) retains
all 21 rows; [exact p,b,n,seed identities](inputs/historical.tsv) preserve the
original curve boundary. This order-only analysis introduces no curve aliases.

## A stronger, checked counterexample

Both omitted p239 orders have no larger value in any of the ten recorded
quantities than published prime239v1 (index 2185) and prime239v3 (index 3048).
The [separate token-scanning verifier](verify.cpp) re-extracted every count
from original text and found all four dominance pairs. All 40 recomputed
envelopes agree with the first implementation. Three altered-table controls
are refused for the intended wrong-count, missing-row and changed-status reasons.

![Exact recorded count vectors](reports/dominance-table.png)

The table shows literal native-checked values, not normalized scores. For a
cost C which is nondecreasing in these ten quantities, u≤v componentwise
implies C(u)≤C(v). If the published vector v fits a budget B, transitivity gives
C(u)≤C(v)≤B for the omitted vector u. Therefore an upper-budget screen on
these counts cannot reject either omission while accepting prime239v1. This
also constrains nonlinear monotone costs, beyond the single cutoffs or their
conjunctions. It says nothing about additional unrecorded work or a different
acceptance predicate.

The [Lean source](lean/WorkLimits.lean), [literal data bridge](lean/count-data.json)
and [proof map](proof/PROOF_MAP.json) make that conditional argument explicit.
Lean 4.34.1 checks four exact vector inequalities, natural-valued and general
transitive-cost implications, and the two-omission corollaries. The unchanged
repository [axiom auditor](proof/axiom-audit.txt) accepts eight theorems with
empty transitive axiom lists: zero failures and zero trusted native evaluations.
The data constants are bound to the independently checked table. Monotonicity
and transitivity are model assumptions, not asserted properties of historical
runtime. This is not a formalization of curve security or primality certificates.

## What ran and what the quantities mean

The source-bound [protocol](PROTOCOL.md) and [custom saved-analysis plan](plan.json)
preceded complete-cohort extraction. The non-freeze plan hash is
85a646f6d5e87f90b2b7e96bf7c327fd4969ab2615de0805d7acc08729ce2d77.
All labels and one example transcript's format were already visible. This is
an analysis of the fixed census, not blinded preregistration. The additional
dominance diagnostic and [independent validation protocol](VALIDATION_PROTOCOL.md)
were declared after the primary output; original source, plan and results
remain unchanged.

One strict C++17/O2/Wall/Wextra/Werror invocation processed all 21 original
replay004 packets under source b1f58e9d8e08bb3a9e1794166cd69e7556bbffa3.
Saved verification under 27018452d455be75b27167812cb0d8f11a2876d4 uses a separate
line/token implementation instead of the first parser's regular expressions.
Both use a 60-second watchdog and 768 MiB address-space ceiling. The frozen
source/input and binary checks pass before and after execution. All original
ECPP and FreeLIP programs ran zero additional times. No wall/CPU/RSS clock
measurement is admitted.

The ten quantities are root, peak and total catalog-record scans; root and
total returned forms; build-chain levels; elliptic-route levels; printed
factor records; printed factoring-parameter records; and forced retries.
They concern proving the integer group order n prime. The original curve's
point-counting workload and ECDLP cost are outside this vector. Printed factor
records include repeated prints and are not counts of unique prime factors.

The [original source audit](sources/source-audit.json) matters: findD silently
skips catalog records failing genus/norm tests, so next-D messages alone
undercount catalog scans. In this rank-zero, serial configuration, the last
returned discriminant's unique catalog rank gives records consumed at each
depth when no retry/exhaustion occurs. All 21 traces satisfy those conditions;
the independent parser checks increasing catalog ranks. Explicit N_i/D[[i]]
pairs count the terminal build level, whose timing line can be omitted. The
subsequent Pratt record is separate. This is recovered V3.4.1 source, not an
authenticated original curve-generation caller.

| Declared comparison | Observed result |
|---|---|
| Forty single-feature envelopes | Zero complete separators |
| All ten global upper bounds jointly | All 11 omissions retained |
| All ten per-field upper bounds jointly | Two omissions retained: p239-1562, p239-2682 |
| Same-field pointwise dominance | Four pairs, using published indices 2185 and 3048 |
| Historical attribution | Not established |

No unknown row was dropped or encoded numerically. The old ECPP certificate
admissions remain 3 published and 6 omitted accepted, with 7 published and
5 omitted indeterminate. All twelve unknowns stay text labels and enter no
work-count predicate. Their mathematical group-order primality had already
been checked separately in the earlier study. A complete work profile does
not turn a failed legacy theorem transcript into a valid certificate.

## Source review and preserved failures

The renewed review found a [24 October lab prefix reconstruction](sources/prior-freelip-prefix.md),
retrieved at the immutable commit in [upstream custody](sources/upstream.json).
It already reports 24 matching FreeLIP call/argument/curve paths, six matching
manual-seven baselines and 24 nonmatching controls. Its portable-build and
Python-verifier provenance is retained. No such Python code was executed here.
The seven-call compatibility is prior reported work; historical invocation
remains unproved. The corresponding [period-NTL source audit](sources/prior-ntl-source.md)
describes a different default path. Neither modular RNG automatically shifts
the separate lrand48 seed stream; coupling needs a caller.

The primary [Anthropic report](https://www.anthropic.com/research/ecdsa-curve-origins),
[Valsorda enumeration](https://words.filippo.io/seeds-solved/) and
[Mattsson TLS message](https://www.mail-archive.com/tls%40ietf.org/msg25031.html)
still leave the omissions unexplained. The [retrieved problem record](sources/prior-selection-problem.md)
also reports broader 69/99-position draft-based cohorts. Those remain distinct
from these 21 fixed records; no untested broader rule is certified here.
An unpublished eligible seed position is not, by itself, proof of an original
program visit followed by rejection. The original caller and point-count inputs
are still missing from the retrieved evidence.

[Proof development records](proof/attempts.json) preserve two thread-startup
failures under a 4 GiB virtual-address cap, a decidability elaboration failure,
an auditor memory failure and a PATH failure. Unfolding the unchanged predicate
resolved decidability; no statement, vector or axiom was weakened. Final proof
checking uses one worker and internal 1 GiB/4 GiB allocation limits for build
and audit, without a virtual-address cap or measured RSS. The exact official
release bytes match the published SHA256. lean4checker was unavailable. See
the [proof reproduction guide](proof/REPRODUCE.md) and descriptive run reports.

An intervening storage migration broke nine earlier file bindings. Exact
original paths, metadata and PDF bytes were restored; all 97 bindings pass.
The [custody record](evidence/custody-restoration.json) preserves both observations
without attributing the change to an unverified actor. Scientific bytes remain
unchanged.

## Scope and evidence custody

| User requirement | Status |
|---|---|
| Four outstanding factorizations and full comparison | Preserved complete in the earlier factor follow-up; all 21 controls retained |
| Fixed ECPP work-limit explanation | Excluded for the declared limits and monotone costs of these ten counters |
| Historical reason for every skip | Unresolved; no supported complete criterion found |
| Selected/skipped weakness hypotheses | Neither established; no applicable attack or measured DLP |

The preparation archive was fully downloaded and all 27 members checked before
the primary invocation; the validator binary also had a full remote byte check
before saved validation. The final archive has 144 verified member files and
includes exact copies of all 65 consumed original packet/catalog files. Every
copy matches its source. Full GET of the raw archive, export and external
manifest matches, and internal/external manifests are byte equal. Raw files
were ignored only after this closure. [Receipts](evidence/final-delivery.json),
[unified export](export.json), [run reports](reports/INDEX.md) and the
[reproduction guide](REPRODUCE.md) retain the commands, limits and failed attempts.
The export uses one primary analysis invocation with 21 reused packets; the
verifier/controls are not new historical samples. Export Schema/file admission
ran; the custom plan is not canonical experiment-protocol-checker certification.

Checked the canonical IC scoreboard, progress timeline, relevant curve diagrams
and performance-gain pages. This count-based exclusion supplies no attack,
isogeny, speed or crossover value for those graphs. Study figures and a neutral
native count-cell coverage metric are updated. Hosted merge checks and review
remain separate from the saved-data and proof checks; no hosted pass is claimed.

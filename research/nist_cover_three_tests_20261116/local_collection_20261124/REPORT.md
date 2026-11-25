---
title: "P-192: actual local genus-3 relation collection"
date: "2026-11-24"
geometry: margin=0.7in
fontsize: 10pt
---

The prepared pipeline **actually completed locally** after the user's explicit
instruction. All selected tangent sections pass the target-Jacobian check;
the separate checker accepts their point images, elliptic sums and complete
elimination witnesses. The screen has **zero additional old-base rank**:
every added relation introduces two private new columns. This is a completed
functional run from pinned construction inputs, with producer-unverified
results and no independent signed verdict. [pcc:unverified]

## Recorded outcome

| Quantity | Native result |
|---|---:|
| Original quartic pool / original elliptic columns | 226 / 225 |
| Rechecked tangent selection outcomes | 226 |
| Selected sections / actual target principal checks | 102 / 102 |
| Expanded point images checked / elliptic rows checked | 430 / 108 |
| Expanded elliptic columns / new columns | 429 / 204 |
| Full matrix rank / new-column rank | 108 / 102 |
| Closed old-base rank / original rank | 6 / 6 |
| Additional closed rank | **0** |
| Actual checker corruption controls rejected | 5 |

The primary collector and separate checker exit 0, as do baseline, transfer,
preflight and the outer wrapper. The raw
[collector result](evidence/attempt-v1/pipeline/result.json) and
[checker result](evidence/attempt-v1/pipeline/checker-result.json) are bound by
PCC-20261124-695364. The current host uses one allowed CPU, native C++17/GMP
and OpenSSL 3.5.7. No scalar recovery was needed. [pcc:unverified]

![Exact finite ranks over the P-192 subgroup coefficient field. These are matrix dimensions for one fixed batch, not sampling estimates or performance measurements.](ranks.pdf){width=87%}

The local instruction supersedes the predecessor's EKS/local-execution
restriction for this run. The recorded envelope documents that actual user
authorization; it does not invent an EKS budget reservation or signing key.
Old protocols, drivers and proof subjects remain byte-equal.

\clearpage

## What the run checks

For the fixed P-192 d7/kernel3 quartic and degree6 map f, each selected tangent
has divisor $H_i=2P_i+R_i+S_i$. Relative to the unchanged split reference H,
the line ratio gives $H_i-H$, a principal divisor. The producer reconstructs
each quadratic and both residual points, checks exact quartic incidence,
encodes the section with the original KM evaluation pool and checks its actual
target principal class. Full field coordinates avoid the old integer-x
representation limit. [pcc:unverified]

Every individual image is then mapped and assigned its canonical signed
elliptic column. All rows, including old rows, are retained. The separate
GMP/OpenSSL path rederives selection, residual products, images and rows and
checks complete coefficient-kernel witnesses and both ranks. Changed residual,
selection, coefficient, mapped point and elimination witness are rejected.
Separate arithmetic methods do not constitute independent signed verification.
[pcc:unverified]

The fixed baseline replay again produces seven split fibers and six original
relations. Its fresh transfer panel exercises O,G,-G,2G,Q,-2G, six complete
transfer fixtures and five nonidentity dual-source composition3 checks,
including the old-shift exceptional -2G representation. Separate elliptic
companion checks agree with $BF(P)=[6]P$ on that fixed panel. The transfer
checker explicitly reports `independent_geometric_verification=false`;
this finite replay is not a general independently signed proof of the maps.
PCC-20261124-dbb2dd binds these results. [pcc:unverified]

## Why the new rows give no old-base gain

The instrumented successor emits all 204 new-column incidence certificates.
A separate checker scans the original matrix by rows, sorts the occurrences
by column and matches the complete certificate. It finds two private unit
columns on every added row, and zero new support on the six original rows.
Wrong owner, missing record, changed sign and out-of-domain coefficient all
reject. PCC-20261124-655ae2 binds this check. [pcc:unverified]

![First two added rows and their actual private-column incidences. Edge labels are recorded signed coefficients. The complete certificate contains every new column.](incidence.pdf){width=87%}

For any combination eliminating new columns, a private column forces its
owner's coefficient to vanish modulo the prime subgroup order. Applying this
to every added row leaves only the original row space. The complete-kernel
check agrees: closed rank is $108-102=6$. H-F therefore fails on this fixed
screen. This does not exclude other relation families or other curves.
[pcc:unverified]

The prior four-row lower bound was necessary, not sufficient. Baseline-only
kernel witnesses use zero added rows and are valid; the bound concerns
nonempty added-row support. Here no such support survives at all. The parent
evaluated singleton-step record IDC1h8e78cc552dcce5cc concerns scalar
bookkeeping only, not a signed proof of this geometric construction.
[pcc:unverified]

\clearpage

## Complete local run costs and additional diagnostics

The original wrapper records 119.375 wall seconds, 116.085 user seconds and
3.055 system seconds on one CPU. Its inner pipeline records 119.365 wall
seconds. These are local process diagnostics for one completed call, including
its builds, qualifications and native checks; no benchmark comparison or
speedup is claimed. PCC-20261124-2b81de pins these records. [pcc:unverified]

| Clock scope | Wall seconds |
|---|---:|
| Entire original local wrapper | **119.375** |
| Nested fixed baseline/transfer pipeline | 65.835 |
| Nested actual collector | 31.086 |
| Nested separate collection checker | 0.690 |

The nested entries are included in the outer total and must not be summed
again. All 26 retained process-clock paths, including each native build,
qualification and preflight call, are listed in [COST_INDEX.json](COST_INDEX.json).
The actual caller source, CPU selection, environment, exact input/source/binary
hashes, stdout/stderr and exits accompany the records. [pcc:unverified]

The first collector clock aggregates its internal work. A separately frozen
instrumented copy repeats exactly that computation to expose subphases and
incidence certificates. The entire original mathematical byte prefix through
the closure object matches attempt-v1; the unchanged separate checker accepts
the instrumented output. The equality receipt is
[mathematical-prefix.json](evidence/instrumented-v1/mathematical-prefix.json).
The original source/result stays untouched. [pcc:unverified]

| Additional diagnostic process | Wall seconds |
|---|---:|
| Instrumented strict build | 14.751 |
| Instrumented collector | 33.343 |
| Unchanged separate checker on instrumented output | 0.801 |
| Separate incidence-inspector build | 1.335 |
| Separate incidence inspection and controls | 0.026 |

These calls are additional work, not included in 119.375 seconds. Their shell
staging overhead lacks an aggregate clock and is explicitly unpriced. No
attempt failed, timed out or used a widened limit. Strict builds retain
300 seconds/1GiB; collector/checker retain600 seconds/1GiB; inspection uses
30 seconds/1GiB. The primary outer limit is 7200 seconds/1GiB. Peak RSS was not
measured. [pcc:unverified]

\clearpage

## Collector subphases and remaining accounting boundary

![Recorded wall-clock subphases of the additional instrumented collector, one fixed call on one CPU. The logarithmic axis shows short checks alongside the target arithmetic. This is a diagnostic decomposition, not a speed comparison.](phases.pdf){width=92%}

The native clocks attribute 28.615824 seconds to target principality in this
diagnostic call. All nonnegative phase clocks, including unassigned overhead,
sum exactly to 33.327379 seconds before final metadata serialization; the
separate inspector checks that sum. The Bash process clock additionally
includes final output and process overhead. These figures do not estimate
field-size scaling or candidate yield outside this fixed batch.
[pcc:unverified]

## Reproduce and assess the remaining goal

From the repository root, use a fresh output name:

```bash
local_study=research/nist_cover_three_tests_20261116/local_collection_20261124
bash "$local_study/run-local.sh" evidence/<fresh-name>
```

The [README](README.md) gives diagnostic/inspection replay commands and the
path-bound raw-record policy. Source checkpoint 435d04b3fb precedes the primary
call; the recorded driver and all native sources are published. Mathematical
inputs are the pinned existing construction; the input, map and KM setup
clock measures their loading/initialization, not their original synthesis.

The completed local component supplies actual principal relations, elliptic
transport, separate arithmetic checks, closed-rank/independence evidence and
recorded costs for every executed native stage. **Original map-synthesis and
upstream construction/search cost remains unmeasured.** Local metadata,
reporting/publication and supplemental shell staging are outside the recorded
primary wrapper. Independent signed verdicts also remain unavailable. These
limits prevent treating this as the fully signed, fully priced broader goal.

Sources reviewed: the exact frozen collection/preflight protocols, native
collector/checker and baseline/transfer sources, prior private-column and
finite support reports, and the already recorded Diem/Magma literature.
KB is unavailable as previously recorded. No novelty, overall ECDLP-hardness,
general curve-family or performance-ranking claim follows from this run.
Prior canonical performance graphs remain unchanged; the new figures record
this local component only. All result claims remain producer-unverified.
[pcc:unverified]

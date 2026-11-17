# Matched speed experiment — frozen before measurement

Status: implementation/development checks; backend selection pending. No
speedup has been established by this protocol `[pcc:unverified]`.

## Question, arms, target

Does directly constructing certified Semaev pair cubics improve the actual
cost of computing the complete reduced grevlex Gröbner basis of the same ideal?
Hypothesis only `[pcc:unverified]`.

- Baseline: the strengthened common native Buchberger path on raw equations.
- Candidate: structural recognition, direct cubics, exact identity checks,
  then the identical Buchberger path on raw equations plus those consequences.
- A/A: baseline versus an unchanged baseline, same repetitions and ordering.
- External references: fastest tried of Singular std/slimgb and msolve F4,
  same exact inputs and reduced basis checked equal. Both raw and preprocessed
  inputs are tested. msolve's documented p<2^52 restriction excludes the 53-bit
  prime, which stays in all native and supported Singular arms.

A "drastic" result means a paired ratio with median >=10 and a preregistered
bootstrap 95% interval wholly above 10, with an admitted A/A control. Results
are per case and solver; no pooled ratio may turn losses into wins. No claim
of fastest performance until the supported external comparisons are complete.

## Frozen workload

Two primes: 2147483647 and 9007199254740881. Variable counts 2,3,4,6. Seeds
101,307,911 (held out from development seeds 7,19,43). There are 24 cases.
Every variable is an x-coordinate of a public known-answer curve point.
Cycle edges reuse those variables; every edge contributes f3(x_i,x_j,u) and
f3(x_i,x_j,v) at the sum/difference abscissas. Two variables use one edge.
No unconstrained padding, repeated independent blocks, or hand-picked subset.
Point scalar labels and witnesses construct/check fixtures only; neither the
preprocessor nor any solver receives a witness as an input shortcut.

`compare --prepare NEW_DIR` exports the native-generated exact fixtures and
matching Singular/msolve inputs. Failed/degenerate generation is a recorded
failure, not a silently substituted seed. This is a polynomial-solver component
workload, not a complete factor-base or logarithm-recovery benchmark.

## Accounting and procedure

Same code revision/compiler/profile/host, one thread, pinned to the same allowed
CPU. Twenty-one paired repetitions per case, alternating AB and BA. At least
seven matched A/A pairs. Fixed initial worker batch: one cold solve. For cases
below clock resolution, declare unresolved; do not choose a favorable batch
post hoc. An additional fixed 100-solve batch may be reported separately.

Native workers exclude process launch, JSON parsing/primality validation and
known-answer construction in both arms. The charged interval includes cloning
solver input, candidate recognition/construction/identity checks and the full
solve including the native reference's membership and S-pair checks. Inter-arm
basis equality replay is timed/reported separately and added to each arm's
full checked interval before making a headline ratio. No precomputed reference
basis or degree-completion oracle is used by the measured solver. Diagnostic
closure traces are outside the measured solver and cannot select per-instance
relations after observing their cost.

CAS scripts set CPU-timer ticks to 1e6 per second; their internal solver timers
exclude process launch/printing. Report CPU separately from native monotonic
wall time; never form a speed ratio across those two units. For cross-tool
wall comparisons include each process's overhead explicitly or use a matched
in-process timer for both. Report preprocessor/check costs with the external
candidate arms as well. A stdout file or zero process exit alone is not a
successful basis; parse/check every polynomial and completion marker.

An independently built `count-field-ops` binary records modular-product counts
separately from the uninstrumented timing binary. Counts are neither elapsed
time nor GAE; do not mix them in one ratio.

## Caps, failures, uncertainty and storage

One thread; per-worker deadline 30 seconds; total execution deadline 20 minutes;
no automatic retries; memory ceiling 2 GiB where enforceable. The native
reference's existing 128-basis/10000-pair cap remains fixed. Timeouts, budget
stops, external prime/ring incompatibilities and incorrect bases remain rows
with no successful speed ratio. Local runs, if selected, are L0 observations,
not isolated L2 benchmark evidence or leaderboard promotion.

Bootstrap: paired resampling within a case, 10000 draws, fixed seed 20261115.
Keep every raw sample, permutation/order, compiler/tool versions, binary/source
hashes, CPU/affinity, included/excluded phases and failure record. Controls or
partial data cannot establish a declared tenfold result. Publish raw evidence,
source, report and native review tests in the PR before any positive claim;
PCC standing remains unverified until the independent signed process runs.

Cloud backend: do not launch with unknown/exhausted budget; keep the backend
question pending. Commit/push inputs, protocol, source and launch spec and
obtain budget/capacity receipts before any cloud build or trial. This protocol
does not authorize changing providers or raising the lab's spending limits.

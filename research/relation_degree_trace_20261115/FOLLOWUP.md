# Follow-up: make the degree drop cheaper to obtain

User objective: use the tracer to establish a drastic actual speedup. This is
an objective to test, not a predetermined result `[pcc:unverified]`. Keep both requested primes,
charge discovery/preprocessing and equal correctness work, compare against the
same solver and a strong external solver where available. Do not infer elapsed
time from a smaller matrix degree.

## Initial hypothesis and algebraic construction

For two f3 equations f_u, f_v at distinct constants u,v, write
s=x+y, t=xy, k=u+v, l=uv. Define

```
g = (f_u-f_v)/(u-v)
h = (u*f_v-v*f_u)/(u-v)
g = -2*s*t + k*s^2 - 4*k*t - 2*a*s - 4*b
h = t^2 + (4*l-2*a)*t + a^2-l*s^2-4*b*s
C = t*g + (2*s+4*k)*h + (k/2)*s*g
```

The weighted-degree-four terms cancel, leaving a cubic in x,y. Its s^3
coefficient is (u-v)^2/2. This gives a candidate with an explicit membership
certificate using a bounded number of polynomial operations, rather than
running a degree-five scout. Invertibility requires odd characteristic and
u != v. These assumptions will be checked rather than silently imposed.

Prediction: augmenting the frozen f3 pair with g,C will preserve the ideal and
match the selected downstream threshold, with construction below the cost of
the scout. That does not predict a tenfold elapsed-time gain. A finite frozen
comparison will decide the actual ratio and include losses and failures.

## Before implementation / evidence receipt

- Prior work read: degree-reporting correction; guarded-degree-invariants
  report (filtered closure and BFS/CM distinction); primary Caminata–Gorla
  definitions reviewed in the initial work. Semaev formula is the existing
  primary-source construction, not a novelty claim.
- KB unavailable: no MCP tool and wrapper exits 127 (missing executable).
- Conductor unavailable: CLI is absent; scope remains inside this new study.
- PR #4228 is open, with a shared session-log conflict assigned to Copilot.
  Do not edit that conflicted shared file while resolution is pending.
- IsoLab cost controls forbid launches with unknown/exhausted remaining
  budget. Latest log records exhausted monthly spending; no AWS CLI/identity
  is configured here. A backend question is pending while native algebra and
  development tests proceed. No new cloud job is launched.

Acceptance for a "drastic" measured win: preregister a tenfold threshold with
paired repeated samples and an uncertainty interval entirely above ten, under
the declared matched computation and host. Report preprocessing, solve,
verification, reference-oracle and setup costs separately. Any incomplete
campaign remains incomplete; a smaller diagnostic or favorable subset does
not replace the full frozen comparison.

## Direct-construction development check

Implemented native structural f3 recognition (including scaled equations and
embedded variable pairs), the short syzygy, exact original-input certificates,
and degeneracy checks. Focused native tests passed on both fixed primes and
retained the degree-four threshold without calling a scout from preprocessing.
These are producer checks, not a signed independent verdict `[pcc:unverified]`.

Primary Semaev arXiv:1504.01175 section 2 formula was read directly. msolve's
current official README supports prime-field Gröbner bases only below 2^52;
therefore the requested 53-bit fixture cannot silently use it. Installing
native msolve and Singular in a user-owned prefix to check strong external
references; no scientific timing comparison has run yet.

Next: matched cold full-GB workload; avoid timed calls to the trace's reference
completion oracle as if it were free. Strengthen the baseline equally in both
arms (for example avoid exponentiating inv(1)), then freeze it before timing.
Retain failures and all workloads; a tenfold target is not a claimed outcome.

## Accounting and baseline refinements before any timing

The common native baseline and candidate now return inv(1)=1 directly; both
arms share that change so an avoidable modular exponentiation cannot create a
candidate-only win. The preprocessor uses closed inverses of 2 and 4 and reuses
its single inverse of u-v to normalize the cubic. Its identity verification
remains included. A separate `count-field-ops` build records modular products,
nontrivial inversions and sparse coefficient updates; these counters are not
full machine-operation costs or GAE.

The first reference-install attempt failed because conda-forge has no `msolve`
package. Installing Singular plus native FLINT/GMP dependencies, then building
msolve from its official source is the next route. This is dependency setup,
not a substituted performance measurement.

## Prepared matched execution path

Added a native fixture/CAS exporter and full-GB comparison worker, plus a
bounded paired native campaign driver (21 paired samples, seven A/A controls,
exact returned-basis comparison, raw per-attempt files, preserved failures,
fixed bootstrap seed). The campaign is not executed while the backend choice
is pending. External CPU timers remain separate from native wall ratios.

Singular's 53-bit modular coefficient ring passed an installation correctness
check. msolve 0.11.0 built from official commit
8667f956a2534a3b414a3cb77f82eb8e3ef204cc; its p<2^52 restriction remains.
The first Singular timer setup syntax lacked `--`; the documented
`system("--ticks-per-sec",1000000)` works. This is setup, not speed evidence.

## Development checkpoint before backend-dependent measurement

The new native fixture generator tests shared-point cycles at 2,3,4,6 variables
on both requested primes, and checks every witness and direct consequence.
The campaign uses held-out seeds rather than the development seeds. Its driver
rejects counter-instrumented timing binaries, mismatched bases and incomplete
runs as wins; it preserves one file per attempt and never overwrites a run.
No timed campaign or tenfold claim is made while the backend answer is pending.

Initial `development/` receipts and hashes belong to PR commit
4dbcef3ce79b525d11994447cc1f2d7f7cd46ee6. They are preserved, not relabelled
as checks of this follow-up. New checks are recorded separately below.

## Final follow-up development receipts and open dependency

Native normal and counter builds, parameter/cancellation tests, CLI tests and
campaign safety/statistics tests are retained in `followup-development/`.
Prose lint and ledger-chain checks pass as producer checks `[pcc:unverified]`.
The interpretive claim is `PCC-20261115-87803b`, with no independent signed
verdict. Initial receipts remain pinned to their original commit.

The scoped deliverable now removes the per-instance scout from the candidate
construction and prepares matched evaluation. The requested drastic actual
speedup is **not established** `[pcc:unverified]`: no backend answer has arrived,
no timed campaign has run, and external supported CAS comparisons remain
required. Existing IsoLab notes report exhausted monthly spend; no live budget
headroom or AWS identity is available in this runtime. The explicit local-vs-
IsoLab question remains pending. No default-backend or spending control is
bypassed, and no favorable ratio is invented from the degree reduction.

## 2026-11-15T05:20Z — Continue with local L0 execution

The user's "Continue" is taken as direction to proceed with the previously
offered local workspace option. No cloud resource is launched. All observations
will retain the local L0 label; the frozen cases, tenfold criterion, controls,
cost intervals and failure rules remain unchanged. The original protocol's
backend-pending text is historical and is resolved by this execution note.

Before launch: verify staged source hashes, build the uninstrumented release
workers, regenerate and compare all fixed inputs, then run the bounded native
campaign. Preserve every failure and use its outputs to select the next step;
no speed ratio is assumed. Supported external CAS runs remain required.

## 2026-11-15T05:25Z — First local observations and next construction

The in-progress frozen native campaign reports only modest changes in the
completed two-/three-/four-variable cases; the tenfold criterion has not been
met there, and failed A/A controls are retained. These are partial producer
observations, not a conclusion about uncompleted cases `[pcc:unverified]`.

Next falsifiable construction: project each paired-f3 edge to a univariate
polynomial using the Sylvester resultant of its certified cubics. Cofactors
supply explicit original-input membership certificates. Shared vertices then
admit extended-Euclidean GCDs of their incident projections, with certificates
inherited by linear combination. Prepend these consequences before the same
full solver. This uses input overlap, not an observed answer or per-instance
scout. The baseline, all fixed seeds/cases, time/memory limits and accounting
stay fixed in a separately labelled successor comparison; original results
are not overwritten or pooled with it.

Prediction: shared-variable projections remove work from the global solver.
Counterprediction: construction and certificate growth dominate. Both costs
must be charged, and incomplete baselines cannot become numeric wins. The
small-case field/cancellation tests precede any new timed comparison. This is
standard resultant/cofactor and Euclidean elimination, no novelty claim.

## Successor implementation prepared while original run continues

Implemented sparse Sylvester cofactors, exact original-input projection rows,
and inherited extended-GCD certificates. The original timed workers are still
the frozen binaries; no active worker binary is overwritten. Compilation and
new tests will wait until the original run ends, to avoid a competing build
load on the measurement host.

The successor dispatch and deterministic-failure rule are frozen in
PROJECTION_PROTOCOL.md. Singular output replay will require an exact match to
a completed native basis; an unsupported ring, parse failure, missing marker,
or absent native reference remains explicitly unverified. Solver-only external
CPU batches are diagnostic, not a mixed-unit or uncharged end-to-end ratio.

## 2026-11-15T05:34Z — Original run stopped for repeated-timeout defect

The six-variable case has repeated 30-second worker timeouts (empty worker
outputs; timeout status retained in per-sample records). The original driver
continued the frozen repetition schedule after deterministic failure. This
conflicts with the no-unchanged-retry rule and wastes the campaign budget.
Stop that defective campaign; preserve all completed cases and partial files,
and explicitly mark the overall comparison incomplete. Do not turn the
censored six-variable baseline into a numeric speed ratio. The successor
already stops a case after its first failed pair and interleaves fields.

This is a disclosed harness correction, not an outcome-based exclusion or a
claim that every frozen case was measured. Completed native observations are
producer-only `[pcc:unverified]`; the remaining 53-bit cases still need their
matched baseline measurements.

## 2026-11-15T05:35Z — Successor correctness checks

Projection and shared-vertex gcd construction passed exact original-input
certificate checks on both primes; the three-variable development gcds are
quartic and the complete reduced bases match the raw-system bases. The full
native suite and clippy passed as producer development checks `[pcc:unverified]`.
Source and successor protocol are published before the next timed run. The
failure-stop correction now applies to either candidate arm, not just the
successor. No old raw result or frozen binary is rewritten.

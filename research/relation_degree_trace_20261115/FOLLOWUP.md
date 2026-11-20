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

## 2026-11-15T05:39Z — Projection observation and solver ablation

The first completed four-variable projection cases have producer median ratios
about 4.9 against the frozen native solver. The 31-bit seed-101 A/A control is
admitted; the 53-bit seed-101 control is not. This is not the tenfold target
and not a statement about remaining cases `[pcc:unverified]`.

Next ablation, frozen before timing: interreduce the initial generator set
before making S-pairs. This is standard Buchberger housekeeping; it can expose
cheap consequences and discard obsolete generators. Preserve the original
reference routine as the timing baseline and compare both raw-input and
projection-input variants of the interreduced routine. Report the engine effect
separately from any relation contribution. All final bases and original-input
certificates must pass the existing check. No running worker binary is changed.

## Failure-stop review before engine comparison

Static review found the successor stopped failed pairs only after entering an
A/A block on sample zero. That would launch two more timed-out baselines.
Correct it to admit controls only when both arms completed, and add a regression
test. The currently running projection campaign's completed four-variable
observations remain preserved; stop it before entering the known problematic
six-variable failure path, rather than launch unchanged failures again. The
full projection panel will be marked incomplete, never silently promoted.
The corrected engine campaign will retain all 24 planned cases and stop after
a first failed pair without A/A retries. All revised decisions are explicit.

## 2026-11-15T05:43Z — Engine ablation readiness

The initial-interreduction routine passed original-input certificate, complete
basis and projection/raw equality tests on both fields. The corrected control
eligibility has a regression test: failed pairs cannot launch A/A retries.
Full native tests and clippy passed as producer checks `[pcc:unverified]`.
The previous projection interruption records the stop race and orphan-worker
cleanup explicitly. No timed process or build remains before the next launch.
Publish the engine/control fix and its protocol before running both ablations.

## External comparison preparation

The native replay parser now covers msolve's documented positive-coefficient
reduced-basis format as well as Singular's term dump, requiring proper framing
and exact coefficient/exponent agreement. Unsupported 53-bit msolve calls are
explicitly not attempted. External solver CPU batches and CLI whole-process
wall times are separate diagnostics, not mixed with native solver-only wall
or relabelled as end-to-end gains. Preprocessing costs remain an obligation
for any external candidate ranking.

## Analysis/replay prepared without changing running binaries

A native report renderer retains the entire fixed case roster, failed controls,
missing/interrupted cases, matched milliseconds, paired intervals and per-case
thresholds. It does not infer speed from degree, pool different cases, or turn
censored baseline costs into wins. The external parser accepts only fully
framed, exact coefficient/exponent bases. No analysis or build load is run
concurrently with the active timed campaign; code changes are compiled after
those workers finish.

## 2026-11-15T06:02Z — Combined result and final bounded engine test

Completed four-variable combined cases show about 6.7–6.9 producer median
ratios, with failed controls explicitly not admitted. The declared tenfold
criterion is still unmet in those completed cases `[pcc:unverified]`.

Next bounded test: retain full membership and every final S-polynomial check,
but stop propagating unused provenance during the verifier's polynomial-only
normal forms. Apply this equivalent checker to both arms. Preserve the legacy
checker and test equal outcomes on valid, missing and forged bases. Separately
add the standard relatively-prime-leading-monomial product criterion to the
candidate engine; the final checker still recomputes every S-pair. Compare raw
and projection inputs to attribute the construction effect. No novelty claim;
this is standard Buchberger pruning and verification housekeeping. Freeze the
comparison before timing, retain all prior losses, and do not weaken checks.

## 2026-11-15T06:08Z — Engine panels finished

The raw-interreduction and combined panels have ended and retained their full
planned case roster. Six-variable cases are incomplete under the worker cap,
not speed ratios. Four-variable combined cases are approximately 6.7–6.9
producer medians; the tenfold target has not been established there
`[pcc:unverified]`. No active timing process remains before building the final
bounded criterion/checker comparison and the external replay/report tools.

## Final comparison safety review

The fast checker and product-criterion routine passed the exact legacy/new
checker and forged/missing-basis tests. Prose remains explicit that this is a
producer check, not an independent verdict `[pcc:unverified]`. A final harness
review also stops a case on any failed exact-basis/counter-build eligibility
check, not only a failed process status. External-parser clippy requested the
standard divisibility method; no lint or correctness gate is disabled.

## 2026-11-15T06:20Z — Fresh confirmation preregistration

CONFIRMATION_PROTOCOL.md fixes fresh seeds 1009/2003/3001 for the four-variable
family on both fields, before the final projected arm's results. The native
exporter does not call a solver. Its complete six-case panel will be used only
if the final exploratory arm has an admitted tenfold observation. Failed
controls/seeds will not be replaced, and no old/fresh samples are pooled.
The raw-input criterion control remains a measured regression on completed
four-variable cases; the projected comparison is queued, not yet interpreted
`[pcc:unverified]`.

## 2026-11-15 — Fresh confirmation and external closeout

The frozen six-case confirmation completed all 21 paired samples and seven
A/A pairs per case with byte-identical source-2471e4c workers. Five cases are
admitted at 23.4–23.9× median checked native wall ratios; p31 seed1009 fails
its A/A control and is excluded. No replacement, pooling or rerun was used.
All 24 supported external arm/algorithm basis checks match exactly, with 12
documented unsupported combinations. CAS solver-only diagnostics are much
smaller than native charged intervals and do not establish a native advantage
over mature solvers. Full scope, limits and earlier losses are in
FINAL_REPORT.md `[pcc:unverified]`.

No timed workload remains running. Native release tests, counter-feature
tests, formatting and lint receipts are under validation/. Final analysis
uses standard Plotters SVG output; the plotting dependency and formatting
changes were made after the immutable timed workers finished. Historical
development receipts retain their original sources. Raw runs, exact CAS
inputs and measured binaries are archived with hashes under artifacts/.
The full root repository suite is outside this partial source checkout.

The study remains local L0 without independently signed replay or matched-rho
validation. No accepted performance/leaderboard or DLP standing is promoted.
PR #4228 retains a concurrent root-session-log conflict. Copilot was asked
once to merge the base without rewriting evidence; no reply has arrived.
Main has now also appended a different hash-chained PCC history. That ledger
conflict is reported, not resolved by selecting or rewriting either side.
The study log here avoids concurrent edits to the root session log.

## 2026-11-15 — Chase direct-quadratic and quotient-algebra leads

LEADS.md records a division-free quadratic resultant identity on the original
equations. Implemented as augment_quadratic with original-input certificates;
small-prime/scaled/degenerate identity tests and full-ideal checks on both
fields pass. On the fixed three-/four-variable development cases, the complete
ordered augmented inputs equal the previous construction. Thus this lead
targets construction/checking, without claiming different downstream degree
or solver work `[pcc:unverified]`.

The harness now selects the native baseline explicitly, applies it to A/A
children too, and retains the actual protocol label. A fake-worker integration
test checks all dispatches. The exporter accepts an explicit construction and
frozen seed list without implying freshness. QPROJECTION_PROTOCOL.md is frozen
before any successor timing, with the existing candidate as baseline and
conditional fresh seeds 5003/7001/9001. No new scientific run has launched.

The real sparse clone at /workspace/crypto-leads fixes the old API-staged
directory's missing Git metadata. A complete resume note is in LEAD_RESUME.md
and the local root checkpoint path. The actual checkpoint gate still cannot
upload: no configured AWS profile or presigned upload token. AGENTS.md requires
that gate to pass before launching, so timing remains blocked. Development
continued with 29 normal tests, 30 counter-feature tests and strict clippy;
receipts/source hashes are under quadratic-development/. Old raw evidence is
unchanged. The earlier count-feature suite had 27 tests rather than 26; its
validation README now states the actual count without rewriting the log.

The stronger next lead is a certified quotient-algebra route: invert a linear
subresultant modulo a certified vertex quartic, propagate coordinate maps and
derive the complete grevlex basis by quotient linear dependence. Nonunit
denominators, certification cost and full ideal equality must be checked.
This remains a lead, not an implementation or timing result `[pcc:unverified]`.

Current producer record: PCC-20261115-1f0cb7, unverified. The earlier mutable
lead-note pin is explicitly documented as stale in quadratic-development/
CLAIM_HISTORY.md, with its original bytes preserved. Ledger rows and frozen
claims were not rewritten. Main's divergent ledger and root-log conflicts
remain outside this development update.

## 2026-11-15 — Certified quotient prototype

Implemented quotient::solve: a certified vertex polynomial, Euclidean unit
inverses of linear subresultants, graph propagation of coordinate maps,
original-input vanishing under those maps, and grevlex change of order by
exact quotient linear dependence. Final basis rows retain original-input
certificates; monicity/reducedness, input normal forms and all S-pairs are
checked. No witness, reference-basis oracle or hidden Buchberger fallback is
used by the constructor. Exact bases match the reference on the fixed
three-/four-variable seed-7 cases over both fields `[pcc:unverified]`.

The QUOTIENT_PROTOCOL.md development protocol preceded implementation.
QUOTIENT_SPEED_PROTOCOL.md separately freezes a future comparison against
the best measured native arm, including all construction/checking cost and
conditional fresh seeds 11003/13001/17011. No quotient timing, six-variable
validation, degree-of-regularity change or practical CAS advantage is claimed.
Unit-test elapsed times are not benchmark results `[pcc:unverified]`.

Final development checks pass 35 normal tests and 36 counter-feature tests,
plus strict all-target clippy in both modes. Nonunit, missing/forged-rule and
exact dependence tests are retained. Harness eligibility now stops a completed
but basis-mismatched pair before any A/A children; a fake-worker integration
test covers that case. Initial warning/lint receipts are retained separately
from the final passes under quotient-development/ `[pcc:unverified]`.

The worker's quotient_info reports stored certificate product degrees and
final basis degree, not a measured solving threshold or paid maximum degree.
Dense quotient writes are explicitly outside the sparse coefficient-update
counter; modular-product/inversion counters are not GAE. Original raw evidence
and prior standings are unchanged. Timing remains blocked by the required
S3 checkpoint upload; the complete nonsecret handoff is in LEAD_RESUME.md.

## Native Rust API integration and fixed-cap six-variable qualification

The implementation now lives once in crates/certified-relations. The study
reexports it, and the real crypto_lib F4 module exposes an opt-in certified
polynomial adapter and degree tracer. Exact complete F4 basis controls on
the fixed 31-bit three-/four-variable cases, wide-field arithmetic/certificate
checks, malformed-input/report rejection and incomplete-trace status pass in
a separately copied immutable native qualifier `[pcc:unverified]`.

Six variables initially exceeded the proof cap at input rewrite, then after
lean polynomial-only containment exceeded it at change of order. The failures
and all prior qualifier bytes are retained. Breadth-first dependency waves
reduce the fixed cycle's derivation depth from 4 to 3; both requested fields
now return full checked six-variable bases under the same caps. Final stored
basis proof size is at most 8336 terms, with stored certificate product degree
40 and final basis degree 2. These are not solving-degree or runtime results
`[pcc:unverified]`.

See INTEGRATION_REPORT.md, the three hypothesis files and integration-development/
for source commits, pre/post manifests, raw results, failed builds and executable
archives. The original LEADS.md claim subject is restored byte-for-byte;
current notes live in LEAD_STATUS.md. Concurrent parent-branch main/ledger
merges advanced while the work ran. Publication therefore uses a focused
follow-on branch preserving the qualified commit history; no frozen ledger
side is selected or rewritten.

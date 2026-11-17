# Degree-capped RR preprocessing — proposed qualification protocol

Status: frozen candidate definition; **execution not admitted**. No scientific
run has started. This protocol preserves the original cost/degree/width question.
Before timing, pin the final source, engine version, original/reduced input
hashes, exact target manifest, output normalizer and their hashes in an addendum.
Do not choose the best candidate using claim-panel outcomes.

## Competing hypotheses

- H1: redundant auxiliary variables drive RR width; constant-pivot substitution
  lowers matrix width while retaining RR solving degree and reducing full cost.
- H2: solver preprocessing already removes these pivots, so the change is inert.
- H3: substitution increases equation density or changes the solve enough that
  retained input degree produces worse solving degree, width or complete cost.

Arms: frozen RR, RR-affine, RR-quadratic, F and T. The RR variants are the new
emitter modes; F is the stronger existing m3 comparator and T the historical
control. All use the same msolve 0.6.5 settings (`-g 2 -t 1 -v 2`) initially.
Magma/FGb are not available in this environment; any eventual ranking must
name missing external comparisons. Do not call F universally fastest.

## Inputs and output equivalence

Start with the existing P1 T24/T31 m3 t3–5 instances and admissible seeds, reused
without resampling. Kummer p0 controls retain the P1 fixture and curve identity.
These are planted calibration inputs, not a natural-target result.

Before a complete-output claim, implement extraction to unordered signed point
multisets summing to the same public finite R. F accepts ±R at the x level;
enumerate and verify signs, deduplicate, and orient to R. RR extraction checks
its monic A/C chart, C denominators, rational roots, on-curve points and exact
group sum. Restrict both arms to an explicitly declared common chart for the
stage comparison. Full domain qualification additionally covers excluded charts
and repeated points, or keeps those exclusions explicit. A known planted solution
or quotient dimension alone never establishes completeness.

Check the reduced-to-original map for every extracted RR solution, verifying
the original input hash and original equations. On exhaustive small fixtures,
compare the entire normalized output set with an independent finite reference.
Do not use only the producer's stored planted assignment as validation.

If stage correctness passes, freeze a separate fresh-uniform-target panel before
sampling. Include zero-output, multiple-output and chart-exception cases; all
attempted targets count. Choosing only admissible planted positives is not that
panel. The target manifest and sample count are still pending, so this document
does not authorize timing an unspecified natural-target workload.

## Observables and admission

Record preprocessing CPU and memory, original/reduced variables and input
terms, maximum input degree, F4 solving degree, widest matrix rows and columns,
aggregate matrix work, solving CPU, extraction/verification CPU, peak RSS,
complete normalized solution set, status, and original/reduced input digests.
Retain engine-internal solve time separately from full per-query cost.

Same host/engine/thread count, warmup, at least five measured rounds, A/A control,
and native L2 admission. Charge transformation and reconstruction, extraction,
verification, and failed queries. Report reusable setup separately. A capped job
is censored, never a win. Blindly pooling planted and natural-target costs is
not permitted. Record per-target first-witness and complete enumeration separately.

Success requires RR degree no higher than original RR, strictly smaller widest
matrix, equal normalized output sets, and complete cost below the best completed
equal-output baseline on the paired cells, with a paired upper 95% cost-ratio
bound below one on a preregistered independent panel. A narrower matrix alone
is a component observation. If F is still cheaper, report the loss and retain
the original objective. If input changes are eliminated internally, reject H1
for this transformation and inspect a different structured-solving change.

## Resource and stop conditions

The first admitted stage panel is m3 only, one job at a time, 4 CPUs requested
(one solver thread), 6 GiB/job, 600 s/solve, no automatic retries. Campaign
ceiling USD 10 including builds, failures, idle/storage/transfer; this is within
the repository's existing shared allowance only if current daily and monthly
headroom permit it. Before launch assign an absolute UTC deadline, reconcile
existing requests, verify live pool capacity and commit/push exact source and
inputs. No launch while the balance is unknown. Do not move to m5 or raise
resources under this panel; larger m5 tower work needs a separate admitted card
within known headroom. No such run was launched here.

Blockers: S3 checkpoint transport, current budget/pool readback, pinned
source-built experiment image, output-normalization implementation and frozen
target/input manifest. GitHub submission is an available route after those
conditions, not a substitute for them. Preserve artifacts and follow
`AGENTS.md` and the IsoLab skill; do not run local timing as a workaround.

## Qualification addendum — 2026-11-12 (no change to frozen success criteria)

`p1tool extract` now validates one rational RR/F assignment and emits canonical
signed triples on the explicit common m3 chart. Its development fixture and
exact exclusions are in EXTRACTION-20261112.md. All supplied input equations
and the reconstruction digest are checked before extraction. This is an
implemented normalizer, not a complete rational-assignment enumerator.
Before any equal-output timing, the runner must still enumerate all rational
solver assignments, union normalized outputs, retain excluded charts explicitly,
and qualify completeness against an independent reference. No scientific F4
run has begun; checkpoint transport and current budget/pool admission remain
missing. The original degree/width/cost acceptance criteria are unchanged.

## Collector addendum — 2026-11-12 (same success criteria)

The native collector handles supplied basis equations reducible by affine pivots
and prime-field quadratics, with explicit unknown for unsupported/capped cases.
It validates all enumerated assignments through the input/extractor and unions
normalized points. See COLLECTION-20261112.md. This provides a collector for the
archived linear/quadratic forms; it does not certify basis equivalence to input.
A matched run must still bind each basis to its exact solver execution and
qualify complete input output-sets against an independent reference. Exit 0
from `collect` alone cannot admit timing or certify original-input UNSAT.

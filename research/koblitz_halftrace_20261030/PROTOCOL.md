# Native Koblitz point-lift cache

## Request, baseline and prior work

Integrate useful prototype mechanisms into the existing Rust library, measure
against its native baseline, and open a PR. Baseline:
`3b34d504bc13ce668e86d8e68cdf57e3fbaba89a`. Existing Frobenius folds, normal-basis
keys, pair joins, collectors and sparse solvers are prior work, not contributions
of this change. Earlier Python timings are historical prototype diagnostics,
not evidence against this Rust baseline.

Track: **generic decomposition (bounded by sqrt(r); cannot beat rho)**.
This change concerns native implementation cost, not algebraic progress.

Literature and implementation reviewed before choosing the change:

- SEC 1 v2.0, section 2.3.4 step 2.4.3,
  <https://www.secg.org/sec1-v2.pdf>: binary recovery solves
  `u^2+u=x+a+b/x^2`, then returns the corresponding `y=x*u` and its negation.
- `src/binary_ecc/f2m.rs`: existing linear trace/halftrace tables and
  definition-based parity tests. Use the same identity in the one-word path.
- `src/cryptanalysis/koblitz_index_calculus.rs`: repeated halftrace squarings
  in `points_with_x_fast` and its invariant/subspace/subgroup-base callers.
- `src/cryptanalysis/ic_boundary.rs`: its existing echelon Artin-Schreier
  section can select the other root, so direct substitution would change
  point ordering. Preserve the halftrace-selected root exactly.
- `src/cryptanalysis/koblitz_fast.rs`: normal-basis rotation keys and bit-parallel
  minimum rotations already exist. No novelty is claimed for those mechanisms.
- `RESEARCH_KOBLITZ_INDEX_CALCULUS.md`, `RESEARCH_ECC2K130_IC_FEASIBILITY.md`, and
  `research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md`:
  existing orbit folds, pair tables, degree guards and negative scaling results.
- GGMP, SAC 2020, <https://eprint.iacr.org/2020/1315>: repository implementation
  and notes read; the primary PDF returned HTTP 403 here. No conclusion is
  inferred from an abstract. Connected lead-atlas code search for `half trace`
  returned no entries; its executable atlas has not been run in this environment.

## Hypothesis and implementation scope

The odd-degree halftrace is binary-linear. A lazy byte table on FastCurve should
replace repeated dependent squarings without changing accepted coordinates,
the first returned root, signs, point ordering, relations or recovered scalars.
Charge the cache's cold construction. Share it on clone, not globally across
unrelated fields. Retain the original path as the default and explicit control;
the factor-base parameter `halftrace_tables=1` selects the candidate. No new
83-bit or 131-bit IC backend is claimed by a one-word optimization.

## Frozen workload and controls

`spec.json` uses the existing K_0 constructor at degree 31, eight independent
one-target workloads, target and order seed 20261030, one warmup, six alternating
measured rounds and mandatory L2 admission. Divisor indices `1;2` define the
same base in both IC arms. The sole candidate difference is
`halftrace_tables=1`. Use `mitm-frobenius-counted:m=3` and full-rank checked LA.
The matched reference is strong signed-Frobenius rho, with its own A/A control;
baseline IC also has an A/A control. Require identical factor-base point hashes,
verified scalars and independent replay before any comparative claim.
The IC baseline arm selects the legacy lift inside the candidate executable;
it is not the preserved unmodified executable. A comparison of those arms can
measure this opt-in cache against that control. It cannot establish that the
new selector branch has no cost relative to the unmodified revision. A matched
comparison against a source-pinned baseline image is a separate pending gate
for any claim about the change's default-path regression or overall improvement
over that revision.
The first four deterministic workload records are the primary panel and the
last four are independent holdouts; all eight must pass. No workload is used to
retune the cache. The native plan warned that five rounds cannot balance the
alternating order; the protocol was changed to six before any measurement.

Degree 31 has no proper intermediate subfields, but ord_31(2)=5, unlike
ord_83(2)=82 and ord_131(2)=130. Its small invariant subspace is not evidence of
structural fidelity to the challenge. The m=83 gate is unmeasured and any
high-fidelity transfer toward ECC2K-130 remains unestablished.

## Accounting and acceptance

Keep base and cache setup, relations, matrix solving, target descent and
verification in the cold total. Also report reusable setup separately from the
complete target-dependent online window. The generic reference floor in S units
is sqrt(pi/(2*2*n)); compare to the strong single-target rho on the same points.
Retain every arm's unpriced counts and apply the 0, 0.1 and 1 GAE band.
The current coarse `as_solves` counter does not price the table/squaring delta:
do not infer an S improvement from an unchanged solve count or a stage time.

Success requires an admitted native comparison with exact output parity and all
answers verified. Stage-only evidence stays a diagnostic. No leaderboard
promotion, fastest-tool claim or pipeline speedup precedes the required evidence.
External arithmetic references (FLINT/NTL/Magma) have not been measured on EKS.
Retain failures, timeouts and regressions. Do not retune inputs after seeing data.
The intended EKS Spot run is bounded to 900 seconds including provisioning,
four CPUs, 4 GiB and zero automatic measurement retries. Reconcile any ambiguous
submission against the original request identity rather than renaming a retry.

## Access status

Conductor check returned exit 127 (binary absent); `docs/CONDUCTOR.md` documents
warn-and-allow without installation. No shared reservation is claimed. EKS
submission, successful S3 checkpoint admission and a source-pinned executable
image remain separate requirements. Local builds and correctness tests are
development work, not experiment timings.

## Prospective execution record, 2026-10-30

The unchanged workload is now also recorded in `protocol.json`, validated against
the canonical experiment.protocol/v1 schema and frozen before any scientific data.
`analyze.sh` only invokes the existing native ecbench comparisons, with fixed
bootstrap seed/resamples and the three pricing bands; its hash is in that record.
The protocol's decisive comparison is the cache flag against the preserved
legacy algorithm in one executable. The separate unmodified-source comparison
and default-path regression remain open; this record does not silently discharge
them. The ordinary v1 measurement is non-resumable: interrupted clocks are invalid
and there are no automatic measurement retries. The source-resume S3 checkpoint
is a separate pre-launch safety gate, not a claim of computation recovery.

Pin the executable to tested feature commit
`7c58661935b79153ff74c1208d0592aa56ed04a1`, which already contains these exact spec
bytes and passed the native cache, ordered-lift and end-to-end checks. The broad
release run finished with 5447 passes, zero failures and 123 existing ignored
tests; its only source difference is the previously recorded parity lint cleanup.
Later main merges preserved the three feature files but exposed an existing
syntax error in `ecbench/methods.rs`. That failure is retained separately; a run
of the pinned source does not validate those later merges.

`isolab-experiment.yml` has a checkpoint-only preparation mode. It checks out a
full immutable preparation SHA, uploads this study's resume/spec/protocol through
the existing approved upload secret, and explicitly skips every build/experiment
job. A successful receipt is required before a separate trusted-main scientific
dispatch. Missing or invalid secrets fail closed. No credential is exported to
the authoring workspace, Git or the report.

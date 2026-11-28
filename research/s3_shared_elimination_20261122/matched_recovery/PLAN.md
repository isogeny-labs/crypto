# Matched public recovery: development plan

## Objective and preserved scope

Continue PR #4922 toward a complete cold solving-cost comparison over
Fp with p=2147483647, using the existing ordinary curve and relation modulus
r=2147446831. The next implementation connects original F4, independently
prepared fresh Macaulay elimination, and shared Macaulay elimination to the
same public-input recovery path. Existing production F4, signed group lifting
and IncrementalGauss remain the arithmetic and recovery implementations.

This phase is development correctness qualification. It does not replace the
requested independently validated complete-cost comparison, random public
target corpus, full-chart adapter or experiment admission. No clock or
performance experiment is authorized by this plan alone.

## Source review before implementation

- KB unavailable: the exposed tool registry contains no research-kb tools;
  `scripts/research-kb status` exits 127 because the executable is unavailable.
  The fallback is the pinned #4922 source and report, and current #4920/#4901
  source/PR inspection. No novelty or exhaustive prior-art claim is made.
- #4922 source at 197d7444ff6abcdfdd47e8840b8234258dbf39c1:
  `src/cryptanalysis/shared_macaulay.rs` independently supplies FreshMacaulay
  and SharedMacaulay preparation, while `examples/shared_s3_matrix.rs`
  currently admits only the shared preparation in recovery.
- `src/cryptanalysis/ic_framework/stages.rs` and `ic_framework/mod.rs` expose
  the existing DecompositionOracle and public-input pipeline boundary for the
  later general adapter. `ic_boundary.rs` supplies the existing collector,
  completion contracts and IncrementalGauss. Do not clone the LA algorithm.
- #4920 at d10c5b1e6e5f4c506a2100b43b78a873f65738f9 publishes a standalone
  pair-root pilot, not the unpublished sibling's production chart adapter.
  #4901 already owns generic cold public-target recovery. Do not duplicate
  either mechanism or supply pair-enumeration witnesses to the F4 arm.
- The primary exact finite-field linear algebra references and Semaev source
  reviewed for this study remain linked in `../REPORT.md`, section 6. This
  phase introduces an integration/comparison boundary, not a new algorithmic
  or asymptotic claim.

## Planned implementation contract

1. Make relation preparation strategy-neutral: Original, Fresh and Shared.
   Original must construct only the original equations. Fresh must use
   FreshMacaulay::prepare, without first constructing SharedMacaulay. Shared
   pays its own preparation once per cold public-target recovery.
2. Preserve all original equations, factor-coordinate domains, unrestricted
   auxiliary domain, signed column mapping and solver limits in every arm.
   Coordinate and relation moduli stay separate.
3. Use the same deterministic Q+aG query policy and attempt bound for every
   arm. Record actual query identities, accepted witnesses, rank changes and
   stopping reasons. Different valid witnesses may change the stopping prefix;
   report that rather than asserting equal seeds imply equal work.
4. Retain incomplete F4, exhausted query budget and complete finite-chart
   refusal as distinct outcomes. An arm failure must not suppress later
   scheduled arms or their outcomes.
5. Expose a phase ledger for admission, factor-base construction, preparation,
   query/specialization/F4, lifting/replay, relation LA and final verification.
   Preserve the existing counters and their exclusions. Do not double-count
   kernel products already included in logical products; do not label partial
   counts, payload bounds or diagnostic test duration as complete cost.
6. Keep the existing structured Prime31 fixture and historical evidence.
   Expected scalars belong only to calling checks. The solver receives public
   Q, declared public parameters and the attempt policy.

## Development acceptance before execution

- Existing correctness cases remain present.
- Prime31 positive and negated public targets recover correctly with Original,
  Fresh and Shared; all intended preconditioning backends are exercised.
- An exhaustive small-field public-target check validates the common driver
  independently of a single chosen scalar, using caller-only expectations.
- Query-budget and interruption controls retain incomplete outcomes and the
  complete planned arm list; original/fresh arms carry no shared setup report.
- No expected scalar, reference solution set or factor-base logarithm enters
  the public recovery function.
- Separate source review precedes execution. Build, immutable executable
  pre-call and post-call records remain distinct; pre-call files refuse
  replacement. Run the existing actual-source qualifier as development work.
- Whole-library validation remains a separate required gate. The maintained
  manual `rust-all-targets.yml` full-library job is the supported candidate;
  only an actual successful source-bound run closes that gate.

## Next measurement dependencies

General public-target/factor-base integration through the production pipeline,
the full chart dispatcher, complete cold accounting, exact matched inputs and
holdouts, the named IC incumbent, a readable frozen canonical protocol, and an
authenticated supported runner with live budget/capacity admission remain
necessary before the requested performance comparison.

## Frozen execution layout

The corrected example is frozen at SHA-256
`dd965d7116f387ed9cd61cf79fedcee8230af768f3352a503d3da563d69d0868`
after the separate source review in REVIEW.md. No performance clock is added.

The full example test executable contains the 12 retained cases and five new
matched cases. It runs sequentially with --nocapture and a 180-second guard.
The exhaustive F17 test prints all 126 target/arm outcomes; full failed panels
are included on failure. The Fresh/Shared expired-deadline test prints both
complete interrupted records. The separate --check-prime31-recovery-arms CLI
uses a 180-second guard and emits complete records for all 28 planned Prime31
arms (two target panels and two control panels). These records remain distinct
from the F17 test output. Builds have one compiler job and a 600-second guard.
All guards are cancellation limits, not performance samples.

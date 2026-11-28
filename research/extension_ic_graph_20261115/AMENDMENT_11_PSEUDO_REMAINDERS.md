# Amendment 11: inversion-free S3 remainder steps

Review and implementation decision, 16 November 2026. No new timing has
started. Rust only; authorized local backend. Producer assertions remain
[pcc:unverified].

The named baseline is PreparedSymmetricPairOnly, with the same cached curve
constants, degree-three eliminant, degree-four compatibility polynomial,
exceptional scans and signed group checks. Its tiny Euclidean GCD currently
inverts the divisor leading coefficient even when the dividend degree is
smaller. Most rejected residuals need several subsequent inversions.

Sources checked (KB): no research-KB MCP tools are loaded; the repository
wrapper query for pseudo-remainder finite-field S3 GCD failed with
`research-kb: no executable found` (local /tmp/ic-pseudo-kb-search.log).
Earlier resultant query has the same recorded failure. Fallback sources:

- Existing gaudry_cubic_prepared.rs and gaudry_cubic_symmetric.rs, plus
  research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md sections 11.4/11.9:
  the same elementary-symmetric equations and exact exceptional recovery.
- [FLINT, Division and pseudo-division](https://flintlib.org/doc/fmpz_poly/divrem.html),
  read the pseudo-division contract and algorithm discussion, including its
  reference to Cohen Algorithm 3.1.2. Retrieved 16 November 2026; HTML SHA256
  4bf0b619ea45a849bf26da51951bb89feaa046c01ed438284d62c1feda3a7688.
  Local source /tmp/ic-flint-pseudodiv-primary.html. The documented identity
  is lc(B)^d A = BQ + R with deg(R) < deg(B). This is established polynomial
  arithmetic, with no novelty claim. The integer-ring implementation is a
  reference for the identity, not a measured dependency.
- docs/ic/STANDARD_PANEL.md at incoming main7025256cc: its binary/Koblitz
  and prime-field rungs have no matching odd-characteristic Fp^3 family;
  its documented exception requires the explicit rationale below.

Over the existing prime fields every nonzero leading coefficient is a unit.
Replacing R by lc(B) R - lc(R) x^shift B at each degree drop therefore
preserves the GCD up to a nonzero scalar. The leading terms cancel exactly;
all remaining coefficient multiplications are counted. Normalize only the
final nonconstant GCD, keeping the exact monic polynomial and root RNG stream.
Unlike integer pseudo-division there is no coefficient growth beyond the
field representation. No changed relation density or complexity claim follows.

Implement an explicit PseudoRemainderPairOnly / pseudo-pair option; preserve
the existing baseline and all defaults. Differential tests cover zero,
constants, degree drops, planted factors, random bounded polynomials, all tiny
targets and the existing general cubic. The full cold comparison must require
matching factor bases, residuals, relation streams, group work and recovered
scalars, while allowing changed field operation counts. Full setup, sampler,
failed tests, graph elimination and scalar replay stay charged.

Use synthetic public Fp^3 fixtures because this experiment isolates the
existing cubic-extension oracle on identical inputs; the standard panel does
not supply this generated cubic-extension family. This is a component/family
comparison, not a panel result. Freeze a fresh paired plan and source manifest
before timing. Existing local rho remains an explicitly limited reference;
no strongest-reference, parity-tier or leaderboard admission is implied.

Conductor is unavailable (command not found); existing authorized cubic IC,
example and study scope applies. No cloud spend or new service required.

## Frozen full cold comparison

Stage16: primes127/523/1039, fresh seeds139/149/151. Explicit oracle order
prepared-pair,pseudo-pair; 540 executions (90 warmup, 450 measured), with
pivot_a/graph/pivot_b/rho_a/rho_b reversed on odd rounds. Require exact
paired-oracle stream/residual/group/scalar controls. Use r-adding-long,
small-base ceil(|F|^(2/3)), at most two auxiliary variables, 500000 residual
cap, one retained warmup and five paired timing rounds, CPU0, 8GiB virtual
memory, 900seconds/process, zero retries. Both median A/A ratios must fall
within [1/1.10,1.10]; 10000 deterministic paired-round bootstrap draws.
Retain every failure, censor, incomplete stratum and control rejection.
All startup/base/precompute/walk/restart/oracle/elimination/replay costs are
inside each arm. Public fixture generation is shared supplied input outside.

Primary outcome is candidate/reference cold graph wall; report every stratum
and its controls, and the same-instance rho ratios beside it. A controlled
local speed improvement requires the direct ratio's upper interval below1. [pcc:unverified]
No composing old ratios or repeating unchanged cells to improve A/A results.
The local n>=2^30 diagnostic flag is not an admitted parity claim. A next
scaling experiment requires a new frozen holdout plan after this outcome.
No selection of seeds or sizes from candidate timings occurs in this plan.

Create-new local directory:
/workspace/ic-local-runs/local-pseudo-pair-20261116-16

Source/checkpoint/binaries and this plan must be clean, committed and published
before the wrapper launches. Numerical analysis remains native Rust. Exact
starts identify any unfinished arm; partial-arm time remains unknown. No
compiler may run during timing. The plan is pseudo-pair-plan.json with its
source manifest validation/pseudo-source.sha256. No runs started at freezing.

## Validation harness note

The default parallel S4 example tests exposed two exact work-count mismatches.
F4 documents that default per-report differences read a process-wide counter
and interleave under concurrent solves. The identical binary passes all five
tests with --test-threads=1. A test-only mutex now prevents overlap in the
default test command, retaining every equality assertion. Production code and
the sequential frozen driver are unaffected. The original failure and serial
diagnostic logs are retained; do not substitute a passing run for that history.
[pcc:unverified]

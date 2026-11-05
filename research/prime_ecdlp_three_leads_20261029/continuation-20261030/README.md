# G1–G6 continuation: independently check useful rank and target recovery

## 2026-11-05 terminal collection and separate continuation

PR2949 merged on2026-11-01 as8b147fd36e71990d9f44eb2cbfa435aa769cefc5.
Maintenance of that PR has stopped. All unpublished commits are preserved on
`codex/pkm-certificates-accounting-20261105`; no rebase or force-push.

The existing49719 run is terminal, not restarted. [Final receipt](validation/repaired-main-cc5addeba/final-20261105/receipt.json)
verifies17 unchanged source objects and all6 actual binaries. Repaired-source
23examples, strict lint/format, CLIbuild/verified checkpoint refusal,
backlog and library build pass. Full5848-case execution20:48:46–21:28:46UTC
on2026-10-30 hit40minutes:2272passes,1failure,48ignored,1JVactive,
3526not reached. Wrapper/supervisor124; Cargo exit unknown, no final summary.
A focused existing-binary run reproduces the inherited degree-bound assertion
(Some190568 versus None) without modifying assertions or tests. The source file
is identical to maincc5addeba. The delimiter compilation error is repaired;
the separate assertion and JV limits keep full-suite validation incomplete.
Raw221629bytes SHA25641ca32cee963b13adb6b5638abc52191de20f676b038874957a6b06e5e5ebebf
remains NOT_UPLOADED; exact local path and bounded excerpt are in the receipt.
No scientific acceptance has changed. The next implementation retains validation
reference counts and explicitly emits query field/reduction counts with ownership.


**20:25UTC:** [23 repaired-source example tests PASS](validation/repaired-main-cc5addeba/examples-pass-20261030T2025Z/receipt.json); zero failures/ignored.
Remaining native phases are pending in the existing run, not passed. No scientific result.

2026-10-30. **All six scientific goals remain incomplete.** This is native
checker and proof development, with no scientific panel or admitted performance
result. The original acceptance criteria in [PKM_GOALS.md](../PKM_GOALS.md)
remain in force. [Goal state](goal-state.json), [proof map](proof-map.md),
[diagram](scalar-certificates.svg), and [PDF](report.pdf) accompany this record.

**Latest20:04UTC:** [repaired main b022](validation/main-cc5addeba-resolution.json)
preserves both histories and the prepared handoff once. A [new native run](validation/repaired-main-cc5addeba/README.md)
is compiling, not passed. Formal objects exactly match the passing18/zero-trusted
audit; no redundant formal run is claimed. [Current report](repair-20261030T2004Z.md)
and [PDF](repair-20261030T2004Z.pdf) preserve all earlier reports and failures.

**Historical integration,18:37UTC:** [mergea030](validation/integration-0cc5-resolution.json)
preserves both histories and frozen evidence. Its changed Lean gate passes18
declarations/zero trusted, with module replay against imports. Fresh native
validation stopped at inherited compilation error101 before tests; see [current validation](validation/integration-0cc5/README.md)
and [integration report](integration-20261030T1837Z.pdf). Earlier faade library
execution capped incomplete at2208pass/0fail/47ignored/1active/3390not reached.
The [G2 gap recheck](G2_RECHECK_A030.md) is source review, not a counter fix.

**Historical17:03UTC follow-up:** [23 integrated example tests and all scoped checks pass](validation/scoped-20261030T1703Z/receipt.json).
The 5646-case library execution was then active; its terminal cap is below.
Those results bind sourcefaade, not newer remote0cc5. [Remote merge review](validation/remote-0cc5-review-snapshot.json)
preserves all179 earlier study blobs and both parent logs; eight bound
source/dependency paths changed and need affected validation after integration.
The [G2 accounting audit](G2_ACCOUNTING_AUDIT.md) identifies discarded validation
counts, serialized query-count gaps and overlapping high-level/expanded counters.
Its closure map is implementation work still owed. [Follow-up report](followup-20261030T1703Z.pdf)
records the new status; the initial report/receipts are preserved unchanged.

## What changed

The cold toy control previously replayed rank with the producer's elimination
implementation. Replay now uses a separately written rightmost-pivot row space,
integer Euclidean inverses and sparse combinations of original rows. It checks
coefficient and augmented ranks, rank-increasing yield after every query, and
the shape and membership of the producer's final RREF. It then emits either:

- **Recovery:** weights on retained original rows that isolate the target
  coordinate, including its right-hand side. These weights are checked by
  directly summing the original rows, independently of echelon reduction.
- **Unresolved:** a target-normalized kernel vector, checked directly against
  every original coefficient row. This proves the retained scalar equations
  leave that coordinate unresolved. It does not say the curve has multiple
  discrete logarithms or that further relations cannot recover the target.

Actual affine point equations and the final scalar equation remain independently
checked. The request and producer record stay `pkm.toy-dlp-request/v1` and
`pkm.toy-dlp-output/v1`; the replay certificate becomes
`pkm.toy-dlp-replay/v2`. Old producer records remain readable. The producer,
setup, relation encoder and producer timing code are byte-identical to reviewed
head `94e243dcc56b58dc78b0cc597b08b3a5afc04970`; only replay, certificates and
tests change. Source commit: `faade165a46fd51e5a0903ba8e4df85a134c68db`.

The independent checker shares request/setup and relation encoding with the
producer, and uses the existing separate ordinary affine reference. Its scalar
row-space implementation does not call producer `Elimination`, producer field
helpers, counters or the library's linear solver. Independent algorithms reduce
shared-implementation risk; their Rust semantics are not formally verified.

## Scientific status and next discriminating work

| Goal | Current status | Contribution here | Acceptance still owed |
| --- | --- | --- | --- |
| G1 faithful oracle | Partial | Replay rejects forged rank, RREF and scalar records; supported output collector unchanged. | Frozen multi-curve m=4 complete-output qualification, target laws and positive/miss/cap controls. |
| G2 complete cost/yield | Partial | Useful rank is independently recomputed; recovery weights expose which original rows suffice. | Full arithmetic/logical coverage, durable failed-process records, residual pricing at 0/0.1/1 and admitted measurements. |
| G3 attribution | Partial | Replay interfaces apply to existing presentations; no new external arm. | Equal-domain chain/plain/symmetric ablation, pinned external solver/extractor, A/A and null controls. |
| G4 improve a component | Unrun | No optimization or gain promoted. | Freeze a complete equal-output baseline; measure one held-out candidate under unchanged thresholds. |
| G5 scaling/reach | Unrun | No larger-cell measurements. | Predeclared size/depth panel, retained censoring and held-outs. |
| G6 cold DLP/proof | Partial | Independent rank, recovery/ambiguity certificates; five standard Lean contracts. | Qualified native runtime/resume, growing-family matched cold references and Rust/polynomial/complete-cost bridges. |

These are prerequisites toward the user's complete-cost question, not replacement
goals. `comparison_admissible=false`; all observed cost-ratio, GAE, rho-gap and
scaling-exponent fields remain unset. No measured timing comparison was made.
Replay checking occurs outside the producer's diagnostic cold interval; its
work is not silently included or free. Existing incomplete F4/root/logical
accounting and whole-process/startup/serialization gaps remain explicit.

## Verification and domain

The toy producer remains limited to coordinate primes p<=257, independently
enumerated whole-curve prime order n, a nonzero generator and finite targets.
The m=4 public fixture is E/F17: y²=x³+2x+2, #E(F17)=19, P=(5,1), Q=(6,3).
The producer receives no secret target or factor-base logarithms. Scalar rows
are over F19 in that fixture, not the coordinate field F17. The independent
scalar module accepts prime n<2³² and 1..512 columns; this wider unit-test
domain does not expand the producer's supported curves.

Three direct native tests pass, including all 729 two-by-three matrices over
F3, exhaustive vector-membership checks, sparse recovery and kernel witnesses,
all row orders of a fixed F19 system, near-32-bit residues and corrupt/noncanonical
certificate controls. Strict scoped clippy passes after correcting one lint.
Five new Lean contracts build with only standard foundations. Fresh final
proof audit, kernel replay and integrated native outcomes are recorded separately
in [validation](validation/README.md); running work is never a pass. Earlier
failed elaboration and earlier three-contract audit remain historical evidence.

The fresh detached native recipe binds 17 committed source/fixture objects and
checks tracked cleanliness before each phase. It covers all three examples,
strict lint/format, CLI build and expected checkpoint refusal, backlog validation,
library compilation and a 40-minute serial release-library execution cap after
the build. Exact recipe/bindings are retained in validation. Historical 5553-
and 5646-case capped runs remain unchanged and cannot validate this new source.

## Literature, mathematical scope and novelty

PKM 2016 Algorithm 1 and §3.2 already separate relation search from linear algebra
and allow extra relations for dependence. Permutation symmetry is already in
Algorithm 1. Standard row-space/provenance and nullspace certificates are not a
new ECDLP algorithm. This continuation's contribution is an independent native
replay path and concrete contracts in the existing symmetric-PKM instrument.
It supplies no cheaper relation algorithm, global priority or exponent claim.

The [proof map](proof-map.md) gives exact hypotheses for the five new theorems.
Mathlib constructs cyclic scalar coordinates from generation and exact group
cardinality; a nonzero point generates an exact prime-order group. Recovery and
ambiguity contracts expose their original-row hypotheses. Rust parsing,
arithmetic/curve representation, the summation-polynomial ideal correspondence
and complete solving/extraction-cost bounds remain open. These are conditional
group/linear-system theorems, not a newly discovered polynomial identity;
`identity.certificate/v1` (unconditional equality over a fixed field) does not
encode these hypotheses. No such identity claim is promoted here.

Sources checked before implementation:

- [Petit–Kosters–Messeng, PKC 2016](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf), Algorithm 1, §3.2: relation dependence and LA are material; symmetry and the tower framework are prior work.
- [Local tower review](../../notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md), §3.3–3.7, and [novelty audit](../PKM_GOALS.md): complete extraction and total cost, not first-fall degree alone, decide the lead.
- [Existing linear algebra](../../../src/cryptanalysis/ic_framework/linalg.rs): existing dense/sparse infrastructure is reused conceptually, not called by the new reference checker.
- Mathlib revision `d13f23b723b8a846827a245b89c10fc7d3f11612`, `GroupTheory/SpecificGroups/Cyclic.lean`: `zmodAddEquivOfGenerator` and prime-cardinality subgroup dichotomy supply actual scalar-coordinate construction.
- [Existing formal row contracts](../lean/ThreeLeads.lean) and [affine chain bridge](../lean/ThreeLeads/PKMChainCoverage.lean): standard algebraic dependencies, with the remaining bridges stated explicitly.

## Execution and publication limits

The managed runtime reports no configured lab/artifact identities, secrets,
runtime variables or execution capabilities. The unchanged checkpoint gate
refuses for missing transport. No scientific panel, lease, funding, IAM, quota
or pool change occurred. The full-panel card remains blocked/unfunded at its
historical pin. Existing approved access and a source-bound qualified native
runtime are required before frozen panels may run.

Reviewed merge `94e243dcc` against both parents. All PKM source/input/report/
receipt blobs were preserved, but 91 append-only log paragraphs had been lost.
Restored 17 complete original entry blocks and verified both parent histories;
see [resolution review](resolution-review.json). Old receipts are unchanged.

Canonical `docs/index-calculus-scoreboard.html`, `docs/ic/progress-timeline.json`,
`docs/curves/` diagrams, `docs/performance-gains/`, `figures/` and existing study
figures stay unchanged: there is no admitted numeric result, new curve or
isogeny edge. This round's explanatory diagram records the new checker path.
No benchmark entry or plot implies a speed result. Software integration and
proof-audit success do not complete G1–G6.

## Terminal collection at18:29UTC

The [final pinned-source receipt](validation/native-final-faade/receipt.json)
records the40-minute cap:2208passes,0failures,47ignored,1JVcase active and
3390not reached among5646. Wrapper/supervisor124 is not Cargoexit; no full
library pass exists. The214615-byte raw log is NOT_UPLOADED with new SHA256
`34fdd95ea133361298e8911f9a5899c431a5e9af97b73e7828e1b5289cdf87bd`.
All17source bindings and six binaries still match. Earlier running snapshots
and reports remain dated history. Newer remote0cc5 still needs integration
and fresh affected validation; scientificG1–G6 status remains unchanged.

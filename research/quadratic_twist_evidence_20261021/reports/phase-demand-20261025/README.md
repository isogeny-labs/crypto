# F43 setup and recovery demand: actual phase accounting

2026-10-25 · PR2381 · run `phase-demand-20261025` ·
[protocol](../../PHASE_DEMAND_20261025_PROTOCOL.md) ·
[pre-execution counter contract](../../PHASE_DEMAND_20261025_COUNTERS.md) ·
[PDF](report.pdf) · [quantitative graph](demands.svg) · [pipeline](bridge.svg) ·
[canonical results and independent receipt](../../native/phase-demand-20261025/) ·
[Lean recovery theorem](../../lean-phase/PhaseRecovery.lean) ·
[reproduction and attempts](../../validation/phase-demand-20261025/README.md).

The new native run executes all **40 role/policy setups and 400 separate
nonidentity main-source recoveries**: ten frozen F43 size-two base policies,
four roles, ten source points per role/policy. Every source scalar is verified.
Independent Rust reproduces every phase-request vector, base-log solve,
empty query, sign lift, recovery shift and answer. A second batch-RREF and
witness audit checks all100 distinct solver certificates used by these runs.
The fresh replay matches canonical bytes and rejects six rehashed false
payloads without an audit receipt.

This round is **accounting**: it replaces an unpriced assumption about what
lower degree saves with exact demand records for the named pipeline. The
29 counters have different and sometimes nested units. They are not added
into a fictitious scalar time or GAE. Hashing, fixed finite-field table
construction, allocation, sorting, map traversal, internal exponentiation and
independent audit remain unpriced. Sparse code's represented dense column
slots are a structural metric, not an allocation or memory measurement.
Complete attack cost and a speed claim remain open.

## What the first policy actually demands

The main model is `icv1-fp6-t0-74779950`, E/F43:y²=x³+x, P=(31,18),
and the monic twist is `icv1-fp6-t0-c6a8a47c`, T/F43:Y²=X³+4X,
U=(24,26). Both selected subgroups have order11. Prefix bases are main
(4,5),(13,19) and twist (15,9),(17,6). The generator U is explicit; the
older generator's EC1 alias is not reused.

| Prefix policy, source Q=P | Main route | Twist route |
| --- | ---: | ---: |
| Setup solver calls to reach full coefficient rank | 3 | 1 |
| Online solver calls including failed shifts | 2 | 1 |
| Setup + online + verification solver calls | 5 | 2 |
| Represented dense matrix column slots | 540 | 36 |
| Point-add requests, all branches included | 125 | 136 |
| Membership-polynomial evaluations | 645 | 258 |
| Coordinate-map multiplications | 0 | 2 |

The first-target reduced-S3 degree/rank observation from the preceding round
survives, and the new ledger exposes its other demands. In particular, fewer
solver requests and matrix slots coexist with more point-add requests. More
returned roots require more signed actual-point lifting. The complete
ten-target graph retains the later losses; it is not a timing graph.

Across100 unmatched main/twist policy-target comparisons, twist cold solver
calls are lower/equal/higher in **35/36/29**, represented matrix slots in
**44/15/41**, and point-add requests in **49/1/50**. These are exact finite
comparisons of repeated policies and targets, not independent curves or
probabilities. The sum of hypothetical cold solver calls is393 in both
presentations, and represented slots sum to75,804 in both. Those are summaries
of100 separate cold records per arm; only40 target-independent setups were
actually executed, and no multi-target amortized speed average is admitted.

Transporting a main base to the twist, or pulling the twist base back to the
main curve, reproduces **every recorded component except the two coordinate
map multiplications per nonidentity input**. This is a finite request-profile
control. It does not prove general algorithm-cost invariance for every curve
or implementation.

## Actual pipeline and formal recovery

Setup queries mG in order until the relation coefficient matrix has rank2,
solves its augmented system and verifies both resulting base logs on actual
points. Each separate target then passes through the previously proved
injective additive map phi(x,y)=(41x,11y) where applicable, tries explicit
shifts, runs the entire certificate-seeking solver, lifts all returned roots,
recovers the integer combination of base logs and verifies it on the main
curve. Source labels generate and check the known-answer public fixtures;
the recovery function receives the source point alone.

For an injective additive map φ, verified base equations Bi=li·φ(P), and an
actual signed relation

    i·B0 + j·B1 = φ(Q) + s·φ(P),

Lean proves Q=(i·l0+j·l1−s)·P. Reducing that integer modulo r additionally
uses the explicit source annihilator r·P=0. The theorem is instantiated on
the actual frozen F43 map and checks 11·P=0 in the kernel. It does not assume
a map between arbitrary base-field rational main/twist groups.

| Claim | New Lean declaration |
| --- | --- |
| Exact shifted signed-relation recovery | `PhaseRecovery.shifted_relation_recovers` |
| Valid reduction of the recovered integer modulo an annihilator | `annihilator_mod`, `shifted_relation_recovers_mod` |
| Actual F43 source annihilator and instantiation | `F43.source_annihilator`, `F43.actual_recovery_mod11` |
| Every native hypothesis and phase count | Independent Rust replay, plus the old batch-RREF/witness auditor |

The unchanged scoped repository gate passes **176 public research theorems /
231 audited declarations**, with zero failing/extra trusted axioms. Only
propext, Classical.choice and Quot.sound occur. The five new group theorems
do not introduce a new polynomial identity; all14 old source-bound identities
and their sources remain unchanged. Full proof of the native request-accounting
algorithm itself is not claimed.

## Next lead and records retained

Every solver query in this fixed two-root setting records129 membership
evaluations before its four reduced-S3 checks. A concrete next implementation
lead is to supply the already verified factor-base roots directly, with a
proof that this preserves the exact constrained solution set, then charge the
changed pipeline under the same policy/target workload. This is a proposed
optimization, not a result of this run. Low-level calibration and applicable
external-reference comparisons are still needed before a speed claim.

All earlier studies, sources, Lake inputs, registry, catalog provenance and
benchmark history remain frozen. The full library suite,22 backend tests,
four workflow parser tests, hash checks and source-linked inspected PDF are
recorded in validation. The first local Lean build's outdated division-lemma
name and exact failed source/log are retained. Shared Conductor still exits127;
no reservation or replacement coordinator is claimed. PR CI/review/integration
and broader normalization, exceptional lifting, generic pairing, trace-zero,
useful-yield and complete-work gates remain explicit.

Canonical scoreboard and performance graphs stay unchanged because no admitted
speed or calibrated total-cost metric is added. This round's graph and pipeline
diagram record the new request accounting. The atlas was consulted at
crypto-autoresearcher5998553341fb29d3b9ca1a4c9ac6e58d96ca3ffa; relevant
charged-IC, presentation-invariance and closure-degree leads are cited in the
protocol, without an invention claim.

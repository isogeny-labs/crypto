# Four collection-architecture prototypes

All four bounded native arms completed on the fixed two-fixture plan; a second
call reproduced the complete report bytes. Producer assertion
[pcc:PCC-20261123-4a854c], standing **unverified / not_covered**. No speed,
full-cost or exponent result is established. This is ordinary affine
architecture qualification, not production APS/F4 integration.

Source pin: [e791ac91f7535cdc7bcd2c0ee09aa3c1af523621](https://github.com/isogeny-labs/crypto/commit/e791ac91f7535cdc7bcd2c0ee09aa3c1af523621),
base 015f50a6526b0ca85ad4556dc62fa20900e4723c. The source and lock digests before
and after the call agree. [Custody](custody.json), [literal report](qualification.json),
[transport-only control](transport-control.json), [frozen plan](PROTOCOL.md),
[normalized checks](validation/checks.json), [print report](report.pdf).

## Actual scoped observations

| Route | F17 / order 19 | F101 / order 107 |
| --- | --- | --- |
| One-large-point, one target | 164 matches / 512 draws; check scalar 7 | 32 / 512; check scalar 7 |
| Two-large-point, one target | 162 / 512; check scalar 7 | 28 / 512; check scalar 7 |
| Coupled three-target graph | Both routes return [7,11,16] | Both routes return [7,11,16] |
| Recursive cold/shared/direct | Complete tuple sets 5,7,2 on the three targets | Complete tuple sets 5,7,2 |
| Frozen partition: useful fresh relations | Selected [1,0,1], fixed rule [1,0,1] | Selected [0,0,1], fixed rule [0,0,1] |

These are complete outcomes of tiny known-answer controls, not independent
random samples or speed measurements. Partial attempts are deliberately
repeated in pairs; unmatched draws and graph-dependent rows remain in the
report. Partial construction is an affine matching baseline, not a new
smoothness test. [pcc:PCC-20261123-4a854c]

The F17 pilot selected rule 1 (x modulo four), tied in pilot volume/yield with
rule 3; fixed tie order selected rule 1. F101 had zero useful pilot outcomes
for every rule, so the explicitly flagged fallback is rule 0. It is not
evidence of useful learning. No held-out target was used to train or retune.
The complete unpartitioned roots and omitted relations remain beside the
admitted ones. [pcc:PCC-20261123-4a854c]

Each recursive shared table constructs all 28 pairs for eight index slots.
This setup is charged once to the batch row; cold rows rebuild it per target.
Every returned index witness is retained. This is standard meet-in-the-middle,
not a new asymptotic algorithm. The library separately checks complete tuple
sets on every target in both groups, including identity and opposite-pair
cases. [pcc:PCC-20261123-4a854c]

## Validation boundaries

Final source Rust/lock bytes passed 10 library and 2 CLI tests on Rust 1.98.0
and 1.99.1; strict Clippy and formatting passed on both. The actual-wrapper
transport control intentionally fails four mock arms and retains all four;
its process success does not count as scientific completion. Dedicated source-
pinned release build and qualifier exited zero. These are producer checks,
not signed independent verification. [pcc:PCC-20261123-5fb8f5]

Initial library run was 9 pass / 1 fail for missing duplicate-row coverage;
initial strict Clippy failed on an index-only loop. Both original logs and
the failed relative-path PCC lookup (exit3) remain private. The later native
PCC-ID lookup returned not_covered/exit2, not verification. Native graph and
curve dependencies were checked against the public main pin before building.
No current-root full-library, production APS/F4, ecbench, external-solver,
timing, performance, scaling or EKS result is claimed.

The literal report is custom `aps.collection-architecture-qualification/v1`.
It is not represented as a conforming canonical `experiment.export/v1`.
All local raw build/dependency logs and raw verdict host metadata remain
private; public checks are normalized producer receipts with original digests.

## Requirement closeout and next implementation

| Requested lead | Status | Next concrete integration |
| --- | --- | --- |
| Large-prime partial relations | Bounded adapter implemented; production partial generation open | Emit exact one/two-residual point witnesses from APS decomposition, retaining acquisition failures |
| Recursive shared intermediates | Exact two-level bounded route implemented | Plug support construction into the production builder and compare with its existing direct join |
| Multiple targets/shared base | Coupled symbolic target columns implemented; shared fixed small base | Use a single native labeled-base sampler with per-target coefficient vectors; preserve cold vs batch charging |
| Pilot-frozen conditional partitions | Bounded rule selection implemented; no gain on this fresh panel | Add actual elimination-cost features and charge training before a preregistered disjoint comparison |

Full phase/bit-operation/memory accounting, canonical replay/export, the
growing-family cost/yield model, frozen size/power/budget/held-out protocol,
prospective checkpoint and run admission remain open. No broader scientific
goal is closed by this component. The source S3 approval persists, but the old
checkpoint does not cover this source or a future protocol. No benchmark or
pilot was launched.

Native Conductor sign-in failed to the existing service. Work remained within
the user's existing native implementation/bounded correctness exception; no
substitute lease or service was created. KB access was unavailable; sources
and their assumptions are recorded in the plan. Auto-merge and merging remain
held; no private ledger or shared full-log publication is included.

# Inspected source and proposed changes

Read-only inventory on 2026-10-30 in `/home/ec2-user/crypto-ml-sat-prefix-20261030`.

| Source | Existing behavior and exact gap | Minimal proposed change |
|---|---|---|
| `src/ml_dataset/types.rs:6` | CaseConfig has size but no policy and denies unknown fields. | Explicit versioned case envelope and internal normalized CaseSpec; preserve old bytes. |
| `src/ml_dataset/mod.rs:251,299,326,389,404,941` | Plan/CLI, validation, count/index and construction assume v1 and one size. | Explicit v2 parser/serializer, policy axis, checked multiplier, most-significant policy digit, legacy omission behavior. |
| `src/ml_dataset/mod.rs:1258,2491,2665` | Worker/cell admission and replay depend on typed config and v1 schema. | Version dispatch and binding; no bypass of curve/base replay for v2. Audit start/preflight/budget/restore for the same identity. |
| `src/ml_dataset/curves.rs:338,383,624,747` | Existing projection, diagnostic cyclic membership, exact model/twist construction and generator/target sampling. | Reuse arithmetic and setup; extract policy constructor without coupling target sampling RNG to policy. |
| `src/ml_dataset/curves.rs:820` | Small-x projected lifts, both signs, possible one-point size overshoot. | Preserve legacy path; add whole signed-Frobenius-orbit admission with separate cap semantics. |
| `src/ml_dataset/curves.rs:855,859` | Population action may be null; separately enumerated full orbits do not prove population closure. | V2 total population permutations and complete signed-orbit partition/stabilizers. |
| `src/ml_dataset/curves.rs:1145,1210` | Independently verifies actual twist coefficients and minimal coefficient field. | Reuse; explicitly verify subgroup stability for the new action. |
| `src/ml_dataset/curves.rs:1309` | Strict v1 rule; reconstructs candidate sequence, points, maps and Frobenius-only orbits. | Keep v1 verifier; independent v2 reconstruction with policy expectations, status/budget boundaries, signed-action witnesses. |
| `src/ml_dataset/solvers.rs:450` | Concrete domain adds/deduplicates signs; actual order may differ from storage. | Bind concrete point and x-domain index maps; no inferred backend support. Reserved path requires coordination. |
| `src/ml_dataset/sat_prefix.rs:581` | Replay reconstructs exact finite coordinate domain. | Bind v2 verified population/domain; retain causal/unknown-prefix rules. |
| `src/ml_dataset/export.rs:188,239` | Strict v1 structural allowlist, with public point/orbit projections. | Explicit v2 allowlist; preserve historical feature bytes, restrict construction/provenance and avoid duplicating mathematical inputs. |
| `src/ml_dataset/export.rs:263,299` | Separate Kummer and large-prime mathematical domains. | Keep them distinct; verify action/domain substitutions and quotient coefficients explicitly. |
| `src/ml_dataset/export.rs:495,1384` | Conservative whole-field-size split, cross-size links unmerged. | Preserve all new policy variants within same split; do not claim cross-size dedup solved. |
| `tools/factor-base/src/recipe.rs:12,20,135` | Existing subspace, explicit, orbit, progression/random recipes; separate API/schema. | Reuse in later explicit integration after semantics/caps checked, not copy code or rename projection as filtering. |
| `research/ml_dataset_sat_prefix_20261030/COVERAGE-RECONCILIATION.md` | Current broader-method coverage and unresolved cloud/cost/label gaps. | Keep full requested axes and update only after exact executable evidence. |

Exact inspected hashes (SHA-256):

```text
1eb9bc15b483e7652a2cd8368c1eb9b4193c16a2105397135ca60c9f042e6637  src/ml_dataset/curves.rs
5181528c4ec19d4e10ae1ce31c3c8950fb4184921019ed9cc25e8ac85bf25f5d  src/ml_dataset/types.rs
1128d5716835d83cd9c2cf1216c56bf1ccf182119e57af503aad84e43f07598c  src/ml_dataset/mod.rs
ea72eb0f45091316a07fafc7d162d7503351c848ae34b79a4f7dbdb934cc0bdf  src/ml_dataset/export.rs
```

These are source observations, not an integration claim. No repository file was
edited. Parent owns check-in, reservations, independent review and any later
official experiment protocol. Scratch draft files are ready for parent review.

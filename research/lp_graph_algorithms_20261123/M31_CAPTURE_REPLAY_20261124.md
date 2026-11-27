# M31 retained-capture replay — preregistered development scope

User target: ordinary elliptic curves over M31, p=2^31-1=2147483647. Continue existing PR5076. This step consumes an existing M31 collector output; it does not recollect relations or introduce a new timing experiment. No paid compute or merge. Status before implementation: not executed. [pcc:unverified]

## Frozen input and ownership

Producer PR5097 owns the native generic four-sum source. Pin repository isogeny-labs/crypto, commit577d6c2c1c4a527604d965e6fb62c44df7902f36, file research/prime31_graph_ecdlp_20261123/evidence/mitm-b677-control-01/ten-query.json. Git blob7fd27b9888058399d8814cef373d373243e6a1e5; 165511 bytes; SHA256 e2407b695d4df7a7c9d71eae00895a643aebe3afab50d1c338e58778884dabab. Retain the producer source, stderr, source-hash list and complete input unchanged.

This is an exhaustive-pair baseline on fixed public aG+bQ queries and a known-answer public target. Rows were found by search, not constructed from a selected decomposition. It is not blinded algebraic extraction and supplies no new performance evidence. The registered a=-3,b=21 M31 curve in the other part of PR5097 is a different instance; do not mix it with this capture.

Selected instance: p=2147483647, a=847514846, b=2087444724, prime group order ell=2147446831, G=(625037715,2100627251), Q=(976979610,1228979761). Keep the target's recorded orientation. Scalar/relation arithmetic is modulo ell, not p.

## Conversion and independent replay

Add one native Rust example to the existing prime31_ic_20261115/native crate. Reuse unchanged Arithmetic, Base::build and the original b677_fixture.rs via a source path. Reconstruct the exact 677-point sorted union from the Dickson B509 and frozen Y7 extra168 CSV. Fixture SHA256 3f491defbc7aa1643192db4d0c7144afcec1f84a41f03489355a866459329624; CSV SHA256 fc484615394ee423190747d05001cea526580fb0df0279fbcf789ab6cafc0c46; original native lib SHA256 52b30e41478b0306603db7d09bcb17377cae64efcfd78d4c34db7bf91325a867. Use original Base::build to classify base membership rather than copying its Dickson recurrence.

Validate exact instance, source scope, seed, dimensions and fixture certificate. Validate unique labels, canonical scalars and their recorded points R=aG+bQ. Replay every signed four-point equation, including rows later excluded for arity. Preserve original global relation_rows ordinal as row id, original sorted point index as column id, labels and a/b. Signs +1/-1 become 1/(ell-1). Ignore producer rank, recovered scalar and recovery-result fields for conversion and verification.

Emit the existing prime31-lp-capture/v1 input only after all rows pass. Do not insert a target column: the unchanged group-admission example writes its protected target column. Emit a separate audit recording query joins, every excluded row, partition and reconstructed point-table identity. Unexpected bad metadata, signs, indices, labels, duplicate rows or group equations are whole-input refusals, not exclusions.

The producer reports645 rows in classes [2,30,138,276,199] by factor-base term count. Recompute this from reconstructed points. Keep613 rows:199 zero-LP,276 one-LP and138 two-LP. Preserve all32 exclusions:30 three-LP and2 four-LP. Expected per-query kept counts are [50,51,70,61,68,68,62,54,66,63]. These are fixed comparison literals, not newly measured results.

## Bounded execution and outputs

Use the existing single source-exact graph CI job, unchanged runner/permissions/concurrency/12-minute limit. Add one bounded saved-data replay step: one build thread, committed native Cargo.lock, 4GiB address-space limit and300-second outer deadline. Fetch only the exact producer commit with submodule recursion disabled, extract the named input with git show, and verify bytes/hash before execution. Build the new converter and copy/hash it before running; retain the previously qualified original admission and graph evaluator identities. No collector executable is invoked.

Run native conversion and unchanged group admission. Feed the unchanged graph evaluator the first256, first512 and all613 admitted rows, in source order, with modulus2147446831 and exact-forest cap4096. Retain native useful/augmented ranks, LP rank/nullity, source replay failures, residual/source nonzero counts and explicit refusal dispositions for forest/pivot/MST. Do not interpret their timing fields. The producer's bespoke rank452 is a comparison observation, not a predeclared result for these consumers. No1024/4096 checkpoint exists in this input; do not manufacture one.

Use corrupted copies of the same M31 input to exercise refusal of a changed field, a changed query coefficient and a changed row sign. Each must exit nonzero with empty capture stdout. Keep all commands, complete stdout/stderr/exits, original input, converted capture/audit, admitted JSONL, derived point tables, graph reports, binaries and before/after source/input/binary hashes in the existing always-uploaded artifact.

## Evidence boundary and next gate

This is development analysis of retained results under AGENTS9a, not a new EKS experiment or speed measurement. Source custody lacks per-probe reject traces and the original collector binary/toolchain attestation. Its arithmetic counter also omits the fixture's separate setup counts and bespoke graph arithmetic. Record those limitations unchanged. Native consumer replay can establish group/linear-algebra consistency, not fill these custody or complete-cost gaps. No extrapolation from graph counts to cold ECDLP speed.

Scope: the new converter example; a new replay shell script under native_gain_dsu; an additive step in the existing graph workflow; this study's protocol/receipts/handoff and scoped session log. No edits to the producer, RR5 extractor, fixture, Arithmetic, Cargo manifests/locks or production graph kernels. KB and Conductor are unavailable here; current source review and three disjoint read-only workers supply the documented fallback, with no shared lease asserted.

# M31 solver-group manual invocation: source and interface audit

This bounded invocation performed current-source inspection and retained-input custody checks for Cheap ECDLP, Chained ECDLP Transfers, and Graph IC. It did not execute a new cryptographic experiment, benchmark, or independently signed scientific validation. All scientific interpretations below remain [pcc:unverified].

## Exact M31 instance and source separation

The retained B677 capture uses p=2147483647, a=847514846, b=2087444724, subgroup order r=2147446831, G=(625037715,2100627251), and Q=(976979610,1228979761). These values were read from the exact saved capture and its preregistration. The current native receipt checker uses the corresponding PRIME31 arithmetic and scalar modulus r. This is distinct from the other ordinary M31 curve y²=x³−3x+21, order2147487637, appearing in the same producer PR. Sharing p does not make the curves or scalar fields interchangeable. [pcc:unverified]

Sources: [saved-capture protocol](https://github.com/isogeny-labs/crypto/blob/45eb2ec79bc11168dc434e8a235ea358cf674f9f/research/lp_graph_algorithms_20261123/M31_CAPTURE_REPLAY_20261124.md), [producer PR5097](https://github.com/isogeny-labs/crypto/pull/5097), and [native receipt source](https://github.com/isogeny-labs/crypto/blob/81f0308c75204f3a1c0ec52fdf78396eee0898ed/research/cheap_ecdlp_cold_gate_20261123/src/bin/rr5_receipt_check.rs).

## What was checked here

The exact ten-query input was downloaded from commit577d6c2c1c4a527604d965e6fb62c44df7902f36:

- Path: research/prime31_graph_ecdlp_20261123/evidence/mitm-b677-control-01/ten-query.json.
- Size:165511 bytes.
- SHA256:e2407b695d4df7a7c9d71eae00895a643aebe3afab50d1c338e58778884dabab.
- Git blob:7fd27b9888058399d8814cef373d373243e6a1e5.

Local sha256sum and git hash-object matched the preregistered identities. This establishes retained-byte identity at this observation; it is not a group-law, rank, or execution attestation. [pcc:unverified]

| Item | Status in this invocation |
| --- | --- |
| Exact capture bytes and hashes | Checked against the preregistration |
| Ten queries and645 stored relation records | Read from the retained JSON structure |
|613 eligible rows and32 arity exclusions | Protocol/producer assertions; native recomputation is owned by PR5076 |
| LP rank161 and projected rank452 | Producer fields; not independently recomputed here |
| recovered_scalar=null in the ten-query arm | Observed recorded result |
| Eleven-query707-row/669-eligible recovery in PR5097 | Separate producer-reported cohort; not substituted for the ten-query input |
| Curve-group replay and exact native projected rank | Pending in the existing PR5076 continuation |
| Complete cold comparison with strongest IC/rho | Not established by this audit |

The prior capped uniform-query producer bound (about0.003906317 expected rows) refers to a different residual-pair mechanism. The new retained exhaustive-pair source must be evaluated at its actual method and costs. It is a generic-decomposition control, not nonenumerative algebraic extraction. [pcc:unverified]

## Cheap ECDLP: concrete native interoperability contract

At main81f0308c75204f3a1c0ec52fdf78396eee0898ed, rr5_receipt_check.rs blobd3ebbe79b09266acd541c3804c215c8d8d830a2e defines:

- Receipt.base: oriented public points.
- Receipt.target_q: the public target.
- Receipt.relations: original signed terms and canonical query coefficients a,b.
- Optional stop_when_target_determined and emit_dual_certificate flags.

Its signed_sum guard allows at most four signed leaves per input row. Graph projection can produce longer combined rows. Therefore the unchanged checker should consume the original four-term source rows with the full677-point table, rather than arbitrary graph-projected rows. Join each original relation label to exactly one source query and preserve the original source order, signed indices, and Q orientation. The checker itself constructs [C|-b|a] modulo2147446831; no second target column is added by the adapter. The full677-point input and the recorded645/707-row controls fit its existing opt-in B<=1024,M<=4096 caps. [pcc:unverified]

Keep the correspondence between checker row indices and original row IDs in a custody sidecar. The Receipt schema rejects unknown fields, so source/collector metadata belongs in that external binding rather than silently adding JSON fields the checker ignores. Do not feed the producer's recovered scalar or rank as an input. If prefix stopping is enabled, retain authenticated_rows and unchecked_tail_rows; a checked prefix must not be described as validation of the whole stream.

This is a source-derived transport contract, not an implemented or executed adapter. PR5076 already owns reconstruction of the exact point table and native same-stream conversion. Reuse its resulting source-bound artifact rather than launching another conversion or collection.

PR5099 now reports nineteen original checker tests passing as part of its exact8aa70e2 development bundle, superseding older uncompiled-checker status. Its scientific RR5 B677 extraction remains unrun. Its report names checkpoint transport, complete results-readback authorization, image publication, finite budget/live capacity, and separate root-library compiler failures. Those prerequisites were not bypassed or re-probed here. [pcc:unverified]

Sources: [PR5099](https://github.com/isogeny-labs/crypto/pull/5099) and [payload report](https://github.com/isogeny-labs/crypto/blob/0a3932b2801738c14fd166a7239e519a587e8346/research/prime31_four_summand_20261122/scaling-b677/payload-integration/README.md).

Impact score:3/10 — This run identified a concrete input incompatibility and a source-preserving path to reuse the native checker on the retained M31 control. No new native solve or cost reduction was measured.

## Chained transfers: use the existing M31 controls at their actual scope

Current-source review found that M31 work already exists in CT334–338: actual genus-two equations, rational bielliptic quotient maps, and native C++ Cantor homomorphism controls for an intentionally split Jacobian. Later CT340–343 results concern F103 controls and remain labeled as such. Neither group of inspected controls supplies the missing usable map from the ordinary M31 elliptic benchmark into a representation with lower complete solving cost. Replacing the scalar field, retitling F103 output, or treating a split-Jacobian positive control as an ordinary elliptic-source result would not discharge that obligation. [pcc:unverified]

Existing transfer owners and execution boundaries remain unchanged. No historical control was rerun in this review.

Sources: [current queue3904](https://github.com/isogeny-labs/crypto/issues/3904), [CT334–340 history in PR4997](https://github.com/isogeny-labs/crypto/pull/4997), and [F103 CT343 continuation](https://github.com/isogeny-labs/crypto/pull/5066).

Impact score:2/10 — The M31 applicability review separates existing valid M31 component work from F103 controls and preserves the exact elliptic-source transfer dependency. No new transfer or complete-cost result was produced.

## Graph IC: avoid duplicating the active retained-stream replay

At the inspected head3ba271b1f7100bdfd06eafbac8af283ec9d55094, PR5076 already owns one bounded native conversion/admission/evaluation of the ten-query capture at prefixes256/512/613. Its prospective protocol requires original row/query identities, full replay of excluded rows, exact target-column handling, and corruption refusals. It explicitly retains missing per-probe rejection traces and original collector binary/toolchain attestation, plus fixture-setup and bespoke-graph accounting gaps.

That active replay was not duplicated. This invocation supplies the exact input-byte receipt and the cross-consumer interface audit above. A successful eventual graph replay will qualify its reported group and matrix checks, not retroactively fill the collector's cost/custody gaps. No cold-speed ratio is inferred from the recorded counters or wall times. [pcc:unverified]

Source: [PR5076](https://github.com/isogeny-labs/crypto/pull/5076).

Impact score:2/10 — Exact capture custody and cohort identity are now checked for this handoff. The existing owner's native replay and a fully charged comparison remain pending.


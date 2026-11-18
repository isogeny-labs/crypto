# CT214 — raw-pair provenance and bounded exact-head CI route (2026-11-18)

**Status:** source and log-audit controls committed; native timing ratio NOT observed here. [pcc:unverified]

## Decision and evidence scope

CT212 native Rust recovery was observed in [run 44351488002](https://github.com/isogeny-labs/crypto/actions/runs/44351488002). CT213 added a 301-pair fresh-table toy BSGS cost probe, but at head aec4d453 no corresponding CT205 workflow was visible among the branch's last 100 workflow runs. Only pull_request_target artifact checks appeared. This is an observation, not a diagnosis of GitHub's trigger behavior.

The new CT214 change makes a **branch-only push** of this exact task branch eligible for the same 40-minute hosted workflow, without changing paid compute or opening a second research lane. It does not modify original 695/701 fixtures or the CT201 independent adjudicator. The existing pull-request and manual triggers remain. GitHub [event and branch/paths filtering documentation](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows) specifies the scope.

## Raw evidence required

The CT213 Rust integration test additionally emits 301 \`CT214_PAIR_JSON\` records with round, offset, target index, AB/BA order, per-arm real elapsed nanoseconds, per-arm BSGS additions and baby table sizes. Original aggregate \`CT213_TOY_COST_JSON\` remains unchanged.

\`ct214_cost_log_validator.py\` rejects missing, duplicate, moved or reordered roster entries; wrong per-target additions, invalid timing, mismatched aggregate sums, medians or ratio; and fabricated memory or PCC standing. Seven stdlib adversarial controls provide a precompilation smoke gate. Reproduce with:

\`\`\`sh
python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/test_ct214_cost_log_validator.py -v
cargo test --locked --release --test prime_cover_toy_cold_cost -- --nocapture 2>&1 | tee ct214-native.log
python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct214_cost_log_validator.py ct214-native.log
\`\`\`

This is **producer-owned log accounting**, not independent hardware attestation or PCC. The target roster is reused seven times, source target construction is outside both timers, source setup is preloaded while transferred setup is rebuilt, memory is unmeasured, and GitHub shared hardware is not isolated. A/A, paired confidence intervals and strongest rho, BSGS and IC comparisons are absent. The toy performance ratio cannot support a complete single-target advantage.

## Existing work preserved and next decisions

- Merged [crypto #3877](https://github.com/isogeny-labs/crypto/pull/3877), closed counter-only [#4215](https://github.com/isogeny-labs/crypto/pull/4215), current parent [#4697](https://github.com/isogeny-labs/crypto/pull/4697) and child [#4715](https://github.com/isogeny-labs/crypto/pull/4715).
- Existing [cryptanalysis #1523](https://github.com/isogeny-labs/cryptanalysis/pull/1523), [#1766](https://github.com/isogeny-labs/cryptanalysis/pull/1766), mirror [#1770](https://github.com/isogeny-labs/cryptanalysis/pull/1770) and GPU IC [#4178](https://github.com/isogeny-labs/crypto/pull/4178) remain the owners of original-model/independent full-solve and GPU IC work. No duplication.
- The original 695/701 authenticated model is still reported blocked by S3 HeadObject HTTP 403. Do not infer valid custody or physical C169, C13a, C13b, N2_actual and D costs from toy instrumentation.
- Next admissible physical experiment: authorize fixture retrieval, replay the immutable fixture, price actual norm-point reuse and C169 vs both C13 hops on one host, then execute blind one-target transfer plus destination IC and matched strongest rho/BSGS/IC including setup/LA, memory, uncertainty and signed PCC. Report losses and redirect, not just positive cases.

No paid EKS/Spot work, no protected merge, no speedup or independent signature is claimed. [pcc:unverified]

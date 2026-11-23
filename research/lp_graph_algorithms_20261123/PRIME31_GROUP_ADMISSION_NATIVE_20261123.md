# Prime31 graph admission: native correctness qualification

Status: preregistered development checks, not a relation-generation or performance experiment. [pcc:unverified]

## Scope and source

Continue PR #5076 after source `17db8809963a5570211328aa5e431fd1e6507ddd` and tracking `c1ed7fe73e46b3c27cfb4374a2bc981b3e190e26`. Preserve no merge/no paid compute. The original admission implementation and curve arithmetic remain byte-identical outside its test module. Reuse the existing `Graph-IC GainDsu source-bound native test` job and its runner, permissions, concurrency and existing pull-request/workflow-dispatch event model.

Scope expansion reserved before edits: the admission example's test module; a thin shell CLI qualification script; additive steps/path filters in the existing graph workflow; this protocol, lane handoff and scoped log. Do not alter collector, production graph kernels, rank algorithms, original Arithmetic, root parser, extraction workers, frozen evidence, secrets, or experiment admission policy.

## Fixed correctness controls

- Preserve the two existing tests.
- Add deterministic planted known-answer controls on the fixed original curve with LP arities zero, one and two, signed coefficients, b=0 and a negated public target with correctly adjusted b.
- Exercise canonical bounds, identity/signed residual cases, invalid mapping/target/source ID and late-row corruption.
- Rust tests emit a small immutable control bundle, including valid input, exact expected JSONL and a capture with an invalid final row. These controls are visibly planted and supply no natural relation-yield evidence.
- Build the actual example, compare its valid CLI output byte-for-byte with the expected JSONL, and require a failed exit and empty stdout on the invalid final row, malformed input, oversized input and bad argument controls.
- Feed the same admitted fixed control into the unchanged original `lp_graph_eval` through its existing source-symlink native harness. Check source replay and exact rank on supported forest/pivot/MST arms. Record a legitimate refusal explicitly rather than silently excluding it.

## Execution and stop conditions

One existing GitHub-hosted development job, no additional job or runner, no scientific timing interpretation. Existing 12-minute job ceiling; one Cargo build thread and one Rust test thread in the new steps, 4 GiB address-space bound for the new commands, maximum 300 seconds for adapter build/test and 60 seconds for fixed CLI checks. Stop on failure. Preserve exit codes, complete stdout/stderr, compiler version, exact checkout, source hashes, lockfile and executable hashes in the uploaded CI artifact, including failed attempts. Upload is an additive always-on step in the same job; it is not an S3 experiment receipt.

No local cargo/rustc is available. Therefore full `cargo test --release --lib` and local compilation cannot be claimed. The existing standalone crate imports the original arithmetic/F4/signature source but does not qualify the unrelated root library. Native compilation is pending CI, not replaced by source review. No caller-supplied capture, parameter search, relation collection, stage-cost measurement or cold ECDLP run is included.

## Review and coordination

Two independent read-only workers reviewed source semantics and available execution instructions; another searched for a source-custodied ordinary Prime31 capture. KB MCP and executable are unavailable. Conductor executable/control plane are unavailable; `docs/CONDUCTOR.md` documents warning-and-continue behavior without them. This is a bounded scope record, not a claimed shared lease. Preserve other branches and re-read exact PR head before each guarded fast-forward.

Sources checked: current AGENTS; latest canonical journal via Git blob `63dda03a8c9eefdc0fa9e94b5516cea0627eec82`; current scoped graph log and protocol; original `native/lib.rs`, `PartialRelation`, `lp_graph_eval`, existing graph workflow; PR #5076. Source review supports the -b target-column sign but is not executed qualification. Independent signed PCC remains separate.

## Pending natural-input requirement

The bounded cross-repository search identified `src/ml_dataset/relations_generic.rs` and `ml-ecdlp-generic-large-prime-transcript/v2` as a reusable generic-decomposition producer. The source and workload bound will be recorded separately after direct source review. Do not silently substitute a planted control for a natural capture or silently raise its workload caps. The original 256/1024/4096 same-stream rank/runtime/RSS/cold-solving evaluation remains pending authentic input and its prescribed experiment admission.

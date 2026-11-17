#set page(paper: "a4", margin: 19mm)
#set text(size: 11pt)
= Strict native panel analysis
Prime-field research infrastructure · 2026-11-03

The analysis now checks the complete matched-runner receipt before deriving any comparison. The fixed decision requires strict query-plus-verifier product reduction on all eight targets and exact baseline/A-A agreement across every counter.

#image("pipeline.svg", width: 100%)

== Checked software behavior
Five synthetic unit methods pass, covering supported and mixed outcomes, missing/extra/reordered records, duplicate keys, invalid numeric types, stage contamination, failed recovery and A/A disagreement. Separate CLI checks confirm exclusive output creation and invalid partial-panel receipts retaining the raw digest. These checks execute no cryptography and are not measured comparison results.

== Protocol and execution boundary
The revised draft binds the published native commit, binary hash and analysis hash. Canonical validation passes with zero findings. The initial build-metadata schema error is retained. Preflight and authoritative budget/capacity/quota admission remain before freeze and execution. Live NodePool and namespace quota reads were Forbidden under the existing role; no cloud job was submitted.

== Evidence interpretation
The parser validates receipt structure and internal consistency. A supplied producer exit or protocol hash is not authentication. Source and binary custody, actual point verification provenance and independent replay remain external gates. Query allocations denote requested bytes, not peak memory. The original 4563-case cohort remains unchanged.

== Sources
Native matched main.rs and receipt, original meter, saved protocol decision rule, and canonical experiment-protocol schema/tool. See README.md, source-lock.json, raw test/validation logs and cli-check.json.

# Pinned development-code audit observations

These are static code-review findings against saved development snapshots,
not executed exploits against a released producer and not claims about the
producer's subsequent fixes. The producer was being actively edited in
another worktree. Recheck its final revision before applying these findings.
The consumer's adversarial fixtures reproduce the rejected data shapes;
they do not establish that a particular compiled producer accepted them.

| Snapshot file | SHA-256 |
| --- | --- |
| storage-review/mod.rs | ec768b365b51b337ef7548b614fd177f85cecdf44adbfa5e7cff8136e1cf098e |
| storage-review/storage.rs | 376e8e44042b59fcc50b77eacf1a4c721e9ad79a18b55b26cebcd4c1ad94bd45 |
| storage-review/types.rs | 5181528c4ec19d4e10ae1ce31c3c8950fb4184921019ed9cc25e8ac85bf25f5d |
| curves-review/curves.rs | 5e7633d14bce429b7094061149d0ceaf50aa494fbae12ed9b9351188a5c0e54d |
| solver-review/solvers.rs | af5563aae0c99d8adbebd63460c02c07f7cfeb530eef0cf9c9cf8c53720885b1 |

The archive receipt in `evidence/audit-snapshots.json` binds these exact source
bytes. Paths below are within the named snapshot, not current line references.

1. **Positive verification can skip its prerequisites.** In
   storage-review/mod.rs:1186, a recovered-scalar flag requires a certificate
   role. However, certificate verification at :1208 is conditional on both
   replay and a complete factor base. A certificate-bearing failed cell with
   no public/base inputs can therefore bypass that branch; :1307 counts the
   scalar success flags separately. Consumer mitigation: every positive role
   or outcome requires all prerequisites, success status, and consistent flags.

2. **Public instances are not fully bound to the frozen cell.**
   storage-review/mod.rs:1170 binds manifest config to the plan, while curve
   replay receives public/private artifacts without CaseConfig. Coherent
   artifacts from another cell can satisfy their internal arithmetic while
   disagreeing with manifest family, n, m, twist or seed/key/generator provenance.
   Consumer mitigation: check those cross-artifact bindings explicitly.

3. **The factor-base verifier does not establish requested completion.**
   curves-review/curves.rs:951 checks point and action data, but not the
   requested size, stop condition and construction-status assertion. Consumer
   mitigation: bind requested size and reject a claimed completed base below
   the requested size unless its nonidentity subgroup is exhausted. Arithmetic
   membership and distinctness still require native verification.

4. **Private labels can leak through nominally public artifacts.**
   The full manifest embeds CaseConfig seeds used to generate the known scalar.
   Solver BSGS traces include the terminal giant index and matching baby index,
   which together recover that scalar. Consumer mitigation: select typed public
   static fields and never use manifests, seeds or solver traces as features.
   Retaining those artifacts for audit is appropriate; feature eligibility is
   a separate decision.

5. **Solver-specific limits and counters need precise treatment.**
   solver-review/solvers.rs:1189 and :1272 enter prime F4 without an explicit
   modulus-width guard at that layer; the reviewed f4_fp implementation requires
   p < 2^32. Large-prime calls can panic instead of yielding an unsupported
   outcome. The F4/F5 prepass occurs at :854, before the search counter baseline
   at :870; a search-only counter is not total work. No timing or instruction
   accuracy conclusion is established by this static review.

6. **PDP and negative-label scope need replay.** Public m controls the checked
   PDP summand count, which makes the public/config binding necessary. A
   certificate's alternate factor-base definition must be tied to PKM and its
   frozen domain. A positive certificate cannot coexist with
   `exhaustive_no_decomposition: true`. Consumer checks these structural
   restrictions but does not independently prove exhaustive negative labels.

7. **Interruption retention is distinct from completed-cell publication.**
   The reviewed native publisher publishes completed cells. Preserved local
   `interrupted-attempts/` directories are not automatically covered by that
   operation. The complete campaign still needs durable incremental retention
   of interrupted work and raw intermediates, as the user requested.

The reviewed native storage implementation already used content-addressed
objects, download/hash checks and completion-last publication. This audit
found no demonstrated loss of completed-cell objects in that code path.
The companion adds independent consumer-side retrieval and hash checking.

The consumer itself was independently reviewed at SHA
`a5f5c4002f109e0baa468db0710a2f964b5637812b91e001989827da8c371a43`.
The reviewer reproduced two software defects: rejection of the producer's
revised factor-base rule and accepting cached rows after native replay changed
their manifests or replaced the verifier executable. Both are corrected by
exact rule support and pre/post replay identity checks, with regression tests.
These corrections are software validation, not new cryptanalytic evidence.

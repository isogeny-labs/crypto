# Independent isolated progress diagnostic review

Scope: static source/diff and retained software fixture logs only, 2026-10-30. No solver tests, experiments, Git mutations, or cloud actions. Reviewer: /root/four_lead_state. Package: research/pencil_progress_20261030 in /home/ec2-user/crypto-pencil-signed-graph-20261029.

Reviewed SHA-256: src/progress.rs 35357e520da9f0edf591f0c45603f0b40715aff01702a63cf0f628acd1573f44; src/native_solver.rs 4d88d814a04da81ec723aa9e91e9f742e329780cdffdd118d7a9906096d0de5c; preserved baseline e71e9cf9b356db1f16781f394994b0ca13abbb05b4a7db82bfd2a6925a0ef4df.

## Concrete correction

1. `progress.rs:127–131` documents a private leaf but uses `fs::create_dir` without explicit Unix mode. Permissions depend on ambient umask; with 022 the leaf is 0755, not private. `OpenOptions` at145 similarly produces files with ambient mode, normally0644. Set creation mode0700 for directory and0600 for files atomically (Unix DirBuilderExt/OpenOptionsExt), retain trusted-parent prerequisite, and add explicit mode assertions. Postcreation chmod alone leaves a creation window. This is a real contract mismatch even though workload fixtures are public.

## Explicit accounting/integration requirements

2. `progress.rs:151–154,191–196`: after successful hard_link, unlink or directory-sync failure returns Err; a visible complete JSON may exist although committed and bytes remain unchanged. README explicitly discloses this, so it is not a hidden assertion of durability. However complete failure accounting needs separate published/durability-confirmed stages (or explicit indeterminate publication state), with fault-injection tests. A future parent must inspect retained files rather than infer no publication from committed=0. On termination final in-memory totals may disappear; absence must remain unknown, not zero.

3. Timing: caller JSON materialization and Snapshot identity cloning occur before the writer timer (`progress.rs:172–188`). completed_write_ns is consequently a writer interval, not total diagnostic overhead. `native_solver.rs:1310` emits cold_setup_complete after setup elapsed capture at1306 and before candidate timer at1319: that whole callback is outside BOTH reported setup/candidate elapsed intervals. The solver_returning callback at1459 also follows candidate elapsed capture at1424. README recognizes outer-envelope accounting but should explicitly enumerate the between-epoch callback too. An integrated complete wall interval must cover both callbacks, and observer overhead cannot be inferred by summing writer intervals alone. No existing clock is qualified by this fork.

4. Identity validation checks supplied values and exact expected Identity equality, not actual binary/protocol/plan provenance; README correctly states this. Snapshot validator currently allows arbitrary boundary strings and arbitrary counter JSON, and does not enforce sequence<256, boundary/epoch/walk combinations or counter-schema types. This is acceptable only as an explicitly untrusted partial sidecar until the promised bounded parent ingestion/association validator exists. Do not promote current validate() into a scientific or full progress-schema checker.

## Static observations within scope

The fork keeps original mathematical calls, RNG walk labels, candidate ordering, and semantic-meter calls. New completed_candidates is diagnostic bookkeeping only; Noop.enabled=false prevents JSON construction and emission. Disabled source-level arithmetic/RNG/counter semantics appear unchanged. Extra branches/function dispatch remain, so unchanged timing is not asserted. No solver parity test was run or inferred.

Fixed milestones give at most165 emission opportunities over16 walks; sink additionally caps attempts256 and encoded bytes65536. Encoded buffer growth is bounded, but caller materialization and filesystem latency are not. Exclusive temporary creation and hard-link publication preserve an existing destination; the trusted parent assumption remains necessary. Diagnostic rows unconditionally mark incomplete and prohibit accepted results/ratios. No final-result path was added.

Retained tests-identity.stdout records seven diagnostic fixtures passing, twelve solver tests filtered out. Existing tests cover identity/sequence, cap, temporary/destination collision, symlink temporary, forbidden accepted values, and disabled sink. They do not cover explicit permissions, post-link unlink/fsync failure, process interruption, producer/parent association or live runner timeout ingestion. This review did not rerun them.

Conclusion: isolated diagnostic software review with one concrete privacy-mode correction and the above accounting/ingestion requirements. It is not production admission, scientific execution, timing equivalence, timeout repair, or completed cohort evidence.

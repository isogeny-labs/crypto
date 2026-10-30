# Independent software/provenance review: PR2148 CI repair

**No blocking finding in the reviewed stable source/provenance diff.** This is an internal read-only software and byte-integrity review against pre-repair head `eaedaf1c132ed1f1ff101edd10c75369ab5e2dfe`, not scientific replay or approval of completed CI. The reviewer ran no tests, trials, cloud work or Git changes. Strict clippy, touched release-example checks and root-owned generated freshness are tracked separately; pending generated traits/browser/leaderboard outputs were not treated as source defects or verified complete.

The bounded-memory exporter changes formatting only. In the oracle example, `MobiusPoint::Affine.y` changes from owned `F2mElement` to owned `Box<F2mElement>`; encode boxes the same clone and decode clones the same referenced value. Derived Clone remains deep-owned. Existing chart/branch behavior is preserved, and the new clone/drop/roundtrip test enumerates every finite toy point plus infinity. No lint suppression, skipped test or relaxed mathematical predicate was added. Boxing changes allocation behavior, so historical timing/counted-source attribution must not describe the new executable; the snapshot README and v2 binding explicitly say this.

The provenance chain is sound within the reviewed bytes:

- Original v1 binding SHA256 `3e2c19a3bd0e08d65fd001d01f9588b552c2391723230d6ca9176dd7702e2074` is unchanged from pre-repair Git.
- Additive v2 binds snapshot manifest SHA256 `2d0215f5cec540c33bc486642b5f87c79f09a9171234bb9f8d3a7ad183d6baec` and locates the measured oracle source as historical `.rs.source` data. Its original compilation path remains `examples/ideas_oracle_components.rs`.
- All six snapshot files match their declared SHA256, actual Git blob framing/hash, and bytes at their stated source commits. The measured oracle snapshot is `ef34a53b6eb6daf47467cf94ccb5de9427ea6ca309d0a68d50a0cf5e5d4b260f`, blob `697b9ce42735024d82c3ae82411c46460a65120b`, at historical commit `a64f374747836eeac709bfade8cb851e46cf1443`.
- Current boxed source SHA256 `feb7a4cb3515c5eade5edcd094b72bdaab7f7c34edd7ce6284057de46e060643` matches the maintenance field and differs from the measured source. It is not substituted into old raw manifests.
- The frozen protocol, raw archive, raw certificate and raw source-manifest hashes still match v1. The raw source-manifest line still names the original compilation path. The revised tests preserve these checks and add snapshot/predecessor tamper, live-source redirection and current-source-independence controls.

An independent current-byte scan of all930 previously tracked campaign files found929 unchanged and only the authorized living oracle README erratum changed. Thus no old raw record, receipt, protocol, archive or v1 binding in that captured set was modified. Exact results and source hashes are in `independent-repair-review.json`.

The separate read-only metadata inspection found no blocker: restored authoritative `curves.yaml` plus the exact four-entry candidate snapshot reconstructs pre-repair YAML byte-for-byte; the complete old fields, curve preimages, EC1 identities and full UIDs are retained. All four canonical hashes check, and both computed and legacy model slugs remain absent from the unchanged main model registry. All five peer files match the pinned downloaded cryptanalysis source; local schemas, validator, controls and lock are unchanged, and all three local lock digests match. Living READMEs explicitly retract the earlier crosswalk-registration wording. No upstream or registry promotion is made.

Authority limit: the metadata reviewer verified the pinned downloaded source receipt/bytes for cryptanalysis commit `3ce4c29ec497ff3149eb841c07e9dd9f6631de77`; that commit is unavailable in its local Git object store, so this review does not claim an independent upstream refetch. Existing metadata logs report9 controls with2 expected skips and local/peer validators passing; they were read, not rerun. Existing provenance log reports9 tests passing. Full CI completion and root's final generated-file receipt remain separate gates.


## Final documentation/generated-delta addendum

No remaining blocker in this narrow follow-up. The two living README additions correctly qualify “current source” and prior release checks as referring to their captured revisions. They explicitly separate the new boxed source and software controls from historical sessions, timings and operation counts.

Read-only comparison against pre-repair Git independently confirmed the generated-delta receipt: leaderboard JSON, browser JSON and traits JSON each change exactly one source-hash leaf; their full recorded before/after digests match. The Markdown and HTML leaderboard renderings change only the displayed curves.yaml digest prefix. No numeric, model, roster, leader or array value changed. This byte comparison is not a native freshness or full rebuild result.

The initial no-op identity-module receipt referenced log paths subsequently overwritten by the corrected self-test. That issue is now corrected in both the temporary and proposed repository receipt: the original invocation has no stdout/stderr path claims, explicitly records the original logs as unavailable/overwritten before archival, and infers no original payload. The retained self-test logs are attributed exclusively to python3 scripts/curve_id.py selftest. This is an honest provenance qualification, not recovery of the missing original logs or validation of the no-op command.

The failed initial traits setup is separately recorded as no trait computation. The native/full trait check and bounded release checks remain pending and are not claimed passed by this review. No source edits or tests were performed during this follow-up.

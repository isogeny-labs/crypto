# Private semantic resolution of six overlapping files

These are proposed files only. Root and index are unchanged. Ancestor is3966c95fea3c87f3fdc6d13c07c882cd2a33d55c, fetched upstream iscc5addeba4dd53edc51f2c0ad9331b3ba0307adc, and candidate is the exact staged index overlay onHEAD1befe970928bc83257caa4cfc35432ca72385ea4 captured in the readiness packet. Manifest records all input/output SHA256 values. No build, merged-tree test or scientific validation is claimed.

All six resolutions are byte-identical to the existing candidate after examining the upstream changes individually. This is not a recommendation to take ours blindly across a repository: within these files upstream adds the earlier diagnostic-v1 slice, which the candidate already preserves or extends. Outside the six files, keep upstream changes normally. The supplied six-file patch is relative to fetched upstream, not a complete feature patch: the inherited new modules and other staged changes remain required.

## examples/ecbench_child_overhead.rs: one conflict

Retain diagnostics:None supplied by upstream and diagnostic_lifecycle:None required by the candidate ChildInput definition. This ordinary example remains outside the optional lifecycle path. Existing workload construction and method parameters stay unchanged.

## src/bin/ecbench.rs: one conflict

Retain both None fields in ordinary child-input construction. Upstream's reserved v1 field is preserved; the optional v2 field must be explicit for Rust construction. This does not enable diagnostics for ordinary CLI input. Candidate's separate explicit diagnostic-launch handling is retained.

## src/cryptanalysis/ecbench/audit.rs: one conflict

Retain diagnostics:None and diagnostic_lifecycle:None in ordinary auditor replay input. This preserves upstream reserved-context behavior and does not silently turn normal replay into a lifecycle execution.

## src/cryptanalysis/ecbench/mod.rs: one conflict

Retain upstream pub mod diagnostics and the candidate's diagnostic_launch, diagnostic_resources and diagnostic_runtime exports. Removing the latter would discard modules referenced by the lifecycle/custody implementation. Other module exports follow the clean three-way result.

## src/cryptanalysis/ecbench/record.rs: four conflicts

1. Keep serde-default/skip-none diagnostic_lifecycle alongside upstream diagnostics. Absence stays compatible, while exact optional v2 transport remains representable.
2. Keep the candidate bounded/duplicate-key/identity/lifecycle validation before workload construction. The legacy v1 rejection block remains immediately after it, including its original error and context validation; accepting v2 is explicitly separated from the still-reserved v1 context. The upstream-only generic comment is superseded by a precise v1 comment, not by removing its rejection.
3. Keep both None fields in the ordinary known-answer fixture; do not drop the required struct member.
4. Keep diagnostic_child_v2_tests. Upstream's diagnostic child-input tests remain intact; candidate fixtures add v2 identity/dispatch/same-UID boundary coverage rather than replacing upstream negatives.

## src/cryptanalysis/ecbench/runner.rs: seven conflicts

1. Keep lifecycle/resource execution wrapper, context publication, observed spawn/reap and resource journal publication from candidate. Check the upstream additions inside this replacement block explicitly: input.diagnostics remains None; record write and sync precede the legacy diagnostic attachment; the legacy input.diagnostics ingestion block remains with v1 schema, raw-record binding, manifest binding and null accepted_result/accepted_cost on both success/error. V2 attachment and resource records are additional evidence, not replacements for ordinary accepted results. Failure publication retains partial records and does not turn missing reap into a completed observation.
2. Keep both None fields in ordinary calibration child input.
3. Keep both None fields in run_child known-answer fixture.
4. Keep diagnostic_lifecycle:None and diagnostic_call_wall_ns:None in the upstream synthetic output fixture; retain the upstream accepted_after_stop test logic.
5. Keep the same two None fields in the upstream late-report fixture; retain its scalar/cost censoring assertions.
6. Keep reap_observed:true in the synthetic Reaped fixture. This satisfies the candidate explicit reap contract without changing timed_out/interrupted cases.
7. Keep the appended diagnostic_publication_fixture_tests, in addition to upstream diagnostic_acceptance_tests. The actual assemble path still calls diagnostics::accepted_after_stop; the two upstream test functions synthetic_reaped_stop_excludes_child_without_solver and assemble_censors_complete_synthetic_late_report are present and continue checking timeout/interruption exclusion.

## Review boundary

The private three-way outputs preserve all nonconflicting source changes automatically; each selected conflict region was checked against its upstream obligation. Because resolved bytes equal candidate bytes, existing candidate validation is not invalidated by these six-file resolutions alone. A branch integrated with all other fetched-base changes is a different whole tree and still needs applicable checks and exact-head CI. New source changes or post-capture staging require rechecking the manifest. No merge was attempted and no source was compiled here.

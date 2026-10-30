# Factor-base standalone source frozen for first validation

Contract8eddc0107ffa preceded code changes. Plan module7f99becc19a9 preserves v1
serialization/hash/index behavior and adds strict v2 policy axes. Native
constructor/replayer and curve refactor passed independent source review at the
hashes in native-source-review.json. Twelve plan and nine native policy tests are
authored, not yet executed. Full v2 CLI/export integration is still pending.

Dependency inventory02 explicitly checks2731 files/253915749bytes, including
nine historical golden/provenance files. All earlier materialization/checkpoint
artifacts are retained with fresh S3 comparisons. Consequence pipeline05 is now
terminal, exit0, with5537 passed/0failed/123ignored and matching source checks;
shared target may be reused for this separately frozen standalone phase.

The4096-byte minimum policy reserves4096 terminal bytes and therefore cannot
commit a candidate; it may retain a typed empty capped result. Counter units are
scoped admissions/calls, not retired instructions or complete native execution.
V2 worker admission must bind expected policy and replay the restricted transcript,
not merely accept a mathematically closed point population.

No new factor-base source has compiled or executed yet. Current CLI still uses
v1 plans. Full original corpus goal remains active; verifiedcloudcells0.

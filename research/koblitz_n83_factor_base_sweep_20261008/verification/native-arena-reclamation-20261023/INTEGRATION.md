# Current-main integration and final local gates

No new timed N83 arm has launched at this integration freeze. Original
PROTOCOL.md budgets and staged acceptance checks remain in force.

Published snapshot41e3464b had merge conflicts with current main, preventing
ordinary pull-request CI workflows from being created. Actual main commit
`d34b9b3908df5d3feafe6189f83b2ce3244d63c3` was merged locally as542b118835,
then published snapshot41e3464b was reconciled as
`e0fbb5eb8ebd79e6a43ba0d66e2b4094903ed5ef`. The problem index retains both
incoming and N83 entries. Current main's canonical HTML and JSON are preserved
exactly outside the frozen native XOR component; its existing rendering-only
path and consistency check pass. No legacy mathematical computation was rerun
in Python. All9N83 worker source files and the native guard are byte-identical
to90dc5a262f; final-native-source-diff.txt is empty.

The first integrated full library run passed4494tests and failed1, with119
ignored. The retained GPU mock-stage receipt reports pre-execETXTBSY(Text file
busy),PID0,null exitcode,no timeout. The private executable and original driver
bytes, receipt and empty/raw streams remain preserved in the raw integration
evidence. This was not an N83 solver attempt or a device-computation failure.

Commit`9d85821127abba88001b5a2ad8a3c8bf0e4c10e5`adds narrowly bounded handling
for that launch error: wait only before a successful executable launch and
charge the wait to the original watchdog deadline. Successfully spawned
programs are never retried. Other launch errors return immediately. Native
tests hold a real executable open for writing, verify release permits one
launch, and reject a permanently busy executable, missing path and expired
deadline. A parallel fork inheriting a writer before exec is a possible
explanation for the original transient error; the original writer was not
identified. All7focused GPU tests pass.

Final combined release validation passes4497library tests,0fail,119ignored;
19exporter,2independent model-checker,13CLI,3unknown-scalar panel,
40native guard unit+7card tests. The earlier failure log is preserved separately
as integrated-failure-library.txt. Final full logs and inventories are in
integrated-validation/. No gate or ignored-test policy was weakened.

Final worker SHA256
`4fcda42fe79c90912c4ba4e8f702945dc0cc2914d93d3c1941dd3966d481d060`;
guard-v1 SHA256
`563d2168849f7096c217c6bcbba18ed915e2c3c44bc50f422de226631b0dbad0`.
The old pre-merge worker80f296ae is retained separately. Guard-v1 is unchanged
and still admits at most the legacy1M/6000-second natural budget. The proposed
1.5M/9000-second continuation requires its separately tested explicit policy
extension after both1M candidate validations pass. No higher-cap arm is claimed.

The added/modified-file check against current main passes the10MiB gate.
The initial verified readiness archive remains immutable at SHA256
`585d71ad1ce557a3354062a732834c5a2f61cc135a2b5038e7f65ba7fe971e16`,
3,390,892bytes, in setec-astronomy-artifacts. Additional integration evidence,
source history and receipts are published separately. CI readiness still needs
fresh exact-head outcomes after the reconciled branch is pushed.

The reconciled push restored27queued workflows and PR mergeability. Its final
naming lint also exposed3unregistered model IDs used as literal results.json
metric keys in incoming main's independent validator report. The report is
preserved byte-for-byte(SHA25696b965a9772e868b8dd71ffdc6b1daf5447a19b6df1080b18c67100bf2577daf).
Existing archived-label handling now retains only those3keys at that exact
path/hash; it neither registers models nor allows their use in new prose.
Changed report bytes, another path or another unknown label are rejected.
All5naming regression tests and the full naming lint pass; the initial3-error
output remains recorded separately. The validator report, its gate certificate
and scientific results were not changed.

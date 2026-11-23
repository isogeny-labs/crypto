# Separate-role execution log

This log records the independent reviewer calls. The parent owns the shared
session log and publication. All outcomes here have unsigned PCC standing;
same-host calls and hashes are not signed execution proofs. [pcc:unverified]

## Replay completed

### What was done

- Ran the released immutable qualifier once through the frozen bounded wrapper,
  using a distinct 300-second, 2-GiB, single-thread receipt in `replay/`.
- The command and source/binary/original-artifact manifests are retained there.
  Call exit and post-call custody exit were both zero; stderr was empty.
- The 42,695-byte stdout is identical to the producer stdout, SHA-256
  `aac7736e0977f829eb2b638fc3a0ec45a134ebf9dd034c2103c4682c2b1dd7f8`.
  The complete 84-file proof/input inventory and every file's bytes also match.
- The retained summary reports all 132 cases, 16 invalid-input refusals,
  1,496 ordered triples (1,166 affine and 330 identity-prefix), 320 permutation
  groups, 198 integer-row groups and 66 empty signed outputs. These are finite
  panel observations, not independent-row or scalar-recovery counts.
  [pcc:unverified]

### Decisions made

Proceed to one saved-output reader call on the original producer artifacts,
with a fresh manifest and receipt, then to the frozen reader mutation panel.
Keep source, binaries and producer/replay output files unchanged.

### Open threads

Saved-output reader, 91-rejection/three-positive control panel, diagonal
identity read, and final separate-role readout.

### Errors

No native replay, byte comparison or post-call custody check failed.
The call remains same-host and same-binary checking. [pcc:unverified]

## Saved-output reader completed

### What was done

- Ran one reader call against the original 42 producer input/output pairs and
  original compact raw stream, through the frozen wrapper with a new manifest
  and 300-second, 2-GiB, single-thread receipt in `reader/`.
- Call exit and custody exit were zero; stderr was empty. The fixed 132-case raw
  registry and all 42 retained pairs passed exact input, byte and semantic
  binding. No extractor, norm constructor or quotient solver was invoked.
  [pcc:unverified]

### Decisions made

Proceed once to the frozen 91-rejection/three-positive panel, using the same
reader core and fresh in-memory copies. Do not change original artifacts.

### Open threads

Reader controls, diagonal identity read, and the final separate-role readout.

### Errors

No saved-output reader or post-call custody check failed. Arithmetic-call
counters remain unpriced producer diagnostics, not independently measured cost.
The reader checks retained objects; it does not recreate the 90 nonpersisted
complete objects in this mode. [pcc:unverified]

## Reader control panel completed

### What was done

- Ran the fixed control panel once, through the same frozen reader core, with
  its own 300-second, 2-GiB, single-thread manifest and receipt in `controls/`.
- Call exit and custody exit were zero; stderr was empty. All 43 semantic
  mutations were rejected in each reading mode, all five raw-only controls
  were rejected, and all three positive checks passed: 91 rejections total.
- Bare-mode removal of the noncancelling (P,P,P) witness, omission of the
  identity-prefix branch, cancellation-coordinate filtering and coordinate
  collision deduplication each failed the exact complete ordered-tuple check.
  The inverse subgroup generator was rejected by the independent input
  registry even though it generates the same group. [pcc:unverified]

### Decisions made

Proceed to one diagonal-identity saved-certificate read under its separate
60-second bound. All control mutations were in-memory copies; source, binaries
and original artifacts remain unchanged.

### Open threads

Diagonal identity read and final separate-role readout.

### Errors

No control, positive check, or post-call custody check failed. The reader
control results remain finite unsigned development observations. [pcc:unverified]

## Diagonal identity read completed; native calls finished

### What was done

- Read the retained diagonal certificate once with the separate immutable
  identity binary, through its own 60-second, 2-GiB, single-thread receipt in
  `identity-read/`. Call and custody exits were zero; stderr was empty.
- The reader bound the exact expression, metadata, producer seed and point
  count, rechecked all 32 producer points and 32 extra fixed points, and
  rejected the changed coefficient and changed value digest. No formal-proof
  or adversarial false-acceptance probability was asserted. [pcc:unverified]
- All four separate-role calls are finished. Every call used a new manifest,
  recorded its command and bounds, and retained all stdout/stderr/status and
  post-call source/binary/artifact checks. No retry or source edit was needed.

### Decisions made

Return native-call ownership to the parent for the separate root-library gate.
Write the final independent readout from the retained receipts; run no additional
scientific calls for this qualification.

### Open threads

Parent publication, report, PCC registration and separate library gate.

### Errors

No identity read, mutation control or post-call custody check failed. These
are same-host finite checks. The written completeness theorem and the diagonal
polynomial identity have no checked Lean proof in this continuation.
[pcc:unverified]

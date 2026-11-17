# Independent durable validation source review

[pcc:unverified] T-1606: source-only review; no wrapper import, syntax check, test, compiler, unit or scientific execution. Reviewer did not author the producer wrappers; review was not blind.

- F1 (v1 launcher67,116-117; worker77,126): Duplicate rejection can overwrite prior receipt. Disposition: v2 owned-output flags address source path.
- F2 (v1 launcher75-81,101): Launch gap can leave worker executing after lease loss. Disposition: v2 worker waits for short-lived ownership permit.
- F3 (v1 durable_io39-48; worker104): Admission can omit all four exact controls. Disposition: v2 exact ordered list and critical recipe fields compared.
- F4 (v1 durable_io24-27): Live prefix may include bytes appended after fsync. Disposition: v2 fixed length hashed then fsynced.
- F5 (v1 durable_io35): Positive ceiling accepts infinity and bool. Disposition: v2 finite nonbool values and allocation receipt checked.
- F6 (v2-initial durable_io74-78): Concurrent atomic temporary deletion can abort size census. Disposition: Known atomic-temp disappearance only is tolerated; other I/O errors fail.
- F7 (v2-initial launcher125-127): Seal failure can leave previous terminal receipt marked passed. Disposition: Pending receipt, sealed manifest and separate completion marker required.

Finalization follow-up: final UTC/monotonic/signal/error check precedes completion marker. Acceptance additionally requires durable outer exit zero and consistent sealed receipts/manifest; marker alone is insufficient.

Original candidate, test and fixed fixture hashes match. Four ordered exact controls and full unfiltered library suite remain. Final source hashes and snapshots are in review.json. Runtime behavior has not been validated. Complete input closure, funding/resource admission, publication, durable outer-exit recording and prospective execution review remain required.

# Original-root follow-up freeze

PR #4962 continues the correctness gate from merged #4928 on current main f0ca664eca0081a30adfcc79dcf62a32de40d2af. Only four exact guard lines in ic_framework/mod.rs are changed. The manifest, boundary and spec repairs already present on current main are retained byte for byte. Native examples, original arithmetic and Cargo.lock are unchanged. [pcc:unverified]

SOURCE_FREEZE.json records every intended command before execution and identifies the selected source inputs. It is a preregistration, not an execution receipt. The workflow records the actual checkout commit and SHA256 identities, retains independent step failures without continue-on-error, and uploads every available log. Any failed step still fails the job. The malformed-plugin test covers its existing col_of and subgroup-bound fixtures; it does not separately exercise x_index.

Historical source-audit scripts refer to the originally named local snapshots. They are retained unchanged for audit custody. Their receipts identify each Git blob and commit so the snapshots can be recovered from this repository; these are source/parser observations, not build or test outcomes. No large duplicate source snapshots are added. The minimal patch applies to the current framework source; the boundary patch is empty because its restoration is already upstream.

The earlier PR-wide manifest failure used synthetic merge checkout4a40be161b9b3e6bc4db8a8dc16864e82d754d7b, while its PR head wasfc184b03d47e13ec922c9b489f3b3f8a976a8bd9. It is preserved separately from the two dedicated rational-chain workflow failures. [pcc:unverified]

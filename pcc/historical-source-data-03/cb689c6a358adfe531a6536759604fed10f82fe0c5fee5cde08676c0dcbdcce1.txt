# Current chained-transfer store pulls and migration startup

- **Status:** investigating
- **Kind:** observed access failure and workflow startup failure; causes unknown
- **Affected work:** [crypto PR4425](https://github.com/isogeny-labs/crypto/pull/4425), current authored sourceb8462a65, full11-object migration and final-head merge gates
- **Next action owner:** current research continuation
- **Last updated:** 2026-11-16

## Problem

Current final-head PCC/validation cannot reach their original gates because store pulls return403, while the original migration workflow fails before starting any job. The current PR cannot meet normal merge requirements until those original operations succeed. This is separate from the [earlier resolved frozen-address problem](chained-transfer-pcc-evidence-addresses.md). [pcc:unverified]

## Evidence and uncertainty

Observed currentb846 runs: PCC43913455042/job133461226872 fails at the original artifact pull with30 HeadObject403 errors; validation43912916846/job133459318512 fails there with32. Original artifact-autofix43912911580 is startup_failure with zero jobs, and its check suite118879215577 reports rerequestable=false/runs_rerequestable=false. The original workflow is unchanged; no new eleven-object upload or full remote replay is observed. Whole logs/API/run/suite data are retained in `research/chained_jacobian_transfers_20261110/evidence/current-b846-whole-proof-CI-and-store-failure-34`, data115681, and preliminary0a5beb. [pcc:unverified]

All four actual current proof workflows succeed with both original audits9/0,13/0,24/0,37/0. Every byte of five original ZIPs and all276 regular members is retained and checked; the215B/one-member PCC archive is a partial failure outcome. The proof results do not substitute for the failed PCC/validation gates. The historical04e fourteen-object upload/replay and ownCI3c4318 remain scoped to that earlier source. [pcc:unverified]

The cause of403 is unknown; it does not establish missing objects, invalid credentials, a specific policy change or a failure caused by this diff. Run/suite metadata contains no startup error explanation. Historical successful authentication does not establish current access. No alternate identity, role, provider, network path or policy exemption is proposed. [pcc:unverified]

The separate first FileService archive request used a signed URL after expiration18:01:06 and failed403 before creating a ZIP, data34333d. Fresh authorized URLs consumed immediately close all five complete downloads115681. That resolved transport issue does not resolve the independent CI S3/startup failures. [pcc:unverified]

## What would unblock it

- The original current-source artifact workflow must start under its existing policy/identity, upload every original-policy whole object and independently replay every remote byte. [pcc:unverified]
- Current-source PCC and validation must complete their original authenticated pulls and all unchanged applicable gates, retaining full logs/artifacts and original unsigned exit2 handling. [pcc:unverified]
- Normal merge still requires the exact final head, every applicable check successful or legitimately skipped, no pending failure/conflict/blocking review, complete current byte/shape/mode guards and append-only ledger prefixes. [pcc:unverified]

These conditions close current CI/store custody only. They do not establish signed scientific standing, actual field/native/Jacobian program refinement, full historical LP/360/witness/controls/feedback/post execution, paid admission or matched solving-cost improvement. [pcc:unverified]

## Next discriminating step

Publish the substantive completed norm04 and trace-descent05 source additions through an ordinary expected-head FF on the existing ready PR. Its normal original workflow events can then supply new current-source startup/pull evidence. Do not push an empty change, repeat an unchanged failing suite, close/reopen the PR or change identity/roles/policy to force a pass. [pcc:unverified]

Collect the next eligible own-source observation about50 minutes after actual publication, then four-hour unchanged cadence. Successful original upload/replay and final gates permit guarded normal integration; repeated403 or zero-job startup leaves this record open and requires new evidence about the original access/startup cause. The paid EKS STOP remains unchanged; no local substitute population or paid job is launched. [pcc:unverified]

## Resolution and history

Unresolved. The previous historical address compatibility record remains resolved at its stated checkpoint; it is not reopened or relabelled as a current S3 authorization result. [pcc:unverified]

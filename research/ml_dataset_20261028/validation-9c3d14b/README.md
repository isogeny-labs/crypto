# Native launch allocation validation

Runtime source: `9c3d14b03950affa0237529ce5b55238eda011b5`.

The exact-source release library suite passed **5,119 tests, zero failed,
123 ignored**, with exit 0 and matching source readback. Focused dataset tests
passed 119, zero failed, one ignored. Strict maintained Clippy under Rust 1.98,
release CLI build, 22 actual-binary offline CLI checks, and UID 1000 container
preflight with network disabled and read-only root passed.

The CLI checks include canonical typed-plan hashing, read-only admission,
reservation and binding retries, unchanged resource totals, changed owner/evidence
rejection, direct-route rejection of launch IDs, and actual old/new binary ledger
compatibility. Local synthetic grants were never activated. These checks do not
establish external owner attestation or an implemented startup gate.

`validation-initial.json` retains the earlier partial status; `validation-final.json`
collects completed checks. The full log contains a nested one-test subprocess
summary as well as the terminal suite total; the collector selects the terminal
summary. No test was rerun to resolve that collector parsing issue.

The content-addressed S3 archive contains **31 objects, 42,690,609 bytes**.
`remote-readback.json` records fresh GET, byte-length and SHA-256 verification for
every manifest entry, without a local cache. `.artifacts.json` identifies raw
logs, exit receipts, CLI fixtures, procedures, binary and image build inputs.
Restore with the repository artifact helper against
`s3://setec-astronomy-artifacts/store`. The earlier eight-object readback remains
in `remote-readback-initial.json`.

Binary SHA-256:
`6d38ac6a2a78d60b7380fe518cd747b913d0febbfec858e62feaad35b326595e`.

Published image:
`104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:ae3ad91b07ccbd0710e90b453aa300658362be6b7b4e615a6d3bb5ea511e7a01`.

The first image push failed because the existing ECR login token expired. A
same-identity login refresh and retry succeeded; the remote ECR digest matched.
Both attempts are retained. Image build-info truthfully records the full suite
as running at image build time; the later final validation receipt records its
successful completion without rewriting that immutable build input.

Verified native cloud dataset cells remain **zero**. Software archive uploads
and container preflight do not establish worker-role S3 access, finite billing
lifetime, generation, independent cloud restore/replay, instruction labels,
mathematical conjectures, trained-model efficacy or novel solver results.

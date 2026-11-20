# Verified S3 publication — native N83 1M campaign

2026-10-21UTC: complete raw archive successfully uploaded and independently
freshly downloaded from the user-selected `setec-astronomy-artifacts` bucket.
All534payload hashes, current native audits/profiles, historical replay,
known resource-profile rejections, interrupted/nonterminal-prefix checks,
worker/input byte comparisons and deterministic archive repack pass.

- Native store name: `store:evidence/n83/natural-1m-20261020/runs.tar.xz`.
- Bytes:55236480; payload files534 plus SHA256SUMS.
- SHA256:`98c4396b189493eba773c7d3d5e10f05f974d77cf9b811af8437c1150417c5ea`.
- Payload manifest SHA256:`93ef1f19bb95f947876b8beca142cb0e6c5b3db0f5cf024ecf115710c4301e02`.
- S3 key:`store/sha256/98/98c4396b189493eba773c7d3d5e10f05f974d77cf9b811af8437c1150417c5ea`.
- [Native store index](publication-proof/store-index.json), [PUT](publication-proof/push.json),
  [HEAD](publication-proof/head.json), [freshGET](publication-proof/get.json),
  [payload checks](publication-proof/payload-checks.txt), [replay receipts](publication-proof/replay/),
  [complete verification status](publication-proof/status.txt).

The raw tree contains all seven actual attempts, available original receipts/
actual relays/cards/assessments, all retained ELFs/public inputs/source snapshots,
complete logs/profiles, host/migration/readiness/controlled diagnosis/native smoke
and test evidence, native summaries, inspected four-page PDF/graphs/render source.
Earlier4GiB800K raw evidence is separately stored and now freshly verified;
[recovery receipts](recovery-20261021/) preserve that immutable object and sidecar.
[Original1M checkpoint](checkpoint-20261021/) and
[pre-worker scope-sync checkpoint](checkpoint-v20-20261021/) remain immutable.
Their archive blobs are referenced, not duplicated inside the final archive.

All earlier failures remain unadmitted. K4096-v16's historical native PASS is
retained and replayed, but its subsequent swap-profile rejection prevents
resource admission. K2048-v16 has no original terminal/receipt/verified cleanup;
its5sample prefix remains incomplete. K2048-v19's420K trace is nonterminal,
with rejected28sample profile and verified failure cleanup. No missing value is
fabricated. Full original control/worker paths are replay prerequisites; the
fresh extracted worker/panels are byte-compared with those originals.

## Reproduce custody and replay

Use the native store resolver or fetch the immutable key above, then extract
`runs.tar.xz` into a fresh directory. From extracted `runs/`, run
`sha256sum -c SHA256SUMS`. `runs/bin/runner-v20 audit runs/runs/k2048-v20`
and the K4096 equivalent independently replay completed records.
`runner-v20 summarize-memory RUN NEW_OUTPUT` checks profile custody/counters.
Historical K4096-v16 uses retained runner-v15 for the original audit and
runner-v18 for its mandatory negative memory check; K2048-v16 must stay prefix-only.
Never regenerate original evidence in place. Exact complete commands and all
positive/negative exit conditions are preserved in
[seal-final.sh](publication-proof/seal-final.sh) and
[publish-final.sh](publication-proof/publish-final.sh).
Canonical tar root `runs`, sorted names, mtime2026-10-21 00:00:00Z,uid/gid0,
mode`u+rwX,go+rX,go-w`,gnuformat,xz-6-T1; fresh repack is byte-identical.

The raw archive contains the report frozen before its own upload receipts
exist. Those external receipts and the independently uploaded verification
sidecar establish storage completion without recursive self-hashing.
Current full source/commit history and post-merge validation are retained
separately; frozen experiment source66ab009b and worker are unchanged.

## Verification sidecar and PR integration

Raw and compact verification-sidecar publication are verified complete;
post-merge validation also passes. Final receipts and counts appear below.
The PR branch advanced to7dbc3b790860fb04a5132fb42e5f85986abe7b6e while the
experiments ran; its main merge is preserved, including all independent
benchmark rows. Frozen4409library/19exporter/2model-checker checks refer to the
experimental66ab009bsource. Fresh integrated-tree checks are reported separately.
PR readiness is not inferred from the experiment audit or these storage checks.

### Final completion

Compact sidecar **1077344bytes**, SHA256
`3a7bb909928f164075ac695f58959968eaf4a692463c38b37b00d0b9bf4a0f6b`,
`store:evidence/n83/natural-1m-20261020/verification.tar.xz`.
Its successful native PUT, freshHEAD/GET, exact archive-byte comparison,
all payload hashes, extracted Git-bundle verification and deterministic
byte-identical repack pass. [Sidecar receipts](sidecar-proof/),
[final status](sidecar-proof/status.txt). Both raw and sidecar publication
are **verified complete**. All six recovered/new archive entries are committed
in the shared store/index.json without changing other names.

Fresh post-main-merge validation at7abf1175b7fd9f2bfac86ff1657c19b430e964e9:
**4468library pass,0fail,119ignored**,731.88s test execution; exporter19pass,
independent model-checker2pass, all command exits0. Logs, actual commands/source
and wrapper exits are in [integration-validation](integration-validation/).
Guard source and isolation.rs are unchanged from the experimental66ab009bfreeze.
Only publication/report clarification/store metadata changed afterward.
These correctness checks do not promote the descriptive experiment clocks
or establish broad GitHub CI/readiness. [Source history](SOURCE-COMMITS.md).

For native store resolution, select this task's destination explicitly:
`CRYPTO_STORE_REMOTE=s3://setec-astronomy-artifacts/store` and
`AWS_DEFAULT_REGION=us-east-2`; use the committed shared store index or
`CRYPTO_STORE_INDEX` pointing to the retained native index. Raw S3 object key
and SHA256 above also permit direct fresh download without any mutable URL.

[Lock-ownership wording clarification](../../reports/run-20261020T1546Z/CLARIFICATION.md):
the report's ownership change is observed, but the writer was not identified.
The frozen archived report and readiness records remain unchanged; no
experimental result, resource admission or memory count changes.

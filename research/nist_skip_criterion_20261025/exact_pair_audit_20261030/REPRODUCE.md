# Reproduce the exact allocation audit

Source freeze fc7bac88dc217d01c59ff750d973d2796ebba1b4; all 21 original
preparation bindings stay unchanged. The preparation README/BUILD status is
historical; RESULTS.md adds actual completion. Requirements are C++17,
pkg-config and json-c development headers/library (Debian: libjson-c-dev),
with no PARI/GMP or external Monte Carlo dependency. The active preparation
path is native. The bound flatten.py is an inactive historical snapshot;
it must not be executed. REVIEW-FIXES.md records this correction without
rewriting the original preparation history or its 21 bindings.

```sh
cd research/nist_skip_criterion_20261025/exact_pair_audit_20261030
sha256sum -c SOURCES-SHA256SUMS
sh build-native.sh /tmp/exact-pair-build-fresh
sh run.sh /tmp/exact-pair-build-fresh /tmp/exact-pair-run-fresh
/tmp/exact-pair-build-fresh/summarize /tmp/exact-pair-run-fresh/results /tmp/exact-pair-derived-fresh
/tmp/exact-pair-build-fresh/copy-score-fields sources/heldout.json /tmp/external-score-fields-fresh.tsv
cmp inputs/external-score-rows.tsv /tmp/external-score-fields-fresh.tsv
sh test-native-preparation.sh /tmp/exact-pair-build-fresh /tmp/exact-pair-tests-fresh
```

Directories/files must be fresh. copy-score-fields validates the fixed source
schema and copies integer fields; it computes no trait, percentile or null.
The launcher records arguments,
native/expected statuses, stdout/stderr and source/binary bindings. Every phase
must match, including three expected refusals with no verification receipt.
Any timeout or unexpected failure stays a failure, not a dropped observation.

The original raw archive in DELIVERY.json includes all source snapshots, both
methods, inputs, distributions, controls/corrupted copies, build/phase records,
native summary and paper/source/inspection files. Verify every MEMBERS.json
size/hash after fresh extraction. Its delivery receipt sits outside its own
archive to avoid a hash cycle. The exact original bytes now have durable S3
custody, verified by a full download and all 140 member hashes. This later
upload is not a pre-experiment checkpoint or retroactive admission.

Retrieve the original evidence with the repository artifact CLI, or directly:

```sh
aws s3 cp s3://setec-astronomy-artifacts/store/sha256/3b/3bedeed0581f3e6ac286be9b69d340055e2c9ea8c83c9c1561aa4e78af1327f6 raw-records-20261030.tar.xz
printf '%s  %s\n' 3bedeed0581f3e6ac286be9b69d340055e2c9ea8c83c9c1561aa4e78af1327f6 raw-records-20261030.tar.xz | sha256sum -c -
mkdir original-records
tar -xJf raw-records-20261030.tar.xz --no-same-owner --no-same-permissions -C original-records
cd original-records
jq -r '.members[] | .sha256 + "  " + .path' MEMBERS.json | sha256sum -c -
```

S3 retrieval requires authorized read credentials for the lab artifact store;
this is a durable authenticated location, not a claim of anonymous access.
The 141 archive members retain the first-run history, including inactive Python
snapshots. Use this Git revision's native instructions for current reproduction.
The original three documentary receipts and original 13-entry result manifest
are retained byte for byte under historical/pre-review/. The current result
manifest binds their amended current counterparts; REVIEW-SHA256SUMS also
binds the new native path, tests and custody receipts.

The source/publication audits retain scope: 60 exact seed-ID score matches,
eight publication-score observations checked as within-field multisets. Source
model percentiles and unauditable 48-bit aggregates are not validated by this
audit. External procedural deviations remain; no external gated run is repeated.

The report lives under papers/nist-exact-pair-audit-20261030 and uses shared
paper.mk. Both table files are native summary output copied without arithmetic.
make check and a freshly extracted standalone ZIP build pass; extracted text
and all page rasters match. No earlier paper, source binding or raw archive is
rewritten. The separate60000-real-curve experiment remains unrun.

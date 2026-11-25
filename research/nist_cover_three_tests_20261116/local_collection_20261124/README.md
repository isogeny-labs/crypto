# Actual local P-192 collection

The user explicitly authorized local execution. The prepared pipeline actually
ran in this workspace with the frozen inputs and original caps. Main outcome:
102 added principal tangent relations, 204 new projected columns, and **zero
additional closed rank on the original 225 columns**. Native collector,
separate arithmetic checker and separate incidence inspection finish with
exit 0. These records remain producer-unverified. [pcc:unverified]

Read [REPORT.md](REPORT.md) or [REPORT.pdf](REPORT.pdf). The metadata projection
is [RESULTS.json](RESULTS.json); all process clock paths are indexed by
[COST_INDEX.json](COST_INDEX.json). The original complete local wrapper took
119.375 wall seconds. Supplemental profiling/inspection calls are additional
work and are listed separately. Original map synthesis is not priced by this
run. [pcc:unverified]

Reproduce the complete prepared pipeline from the repository root:

```bash
bash research/nist_cover_three_tests_20261116/local_collection_20261124/run-local.sh evidence/<fresh-name>
```

The output name is relative to this study directory. The recorded first call
is `evidence/attempt-v1`. To reproduce its supplementary diagnostic:

```bash
cd research/nist_cover_three_tests_20261116/local_collection_20261124
bash run-instrumented.sh evidence/attempt-v1 evidence/<fresh-diagnostic>
```

The saved inspection harness uses the fixed first-run and diagnostic records:

```bash
bash check-inspection.sh evidence/<fresh-inspection>
```

These entry points preserve fresh-output ownership and all native limits.
The user authorization and its scope are in LOCAL_ENVELOPE.json and
LOCAL_PROTOCOL.md; their presence grants no new authority to another job.
No external compute provisioning or CI signing key is implied.

Records stay loose under AGENTS 13a's path-bound exception: native manifests,
PCC subjects, checker inputs and cost/source indexes name individual paths.
Keep those exact paths and bytes. Compiler outputs are local-only and ignored;
their SHA256 bindings and reproducible source are retained. All mathematical
inputs are included in the recorded input trees or their tracked parent
archive. The parent `kernel3-resume-v1.tar.gz` is tracked and hash-checked by
the baseline driver. No file is split or compressed to evade the store policy.

Claims: PCC-20261124-695364 (actual collection), PCC-20261124-dbb2dd (fresh fixed
transfer/exception replay), PCC-20261124-2b81de (original wrapper costs), and
PCC-20261124-655ae2 (profiling/prefix/incidence inspection). All are
producer-unverified. [pcc:unverified]

Canonical-main merge bookkeeping is recorded in
[evidence/merge-v1](evidence/merge-v1/receipt.json). Initial fork-detection
outputs remain in local-checks; passing successor checks are in local-checks-v2.
Old registration envelopes are archived, unchanged claim bodies are mapped
to the canonical prefix, and both Git parents are retained. No science reran
for this metadata integration. Later publication/check receipts are outside
the sealed scientific-record inventory in RECORDS.json.

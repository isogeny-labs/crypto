# Fixed-base first-fall coverage format

`coverage.json` is an aggregate index of existing frozen V2 first-fall records. It does not contain or regenerate case outputs. Each source summary is identified by repository path, byte length, and SHA-256. The spike35–spike37 raw case archive is referenced by its S3 key, size, digest, and readback flag from the existing custody record. That archive contains report/source and per-case input JSON, but excludes the large source system payloads; the custody record identifies those separately. The six earlier 48-row candidate summaries keep their own evidence directories and persistence indexes.

The `coverage` object compares the protocol's 624 planned systems (13 coordinates × 48 systems) with candidate groups that have saved first-fall summaries. `candidate_groups` keeps per-coordinate counts, replay status, producer status, and FFD outcomes. `first_fall_outcomes` summarizes those rows by family. `last_fall_degree.exact_rows` is deliberately separate: a first-fall certificate or a null LFD value does not prove an LFD bound. `last_fall_degree.related_restricted_propagation_panel` links the separate 12-case m3 V4-consequence propagation result; its inherited strict lower bound is retained, while the absence of a unit or exhaustive affine certificate leaves the global upper bound unknown. The `degree_of_regularity` object is separate again; highest-form proxies are not affine PDP regularity labels. `source_files` identifies every summary and custody file by path, byte length, and SHA-256.

Regenerate or validate from the preserved source summaries with:

```sh
python3 research/n83_candidate_pdp_20261028/evidence/n83-factorbase-variants/first-fall-coverage-20261102/build_coverage.py
python3 research/n83_candidate_pdp_20261028/evidence/n83-factorbase-variants/first-fall-coverage-20261102/build_coverage.py --check
```

# Reproduce the recovered FreeLIP result

Hydrate this study with the standard artifact tool, using authorized store
access. The archive and any managed JSON/PDF/PNG retain their listed paths.
The current committed artifact policy keeps PDFs in Git. All four new PDFs
are tracked directly; their additional immutable store copies and all five
PNG downloads are byte-verified in evidence/document-delivery.json.
Run `sha256sum -c runs.tar.xz.sha256` and the preparation archive's corresponding
checksum file. Extract each into a separate fresh directory, compare its
MANIFEST.json to the external manifest byte for byte, then check every member's
bytes and SHA256 and reject missing/unlisted files. Source licenses/notices
stay intact. Inspecting the package requires no executable invocation.

The preparation archive root contains package/, development/, source/ and
frozen-study/. It includes the exact executable inputs, compiler download
hash/URL, both original source versions and every failed development attempt.
Build caches and the compiler archive are omitted; runnable binary bytes and
all required source/configuration are included. Its full readback predates
cohort execution.

The final archive root contains runs/, custody/ and MANIFEST.json. Extracting
runs/ beneath this study restores paths named by export.json. Original
replay001 and saved validation002 are distinct. The former's source snapshot
contains the initially frozen observer; the latter has the corrected grammar.
No legacy routine was rerun for saved validation. Compare results/cohort.tsv
and summary.json to the saved-validation output files; retain initial-summary.
Native export.cpp formats recorded classifications and hashes, and the
metadata-only schema/file validation is identified in export-admission.json.
It is not the canonical Python checker or independent mathematical primality
verification. Host/clock/rusage fields not recorded remain null.

Repack from the extracted final root with sorted GNU tar, fixed date/ownership
and the original compression settings:

```sh
tar --sort=name --mtime='2026-10-27 00:00:00Z' --owner=0 --group=0 --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - MANIFEST.json custody runs | xz -6 -T1 > runs.tar.xz
```

An actual new cohort run must use a fresh output, the declared backend,
unchanged source-bound package and successful checkpoint gate. Re-running
historical observations does not create new historical chosen/skipped labels.
Keep the exact prototype/call counts, seed behavior, controls and limits; do
not swap in an unrelated EKS benchmark fixture. Current hosted merge gates
and source/control validation remain separate from artifact custody.

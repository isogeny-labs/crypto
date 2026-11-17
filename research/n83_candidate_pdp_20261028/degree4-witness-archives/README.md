# Strict degree-four candidate dataset

The archive preserves every attempted case, all candidate and remainder rows,
ordinary and Boolean-normal field polynomials, ordered reducer IDs, input and
parent bindings, executable, controls, logs, statuses and partial summaries.
Read ../DEGREE4_WITNESS_FORMAT.json for byte layouts, monomial columns and exact
claim boundaries. A nonzero remainder against complete V3 gives lower4; an
allzero finite family gives no global upper bound. Independent review is pending.

Both immutable parent archives are required: ../degree3-m3-archives and
../degree3-continuation-archives. Hydrate all three with the repository's
tools/artifacts/artifacts.py pull command, verify their runs.tar.xz.sha256 files,
then extract into separate fresh witness, prefix and suffix directories.
Verify manifest.sha256 in each directory. For every successful producer:

```sh
witness/instrument replay witness/case-specs/KEY.json witness/inputs/ARM \
  witness/v2/ARM/ID prefix/cases/KEY suffix/cases/KEY witness/cases/KEY
```

Use the frozen protocol for KEY, ARM and ID. Allow8GiB virtual memory and1800s
per replay. Verify all three manifests again after replay. The verifier writes
no parent or witness file. Preserve failed attempts and partial files too.

Repack the archive deterministically:

```sh
tar --sort=name --mtime='2026-10-29 00:00:00 UTC' --owner=0 --group=0 \
  --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - -C witness . \
  | xz -0 -T1 > repacked.tar.xz
```

## Publication support

`support.tar.xz` retains the separate development and publication support tree:
failed control fixtures, later successful test artifacts, EKS inventory responses,
source/checkpoint/download logs, metadata adapters, report rendering sources,
page-inspection images and exact progress-metric checks. Its `manifest.sha256`
lists every payload; it excludes itself. These files document the process and
do not substitute for the raw native witness archive or its two parent archives.

After this support archive is sealed, later upload receipts and Git/PR/checkpoint
metadata are retained separately under the same run's S3 `publication/` prefix.
The Git `evidence/degree4-witness-delivery.json` record binds the inspected report,
support archive and verification coverage. Earlier failed attempts remain.

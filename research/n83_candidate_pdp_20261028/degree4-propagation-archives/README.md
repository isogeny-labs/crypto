# Degree-four propagation dataset

The raw archive preserves every attempted case, including failures and caps:
new independent dense rows, all reduction operations, pending queues, exhaustive
affine assignments, verdicts, controls, logs, executable, frozen protocol and
unchanged inputs. Read ../DEGREE4_PROPAGATION_FORMAT.json for byte layouts and
claim boundaries. Restricted queue exhaustion is not full V4 or a global upper.

Three immutable parent archives are required: degree3-m3-archives,
degree3-continuation-archives and degree4-witness-archives. Hydrate all four with
tools/artifacts/artifacts.py pull, verify the archive checksums and extract into
four distinct fresh directories. Check every manifest before and after replay:

```sh
propagation/instrument replay propagation/case-specs/KEY.json \
  propagation/inputs/ARM propagation/v2/ARM/ID prefix/cases/KEY \
  suffix/cases/KEY witness/cases/KEY propagation/cases/KEY
```

The frozen protocol supplies KEY, ARM and ID. Replay each successful producer
with8GiB virtual memory and1800s. Native replay writes no dataset file. Preserve
failed and partial cases without assigning unsupported upper or exact degrees.

To reproduce packing, use sorted GNU tar, mtime2026-10-29 00:00:00UTC, owner/group0,
mode u+rwX,go+rX,go-w, and xz -0 -T1. The support archive, when published, retains
build/test fixtures, metadata and publication helpers and report sources. Its
manifest inventories payloads; later receipts use a separate publication bundle.

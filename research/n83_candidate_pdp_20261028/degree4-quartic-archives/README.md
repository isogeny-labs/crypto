# Hybrid quartic closure dataset

The raw archive preserves every attempted case, including failures and caps:
new independent hybrid rows, every attempted product and reducer sequence,
blocked partial candidates, full compact pending queues, affine assignments,
verdicts, controls, executable, frozen protocol and unchanged inputs.
Read ../DEGREE4_QUARTIC_FORMAT.json for byte layouts and claim boundaries.
An exhausted complete product queue certifies Boolean normal-form V4 only.
A global LFD upper bound requires a separate unit or affine ideal certificate.

Four immutable parents are required: degree3-m3-archives,
degree3-continuation-archives, degree4-witness-archives and
 degree4-propagation-archives. Hydrate all five with tools/artifacts/artifacts.py
pull, verify archive checksums and extract into five distinct fresh directories.
Check every manifest before and after replay:

```sh
quartic/instrument replay quartic/case-specs/KEY.json \
  quartic/inputs/ARM quartic/v2/ARM/ID prefix/cases/KEY \
  suffix/cases/KEY witness/cases/KEY propagation/cases/KEY quartic/cases/KEY
```

The protocol supplies KEY, ARM and ID. Replay successful producers with 8 GiB
virtual memory and 1800 seconds. Native replay writes no dataset file. Failed,
partial or capped cases remain recorded without unsupported upper/exact degrees.

Packing: sorted GNU tar, mtime 2026-10-29 00:00:00 UTC, owner/group 0, mode
u+rwX,go+rX,go-w, and xz -0 -T1. The support archive retains development fixtures,
failed attempts, publication helpers and report sources. Later receipts use a
separate final publication bundle after support is sealed.

# Prime61 root and enumeration pilot: run report

The frozen four-cell `n=61, m=2` prime-field plan completed. Both bounded
enumeration cells produced verified method artifacts; both root cells reached
their predeclared timeout. All four cells were retained and exported, and no
cell independently recovered an ECDLP scalar. This is a method-artifact and
custody result, not evidence for a method speed ranking or an exact fall degree.

| Method | Untwisted | Twisted | Verified method artifacts | Timeouts |
|---|---|---|---:|---:|
| root | timeout | timeout | 0 | 2 |
| enumerate | verified | verified | 2 | 0 |

The native public artifacts identify the modulus as
`2305843009213693951 = 2^61-1`. The [independent audit](../AUDIT.json)
replayed each restricted archive's hash, manifest, public input, method,
twist, modulus, and status against the four [checkpoints](../checkpoints/).
The official [experiment export](../experiment-export.json) retains four
attempts, marks two complete and two censored, and passed the pinned export
checker with zero findings. The [producer receipt](../experiment-export-producer-receipt.json)
and [independent S3 readback sample](../S3-READBACK-SAMPLE.json) record custody;
the latter freshly read and hash-checked archive indices 0 and 3. Producer
fresh-readback receipts cover all four archives.

The [frozen protocol](../protocol.json) and [plan](../plan.json) were committed
before scientific attempts in `358e5c76f414`; the controller was committed in
`c7a3b3c9fe05`. The pinned native binary has SHA-256
`b8e0881ec5e46f60c23e6bbb49ad7c3677b9168199f5787b68d89169bfaaa77e`.
The protocol file SHA-256 is
`7a73527088b488ca6f941218f2127949eae5324cf98de629a51aaf3ad8c788c7`;
the plan file SHA-256 is
`0de282fa730d437837a0fe4b9c1893545421b5a908cf8f5cdb9837a85f0be12d`.
Native work was bounded at 30 seconds per cell, with a 90-second outer call,
2 GiB memory, and 64 MiB trace cap. See the [quantitative chart](outcomes.svg)
and [PDF report](run.pdf).

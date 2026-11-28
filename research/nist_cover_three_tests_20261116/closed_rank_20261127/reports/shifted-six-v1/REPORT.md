# Native attempt shifted-six-v1

Strict native build passes; nonzero fixture and brute six-word controls pass before actual enumeration. Source review finds a hard-coded final reference-word label off by one column. The actual target calculation is correct. Native worker is intentionally interrupted by SIGTERM (exit 143); wrapper exits 1. Partial progress and source/input/binary receipts remain intact. No complete enumeration, final stream digest, final result or rank outcome is claimed from this attempt. The correction derives labels from verified reference images and checks them before enumeration; it runs in a fresh v2 snapshot. [pcc:unverified]

![Recorded calls for shifted-six-v1. One execution each; no repeated-sample interval. Failed builds use a workflow diagram because no comparable mathematical executions exist.](calls.pdf)

| Call | Exit | Recorded wall seconds |
|---|---|---|
|build|0|13.789|
|full|143|44.156|

Complete wrapper: 57.982 wall seconds. These are
resource receipts, not performance comparisons. Host: Linux6.18.57 x86_64;
allowed CPU is in resource-envelope.txt when that stage was reached. Declared
limits:one CPU,1GiB address space,300s build and600s per native worker. No
random seeds or statistical sampling: fixed inputs and bounded deterministic
enumerations. Sample size is one execution per listed call. No timing spread
was measured.

Exact command/configuration: frozen source/run script and protocol under
../../evidence/shifted-six-v1/source/. Source ancestor,when captured:

```
c3c394e713bbb0833bbb64184e112c34287d4455
```

The manifest,when reached,identifies the frozen source, including uncommitted
changes at execution time. Available inputs,stdout,stderr,terminal exits,cost
receipts and subject hashes are under ../../evidence/shifted-six-v1/. A pre-build
wrapper failure has no compiler,worker or later-stage sealing receipt.
See ../../REPORT.md for controls, domain limits, primary references,
interpretation and follow-up; no positive additional rank is established.

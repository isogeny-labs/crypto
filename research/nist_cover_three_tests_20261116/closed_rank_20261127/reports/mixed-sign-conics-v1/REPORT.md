# Native attempt mixed-sign-conics-v1

Native plan/batch/combine passes:538new mixed-sign pencils,5complete batches,87completed same-sign cases skipped;771492candidate occurrences,0supported conics. Computed core ranks134/128/6;restored full/auxiliary ranks236/230derive by adding102private ranks. Coverage,replay,nonempty and corruption controls pass. [pcc:unverified]

![Recorded calls for mixed-sign-conics-v1. One execution each; no repeated-sample interval. Failed builds use a workflow diagram because no comparable mathematical executions exist.](calls.pdf)

| Call | Exit | Recorded wall seconds |
|---|---|---|
|build|0|17.343|
|plan|0|8.924|
|batch-0|0|39.634|
|batch-128|0|40.567|
|batch-256|0|43.65|
|batch-384|0|41.493|
|batch-512|0|19.973|
|combine|0|9.979|

Complete wrapper: 221.651 wall seconds. These are
resource receipts, not performance comparisons. Host: Linux6.18.57 x86_64;
allowed CPU is in resource-envelope.txt when that stage was reached. Declared
limits:one CPU,1GiB address space,300s build and600s per native worker. No
random seeds or statistical sampling: fixed inputs and bounded deterministic
enumerations. Sample size is one execution per listed call. No timing spread
was measured.

Exact command/configuration: frozen source/run script and protocol under
../../evidence/mixed-sign-conics-v1/source/. Source ancestor,when captured:

```
3e3b5081f6cf60f4765460f2fc72b1b1f04cba92
```

The manifest,when reached,identifies the frozen source, including uncommitted
changes at execution time. Available inputs,stdout,stderr,terminal exits,cost
receipts and subject hashes are under ../../evidence/mixed-sign-conics-v1/. A pre-build
wrapper failure has no compiler,worker or later-stage sealing receipt.
See ../../REPORT.md for controls, domain limits, primary references,
interpretation and follow-up; no positive additional rank is established.

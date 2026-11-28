# Native attempt trace-v2

Both domains passed at targets0,kappa,2kappa. Only the formal zero row at0; no quartet at2kappa. Closed rank6 in both modes; gain0. [pcc:unverified]

![Recorded calls for trace-v2. One execution each; no repeated-sample interval. Failed builds use a workflow diagram because no comparable mathematical executions exist.](calls.pdf)

| Call | Exit | Recorded wall seconds |
|---|---|---|
|augmented|0|27.131|
|build|0|6.546|
|old|0|6.724|

Complete wrapper: 40.438 wall seconds. These are
resource receipts, not performance comparisons. Host: Linux6.18.57 x86_64;
allowed CPU is in resource-envelope.txt when that stage was reached. Declared
limits:one CPU,1GiB address space,300s build and600s per native worker. No
random seeds or statistical sampling: fixed inputs and bounded deterministic
enumerations. Sample size is one execution per listed call. No timing spread
was measured.

Exact command/configuration: frozen source/run script and protocol under
../../evidence/trace-v2/source/. Source ancestor,when captured:

```
35417b073751b1db58f59ba1c99b50b9643c3c8d
```

The manifest,when reached,identifies the frozen source, including uncommitted
changes at execution time. Available inputs,stdout,stderr,terminal exits,cost
receipts and subject hashes are under ../../evidence/trace-v2/. A pre-build
wrapper failure has no compiler,worker or later-stage sealing receipt.
See ../../REPORT.md for controls, domain limits, primary references,
interpretation and follow-up; no positive additional rank is established.

# Native attempt audit-lifted-v1

Independent audit of the lifted pool passed. Full rank108, auxiliary102, closed6, gain0. [pcc:unverified]

![Recorded calls for audit-lifted-v1. One execution each; no repeated-sample interval. Failed builds use a workflow diagram because no comparable mathematical executions exist.](calls.pdf)

| Call | Exit | Recorded wall seconds |
|---|---|---|
|build|0|5.947|
|check|0|0.239|

Complete wrapper: 6.236 wall seconds. These are
resource receipts, not performance comparisons. Host: Linux6.18.57 x86_64;
exact allowed CPU is in the attempt's resource-envelope.txt. One CPU,
1GiB address-space cap,300s strict build and600s per native worker. No
random seeds or statistical sampling: fixed inputs and bounded deterministic
enumerations. Sample size is one execution per listed call. No timing spread
was measured.

Exact command/configuration: frozen source/run script and protocol under
../../evidence/audit-lifted-v1/source/. Source ancestor: `48ff0c9c79ec7e3559d28554e9f68f2cf8e826ee`.
The manifest identifies the actual frozen source, including uncommitted
changes at execution time. Inputs, stdout, stderr, terminal exits, cost
receipts and immutable subject hashes are under ../../evidence/audit-lifted-v1/.
See ../../REPORT.md for controls, domain limits, primary references,
interpretation and follow-up; no positive additional rank is established.

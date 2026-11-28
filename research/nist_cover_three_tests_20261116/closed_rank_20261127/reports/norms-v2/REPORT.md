# Native attempt norms-v2

Both signed-norm domains passed. Old101926 pairs:7rows, rank6. Augmented369370 pairs:109rows including zero, full108, auxiliary102, closed6, gain0. [pcc:unverified]

![Recorded calls for norms-v2. One execution each; no repeated-sample interval. Failed builds use a workflow diagram because no comparable mathematical executions exist.](calls.pdf)

| Call | Exit | Recorded wall seconds |
|---|---|---|
|augmented|0|9.248|
|build|0|6.234|
|old|0|2.217|

Complete wrapper: 17.734 wall seconds. These are
resource receipts, not performance comparisons. Host: Linux6.18.57 x86_64;
exact allowed CPU is in the attempt's resource-envelope.txt. One CPU,
1GiB address-space cap,300s strict build and600s per native worker. No
random seeds or statistical sampling: fixed inputs and bounded deterministic
enumerations. Sample size is one execution per listed call. No timing spread
was measured.

Exact command/configuration: frozen source/run script and protocol under
../../evidence/norms-v2/source/. Source ancestor: `48ff0c9c79ec7e3559d28554e9f68f2cf8e826ee`.
The manifest identifies the actual frozen source, including uncommitted
changes at execution time. Inputs, stdout, stderr, terminal exits, cost
receipts and immutable subject hashes are under ../../evidence/norms-v2/.
See ../../REPORT.md for controls, domain limits, primary references,
interpretation and follow-up; no positive additional rank is established.

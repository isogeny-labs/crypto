# Terminal-domain and residual-sliced S3 — 2026-11-15

All **686/686** primary executions complete with no invalid decision. The
new sliced strategy decides all **98/98** rows in its arm, including
distinct-x planted controls on n131 bases of 26 and 112 points. Sliced A/A
outcomes/conflicts agree. These are controlled original-group point
decompositions, not recovery of public challenge Q [pcc:unverified].

| Arm, 98 executions each | Found | Refuted | UNKNOWN |
|---|---:|---:|---:|
| Constants baseline |36|14|48|
| Identity-first |50|14|34|
| Marginal T-B domain |56|22|20|
| Correlated tail pairs |52|22|24|
| Constant-residual sliced S3 |68|30|0|
| Sliced A/A |68|30|0|
| Fresh classical pair-table |68|30|0|

Retain all 33 previous inputs and add 16 predeclared holdouts at n83/131,d6/8.
Every planted holdout uses three distinct x values and a nonidentity target
outside the base. The actual bases are 26/134 points at n83 and 26/112 at
n131. Each of 49 targets is repeated twice; no independent population sample
or timing/scaling claim [pcc:unverified].

For `P+Q+R=T`, the finite partial sum lies in actual `T-B`. The marginal
option installs those finite x codes; the paired option retains their
coupling with the final x. Infinity has a separate identity mask. Identity-
first changes only complete mask order. The sliced solver fixes the actual
last point R and solves `P+Q=T-R` by constant-target two-point S3, sharing the
total conflict/model/deadline budget across residuals. Any incomplete slice
prevents Refuted unless another slice finds an actual valid sum [pcc:unverified].

The classical reference rebuilds a full pair table per query and charges
setup; it never receives the independently generated truth table. Group
additions+doublings total 3,342 sliced versus 132,124 fresh reference. Field,
SAT/ANF work and batch amortization are not priced by that group unit, so
these are not runtime/full-cost ratios [pcc:unverified].

The 35-call pilot and native analysis smoke pass. One CPU, 2GiB, 600 seconds
including pilot/freeze/run, 2,000 conflicts/750ms query safety fuse, no
retries or paid compute. Flow has no exclusions, interruptions or missing
primary rows. The native audit rechecks all 49 truths and replays all 398
saved original-point tuples. Canonical export/source/file validation passes;
independent signed review is absent [pcc:unverified].

[Dated illustrated run report](reports/run-20261115T0542Z-terminal/report.pdf) ·
[summary](summary.json) · [accepted export](export.json) ·
[truth/relation audit](integrity-audit.json) · [protocol](PROTOCOL.md) ·
[next larger-base linearization study](../trace_zero_linearized_20261115/README.md).

Source `3827643341debd85404333b090bd2fbfec6f3472`; frozen protocol
`7883b155e9c7015e213d879c21a3c4fd0aa79145b408cc00f1adf54e913a2c94`;
binary `55c9827dda5a903b5386087e6118170c959809aa9246b695524ec55b277dc664`.
Later prepared-admission/linearized source additions do not rewrite this
instrument or its historical [source pins](SOURCES.sha256).

Use the native binary's `analyze rows.jsonl` command and the canonical
`experiment_protocol.py check-export --verify-files` tool to replay the
summary/file checks; do not repeat a trial to change its outcome. Raw files
may live in the content-addressed artifact store with manifests in this
directory and exact earlier Git objects. No benchmark record or validation
gate was changed. Existing ledger/whole-root/benchrec merge gaps remain.

Sources: native `constrain_intermediate` already documents the target-minus-
tail restriction; repository Koblitz notes document meet-in-the-middle.
Kosters–Yeo supplies the trace/S3 context. KB unavailable, wrapper127;
repository/primary-copy fallback recorded. This reuses known algebra and
claims no mathematical novelty or general arbitrary-algebra variety chart.

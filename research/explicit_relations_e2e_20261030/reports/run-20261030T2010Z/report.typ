#set page(paper:"a4",margin:20mm)
#set text(size:10pt)
= V15 stopped at source checkout

Observations only. Frozen protocol-v15 launched one RTX PRO 6000 Blackwell
Server Edition pod, but reached neither compilation nor experimental data.
Zero of eight prescribed recovery executions were attempted. The source-delivery failure is preserved for reproducibility.

#image("stages.svg",width:100%)

== Recorded failure
Pod `ndiwvrvehbf4hy` was created 2026-10-30T20:10:04 UTC in EUR-IS-2 at the
provider rate USD 2.49/hour. This is a rate, not a final bill. The pinned CUDA
image was the same as v13. Driver/memory telemetry was never reached.
The bootstrap installed prerequisites and fetched the original commit, but its
partial-clone lazy checkout failed fetching root file `.artifacts.json`:
`fatal: could not read Username for 'https://github.com': No such device or address`.
Required blob: `52a224ca9141900ae3779a85a56f3e810251d6d3`.
Bootstrap exit:128. Controller exit:1. Build, host/CUDA tests, sanitizers,
curve generation, certificates and timings: absent because checkout failed.

== Preserved contract and custody
Source: `23dc8a61ca928e47237f8fb5a3a8d87228649628`.
Protocol hash:
#text(size:8pt)[`8e079c9e143ddfcb00dcf66a5d5002d22e9ce7722291e14b4f28292c8eee65ad`].
The intended launcher was `bash research/explicit_relations_e2e_20261030/run-filter.sh`.
It prescribed six 52-bit baseline/filter/A-A executions followed by two 63-bit
filter executions, seeds101/202, m8, row cap2^32, work cap2^34, chunk2^20,
columns16384 and16trials. Solve caps180/7200s; worker18000s; controller18300s.
None ran. The one-attempt contract is closed; no retry is hidden in its records.

The archive was collected and its SHA256 verified before pod deletion:
#text(size:8pt)[`ce7b1a34a7059c30331e4a9f9e3b34473c101be74d51c603cd2fd7db0ebe6554`].
Location: `s3://setec-astronomy-artifacts/validation/ecbench-explicit-x/e2e-20261030-v15/results.tgz`.
`failed-launch-v15/` retains logs, receipts, bootstrap/controller scripts,
exit codes and deletion evidence. Raw records were pushed in `3e3e3a10cfe3`
before this report. A fresh complete provider list confirmed no task pod.
No recovery export or rows.csv was invented for this zero-trial launch.

== Prepared correction
V16 is separately frozen before data and keeps the measured solver/instruments
unchanged. Source delivery uses an archived sparse checkout of the original Git
commit. All1030 materialized files were verified against original blob IDs,
with clean HEAD and no remotes. The worker checks the archive digest before
extraction; full preflight remains mandatory. V16 next runs device correctness checks and the larger recovery targets.
Completed v11/v13 records supply the verified 62-bit starting point.

# Qualification result

The actual ecbench CLI/module passes 8 focused native codec controls. Historical legacy and paired plans are byte-identical; source/input/executable pre/post hashes match. Four frozen endpoint/control selections are joined in plan-only mode. [pcc:unverified]

| Selection | Planned pairs | Targets | Warmup pairs | Observed records |
|---|---:|---:|---:|---:|
| field202-image | 224 | 32 | 32 | 0 |
| field202-coordinate | 224 | 32 | 32 | 0 |
| field96-image | 224 | 32 | 32 | 0 |
| field96-coordinate | 224 | 32 | 32 | 0 |

The input has 192 workloads and 4032 planned executions. No ECDLP/scalar/timing session ran. Statistical comparison and the original at-least-80-bit prime conductor crossing remain open. [pcc:unverified]

PCC-20261118-c121f7 binds the checked metadata/source/qualifier. PCC-20261118-368507 retains the failed diagnostic. Both are unsigned producer attestations, not signed independent verdicts. [pcc:unverified]

- [Checked summary](summary.json)
- [Inspected report PDF](reports/report.pdf)
- [Command and input contract](README.md)
- [Successful qualification output](qualification-b.stdout)
- [Actual CLI module](../../../src/bin/ecbench/endpoint_pairs.rs)

Build attempt-a fails on a test API misuse; attempt-b builds but qualification-a has one incorrect fixture-arm selector. Attempt-c and qualification-b correct the fixtures. Earlier source/output versions remain unchanged. The two-field checks are plan/codec checks, not new measured isogeny or solver data. [pcc:unverified]

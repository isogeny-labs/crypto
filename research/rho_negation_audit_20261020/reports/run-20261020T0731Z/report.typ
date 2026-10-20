#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
= Run report: ecbench session ECBS1h83b97eccdcda and volcano-rho runs (2026-10-20T07:31Z)

*Observations only; interpretation is in the study README.*

- *ecbench spec* `../../specs/negation_vs_plain.json` (ECS1h7a0b49db98d3), binary at main `6e35e1f42` (clean, sha256 68cf18fb…). Command: `ecbench run --spec specs/negation_vs_plain.json --out sessions/negation_vs_plain --cpus none --allow-busy`. Host: Intel Xeon \@ 2.10 GHz, 4 vCPU; isolation L0 on every run.
- *Arms:* `rho-neg` = rho.negation (reference), `rho-plain` = rho.plain (baseline). Measurement seed 20261020, target seed 20261020, 64 targets per curve, 4 curves (`../../curves.json`), one round, order alternate.
- *Runs:* 512 requested, 512 completed and verified, no failures. `ecbench verify --replay 16`: audit OK, 16 replays reproduced.
- *Recorded values* (unit: GAE / $sqrt(r)$, mean over 64 runs, 95% interval as recorded): see `../../table.md`; band at weights 0, 0.1, 1 in `../../band.json`; paired comparison in `../../sessions/negation_vs_plain/comparisons/rho-neg__rho-plain.json` (ops ratio 1.3709 [1.257, 1.491], wall median 1.1395, both as recorded).
- *volcano-rho* (working tree of this PR, release build): `--bits 36 --depth 2 --seed 20261020 --trials 256 --instance-study --negation on|off`; 5 arms × 256 solves per setting, all correct (`../../volcano/instance-*/`). Smoke runs `--bits 30 --depth 2 --seed 7 --trials 16 --walk native|pullback`: 176/176 correct each.
- *Missing:* no L2 isolation; wall times are not admitted beyond L0.

#figure(image("../../figures/negation_vs_plain.svg", width: 100%), caption: [Recorded values of session ECBS1h83b97eccdcda (left) and the volcano-rho instance studies (right).])

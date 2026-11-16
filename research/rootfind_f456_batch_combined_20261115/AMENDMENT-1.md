# Amendment 1 — fixed-panel analyzer admission

**Filed 2026-11-16 02:39 UTC, before affected data.** No execution of this
combined panel had been submitted when this amendment was filed. Version 1 is
retained in [protocol-v1.json](protocol-v1.json), with canonical freeze hash
`8a090e8c7ee70c6969ef652eaab0c0ade66cc4cba6794b5b9f281491c7c05b66`.
Version 2 is [protocol.json](protocol.json), with canonical freeze hash
`441c2fd187f759d3e5cccaf78c89ede1057ab0d92bd9a44cbd8055ba6b0c228a`.

## Why this changed

The version 1 analyzer used truthiness for `isolation.node`. An ecbench record
on NUMA node `0` therefore received `host_node_missing` even when it reported
L2 and a verified scalar. A read-only check against an existing K13 F6-IC
record reproduced the rejection [pcc:unverified]. The analyzer also required
eight workload IDs without checking that they were the frozen eight public
targets. A different eight-target panel could have passed its row-count gate.

## Exact correction

- Admit `isolation.node` when it is an integer at least zero.
- Require every row to name registered Koblitz E0 n13 curve UID
  `urn:ec-record:1:sha256:1dfa32cdf623e34fb258128415cab472abaa51695380fa5d4f5d636ed5a3da60`,
  public target law `hash_to_subgroup_v1`, target seed `202611151301`, and an
  index in `0..7`.
- Require one workload ID, workload SHA-256, target point and target counter
  for each index across all arms and rounds. Replay remains required to
  establish that those fields really follow the deterministic target law.

The ten arms, 560 executions, full cold L2 process-wall metric, five
comparisons, all-pair rule, target-cluster bootstrap, A/A rule, stopping rule,
and budget admission are unchanged. The corrected analyzer SHA-256 is
`89fb800fea4a0adc6c9a9a4844e41becf84288dc016b8857f6768c0971f4199a`.

Synthetic controls accepted a complete 560-row panel on node zero and
rejected one wrong seed and one inconsistent target point [pcc:unverified].
The design graph in `panel.dot` and rendered SVG/PDF still show the same
ten-arm experiment and decision path; this correction adds only input
admission checks. No performance result follows from these controls.

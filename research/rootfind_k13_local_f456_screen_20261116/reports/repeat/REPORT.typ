#set page(paper: "us-letter", margin: 0.65in)
#set text(font: "Helvetica", size: 9pt)

= K13 local rootfinding screen: custody-correct repeat

*Observation only.* This report describes session `ECBS1hc322a24b6610`; it does not change a hypothesis status or establish a qualified speed claim. Claim `PCC-20261116-3b7488` remains `not_covered`: the attached replay and arithmetic checks are unsigned producer artifacts.

The frozen `ECS1h326f1843d687` spec used Koblitz E0 over F2^13, subgroup order 2003, a dimension-5 binary subspace factor base, two fresh public targets, two measured rounds, and one warmup per arm and target. Eight arms completed 48/48 verified answers; `ecbench verify --replay-all` reproduced 32/32 measured executions. Source commit `8f4cc824c87a236aafd28aa390c744e355a08e45` and executable hash `dba933709f466966dc13f069b9bf0e91def033284f8b1cb779b25c8c9b3448c1` are bound in the session. The host was Apple M4 Pro, macOS, isolation L0.

The previous session `ECBS1h124a5b50bd9a` has an invalid source stamp and contributes no timing row here. This repeat reused the frozen target identities, so it is a custody repair, not an independent holdout.

#image("paired-ratios-log.svg", width: 100%)

The graph gives *derived* `sum(batch process_wall_ns) / sum(reference process_wall_ns)` over four verified exact target-round pairs per reference, with zero omitted pairs. Batch versus F6-IC is 0.22155; versus inherited F4 is 0.10225; versus matrix F4 and F5 is 0.00092448 and 0.00110148. Batch versus scalar rootfinding is 1.10630, versus the batch A/A arm 1.03857, and versus strong rho 1.13928. Complete-process medians were 10.77 ms for batch, 48.58 ms for F6-IC, 104.67 ms for inherited F4, 11.94 s for matrix F4, 10.37 s for matrix F5, and 9.31 ms for strong rho.

For batch, median method call was 0.572 ms and relation phase 0.105 ms. The roughly 10.2 ms median process/method gap includes process launch, serialization and harness work; those portions were not separated. Matrix arms reached median rank 17.5 after 32 trials; the other IC arms reached rank 16.5 after 30.5 trials. All IC arms share factor-base digest `cc3c0ee034174a528849ca7efd95d70459404f6cc1e9361f0b4da7f6c3da2ae3`, but their relation streams differ. L0 timing and four pairs do not support a scaling inference. The fixed EKS L2 panel is still pending its resource gate.

Evidence: `../../sessions/local-custody-repeat-202611160511/`, `../../audits/repeat/replay-all.json`, `analysis.json`, `pcc-recompute-wrapper.json`, and `RESULT.md`. Large JSONL files are bound through content-addressed artifact manifests. The figure source is `../../plot_log.py`.

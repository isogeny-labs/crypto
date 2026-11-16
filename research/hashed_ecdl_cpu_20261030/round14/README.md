# Optional-profiling snapshot comparison

Observations only: shared-host L0 diagnostics, not a certified speedup or scientific verdict.
This run measures an **unapplied snapshot**, not the integrated shared solver.
Repository commit `fb3cb55f518a` plus the optional-profile patch
`12b08d04ada9b9bcf6834212a204eb2e7ebe4fb4913156c270f224befd21bb3e`
produces effective shared-source hash
`f686bec1052c408c02d443263035cec833771e38743227bd8e049fe37ab77222`.
The materialization manifest pins every source/manifest input and confirms the
shared checkout source was unchanged. The separate target-reuse patch is absent.

Baseline u/x remains `2bd4588fba60`; previous integrated candidate y is
`92463171363c`; snapshot v and batch1 control w disable only discarded internal
stage clocks. The profiled API remains available. Outer timers, the Timed oracle
wrapper, all algorithm phases, serialization and both full checks are unchanged.

| Fixture | Baseline u ms | Previous y ms | Snapshot v ms | u/v | y/v |
|---|---:|---:|---:|---:|---:|
| 8-bit, 3 slots | 0.389109 | 0.152397 | 0.146386 | 2.658 | 1.041 |
| 16-bit, 3 slots | 5.784662 | 0.855458 | 0.830132 | 6.968 | 1.031 |
| 16-bit, 4 slots | 6.036159 | 1.057224 | 1.027651 | 5.874 | 1.029 |

All 150 executions completed: 135 measured and 15 warmups. Full certificate bytes
agree across all arms; scalar checks, replay and corrupted-scalar rejection pass.
No failed or excluded cells. Nine measured repetitions follow one warmup per cell.
Intel Xeon 6975P-C, one software thread, 2 vCPUs/4 GiB reserved, shared EKS VM.
Do not pool these times with earlier AMD EPYC runs. Medians are measured; ratios
are the named numerator divided by v. A/A x medians are 0.389082,5.756197,6.044363ms.

The small fixture has v range0.141847–0.151094ms versus y0.151507–0.158814ms.
The larger ranges overlap: v0.815642–0.848126 versus y0.846418–0.873087ms, and
v1.012017–1.054663 versus y1.034722–1.079621ms. These observations support further
integration work, not a certified latency claim. Build-path layout is a recorded
nuisance factor. The further10x target remains unmet on every fixture.

Setup, discarded trials, relation collection, elimination, both certificate
checks and serialization remain charged. Process launch, archive I/O and
negative controls remain outside the interval. Internal clocks removed by the
patch had been collected and discarded by the ordinary solver API; no algorithm
phase was excluded. The frozen five-arm schedule and inputs are unchanged.

Export completeness/file-integrity verification passes with zero findings.
The returned materialization manifest is byte-identical to the frozen copy.
The tested snapshot passed32 release tests, strict Clippy and formatting; this
run independently confirms complete output equivalence on the three fixtures.

[Protocol](protocol.json), [materialization](candidate-materialization.json),
[runner](run.sh), [rows](results/rows.csv), [summary](results/summary.csv),
[export](results/export.json), [build](results/build.json),
[terminal receipt](status.json), [archive receipt](results-receipt.json),
[report](../reports/round14/report.pdf), [patch](../optional-profile/README.md).

## Handoff

- Completed: measured the source-hashed optional-profile snapshot without editing
  the reserved source; preserved all150 executions, exact source identities,
  receipts and verified export on follow-up PR #2963.
- Decision: keep the patch as an integration candidate; do not attribute this
  result to the current integrated executable or claim certified improvement.
- Remaining: reconcile T-906 ownership before integration, then validate the
  integrated source. Separately measure target reuse and address remaining
  reduction costs. Strict isolation, independent review, external comparisons
  and larger-scale CPU recovery remain outstanding.
- Errors: no run, native-analysis or export-verification failures. Ordinary EKS
  scheduling/pull delay preceded execution; no duplicate or replacement job.

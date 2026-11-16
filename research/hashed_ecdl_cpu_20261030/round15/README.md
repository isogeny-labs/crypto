# Combined reduction snapshot comparison

Observations only: shared-host L0 diagnostics, not a certified speedup or scientific verdict.
This run measures **unapplied snapshots**, not the integrated shared solver.
Combined candidate repository commit `9a05eb488405f0916dc66f643fc0540bfda75419` plus patch
`16aa67c94f1382d6e1c6e26c1e65e79be6e3604e9af25824de9a809f4873ea34` produces effective shared-source hash
`050a35a500f1685e16870f3375d67f2c61fbd979be5ded5e4ee374c77554b026`. The manifests pin all source inputs and
both snapshot binaries; the reserved shared checkout remains unchanged.

Baseline u/x remains `2bd4588fba60`. Reference y is the preceding optional-profile
snapshot (repository `fb3cb55f518a` plus its frozen patch). Combined v and batch1 w
add collection target reuse on top of optional profiling. Collection still
validates every tuple point, factor-base membership and affine-sum equation.
The original relation checker and full verifier are byte-identical, so both
certificate replays still independently recompute targets from recorded u/v.

| Fixture | Baseline u ms | Profiling-only y ms | Combined v ms | u/v | y/v |
|---|---:|---:|---:|---:|---:|
| 8-bit, 3 slots | 0.397709 | 0.147832 | 0.143688 | 2.768 | 1.029 |
| 16-bit, 3 slots | 5.796725 | 0.838992 | 0.814772 | 7.115 | 1.030 |
| 16-bit, 4 slots | 6.060122 | 1.032969 | 1.007229 | 6.017 | 1.026 |

All 150 executions completed: 135 measured and 15 warmups. Full certificate bytes
agree across all arms; scalar checks, replay and corrupted-scalar rejection pass.
No failed or excluded cells. Nine measured repetitions follow one warmup per cell.
Intel Xeon 6975P-C, one software thread, 2 vCPUs/4 GiB reserved, shared EKS VM.
Only within-run comparisons are reported. Medians are measured; ratios are the
named numerator divided by v. A/A x medians are0.389588,5.801267,6.070398ms.

All candidate/reference ranges overlap. Candidate v ranges (ms) are
0.140297–0.154517,0.801787–0.836947,0.986422–1.020244; reference y ranges are
0.143217–0.161480,0.824788–0.858150,1.014289–1.062496. The medians justify retaining
the composed patch for further investigation, not a certified latency claim.
Build-path layout and co-tenancy remain nuisance factors. The further10x target
remains unmet on every fixture; no cross-panel absolute times are pooled.

Setup, discarded trials, relation collection, elimination, both certificate
checks and serialization remain charged. Process launch, archive I/O and
negative controls remain outside the interval. The complete driver and its
external timers are unchanged. No fixture or algorithm phase was removed.

Export completeness/file-integrity verification passes with zero findings.
Both returned source-materialization manifests match their frozen copies exactly.
The combined snapshot passed33 release tests, strict Clippy and formatting,
including the new tests from both constituent patches and archived certificates.

[Protocol](protocol.json), [candidate source](candidate-materialization.json),
[previous source](previous-materialization.json), [runner](run.sh),
[rows](results/rows.csv), [summary](results/summary.csv),
[export](results/export.json), [build](results/build.json),
[terminal receipt](status.json), [archive receipt](results-receipt.json),
[report](../reports/round15/report.pdf), [combined patch](../combined-reduction/README.md).

## Handoff

- Completed: composed and tested both patches without editing reserved source;
  measured all150 cases against the preceding profiling-only snapshot; verified
  receipts, manifests and export; published source and evidence on PR #2963.
- Decision: retain as an unapplied engineering candidate, without attributing
  this result to integrated code or claiming certified performance.
- Remaining: reconcile shared-source ownership before integration; investigate
  remaining per-query target generation and arithmetic costs. Preserve complete
  timing and independent verification. Strict isolation, independent review,
  external comparisons and larger-scale CPU recovery remain outstanding.
- Errors: no run, native-analysis or export-verification failures. No duplicate
  or replacement job was launched.

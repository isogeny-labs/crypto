# Replication of the padd rank-stage panel on the merged head

Requested by the owner 2026-10-20, after `origin/main` was merged into the
PR branch (merge commit `0124cca3b`, `main` at `97a4c5a86` plus seven later
commits). The first panel (Actions run 39296352594, commit `4b2b3efa6`) is
reported in [RESULT.md](RESULT.md) and stays as recorded.

This run is frozen before its timing starts:

- Same protocol as [PROTOCOL.md](PROTOCOL.md), unchanged: same 20 points, same
  arms, order, rho draws and seeds, same gates, same A/A band, same kill
  threshold (rank-stage speedup < 1.2), same `run.py`, `analyze.py` and
  bootstrap seed. Nothing is tuned after seeing the first panel.
- The binary is built from the merged head. `examples/koblitz_orbit_dlp_fast.rs`
  and `examples/n83_rho_optimization_eval.rs` are the same sources as in the
  first run apart from the merge itself; `main` changed neither, nor
  `koblitz_fast_arith`, `koblitz_index_calculus`, `semaev_decomp` or
  `binary_ecc`, which the measured path calls. It did change other library
  modules linked into the binary (Groebner and tower code), which the example
  never calls; code layout can still move timings slightly.
- Both runs are reported whatever this run shows, side by side: it is a second
  runner allocation, so the difference between the two runs is a (single)
  sample of between-allocation variation, not a new interval. If the second run
  fails a gate or its rank-stage speedup falls under 1.2, that is reported as
  such and the PR's claim is restated to cover both runs.
- The canonical N83 row and the leaderboard section keep the first run until the
  second is analysed; no number is replaced before both are in.

## Trigger log

- `ae1819e56` (05:44Z) changed `trigger.txt` but no run of the panel workflow
  appeared in 15 minutes (only the LLM review workflow fired); `workflow_dispatch`
  is refused to this session (403). The trigger was pushed again below. Only one
  panel run on the merged head is counted; if two start, the earlier one is used
  and the other is reported.
- Local check on the merged tree, before the run: `cargo test --release --example
  koblitz_orbit_dlp_fast` 24 passed, 0 failed.

- Finding at 06:00Z: the workflow `n83-padd-rank.yml` has state `disabled_manually`
  (changed 2026-10-19T21:59:17Z, after the first run finished at 21:54Z), which is
  why neither push started a run. This session cannot re-enable it (no Actions write
  permission) and does not work around a deliberate disable. The replication run has
  **not started**; it needs the owner to re-enable the workflow (or say which other
  way to run it). Nothing in this file is a result.

## Correction (2026-10-20, after the second merge of `main`, head `058937890`)

The note above says `main` changed neither `koblitz_index_calculus` nor the other
modules the measured path calls. That held for the first merge (`0124cca3b`). The
second merge brought in `src/cryptanalysis/koblitz_index_calculus.rs` changes
(commits `aebe1f0f7`, `dcb07e6cd`). Every hunk is inside `PairSumTable`,
`ScanScratch` or `mod tests` (the new struct fields and the folding-kernel scan
path); none is in `KoblitzCurve::new` or `find_irreducible_sparse_wide`, the only
functions of that file the producer calls. So the producer's executed path is
unchanged, but the file is not, and the linked binary differs. This is another
reason the replication on the merged head is wanted and is still not run.

## Replication on a second host (2026-10-24)

Another session re-ran this panel's runner unchanged (`run.py` functions, arm
order, rank seed 29, rho seeds `2026101900 + 10 i + d`) from binaries built at
PR head `e00d818a4`, on its 4-vCPU cloud container (Intel Xeon @ 2.80 GHz),
inside a larger panel (`research/n83_mitm3_ic_20261024`, unrelated arms run
after each point's block). That session stopped the panel after point 2 when
it moved to other work. Points 1 and 2 completed every arm (cur_a, cur_b,
padd, three rho draws), point 3 completed its padd arm, and point 3's cur_a
was stopped mid-run. Records are in [`replication-20261024/`](replication-20261024/);
base dumps are replaced by their SHA-256.

All 13 completed replayed arms reproduce the original Actions arms exactly:
recovered scalar, relation point indices and probes, rank attempts, relations,
failures, rows without gain, probe mean and rank, factor-base digest, and for
rho the scalar and group-addition count. That gives 26 scalar checks against
the sealed answers (`examples/n83_panel_analyze.rs replay-2282`;
[`replay-certificate.json`](replication-20261024/replay-certificate.json), status `verified`).

This host is slower than the Actions runner. The baseline arms took about
216–221 s per cold process here, against about 120 s there. That was one host
and two points, so no timing claim is drawn from this replication; it
establishes reproducibility of the counted work and the answers.

Validation gate: [`benchmarks/20261019-n83-padd-rank-stage/VALIDATION.json`](../../benchmarks/20261019-n83-padd-rank-stage/VALIDATION.json)
names this replay certificate. It also names the validator report, which
recomputes the 11 headline point estimates exactly from the raw Actions
records, and the strong-rho reference with the §5a band. Rho is cheaper at 0,
0.1 and 1 GAE: 34×, 41× and 61×.

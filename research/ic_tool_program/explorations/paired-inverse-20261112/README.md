# One inverse per ± twin pair in the aimed scan (lever 2): exploration

**What this is.** A timed exploration of `a31ae987a6` on this branch,
`main` at `0a1969d089` plus this change. It ran on 2026-11-12 on this
session's container: Intel Xeon @ 2.1 GHz, 4 vCPU, AVX-512, VPCLMULQDQ and
GFNI. It is a record, not a round, and no run is pooled with a round's.
The figures are for the protocol that comes next, and support no claim.

## The change

- **Why twins can share an inverse.** Aimed collection scans whole columns,
  and the base stores each point's negated twin at the next index. This was
  measured with `lever2_twin_in_block_fraction`: 100% adjacent at n = 53
  (27,984 points) and at n = 61 (39,040). Two twins share the denominator
  `x_R + x_k`.
- **The kernel.** `FastCurve::add_many_lazy_soa_paired` and the 8-lane
  `kernel_fold_paired` form both sums from one prefix product and one
  inverse. The outputs are interleaved, word for word as the unpaired
  kernel gives them (`the_paired_kernel_forms_the_interleaved_list_s_sums`).
- **Odd-aligned windows.** A window that starts or ends inside a pair takes
  the whole pair, and the extra output is skipped by offset, never read.
- **Counters and fallback.** Additions are counted for the window's own
  summands only. A declined pair falls back silently to the unpaired
  kernel, which counts as before.
- **Control arm.** `KIC_SCAN_PAIRED=0` gives exactly today's path.

## Checks

- **Identical outputs.** Certificates, counts and status match between
  `KIC_SCAN_PAIRED=0` and the default on `M1-T01` at `k0n41`, `k1n47`,
  `k0n53` and `k0n61`, and on every run below.
- **Tests.** `cargo test --release --lib -- koblitz_fast:: koblitz_index_calculus::`
  gives 213 passed, 0 failed (5 ignored). Clippy is clean on the changed
  code.

## Kernel micro-benchmark

`lever_kernel_microbench` ([`microbench/microbench.txt`](microbench/microbench.txt)):
1,024 twin pairs, best of 7, in ns a summand.

| n | unpaired | paired | saving |
|--:|--:|--:|--:|
| 53 | 1.985 | 1.435 | 0.550 |
| 61 | 2.058 | 1.495 | 0.563 |

## Paired against unpaired

`M1-T01` at `k0n53`, `k1n59` and `k0n61` from the suite. Arms alternate
order by round, three rounds, under `isolated_bench --cpus 2` with
`RAYON_NUM_THREADS=1`. One `k0n61` control run was refused by the busy gate
and never ran. Figures are the mean over processes of each process's
median over its repetitions, in ms
([`runs.tar.xz`](runs.tar.xz), [`SHA256SUMS`](SHA256SUMS)).

| row | arm | processes | collect | set-up |
|:--|:--|--:|--:|--:|
| k0n53 | unpaired | 3 | 191.3 | 366.6 |
| k0n53 | paired | 3 | 183.3 (−4.2%) | 372.6 (+1.6%) |
| k1n59 | unpaired | 3 | 201.5 | 404.9 |
| k1n59 | paired | 3 | 189.3 (−6.1%) | 384.8 (−5.0%) |
| k0n61 | unpaired | 2 | 1116.1 | 1418.6 |
| k0n61 | paired | 3 | 1007.1 (−9.8%) | 1313.8 (−7.4%) |

## Reading

- **Collect is faster** at all three sizes, most at 2^47.2, where
  collection is about 80% of set-up.
- **At k0n53, set-up does not follow.** Its build phase varies between
  runs by more than the collect gain. Three processes an arm cannot separate
  a few percent from noise.

## Next

A protocol-first paired A/B with a same-session A/A: one binary and two
arms (`KIC_SCAN_PAIRED=0` or not), its stop and accept rules frozen before
any row. It should run on the Iso Lab EPYC class, which is blocked while
the CodeBuild quota reads 0, or on a declared local host.

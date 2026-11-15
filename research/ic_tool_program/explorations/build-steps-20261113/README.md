# The folded pair-table build: a step probe (diagnostic)

- **Status:** measured, 2026-11-13, on this session's container. A
  diagnostic only: it decides nothing and claims no speed-up.
- **Plan:** [`PLAN.md`](PLAN.md), declared before any run.
- **Report:** [`report/report.pdf`](report/report.pdf).
- **Data:** [`summary.json`](summary.json), with the raw outputs in
  `runs/`.

## Result

ns per stored pair, median over 5 processes, one thread:

| step | k0n53 (2^44.3) | k0n61 (2^47.2) |
|:--|--:|--:|
| set-up | 0.50 | 0.34 |
| row additions | 3.24 | 2.74 |
| row keys | 4.09 | 4.29 |
| **stream push** | **15.07** | **13.85** |
| **counting sort + filter fill** | **11.63** | **12.88** |
| assembly | 0.22 | 0.22 |
| **whole** | **34.58** | **35.48** |

- **Checks:** the steps sum to 98.6–99.3% of the build's wall time, and the
  whole build agrees with the report's build phase to 0.02%.
- **Reading:** placing pairs takes three quarters of the build; forming
  them (additions and keys) takes about 7 ns.

## What would move it ([`microbench/`](microbench/microbench.txt))

On random keys at the same sizes:
- **Push variants:** fewer partitions, write-combining buffers (with or
  without non-temporal stores), a skewed flat buffer and pre-faulted memory
  gain at most 1–2 ns, within the spread.
- **A partition-local filter word:** taking the filter word from the hash's
  top bits, the partition's own bits, cuts placement from 8.5–12.6 to
  5.0–5.7 ns per pair. That is the candidate lever, worth about 4–6% of
  set-up at k0n53 if it carries over.
  - It changes the table format shared with the GPU kernel
    (`gpu/ecc2k/pairtable.cuh`, `fold2k.cu`) and stored tables, so it is
    left as a decision, not made here.

## Instrument

`build_probe` in `src/cryptanalysis/koblitz_index_calculus.rs`: timing
points compiled only under `--features scan-probes`, reported under
`scan_probes.build` by `ic price`.

# rho.negation_lanes: Pollard rho with batched inversions (2026-10-23)

This study adds a new ecbench method, `rho.negation_lanes`, and measures it
against `rho.negation`. The code and counts of `rho.negation` are unchanged.

The new method follows `rho.negation`'s step rule exactly: negation classes,
Wiener–Zuccherato look-ahead, cycle escape by doubling, distinguished points
and the cap. The difference is that it runs `lanes` walks in lockstep on a
prime curve, and each round's additions share one field inversion
(Montgomery's trick, `PrimeCurve::add_many_counted`).

Report: [`REPORT.pdf`](REPORT.pdf) (source `report.html`, figure
`solve-time.svg` / `solve-time.png`). Benchmarks row:
[`benchmarks/20261023-rho-negation-lanes`](../../benchmarks/20261023-rho-negation-lanes/).

## What is charged

The method charges everything `rho.negation` charges, for every lane. That
includes the walks still in flight when another lane's collision recovers
the logarithm, so `S` sits a few percent above `rho.negation`'s.

The `field_ops` block records the work actually performed. For a round of
`n` additions, the shared inverse costs `3(n − 1)` multiplications and one
inversion, then each addition costs `2M + 1S`. `rho.negation` costs
`2M + 1S + 1I` per addition. The `lanes` parameter has no default, because
it changes the search. The method runs on prime curves only.

## Results

Four sessions per curve, all on one binary. Every arm is alternated within
each session over the same targets and rounds, and warm-up runs are
excluded. `rho-neg-aa` is an A/A copy of the reference arm. Ratios are paired
geometric means over the same (session, target, round).

| curve | arm | verified | mean S | median wall | wall / rho.negation | S / rho.negation |
|:--|:--|--:|--:|--:|--:|--:|
| 24-bit | rho.negation | 160/160 | 1.083 | 0.72 ms | 1 | 1 |
| | lanes = 8 | 160/160 | 1.158 | 0.50 ms | **0.676** | 1.074 |
| | lanes = 16 | 160/160 | 1.228 | 0.53 ms | 0.707 | 1.159 |
| 28-bit | rho.negation | 160/160 | 1.002 | 2.67 ms | 1 | 1 |
| | lanes = 8 | 160/160 | 1.045 | 1.44 ms | **0.544** | 1.060 |
| | lanes = 16 | 160/160 | 1.080 | 1.44 ms | 0.545 | 1.105 |
| 32-bit | rho.negation | 120/120 | 0.930 | 9.24 ms | 1 | 1 |
| | lanes = 8 | 120/120 | 0.959 | 4.57 ms | 0.489 | 1.039 |
| | lanes = 16 | 120/120 | 0.989 | 4.44 ms | **0.472** | 1.078 |
| | lanes = 32 | 120/120 | 1.045 | 4.45 ms | 0.476 | 1.147 |

The A/A control's wall ratio is 0.99–1.02. Field inversions per group
operation fall from 1 to 0.04–0.21. Full rows, including lanes = 32 at every
size, are in [`results.json`](results.json).

**This is a wall-time result.** In the counted unit (`S`, GAE/√r),
`rho.negation_lanes` costs 4–30% more than `rho.negation`. It is therefore
not a stronger matched rho reference for index-calculus comparisons, which
are made in that unit. Its use is faster rho solves when wall time matters.

### Why it is faster

At 32 bits, a `rho.negation` step spends most of its time in its one field
inversion: a word extended Euclid of about 110 ns, a chain of dependent
divisions. When that inversion is shared over a round, it costs a few
nanoseconds per step, and the walk becomes instruction-bound.

Callgrind counts the same instructions per solve for both methods: 78 M for
`rho.negation` and 72 M for `lanes = 16` on one 32-bit target, about 680 per
group operation. The difference is that the lanes version retires them
without waiting on the division chain.

Things tried and dropped:

- An exact Barrett reduction for the batched products measured 20% slower
  than the hardware divide on this host.
- A key ring for the cycle window (in the commit) left all 144 probe records
  identical and moved wall time by under 2%.

## Validation

- Every run verified: 2,200 measured runs, and every recovered scalar
  matches its target.
- `ecbench verify --replay-all` re-executes every measured run of all 12
  sessions in a fresh process, with identical counts (2,200 replays,
  [`replay-certificate.json`](replay-certificate.json), receipts in
  `verify/`). The method is new, so no earlier binary can replay it.
- [`validator.py`](validator.py) recomputes all 24 headline ratios from the
  raw records, with no code shared with `summarize.py`
  ([`validator-report.json`](validator-report.json): pass).
- The §5a band for each curve is in `band/`, with the printed tables in
  `band/raw/`. No arm has unpriced native work, so the order holds across
  the band.
- Unit tests in `ic_boundary`:
  - `prime_add_many_counted_matches_the_scalar_law`: the batched sums equal
    the scalar law's, and the tally follows the formula above.
  - `negation_lanes_solve_and_replay`: planted logarithms are recovered for
    1, 4 and 16 lanes, and runs replay exactly.

## Reproduce

```sh
ecbench run --spec specs/lanes-32.json --out OUT --allow-busy
ecbench verify --dir OUT --replay-all --exit-code
ecbench band --dir OUT... --json band.json
python3 summarize.py && python3 validator.py
```

Host: Intel Xeon @ 2.10 GHz (family 6 model 207, AVX-512), 4 vCPU cloud
container, `--cpus auto --allow-busy` (below L2). Binary: ecbench
`b94301af7`.

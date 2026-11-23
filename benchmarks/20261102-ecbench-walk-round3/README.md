# ecbench round 3: the 32-bit inversion unrolled, the tuned walk's states in its key ring (2026-11-02)

Copies for browsing. The study, with the data, the exported inputs and the
figure source, is
[`research/ecbench_walk_round3_20261102/`](../../research/ecbench_walk_round3_20261102/README.md);
the run report is its `reports/run-20261101T2320Z/`.

- `report.pdf`: the run report;
- `round3.svg`, `round3.png` (150 dpi): Callgrind Ir before and after at 32
  and 20 bits, and the interleaved wall rounds with an A/A arm.

Headline (Callgrind Ir, deterministic, against `main` 8931f24b88 built
from the same tree): 32-bit `rho.negation` solve −9.7%, 32-bit
`ic.pipeline` child −9.0%, 20-bit solves −5.8% to −6.1%. Every record
identical. Wall: no difference resolved (A/A spread larger). Engineering.

**No row in `benchmarks/README.md`** (validation gate).

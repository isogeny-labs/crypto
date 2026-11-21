# F4 speed-up round 3: allocator per consumer, fewer per-term allocations (2026-11-21)

**Standing.** `[pcc:unverified]`. One shared 4-vCPU cloud VM, isolation L0, no PMU, no pcc claim filed.

**Scope.** Wall-clock, CPU time and callgrind instruction counts of the F_p F4 engine's index-calculus consumers. Counted operations do not change, so no IC row or gap to rho moves.

Follow-up to #4851 (`research/f4_inner_dispatch_20261120/`). PR #4859.

## Result

| measurement | result | record |
|:--|:--|:--|
| `jv_cover --exp dlp`, p = 53: main + jemalloc vs this branch's f4_fp changes, pre-registered (`CONFIRM_PROTOCOL.md`) | wall **1.090** (13.93 → 12.86 s), CPU 1.091, 11/12 rounds; A/A 1.0045 | `20261121T0018Z-jv-cover-f4-packed-interreduce-confirm` |
| `jv_quartic --exp cprime`, p = 269: main (glibc) vs shipped (jemalloc + f4_fp changes) | wall **1.45** (9.2 → 6.3 s), 4/4 | `20261121T0018Z-f4-consumer-allocator-screen` |
| jemalloc preloaded vs glibc, jv_quartic | 1.40, 4/4 | same |
| jemalloc preloaded vs glibc, jv_quintic weierstrass, p = 271 | **0.94, 0/4: slower** | same |
| callgrind F4 instructions, 104 Nagao tests | 1.317 G → 1.137 G (−13.6 %); whole program −5.0 % | `data/callgrind/instructions.json` |

Every run reproduced identical work: residuals, relations, counted oracle operations or F4 multiplications, and planted or decomposable counts. The callgrind output JSON is identical apart from timing fields.

Together with #4851, `jv_cover --exp dlp` has gone from about 18.2 s (main before #4851) to about 12.9 s. Those numbers come from different days and are not a paired measurement.

## Changes

- **`Cargo.toml`:** `jemalloc` is a default feature. Only binaries that declare it as their global allocator use it: `examples/jv_cover.rs`, and now `examples/jv_quartic.rs`. Every other binary keeps glibc.
- **`src/cryptanalysis/f4_fp.rs`:**
  - **Packed interreduction:** `interreduce_linear` uses packed monomials through `Packed::symbolic` (grevlex, n ≤ 15, degrees < 128). It builds the same matrix; `F4_FP_PACKED=0` keeps the vector path.
  - **Lazy unpacking:** the matrix columns (`Cols`) stay packed until a term is emitted. The reducer multipliers are unpacked only for a recording run.
  - **Interning table:** `MonoTable`, an open-addressing table, interns monomials in `Packed::symbolic`. It starts at 256 slots and doubles.
  - **No counter change:** `Packed::symbolic(count)` keeps the interreduction off the native-operation counter, as before.
- **Tests:**
  - `mono_table_interns_like_a_map`.
  - `packed_linear_interreduction_matches_the_vector_path`: packed against vector, and against the reduced basis, on perturbed bases.

## Why jemalloc is not global

On large F4 systems jemalloc costs more system time than glibc (3.1 s vs 0.85 s on jv_quintic). `MALLOC_CONF` decay -1 and `oversize_threshold:0` did not remove that cost (`data/quintic_conf/`). The rule for adding a consumer was fixed in `scripts/consumer_screen.sh` before data: every round must favour jemalloc and the median ratio must be at least 1.05.

## Rejected along the way

The first interning table was sized from the seed rows' total terms. Clearing it cost more than the probes saved: 1.245 G F4 instructions, against 1.184 G without the table.

## Next

- Monomial interning (17 %) and the reducer search in `Packed::symbolic` (11 %) are the largest remaining F4 costs on these systems.
- `f4_fp::Poly` still allocates one exponent vector per emitted term.

## Layout

- `CONFIRM_PROTOCOL.md`.
- `data/`: raw runs per measurement, callgrind summaries, binary hashes.
- `scripts/`: bash runners, plus `plot_report.py`, which only lays out recorded values.
- `reports/run-20261121T0100Z/`: the report, three graphs and the PDF.

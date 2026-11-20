# Results: block Lanczos vs coordinate Wiedemann on the genus-4 x* core (lead #2 pre-test)

Every number here is `[pcc:unverified]`. The protocol was frozen before any data was taken:
`PROTOCOL.md`, sha256 in `frozen.sha256` (it still checks). Solver: `pretest.py`. Raw data: `raw/`
(hashes in `raw.sha256`). Export binary: `binary.sha256`.

## Verdict: KILL

Over F_r, block Lanczos needs the same number of sparse matrix-vector products as coordinate
Wiedemann. On the real p = 16411 x\* core the best width (b = 1) uses 3675 lane-SpMVs against
Wiedemann's 3676, so `rho_spmv = 0.9997`. That is at or above the 0.80 gate for every width tested.
On charged cost, block Lanczos does worse, because its dense block work exceeds Berlekamp-Massey's.
`rho_ops` is 1.20 at b = 1 and rises with b. Planted-log recovery passed on 4 of 4 toy instances and
on the main instance, so the recovery clause of the kill rule does not apply. The kill comes from
the SpMV clause alone.

## 1. What the binary does, and what it costs (accounting)

- **The solver arm x\* actually uses.** It is not the baseline `wiedemann`. Arm x\* runs SGE
  (`sge_reduce`, w_max 4, fill 1.5) and then `ic_control::wiedemann_coord(square = true)` on a
  random square subset `A'` of the reduced core (N x N, N = reduced cols). It forms the target
  coordinate only. The baseline `wiedemann` on `AᵀDA` runs only as the certified fallback, and it
  never ran: across the study's 240 runs, la_fallbacks = 0 and attempts = 1.
- **Unit.** `Nat { mul, add }` per mod-r operation:
  - `mat_vec` charges 1 add per stored non-zero, plus 1 mul when the entry is not ±1.
  - `berlekamp_massey` charges 1 mul + 1 add per discrepancy term and per update term, plus
    `pivot_inversion_steps(r) = 2·bitlen(r)` muls per inversion.
  - The final coordinate sum costs L mul + L add, then `2·bitlen(r) + 1` muls.
- **Wiedemann cost.** It is `(2N + 2)` SpMVs of `A'` + BM (about `2N²` mul and `2N²` add) + O(N).
  On the exported instance (N = 1837, `A'` nnz = 26,922 of which 1,007 are not ±1):
  - 3676 SpMVs: 98.97 M adds + 3.70 M muls.
  - BM and the coordinate sum: about 6.75 M mul + 6.75 M add.
  - Total: 116,389,511 = 10,669,589 mul + 105,719,922 add.
  - The Python re-implementation reproduces the binary's recorded `la_wied_add` exactly and
    `la_wied_mul` exactly once the 115-mul unit-T inversion is excluded, which sits outside the solve.
    It also returns the same coordinate value.
- **Recorded study (p = 16411, x\*, 60 runs).** Cells record core 2029-2051 x 1995-2018. After SGE
  the cores are 1844-1905 x 1812-1872, with nnz 26,688-28,400 (mean 27,641). `la_wied_mul` has mean
  10.62 M and `la_wied_add` mean 106.4 M. "dim ~2007, nnz ~27641" in the lead mixes the pre-SGE
  column count with the post-SGE nnz. The solver works on the post-SGE matrix, N about 1836.
- **Blocked methods over a large prime field.** They give no count reduction.
  - Over F_2, one 64-bit word carries 64 lanes, so a block is free in operation count.
  - Over F_r (r about 2^56 here), each lane of a block product costs its own mod-r mul or add per
    non-zero.
  - Block Lanczos on `A'ᵀDA'` needs about N/b iterations x b lanes x 2 products (`A'`, `A'ᵀ`), which
    is about 2N lane-SpMVs. That equals Wiedemann's 2N + 2 products of `A'`, because Wiedemann's
    non-symmetric square variant already avoids the `AᵀA` doubling.
  - Blocking cuts block passes over the matrix (memory traffic, parallelism, latency) by a factor of
    b. It does not cut counted work.
  - It adds dense block work of about `5·b·N²` mul and the same in adds: Gram products, block axpys,
    and b x b inverses.

## 2. Counts (main instance: p = 16411, g = 4, curve seed 373, r = 72319133634535537, N = 1837)

| arm | b | lane-SpMV | block passes | mul | add | total | dense (mul+add) | rho_spmv | rho_ops | planted log |
|---|---|---|---|---|---|---|---|---|---|---|
| W (binary's coord Wiedemann) | 1 | 3676 | 3676 | 10,669,589 | 105,719,922 | 116,389,511 | BM about 13.5 M | 1 | 1 | ok |
| L1 | 1 | 3675 | 3675 | 24,164,904 | 115,813,031 | 139,977,935 | 33,962,454 | 0.9997 | 1.203 | ok |
| L2 (pad 1) | 2 | 3678 | 1839 | 41,094,172 | 132,814,414 | 173,908,586 | 67,800,128 | 1.0005 | 1.494 | ok |
| L4 (pad 3) | 4 | 3684 | 921 | 75,085,164 | 166,949,630 | 242,034,794 | 135,740,352 | 1.0022 | 2.080 | ok |
| L8 (pad 3) | 8 | 3688 | 461 | 142,984,744 | 234,930,802 | 377,915,546 | 271,502,016 | 1.0033 | 3.247 | ok |
| L1_rect (whole reduced core 1871 x 1837) | 1 | 3675 | 3675 | 24,289,871 | 117,650,531 | 141,940,402 | 33,962,454 | 0.9997 | 1.220 | ok |

- Block Lanczos uses the recurrence of the lab's `koblitz_sparse_la::block_lanczos_solve`, on
  `A'ᵀDA'`. Lane 0 carries the right-hand side and the other lanes are random.
- "pad" is the number of identity unknowns added so that b divides N. They are charged.
- Only `x[ct]` is formed, as in `wiedemann_coord`. No residual check is charged to either arm,
  because both rely on the group certificate.
- In block-pass units, which this protocol rejects as a measure of work, L8 would read 461/3676 =
  0.125. That is the only reading under which the explorer's 20-30% gain appears.

## 3. Planted-log recovery (toy instances, real `genus_ladder` exports, p = 1031)

| toy | g | r | h | N | W | L1 | L2 | L4 | L8 | L1_rect | rho_ops L1 |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 0 | 3 | 27805807 | 40 | 84 | ok | ok | ok | ok | ok | ok | 1.280 |
| 1 | 3 | 1109725907 | 1 | 85 | ok | ok | ok | ok | ok | ok | 1.302 |
| 2 | 4 | 69923292839 | 16 | 202 | ok | ok | ok | ok | ok | ok | 1.263 |
| 3 | 4 | 144616107149 | 8 | 196 | ok | ok | ok | ok | ok | ok | 1.249 |

- "ok" means the arm returned `x[ct]` such that `x[ct]·h⁻¹ ≡ t_secret (mod r)`, the binary's
  planted log.
- In all 4 toys, the Python W matches the binary's Wiedemann counts and value exactly.
- An extra synthetic check (n = 60, r about 2^56, planted solution) also passed. It is not
  recorded in `raw/`.

## 4. Implied change to the genus-4 x\* row (a projection, not a measurement)

The projection is `new/old = 1 − s_LA (1 − rho_ops)`.
- s_LA is 0.561 on this run, against the study's 0.56-0.60.
- The x\* row at p = 16411, w = 1 is 0.876 [0.872, 0.880] of the ideal negation-rho bound.

| replacement | rho_ops | projected x\* row, w = 1 (s_LA 0.56-0.60) |
|---|---|---|
| none (current) | 1 | 0.876 |
| L1 | 1.203 | 0.975-0.983 (worse) |
| L2 | 1.494 | 1.118-1.136 (worse, above rho) |
| L4 | 2.080 | 1.41-1.44 |
| L8 | 3.247 | 1.98-2.06 |

The explorer's prediction of 0.63-0.68 is not supported. It requires `rho_ops` of about 0.45-0.55,
and that is reachable only by counting block passes instead of mod-r operations.

## 5. Notes for the orchestrator (not part of the verdict)

- **Where a real SpMV reduction could come from.** With several cheap left projections, the
  sequence needs about N/m + N/n terms instead of 2N. For example, block Wiedemann with m unit
  (coordinate) projections and n = 1 needs about N(1 + 1/m) products of `A'`: up to a 50% SpMV cut
  as m grows, because unit projections are free. The price is a matrix Berlekamp-Massey (or
  approximant basis) whose dense cost grows with m and must be charged. It is untested here, so it
  is an analytic hypothesis only. The lab has the machinery in `koblitz_sparse_la::block_wiedemann_kernel`.
- **Possible double charge in `run_ic_full`** (`src/cryptanalysis/genus_ladder.rs`, about lines
  1780-1866). The `if answer.is_none() { ... wiedemann ... }` fallback block and the
  `res.la_wied.merge(w_nat); res.la.merge(w_nat)` after it appear twice in a row. On an attempt
  where the coordinate solve and the first fallback both fail, `w_nat` is merged twice, so earlier
  work is counted again. In the genus-4 study this path never ran (la_fallbacks = 0, attempts = 1 in
  all 240 runs), so recorded rows are unaffected. I have not checked whether this is intentional.
- **Visual report.** Not produced: no figure or PDF. Under the research-visuals rule, a figure and
  PDF should be added before this lands in a PR.

## Files

- `PROTOCOL.md`, `frozen.sha256`: frozen before data.
- `pretest.py`: the solvers and exact counters.
- `raw/main_export.jsonl.gz`, `raw/toy_export.jsonl.gz`: the exported cores.
- `raw/main_run.jsonl`, `raw/toy_run.jsonl`: the binary's own rows.
- `raw/main.json`, `raw/toy_[0-3].json`: per-arm counts.
- `binary.sha256`, `raw.sha256`: hashes.
- Source change: an opt-in dump in `src/cryptanalysis/genus_ladder.rs`, gated by
  `GENUS_LADDER_EXPORT_CORE`. Its sge_coord branch calls the new `export_core`, which writes the
  reduced core, the square subset that `wiedemann_coord` draws (replayed from a clone of the RNG),
  the solve's counted work, and `t_secret`. With the variable unset, behaviour and counts do not
  change: the exported run's row equals the study's recorded row for curve 373.

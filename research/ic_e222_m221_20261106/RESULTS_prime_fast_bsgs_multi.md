# prime_fast multi-target IC vs a measured multi-target BSGS

Status: [pcc:unverified] producer's own measurement; not independently replayed; same machine, wall clock, 1 thread, toy sizes.

Why: the validator found the rho references in `RESULTS_prime_fast_multi.md` mis-sized and the multi-target rho an assumption. This replaces the assumption with a measured arm: `examples/prime_fast_bsgs_multi.rs`, a multi-target baby-step giant-step on the same native engine (baby table x(jG) -> j for j <= B covering +-j via the negation map, giant step M = 2B + 1, B = ceil(sqrt(T n / 4)), all T targets stepped together with one batched inversion per round). Every one of the targets is verified `k G = Q_t`. Same planted targets (same seeds) as the IC evaluation in `results/pf_multi`; raw rows in `results/pf_bsgs/bsgs_*.jsonl`. Timed: baby table, giant steps, lookups and verification (planted targets outside the timed region, as for IC).

Result at T = 1000, fresh curves (the frozen IC evaluation set), all targets verified:

| bits | rows | BSGS total ms (median) | IC multi-target / BSGS (median; per-row) |
|---|---|---|---|
| 23 | 4 | 16 | 3.1x (2.7 to 3.2) |
| 26 | 4 | 44 | 4.6x (3.8 to 6.8) |
| 30 | 4 | 170 to 270 | 8.8x (8.1 to 11.5) |
| 31 | 4 | 275 to 300 | 8.0x (6.4 to 8.7) |
| 33 | 2 | 620 to 700 | 12.6x (11.9 to 13.2) |

Reading: against a measured multi-target BSGS, multi-target IC is 3x to 13x slower at T = 1000, and the ratio grows with size. This agrees with the validator's corrected idealised-rho figures (3.8x to 44x) and shows that the distance to the best multi-target attack is real, not an artifact of the rho reference. BSGS needs sqrt(T n / 4) memory (1.5 million table entries at 33 bits), which IC's O(p^(1/3)) factor base does not; this is a trade the table does not price (no §5a band). The multi-target IC gain over repeated single-target `ic_solve` (134x to 570x) stands; the gain over the best multi-target attack does not exist at these sizes.

Limits: BSGS here is simple (no per-round hash batching), so it can be sped up further, which would widen the gap; 1 thread; T = 1000 only; the 23-bit IC instance rows with inst = 2 have no BSGS counterpart; not independently replayed.

## Why the gap grows (counted-operation reasoning; not a measurement)

Multi-target BSGS costs about 2 sqrt(T n / 4) group additions in total, so about sqrt(n / T) per target, which falls as T grows. Multi-target IC pays a one-time setup of order p^(2/3) additions (at 33 bits: 4.2 s) and then a per-target relation search of order fb = c p^(1/3) additions, a floor that T does not lower (measured 0.8 ms at 30 bits and 1.3 ms to 8 ms per target in the wider runs). So for T beyond about sqrt(n) / fb^(...) the per-target floor of IC exceeds BSGS's per-target cost; BSGS's table (sqrt(T n / 4) entries) is the price. Within the sizes measured, no T makes IC cheaper than BSGS; at T = 10^6 and 33 bits BSGS needs about 46 additions per target against IC's O(10^3). This is the Gaudry-Diem obstruction in multi-target form; the lead is documented as blocked for prime-field Semaev-S3 IC with this factor base. Remaining unexplored directions: a factor base with symmetry-reduced per-target search (cost below fb per target), or sharing the per-target sweep across targets (batched sweeps give fewer inversions but not fewer lookups).

Correction (validator, `results/pf_multi2/VALIDATOR_REPORT.md`): this BSGS is untuned; with batched baby steps and a cheaper hash it is 2.6x to 3.9x faster, so every IC/BSGS ratio above is understated by that factor (multi-target IC with the sweep collector is then roughly 8x to 49x behind, with the table collector 7.6x to 16.8x behind).

---
title: "Protocol: a native, independent-prime finite degree-growth ladder for the base-2 digit presentation of the prime-field Semaev system"
date: 2026-10-29
status: registered before the counted runs (hash in REGISTRATION.sha256)
---

# Protocol: digit-presentation degree ladder

**Lead.** `LEAD-ICP-digit-presentation-solving-degree` (crypto-autoresearcher atlas), next action: "run the certified finite degree-growth ladder with blocking controls (H-PFDR-d5f90f) on the digit system at m = 2, 3 over s = 2..8 with independent primes per s (fixing the pseudoreplication of EV-PFDR-99c699) and the support-matched and block-factored nulls".

**Track.** Algebraic. A degree is not a solving degree, not a cost and not an exponent: nothing here adds a dashboard point or a `benchmarks/` entry.

## 1. Question

Does the last fall degree `d_last` of the base-2 digit-presented Semaev system `S̃ = S_{m+1}(ℓ_1, …, ℓ_m, x_R)` over `F_p` stay bounded as the digit count `s` grows, strictly below the structure-free nulls? If so, it is the first early fall over `F_p` and reopens the conditional cost table. If `d_last` grows with `s` at the rate of the closed form of H-PFDR-4148b8, the lane is exhausted at the measured scale.

## 2. Objects (all native; no Python)

- `ℓ_k = Σ_{i<s} 2^i a_{k,i}`, digits `a_{k,i}` Boolean, ring `B = F_p[a]/(a² − a)`, `n = m s`. `S̃` is the reduction of `S_3(ℓ_1, ℓ_2, x_R)` (m = 2) or `S_4(ℓ_1, ℓ_2, ℓ_3, x_R) = Res_u(S_3(ℓ_1, ℓ_2, u), S_3(u, ℓ_3, x_R))` (m = 3) in `B`, over `y² = x³ + ax + b`.
- Conventions are those of the Huang–Kosters–Yeo engine `examples/hky_last_fall_degree.rs`, ported from `F_2` to `F_p` (see `digit_fall.rs`, header):
  - Macaulay rows `m·f` with `deg m + deg f ≤ c`, multiplied in one variable at a time, a product leaving degree `≤ c` skipped;
  - `V_{F,c}` = mutant closure (every row of degree `< c` multiplied by every variable);
  - operational first fall `ffd` = first `c` at which the plain Macaulay space gains a row of degree `< c` beyond the previous plain dimension;
  - `e0` = first `c` with `dim V_{F,c} = dim(I ∩ R_{≤c})` and `c ≥ DG`; `d_last` = least `c ≤ e0` from which every `c' ∈ [c, e0]` agrees.
- **Exactness.** `B` is a function algebra on `{0,1}^n`, so `(f)` is radical and equals `I(S)` for the zero set `S`. `dim(I ∩ R_{≤c})` and `DG` are read from the evaluation rank on `S` (Buchberger–Möller scan), with no Gröbner basis. `d_last` is therefore a certified exact value whenever the run is not marked `censored`.
- **Controls.**
  - `Usupp`: random nonzero coefficients on exactly the monomial support of `S̃` (support-matched null).
  - `Ublock`: `∏_k q_k`, `q_k` random on the support of `ℓ_k^e` plus the constant, `e = 2^{m−1}` (block-factored null).
  - `plain`: the same Semaev polynomial in `F_p[x_1, x_2]/(x_i^p − x_i)` with the unknowns ranging over all of `F_p`, run through the same engine.

## 3. Panel

The replication unit is a draw. Every draw has its **own prime, curve and target** from its own seed, which fixes the pseudoreplication of EV-PFDR-99c699. Primes lie in `(2^b, 2^{b+1})` with `b = min(2s + 1, 11)`, so `p > 2^s` always.

| panel | m | s | arm | target | draws per cell |
|:--|--:|:--|:--|:--|--:|
| A | 2 | 2..7 | `U` (real, unsatisfiable) | random target, unsatisfiable system selected by search | 12 (s ≤ 5), 8 (s = 6), 4 (s = 7) |
| B | 2 | 2..8 | `P` (real, planted) | `x_R` = x of a planted decomposition; the planted digits are a solution | 8 (s ≤ 6), 4 (s = 7), 2 (s = 8) |
| C | 2 | 2..7 | `Usupp`, `Ublock` | unsatisfiable nulls | 8 (s ≤ 6), 4 (s = 7) |
| D | 3 | 4 | `U`, with closure | | 4 |
| D | 3 | 4, 5 | `U`, first fall only | | 4 |
| E | – | `p ∈ {7, 11, 13, 17, 19, 23, 29, 31}` | `plain` | random target | 4 per p |

`cmax` is 14 (panels A–C), 16 (D) and `2p + 2` (E). A draw whose `e0` is not reached by `cmax` is `censored` and is not counted as a value.

**Arm `U` regime.** At `s ≥ 5`, `p < 4096 ≪ 2^{2s}`, so most random targets are satisfiable; the search keeps only unsatisfiable draws. At `s = 8`, `p < 4096` leaves no unsatisfiable target in the 2^16-point grid, and the `U` arm stops at `s = 7`. That is the scale limit of this study's native `u32` accumulators (`p < 2^12`), not a mathematical one.

## 4. Predictions (registered)

Each is [predicted]. H-1 and H-2 repeat the existing ledger (EV-PFDR-1394a4, EV-PFDR-99c699); the rest are new.

- **P1 (closed form).** `ffd = 4 + ⌊s/2⌋` at `m = 2` on every real draw (5, 5, 6, 6, 7, 7, 8 for `s = 2..8`), and `ffd = 13` at `m = 3`, `s = 4, 5`.
- **P2 (last fall = first fall).** `d_last = ffd` on every real unsatisfiable draw, so `d_last` grows by 1 for every 2 digits. This holds on every independent prime.
- **P3 (support-matched null).** `Usupp` has `ffd = d_last = D_null = s + 3` (`m = 2`), strictly above the real system for `s ≥ 3`.
- **P4 (block-factored null).** `Ublock` reproduces the real `ffd` and `d_last` on every draw.
- **P5 (planted).** `d_last = ffd` on every `P` draw too.
- **P6 (plain).** The plain presentation falls late: `ffd = d_last` grows linearly in `p` (earlier probes read `p + 2`). That is exponential in `s` for `p ≈ 2^s`.

## 5. Decision rules

- **Reopen** if, for the real system, `d_last` at `s = 7` is at most `d_last` at `s = 3` (bounded in `s`) on every draw. Or if `d_last` is below the `Ublock` null in any cell. Or if any real draw has `d_last < ffd`.
- **Exhausted at the measured scale** if P1 and P2 hold on every draw and `d_last` follows the closed form on all independent primes. The report then states the scale reached (`s ≤ 7`, `m = 2`; `m = 3` at `s = 4, 5` for `ffd`) and the scale that would reopen it.
- Any prediction that fails is reported with its draws. A failed P4 would mean the block structure alone does not explain the integers.

## 6. Analysis and outputs

`derive.mjs` (node, tooling) reads `results/*.jsonl` and writes `analysis/tables.md`, `analysis/summary.json`, the figures (`degree_vs_s.svg`, `plain_vs_digit.svg`, each with a PNG render) and the README numbers. It counts the distinct `(p, a, b, x_R)` per cell. It also reports the column count `Σ_{j ≤ d_last} C(n, j)` (`[derived]` from the measured `d_last`) beside the plain presentation's `p²`.

## 7. Disclosure

- **Tool validation.** Before registration, `digit_fall` was run on seed 7 at `s = 2..8` for `U`, `P`, `Usupp`, `Ublock`, at `m = 3, s = 4` for the first fall, and at `p = 7, 11, 13, 17` for `plain`. This was a check that the F_p port reproduces the ledger integers (5, 5, 6; 13). Those draws are not part of the panel, and no registered prediction was changed after seeing them beyond P6's wording ("earlier probes read p + 2").
- The `Ublock` null was first generated without a constant term, which made it always satisfiable (`q(0) = 0`). It was corrected before any panel run.
- **Scope.** Toy scale: `p < 4096`, `s ≤ 7` (8 for planted), `m ≤ 3`. One convention (the function-algebra quotient `B`). The ordinary-ring convention of `research/guarded_fall_degree_20261028/` is a different object.
- **Formalization.** No new theorem is claimed. The closed form for `ffd` is cited, not proved here (it rests on Wilson's rank theorem H-WIL in the ledger). `lean/NullTable.lean` checks the arithmetic of the registered table only; the blocker for the closed form is recorded in the README.

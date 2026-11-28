# PROTOCOL — the 2-torsion symmetrised frame as a *degree* lever (frozen)

Slug `rootfind_degree_lever_20261123`. Frozen 2026-11-23, before any
positive-control or arm data. The instrument was built and the fidelity gate G0
(replay of the existing instrument's pilot rows) was started before freezing;
G0 is instrument validation, not data on the question. Nothing below is
changed after data is taken; deviations go to `README.md` under "Deviations".

All numbers this study produces are `[pcc:unverified]`, isolation L0 (shared
4-vCPU container, three agents). No leaderboard, no `benchmarks/README.md` row.

## 1. Question

On binary Koblitz curves (`b = 1`; the binary `S_3` is `a_2`-independent), does
writing the Weil-descended decomposition system in the **2-torsion invariant
(Artin–Schreier) frame** — `u = 1/(x+1)`, `w = u² + u`, `s = Σu`, factor base
`{P : u(P) ∈ V}`, `1 ∈ V` — change **how the last fall degree `d_F` grows with
`n`**, compared with the production `x`-frame systems at the same `F_2`-dimension
`ℓ = n/3`?

Why this lever (and not the other candidates). See README "Lever choice". In
short: it is the only candidate that changes the *equation degree and the
variable count of the descended system* (m = 3: Boolean degree 4 in `3ℓ−2`
unknowns, against degree 6 in `3ℓ` for direct `S_4` and degree 3 in `3ℓ+n` for
the chain), which is the mechanism by which a solving/fall degree could grow
differently in `n`. Its *time* and *first-fall-proxy* behaviour were measured
before (RESEARCH_EXOTIC_COORDINATES §8, §17, §20; H2 there falsified); its
`d_F` and `d_F`-vs-`n` slope never were. 4-torsion: §10.2–10.4 there showed it
gives no per-point degree drop on `K_0` (it is the `π − 1` transport), so it is
not a degree lever. Trace-cut / `V·V`-structured bases: the polynomial window
already minimises `dim V·V = 2ℓ − 1` and is a baseline arm here.

## 2. Instrument

`examples/rootfind_degree_lever_20261123.rs` (std-only, compiled with
`rustc -O`; see README for why not `cargo`). Per system it computes the HKY
last fall degree `d_F` (least `c` with `1 ∈ V_{F,c}`, mutant closure in the
Boolean ring), copied from `examples/binary_falldeg_arms.rs` / `hky_par.rs`.

Gates:
- **G0 fidelity**: `rdl replay` re-derives `d_F` and every per-degree
  `(plain, closed)` closure dimension of the 72 rows of
  `research/binary_falldeg_20261118/raw/pilot.jsonl` (library-built systems).
  Pass = 72/72 identical. Failure → instrument rejected, no arm data reported.
- **G1 selftest**: irreducible choice is irreducible; chain `m=2` ≡ direct
  `m=2` ANF; the 24-term `S_4` and the 18-term symmetrised `S_4` (and 5-term
  `S_3`) vanish on genuine chained relation tuples.

## 3. Positive control (can the detector see a degree change of 1?)

Random dense inconsistent quadratic Boolean systems (`k` vars, `e` eqs), seed
`20261123`, 3 draws each, specs `(k,e) = (8,16), (12,14), (18,19)`, whose
Boolean semi-regular degrees of regularity are 3, 4, 5 (computed from
`(1+z)^k/(1+z²)^e`). **Pass** iff the per-spec median `d_F` is strictly
increasing across the three specs and every median is within ±1 of the
semi-regular value. If it fails, the study reports "instrument cannot resolve
degree changes" and the H1 verdict is *inconclusive*.

## 4. Arms (same target `x_R`, same draw)

| arm | system | factor-base space | role |
|---|---|---|---|
| `xchain-rand` | production chain (`build_decomposition_system` layout), `S_3` links, `3ℓ+n` vars, degree 3 | random `V ∋ 1` (`V_A`) | baseline (existing *random* arm) |
| `xchain-win` | same | window `span{1,z,…,z^{ℓ−1}}` | baseline (existing *subspace* arm) |
| `xdirect-rand` | unchained `S_4` in `x`, `3ℓ` vars, degree 6 | `V_A` | control: "no intermediate variables" without the symmetry |
| `sym-rand` | symmetrised `S_4`, `3(ℓ−1)+1` vars, degree 4 | `V_A` | **lever** |
| `sym-rand2` | same | independent random `V_B ∋ 1` | **A/A** against `sym-rand` |
| `sym-win` | same | window | lever on the structured space |

The orbit arm is not run: a Frobenius-stable `V ∋ 1` exists only for special
`n` (2 not primitive), and `binary_falldeg_20261118` already found no orbit
effect on `d_F`. A draw is kept only if **every** arm's system is inconsistent
(no root): `x`-frame consistency is decided exactly on the direct system (chain
roots ⊆ direct roots), `u`-frame on its own truth table.

## 5. Cells and censoring

- Primary ladder `m = 3`, `n ∈ {9, 12, 15, 18, 21}`, `ℓ = n/3` (fixed ratio).
- Secondary ladder `m = 2`, same `n`, `ℓ = n/3`.
- `K = 4` kept draws per cell; master seed `20261123`; draw seeds derived as in
  `binary_falldeg_arms` (`cell_seed`).
- Closure degree cap `cmax = 8` (`xchain` at `m = 3`: 5); a degree `c` is not
  attempted if the monomial count `Σ_{i≤c} C(n_vars, i) > 150 000`; a system's
  closure has a 600 s wall budget. A censored row records `d_F > c_last`.

## 6. Metrics

- **Primary**: `d_F` per row; per-arm slope `β_a` = OLS slope of `d_F` on `n`
  over all kept rows of the `m = 3` ladder.
- Secondary: excess `d_F − input degree`; cost proxy `L = log2 Σ_{i≤d_F}
  C(n_vars, i)` (Macaulay columns at the fall degree) and its slope; the
  semi-regular `d_reg` of each system as a yardstick.
- Uncertainty: 2 000-sample bootstrap over draws within cells (percentile 95 %
  intervals) for slopes and slope differences. Censored values enter as
  `c_last + 1` (primary) and `c_last + 3` (sensitivity).

## 7. Hypotheses, predictions, falsifiers

- **H1 (lever changes growth).** `β_sym-rand < β_xchain-rand` **and**
  `β_sym-rand < β_xdirect-rand`, each difference's 95 % interval entirely
  below 0, under both censoring treatments, and the A/A condition holds.
  **Prediction (preregistered, contrarian): H1 is not supported** — the frame
  shifts `d_F` and `L` by an offset (fewer unknowns, lower input degree) but
  the slope differences' intervals contain 0. *Falsifier of the prediction*:
  H1's condition met.
- **H2 (no excess at m = 3).** Prediction: median `d_F(sym-rand)` equals its
  input degree 4 in every `m = 3` cell. Falsified by any cell median ≥ 5.
- **H3 (cost offset).** Prediction: `L(sym-rand) < L(xchain-rand)` in every
  `m = 3` cell where both are uncensored. Falsified by any cell where it is not.
- **A/A.** `sym-rand` vs `sym-rand2`: per-cell medians differ by ≤ 1 and the
  slope difference's 95 % interval contains 0. If not, the noise floor is
  larger than 1 degree and H1 is inconclusive.
- **Window vs random** (exploratory, inherited from `binary_falldeg`): reported
  without a verdict.

## 8. Stopping rule

One pass over both ladders in the order: positive control, `m = 2` ladder,
`m = 3` ladder, increasing `n`. Total wall cap 2 h for arm runs; cells not
reached are reported as "not run". No additional cells, seeds or draws are
added for the verdict; anything added later is labelled exploratory.

## 9. What a result can and cannot mean

A slope in `d_F` over `n ≤ 21` is a toy-range statement. A null says the frame
did not change degree growth *in range*; it does not bound asymptotics. A
positive H1 would be a lead for a larger host, not an improvement claim: no
relation-phase cost, no rho comparison, no success-criterion row.

# The symmetric PKM tower: F4 solving degree against the plain Kummer tower

2026-10-26 · item 1 of the 2026-10-26 triage (`docs/session-logs/2026-10-26.md`) ·
instrument [`examples/pkm_symmetric_tower.rs`](../../examples/pkm_symmetric_tower.rs) ·
records `rows.jsonl` (m = 2), `rows_m3.jsonl`, `rows_m3_symmetric.jsonl` (m = 3), `run.log` ·
graph [`d_vs_n.svg`](d_vs_n.svg) (`plot.py`, recorded values only) · PDF `report.pdf`

**Class: stage diagnostic of the solver axis (AGENTS.md §2). No pipeline is
priced, no `S`, no leaderboard row.** It answers one question about the factor-base
encyclopedia's flagship tower family (`docs/ic/factor-base-encyclopedia/README.md`
§0.1, §8): does combining the Kummer membership and the point-sum constraint in the
elementary symmetric functions, *before* expansion, change the F4 solving degree?

## Verdict

**Yes, by one degree, and with matrices about 10× narrower at m = 3. The growth of
`D` with `N` is not removed.** The symmetric tower's `D` still rises (4, 5, 5, 6 at
`m = 3`, `N` = 6, 9, 12, 15). The effect is a shift of the staircase, as far as
these cells can show, not a different law. It does not settle lead 1, whether `D`
stays bounded as `N` grows.

| m | N | plain chain D | plain, one S₄ D | **symmetric D** | widest matrix, plain / sym | time per system, plain / sym |
|--:|--:|--:|--:|--:|--:|--:|
| 2 | 10 | 4 | — | 4 | 606 / 331 | 0.05 s / 0.01 s |
| 2 | 12 | 5 | — | **4** | 2,451 / 627 | 0.36 s / 0.09 s |
| 2 | 14 | 5 | — | **4** | 5,031 / 1,188 | 3.6 s / 0.44 s |
| 2 | 16 | 5 | — | 5 | 9,119 / 6,281 | 34 s / 24 s |
| 2 | 18 | 5 | — | 5 | 14,922 / 12,214 | 280 s / 176 s (one target) |
| 3 | 6 | 5 | 12 | **4** | 368 / 35 | < 0.01 s |
| 3 | 9 | 6 | 12 | **5** | 3,285 / 248 | 0.6 s / 0.02 s |
| 3 | 12 | 6 | 12 | **5** | 12,314 / 1,224 | 14.8 s / 0.3 s |
| 3 | 15 | ≥ 6 (900 s budget stop, 97,456 columns) | aborted | **6** | ≥ 97,456 / 10,532 | > 900 s / 43 s |

Values are the maximum over the two random targets of each cell (identical in every
cell where both finished). "Plain, one S₄" replaces the S₃ chain by the single
summation polynomial `S₄` (by resultant). Its D = 12 is `S₄`'s own degree, so a
single high-degree `S_{m+1}` without symmetrisation is the worst arm. The recorded
tower-engine D for the plain Kummer tower at `m = 3, N = 15` is 7
(encyclopedia §0.1). The plain `f4_fp` run here did not finish within its budget.

## Construction

Kummer base `V = g·μ_{2^t}`, summands `x_i = g·w_i`. Distinct `w_i ∈ μ_{2^t}` ⟺
`Q(X) = ∏(X − w_i)` divides `X^{2^t} − 1` ⟺ `X^{2^t} ≡ 1 (mod Q)`. The unknowns
are the `e_k` (coefficients of `Q`) and the coefficients of `R_j ≡ X^{2^j} mod Q`
for `j = 1..t−1`, so `N = m·t` as in the plain system. The equations are
`R_j = R_{j−1}² mod Q`, `R_{t−1}² ≡ 1`, and `S_{m+1}(g w, x_R)` rewritten in
the `e_k` (leading-term algorithm). The symmetric system sees unordered summand
sets of distinct points only. That is the same relation for index calculus, with
`m!` fewer solutions; doublings `x_i = x_j` are not represented.

Controls and checks:

- **Positive control on the instrument.** The plain arm is the pilot's system
  (`examples/pkm_tower_pilot.rs`, `tower` control). On cells run by both, the pilot
  binary gives the same D and the same matrix widths (9, 37, 127, 280 columns at
  N = 2..8).
- **Planted check.** Every system of every arm is first evaluated on a planted
  decomposition, distinct summands from `V` and an `x_R` solving `S_{m+1}`. Any
  non-vanishing equation stops the run.
- **Reproducibility.** The symmetric-only rerun (`rows_m3_symmetric.jsonl`)
  reproduces every symmetric row of `rows_m3.jsonl` exactly (same D, widths,
  inconsistency).
- Same prime (`p = 786433 = 3·2^18 + 1`), curve, `g`, `ζ` and targets for all
  arms. The seeds are fixed in the source. Engine `f4_fp`, grevlex, degree bound
  `n + d + 6`, measured on random targets (`--random 2`), as the recorded Ds were.

## What did not run, and why

- `m = 2, N = 18`, target 1, plain chain: ran about 55 minutes and 4.4 GB against
  a 900 s budget. The operator stopped it, because `f4_fp` checks its budget between
  steps, not inside one. Its symmetric row was not run (`run.log`).
- `m = 3, N = 15`: the plain chain hit its 900 s budget (D ≥ 6, 97,456 columns).
  The plain-`S₄` arm then aborted (exit 134, an allocation failure under the 10 GB
  cap) before the symmetric arm ran. The symmetric arm was re-run on its own: the
  draws do not depend on which arms run.
- `m = 4` and `N > 18` were not attempted on this 4-core, 15 GB container.
- The resume checkpoint could not be pushed: the container has no S3 token.

## Reading and next step

The symmetric presentation is a real constant-factor improvement for the
decomposition oracle's F4 stage: one degree less and about 10× narrower matrices at
`m = 3` for `N ≤ 15`, and about 50× faster per system at `N = 12` (14.8 s against 0.3 s) and more
than 20× at `N = 15` (over 900 s against 43 s). It is not an
index-calculus improvement until it is priced end to end. Its D keeps growing, so it
does not answer lead 1 (bounded D). The next measurement that would matter is the
symmetric arm at `m = 3, N = 18, 21`, which is cheap (43 s at N = 15), and at
`m = 4`. If D still steps up every one to two cells, the family's
subexponentiality question stands exactly as before, one degree lower.

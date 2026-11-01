# L4-A: measured S₄ small-root reach of symmetric lattices (pre-registration, 2026-10-28)

**Status.** Registered after a timing pilot and before any measured cell.
Public synthetic curves; no security claim. **Class: control.** It
measures, for the first time in this program, rows that README §3.2 only
derived: the m = 3 (S₄) reach of symmetric Jochemsz–May lattices. It is
not an improvement under the success criterion.

## Code

[`examples/semaev_lattice_reach.rs`](../../../examples/semaev_lattice_reach.rs),
native Rust, on main's `lattice::lll_reduce` (δ_LLL = 0.75). Per target:

- symmetrise `S₄(·; z)` into `g(e₁, e₂, e₃)` by a 35-unknown linear solve,
  checked on 6 extra points;
- build the level-ℓ lattice described in the file;
- reduce it;
- evaluate every reduced row over ℤ at the planted `e*`.

## Pilot (timing only, not a result)

One planted and one unplanted trial per configuration, seed 7, 40-bit
curve, on `semaev_lattice_reach-pilot` (the registered binary differs only
in being rebuilt from the same source).

| m | ℓ | dim | LLL time |
|--:|--:|--:|--:|
| 3 | 1 | 35 | 6–20 ms (smoke run) |
| 3 | 2 | 165 | 116 s |
| 3 | 3 | 455 | see `pilot/` (killed after the cap if unfinished) |
| 4 | 1 | 495 | not started (queued behind ℓ = 3) |

Fitting d⁴–d⁵ from 35 to 165 puts dim 455 at 1–3 h per reduction. Main's
LLL is BigInt Gram–Schmidt. So:

- **Measured here:** m = 3 at ℓ = 1 (dim 35) and ℓ = 2 (dim 165).
- **Not run (cost):** m = 3 at ℓ ≥ 3, and all of m = 4 (dims 495 and
  4845). Those rows stay `[derived]` in README §3.2. This is the advisor's
  "paper-only unless the fit says otherwise", and the fit does not say
  otherwise.

## Cells

Curves (all with `a = −3`):

| bits | p | b |
|--:|--:|--:|
| 32 | 4294967291 | 31 |
| 40 | 1099511627563 | 56 |
| 48 | 281474976710591 | 9 |

- **ℓ = 1:** all three curves; `B ∈ {1, 2, 3, 4, 6, 8, 12, 16}`; 8 planted
  and 8 unplanted targets per cell.
- **ℓ = 2:** 32 and 40 bits; `B ∈ {1, 2, 3, 4, 6}`; 4 planted and 4
  unplanted per cell. That is about 80 reductions at about 2 min each.
- Seed `20261028`; per-cell RNG streams are derived in the program.
- Planted: `x_i` uniform in `[1, B]`, and `z` a root in `F_p` of
  `S₄(x; Z)`, resampled until one exists. Unplanted: uniform `z`.

## Measurements and readings

- **Success** (planted): `hg_holds` and `vanishing_rows ≥ 3`. Three rows
  vanishing over ℤ is the heuristic requirement for recovering `e*` by
  resultants; recovery itself is not run.
- **Reach** per (bits, ℓ): the largest `B` with at least half of the
  planted targets succeeding. As δ: `log B / log p`.
- **Unplanted:** the `hg_holds` rate, reported as decision behaviour.
- **Costs:** `lll_ms` against the enumeration cost `(B + 1)^{m−1}`.

**Predictions** (README L4-A, from the §3.2 determinant bounds plus the
m = 2 slack of at most 2.5×):

- ℓ = 1: `δ_1 = 1/210`, so B = 1 at every size, with slack up to B ≈ 1–2.
- ℓ = 2: `δ_2 = 1/55`, so B ≈ 1.5 at 32 bits and 1.7 at 40 bits, with
  slack up to B ≈ 2–3.
- At every cell the reduction costs more than the enumeration it would
  replace.

**Falsifiers:**

- Success at ≥ 4/8 (ℓ = 1) or ≥ 2/4 (ℓ = 2) at `B ≥ 4` on 40 bits, with
  reach not decreasing in p. That is structure-specific slack beyond the
  §3.2 bounds and reopens L4-B.
- An unplanted `hg_holds` rate above 0 at any `B ≥ 2`. That would show the
  short vectors are not root-driven, and the success column would mean
  nothing.

**Limits.** `B` is coarse at these sizes: one step of B moves δ by
`log₂(B)/bits`. The milestone δ = 1/4 is `B = 2^{10}` at 40 bits, so any
real lead would be unambiguous, but small reaches are only resolved to
whole B.

# Standard benchmark panel

**Frozen 2026-11-10.** New index-calculus and rho measurements use these
curves. A result on any other curve is allowed, but it states why it left
the panel. The data is [`standard_panel.json`](standard_panel.json).
[`scripts/build_curve_registry.py`](../../scripts/build_curve_registry.py)
copies each alias into its registry entry as `panel_alias` and fails if a
row disagrees with the registry.

Results were hard to compare because nearly every study chose its own
curves. The panel fixes one ladder. Each rung keeps its ICV1 identity, and
each gets a short alias that prose can use.

## Short aliases

| prefix | family | form | example |
|:--|:--|:--|:--|
| `K` | Koblitz, `y² + xy = x³ + ax² + 1` over `GF(2^n)` | `K<a>-<n>` | `K0-23` |
| `B` | binary, not Koblitz | `B<n><letter>` | `B23a` |
| `P` | prime field | `P<bits of p><letter>` | `P36a` |

- **One alias, one ICV1.** `K<a>-<n>` fixes the curve up to isomorphism,
  but the registry can hold several models of it (two moduli for `K1-23`,
  three models at `n = 83`). The panel pins one model, and only that model
  carries the alias.
- **`B` and `P` letters are slots.** `a` is the first curve of that size
  on the panel, `b` the second, and so on. A letter never moves to another
  curve.
- **Machine records keep ICV1.** JSON, ecbench specs and records, and file
  names carry the slug or full ICV1 string. The alias is for prose and
  tables, and `curve_id` resolves it like any other registered name.
- **Lint.** [`scripts/check_curve_names.py`](../../scripts/check_curve_names.py)
  rejects an alias-shaped name (`K0-29`, `P45a`) that is not on the panel.
  It checks `docs/ic/` and research directories dated 2026-11-10 or later.
  Older text used `P35a`, `P45a` and similar as local instance labels, and
  those labels stay as written. Code fences are exempt.

## The panel

| alias | ICV1 slug | role | log₂ r | cofactor | decompositions |
|:--|:--|:--|--:|--:|:--|
| `K0-13` | `icv1-f2m13-t181-515ee569` | smoke | 10.97 | 4 | exhaustive, m ≤ 16 |
| `K1-17` | `icv1-f2m17-tm101-00378d4e` | exhaustive | 16.00 | 2 | exhaustive, m ≤ 16 |
| `K0-19` | `icv1-f2m19-t797-b6cf2467` | exhaustive | 17.00 | 4 | exhaustive, m ≤ 16 |
| `K1-19` | `icv1-f2m19-tm797-9c54981b` | exhaustive | 18.00 | 2 | exhaustive, m ≤ 16 |
| **`K0-23`** | `icv1-f2m23-t5197-69e76b73` | **primary** | 21.00 | 4 | exhaustive, m ≤ 16 |
| `K1-23` | `icv1-f2m23-tm5197-1f85e9e1` | twin of the primary | 22.00 | 2 | exhaustive, m ≤ 16 |
| `K0-41` | `icv1-f2m41-tm2308219-7f48b14a` | scaling anchor | 39.00 | 4 | sampled, m ≤ 4 |
| `K1-83` | `icv1-f2m83-t6151469093347-9eba6cd5` | scaling anchor (the repository's N83) | 52.93 | 1,128,547,018 | sampled, m ≤ 4 |
| `K0-131` | `icv1-f2m131-tm22283658519494248867-115e0dc5` | target (`ECC2K-130`) | 129.00 | 4 | — |
| `P24a` | `icv1-fp24-tm5099-a6f137c6` | prime | 24.00 | 1 | — |
| `P32a` | `icv1-fp32-t7319-90da43e9` | prime | 32.00 | 1 | — |
| `P36a` | `icv1-fp36-t454193-44a20818` | prime | 36.00 | 1 | — |
| `P40a` | `icv1-fp40-tm924137-0ecaaeb9` | prime | 40.00 | 1 | — |

**Why these.**

- **Continuity.** The `K` rungs from 13 to 23 are the round-0026 tournament
  cells. `K0-41` was the closest cell of the nine-size ladder. `K1-83` is
  the model the N83 work used most. The `P` rungs are the prime-frontier
  `a = −3` curves. Earlier results on these curves stay comparable without
  a rerun.
- **Exhaustive ground truth through n = 23.** One dynamic-programming pass
  over the group counts, for every target and every m ≤ 16, the
  decompositions over a given factor base. The pass costs about
  `m · |F| · #E` group operations: about 2³² at `K0-23` with `|F| = 2⁵`.
  It is independent of any solver, so every F4, SAT or pair-table result
  on these rungs can be checked against it, including an "unsatisfiable"
  verdict.
- **m = 16.** A typical target decomposes once `m·ℓ ≥ n + log₂ m!`. At
  `m = 16` that is `16ℓ ≥ n + 44.25`, so `ℓ = 4` through `n = 19` and
  `ℓ = 5` at `n = 23`. The chained-`S₃` Weil descent at `K0-23, m = 16`
  has `16ℓ + 14n = 402` Boolean unknowns and `15n = 345` equations.
  Whether the in-house F4 closes it is the measurement; the exact count
  always finishes.
- **Every rung has an exact matched rho.**

## Changing the panel

Add a row to `standard_panel.json`. Never re-point an existing alias.
Then run `python3 scripts/build_curve_registry.py` and
`python3 scripts/test_standard_panel.py`, and commit the panel, the
registry and the generated alias map together.

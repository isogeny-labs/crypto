# L22/L23 — H016 re-score against a negation-closed null (2026-11-24)

**Leads:** L22 and L23 in `crypto-autoresearcher/docs/open-leads-20261123.md`.

**L22:** replace/augment H016's `"random_base": "B distinct uniform logs in
[0,N)"` with a negation-closed random base, before `EXP-FB3-001`.

**L23:** reproduce the H016 base families and compare yields under both nulls.

## Data provenance

H016 results live at
`crypto-autoresearcher/inputs/h100_session/h016_base_yield.json` as a VLT1
authenticated blob. It decrypts in a checkout with the vault filter configured:

```sh
python3 tools/vault/vaultctl.py smudge < inputs/h100_session/h016_base_yield.json > h016.json
```

(requires `$KEY`; no plaintext or key is committed here). The record contains
four cells: 14 bits (p=8287, B=20; p=14083, B=24) and 17 bits (p=128629, B=51;
p=113683, B=48), each with 800 targets and three stored bases: `qr`,
`small_mult`, `random`.

## Null definitions

Adopting the H016 convention that a base is a set of **B logs**:

- `uniform` (H016): B distinct uniform logs in `[1,N)`.
- `symmetric` (L22): B/2 distinct logs `s` uniform in `[1,(N-1)/2)`, base
  `{sG, -sG}`. This is negation-closed and has exactly B points.

**Size caveat.** A B-log symmetric base has only B/2 distinct x-values, while
H016's `small_mult` (`x(jG)`, j=1..2B, deduplicated by x) and `qr` bases are
built as B-distinct-x objects. The "same size" clause is therefore read as
B logs, matching H016's own `random_base` definition. An x-support-matched null
would use 2B logs and is a different object; it is not run here. This is the
remaining contract ambiguity for `EXP-FB3-001`.

## Method

For each cell, regenerate the curve from the recorded `(p,a,b,N)` and 800 fresh
uniform targets. For 50 draws of each null family, count unordered triples
`i<=j<=k` with `P_i+P_j+P_k = R`, exactly as H016's `triples i<=j<=k`
convention. Report the mean target yield. Stored `qr`/`small_mult`/`random`
means are single H016 runs; the null mean/sd is over 50 draws, so the z values
are descriptive, not a formal test.

Command:

```sh
python3 scripts/l23_h016_negation_null.py 50 h016.json
```

## Results (50 draws, 800 fresh targets/cell)

| bits | p | B | stored qr | stored small_mult | stored random | uniform null mean (sd) | symmetric null mean (sd) |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 14 | 8287 | 20 | 0.1950 | 0.0475 | 0.1862 | 0.1845 (0.0159) | 0.1739 (0.0144) |
| 14 | 14083 | 24 | 0.1812 | 0.0963 | 0.2062 | 0.1787 (0.0141) | 0.1778 (0.0161) |
| 17 | 128629 | 51 | 0.1988 | 0.3400 | 0.1900 | 0.1873 (0.0168) | 0.1677 (0.0175) |
| 17 | 113683 | 48 | 0.1825 | 0.0000 | 0.1963 | 0.1748 (0.0126) | 0.1638 (0.0178) |

z of the stored mean against each null:

| bits | p | qr z uniform | qr z symmetric | random z uniform | random z symmetric |
|---:|---:|---:|---:|---:|---:|
| 14 | 8287 | 0.66 | 1.47 | 0.11 | 0.86 |
| 14 | 14083 | 0.18 | 0.21 | 1.96 | 1.76 |
| 17 | 128629 | 0.68 | 1.77 | 0.16 | 1.27 |
| 17 | 113683 | 0.61 | 1.05 | 1.70 | 1.82 |

`small_mult` is erratic (0 in one cell, 0.34 in another) and its z is not
informative at this sample size.

## What this establishes

- The stored H016 `random` base reproduces the uniform null (e.g. 0.1862 vs
  0.1845 at 14/8287), a sanity check on the regenerated curves/targets.
- Switching to the B-log negation-closed null shifts the null mean down by
  about 0.01–0.02 (support halving) and moves the stored `qr` z from
  0.18–0.68 to 0.21–1.77. The direction is the one the G31-001 note predicts
  (the uniform null is conservative for structured bases), but at these cells
  the shift is under 2 null-sd and does not create or rescue a signal.
- **H016's "no yield advantage" conclusion is not overturned** by the
  negation-closed null at these sizes.

## What this does not establish

- Not a formal test: stored bases are one run each; nulls are 50 draws with
  fresh targets (the original 800 targets are not stored).
- The B-log null halves x-support; an x-support-matched (2B-log) null is a
  separate object and is not measured here.
- Not a claim about `EXP-FB3-001`'s six structured geometries; this is only a
  re-score of the four prior H016 cells.
- No plaintext H016 data or vault key is committed.

## Contract note for `EXP-FB3-001`

The spec says "matched random base, same size, same curve, same targets" but
does not define size. H016's own `random_base` is B logs, so that is the
convention used here. Before the battery runs, the contract should state
whether "same size" means B logs or B distinct x; the two give different
negation-closed nulls.

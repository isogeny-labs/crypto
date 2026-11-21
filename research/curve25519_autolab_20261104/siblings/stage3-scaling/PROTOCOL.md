# SIBLING-POINTS Stage 3: how the gain and the gap to rho move with r (prime fields)

Status: frozen before any data of this stage (2026-11-11). Every number below is an expectation or
a prior measurement, labelled `[pcc:unverified]`.

## Question

Stage 2 (`../stage2-primefield/README.md`, PR #3858) measured sibling points on prime-order curves
of three prime-field shapes, at 31-bit p only [pcc:unverified]:
- sib8 / tw2cj-lazy was 0.49 to 0.52 at w = 0.
- The best sib arm was level with rho-neg in bit-ops on a = -3.
- On j = 0, b = 7 it was 1.5 times rho-cubic.

Engine commit `ce7cd4e716` lifts the r < 2^31 limit. All existing sessions replay byte-identically
with it: 6,000 runs [pcc:unverified]. Stage 3 asks whether these ratios are constant factors or
move with r, from about 2^33.5 to 2^40.

## Design

- 9 specs `spec-{a3,gen,j0b7}-{34,37,40}.json`: shape by p bits.
  - Each spec has five prime-order curves from `prime_orbit_shape`, giving r from 2^33.1 to 2^40.
  - 6 public targets per curve, 6 rounds plus 1 warmup, alternating order, L0.
  - Per-execution timeout 900 s.
- The panel runs locally on this session's host (no AWS identity).
- Arms:
  - `rho-neg`, the reference.
  - `rho-neg-aa`, the A/A control.
  - `rho-cubic`, j0b7 only, the matched rho.
  - `tw2cj-lazy`.
  - `sib8-p256` and `sib8-p512`; sib8-p64 is dropped, since Stage 2 showed it is never the best.
- Binary: `ecbench` built from `ce7cd4e716`.
- Analysis, per spec, as in Stage 2:
  - `ecbench verify --replay 8`.
  - `ecbench compare --save` at w = 0, 0.1 and 1.
  - `ecbench band --ranks`.
  - `stats_bitops.py` for bit-op ratios.
- Trend: `ecbench trend` over the saved comparisons of the three sizes of each shape fits
  ln(ratio) against ln r. Its fitted slope with interval is the scaling statement.

## Pre-registered expectations [pcc:unverified]

- **G1.** Best sib arm / tw2cj-lazy, GAE, w = 0: at most 0.56 at every size and shape.
- **G2.** Sibling points is a constant-factor mechanism. The fitted slope of ln(best sib / rho-neg)
  against ln r, GAE at w = 0, has an interval that contains 0 or lies within [-0.05, 0.05] on each
  of a3 and gen. A slope interval entirely below -0.05 would be an exponent signal; it would be
  reported as such and not claimed before replication.
- **G3.** j0b7: the best sib arm stays above rho-cubic at every weight at every size.
- **G4.** Band order against rho-neg: unresolved at every size on a3 and gen, as in Stage 2.

## Admission

Each item is checked per spec. A failure voids that spec; it is reported, not re-run under the same
name.
- Every record verified.
- `verify --replay 8` reproduces.
- The A/A ratio is 1.

## Not claimed

- No leaderboard or `benchmarks/README.md` row: that needs the validation gate.
- No security claim about P-256, secp256k1 or any standard curve.
- Two-and-a-half octaves of r cannot settle an asymptotic exponent. G2 is read only as "consistent
  or not consistent with a constant factor over this range".

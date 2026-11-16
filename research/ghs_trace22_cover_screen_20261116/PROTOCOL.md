# Frozen trace +22 degree-seven cover screen

16 November 2026 UTC. This screen follows the trace +10 null result and tests a different genus-five isogeny class with a larger prime source subgroup at `s=11`. It tests a fixed carrier and two denominator shapes, not all GHS constructions. Finite claims remain [pcc:unverified] until PCC admission.

## Fixed inputs

- `K=F3[a]/(a^5+2a+1)` and `Y/F3: y²=f(x)` with `f=x^11+x^9+x^8+2x^7+2x^6+2x^4+x³+x²+2x`. This is mask 32280 of the checked genus-five carrier census `../ghs_odd5_carrier_20261115/screen.json`, with Weil polynomial `X^10−22X^5+243`.
- `E_m/K: v²=u³+u²+b_m`, with `b_m=Σ_i floor(m/3^i) mod 3 · a^i`. The checked class census gives ten trace +22 sources of order 222, split into Frobenius orbits `68→120→74→99→111→68` and `84→144→211→96→105→84`. The two frozen representatives are masks 68 and 84.
- `degree7_quadratic_square_trace22.sage` enumerates degree-seven maps `u=A/B`, `v=yC/B²` with `B=H²` or `B=Q_i H²`, monic quadratic `H`, and `Q_i` the five quadratic factors of `f` over `K`. It checks degree, coprimality, squarefreeness, exact source order and `fC²=B(A³+A²B+b_m B³)`.

## Decision rules

Run each frozen source once, retaining JSON stdout and `/usr/bin/time -l` stderr. A hit is only a candidate map until a second implementation independently checks the identity and an evaluated corrected pullback/trace gives a nonzero prime-subgroup image. A vulnerability comparison then needs complete GHS relation, rank, target descent and verification costs against same-target Frobenius-aware rho. A null result excludes only the fixed carrier and two specified denominator shapes for these Frobenius orbits. The trace +22 Weil polynomial is distinct from the trace ±26 and trace +10 classes; construction method and global publication priority require separate audits.

Prior-work context checked before execution: [Iijima–Momose–Chao 2015](https://eprint.iacr.org/2015/805.pdf), [Joux–Vitse 2012](https://www.iacr.org/archive/eurocrypt2012/72370010/72370010.pdf), and [Diem 2006](https://arxiv.org/abs/math/0606607). Targeted exact-equation search found no published match; that is not evidence of global priority.

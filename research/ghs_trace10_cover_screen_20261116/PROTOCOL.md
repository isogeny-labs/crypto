# Frozen trace +10 degree-seven cover screen

16 November 2026 UTC. This is a targeted independent-carrier screen after the completed trace ±26 attack. It is a bounded search, not a classification of all covers or a claim of a new vulnerable family. Finite claims remain [pcc:unverified] until PCC admission.

## Inputs fixed before execution

- `K=F3[a]/(a^5+2a+1)` and `Y/F3: y²=f(x)` with `f=x^11+x^7+2x^5+2x^4+x^3+x^2+1`, the checked trace +10 genus-five representative in `../ghs_odd5_carrier_20261115/representative.json`.
- `E_m/K: v²=u³+u²+b_m`, where `b_m=Σ_i floor(m/3^i) mod 3 · a^i`. The exact class census in `../ghs_odd5_carrier_20261115/class_count.json` has ten trace +10 models of order 234. They split into Frobenius orbits `12→42→227→57→115→12` and `151→185→176→170→209→151`. This screen uses orbit representatives `m=12,151`.
- The adapted source `degree7_quadratic_square_trace10.sage` enumerates degree-seven maps `u=A/B`, `v=yC/B²` with `B=H²` or `B=Q_i H²`, monic quadratic `H`, and the five degree-two branch factors `Q_i` of `f` over `K`. It checks `f C² = B(A³+A²B+b_m B³)`, degree, coprimality, squarefreeness and the exact source order.

## Measurement and interpretation

Run each frozen source once and save its complete JSON stdout and `/usr/bin/time -l` stderr. Retain all hits and null results. A hit must be independently replayed in a second algebra system or a separate implementation, mapped on a nonzero source prime subgroup, and benchmarked end to end against same-target Frobenius-aware rho before it can be called a vulnerable exact curve. The screen is complete only for its two representatives, fixed carrier, and specified denominator shapes; zero hits would not exclude other carriers, degrees or denominator shapes. A trace +10 hit would be distinct from the trace ±26 twist orbit by its Weil polynomial, while its degree-seven cover method would still require a prior-work audit before any family or method novelty claim.

Primary context: Iijima–Momose–Chao 2015, Section 5 (https://eprint.iacr.org/2015/805.pdf); Joux–Vitse 2012 (https://www.iacr.org/archive/eurocrypt2012/72370010/72370010.pdf); Diem 2006 (https://arxiv.org/abs/math/0606607). These works establish nearby construction and attack mechanisms; an exact-equation search cannot establish global priority.

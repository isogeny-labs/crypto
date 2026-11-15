# Protocol: coordinate frames for the Curve25519-family algebraic factor base (frozen 2026-11-03)

Frozen in git before any confirmation count. The pilot (below) was run first and
is exploratory; it chose the hypotheses and is not evidence for them.

## Question

The owner asked whether a coordinate system other than the one in use
(other Montgomery forms, projective, Edwards, and so on) yields more relations
for Curve25519 index calculus.

## What is already settled (not re-tested here)

- **Generic track (the baseline, `prime-projected-abscissa` + `mitm`).** The
  base is projected by the cofactor once and targets are subgroup points, so
  the coordinate that defined the base drops out of per-target work. For any
  base of N columns, P(target = ±P_i ± P_j) ≤ #signed pairs / r. Exact support
  for Weierstrass, Montgomery, Edwards and the PKM tower already sits at that
  ceiling (`research/curve25519_algebraic_ladder_20261102`, Q-A). There is no
  coordinate lever on this track.
- **Models that do not exist here.** Hessian and twisted Hessian need a rational
  3-torsion point. Jacobi intersection and Huff need Z/2 × Z/2 ⊂ E(F_p). The
  family has #E = 8r with cyclic Z/8, so none of them is defined over F_p.
  Jacobi quartic exists (rational 2-torsion), and its coordinate is x ∘ τ_T up
  to Möbius.
- Every degree-2 function on E is a Möbius frame of x composed with a
  translation (`src/cryptanalysis/coordinate_search.rs`). A translation by a
  point Q maps the base bijectively and shifts every log by log Q, so it leaves
  the point count and coverage unchanged. The free parameter is the Möbius
  frame of x.

## Hypothesis (from the pilot)

On the **algebraic track** (`pkm-semaev`, base = {P : x(P) ∈ frame}, no
projection), the F4 cost per target depends on t (degree and polynomial
size), not on which points the frame holds. Exact coverage depends on:
- how many points the frame holds (cols);
- how their cofactor cosets pair up.

Both fluctuate between frames, and Weierstrass `bits lo=0` and `edwards-bits`
are just two draws. A frame chosen per (curve, t) without discrete logs should
therefore raise coverage, and so lower F4 cost per relation, at an unchanged F4
cost per target.

## Pilot (exploratory, k = 20, seed 11, K = 2000 per family)

`pilot/pilot_k20.json` (sha256 `ceef2188…7094`). It calibrates exactly against
the engine's `coverage_exact`: bits t = 4, 5, 6 give 12, 42, 162 covered, and
edwards t = 4, 5, 6 give 10, 58, 206 (`curve25519_edwards_semaev_20261102/cells`).
Best frame ÷ bits lo=0: 4.7× (t = 5), 3.9× (6), 2.2× (7), 1.9× (8).

## Confirmation design

- **Curves.** Curve25519-family miniatures by the family rule
  (`curve25519_family family`), held out: k = 16, 18, 22, 24. k = 20 is re-run
  as a replication with a fresh seed, but it is not held out.
- **Frames.** On every curve and t ∈ {4, 5, 6, 7, 8}:
  - fixed: `bits_lo0`, `edwards_bits`;
  - K = 2000 random translation frames (1, L, 0, 1);
  - K = 2000 random Möbius frames.
  - Frame seed: `20261103 + k`, never the pilot's.
- **Instrument.** `scripts/frame_census.py`: exact full-group logs in Z/8r
  (cyclic), exact distinct covered subgroup points at m = 2, cols, and the
  log-free selector J (signed pairs whose cosets cancel). No timing.
- **Selection.** The frame used for each (curve, t) is argmax J over the
  translation frames, ties broken by cols, then by the lowest L. Translation
  frames are chosen because `bits@L` runs them in the engine unchanged.
- **Search cost** (priced, reported, not hidden). Per frame: 2^t Legendre
  symbols, plus [r]P for every point found (≈ 1.5·log2 r group operations).
  The total is K times that.

## Predictions (each decided by exact counts)

- **P1, no model effect.** `bits_lo0` and `edwards_bits` lie inside the central
  98% of the random translation-frame coverage distribution in at least 45 of
  the 50 (curve, t, frame) cells.
- **P2, translation ≡ Möbius.** Mean coverage translation ÷ Möbius is in
  [0.93, 1.07] in every (curve, t) cell with mean covered ≥ 50.
- **P3, frame gain.** The selected frame's covered ÷ `bits_lo0` covered is
  ≥ 1.5 at t ∈ {5, 6, 7} on every held-out curve. The ratios at t = 4 and 8
  are reported, with no threshold.
- **P4, log-free selection suffices.** The selected (argmax J) frame's coverage
  is ≥ 0.9 × the oracle-best (argmax covered) translation frame's, in every
  cell.
- **P5, the engine agrees (k = 20, t ∈ {5, 6}, G = 3, 100 targets, frame
  selected in the k = 20 replication).** For `curve25519_family f4cost 20
  bits@L* t 3 …` against `bits t 3 …` with the same seed:
  - `coverage_exact` equals the census exactly;
  - `solving_degree_max` and `poly_terms_per_specialisation` are equal;
  - mul-adds/target are within ±5%.

  So F4 mul-adds per relation (at exact coverage) fall by the coverage ratio.

## Failure handling

A failed prediction is reported as failed and not retuned. If P5 fails on
polynomial size, the gain is not a free gain and the result is reported as a
trade-off.

## Claim scope

- **Algebraic track only.** That track is far behind the generic baseline,
  2.6× at w = 0 and 1.4·10⁵× at w = 0.1 (`curve25519_algebraic_ecbench_20261102`).
  A coverage gain of a few × does not change the order between them across
  the §5a band, and rho remains ahead. The distance remaining is reported,
  not a pass/fail.
- This is not a leaderboard table A–C row. At most it is a component result
  of the algebraic track.
- The miniatures are k ≤ 24. The relative gain shrinks as t grows: extreme-value
  fluctuations scale as 2^{t/2} against a mean of 2^{t-1}. No 255-bit
  extrapolation is claimed.

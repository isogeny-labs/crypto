# L15: the greedy digit-set frames in the F4 engine (k = 20, 2026-11-04)

Protocol: [PROTOCOL.md](PROTOCOL.md), frozen at `808527da5c` before data.
Engine: `curve25519_family` built at 07:36Z from the working tree later committed as
`b847203692` (`Membership::Digits`), hash in `data/binary.sha256`; byte identity with a clean
build of that commit is not established (validator E1). Seven cells, 100
targets each, ran in 40 s; no cell capped, `undecided` = 0 everywhere.

## Results (`data/*.json`)

| cell | form | t | G | cols | covered | degree | terms | mul-adds / target | F4 mul-adds / relation |
|:--|:--|--:|--:|--:|--:|--:|--:|--:|--:|
| ctl_bits_lo0 | `bits` | 5 | 3 | 11 | 42 | 5 | 64 | 3,036,911.76 | 9.487e9 |
| trans | `bits@815271` | 5 | 3 | 24 | 248 | 5 | 64 | 3,014,275.30 | 1.595e9 |
| digits | greedy set, lo 113971 | 5 | 3 | 22 | 522 | 5 | 64 | 3,036,904.34 | **7.633e8** |
| trans | `bits@815271` | 5 | 4 | 24 | 248 | 5 | 32 | 1,364,482.74 | 7.219e8 |
| digits | greedy set, lo 113971 | 5 | 4 | 22 | 522 | 5 | 32 | 1,375,663.15 | **3.458e8** |
| trans | `bits@889850` | 6 | 5 | 48 | 570 | 5 | 44 | 18,211,284.35 | 4.192e9 |
| digits | greedy set, lo 51134 | 6 | 5 | 43 | 1,268 | 5 | 44 | 17,727,535.79 | **1.834e9** |

Per relation = mul-adds per target ÷ `coverage_exact` (derived). Hits:
trans t = 5 cells 1/1 verified each, digits t = 6 3/3 verified, others 0.

## Predictions

| | rule | outcome |
|:--|:--|:--|
| D1 | columns / covered equal the census in all four bases | **held**: 22/522, 43/1,268, 24/248, 48/570 |
| D2 | same degree and terms, undecided 0, capped false, 0.95 ≤ mpt ratio ≤ 1.05 at (5,3), (5,4), (6,5) | **held**: 5/5 and 64/64, 32/32, 44/44; ratios 1.0075, 1.0082, 0.9734 |
| D3 | per-relation digits ÷ trans ≤ 0.6 at (5,3) and (6,5) | **held**: 0.479 (point 0.475) and 0.438 (point 0.450) |
| D4 | E(digits, 5, 4) ÷ E(winner) ≥ 2 | **held**: 22 × 1,375,663 / 522 = 57,978 against 5,533, ratio **10.5** |
| D5 | control reproduces 3,036,911.76, 11 columns, 42 covered | **held**: exact |

## Reading

- The digit-set membership is free in the engine: the system is the
  `bits` system with different linear coefficients, and the per-target
  cost moves by less than the hit-rate noise.
- The greedy coset-aligned digit set therefore turns its census coverage
  (2.1–2.2× the best translation frame) into 2.1–2.3× less F4 work per
  relation on the bits arm. Per relation here is derived (mul-adds per
  target ÷ exact coverage); measured relations are 0–3 per 100 targets.
- Against the `lo = 0` control at matched (t, G) = (5, 3) the digit set is
  12.4× lower per relation (9.487e9 → 7.633e8, = 522/42) but **6.2× lower
  per solve** (columns × mpt ÷ covered; the digit set has 22 columns, the
  control 11). The control was run at G = 3 only. `bits@815271` is 5.95×
  below `lo = 0` per relation; this does not chain with the frame-search
  figures for `bits@976503`.
- It does not overtake the isogeny tower: the bits form's per-target F4
  cost at (5, 4) is 1,375,663 against the tower's 52,816, and the digit
  set models 10.5× behind the T12 winner at matched (t, G).

## Costs and scope

- Greedy search (L14, rep 0): 62,000 Legendre symbols + 31,034 [r]P
  labels at t = 5; 126,000 + 62,902 at t = 6. Per-curve precomputation.
- Engine-only counted mul-adds. No end-to-end S: ecbench has no digit-set
  factor base (`prime-interval` covers intervals only); adding one is the
  natural next step if the bits arm is ever worth pricing end to end.
- Rho remains ahead; L10 fixes the exponent at 1.03 for this track.

## Deviations

None from the frozen cells. Conductor and the AWS CLI are absent (local
run, no S3 checkpoint); no graph or PDF yet.

## Validation (2026-11-04)

`validator-report.json`: **pass_with_errata**, disposition validated. D1–D5 recompute as held with
independent code (Weierstrass and Montgomery group laws, #E = 8r); all four natural relations
re-verified (target scalars 69608, 108445, 71310, 81978); planted replay 36/36, 36/36, 20/20.
Errata applied: E1 (binary provenance, above), E2 (57,978), E3 (the "12×" is per relation at
(5, 3); 6.2× per solve), E4 (per-relation values are derived). E5, unpriced native counters per
target: digits (5, 4) 895, T12 winner 224, digits t = 6 2,472, trans t = 6 2,395; weighting them
at 0, 0.1 and 1 GAE gives a D4 ratio of 10.44–10.48, so no ranking changes. E6: no rustc or host
record; the relations are in the validator report. Distance to matched rho is not restated here:
this is an engine-only modelled cost, not in the units of T12's end-to-end gap. Greedy replicate 0
is mid-range among five (covered 392–582 at t = 5, 1,142–1,364 at t = 6).

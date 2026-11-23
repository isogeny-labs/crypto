# Preregistered prime-window census of rational-lift xedni

The frozen five-window census found two coefficient-box coverage gaps. At
`p=1531`, the reduced group has prime order `q=1553`, while the radius-41 box
contains 6,888 nonzero coefficient pairs. Their reductions attain exactly 984
of the 1,552 nonidentity points. The first finite collision gives
`R̄=11Q̄`; hence every box candidate has scalar `a+11b` in `[-492,492]`
before reduction modulo 1553. Residue 500 is unreachable. The exact modular
certificate and a checked Lean theorem establish this gap. A second cell,
`p=2287`, attained 2,058 of 2,250 nonidentity points by its frozen cap.
Thus coefficient-pair cardinality exceeding group order does not ensure
coverage for this rational-lift construction and cap.

## Frozen selection and result

The [protocol](protocol.json) was committed as `c1faf021320` before the run.
It selects the first eligible prime in each of five consecutive 250-integer
windows, testing ascending rational primes for good reduction, invertible
denominators of `R`, prime curve order, and nonidentity `Q̄`. The
[selection trace](selection-rows.jsonl) retains all 59 eligibility decisions;
no collision or coverage measurement enters selection. The curve is
`[94,0,188,1738,3805]` over `ℚ`, with `Q=(-4,19)` and
`R=(52242961/137641,-1907191742616/51064811)`. Each selected cell uses
`B=⌊√q⌋+2` and ascending 128-bit height-pairing score, then `(a,b)`.

| p | q | B | Nonzero pairs in box | First relation index | Coverage index or cap | Nonidentity points attained | Peak rational coordinate bits | Status |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1531 | 1553 | 41 | 6,888 | 35 | cap 6,888 | 984 / 1,552 | 68,693 | `coefficient_cap_exhausted` |
| 1889 | 1951 | 46 | 8,648 | 399 | 4,736 | 1,950 / 1,950 | 24,472 | `full_coverage` |
| 2039 | 2069 | 47 | 9,024 | 1,463 | 2,842 | 2,068 / 2,068 | 9,202 | `full_coverage` |
| 2287 | 2251 | 49 | 9,800 | 1,505 | cap 9,800 | 2,058 / 2,250 | 98,113 | `coefficient_cap_exhausted` |
| 2699 | 2633 | 53 | 11,448 | 2,255 | 3,040 | 2,632 / 2,632 | 8,835 | `full_coverage` |

Indices count candidate evaluations in the frozen order. A capped row has
unknown first full-coverage index beyond its cap. Coordinate peaks include
exact rational numerator and denominator sizes of constructed points. Exact
[prime rows](prime-rows.jsonl), [window rows](window-rows.jsonl), and
[result digest](result.json) preserve the complete result, including the
two capped cells. This is an ordinary-host structural experiment; its counts
and coordinate sizes are diagnostics, not calibrated wall-time comparisons.

## Proof map and independent checks

* [MissingResidue.lean](formal/MissingResidue.lean) proves
  `missing_residue_500`: for all integers `a,b` in `[-41,41]`,
  `(a+11*b)%1553 ≠ 500`. It also proves `image_in_two_intervals`, which
  confines all attained residues to `[0,492] ∪ [1061,1552]`. Both declarations
  built with the [pinned toolchain and dependency manifest](formal/build.sh).
  `#print axioms` reported only `propext` and `Quot.sound` for each. The
  elliptic-curve reduction and `R̄=11Q̄` bridge are independently checked
  by the Sage run and the exact rational audit, rather than formalized in Lean.
  `lean4checker --fresh` was unavailable on this host.
* [gap_certificate.py](gap_certificate.py) independently enumerates the
  83×83 scalar box. Its [certificate](gap-certificate.json) gives exactly
  985 attained residues including zero, 568 missing residues, and a SHA-256
  digest of the sorted attained list. This confirms the stronger equality
  between the image and `[-492,492] mod 1553`.
* [audit.py](audit.py) recomputes all 59 prime-selection decisions by an
  integer Legendre count and reconstructs 9,980 rational points with
  `fractions.Fraction`. It checks candidate reductions, group sums, first
  relations, coverage or cap, statuses, coordinate peaks, and raw-file
  digests. [Audit result](audit-result.json): `PASS`.

The exact radius threshold at `p=1531` is the next useful check: since
`a+11b` fills the integer interval `[-12B,12B]`, full residue coverage
requires `12B≥776`, first true at `B=65`. A frozen extension to that radius
can measure its exact rational-coordinate cost. The `p=2287` gap merits an
analogous scalar-image analysis at fixed and expanded radii.

## Replay

From `/Volumes/SSD990-2/crypto-worktrees/exotic-ecdlp-recovery-20261119`:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_preregistered_prime_panel_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_preregistered_prime_panel_20261120/run_sage.py > research/exotic_ecdlp_20261021/xedni_preregistered_prime_panel_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_preregistered_prime_panel_20261120/run.stderr
python3 research/exotic_ecdlp_20261021/xedni_preregistered_prime_panel_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_preregistered_prime_panel_20261120/audit-result.json
python3 research/exotic_ecdlp_20261021/xedni_preregistered_prime_panel_20261120/gap_certificate.py > research/exotic_ecdlp_20261021/xedni_preregistered_prime_panel_20261120/gap-certificate.json
sh research/exotic_ecdlp_20261021/xedni_preregistered_prime_panel_20261120/formal/build.sh
```

The saved [Sage runtime receipt](sage-runtime-info.json) identifies the
accepted local build. The formal build command and `#print axioms` output are
retained in [build-output.txt](formal/build-output.txt).

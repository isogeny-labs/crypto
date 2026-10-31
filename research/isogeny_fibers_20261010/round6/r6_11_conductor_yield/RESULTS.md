# R6.11: relation yield across a prime conductor gap (toy test of Conjecture 10′)

**Question.** At matched base size and decomposition oracle, is the
index-calculus relation yield (class factor c) the same on the crater and
on the floor of an ℓ-volcano, ℓ ∈ {5, 7, 11, 13}, over toy p ∈ [2^20, 2^22)?

**Hypothesis (PROGRAM.md R6.11).** Yes: c depends on (p, t) only.
**Falsifier (as implemented in `summarise`).** At some (ℓ, m, |F| ≤ 100, smallx rule),
at least two independent instances have |c_crater/c_floor₀ − 1| > 2·sd (sd =
across-instance spread of that ratio) **and** |z| > 3, where z compares the crater
c with the spread over all same-class floor curves of that instance.

**Verdict: hypothesis confirmed at toy size (falsifier did not fire).** 24
instances (6 per ℓ), 4 base sizes, m = 2 and 3: every mean crater/floor ratio is
within 0.5% of 1 and every mean ± 2 sd interval contains 1 (measured).

## Construction (measured, all certified)
- Class-number-one D ∈ {−11, −19, −43, −67, −163} with ℓ ∤ D; p = (t² + |D|v²)/4
  with ℓ ∥ v, t ≢ 2 (mod ℓ), N = p + 1 − t prime. Crater j₀ = j(D) (End = O_K);
  floor curves = the other F_p-roots of Φ_ℓ(j₀, Y) (End = Z + ℓO_K).
- #E = N on both levels, certified by [N]P = O; E(F_p) ≅ Z/N, h = 1, |Aut| = 2
  on every curve (j ∉ {0, 1728}): O^× and E(F_p)[h] agree, so only the conductor differs.
- Level evidence: crater Φ_ℓ root count equals ℓ − (D/ℓ) + [(D/ℓ) = 1] (`crater_phi_roots_expected`, matched in every instance), every floor curve has exactly 1
  root; the number of floor curves equals the
  expected count in every instance. Every crater→floor edge is certified by
  `kernel::elkies_isogeny` (division-polynomial check) and a Vélu codomain
  F_p-isomorphic to the floor model (`all_vertical_edges_certified: true`).
- j-invariants, p, t, v, N, Φ_ℓ root counts per curve: `raw/r6_11_ell*_i*.json`,
  pooled in `raw/summary.json` → `level_evidence`.

## Census
Exact §15.2 census per curve: every m-multiset (m = 2, 3) of a signed base F
(rule `smallx`: x = 0, 1, 2, … with x³ + ax + b a nonzero square; rule
`randx`: seeded random order of x) is summed; D_m distinct targets hit,
c = D_m · m!/|F|^m, D_m/U per unknown (U = |F|/2), birthday reference c* (derived).
|F| ∈ {60, 80, 100} (planned) and 200 (supplementary).

### Crater / floor₀ class-factor ratio, smallx, mean [mean ± 2 sd] over 6 instances (measured)

| ℓ | |F| | m = 2 | m = 3 |
|--:|--:|:--|:--|
| 5 | 60 | 1.0006 [0.9940, 1.0072] | 1.0023 [0.9833, 1.0213] |
| 5 | 100 | 0.9998 [0.9951, 1.0045] | 0.9974 [0.9835, 1.0113] |
| 5 | 200 | 0.9997 [0.9947, 1.0046] | 0.9982 [0.9873, 1.0091] |
| 7 | 60 | 1.0000 [1.0000, 1.0000] | 1.0008 [0.9969, 1.0048] |
| 7 | 100 | 0.9991 [0.9948, 1.0033] | 0.9980 [0.9826, 1.0133] |
| 7 | 200 | 0.9999 [0.9973, 1.0024] | 0.9986 [0.9902, 1.0070] |
| 11 | 60 | 1.0011 [0.9977, 1.0046] | 1.0012 [0.9902, 1.0121] |
| 11 | 100 | 1.0023 [0.9979, 1.0068] | 1.0049 [0.9953, 1.0145] |
| 11 | 200 | 1.0010 [0.9963, 1.0056] | 1.0014 [0.9933, 1.0094] |
| 13 | 60 | 1.0011 [0.9977, 1.0046] | 1.0038 [0.9933, 1.0143] |
| 13 | 100 | 0.9993 [0.9939, 1.0047] | 0.9986 [0.9826, 1.0147] |
| 13 | 200 | 1.0000 [0.9976, 1.0025] | 0.9994 [0.9950, 1.0039] |

|F| = 80 and the randx rule: same picture (all intervals contain 1), in `raw/summary.json`.
Mean c at |F| = 100, smallx, m = 3: crater 0.956–0.960, floor₀ 0.954–0.962 (measured).

Figures: `fig1_ratio_vs_ell.svg` (ratio vs ℓ with intervals), `fig2_c_over_birthday.svg`
(c/c* for every curve; crater and floor interleave). Report: `report.pdf`.

## Commands
```
export CARGO_TARGET_DIR=<shared r6target> CARGO_BUILD_JOBS=1
cargo build --release --example r6_11_conductor_yield
for l in 5 7 11 13; do r6_11_conductor_yield run --ell $l --instances 6 --seed 1 --out raw; done
r6_11_conductor_yield summarise --dir raw --out raw/summary.json
python plot.py raw   # plots recorded values only
```
Runtime ~12 s per instance (contended wall time).

## Limits
- Weak regime at m = 2: |F|²/2 ≪ N ≈ 2^21, so c ≈ 1 is forced by the birthday
  bound on any curve; m = 2 can only detect gross differences. m = 3 (|F|³/6 up to
  1.3·10⁶ ≈ N/2 at |F| = 200) is where collisions are non-trivial and is the informative arm.
- The census measures a plain x-frame base; it says nothing about algorithms that
  use End(E) explicitly (e.g. a decomposition oracle exploiting an endomorphism),
  which is exactly the loophole Conjecture 10′ leaves open.
- Toy p only, h(D) = 1 craters only (single-vertex crater), one vertical step
  (v_ℓ(f) = 1). No sub-rho claim; no deployed-curve claim.

## Next step
Move "vertical moves across a large-prime gap" in §15.4 to: measured, no constant
change (ratio 1 within ±0.5% mean, ±1.5% at 2 sd). Extending the test needs a
decomposition oracle that uses End(E) (a conductor-sensitive algorithm), or
deeper volcanoes (v_ℓ(f) ≥ 2) with h(D) > 1.

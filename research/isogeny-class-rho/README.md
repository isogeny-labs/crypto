# ECDLP cost across whole isogeny classes: prime, binary and Koblitz curves

**Question.** Take one ECDLP instance `Q = kP`, carry it by explicit,
verified isogenies to many curves of its isogeny class — spread over the
class's endomorphism-ring strata, reached through small and through large
("hard to find") isogeny degrees — and run the same Pollard rho on every
curve. Does the curve, its stratum, its level in any ℓ-volcano, the degree or
kernel field of the isogeny that reached it, the generator of `⟨P⟩`, or the
curve the instance was drawn on change how fast rho solves it? Report min,
max, mean and the rest.

**Protocol.** [`PROTOCOL.md`](PROTOCOL.md), frozen before the run (sha256
`81c3232bdab481244324101d10e2d50199da1990ceba10c8dc66cc206286409e`); the
`controlled-experiment` skill's classes, controls, decision rules and power
check are in it. Run: [`runs/run-20261018T0857Z/`](runs/run-20261018T0857Z/)
(command card [`COMMANDS.md`](runs/run-20261018T0857Z/COMMANDS.md), every
record and log). Tool: `isoclass-rho` (`src/bin/isoclass_rho/`).

<!-- HEADLINE -->

## Method

`isoclass-rho` (`src/bin/isoclass_rho/`, native Rust; every statistic in
this report is computed there, `plot.py` and `make_report.py` only lay out
recorded values):

1. **Class arithmetic (exact).** `#E` by baby-step giant-step on random
   points, checked on eight more; `t`, `D = t² − 4q = f²·D_K`; `h(D_K)` by
   counting reduced primitive forms; `h(d²D_K)` for every `d | f` by the
   conductor formula, cross-checked by counting `h(f²D_K)` directly; the
   class size `Σ_{d|f} h(d²D_K)`.
2. **Walk.** Breadth-first over the active primes from the root, keyed by
   `j`. A prime is active when it has a rational isogeny: `ℓ | f`, or `ℓ ∤ f`
   with `(D_K/ℓ) = 1`; inert primes are recorded as inactive (for odd `m`
   over `F_{2^m}`, every prime of `f` is `≡ ±1 (mod 8)`, which is why the
   binary and Koblitz classes have no 3- or 5-isogenies). Three ways to find
   an `ℓ`-kernel, all ending in the same monic kernel polynomial `h` of
   degree `(ℓ−1)/2`:
   - *Elkies route* (`F_p`, `j ∉ {0, 1728}`): the roots of `Φ_ℓ(j, Y)` are
     the neighbours; the library's Elkies construction gives `h`
     (`crypto_lib::cryptanalysis::isogeny_walk`). `Φ_ℓ` is built natively
     from the `q`-expansion of `j` (ℓ = 61: 15 s).
   - *Division route* (any field, any `j`): factor the ℓ-division polynomial
     over `F_q` into the irreducible factors of degree `d | (ℓ−1)/2`; a root
     `x(T)` lives in `F_{q^d}`; the x-only ladder gives `x(iT)` there, and
     `⟨T⟩` is rational exactly when `∏(x − x(iT))` has `F_q` coefficients.
     The kernel field degree `d` is the "hardness" of the kernel: the
     smallest extension its abscissae generate.
   - *Torsion route* (large `ℓ | f` off the floor, where the Frobenius is a
     scalar `λ` on `E[ℓ]`, so every subgroup is rational): a random abscissa
     in `F_{q^d}`, `d = ord(±λ)`, is a point of `E` or of its twist over
     `F_{q^d}`; the Montgomery x-ladder by the matching cofactor gives an
     ℓ-torsion abscissa, its multiples the kernel, grouped into Frobenius
     orbits `{x(±iλ^jT)}` whose polynomials are over `F_q`. This finds a
     409-isogeny of the Koblitz curve over `F_{2^41}` in `F_{2^{41·204}}` in
     about a minute, where the 409-division polynomial (degree 83,640) is out
     of reach; kernels are sampled (three per vertex), not enumerated, and
     the dual of a sampled edge is found by pushing a second torsion
     abscissa through it.
   - The map on points is Kohel's normalised formula from `h` over `F_p`
     (`X = ℓx − 2σ − 2(3x²+a)·h'/h − 4(x³+ax+b)·(h'/h)'`, `Y = y·X'`) and,
     over `F_{2^m}` with `y² + xy = x³ + ax² + b`, `X = x + xL + x²L²`,
     `Y = y + x³L₃ + xL + (x + y + x²)(xL² + L)` with `L = h'/h`,
     `L₃ = L³ + L·D₂h/h + D₃h/h` (Hasse derivatives), codomain `(a, b + σ +
     σ²)`, `σ = Σ_{Q∈S} x_Q` — derived from Vélu's general formulas and
     checked on points.
3. **Every edge verified on points**: images on the codomain, `φ(P+Q) =
   φ(P) + φ(Q)`, `n·φ(P) = O ≠ φ(P)`; `#E` re-counted on every measured
   curve; every rho solve must return the planted `k` (it did).
4. **Levels and strata.** A curve's level in the ℓ-volcano is found from
   the number of rational ℓ-kernels (one at the floor) and non-backtracking
   walks from three neighbours to the floor; its endomorphism-ring
   conductor relative to the walked primes is `∏ ℓ^{level_ℓ}`, which names
   its stratum, and the stratum's class number says how many curves share
   that endomorphism ring.
5. **Instances.** `P` of order `n` and `Q = kP` on the root, pushed down the
   tree. Shifted instances: `(P_s, kP_s)` drawn on source curves and carried
   up to the root through dual edges (any rational isogeny back, composed
   with the model isomorphism; sixth roots at `j = 0`) and down to every
   curve. Generators: `P_i = r_iP`, `Q_i = r_iQ`, the same `k`.
6. **Rho.** Single walker, 32-adding (Teske), distinguished points `θ =
   2⁻⁸`, no negation map, restart after `40·2⁸` DP-free steps (the same walk
   as `volcano-rho`); trial `s` has the same seed on every arm; all jobs
   shuffled over 4 threads. Folds (positive controls): the walk steps from
   the canonical point of an orbit under negation, the order-6 automorphism
   group of `j = 0`, or the Frobenius `τ` of a Koblitz curve (with and
   without negation), with the scalar tracked, look-ahead against
   2-cycles and deterministic escapes from longer fruitless cycles.
7. **Controls.** Isomorphic models of the root (`F_p`: `(u⁴a, u⁶b)`;
   `F_{2^m}`: the same curve in another polynomial basis of the field, since
   `y ↦ y + sx` leaves every abscissa, and so the walk, unchanged); an A/A
   replica of the root under an independent seed set; planted effects of
   known size (negation fold `1/√2`; 4-adding walk `1.137`); the structural
   folds where the curve admits them.

<!-- TABLES -->

<!-- READING -->

## Limits

- One root per class; the measured curves are a stratified sample (eight
  per stratum where the stratum had that many within the walk radius, plus
  the curves reached through the largest degrees), not the whole class.
  Class sizes are exact; coverage is what the tables say.
- Operations are the unit. Wall times (ms, ns/op, transport, end-to-end)
  are from a shared 4-vCPU VM with other jobs running and are indicative;
  no isolation receipt (`isolated_bench`) was taken.
- Levels of the Koblitz class's sampled primes are assumed descending off
  the crater, exact there because `h(−7) = 1`; the ℓ = 193 prime of P0's
  conductor was not walked (`Φ_193` is beyond this session).
- Sampled kernels (torsion route) are three of `ℓ + 1`; the Koblitz class
  walk is a sample of a sample.
- This is rho against rho, one instrument. It is not an index-calculus
  result, has no §5a band, and says nothing about curves outside these
  four classes. A fold that needs the crater's structure is not an
  "isogeny speedup": it is the crater curve's own endomorphism, which the
  isogeny takes away.

## Reproduce

```
cargo build --release --bin isoclass-rho
target/release/isoclass-rho selftest
# then the commands of runs/run-20261018T0857Z/COMMANDS.md, in order;
# python plot.py runs/run-20261018T0857Z/<class> and
# python make_report.py runs/run-20261018T0857Z draw and tabulate the records.
```

Replay: each class mode re-run with `--trials 5` reproduces the recorded
iteration counts of the first five seeds (`<class>/replay/`, compared in
the run README).

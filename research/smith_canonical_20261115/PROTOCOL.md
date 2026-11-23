# Protocol: the canonical map psi: X -> C of Smith's trigonal construction

Study `smith_canonical_20261115`. Module `src/cryptanalysis/smith_canonical.rs`.
Written before any random curve was run through the instrument. Only Smith's
Example 1 (F_37, the paper's printed quartic and zeta numerator) was used
during development, as the positive control.

## Claim under test

C1. For a Smith-positive genus-3 hyperelliptic curve `H: y^2 = F(x)` over
`F_p` with trigonal map `g = N/D` in normal form, the map

    psi(t; b) = T(t) (b0 : b1 : b2),   T_ik = [x^2]( x^(i+k) D adj(F mod G) mod G )

is the canonical map of Recillas' curve `X` onto a smooth plane quartic `C`
(derivation: module doc; the three differentials are the traces along the
correspondence `R` of `x^i dx / y`).

C2. `Psi([sum m_i P_i]) = sum m_i (psi(Q1(P_i)) + psi(Q2(P_i)))` is a
well-defined homomorphism `J_H(F_p) -> Pic^0(C)(F_p)` with kernel
`S(F_p)` (Smith's tractable subgroup), so a DLP in `J_H` transfers to `C`.

This is a correctness study of a construction (exact checks), not a
statistical comparison. There is no effect size; each check is pass/fail
per curve.

## Population

Smith-positive curves drawn by `tests::random_positive(p, count, seed, degF)`:
`F` with uniform coefficients, nonzero leading coefficient, squarefree,
taking the first rational tractable subgroup that passes Propositions 2 and 4
without twist and its first valid normal-form trigonal map with square
`lc(s)` (same sampler as `smith_trigonal`'s tests). No curve is dropped or
replaced after the fact. A curve on which the canonical map cannot be built
is a reported row (`FAIL`), not an exclusion.

| cell | p   | deg F | curves | C counted over      | seed        |
|------|-----|-------|--------|---------------------|-------------|
| A    | 37  | 7     | 20     | F_p, F_p^2, F_p^3   | 0x5c370001  |
| B    | 101 | 7     | 20     | F_p, F_p^2, F_p^3   | 0x5c370002  |
| C    | 211 | 7     | 10     | F_p, F_p^2          | 0x5c370003  |
| D    | 37  | 8     | 10     | F_p, F_p^2, F_p^3   | 0x5c370004  |
| E    | 101 | 8     | 10     | F_p, F_p^2, F_p^3   | 0x5c370005  |

Command per cell (`<p>:<degF>:<count>:<kmax>:<seed>`):

    SMITH_CANONICAL_SWEEP=37:7:20:3:0x5c370001 cargo test --release --lib \
      smith_canonical::tests::sweep -- --ignored --nocapture

`H` is always counted over `F_p, F_p^2, F_p^3` (this gives `#J_H(F_p)`).
In cell C, `#C(F_p^3)` is not counted (cost); there the third zeta
coefficient is covered only indirectly, by `#J_H . Psi(D) = 0`.

## Checks per curve, with predictions

Per curve the instrument (`verify_curve`, 10 relation pairs, 10 random
classes, planted `k = 22359 mod ord`) records:

1. `fit`: at least 17 distinct image points; they satisfy no linear form
   (the three differentials are independent, dim = 3), no conic (X not
   hyperelliptic), no cubic, and exactly one quartic. Prediction: holds on
   every curve. A conic is reported as "X hyperelliptic for this input".
2. `smooth`: `plane_quartic_pic::smoothness_certificate`. Prediction: true.
3. `N_C = N_H`: `#C(F_p^k) = #H(F_p^k)` for the k of the cell. Prediction:
   equal on every curve.
4. `on_C`: every `psi(Q)` for `Q` in `image_points(P)`, all `P` of `H` over
   `U`, lies on `C`. (Not independent of the fit: the same points define
   `C`. Kept as a consistency check; the independent evidence is 3, 5-9.)
5. `t_rho`: `psi` from `(t, rho)` alone (lifting through `c0, c1, c2`)
   equals `psi` from the `XPoint`. The plane model `Phi(t, rho) = 0` is only
   birational to `X`: at a node two points of `X` share `(t, rho)` (seen on
   Example 1, 2 of 60 image points). There the check is that the `XPoint`
   image is one of the two lifts; such points are counted in `nodes`.
   Prediction: all pass.
6. `relations`: for two functions `y - v(x)` whose zeros are rational
   points over `U`, `Psi(zeros_1 - zeros_2) = 0`. Prediction: all zero.
   `relations_neg` (detector control): the same with one point replaced by
   another point; prediction: all NONZERO.
7. deg F = 7 only (needs `J_H` arithmetic):
   `repr`: two independent rational-point representations of the same random
   class give the same image; `add`: `Psi(D1 + D2) = Psi(D1) + Psi(D2)` with
   the three sides moved independently; `annih`: `#J_H(F_p) Psi(D) = 0`;
   `ord_ratio`: `ord Psi(D)` divides `ord D` with quotient in {1, 2, 4, 8}.
   Prediction: all pass.
8. `2tors` (deg F = 7): for every rational 2-torsion class `(u, 0)`,
   `Psi(u, 0) = 0` exactly when `(u, 0)` is in `S` (decided by polynomial
   arithmetic from `N, D`, independent of `psi`). Prediction: all agree.
9. `dlp` (deg F = 7): `D` the tested class of largest image order,
   `D' = [k] D` computed on `H`, `Psi(D') = [k] Psi(D)`, and the discrete
   log recovered on `C` by BSGS equals `k` modulo `ord Psi(D)`.

The identity `Psi([P-P']) + Psi([P'-P'']) = Psi([P-P''])` suggested in the
brief is a formal identity of the definition (it tests only the Pic
arithmetic); it is a unit test on Example 1 and is not counted as evidence
for C2. Checks 6-9 are the evidence that `Psi` respects linear equivalence.

## Controls

* Positive: Example 1. `#C(F_37^k)` must equal 42, 1342, 50070 (from the
  printed zeta numerator), the fitted quartic must be `F_37`-projectively
  equivalent to the printed one (exhaustive search for the projectivity),
  `#J = 55666`, and the transfer with `k = 22359` must verify.
* Negative: a perturbed `T` must fail the fit (unit test); a perturbed
  relation must map to a nonzero class (`relations_neg`, every curve).
* Counters: `count_quartic_points` against the independent brute-force
  enumerator of `plane_quartic_pic`'s tests; `count_h_points` against the
  printed zeta.

## Failure criteria

* C1 fails on a curve if 1, 2 or 3 fails there. Any such curve is reported
  with its `F, N, D`.
* C2 fails on a curve if any of 6-9 fails there (or `relations_neg` shows a
  zero, which would mean the relation check has no power).
* A check that could not run (no split function found, `transfer_divisor`
  found no representative) is reported in `NOTES` and counted as not run,
  never as a pass.
* No reruns with other seeds, no parameter changes after the freeze. Any
  deviation is listed in RESULTS.md.

## Not covered

`psi` at the fibres over `t = infinity` and outside `U`; primes beyond toy
size (counting is brute force); deg F = 8 class arithmetic (checks 7-9);
the general statement that `phi` is an isogeny (Smith's theorem, assumed in
the derivation and tested only through its consequences above).

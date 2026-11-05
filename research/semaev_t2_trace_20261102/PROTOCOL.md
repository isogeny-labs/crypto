# Semaev 2015, t = 2: first fall degree 2 versus the degree the solver reaches (2026-11-02)

Frozen before any computation. Target: `RT-SP-T2-TRACE-CHECK` (corpus nodes
`CEX-SP-SEMAEV15-T2-TRACE`, `CLAIM-SP-SEMAEV15-FFDA-T2`, `HEUR-SP-SEMAEV15-ASSUMPTION1`).

## Claim under test

Semaev, "New algorithm for the discrete logarithm problem on elliptic curves"
(arXiv:1504.01175v1, sha256 `4db8433b…`), §4.4 and §4.5. The setup:
- the first fall degree `d_ff` of system (11) is the least `d` with polynomials `g_i`,
  `max_i(deg g_i + deg f_i) = d`, `deg Σ g_i f_i < d` and `Σ g_i f_i ≠ 0`;
- the first fall degree assumption is `d_F4 ≤ d_ff`;
- for `S_3(x_1, x_2, z)` with `z` a constant, Semaev writes `deg_F2 S_3(x_1, x_2, z) = 2` and
  "the first fall degree … was bounded by 5 in [11]. The experiments show it is always 4
  again";
- Table 2 reports `d_F4 = 4` on its t = 2 rows `(n, m) = (15, 4), (15, 5), (16, 4), (19, 3),
  (21, 3)`.

Setup (§4.5):
- curve `Y² + XY = X³ + AX² + B`, so `S_3(x_1, x_2, x_3) = (x_1x_2 + x_1x_3 + x_2x_3)² + x_1x_2x_3 + B`;
- `F_{2^n} = F_2[α]/(f)` with `f` irreducible;
- `V` = the polynomials in `α` of degree `< k = ⌈n/m⌉`;
- the system for `t = 2` is the Weil descent of `S_3(x_1, x_2, z) = 0` with `x_1, x_2 ∈ V`:
  `n` Boolean coordinate equations in `2k` variables;
- `z` is random in `F_{2^n}` (standing in for `R_X`), and `B` is random.

## The certificate (hand argument, to be checked by computation)

For `λ ∈ F_{2^n}`, `Tr(λ u²) = Tr(√λ u)`. Hence

    Tr(λ S_3) = Tr(√λ (x_1x_2 + z(x_1 + x_2))) + Tr(λ z x_1 x_2) + Tr(λ B).

With `λ = z^{-2}` (`z ≠ 0`), `√λ = λ z = z^{-1}` and the quadratic terms cancel:

    Tr(z^{-2} S_3(x_1, x_2, z)) = Tr(x_1) + Tr(x_2) + Tr(B z^{-2}).

`Tr(λ S_3) = Σ_j c_j [S_3]_j` with `c_j = Tr(λ α^j)`, where `[S_3]_j` are the coordinate
equations. So this is a combination with constant multipliers of degree-2 equations whose
sum has degree ≤ 1. It is nonzero unless `Tr` vanishes on `V` and `Tr(B z^{-2}) = 0`.
`1 ∈ V` and `Tr(1) = n mod 2`, so for odd `n` it is always nonzero. If every coordinate
equation has degree exactly 2, there is no fall at 1, and `d_ff = 2`.

## Method

`semaev_t2_trace.py`, standard-library Python, exact `F_{2^n}` and Boolean-ring arithmetic.
For each cell `(n, m)` of the five t = 2 rows, `k = ⌈n/m⌉`:

**Field polynomials, two per n.**
- (P1) the lexicographically smallest irreducible trinomial `x^n + x^a + 1`, or the smallest
  pentanomial if no trinomial exists;
- (P2) a seeded random irreducible polynomial.

**Instances.** 20 seeded `(z, B)` pairs per cell and polynomial, with `z ≠ 0` and `A` not
needed (it does not enter `S_3`).

**Checks per instance.**
1. Build the `n` coordinate equations exactly. Record their degrees.
2. **First fall degree** of the coordinate system under Semaev's definition, `d ∈ {1, 2}`:
   - fall at 1 needs a generator of degree ≤ 1;
   - fall at 2 is found by the kernel of the map from constant multiplier vectors to the
     quadratic part of the sum, plus degree-1 multipliers on any degree-1 generators.
   Record whether a fall exists and a certificate.
3. **Explicit certificate.** Evaluate `Σ_j Tr(z^{-2} α^j) [S_3]_j` and check that it equals
   `Σ_{i<k} Tr(α^i)(a_i + b_i) + Tr(B z^{-2})` exactly.
4. **Solvability.** Brute force over the `2^{2k}` points of `V × V` (at most 16,384).
5. **Macaulay degree of 1** for systems with no solution: the least `D` such that the
   Boolean Macaulay span `{red(μ · [S_3]_j) : μ multilinear, deg μ ≤ D − 2}` contains 1,
   for `D = 2..5`. This is the degree a Macaulay/XL computation (§4.4, matrix `M_d`) must
   reach to prove there is no solution. It is reported next to Semaev's `d_F4 = 4`, not
   as F4's own trace.

## Decision rule

- Every instance has `d_ff = 2` with the certificate of check 3 matching: the statement
  "the first fall degree … is always 4" for `S_3(x_1, x_2, z)` is REFUTED on Semaev's own
  t = 2 cells, and `d_F4 ≤ d_ff` fails wherever the solver needs degree > 2. The corpus
  node becomes `native_exact`, and the hand argument becomes the theorem.
- Some instance has no fall at 2: record which (expected only when `Tr` vanishes on `V`
  and `Tr(B z^{-2}) = 0`, i.e. even `n`), and narrow the claim.
- The Macaulay degree of 1 is reported as measured. If it is ≤ 2 on some instances, those
  systems are solved at degree 2 and do not contradict `d_F4 ≤ d_ff`; say so.

No cost or speed claim. Exact computation on the session container (shared, L0).

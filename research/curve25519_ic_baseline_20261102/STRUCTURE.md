# Curve25519: the structure an index calculus can see

Every fact marked **(verified)** is re-derived by
`target/release/curve25519_family audit` (output `analysis/audit-255.json`) or
by that binary's unit tests; facts marked *(literature)* are cited and were not
re-derived here. Nothing in this note is a claim that Curve25519 is weaker than
its rho bound.

## 1. The curve

`E : y² = x³ + 486662·x² + x` over `F_p`, `p = 2^255 − 19` (Bernstein,
PKC 2006; RFC 7748). Birational to the twisted Edwards curve of Ed25519.

| property | value | status |
|---|---|---|
| `p` prime, `p mod 8` | prime, 5 | verified |
| `#E` | `8ℓ`, `ℓ = 2^252 + 27742317777372353535851937790883648493` prime | verified: `[ℓ](u=9) = O` by the x-only ladder, `[ℓ−1](9) ≠ O`, and `8ℓ` is the only multiple of `ℓ` in the Hasse interval |
| twist `#E'` | `4ℓ'`, `ℓ'` prime (253 bits) | verified, incl. `[4ℓ'](u=2) = O` on the twist |
| 2-Sylow of `E(F_p)` | cyclic `Z/8`: `A² − 4` is a non-square, so `T₀ = (0,0)` is the only rational 2-torsion point | verified |
| rational 4-torsion | at `x = 1` (`A + 2` square, `A − 2` non-square) | verified |
| trace `t = p + 1 − 8ℓ` | `−221938542218978828286815502327069187962` (`|t| ≈ 2^127.4`) | verified |
| anomalous / supersingular | no / no | verified |
| embedding degree | `> 10⁴` (no `p^e ≡ 1 mod ℓ` for `e ≤ 10⁴`) | verified (bounded check) |
| `j` | not 0 or 1728: no GLV automorphism beyond `±1` | verified |
| CM discriminant | squarefree part of `t² − 4p` is large (SafeCurves "disc" criterion) | literature; needs a 257-bit factorisation, not re-derived |
| matched rho | `≈ 0.886·√ℓ ≈ 2^125.8` group operations with the negation map | literature (SafeCurves) |

## 2. What is special, and what each property does to index calculus

**(a) Cofactor 8, cyclic.** `E(F_p) ≅ Z/8 × Z/ℓ`. The discrete log lives in
the order-`ℓ` subgroup, so rho pays `√ℓ`, but a factor base chosen by
abscissa (Semaev's "small x", the lab's `prime-abscissa`) is spread over all
`8ℓ` points. In an `m = 2` decomposition `R = ±P_i ± P_j`, only one pair sum
in eight lands in the subgroup that holds `R`, so the per-probe relation yield
is `≈ 2N²/p` rather than `2N²/ℓ`: the cofactor is paid in full by the
relation phase and not at all by rho. Two existing remedies:
`prime-projected-abscissa` (multiply each base point by 8 first) and the
cofactor-fiber oracles of `ic_framework::fiber` (probe all eight lifts
`R + K`, `K ∈ E[8]`), the latter so far implemented for binary curves only.
This is the effect measured by prediction P1 of the protocol.

**(b) The Kummer line and `T₀`.** Montgomery's x-only arithmetic works on
`E/±1 ≅ P¹`. Translation by `T₀` acts there as `x ↦ 1/x` **(verified)**,
and since `(P₁ + T₀) + (P₂ + T₀) = P₁ + P₂`,
`S₃(1/x₁, 1/x₂, x₃) = S₃(x₁, x₂, x₃)/(x₁x₂)²` **(verified)**, where the
Montgomery summation polynomial is

```
S₃(x₁, x₂, x₃) = (x₁ − x₂)²·x₃² − 2[(x₁x₂ + 1)(x₁ + x₂) + 2A·x₁x₂]·x₃ + (x₁x₂ − 1)²
```

(verified to vanish exactly on `x(P₁ ± P₂)`). This is the prime-field,
Montgomery-model instance of the 2-torsion symmetry that Faugère, Gaudry,
Huot and Renault (J. Cryptology 2014) used to cut the degree of `S_{m+1}`
for twisted Edwards / Jacobi models over `F_{q^n}`. Only `E[2]` acts on the
Kummer line (the order-4 and order-8 points do not commute with `±1`), so the
x-only symmetry group is `Z/2` per pair of summands: a factor `2^{m−1}` in
the degree of `S_{m+1}` after symmetrisation, and a factor 2 in factor-base
coverage at fixed unknowns when a base is closed under `x ↦ 1/x`.

**(c) No useful endomorphisms.** `j ≠ 0, 1728` and a large CM discriminant:
the only cheap automorphism is negation, which both rho (`√2`) and the IC
pair table (`negation_folded=1`) already use. There is no GLV orbit to fold
the factor base by, unlike the `j = 0` curves in `research/generator_multiplier_e2e_20261101`.

**(d) Prime field, no subfield.** `F_p` has no proper subfield and no
`F_p`-subspace structure to Weil-descend over, so the Gaudry–Diem route that
gives subexponential IC over `F_{q^n}` does not exist. Decomposition over
`F_p` reduces to finding small solutions of `S_{m+1}(x₁, …, x_m, x_R) = 0`
(Semaev 2004); Petit–Kosters–Messeng (PKC 2016) replace "small" by a tower of
rational-map images, which the lab has as `pkm-semaev`. No sub-`√ℓ`
algorithm is known for any prime-field curve; the lab's prime-field ladder
has rho ahead at every size (`docs/ic/LEADERBOARD.md`, Table B).

**(e) `p ≡ 5 (mod 8)`.** `−1` is a square, `2` is not; square roots take one
exponentiation (Atkin), which is the root-finding cost per trial of an
`m = 2` `S₃` decomposition. It does not change asymptotics.

**(f) Twist security and x-only.** The twist's `4ℓ'` protects the ladder from
invalid-point attacks. It does not touch the DLP on `E`, but it means a
"curve or twist" factor base (both halves of the Kummer line) has no cheap
second DLP to exploit.

## 3. The open-literature situation

There is no index-calculus paper aimed at Curve25519 specifically; the
relevant work is general: Semaev (2004) summation polynomials; Gaudry (2009)
and Diem (2011) over extension fields; Faugère–Gaudry–Huot–Renault (2014)
torsion symmetries; Petit–Kosters–Messeng (2016) prime-field towers;
Galbraith–Gaudry (2016) survey; Bernstein–Lange–Schwabe (2011) on the
negation map in rho. The two properties of Curve25519 that interact with IC
at all are (a) and (b); neither changes the exponent of a prime-field IC.

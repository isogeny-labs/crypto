# CT206 — the complementary elliptic factor is universally nonisogenous for odd-order sextic sources

2026-11-18. Continuation of [crypto PR #4715](https://github.com/isogeny-labs/crypto/pull/4715), stacked on #4697. This strengthens the *specific* CT205 F31 observation into a general theorem for this particular sextic-cover family. It is a written mathematical argument plus independently executable finite checks, **not** a Lean/PCC-certified theorem or measured ECDLP speedup. [pcc:unverified]

## Theorem and quantified scope

Let `p>3` be prime and `b in F_p^*`. Let

- `E_b: y²=x³+b`,
- `C_b: v²=u⁶+b`,
- `E_{b²}: y²=x³+b²`.

Assume **the entire** finite group `E_b(F_p)` has odd cardinality (not merely that some chosen subgroup has odd order). Then

```text
Hom_{F_p}(E_b, E_{b²}) = 0.
```

In particular the two elliptic quotients of `C_b` are **not F_p-isogenous**, so no nonzero **F_p-defined algebraic-group** map transfers the source ECDLP to the complementary elliptic factor. This is not an assertion about arbitrary computational encodings, higher-genus target index calculus, or abstract homomorphisms between finite groups of shared order.

## Proof (standard CM sextic-twist trace description + elementary exclusions)

1. If `p = 2 (mod 3)`, cubing permutes `F_p`. Writing `χ` for the quadratic character,
   `#E_b(F_p) = p+1+Σ_x χ(x³+b) = p+1`, which is even. Therefore the odd-cardinality hypothesis forces `p = 1 (mod 3)`.

2. If `b=c³`, then `(-c,0)` belongs to `E_b(F_p)` and is nonzero rational 2-torsion. Thus odd `#E_b` implies **b is not a cube**, in particular it is not a sixth power.

3. For `p = 1 (mod 3)`, the nonzero coefficients of `y²=x³+b` fall into six sextic-twist classes `F_p^*/(F_p^*)⁶`. Ordinary `j=0` complex multiplication gives a Frobenius `π=(t+v√(-3))/2` in `Z[(1+√(-3))/2]`, with integers `t,v` and `t²+3v²=4p`. Multiplication of `π` by the six units gives the following **six possible Frobenius traces**, one per sextic-twist class:
   ```text
   ±t, ±(t+3v)/2, ±(t-3v)/2 .
   ```
   A collision among these six would imply at least one of
   `t=0`, `v=0`, `t=v`, `t=-v`, `t=3v`, `t=-3v`.
   Substitution into `t²+3v²=4p` makes `p` either a square or three times a square. This is impossible for prime `p>3`. **Hence distinct sextic-twist classes have distinct Frobenius traces.**

4. The two quotient elliptic curves have coefficient ratio `b²/b=b`. Step 2 says `b` is not a sixth power. Step 3 gives distinct Frobenius traces, so the two curves are not isogenous over `F_p` (the standard finite-field elliptic isogeny criterion). Any nonzero `F_p`-defined homomorphism between elliptic curves is an isogeny; therefore `Hom_{F_p}(E_b,E_{b²})=0`.

QED, conditional only on the stated standard CM trace/twist description and elliptic isogeny criterion. This argument has **not** been formalized in Lean and has no independent PCC-signed proof verdict.

## Consequence for the tested double cover

The two quotient maps are `pi_1(u,v)=(u²,v)` and `pi_2(u,v)=(b/u²,bv/u³)` away from their boundary points (their morphism extensions are used for the global statement). The map `pi_{2,*} pi_1^*: E_b -> E_{b²}` is an `F_p`-defined algebraic-group homomorphism, so by the theorem it is identically zero whenever `#E_b(F_p)` is odd.

This strengthens CT205's F31 numerical gcd(43,39)=1: **the algebraic cross-factor map vanishes for every prime/parameter meeting the quantified parity hypothesis**, including hypothetical examples where the rational group orders might share factors. The retained source logarithm remains in the source-isogenous factor of `Jac(C_b)`; the complementary factor cannot supply an algebraic shortcut under these assumptions. The main open option is a genuinely cheaper complete algorithm *inside the transferred representation* or a different transfer family.

Do **not** infer that a general cover, a chain of other maps, or an index-calculus Jacobian algorithm is impossible. For `p=2 mod 3` all the `E_b` have even order, and the odd-order theorem does not apply. Likewise, an odd prime-order subgroup of an even-order elliptic group does not satisfy the hypothesis automatically.

## Finite reproducibility and negative controls

```sh
python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct206_sextic_obstruction.py
```

The independent stdlib-only script enumerates all `44` primes `5..199`, all `4178` nonzero coefficients, the six twist classes for each of `21` primes `p=1 mod 3`, and the `1352` cases with odd full elliptic-group order. It checks that each class has constant trace, all six traces are distinct, every odd-order source has a different companion trace, and no rational `u⁶=-b` branch point exists. It contains an artificial trace-collision rejection control. **Finite coverage is a consistency check, not a proof of the theorem or a cryptographic-size experiment.** [pcc:unverified]

## Prior art, limitations, and next gate

The underlying split-Jacobian geometry and sextic decomposition were already present in autoresearcher `ledger/proposals/IDEA-20261109-4668b8.yaml` and in the literature on paired elliptic subcovers: [Shaska, 2012](https://arxiv.org/abs/1209.3187). The classification of sextic twists and their Frobenius action is standard, and the [Sage sextic-twist implementation and documentation](https://github.com/sagemath/sage/blob/develop/src/sage/schemes/elliptic_curves/ell_field.py) provide an external implementation reference. The **new contribution here is the explicit all-odd-order family obstruction and its audited consequence for this particular transfer lead**, not a novel twist construction.

First resolve the current CT205 native Rust CI verdict and test the actual 43/43 target recoveries with successful exact-head compilation. Then price the real Jacobian arithmetic and destination IC, or switch to alternative covers with a provable route into a nontrivial factor. Resume the owner-prioritized original 695/701-record `C169` vs `C13a+C13b` pricing when byte-authenticated custody and authorized storage access are restored. Complete cold same-host rho/BSGS/IC comparisons, memory, blind target provenance, uncertainty and independent PCC remain mandatory before any speedup statement.

# Addendum to the affine-type transfer literature note

Status: literature correction, `[pcc:unverified]`. This does not change any measured number
in `RESULTS.md`; it corrects the “what remains” claim that no explicit isogeny to a
non-hyperelliptic quotient is known.

## Correction

`RESULTS.md` says:

> Milio’s theta method gives no explicit map to non-hyperelliptic quotients [lit]

and `LITERATURE.md` records the same as “not found”. That is too strong for the cited
paper. E. Milio, *Computing isogenies between Jacobians of curves of genus 2 and 3*
(HAL `hal-01589683`, 2019) gives:

- **Section 4**: rational fractions `S, P, A, R, T, E` describing the `(ℓ,…,ℓ)`-isogeny
  on points, with the image of a single formal point and a continued-fraction
  reconstruction; the genus-3 Mumford form is
  `<X³ − S(u)X² + P(u)X − A(u), v(R(u)X² − T(u)X + E(u))>`.
- **Section 5.3, Theorem 14**: the plane quartic is reconstructed from an Aronhold
  system of seven bitangents, whose coefficients are explicit theta constants
  (`α_ij` in Proposition 13), using Riemann’s equation
  `(x₁ξ₁ + x₂ξ₂ − x₃ξ₃)² − 4x₁ξ₁x₂ξ₂ = 0`.

So the gate is an **implementation** task, not a missing theorem. The explicit
codomain-theta formulas for radical 2-isogenies in dimension three are also given by
Kunzweiler et al., *Radical 2-isogenies and cryptographic hash functions in dimensions
1, 2 and 3* (theta coordinates).

## What remains hard

The laboratory instrument computes theta-null **products** (the 18 pairs covering the 36
even characteristics), so recovering the individual theta constants needed by the
Aronhold formulas has a square-root/sign ambiguity. The `chi_18` square-class test can
select the branch, but the reconstructed quartic must still be checked against the
codomain (Igusa invariants or theta constants), and the point map must be evaluated and
verified.

## Next step

On one p ≈ 47 prime-order example with a known affine kernel whose codomain is a Jacobian
over F_p:

1. recover the codomain theta constants from the instrument’s products;
2. build the seven Aronhold bitangents and Riemann’s quartic;
3. compute the isogeny on a few divisors with Milio’s rational fractions;
4. check `[r]Ψ(G) = 0` and `Ψ(G) ≠ 0` in the codomain.

If step 1 or 2 fails, the failure is a concrete reconstruction obstruction, not a
literature gap.

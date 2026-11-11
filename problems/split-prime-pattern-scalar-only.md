# Split-prime (Elkies) pattern of a prime-order curve reaches its group only through scalars

- **Status:** `open` (derived obstruction for the ℓ-adic / class-group route; the coordinate route stays open under the existing prime-field index-calculus problems)
- **Kind:** derived obstruction (scalar-only action, proof sketch, no Lean proof yet), plus a measured null for the one indirect route (model choice within the class)
- **Affected work:** [`research/split_prime_ecdlp_20261107/README.md`](../research/split_prime_ecdlp_20261107/README.md) (the request for a novel ECDLP algorithm built on the split-prime deficit of the published NIST / X9.62 prime curves); [`research/isogeny_class_difficulty_20261008/README.md`](../research/isogeny_class_difficulty_20261008/README.md) (framing theorem); [`research/isogeny_class_ic_yield_20261102/p2_prime_analogues/RESULTS.md`](../research/isogeny_class_ic_yield_20261102/p2_prime_analogues/RESULTS.md) (measured null)
- **Next action owner:** prime-field algorithm research; no unattended job scheduled
- **Last updated:** 2026-11-07

## Problem

We cannot turn the observed deficit of split small primes ℓ in the Frobenius
field K = Q(√(t² − 4p)) of the ten published prime curves into an ECDLP
algorithm below the generic bound, because for ℓ ≠ n every object the
pattern governs (E[ℓ], its Frobenius eigenlines, the two rational
ℓ-isogenies of a split ℓ, the horizontal class Cl(O), the endomorphism ring
O and the Galois action on ℓ-power torsion) acts on the cyclic group E(F_p)
of prime order n through Z-linear maps, i.e. multiplication by a known
scalar, or carries no information about the point at all.

Scope: E/F_p ordinary, p > 3, #E(F_p) = n prime, cofactor 1 (the ten curves
prime192v1/v2/v3, secp224r1, prime239v1/v2/v3, prime256v1, secp384r1,
secp521r1, parameters as printed by OpenSSL 3.0.13). Threat model: classical
algorithms with all costs charged. Acceptance criterion of the original
request: an essentially new mechanism with a falsifiable prediction and a
measured cost reduction against the current prime-field index-calculus baseline at
some scope (CLAUDE.md success criterion).

## Evidence and uncertainty

- **Observed:** the deficit itself was recounted natively from the curve
  parameters (`research/split_prime_ecdlp_20261107/tools/splitcount.rs`,
  output `data/recount.json`): split counts 60, 58, 57, 50, 42, 57, 57, 57,
  54, 52 at ℓ ≤ 691 against a conditioned expectation of 59.7 to 60.3
  (sd 5.5); pooled z −3.13 at 691 and between −1.8 and −3.0 at every cutoff
  from 100 to 200,000. Every count and z the recount can compare matches the uploaded report
  (sha256 d38be158…); its calibrated bands are not reproduced and its
  L(1, χ) normalisation is questioned (study README §4) [pcc:unverified].
- **Derived or established:** prior in-repo form: `RESEARCH_ISOGENY_FIBERS_ECDLP.md`
  §15.1, Theorem 15.1 (isogeny-augmented generic group model, success
  probability at most (Q + 2)²/r + 1/r; [pcc:unverified] there). Then (i) Lenstra: E(F_p) ≅ O/(π − 1) as O-modules when
  End(E) = O, so every α ∈ O acts as the scalar α mod (π − 1), and every
  F_p-rational isogeny φ: E → E′ of degree prime to n restricts to a group
  isomorphism with φ([k]P) = [k]φ(P). (ii) A point of order n is invisible
  to every ℓ-pairing and ℓ-torsion structure with ℓ ≠ n (bilinearity:
  t(T, P)^n = 1 = t(T, P)^ℓ). (iii) Therefore an algorithm whose only
  non-group-operation inputs are such maps is a generic-group algorithm with
  scalar oracles, and Shoup's Ω(√n) bound applies; Jao–Miller–Venkatesan's
  random self-reduction (horizontal walks over exactly the split ℓ) makes
  every curve of the class equally hard. Full statement and proof sketch:
  `research/split_prime_ecdlp_20261107/README.md` §3. No Lean proof yet. The
  statistic itself was already computed by the lab for the published and
  omitted curves (`research/nist_skip_criterion_20261025/strict_elkies_20261029/results/at691.tsv`),
  with the same counts.
- **Suspected explanation:** none needed for the obstruction. For the
  deficit itself the uploaded report and this study agree that no mechanism
  is known; this record takes no position on its cause.
- **Missing capability:** a factor base on E(F_p) with an algebraic
  description of degree far below its size (the standing prime-field
  index-calculus gap, `problems/prime-scalar-exact-splitting.md`,
  `problems/tree-glued-pdp-separator-coupling.md`).
- **What this evidence does not establish:** it does not bound coordinate-
  based methods (summation polynomials, division polynomials), which are the
  lab's open tracks; it does not cover the p-adic canonical-lift presentation
  (every point of E(F_p) lifts to Ẽ(Z_p); believed closed for n ≠ p because
  the p-adic elliptic logarithm vanishes on torsion, an assertion here, not a
  checked claim); it does not test whether the pattern correlates with
  coordinate-method yield between isogeny classes (Δ is a class invariant,
  so the within-class null cannot; study README §6 P3); it does not say the deficit is harmless for other
  purposes (it lowers the horizontal expander degree, so the JMV transport
  needs larger ℓ for the same mixing); it does not establish anything about the
  generation process of the curves.

## What would unblock it

A non-scalar entry point would have to be a map E(F_p) → (some structure)
that is not Z-linear and is not a function of F_p-coordinates alone.

- [ ] Exhibit a computable map f on E(F_p) that depends on the ℓ-adic or
  class-group structure, is not of the form P ↦ [s]P or P ↦ φ(P) composed
  with coordinate functions, and whose output for P and Q = [k]P reveals
  information about k beyond what a generic-group oracle gives; verify on a
  24-bit prime-order toy class with a planted k and an A/A control.
- [ ] Or: a Lean statement of (i)–(iii) above, so the closure of this route
  is a Lean-checked theorem rather than a derived note.

Meeting the first condition would reopen the route; meeting the second
would close it formally. Neither changes the coordinate-based open problems.

## Next discriminating step

- **Action or proof obligation:** formalise (i)–(iii) in `crypto-lean`
  (statement: an endomorphism of a cyclic group of prime order is a scalar;
  an isogeny of degree prime to n is an isomorphism on E[n]).
- **Inputs and controls:** none beyond the definitions.
- **Execution limits and stop condition:** one Lean session; stop when the
  statement binds or a hole is identified.
- **Outcomes and next actions:** bound → status `resolved (Lean-checked)`; hole →
  record it here.
- **Recheck trigger:** any proposal that claims a non-scalar entry point.

## Resolution and history

- 2026-11-07: opened with the native recount and the derived obstruction
  (`research/split_prime_ecdlp_20261107/`).
- 2026-11-10: the exact class groups of the five P-192 Frobenius fields
  (`research/split_prime_ecdlp_20261107/rationales/README.md` §5, relation
  method in `crypto::isogeny::class_group_subexp`) have the genus-theory
  2-rank and are generated by the split primes below 163 on every curve,
  published or skipped; no non-scalar entry point appears, status unchanged.

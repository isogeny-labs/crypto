# Formal scope

KoblitzCertificates31.lean contains ten named structural theorems:

- cyclic_translation_order: the order of addition by s in Z/m is m/gcd(m,s).
- cyclic_translation_period: precisely the multiples of that order annihilate s.
- relabelled_root_product: an explicit equivalence preserving factors preserves
  the complete product. It assumes the equivalence; CM31 checks its finite data
  natively, not in Lean.
- tate_normalization_a1, tate_normalization_b2, tate_normalization_b3: universal
  rational identities, with every required nonzero denominator an assumption.
- twist_quadratic_coefficient, twist_linear_coefficient,
  twist_constant_coefficient: all three QR cubic coefficient changes under
  T²+T=1 in characteristic2.
- diamond_sign_involution: adding the sign shifts twice is the identity.

The pinned compiler and transitive axiom audit are recorded in lean31-01.
The actual finite-field incidence records and CM root arrays are native checked,
not Lean-imported certificates. Total cocycle genus, modularity, normalization,
good reduction, proper polarized embedding, class nonprincipality and the
order11 modular-factor map are not formalized here. Native assertions are
neither independent mathematical replay nor proof-assistant verification.

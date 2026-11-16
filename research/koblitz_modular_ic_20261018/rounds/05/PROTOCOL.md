# Round 05 protocol — frozen before experiments

Parent: 75b902661f8e0047281d847d9f1b6ec35827f22b. Date: 2026-10-19.
Continue the existing PR; preserve all earlier rounds byte for byte.
This is exact algebra and geometry research, with no timing or IC speed claim.

1. **S1, reverse correspondence and 11-saturation.** Derive the Gram matrix
   of the twelve degree-660 modular maps using the deck action, their
   Hecke sum and a_11=4. Enumerate PSL2(F11) and its ordered-pair orbits;
   verify the proposed Gram and both signed five-coordinate projections
   by integer matrix arithmetic. Derive the polarized adjoint composition,
   including the nonprincipal target polarization. Do not identify a rational
   adjoint with an integral reverse map without checking denominators at 11.
   Formalize the relevant matrix and torsion implications in Lean. Separate
   standard geometry arguments from formal proofs and finite checks.
2. **M6, level-dependent divisors.** Factor the frozen degree-15
   psi_11(1,b) over F2 exhaustively (trial monic factors through degree 7,
   certify irreducible residuals). Count the corresponding ordinary curves
   over each root field, and enumerate the projective Frobenius action on
   cyclic order-49 subgroups when the integral Frobenius model is justified.
   Resolve the ordinary zero support and explain the missing poles. A row
   with incomplete support is rejected and contributes zero rank.
3. **Q3, remaining quotient equations.** Test the five determinant-5
   Hermitian forms from round 03 for explicit integral equivalences to the
   natural form. Bound search coefficients by [-2,2] in the cyclic
   O_-7 algebra; retain every hit and report the bound on misses. Turn
   successful equivalences into explicit quotient maps/equations through
   the natural Weil restriction. Do not identify arbitrary finite gluing
   graphs with the natural graph without an explicit kernel check.
4. **S2, reverse divisor representation.** Give an explicit pullback-divisor
   formula for the reverse correspondence, calculate integer coefficient
   sizes/degrees, and distinguish a correspondence formula from evaluated
   split coordinates in a Jacobian. Evaluate any new finite fibers only
   after an amendment fixing fields and search bounds.

Native Rust performs arithmetic, enumeration, polynomial factoring, record
assembly and independent verification. Raw JSON and failed commands are
write-once. Record protocol amendments before their experiments. Add
identity.certificate/v1 records and Lean claims only with their exact scope.
Replay new records independently; rerun the prior round-04 replay after
changing shared native helpers. Build and audit the Lean project, run the
required root library/native tests, render the report with editable sources,
inspect every PDF page, and archive new raw runs with SHA-256 manifests.

## Amendment A — after the first M6 and Q3 outputs

The zero polynomial is irreducible of degree 15, rather than three quintics.
The a=0 curve has trace -231; its x=1 point becomes rational on the a=1
twist, with trace +231. Preserve the initial failed y-coordinate unwrap.
M6's first implementation admitted only valuation-one discriminants, so it
left its projective-orbit field null. A 7-adic unit discriminant also gives
the maximal unramified order. M6b will enumerate the same 56 projective
lines for valuation zero or one, then all 56*11 normalized complements
under Frobenius^15 (complement r -> -r after choosing the eigenbasis).
Record the resulting closed zero degrees, with poles still unresolved.

All five determinant-five forms have equivalences in the frozen box. Q3b
will check the ten frozen first anti-isometry graphs modulo 5 against those
equivalences, allowing the elliptic sign automorphisms +/-1. It will not
replace a failed graph match with a new graph and call it the original one.
Generate exact integral inverse and pairing identities for Lean from the
five retained equivalences. No larger search bounds are introduced.

## Amendment B — supersingular poles and a symbolic principal row

The division polynomial is monic in x, so x(P) is integral over F2[b]; its
poles can occur only above b=infinity (the supersingular j=0 fiber).
Resolve that fiber using the deformation y^2+s*x*y+y=x^3, with
Delta=1+s^3 and j=s^12/(1+s^3). Normalize x=s^2*X+1/s and
y=s^3*Y+1/s^3. Check the cleared polynomial identity natively and in Lean.
Every nonzero 11-torsion section has X=(s*x+1)/s^3 of pole order 3.
There are 56*660/12=3080 geometric supersingular points on the full
49/11 modular curve, since the order-12 automorphism group modulo +/-1
acts freely on full11 bases. Verify the free action on 660 bases using
the faithful mod11 representation where practical; otherwise keep this
standard prime-to-characteristic automorphism fact as a written bridge.

M7 will record the complete pole stratum and multiplicity, check E49(F64)
has order 56 and its six supersingular modular base images have no
11-primary component, and assemble one symbolic principal row on the 44
degree210 zero columns. Extend the old 544-column matrix with those 44
disjoint columns and check its rank. Keep symbolic divisor/stratum
certification distinct from evaluated new column coordinates or a reduced
Jacobian divisor. Do not count the unresolved new point images as evaluated.

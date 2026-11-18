# General Kummer invariant quintic gate

Prospective producer assertions [pcc:unverified]. Original p601,c7 and
explicit auxiliary-twist kernel R,S from twist-kernel unchanged. Use prior
generic-kummer/terms.rs binomial biquadratics at c7, original-side Kummer
projection with v1² divided by7 for twist inputs. No new targets/curves.

Implement Corte-RealSantos/Flynn section5 Eq5.1-5.2 using all56 index
multisets and all120 permutations with multiplicity. Do not assume the
conjectural12index shortcut. Polynomial arithmetic mod601, degree5 basis56.
Quotient by four linear multiples of binomial Kummer quartic
p⁴+s²h²-2sp²h-4k1ph²-28k1³h-28k1s³+28k1²sp.
Predict invariant dimensions12and12, combined20, intersection4. Write all
four map-coordinate coefficient vectors to stdout in monomials(5) order.

Fixed samples: identity,R,S and first32twist catalogue point divisors and
their S-translates (67total, duplicates retained). Require sourcequartic
zero, nonzero map vector and projective invariance under R,S translations.
Allsamplepairs: map equality iff their difference OR sum belongs to the
25point kernel, accounting for Kummer negation. This is finite map
qualification, not exhaustive morphism proof, quotient curve recovery,
independent pairing check or speed measurement. Identity/kernel exceptional
projection cases must pass; do not silently discard them.

Primary source https://people.maths.ox.ac.uk/flynn/arts/art43.pdf section5
read directly; older generic-kummer SOURCES.md and terms.rs retain source
formula provenance. Existing KB Kummer hypothesis089d15ad1d3d77cf4e3619a73f9982f4
is background only. Prediction may fail due normalization/projective lifts,
exceptional inputs, polarization or implementation; preserve failures.
Freeze source/import/native/prior certificates/binary hashes and push Git
before one120CPU-second invocation. No scientific retry/S3/cloud.

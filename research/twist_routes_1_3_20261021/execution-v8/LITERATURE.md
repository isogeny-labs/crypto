# Sources reviewed before implementation

The existing E14 explicit-map protocol and native Vélu implementation were
read, together with the v7 maximal-generator admission and the proposed
twist-kernel card. The p827 group is C7 x C119; a point of order833 does not
exist. This experiment uses its full 7-torsion, rather than projecting with
833/7 (which would kill that torsion).

Primary source read: Andrew Sutherland, MIT18.783 Fall2025 Lecture5,
[Isogeny kernels and division polynomials](https://math.mit.edu/classes/18.783/2025/LectureNotes5.pdf),
Theorem5.11, the field-of-definition paragraph, Theorem5.15 and Remarks5.16–17
(pp5–8). A Galois-stable kernel, whose individual points need not be rational,
defines a rational isogeny. The paired-root Vélu formula and derivative
y-map below are precisely this known construction. Download succeeded on
2026-10-27; text was read before selecting the implementation.

PARI primary local help `?ellisogeny` and an independent development example
over F11 were inspected. PARI returns `[image,[N,y*M,H]]`; `ellisogenyapply`
uses denominators H^2,H^3. No `ellisogenydual` identifier is available in this
installation. Duals will instead be built from the image of the full7-torsion
and checked by composition, not inferred from equal cardinalities.

Atlas sources read in the sibling crypto-autoresearcher checkout:
`LEAD-STR-isogeny-class-generic-invariance`,
`LEAD-TRF-prime-field-isogeny-to-weak`, and
`LEAD-STR-torsion-module-leak`. Coprime-degree maps preserve the prime subgroup;
same-order isogeny transport alone changes neither generic DLP difficulty nor
the relation matrix of a transported base. Low embedding degree/anomalous
weakness is not created by changing representatives in a prime-field class.

The web-search connector returned an empty error response. Direct primary MIT
access succeeded. This is an implementation/connection check, not a novelty
survey or a claim that these standard isogenies are new.

#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
= Certified adaptive power-image recurrence
T-1820 · 2026-11-07 · derived algebra and native producer checks

Given squarefree G of degree $n<r$, compute B = X^m mod G and moments starting at index zero. Reconstruct a shortest recurrence C from prefixes of length 2,4,8,... . Accept only when C(B) vanishes modulo G. This certificate uses the whole quotient, without enumerating candidate roots.

#image("flow.svg", width: 100%)

*Exactness.* The trace sequence is a sum of t distinct exponential sequences with nonzero image multiplicities. Its minimal polynomial D is the product of the distinct image factors. A shortest prefix recurrence has degree at most t. The quotient certificate implies D divides C, hence C=D. At 2t moments the Vandermonde argument forces D; doubling stops before 4t moments. Include the initial moment: a zero image contributes only there.

*Derived work.* The sequential adaptive core costs O(M(n)(log m + t) + nt + t² + t log r), including moment generation, repeated recurrence synthesis and quotient certificates. Stored coefficient arrays use O(n+t), plus backend temporaries. Squarefree validation, envelope output and original-parent checks are additional. Full blocked projection remains a separate option when images are numerous; there is no total-cost or timing ranking here.

*Finite checks.* All 6,522 accepted subsets over scalar primes 7 and 11 match whole-polynomial schoolbook references and prior sequential/blocked/lcm constructors. Three larger cases use scalar prime 65,537. The certificate rejects 9,185 provisional prefixes. In 3,233 cases the adaptive method uses fewer than n+1 moments. Corrupting the accepted polynomial constant is rejected on every nonempty small case. Named controls retain the zero-image transient, single image at two moments, degree-r rejection, empty input and moment limit.

*Public integration.* All 63 inherited recoveries agree in scalar, route and query fields, with 511 source calls. The curve panels are F7/r13 (m3,6), F509/r479 (m2), and F701/r683 (m11). Runtime uses public points; labels remain in test references. Original 4,563-case cohort unchanged. No cloud work.

*Evidence.* validation-01 is preserved with source snapshots; validation-02 is final. Receipts bind commands, compiler, stdout and dependency hashes. These are producer checks, with no independent signed verdict or full formalization. Native compilation and assertions passed; compiler warnings are retained.

*Sources and next construction.* FLINT nmod_poly and fmpz_mod_poly recurrence documentation; exact URLs and proof in DERIVATION.md. Berlekamp-Massey and trace minimal polynomials are established methods. KB unavailable. Next account for certificate overhead and source cost when selecting between adaptive recurrence, blocked projection and lcm closure.

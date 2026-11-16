# Fixed-domain radix presentation census

This next component keeps the original prime candidate interval [0,2^27) and its existing point manifest. It varies only the polynomial presentation for m=2. It does not replace the active centered-panel run or the full three-family m2-9 objective.

## Source and prior work

The retained HILBERT_DESIGN.md derives the m2 top quotient and its bidegree rank formula. PRIME_WEIGHT_NORMALIZATION_NOTE.md rules out changing its graded structure through nonzero weights alone. Phuong and Tran, Theorem 3.1, https://arxiv.org/pdf/2211.13548 (pages 6-7), establishes the strong Lefschetz property for monomial complete intersections in characteristic greater than their socle degree. The sum-of-variables form follows from the proof and invertible diagonal scaling. This characteristic condition must be checked; small-characteristic controls must not be assigned these ranks automatically.

## Domain and presentation

Enumerate nondecreasing integer partitions k of 27. A part k_j defines radix r_j=2^k_j and digit d_j in {0,...,r_j-1}, with polynomial product_(a=0)^(r_j-1)(d_j-a)=0. Set w_0=1, w_j=product_(h<j)r_h and X=sum_j w_j*d_j. Mixed-radix expansion is a bijection onto [0,2^27); the modulus exceeds this interval. Reuse the identical lift convention and original point manifest. Ordering the radices changes weights but not the top quotient, so partitions cover the graded structures in this family. No point enumeration is repeated.

Use the exact original m2 Semaev equation with this X substitution. Its highest form is X_0^2 X_1^2. After nonzero weight normalization the quotient is (A tensor A)/(L_0^2 L_1^2), where A=k[z_j]/(z_j^r_j), L=sum z_j, and D=sum(r_j-1). All variables have ordinary degree one; changing the number and degrees of selector constraints is the explicit experimental factor.

## Exact and bounded records

The original all-binary baseline has recorded first graded vanishing degree 42. Every candidate has a nonzero quotient component in bidegree (D,1): A_D is one-dimensional, A_1 is nonzero, and the degree-one second factor has no image under multiplication by L^2. Therefore first vanishing degree is at least D+2, without a Lefschetz assumption. Candidates with D>40 cannot beat 42; retain that rigorous lower bound and leave their exact degree null rather than materialize enormous Hilbert arrays.

For every candidate with D<=40, check p>D and compute h_i from product_j(1+t+...+t^(r_j-1)), r_i=min(h_(i-2),h_i), and H_d=sum_(i+j=d)(h_i*h_j-r_i*r_j). Retain the complete arrays, characteristic check, first zero, preceding coefficient, total quotient dimension, and candidate count. Do not hard-code which partition wins. Integer coefficients fit u128 because the total unquotiented dimension is 2^54. Use native Rust for this computation and deterministic fresh replay.

## Controls before production

For every partition of bit widths 1 through 6, independently enumerate bounded-exponent monomials and dense multiplication-by-L^2 matrices in every degree. Compare Gaussian ranks over F101 with the formula and include F2/F3 records showing actual finite-characteristic failures. Preserve all control rows and failures. Check mixed-radix encode/decode exhaustively on these small domains, and reject zero/invalid parts. Production is a separate frozen protocol and EKS request after these development controls pass. No production census has run when this design is filed.

## Claim boundaries and storage

This is an exhaustive search over a specified same-domain presentation family for the original-top first graded vanishing convention. It does not measure affine FFD, LFD, CM regularity of positive-dimensional higher-m systems, or runtime improvement. A different candidate base is not generated. Records must name the original prime base manifest and source identity, digit radices/weights, characteristic hypothesis, exact-vs-lower-bound status, and raw arrays. Keep the original all-binary equation source and results immutable. Publish native source, controls, frozen production protocol, all per-partition records, replay logs, format description and failed attempts in Git/S3 before drawing a search conclusion.

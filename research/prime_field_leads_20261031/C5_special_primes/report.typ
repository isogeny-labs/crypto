#set page(margin: 2cm)
#set text(size: 10pt)
= C5: Special prime shapes, closure note

Writeup. Derivation only; no counted measurement. Purpose: record why 2^255−19 and the Solinas primes touch field arithmetic only.

+ Doubling does not preserve "x has small high bits". x(2P) = ((x²−a)² − 8bx)/(4(x³+ax+b)); the set of x with top k bits zero (density 2^(−k)) is not preserved.
+ Multiplication by 2 on x is not a curve map. x-coordinates of E are about half of F_p; 2x leaves that set.
+ No proper digit-pattern base is closed under the group law. In a prime-order group, an addition-closed nonempty subset is the whole group.

Exceptions not reopened: negation (factor 2), rational 2-torsion Möbius symmetry and automorphisms at j=0, 1728 (owned by LEAD-ICP-xline-symmetry-special-j, open_unexamined).

#image("closure_diagram.svg", width: 100%)

Sources: RESEARCH_NIST_CYCLOTOMIC_POINT_HEIGHTS.md §3 (corrected statements only); card LEAD-ICP-xline-symmetry-special-j.

# Odd-base-field extension of the coprime lift

Over any field K, Z² and (Z−1)³ are coprime because (Z−1)³ evaluated at0 is−1. The map (a,b)→(Z−1)³a+Z²b on deg(a)<2,deg(b)<3 is injective: divisibility by Z² forces a=0, then b=0. Equal finite dimensions prove invertibility. Characteristic3 causes repeated factors within (Z−1)³, but does not break coprimality with Z². Implement the five-by-five inverse by exact field Gaussian elimination and verify its product action. Corrections now subtract F−A*B with proper signs, rather than binary XOR formulas.

The same triangular induction and full bounded-product check prove exact normalized bounded-factorization recovery. Tiny tests over GF3,GF5,GF9,GF25,GF27 include characteristic3 repeated leading factors; independent monic division exhausts all bounded degree2factors when q≤9,d2. Planted arbitrary pairs exercise coefficients outside the prime subfield. Every nonzero tiny element is checked for a multiplicative inverse, guarding bad modulus choices. This establishes only component arithmetic/correctness for those declared fields, not speed or curve-system coverage.

Odd polynomial-prefix domains require digit multiplicity strata and factors for each distinct leading digit, retaining all tuples. NIST prime-field integer intervals have no polynomial-prefix representation; quotient/carry constraints and wrap coefficients need a separate proof. This component does not claim a prime-interval solver or any allcurve/allscale result. Known coprime Hensel lifting is prior art, not a certified novel algorithm.

Worker90s/4GiB/onecore;78sdemotion/84sfinalization. The small configured bound q≤27prevents domain enumeration at cryptographic field size. Output complete flags for original systems remain false.

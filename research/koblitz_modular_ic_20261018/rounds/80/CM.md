# R80: both candidate reduced fibers split as 2+2

Two frozen, serial Singular programs loaded the retained R50 `normalP`
candidate, checked its 22-variable/400-relation ring and literal
`(x,y,p)` source map, specialized `p=0`, and imposed `y=0` or `y=1`.
Each full quotient has F2 dimension 16 and annihilates the matching
R78 source-model factor. The radical of each quotient has F2 dimension
4. `minAssGTZ` returned exactly two prime quotients of dimension 2
over F2 in each case, with the exhaustive degree sum `2+2=4`.

The two independent runs exited zero, printed their exact terminal
markers, and had zero native `?` errors. Twelve historical source-byte
pins and seven donor hashes were checked. Singular emitted only a
routine `prime80` redefinition message on the second loop iteration.

This is a **candidate-algebra diagnostic**. R50's global `normalP`
candidate was rejected by its coefficient-descent gate; its later
global verifier timed out. These four degree-2 factors are therefore
not certified normal branches. In particular, the `y=1` source-model
reduction has residue algebra F2 (R78), while its candidate algebra
has two F4 factors; that discrepancy must be explained by an integral
closure and source-field compatibility certificate before using them
for place counts, ramification, or divisors.

No new Lean theorem is asserted for a CAS minimal-prime claim. The
R78 Lean audit checks the chart identity and source factorization;
R80's radical and primality computations remain native CAS claims.

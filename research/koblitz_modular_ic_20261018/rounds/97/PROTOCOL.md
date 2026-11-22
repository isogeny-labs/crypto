# R97 corrected marked-23 base discriminant locus, frozen before execution

R96 was strictly rejected after `deg(poly,poly)` native errors. Its partial
export is diagnostic only. This round replaces every degree call by
`deg(poly,intvec)` with explicit x-weights `(1,0)` and y-weights `(0,1)`;
the full source-bound computation and replay are repeated. No R96 output
is an input to the accepted calculation.

The R63 accepted source is a degree-112 field over F2(x) built from the
actual marked-23 degree-7 base polynomial `f23(x,y)` and a degree-16
lambda equation. This round computes the **complete finite discriminant
locus of the degree-7 base order** F2[x,y]/(f23), not the degree-112
normalization or its branch indices. A nonzero resultant certifies
generic separability; factors of the resultant identify every finite
x-fiber where this monogenic order can fail to be etale. Actual normalized
branches still require local integral closure.

Bind the polynomial to R52's saved `marked23-field51.sing`. Check degree7,
monicity, the exact x=0 and x=1 reductions, nonzero derivative and
resultant, univariate resultant, all irreducible factors with exponents,
the full factor product and a corrupted-product rejection. Export source,
derivative, resultant and factors. Fresh replay in a separate Singular
process must rebind the donor, recompute the derivative/resultant,
specializations and factor product.

One pinned local Singular process, seed97, one thread, 2 GiB virtual
memory and 300 seconds. Freeze source and donor SHA-256/byte counts in
Git before execution; one attempt `discriminant97-01`, no retries with
changed source. Accept only native exit0, exact pass marker, zero `?`
errors, complete export and fresh replay. Retain any failed attempt.
This exact algebra computation has no timing comparison or statistical
inference. The default EKS image lacks the pinned Singular CAS; a custom
digest-pinned image is required before moving this workload to Spot.

Sources checked: R52 `CM.md` and actual marked-23 donor; R63 `CM.md`
and `GENERATOR112.sing`; R64 `NORMALIZATION-LEAD.md`; R96 `CM.md`
and failed native log;
[Singular manual](https://www.singular.uni-kl.de/index.php/singular.pdf)
for `resultant` and `factorize`; [Stacks integral closure](https://stacks.math.columbia.edu/tag/00GH)
for the distinction between a monogenic order and its normalization.

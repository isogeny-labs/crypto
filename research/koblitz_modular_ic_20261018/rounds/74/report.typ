#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R74: all six degree-five local factors
The complete $f_C$ fiber of the normal rank-32 order is split by absolute Frobenius-fixed idempotents. A six-dimensional fixed basis generates all 63 nonzero binary combinations. Each passes direct squaring and a full 528-product multiplication-operator identity. The complete Boolean order test identifies six primitive idempotents, checks pairwise orthogonality and reconstructs the source unit.

#image("local74.svg", width: 100%)

Each local operator rank gives the full fiber length; the projected relative Frobenius rank gives the relative residue degree. Their ratios yield indices $1,1,1,7,7,1$. The local lengths $2,14,1,7,7,1$ sum to 32, and the residue-degree multiset matches R73. The absolute degree-70 branch is unramified. The two index-seven degree-five branches contribute $6+6=12$, exactly R65's $f_C$ discriminant exponent. R72 found another tame index-seven pair above $z=1$, a concrete pattern to test for an intermediate degree-seven field.

The first native attempt is rejected despite exit 0 and a late marker because Singular rejected an `intmat` increment. Fresh version two passes exit, exact marker, zero-error gate, full idempotent tests and all rank/different checks. Source and raw attempt hashes are retained. Uniformizers, coordinate and denominator valuations, and infinity remain before pole divisors and the level-77 map search.

# R97 complete finite discriminant locus of the marked-23 base order

The single frozen producer `discriminant97-01` passed with native exit
zero, no `?` errors and the exact pass marker. It bound R52's actual
monic degree-7 `f23(x,y)`, checked the reductions

    f23(0,y)=y⁴(y+1)³,      f23(1,y)=y³(y+1)⁴,

and computed the nonzero resultant

    Res_y(f23, ∂f23/∂y)
      = x²² (x+1)¹⁴ (x⁷+x⁶+x³+x+1)².

The three factors are the F2[x] factors returned by Singular's
`factorize`; their degree contributions are 22,14,14 and sum to 50.
The producer reconstructed the full factor product, checked the
resultant is y-free, and rejected a changed product. Explicit weight
vectors `(1,0)` and `(0,1)` govern all x/y degree calls. The actual
resultant polynomial and factor list are exported in the raw archive.

The first fresh-readback replay is **rejected**: its identifier collision
caused `?` errors before a late pass marker. `REPLAY-AMENDMENT.md` and
`replay-fresh97.log` preserve it. The corrected replay source was pushed
at `346c067c7` before execution; on the same fresh S3 readback it
rebound the actual donor, recomputed derivative/resultant, both special
fibers, all factor degrees and the full product, and passed with exit
zero, no `?` errors and `REPLAY97_CORRECTED_PASS`.

The deterministic 16-member raw archive SHA-256 is
`dc76c6be7fac08a4149c7dd35b96e9ac6510a12f70443f5c0f499a6c2aebf2e5`.
Checksum-protected upload to `setec-astronomy-artifacts`, fresh GET,
byte comparison and all member hashes passed. R96's earlier failed
producer remains separate and was never used as an R97 input.

This computes the **finite non-etale locus of the monogenic degree-7
base order**: x=0, x=1 and the closed degree-7 point cut out by
`x⁷+x⁶+x³+x+1`. It does not determine normalized branches or
ramification indices. Nor is it the discriminant locus of the full
degree-112 source: the degree-16 lambda extension can add fibers.
The next exact construction should compute the lambda derivative norm
over the actual degree-7 base and combine its support with these three
fibers, then normalize all resulting finite branches and infinity.

Sources checked: R52 actual marked-23 donor and `CM.md`, R63 degree-112
field certificate, R64 normalization lead, the
[Singular manual](https://www.singular.uni-kl.de/index.php/singular.pdf)
for resultant/factorize, and [Stacks integral closure](https://stacks.math.columbia.edu/tag/00GH).

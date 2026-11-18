# R74: all degree-five local lengths and ramification indices

The accepted `degree-five-local74-02` splits the entire `f_C` fiber of
the R70-normal rank-32 order. The R73 relative Frobenius-fixed kernel
has dimension six over `F32`. Absolute field traces of its six basis
columns and five scalar multiples give 30 checked candidates; six
independent absolute fixed columns generate all **63 nonzero** binary
idempotents. Each is verified by absolute squaring and by its full
32-by-32 multiplication operator from the specialized 528-product
table. The complete Boolean order test picks six primitive
idempotents, which are pairwise orthogonal and sum to the source unit.

| Primitive mask | Relative residue degree `f` | Absolute `F2` degree | Local length `e f` | Ramification `e` |
|---:|---:|---:|---:|---:|
| 14 | 2 | 10 | 2 | 1 |
| 15 | 14 | 70 | 14 | 1 |
| 17 | 1 | 5 | 1 | 1 |
| 22 | 1 | 5 | 7 | 7 |
| 48 | 1 | 5 | 7 | 7 |
| 52 | 1 | 5 | 1 | 1 |

The six local lengths sum to 32; the relative residue degrees reproduce
R73's `1,1,1,1,2,14` multiset. All indices are odd, as R73's
different argument predicted. The weighted tame contribution is
`1*(7-1)+1*(7-1)=12`, exactly the `f_C` discriminant exponent.
The absolute degree-70 branch is explicitly unramified. The repeated
pair of index-seven branches at both `z=1` and `f_C` is a concrete
structural lead for the level-77 quotient: the two loci share the
same tame index, with different residue-degree patterns. This is a
ramification observation, not yet an explicit degree-seven quotient.

Attempt 01 is rejected despite exit 0 and a late marker: Singular
rejected incrementing an `intmat` entry, so final counts did not run.
Version 2 uses explicit assignment and tested integer `div` syntax.
Attempt 02 passes exit 0, exact marker, zero-error gate and all
source/unit, full-idempotent, orthogonality, rank and different checks.
Both historical attempts and their source bytes are retained.

Uniformizers and valuations of `b,c,lambda,nu` and the registered
denominators, plus the normal infinity chart, remain the next inputs
to actual pole divisors. Full quotient saturation, polarized embedding
and order-11 action remain separate obligations.

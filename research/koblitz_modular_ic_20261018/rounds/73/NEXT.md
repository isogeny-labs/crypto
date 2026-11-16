# R74 degree-five idempotents and local lengths

The accepted R73 relative Frobenius matrix `M=F^5` has six-dimensional
`F32` fixed space, one copy of `F32` per reduced branch. Compute its
complete kernel `ker(M-I)` in the full 32-dimensional `F32` fiber and
verify all six columns. For each fixed column and each scalar basis
`1,a,a²,a³,a⁴`, take the **absolute** field trace
`T(v)=v+F(v)+F²(v)+F³(v)+F⁴(v)`. These 30 candidates lie in the
absolute Frobenius-fixed space `F2^6`. Extract six independent columns
and enumerate all 63 nonzero binary combinations. Recheck `x²=x`
directly and use the full specialized 528-product table to form every
32-by-32 multiplication operator.

Apply the complete Boolean order test to obtain six primitive
idempotents, verify pairwise orthogonality and source-unit sum, and
measure each local multiplication rank. The rank of `M` projected to a
primitive factor gives its relative residue degree; require the six
degrees to match `1,1,1,1,2,14`, then divide local length by residue
degree for the ramification index. Check `sum e_i f_i=32`, all indices
odd, and the exact different budget 12. The degree-14 branch should
have length 14 by the R73 bound; a different result is a failure to
investigate, not a value to coerce.

After that, find local uniformizers and valuations at `f_C` and the six
rational branches, construct the normal infinity chart, and assemble
actual pole divisors. The complete prescribed Riemann--Roch space and
order-11 action test then become concrete.

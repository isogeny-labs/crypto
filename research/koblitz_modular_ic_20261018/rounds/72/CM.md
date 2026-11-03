# R72: exact rational-branch ramification

The accepted `rational-local72-01` enumerates **all** nonzero
Frobenius-fixed idempotents of the two full 32-dimensional fibers of
the R70-normal affine order. For each, it uses the complete specialized
528-product table to build a 32-by-32 multiplication operator. The
complete Boolean order test picks precisely the primitive idempotents;
they are pairwise orthogonal and sum to the actual source unit.

| Fiber | Primitive mask | Residue degree `f` | Local length `e f` | Ramification `e` |
|---|---:|---:|---:|---:|
| `z=0` | 1 | 2 | 28 | 14 |
| `z=0` | 3 | 2 | 4 | 2 |
| `z=1` | 6 | 2 | 2 | 1 |
| `z=1` | 7 | 2 | 14 | 7 |
| `z=1` | 8 | 2 | 2 | 1 |
| `z=1` | 10 | 2 | 14 | 7 |

At each fiber the local lengths sum to 32 and `sum e*f=32`. The
derivation `e=local length/f` uses the established normality of the
finite affine order at these primes: its localizations are DVRs. R71
separately determined `f=2` for every branch. This gives the first
exact local branch profile of the affine level-77 source.

There is a useful separate discriminant consistency check. R65's
trace Gram has exponent 24 at `z+1`. The two ramified `e=7` branches
are tame in characteristic two, so their different contributions are
`2*(7-1)` each; together they give exactly 24. At `z=0`, the
discriminant exponent is 76 while `e=14,2` are wild; the tame lower
bound `2*((14-1)+(2-1))=28` leaves wild excess 48. This check does
not supply local uniformizers or valuations by itself.

One native attempt passed exit 0, the exact terminal marker, and the
zero-error gate. Six executed source/protocol/driver files match the
frozen Git revision and eight donor hashes match. Full square, stable
image, fixed-basis and all-idempotent matrices are saved. The next
step is exact local uniformizers and valuations of the requested
coordinates and denominators; `f_C` and infinity remain separate.

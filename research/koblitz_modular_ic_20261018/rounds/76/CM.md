# R76: complete finite zero divisors of the Tate coordinates b and c

The frozen R76 native program rebinds the R72 primitive idempotents to
the full R52 multiplication table and its integral source-lift column
for `V`. It checks the quadratic source equation in both rational
fibers and all six primitive operator ranks. Both `z=0` factors have
nonzero square-zero `V` class. At `z+1`, masks 6 and 7 have `V=1`,
while masks 8 and 10 have `V=0`. The single native attempt passed exit
0, its exact marker and zero native error lines; six frozen source-byte
pins and five donor hashes match.

The source equation and Tate functions are

```text
V^2 + z^2 V + z + z^3 = 0,
R = (1+z)V + z^3,
b = z R(R+1),       c = z(R+1).
```

For the quadratic extension `F2(z,V)/F2(z)`, direct expansion gives
`Norm(R)=z` and `Norm(R+1)=(z+1)^3`. Thus, wherever `z` is finite,
`b` and `c` can vanish only above `z=0` or `z+1`.

At `z=0`, the quadratic equation is Eisenstein: the constant term
`z+z^3` has order one and the linear coefficient has order two. Its
unique quadratic place has `v(V)=1` and `v(z)=2`. For either full-source
branch of index `e`, `v(R)=e/2` and `R+1` is a unit, hence
`v(b)=3e/2`, `v(c)=e`.

At `z+1`, the quadratic equation is etale and splits into `V=0` and
`V=1`. Write `W=V+z^2+z+1`; then `R+1=(z+1)W` and direct quadratic
norm expansion gives `Norm(W)=z+1`. On the `V=0` quadratic place,
`W` is a unit, so `v(b)=v(c)=e`; on the `V=1` place, `v(W)=e`, so
`v(b)=v(c)=2e`. Here `R` is a unit on every branch.

| Prime | Mask | Absolute residue degree | `e` | `V` | `v(b)` | `v(c)` |
|---|---:|---:|---:|---|---:|---:|
| `z` | 1 | 2 | 14 | square-zero | 21 | 14 |
| `z` | 3 | 2 | 2 | square-zero | 3 | 2 |
| `z+1` | 6 | 2 | 1 | 1 | 2 | 2 |
| `z+1` | 7 | 2 | 7 | 1 | 14 | 14 |
| `z+1` | 8 | 2 | 1 | 0 | 1 | 1 |
| `z+1` | 10 | 2 | 7 | 0 | 7 | 7 |

The finite zero divisor of `b` has degree 96 and that of `c` has
degree 80, weighting each row by its absolute residue degree two.
Equivalently, the full degree-32 field norms are
`Norm(b)=z^48(z+1)^48` and `Norm(c)=z^32(z+1)^48`, from the quadratic
norms and the degree-16 upper extension. These give independent
degree checks. On the proper normalization, the total pole degrees
are consequently 96 and 80; the infinity branches and individual pole
orders still require their chart.

The Lean file `lean/Quadratic76.lean` proves the two characteristic-two
norm expansions and the weighted degree arithmetic. Its pinned archival
compile and axiom audit passed: three authored theorems, zero failing
and zero trusted. One earlier launcher attempt stopped before compilation
on a wrong copied filename; its partial raw directory and exact failure
receipt are retained. Lean does not prove the source equation, primitive
mask assignment, or local valuation-theoretic bridge; those are tied to
the native source checks and the arguments above.

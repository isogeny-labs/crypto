# R70: the rank-32 affine source order is normal

The accepted `sixth-frobenius70-02` completes the descending Frobenius
calculation on the actual degree-32 source order `O` over `A=F2[z]`.
The exact quotient lengths are

| Lattice | `dim_F2(V_i/O)` | Complete square-obstruction rank |
|---|---:|---:|
| `V0=O^vee` | 160 | 80 |
| `V1` | 80 | 48 |
| `V2` | 32 | 24 |
| `V3` | 8 | 4 |
| `V4` | 4 | 2 |
| `V5` | 2 | 2 |
| `V6=O` | 0 | stable by old-order multiplication |

The last complete map has 24 binary denominator-remainder rows and two
columns. Its first two rows are exactly `(1,0)` and `(1,1)`, so their
determinant is one and the full map has zero kernel. The accepted run
reconstructs every remainder, checks rank two, saves the complete map,
and explicitly verifies those witness rows. It identifies the sixth
lattice with the old order by a 32-by-32 field-coordinate identity,
checks all 32 old-order basis squares in `O` and `V5`, both inverse
products, polynomial inclusion, and a changed-inverse negative control.
The terminal marker, native exit 0 and zero Singular question-mark error
lines all pass. The order is square-stable, so `V7=V6=O`.

Here is the normalization argument at its exact scope. R50/R52 supply a
finite free rank-32 `A`-order with all 528 multiplication columns inside
the identified source field; R64 verifies the coordinate bridge and all
32 basis images in the actual degree-32 field. R61/R65 give the
nondegenerate trace Gram `G` and `V0=G^-1 A^32`. Any element integral over
`A`, multiplied by an element of `O`, is integral; its field trace lies
in integrally closed `A`. Hence the integral closure `C` lies in `V0`.
The checked adjugate relation also gives `(det G)V0 subset O`.
Since `C` is closed under squaring, induction gives `C subset V_i` at
each complete step. The zero kernel makes `V6=O`, so `C subset O`;
the reverse inclusion is automatic. Thus **this affine order `O` is the
integral closure of `A` in its identified degree-32 function field**.
This argument uses the finite chain directly; it does not require an
unverified stopping heuristic. It is consistent with the
[Singh--Swanson descending Frobenius theorem](https://www.math.purdue.edu/~iswanso/intclSinghSwanson.pdf),
whose fixed-point criterion is more general.

The Lean file `lean/Frobenius70.lean` kernel-checks the generic
two-row minor implication and its `(1,0),(1,1)` instance, iterated
square closure, and the conditional six-step normalization implication.
The accepted `frobenius-lean70-03` compiled and audited 12 declarations
with zero failing and zero trusted; six are authored claim theorems.
The native matrix entries, actual source-field identification, trace
integrality and `V6=O` are explicit inputs to that Lean implication,
not proved by Lean's kernel.

Two failures remain in the exact record. `sixth-frobenius70-01` printed
a late marker but is rejected for Singular zero-column errors after
finding rank two. Fresh version 2 handles the zero-kernel case with an
explicit minor and old-order identity. `frobenius-lean70-01` failed
compilation on a missing `ZMod 2` field instance and an un-applied set
inclusion; version 2 made the row lemma field-generic and compiled/audited,
then version 3 added the direct six-step theorem and independently
compiled/audited. `results.json` checks all five historical source sets.

This certifies the **affine** chart. The next concrete geometry is to
factor the omitted `z=0`, `z=1`, `f_C=z^5+z^3+z^2+z+1` fibers in this
normal order, compute local residue fields and valuations of
`b,c,lambda,nu` and every registered denominator, and add the infinity
chart. Those exact divisors determine a complete prescribed
Riemann--Roch space for the level-77 map and its order-11 action.
The proper polarized model, projective saturation, node187 coupled
divisors, arbitrary reverse transfer and larger source/factor actions
remain separate tasks.

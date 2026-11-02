# R75: uniformizer classes at all twelve omitted finite branches

Two complete native programs pass exit 0, exact marker and zero-error
gates. They rebind every R72/R74 primitive idempotent to the full R52
specialized 528-product table, verifying operator idempotency, source
unit, local rank and residue degree. Four rational-fiber ramified
branches and two degree-five ramified branches each receive an
unrestricted 32-source-basis-element search: **192 candidates total**.
Every candidate `t=y^q-y` lies in its projected local factor and has
`t^e=0`. The saved first candidate for each branch has
`t^(e-1) != 0`, so its nilpotency index is exactly the ramification
index `e`.

| Base prime | Primitive mask | `e` | First source basis | Successful candidates / 32 |
|---|---:|---:|---:|---:|
| `z` | 1 | 14 | 5 | 9 |
| `z` | 3 | 2 | 1 | 16 |
| `z+1` | 7 | 7 | 5 | 11 |
| `z+1` | 10 | 7 | 17 | 10 |
| `f_C` | 22 | 7 | 1 | 26 |
| `f_C` | 48 | 7 | 1 | 25 |

The other six branches are unramified; their local operator rank
equals residue degree, and the base prime is a uniformizer. These
classes cover all twelve finite omitted branches of the normal
affine order. In a local DVR with `p=(unit)*pi^e`, a nonunit whose
`(e-1)`-st power survives modulo `p` has valuation one: if its
valuation were at least two, `(e-1)*2 >= e` for `e>=2` would force
that power to vanish. This is the mathematical bridge from the exact
nilpotency checks to uniformizer classes.

The saved coordinate columns are in the **finite fiber algebras**.
They can be lifted to order elements, but their global expressions,
valuations of `b,c,lambda,nu` and the R51/R52/R53 denominators,
leading coefficients, the infinity chart and actual pole divisors
are still to be computed. Neither a prescribed Riemann--Roch map nor
an order-11 subgroup action follows from these local classes alone.

Both attempts were frozen before data, accepted under the strict
native gate, and bound to 12 historical source/protocol/driver bytes
and nine donor hashes. Complete candidate counts, first columns and
raw logs are retained in `results.json` and the archive.

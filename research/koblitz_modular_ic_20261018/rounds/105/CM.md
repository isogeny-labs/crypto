# R105: x=1 Hasse jets and base Newton edges

Set `t=x+1` and `q=l^2+l+1`. The exact R51 donor gives

    G51(1+t,y,l) = q^8 + t^4 y^3(y+1)^3 l(l+1)q + O(t^5).

The coefficients of `t,t^2,t^3` vanish as full polynomials in `(y,l)`,
not merely after setting y=0 or 1. The displayed t4 coefficient has
eight monomials: lambda exponents 1 and 4 crossed with y exponents
3,4,5,6. Both endpoint projections at y=0 and y=1 vanish through t4.
This high-order contact is a useful constraint for the next local
normalization; mixed y dependence must be retained when expanding along
branches.

The full polynomial `f51(1+t,y)` has smallest t-orders by y-degree 0..7:

    y near 0:  [2,4,1,0,1,1,2,0]
    z=y+1:    [3,3,2,1,0,0,0,0].

Thus the positive-slope Newton edges in the two clusters join `(0,2)`
to `(3,0)` and `(0,3)` to `(4,0)` respectively (coordinates are
`(y-degree,t-order)`). Every intermediate point is strictly above its
edge. Since `f51(1,y)=y^3(y+1)^4`, Hensel separates degree-3 and degree-4
local factors over F2((t)); the one-edge coprime-slope criterion makes
each factor irreducible. **For the degree-7 base field**, there is one
normalized place above each y cluster, with t-ramification 3 and 4 and
residue field F2. This does not count branches of the degree-112 lambda
extension.

Attempt 01 omitted the full mixed G jets despite a zero exit; it is
retained and rejected as protocol-incomplete. Amendment 02 was committed
before data. The accepted source-bound run exited zero, a fresh process
matched `result.json` byte-for-byte, and an altered donor hash failed.
An independent bit-polynomial expansion recomputed all f supports and
all five G jet supports from the parsed source terms. Lean checks the
eight-term mixed factor and numerical Newton-edge/coprimality premises;
the Hensel/Newton theorem is a cited mathematical argument, not a Lean
formalization of the complete branch statement.

Raw attempts and replay: `raw-jets105-02.tar.xz`, SHA-256
`bbb4005d3a3a747f3ff15b8d887e1c905d4a40d8d19856f135cbf6d5d178933c`.
The exact result JSON has SHA-256
`196593c612e43fe185041373bc481abf4c3a0556a3504b8b0327f26b70ec7ffd`.
Next compute the first mixed terms after substituting local base uniformizers
and then normalize the lambda extension over each base completion.

Sources: R51 donor, R101 fiber, R104 leading residue,
[Newton polygon theorem 4.43](https://www.math.columbia.edu/~gyujinoh/ANT_revised.pdf),
[Stacks normalization](https://stacks.math.columbia.edu/tag/0C44), and
[Stacks branches](https://stacks.math.columbia.edu/tag/0C1S).

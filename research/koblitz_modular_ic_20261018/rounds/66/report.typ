#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R66: second Frobenius lattice step
All 80 classes of $V_1/O$ are squared with the actual degree-32 source
multiplication table. Complete denominator clearing gives 229 binary
remainder rows of rank 48, hence 32 square survivors. Every remainder
is reconstructed, every kernel product vanishes, and an altered kernel
is rejected.

#image("frobenius66.svg", width: 100%)

The full second PID lattice has rank 32. The quotient $V_2/O$ has binary
dimension 32 and $V_1/V_2$ has dimension 48. Both module inclusions,
both field-basis inverse products, old-order inclusion, and all 32 new
basis squares back in $V_1$ pass. An altered inverse is rejected.

== Execution and evidence
One native attempt finished with exit 0, the exact success marker, and
no Singular error lines. Its UTC wall interval is 194 s, a receipt rather
than a controlled performance comparison. Six historical source byte pins
and six donor hashes are checked in results.json. The complete quotient,
binary kernel, and polynomial/field lattice bundles are saved.

This is the second descending step. The next calculation tests all 32
classes of $V_2/O$ and continues until the lattice stabilizes. Actual
proper branches and divisors follow that affine normalization.

The method follows
#link("https://www.math.purdue.edu/~iswanso/intclSinghSwanson.pdf")[Singh–Swanson, Theorem 1.1].

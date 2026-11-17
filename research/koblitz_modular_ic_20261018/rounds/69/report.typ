#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R69: fifth Frobenius lattice step
All four classes of $V_4/O$ are squared with the actual degree-32 source
multiplication table. Complete denominator clearing gives 25 binary
remainder rows of rank two, hence two square survivors. Every remainder
is reconstructed, every kernel product vanishes, and an altered kernel
is rejected.

#image("frobenius69.svg", width: 100%)

The full fifth PID lattice has rank 32. The quotient $V_5/O$ has binary
dimension two and $V_4/V_5$ has dimension two. Both module inclusions,
both field-basis inverse products, old-order inclusion, and all 32 new
basis squares back in $V_4$ pass. An altered inverse is rejected.

== Execution and evidence
One native attempt finished with exit 0, the exact success marker, and
no Singular error lines. Its UTC wall interval is 171 s, a receipt rather
than a controlled performance comparison. Six historical source byte pins
and six donor hashes are checked in results.json. The complete quotient,
binary kernel, and polynomial/field lattice bundles are saved.

This is the fifth descending step. The next calculation tests both
classes of $V_5/O$ and continues until the lattice stabilizes. Actual
proper branches and divisors follow that affine normalization.

The method follows
#link("https://www.math.purdue.edu/~iswanso/intclSinghSwanson.pdf")[Singh–Swanson, Theorem 1.1].

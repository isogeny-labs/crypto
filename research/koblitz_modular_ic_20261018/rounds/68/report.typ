#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R68: fourth Frobenius lattice step
All eight classes of $V_3/O$ are squared with the actual degree-32 source
multiplication table. Complete denominator clearing gives 25 binary
remainder rows of rank four, hence four square survivors. Every remainder
is reconstructed, every kernel product vanishes, and an altered kernel
is rejected.

#image("frobenius68.svg", width: 100%)

The full fourth PID lattice has rank 32. The quotient $V_4/O$ has binary
dimension four and $V_3/V_4$ has dimension four. Both module inclusions,
both field-basis inverse products, old-order inclusion, and all 32 new
basis squares back in $V_3$ pass. An altered inverse is rejected.

== Execution and evidence
One native attempt finished with exit 0, the exact success marker, and
no Singular error lines. Its UTC wall interval is 176 s, a receipt rather
than a controlled performance comparison. Six historical source byte pins
and six donor hashes are checked in results.json. The complete quotient,
binary kernel, and polynomial/field lattice bundles are saved.

This is the fourth descending step. The next calculation tests all four
classes of $V_4/O$ and continues until the lattice stabilizes. Actual
proper branches and divisors follow that affine normalization.

The method follows
#link("https://www.math.purdue.edu/~iswanso/intclSinghSwanson.pdf")[Singh–Swanson, Theorem 1.1].

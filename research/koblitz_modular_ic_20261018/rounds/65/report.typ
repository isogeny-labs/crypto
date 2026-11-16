#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R65: first Frobenius lattice step
The complete trace-dual quotient $V_0/O$ has binary dimension 160. The first
native run squares every class using the full rank-32 source multiplication
table. Complete denominator remainders produce 241 binary rows of rank 80,
leaving 80 survivors. Every remainder, kernel product, and whole-field
survivor is checked. An altered kernel is rejected.

#image("frobenius65.svg", width: 100%)

The second native run reconstructs $V_1$ as a rank-32 polynomial lattice.
It checks both module inclusions, both basis inverse products, all 32
basis squares back in $V_0$, and an altered-inverse negative control.
The exact binary quotient lengths are 80 for $V_1/O$ and 80 for
$V_0/V_1$. This is one descending step; equality with the
integral closure requires iteration to a stable lattice.

== Execution and evidence
Both attempts finished with native exit 0, exact markers, and zero Singular
error lines. The first and second UTC wall intervals are 76 s and 187 s,
respectively; these are receipts, not controlled performance comparisons.
Twelve historical source/protocol/driver byte pins and the saved quotient,
kernel, survivor, and lattice bundles are recorded in results.json.
The input field bridge is R64, while the Gram and multiplication table are
the retained R61 and R52 sources. The next full test is $V_2={x in V_1:x^2 in V_1}$
on every class of $V_1/O$.

The normalization method follows
#link("https://www.math.purdue.edu/~iswanso/intclSinghSwanson.pdf")[Singh–Swanson, Theorem 1.1].

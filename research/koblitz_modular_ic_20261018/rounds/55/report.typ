#set page(paper:"a4",margin:16mm)
#set text(size:10pt)
= R55: complete A-zero source quotient and degree-80 CRT
The four binary degree-20 points now exhaust the A=0 scheme in the actual
normal affine source. Full module products identify the quotient, all source
functions, and every quotient class. The complete pole divisor remains a
separate projective geometry calculation.

== Full annihilator on the actual source
B is the retained normal rank-32 RR algebra over F2[z]. All 528 multiplication
columns bind the unit and actual V coordinate. With V²+z²V+z+z³=0, set
#raw("R=(1+z)V+z^3; A=1+z(R+1); fA=z^5+z^2+1",block:true)
The complete 32-by-32 multiplication matrices satisfy
#raw("C=M_A+(z^3+z^4)I; M_A C=C M_A=fA I32",block:true)
Thus fA B is contained in A B. Reduction modulo fA adds no extra constraint
to B/A B. A changed conjugate matrix is detected. Source: quotient55-02.
#image("construction55.svg",width:100%)

== Exact kernel and recovery products
B/fA B has 160 binary coordinates. Abar has rank 80 and E has rank 80.
The retained arrays give all three exact products
#raw("E Abar=0; E G=I80; Abar H=I160-G E",block:true)
The last product proves ker(E)=image(Abar) for every source class, while the
second proves surjectivity. Consequently B/A B is exactly the proposed
80-dimensional algebra. This is a full source quotient identification,
not just the rank of a sample or a determinant prediction.
#image("quotient55.svg",width:100%)
All plotted dimensions are exact native data, without sampling uncertainty.
Source: quotient55-02. The point degrees are the four retained R54 fields.
#pagebreak()
= Complete binary functions, proofs and retained failures

== Four fields and every source function
The product N of the four degree-20 moduli has degree 80 and is squarefree.
Four full cofactor/Bezout products construct CRT idempotents; all 16
orthogonality products, all 16 residue labels and partition of unity pass.
CRT recovers z=rho(w) and all 32 normal RR basis functions. The recovered
base coordinate satisfies fA(rho)=0. The actual unit and all 528 source
multiplication identities pass modulo N.

Thus B/A B is the product of four degree-20 fields. Its four closed points
have local length one and total binary length 80 in this normal affine chart.
The fresh serialized checker reconstructs every binary and evaluation entry,
all recovery products, all 128 point-value labels and all 528 source products.
An altered recovery matrix is rejected. Source: check55-02.

== Lean claim fidelity
Four authored implications compile and pass the separate axiom audit:
surjectivity from E G=I, exact kernel/image recovery, submodule equality, and
scalar multiples lying in image A from A C=f I. Zero failing and zero trusted
declarations. Actual matrix arrays, polynomial arithmetic, irreducible fields
and retained source normalization remain external premises. See PROOF_MAP.md.

== Complete workflow ledger
#table(columns:(1.2fr,.7fr,1.7fr),inset:5pt,[#text("Workflow")],[#text("Status")],[#text("Receipt")],[#text("check55-01")],[#text("rejected")],[#text("exit 0; 12 error lines")],[#text("check55-02")],[#text("accepted")],[#text("exit 0; 0 error lines")],[#text("diag55-01")],[#text("accepted")],[#text("exit 0; 0 error lines")],[#text("quotient-lean55-01")],[#text("accepted")],[#text("compile 0; audit 0; four declarations")],[#text("quotient55-01")],[#text("rejected")],[#text("exit 0; 359 error lines")],[#text("quotient55-02")],[#text("accepted")],[#text("exit 0; 0 error lines")])

The first quotient attempt is rejected: a combined native scalar/matrix
expression produced a 32-by-64 matrix and failed the base identity. Later
undefined objects and a late marker are excluded. A separately frozen
shape diagnostic reproduced the issue. The first checker attempt also retained
a ring-local variable declaration failure; its output is rejected. Entrywise scalar construction passes
the original formulas and checks. No failed output supports the quotient.

Native limits: 4 GiB virtual, 1,200 seconds, one Singular/FLINT thread, seed 55.
Lean 4.34.1 / mathlib d13f23b, -j1/-M4096, separate 1,200-second compile/audit
caps, no virtual cap. Historical executed bytes are checked against Git and
drivers execute immutable copies. No accepted producer is rerun for custody.
CPU times were not collected; no comparative performance claim is made.

== Next geometry and preserved lanes
Determine all infinity branches, residue fields, ramification and A pole
orders, then bind this affine zero scheme to the complete projective divisor.
Use the degree-80 evaluator for an entire finite modular-map control before
generic target identities. Preserve the actual degree-8/128 level-11 map and
intended-11 action, full incidence saturation, proper polarization, node187,
arbitrary reverse classes and the larger degree-112/991 source lane.

Sources: R50/R51 full RR algebra, R52 PID embedding, R53 base geometry,
R54 four point fields, Stacks00DT CRT, native linalg.lib. See CM.md,
protocol.json, results.json and PROOF_MAP.md for exact premises and receipts.

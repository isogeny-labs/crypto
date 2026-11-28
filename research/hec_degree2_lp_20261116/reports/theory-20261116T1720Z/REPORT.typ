#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Quadratic collisions as pole-six relations
16 November 2026 · hec_degree2_lp_20261116 · post-freeze derivation, unexecuted gate

== Explicit identity
Normalize two signed anchor-triple interpolants so they have the same oriented nonbranch irreducible residual E=(Q,v). With f−h1²=U1Q and f−h2²=U2Q, the degree bounds force h1−h2=cQ for a constant c. If c is zero, the collision row is zero. Otherwise a=(f−h1h2)/Q is a monic cubic and direct expansion gives

$a^2-c^2 f=U_1 U_2.$

The function-field identity a−cy=(y−h1)(y+h2)/Q gives the signed six-point relation T1−T2. Its anchor multiplicities are at most two. This is a consequence of classical divisor algebra, with no priority claim.

#image("../../POLE6-BRIDGE.svg",width:100%)

== Restricted span equality
Assume odd rational Jacobian order, nonbranch anchors with distinct abscissae and both signs available. Let R5 contain every rational-split triple norm row, including off-anchor rational residual points. Let C2 contain irreducible quadratic-label collision rows. Let R6 contain primitive cubic/constant-y functions whose six zeros lie in the signed anchor set with multiplicity at most two.

The forward identity gives C2 inside R6. Conversely partition the six zeros into two triples with distinct abscissae in each, putting repeated copies on opposite sides. Conjugate the second triple. Their classes agree; uniqueness of the reduced Mumford representative gives a common quadratic residual. If it splits, the six-point row is a difference of two R5 rows. If irreducible, it is a C2 pair difference. Odd order excludes a nonzero two-torsion residual. Hence span(R5,C2)=span(R5,R6). Complete proof/domain restrictions are in POLE6-BRIDGE.md.

#pagebreak()
== Enumeration consequence and the comparison boundary
The existing genus-two d=1 family uses a cubic a and constant b at four anchors. Exhaustive direct interpolation covers the restricted R6 family. Its scheduled sign-class count is8binom(H,4). Triple norms require4binom(H,3), and already occur in the rational baseline. The total direct-augmentation count divided by the triple count is (H−1)/2.

Thus a cache can reindex this matched restricted row-span augmentation from orderH⁴candidate enumeration to orderH³plus dictionary/output work. This is an algebraic schedule comparison, not measured time, an unconditional complexity lower bound or the speedup over Compact. Compact already uses triple candidates; it does not pay the direct quartet enumeration. Extra rank can be zero, and source/target solving, storage, factor tests and LA remain costs.

The rank budget N−L+Z and conditional quadratic-pool warning still apply. Sparse label collisions may make this augmentation useless at the sizes needed for the full objective. No numerical rank, scalar or tenfold result is established by this derivation.

== Domain and general identity
Higher-contact zeros with multiplicity greater than two, even-order torsion exceptions and unexamined confluent cases are outside the genus-two span theorem. Target-free global signs are handled by whole-row negation; a fixed target would need separate treatment. For general g, the same algebra with a common degree-g reduced residual yields deg(a)=g+1 and a²−c²f=U1U2. A general span theorem needs additional hypotheses and is not asserted here. A composite residual class is not automatically a prime endpoint.

== Current execution state and next validation
The frozen native quadratic-rank gate remains unexecuted. Its guarded resume was retried at17:19Z: checkpoint upload still returned AllAccessDisabled, so the script exited before producing scientific outputs. The compiled original gate is unchanged; no checkpoint requirement is waived. An approved replacement destination has not been supplied.

Once access is restored, run the original gate once. A separately frozen extension should enumerate direct pole-six rows filtered to this domain, compare its augmented span with the cache span, and verify the displayed norm/divisor identities and signs. The current gate does not perform that bridge comparison, and no finite validation is claimed now.

== Sources and status
Current rational.rs and RATIONAL-PROTOCOL.md confirm the existing g=2,d=1 degree bounds and four-anchor construction. Compact/native Jacobian code fixes the triple conventions. KB query and prior-art gaps are recorded in POLE6-BRIDGE.md; the2011group-law PDF fetch returned403. The accessible2005LPpaper was used for background, not attributed this span result. This is a written deduction, not a machine-checked theorem or independent scientific validation.

No scientific outputs, timing benchmark, benchmark admission or performance promotion. Both report pages visually inspected. Original pre-run source/protocol/theory remain untouched; this derivation is a separate post-freeze artifact. PR4357 remains unmerged. The full mathematical tenfold objective stays active.

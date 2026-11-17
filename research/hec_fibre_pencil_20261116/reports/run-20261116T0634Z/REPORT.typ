#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Root-free divisor pencils and exact signed rank
16 November 2026 · hec_fibre_pencil_20261116 · finite algebra/count controls

== Construction and identity
For a full-degree, squarefree, nonbranch-support divisor $(u,v)$ on $y^2=f(x)$, form $h_t=v+t u$ and $Q_t=(f-h_t^2)/u$, monic of degree $g+1$. Bucket rational points by $(y-v)/u$ where $u != 0$. A regular full fibre gives an effective divisor $E_t$ with $[D]+[E_t]=0$ in degree-zero notation. Every point in that fibre gives a successful Cantor point-add transition, but all $g+1$ transitions yield the same decomposition of D.

Identifying conjugate sheets and omitting branch columns in an odd-prime projection turns the fibre matrix into a grounded graph incidence matrix. For F full fibres and c ungrounded components, the difference-row rank is F−1−c+delta, where delta is one precisely when some closed-component size is nonzero modulo the coefficient prime. The proof uses the component-indicator basis of the left kernel. Disjoint oriented supports alone do not prove signed rank. These are classical divisor and graph identities; no novelty claim.

== Frozen gate and exact counters
Scientific source 066cc22aa56e; lock correction de08f9286c6f; pre-run S3 checkpoint 20261116T063449Z. Once-only native control exits zero, with empty stderr. Fixed genus2/F101/c1, genus3/F61/c1 and genus4/F31/c1. Twenty-four multiples [17..40]P per curve; three input exclusions retained. Sixty-nine inputs meet the hypotheses. All regular fibre/root comparisons, group identities, point-add residuals, decomposition equalities and rank checks pass.

#table(columns:7,inset:4pt,
[g / p],[Inputs],[Scanned],[Exceptions],[Fibres],[Transitions],[Rank sum],
[2 / 101],[24],[2740],[43],[487],[1461],[463],
[3 / 61],[23],[1555],[38],[70],[280],[47],
[4 / 31],[22],[716],[48],[5],[25],[0])

Exact deterministic sums per genus; rank sums are not pooled matrix rank. Native Rust reduces raw counters. No timing, statistical uncertainty or held-out population inference. Full-fibre ranges per input are 17–24, 1–6, and 0–1 respectively. All tested components are grounded, so finite rank checks exercise c=0; the general formula rests on the written proof. Modulus1000003 is an algebra-control prime.

#image("../../COUNTS.svg",width:100%)

#pagebreak()
== Complete-cost decision
The sweep must pay for catalogue discovery, evaluations, inverses, storage, exceptional cases and the resulting linear algebra. Its g+1 successful transitions per full fibre are duplicate decompositions. A uniform-fibre estimate p/(g+1)! is heuristic and is not used as a theorem or gain estimate. The genus4 panel has only five full fibres across22eligible inputs, giving no within-pencil difference rank. No new elapsed-time run follows from this screen.

The construction is root-free, yet it uses the full rational catalogue. Compact's existing target-seeded solver uses a smaller anchor system. A useful successor needs a constructive way to obtain independent anchor-supported constraints from the incidence structure, with complete setup and target descent charged. The present rank law gives an exact test for such a construction. There is no tenfold complete-cost premise across the requested genera from this sweep. The objective remains open.

== Evidence boundary and reproducibility
Raw: CONTROL.tsv, stderr and EXIT.txt. Frozen protocol: PROTOCOL.md. Assumptions, proof and literature receipt: THEORY.md. FREEZE.sha256 covers scientific source, lock, imported native sources and binary before execution; dependency-lock correction is committed before the S3 checkpoint gate. Post-run reduce.rs reads the fixed raw counters, writes SUMMARY.tsv and renders COUNTS.svg; it does not select or rerun inputs. Source and PDF accompany PR4357, which remains unmerged.

Three excluded inputs, all exceptional parameters, and every zero-fibre case remain visible. Nonsquarefree residuals and support-overlap parameters are excluded from the implemented simple-fibre gate, not relabelled as nonsplit. Written multiplicity identities are broader than this finite check. Shared field/Jacobian arithmetic remains a dependency. No independent validation, machine-checked full group certificate, benchmark-index admission or performance claim is supplied.

== Sources checked and evidence gap
KB revision9141e386254340a9bdba3036f2bb706a, refreshed14November21:44Z, lexical-only snapshot predates these studies. Queries cover pencils, Nagao/Riemann–Roch, compact anchors and large-prime trees. Metadata/context source:b3c90d39a347ef661ccf10680a742186 warns against uncharged oracle/matrix work. The current anchor.rs, compact.rs and complete direct-profile results are checked directly. Nagao/GTTD literature pointers identify classical prior art; they do not support priority claims.

Joux–Vitse's EUROCRYPT2012 cover/decomposition paper is the primary-source pointer for related extension-field Riemann–Roch sieving. Author PDF timed out, publisher tool access failed and eprint fetch returned403. This access gap is retained; the algebra here is derived explicitly and checked against current group arithmetic. No absence-of-prior-art conclusion is inferred. Existing performance leaderboards and sealed timing graphs are unchanged.

== Post-control constructive anchor elimination
Using only off-anchor columns gives k ungrounded components, with size-functional rank delta_O. The anchor-supported intersection dimension is (k−delta_O)−(c−delta). Sum rows inside each such component, retain anchor coordinates, then combine these component sums to enforce zero total row coefficient. This constructs the full anchor intersection without dense elimination on all catalogue columns. Rank-nullity proves the dimension; THEORY.md gives the basis. This post-control derivation is not exercised by the frozen gate. Its next required check is independent full/off Gaussian ranks and row containment under a new protocol. It does not guarantee positive rank or make the scan/setup costs disappear.

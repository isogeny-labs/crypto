#set page(paper:"a4",margin:15mm)
#set text(font:"Libertinus Serif",size:10pt)
#set par(leading:0.5em)
#align(center)[
 #text(size:18pt,weight:"bold")[Residual-corrected closed-loop action]
 #linebreak()
 Original marked scalar; P-192 primary; five-curve exact census
 #linebreak()
 2026-11-08 · T-1888 · PR 3674
]
= Executable point action
Keep the original $mu$ and write its centered representative as $mu_0=m k+c$. At relation radix $m=ell$, the checked closure supplies $Q=sigma eta phi(P)=[ell]P$, and the evaluator computes
$ [mu]P=[k]Q+[c]P. $
Both signed-digit terms share the main doubling loop. The complete fused prefix, both tables, additions and negations are charged. The separate explicit-map/endpoint checks return the same points. The global noninteger quotient remains unexpanded; the implementation asserts the displayed rational-subgroup action.

#image("flow.svg",width:100%)

= Frozen primary comparison
#table(columns:(1fr,1fr,1fr,1fr),inset:5pt,
 [*Curve*],[*Direct*],[*Binary split*],[*Relation split*],
 [P-192],[228],[229],[232],
 [P-224],[260],[260],[267],
 [P-256],[300],[300],[304],
 [P-384],[449],[450],[451],
 [P-521],[606],[606],[612],
)
Values are minimum unweighted D+A over the declared finite design. The relation is above both controls on every curve, including each declared weight2..4 sensitivity. This is the preregistered comparison; no runtime inference follows.

#image("counts.svg",width:100%)

#pagebreak()
= Construction and qualification
For each radix, the centered Euclidean remainder $r$ fixes $c=r+m v$, $k=(mu_0-c)/m$. Every integer offset $-64<=v<=64$ and window2..7 is retained. Binary controls use the identical joint evaluator at radices1,2,4,8,16,32. The direct reference repeats all centered-scalar shifts -256..256 and the same six windows in this instrument. No scalar, generator, subgroup or point changes across arms.

All42,480 schedules completed once. Independent GMP replay reconstructs every row and20minimum sets without including the BIGNUM producer. All300 selected-plan point/count checks pass. Earlier software qualification passes3150 point/count cases,450 full explicit-map/closing-isomorphism cases and five wrong-remainder controls. Five post-scan corruptions reject: wrong selected plan, wrong raw counter with updated checksum, missing row, duplicate row and changed protocol. Fresh archive replay is byte-identical; deterministic repacking matches.

= Complete selected relation vectors
#table(columns:(1fr,1fr,1fr,1fr,1fr),inset:4pt,
 [*Curve*],[*D*],[*A*],[*N*],[*Table entries*],
 [P-192],[192],[40],[19],[6],
 [P-224],[222],[45],[19],[10],
 [P-256],[252],[52],[24],[10],
 [P-384],[379],[72],[26],[18],
 [P-521],[521],[91],[33],[18],
)
For historical engineering context, the earlier translated zero-remainder recipe selected235,273,309,458,613 D+A. Its design differs; the decisive controls are those in the current table. P-192's new vector removes four doubles, adds one addition and two negations, and removes three table entries against that earlier recipe. The remaining direct-reference gap is four D+A calls on P-192 and two on P-384. Retaining useful prefix intermediates is the next source-level opportunity, with identical reuse required for binary controls.

= Publication, resources and scope
Source/checker commit df8001b87da1 and freeze publication206ba5c3e9bd precede data. Protocol body SHA256:
#text(size:8pt)[31b28e2913c78fc1c78c819f8190af17b777d4f18b810dec9120cc22071fe335]
Both binaries/sources, seven input hashes, finite grids and tie rule are bound. The canonical export validates; its60 cases are derived minimum records, not60 independent runs. Raw archive104,052bytes, SHA256:
#text(size:8pt)[f45bf33a443157615b1a7a68992fe14db5ce3fc1e10a65e85a37e7bc671e6b70]

Initial compiler indentation warnings, the first stopping-rule schema error and an incorrect export-schema lookup remain documented. No scientific scan was repeated. Each invocation was capped at120seconds, one process/thread and2GiB. No new cloud job, controller, role, cap, budget or generator was changed. Existing-host billing was not measured. No cryptographic time was collected; the full cold objective still requires plan preparation, allocations, normalization, conversion, verification and destruction on an admitted host.

#text(size:8pt)[Construction and source review: ../../RESIDUAL_SPLIT.md. Replay/raw custody: ../../residual-v1/README.md. Standard simultaneous-scalar background: Grabner–Heuberger–Prodinger, helmut-prodinger.at/pdffiles/joint.pdf, introduction/section2; read primary author PDF. Independent window expansions are used, with no JSF optimality or novelty claim. KB unavailable; migrated wrapper absent, exit127. Double-base author PDF returned502 and is not relied on.]

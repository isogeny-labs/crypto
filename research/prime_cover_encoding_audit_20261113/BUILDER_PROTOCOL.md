# Paired polynomial builder development contract

Local implementation/correctness work only; no solver, timing or cloud run.
This extension follows the completed saved-input audit and is prospective for
its own exports/tests, not a change to the frozen original measurements.

Inputs are the same two frozen round5 JSONL files. Build the direct and graph
systems from one parsed input object: same p, a,b, g,zeta, m,t, cover numerator,
and target x. Match the frozen pilot's summation-first/tower-after ordering.
The graph appends m image coordinates and m monic graph equations, using
ordinary S3 in the image coordinates. Polynomial cubic covers only; no poles.

Predictions: the saved m4t4 direct layout is18 variables/19 equations. The
cubic raw degrees begin12,8,8; reference4,4,4. Graph layout is22 variables and
23 equations, maximum raw degree4 in either case. Added dimensions and lowered
input degree are structural properties, not work or asymptotic improvements.

Correctness obligations before export: symbolic substitution of every new image
coordinate by its prescribed cubic/identity must reproduce every direct
polynomial coefficient-for-coefficient and annihilate every graph equation.
Check independent symmetric scalar S3 against the expanded builder, canonical
lift evaluations on fixed deterministic vectors, and a changed image-coordinate
negative control. The existing Lean inverse maps cover generic pointwise graph
correspondence; native builder attachment is a separate correctness check.

Reuse the frozen pilot's small sparse-polynomial construction convention,
with explicit dimension checks and no F4 solver dependency. Preserve original
native audit code/output. Emit canonical JSON and msolve input text for later
consumer validation. No solver-format consumption or genuine elliptic-relation
witness is claimed until independently checked on an admitted run.

A failed coefficient, dimension, raw-degree, scalar or projection check stops
export. Keep failed checks and original source snapshots. Fixed local unit
vectors are development controls, not an online attack or timing panel.

Sources: original frozen Pol/S3/summation/Kummer code at3466f3be9e; Galbraith
2.4.1-2.4.3 and15.8.1-15.8.2; original round5 protocol and amendments; prior
bounded auxiliary/cover-map queries and current graph inverse proof. Standard
encoding, no novelty claim. The new algorithmic finder and uniform bit cost
remain unresolved.

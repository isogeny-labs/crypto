# Four Q22 survivors excluded; four new Q24 survivors retained

Q23 generalizes the exact Round16 source normalization to each of the
four ordinary x inputs0,2,4,6. It accepts only constants, that one input,
Add and Mul in the actual dependency cone. It does not pass through
coefficient Frobenius or inverse gates. The degree and h-membership depth
are tracked exactly, with full node/gate/coefficient exports.

Each input supplies a cubic U and a quartic V, with normalized source
relations h²F1,h²F2 in the original ideal I. The same universal integer
quartic residual from Round16 gives h⁶R in I. At h=0, R=c3³U⁴ with c3
nonzero. This yields sixteen actual source-bound separators across the
four graphs, not a full saturation algorithm.

| graphs | ordinary cubic nodes forced zero on the closure boundary |
|---|---|
|0,1|19,31,42,53|
|2,3|19,30,41,52|

The old Q22 point has a unit second cubic, node31 or30. It is excluded
by the second-factor separator in every graph. Generated Lean proves
sixteen scalar boundary evaluations plus the four actual Q22 unit bindings
and four quartic gate bindings. The global identification of these exported
field/circuit equations with the geometric source remains the same written
bridge as Round16. This is an arbitrary-ramification closure exclusion,
not merely a first-jet obstruction.

Q24 adds all four valid radical boundary consequences, retaining every
original equation and all Q13–Q17 repairs. It finds four new checked
nonzero models in nine search visits per graph, without a cap:

| graphs | nonzero nodes at h=0 |
|---|---|
|0,1|59,61,64,65,66|
|2,3|58,60,63,64,65|

Their support is now in the supersingular companion subcircuit. They are
not classified as false or genuine closure points. Generated Lean checks
each complete finite prior-constraint model, its four cubic zeros and its
nonzero status. The individual cube nodes themselves are not asserted to
lie in the saturated ideal; vanishing follows from the radical boundary
quartic with an invertible coefficient.

Full h-saturated ideal equality, a complete boundary atlas and proper
polarized quotient embeddings remain open. The next test is a companion
source separator or a genuine full arc for the Q24 points; no conclusion
is drawn from a bounded search alone.

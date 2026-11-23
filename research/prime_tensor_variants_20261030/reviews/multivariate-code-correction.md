# Conditional review: code distance versus packet occupancy

2026-10-30 UTC. Narrow combinatorial review requested by the sibling mathematical audit. This is not a contraction algorithm, a lower bound on algorithms, or a proof of a general barrier. No implementation or experiment.

The proposed Singleton accounting is valid only after a separate packet-occupancy premise and a separate interaction-cost premise have been proved. Code size and minimum distance alone do not supply the claimed D^k real-slot bound when D is a slot multiplicity independent of the code alphabet.

## What Singleton actually says

For C subset [r]^d with minimum Hamming distance delta, projection onto any d-delta+1 coordinates is injective. Hence |C|<=r^(d-delta+1). If |C|=r^k, then k+delta<=d+1. Algebraically this implies

    d-delta-k>=-1.

Thus, **if** a concrete construction separately establishes an effective factor at least D^(d-delta-k), that factor is indeed at least D^-1. This numerical implication is correct. It is not a proof that the effective factor has the proposed form.

For a packet fixing two symbols per coordinate, let q be the number of unequal pairs. Its feasible orientation codewords form C intersected with a binary q-dimensional subcube. That intersection is a binary code of distance at least delta and has size at most2^(q-delta+1) if nontrivial. If the original code meets Singleton with delta=d-k+1, this is at most2^k. This counts orientation patterns, not all physical slots repeating each orientation pattern.

## Small explicit counterexample to the unqualified occupancy premise

Take d=2 and C={(a,a):a in [r]}. Then |C|=r, k=1, and delta=2: the code meets Singleton. Fix the two packet symbols0,1 on each axis. The allowed row-label patterns are00 and11; their complementary column patterns11 and00 also belong to C.

Suppose each axis has D even slots and its selected orientation bit is balanced, as in a direct repeated-slot interpretation. Each of the four bit patterns is repeated (D/2)^2 times. The allowed patterns therefore occupy

    2(D/2)^2=D²/2

real slots, rather than O(D^k)=O(D). Similarly the d=3 repetition code has k=1, delta=3, two allowed binary patterns, and occupancy2(D/2)^3=D³/4 rather than O(D).

This disproves the inference from code rate/distance to D^k occupancy under that interpretation. It does not prove that those patterns admit the other desired Cauchy-kernel/locality properties or that a multidimensional contraction improves. Its purpose is to prevent an unsupported general barrier claim.

## Conditions that would change the conclusion

If physical slots themselves inject into a code over an alphabet of cardinality D and that code has cardinality D^k, the D^k occupancy bound is immediate. Likewise, independently imposing suitable constraints on every bit level can reduce repeated-slot multiplicities. Either construction needs an explicit map and proof. Constraints on one selected orientation bit per axis only remove a constant fraction of a D^d grid when d is fixed.

A code on entire axis words is also different from a code on the selected symbols used by D.4.5. Distinct axis words can agree at the selected coordinate. Whole-word Hamming distance does not automatically imply the required number of crossing selected groups at every interpolation-node tuple. If different axes select different bit levels, constraints imposed only between matching levels need not control those mixed selections.

Finally, the statement that passive axes **require** D^(d-delta) factors is an independent lower-bound obligation. Ordinary full rank over Q is not automatically a lower bound on weighted mode counts modulo2^p, nor does it by itself handle restrictions to feasible ports or unequal factors. It should remain a conditional premise until the exact interaction/decomposition model is stated and proved.

The defensible result is therefore conditional: Singleton supports the final exponent comparison once the two structural premises are supplied. It currently does not establish either premise and does not rule out small-dimensional codes or other packet constructions.

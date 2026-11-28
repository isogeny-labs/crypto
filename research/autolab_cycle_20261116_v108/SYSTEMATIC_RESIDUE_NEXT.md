# Next hard-family hypothesis: systematic residue coordinates

This is a new unexecuted representation of the SAME76parameter family, not
another univariate timing case. C101/C102/C105 compiled all fourteen local
predicates on the original dense parameter basis and found no model. C99
already verified that the first seven four-coefficient residue observations
give a surjective linear map E:F2^76->F2^72, with a checked right inverse R
and four-dimensional kernel basis K. Its counting experiment did not use
these coordinates for the full fourteen-predicate native search.

Write actual first-seven observations as y=E*theta+b. Define

    theta = R*(y+b) + K*z,  y in F2^72, z in F2^4.

E*R=I and E*K=0 imply that every y,z produces the advertised observations.
Conversely, for every original theta, y=E*theta+b and theta+R*(y+b) is in
ker E, giving exactly one z. Thus this is a bijection of all76assignments.
The first seven local predicates then depend on disjoint blocks of their own
coefficient bits, while later residues supply dense coupling. This directly
exposes block structure instead of expanding each early coefficient through
dozens of unrelated primary bits. Solver branching behavior and total cost
may improve; neither sparsity nor bijection alone predicts a speedup.

Construct the full lifted-coefficient directions BEFORE transformation:
the73canonical H directions plus the three M,2M,4M constant-coefficient lift
directions. Compose every direction and affine base with R,K and R*b.
Do not treat z's last bits as the old q bits. Recover all original theta
bits, including the same three global q bits, as affine expressions; leading
guards, full-prefix support and original source/U recovery use those recovered
values. No independent lift selector is added at a residue.

Next experiments: independent tiny exhaustive bijection/root-set oracle;
actual E/R/K hash verification and all fourteen numeric predicate bindings;
then a new cold45-second necessary-family search whose setup includes the
coordinate map and all reporting. Model-free UNKNOWN remains inconclusive;
engine UNSAT needs an independently checked certificate. Complete original
chart/ghost/source/U coverage and the unchanged release gate remain separate.

This is a systematic-linear-code preconditioner, an established mathematical
idea, with a new measurable placement in this program. Novelty of a complete
rootfinding algorithm is unestablished. Prime fields need their own valid
coefficient/lift coordinates; no proper binary prefix space or unrelated-prime
reduction may be assumed for NIST inputs.

Performance relevance: no model was returned by the current45-second local
searches, so optimizing a root extractor called only after model discovery
cannot accelerate those executions. The cold root-kernel panel is useful
coverage and baseline evidence; this coordinate hypothesis instead targets
work actually executed on the hard coupled family.

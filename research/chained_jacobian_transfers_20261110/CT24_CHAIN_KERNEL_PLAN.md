# CT24 quotient matching and composite-kernel protocol

Before extending the fixed preflight, validate both C2 quotient charts over
F101 against actual targets(59,27)/(23,7), all curve points, and exact paired
2-root pullback supports. Reuse the existing cubic-to-short and checked
coordinate-isomorphism functions. C2 is the frozen preflight01 model
[19,0,10,0,22,0,1], not a newly selected model.

For each actual elliptic two-hop beta*alpha, nondual means ker(beta) differs
from image(alpha(E[7]))=ker(alpha_hat), by exact kernel-polynomial and
Frobenius checks already passed. Hence the order49 kernel has7-torsion of
order7 and is cyclic. Frobenius acts by a unit modulo49, reducing to the
observed original5/6 and satisfying X²+7X+101 or X²+11X+101. Enumerate all49
residues with these predicates, compute exact multiplicative orders and
fixed subgroup under the sixth power. Do not assume the current Fp6 field
splits the composite kernel. Preserve a larger required field as a genuine
implementation boundary; no new extension/Cantor engine is allowed merely
to evade it. Existing Fp6 can still validate each individual degree7 hop.

This is a bounded correctness/proof certificate, no timing experiment. Read
MilneAV10.20 andII1.1–1.2 plus retained HLP/dual[7] identities. New finite
Lean certificates must use kernel computation, not native_decide or added
axioms; distinguish them from the written elliptic/Tate/Picard bridges.
[pcc:unverified]

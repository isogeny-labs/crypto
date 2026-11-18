# Native target Jacobian arithmetic development contract

Exact input: the descended actual-kernel1 candidate quartic and its recorded234-point pool. The quotient/Jacobian Serre twist and source isogeny are still open. Sources checked: Khuri-Makdisi arXiv:math/0105182 Theorem5.3; current Sage khuri_makdisi.pyx small-model second method (including its variable-dimension intermediate-space correction); prior repository native Tian evaluation-space implementation. Native C++ implements the published algorithm; no Sage/Python arithmetic executes.

Use D0=3L0, where L0 is the first simple split vertical fiber. It has degree12. Represent [D-D0] by W_D=L(3D0-D), dimension22. Evaluate at85 distinct recorded Fp points away from L0 and the two test points. Since all products in the algorithm have poles bounded by7D0 of degree84, evaluation is injective. Check the ambient Riemann-Roch dimensions10,22,34,46 from plane-polynomial evaluation.

Construct point classes [P-R0] and [Q-R0] from the first two points in the next split fiber, imposing exact local jets. Check the independently constructed effective-divisor representative of P+Q-2R0 against addflip-based addition, and check P+(-P)=0. Check an actual vertical principal relation as a zero class and by explicit function membership. Corrupted curve-point and principal-function evaluation vectors reject.

This is a fixed functional target-arithmetic fixture: at most the named addition/cancellation comparisons and principal relation, no scalar search, order campaign, paid benchmark, clocks or performance claim. Preserve every dimension/arithmetic failure. It does not implement a source-to-target isogeny or settle the Serre twist.

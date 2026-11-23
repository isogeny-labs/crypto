# Next local construction: retained case0, not yet executed

The old certified ordinary CM catalog contains case0 at prime field
5629830687772387, with crater conductor1 and floor conductor33555419.
Its explicit point map remains uncomputed here. These retained equations
and the old timing records are not newly promoted by this plan.
[pcc:unverified]

The original case's auxiliary kernel curve has coefficients
5627477827931875,84482773359877 and rational kernel generator
(229789005348518,5112785640887010), with twist factor d=2. The actual
crater coefficients are5620419248410339,675862186879016; the retained
floor coefficients are2539833775737119,1737207970637906. Source:
research/conductor_gap_cm_orbits_20261015/prepared/kernels-reference.txt,
prepared/catalog.json and the study's constructor report. No new parameter
search or claim of a completed26bit crossing. [pcc:unverified]

A concrete missing bridge: the existing complete evaluator works on the
auxiliary curve T, while the benchmark points are on its quadratic twist E.
For E coefficients d²A,d³B, and normalized quotient x-map f on T, the twisted
map is (x,y) -> (d f(x/d), y f'(x/d)). Its codomain coefficients are d²A',d³B'.
The square root of d cancels. The rational x-only kernel computations can
therefore be reused, provided source validation is against E and final point
validation is against the twisted codomain. This is a proposed implementation
step, not a completed map or cost result. [pcc:unverified]

Implement as a separately versioned native evaluator so immutable existing
subjects/binaries stay unchanged. Qualify default byte equivalence, direct
PARI kernel-polynomial references on small twists, corrupted twist/points and
normalization rejection. Then freeze the retained large case, public target
panel and resource caps before attempting the full map locally. Verify actual
codomain against the retained certificate and transported prime-order points
independently; include dual composition controls rather than infer all of this
from j or field size. [pcc:unverified]

This is a lower milestone than the original80bit minimum and cannot discharge
that requirement. Prime-field IC with cofactor-one subgroup can use the
existing registered CM folded native controls only where their applicability
checks succeed; no IC method or speedup is supplied by this note. [pcc:unverified]

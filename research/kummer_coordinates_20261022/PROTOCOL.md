# Fast-Kummer coordinate identity gate

Frozen on 2026-10-22 UTC before executing coordinate candidates. Use the 21
Scholten quintics in `research/four_followups_20261021/results/kummer-model-gate.jsonl`
and the first independently verified base-field theta tuple per model from
`research/kummer_theta_20261022/results/theta-models.jsonl`.

Implement the generic Mumford projection and projective xADD/xDBL from
[Chung, Costello, and Smith §§7.2–7.3](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/recovery.pdf).
Transform divisors from the original quintic using the checked affine root
map `x=r0+delta*t`, `y=sqrt_scale*y'`. Construct independent sample divisors
from Fp curve points and compare the transformed genus-two Jacobian sum and
double with source Jacobian arithmetic. For each generic degree-two triple
`A,B,A−B`, compare xADD(Project(A),Project(B),Project(A−B)) against
Project(A+B) by exact projective cross-multiplication. Likewise compare
xDBL(Project(A)) against Project(2A). Test identity, inverse, branch two-
torsion, degree-one and zero-coordinate cases separately; keep each failure
or inapplicability as a raw row, without silently dropping it.

If a generic gate passes, implement the published special-divisor projection
and signed Recover and test the same cases. Only after these correctness gates
pass may an arbitrary-jump labeled collector and the paired 1,260-target
price cohort be attempted. Coordinate identities alone have null DLP cost
ratios. Store raw results, independent validation, a PDF and a graph. Never
infer fast DLP cost from published operation counts alone.

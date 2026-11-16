# Chained Jacobian transfers: executable research protocol (2026-11-10)

## Objective
Determine whether a **computable** sequence of curve/Jacobian homomorphisms transports a specified prime-order elliptic-curve DLP into a representation with measurable structure or lower *total* solving cost. A diagram or abstract subgroup isomorphism is not a transfer algorithm.

## Prior work and scope
Repository references: `papers/ecdlp-index-calculus-treatise/survey/transfer.md`, `research/ghs_cubic_cover_census_20261012/PROTOCOL.md`, `research/notes/ecdlp-general/RESEARCH_REPRESENTATION_STRUCTURE.md`, and `docs/curves/jacobian-transfers.json`. Classical starting points: Kani–Rosen, *Idempotent relations and factors of Jacobians* (1994); Frey–Rück, *A remark concerning m-divisibility and the discrete logarithm in the divisor class group of curves* (1994); Gaudry–Hess–Smart, *Constructive and destructive facets of Weil descent on elliptic curves* (2002). Verify exact hypotheses from primary texts before implementation. Repository code search found existing GHS transfers; this proposal does not claim novelty for covers, Jacobian splitting, or transfer composition. Research-KB MCP unavailable in this session; full literature/KB receipt is pending.

## Mathematical admissibility
Let `E/k` have `P` of prime order `r` and `Q=[x]P`. For each explicit, efficiently evaluable group homomorphism `h_i:A_i(k) -> A_{i+1}(k)`, verify `h_i(Q_i)=[x]h_i(P_i)` and `ord(h_i(P_i))=r`. Reject the path if `h_i(P_i)=0`. Record kernel intersections, field of definition, extension degree, map construction cost and evaluation cost.

For a finite morphism `f:C -> E` of degree `d`, the induced pullback/pushforward obey `f_* f^*=[d]`. If `r∤d`, pullback is injective on the target `r`-subgroup. For another quotient `g:C -> E'`, the composite `g_* f^*` can be **zero**: do not assume distinct elliptic factors communicate. A genus-2 Jacobian cannot be isogenous to a single elliptic curve (dimension mismatch), but can be isogenous to a product of elliptic curves. Distinguish arbitrary homomorphisms from isogenies.

Ordinary algebraic maps, isogenies and Weil restriction do not change characteristic. A cross-characteristic abstract isomorphism between order-`r` cyclic subgroups exists but its efficient evaluation is *not* supplied by equality of orders, lifts or reductions. Prime-to-binary is a separate conjectural computational bridge with no claimed algorithm.

## Phase A: deterministic small-field falsifier
1. Enumerate small odd prime fields and nonsingular `E/F_p` with a prime-order subgroup `r`; record full group order, `P`, and deterministic `x`.
2. Enumerate genus-2 curves with two explicitly defined elliptic quotients (e.g. bielliptic models `y²=f(x²)` with squarefree sextic `f(x²)`); independently check genus, quotient equations and map degrees.
3. Implement/validate divisor arithmetic and induced pullback/pushforward using independently checked point/divisor representatives. Verify `f_* f^*=[d]` on the selected subgroup and that `g_* f^*(P)` is nonzero before claiming a transport.
4. Enumerate and classify *zero* cross-factor composites as valid negative controls. If no nonzero cross-quotient map exists in the tested family, report that rather than substituting an abstract group isomorphism.
5. If a nonzero path exists, verify every `x in [0,r)` on small examples and a second independent implementation; log `h(Q)=[x]h(P)`, order, kernel, timings, and serialized fixtures.

## Phase B: multi-hop and cost
Search additional explicit isogenies/correspondences between same-dimensional Jacobians, recording each hop and composite map. Deduplicate paths inducing the same subgroup action. Benchmark construction, evaluation, relation collection, linear algebra, individual log, and recovery on *identical* target instances; report cold and amortized costs separately. Compare with direct solving in source and destination. No speedup claim from algebraic existence alone.

## Phase C: cross-characteristic boundary
For distinct primes `p,q`, enumerate small groups sharing prime divisor `r`; demonstrate only the abstract existence of cyclic isomorphisms. Require a polynomial-time evaluable mapping on arbitrary encoded subgroup points, without access to their logarithms, before upgrading to a computational transfer. Distinguish characteristic-zero lifting and reduction from a group homomorphism between reductions. This phase is a no-go/assumption audit, not a promised implementation.

## Acceptance criteria
- Deterministic fixtures and replay commands; independent group-law checks.
- For each proposed path: actual map formulas, domain/codomain and field, subgroup order, nonzero image, and equality checks.
- Explicit negative controls for vanishing projections and characteristic mismatch.
- Total-work accounting, not just per-operation speed.
- Any failure remains a published negative result; no unsupported ECDLP break claims.

## Status / next action
**Protocol only; no code or benchmark executed in this PR.** Next: verify primary references and existing repo transfer code, implement Phase A on small public toy curves, then submit a separately reviewed compute run under repository EKS Spot rules. Do not launch paid compute until implementation and local tests pass.

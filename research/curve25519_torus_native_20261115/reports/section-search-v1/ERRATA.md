# Cost-model scope: upper bounds and the rejected sampler

The report and derivation remain byte-pinned historical records. Two scope
clarifications correct potentially overstrong readings of their cost wording.

1. **The chi+theta/2<3 test is a sufficient budget target, not a necessary
   complexity lower bound.** It comes from charging the proposed worst-case
   per-seed query upper bound to every attempted seed. The report/derivation
   and problem record use "needs"/"requires" too strongly if read as an
   exclusion of all other implementations. Early refusals, amortized scalar
   work or a genuinely faster/batched operator could have a tighter complete
   cost bound. Such an algorithm is assessed under the unchanged original
   full cold criteria. Missing this conservative product-of-upper-bounds
   budget alone neither disproves a route nor establishes a lower bound.
   [pcc:unverified]

2. **The counting rejection concerns the explicit uniform trial algorithm.**
   For a supplied target-bearing basis, draw a coefficient vector uniformly
   from its normalized p^5 chart and explicitly generate/test each draw,
   counting at least one ordinary operation per trial. The divisor count
   bounds a draw's success by binomial(M+5,6)/p^5. At M=O(p^(1/6)), even
   the trial count has an Omega(p^4) lower bound for this model. That is
   why this sampler cannot meet the selected cold target; no query upper
   bound is used as a lower bound. An algorithm handling a compact family
   through a bulk zero-search/restriction operation is outside this explicit
   trial model and needs its own proof/cost/witness contract, even if some
   marginal distributions are uniform. Biased construction remains open.
   [pcc:unverified]

These clarifications do not weaken an acceptance threshold or relabel data:
no coefficient-search experiment was run. The fixed native identity records,
unit-cancellation proof, PDF and original route criteria remain unchanged.

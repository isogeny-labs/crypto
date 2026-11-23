# Counting bounds for the published n=7 two-pole families

Frozen before the generalized counting proof, 15 November 2026 UTC. This is a structural classification and exact finite q2 reconciliation, not a vulnerability or attack-cost claim. [pcc:unverified]

## Prior work checked first

- [Menezes–Teske, Sections 5.1–5.2](https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf) define W0 and W1 and give the genus formula. Their W0 cardinality discussion uses scaled, swapped pair classes and treats further product collisions heuristically.
- [Hess, *Generalising the GHS Attack*, Theorems 11–14 and Section 6](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/315278965D8D277A85812D9498236A88/S146115700000108Xa.pdf/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem.pdf) supplies the exact two-pole genus and subgroup-map setting.
- The current repository q2 paired source census and corrected family ledger are in `../ghs_n7_prym_trace_20261115/`; the direct W0 coefficient census is in `../ghs_n7_isogenous_w0_20261115/`.

## Claim and falsifiers

Let q be a power of two, k=Fq, K=F_(q^7), and p1,p2 the two irreducible cubics of T^7+1 over F2. For each cubic kernel V_i=ker p_i(sigma), sigma(x)=x^q, prove |V_i|=q^3 and V1∩V2={0}. Bound direct W0 coefficient values by the number of unordered, k×-scaled nonzero pairs from each V_i. Bound published W1 cubic/(linear×cubic) coefficient values by the number of ordered, k×-scaled pairs. Characterize the separate direct genus-seven W1 profile of the same two-pole construction. Reconcile at q=2 with the existing complete CSV, reporting the W1 coefficient pattern, classical genus-seven slice, additional hyperelliptic isogeny-class leads, and exclusions without calling the latter direct maps.

Stop or revise on any mismatch with the source family's exact order polynomials, a failure of the scaling/swap action count, a q2 census disagreement, or a hidden use of product injectivity. Give inequalities for W0/W1 product sets, since further product collisions can occur. Do not extend the result to all covers or infer a rho advantage. Preserve the formulas and q2 data provenance.

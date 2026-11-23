# Frozen protocol — Frobenius cross-bilinear sum-bucket filter
Date: 2026-11-18. Claim class: CONJECTURE; novelty_unverified. Main experiment is toy-only.

## Non-recycled mechanism
The rejected trace-moment basis (2026-11-18), isogeny-class x-interval coverage lead (2026-11-10), genus-2 torsion sieve, and D4/S5 coefficient-span lattice do NOT test a **pair-dependent** coordinate predicate. This experiment evaluates the symmetric cross bilinear
C_k(P,Q)=Tr(x(P)x(Q)^(2^k)+x(Q)x(P)^(2^k)) in F_2
to predict the **nonlinear** sum-bucket Z(P+Q)=Tr(x(P+Q)^3).
C_k is invariant under simultaneous Frobenius and cannot be evaluated from a single operand alone. It is not a group homomorphism. Evaluate k=1,2,3,4. Reject any statement of global novelty without a broader search.

## Fixed cells, samples and correctness
Koblitz E_a: y²+xy=x³+a x²+1 over GF(2^n):
- n=11, a=1, modulus 0x805, group order 1982=2*991, prime subgroup r=991.
- n=13, a=0, modulus 0x201b, group order 8012=4*2003, prime subgroup r=2003.
Holdout sensitivity (not part of primary decision): n=15, a=0, modulus 0x8003, order 33044=44*751, r=751.
These orders are to be recomputed by full point enumeration; fail closed if incorrect.
Derive G as first nontrivial cofactor-multiple of lexicographically ordered rational points, verify rG=O and every sampled point is on curve. Verify exact group addition, negation, doubling, 64 associativity triples, 64 Frobenius compatibility triples. Python stdlib only, no Sage.

Seed 20261118, derived RNG per (n,a); sample 8000 distinct unordered nonzero subgroup index pairs for training and 8000 disjoint pairs for test, without replacement. No selection or change of k after inspecting test outcomes. Oracle target bit is recorded by a true EC addition, and cross-bilinear feature by field operations.

## Preregistered decision
Train: select the k and Boolean polarity yielding highest training balanced accuracy (ties smallest k, then positive polarity). Test: report ordinary and balanced accuracy, target majority baseline, 95% Wilson CI, and the entire contingency table. Prediction requires BOTH primary fields to show held-out accuracy >=1.20*majority-baseline, and a charged speedup bound >=1.20x.
Cost bound (optimistic heuristic, not a theorem): if classifier accuracy A on a balanced bit, uniformity within the correct sum-bucket would yield at most 2A enrichment in finding a specific target. With filter cost C_f and true EC-add cost C_add, max speed is 2A*C_add/(C_add+C_f). Charge C_f>=2 field multiplications per candidate plus trace and precomputation, and C_add as actual inversion/mul/square counts. Any primary accuracy or charged speed below 1.20 falsifies this exact hypothesis. Evaluate heldout permutation null and positive synthetic bit-C_1 with 10% flips. Flag null calibration failure instead of claiming support.

## Baselines and failure accounting
- Best fair local no-filter control: exact EC pair-sum enumeration at same pair set, no predictor; test label-majority prediction; random bit/label permutation null.
- External best algorithmic bound: automorphism-aware rho E[steps] approximately sqrt(pi*r/(2*2n)) (optimistic 2n folding). Report vanilla rho sqrt(pi*r/2) also; neither is a measured run.
- Best recorded IC arm is pt-l9 at degree 23, still 5.6–6.1 bits behind matched rho, but **not** a matched n=11/13 baseline; do not compare raw cost across sizes.
- Charge all enumerated failures, heldout pairs, construction, field ops, any table setup and fallback; no end-to-end ECDLP speedup inferred from bucket accuracy.
- CGK preprocessing lower bound S*T²=Omega(r/(2n)) remains a generic-oracle wall; pair-dependent field predicates may evade only the model assumption, not automatically the bound on this implementation.

## Reproduction and validation
`python3 research/unexplored_frob_bilinear_20261118/run.py --output research/unexplored_frob_bilinear_20261118/RESULTS.json`
Fail on wrong group orders, wrong identity/curve check, failed independent direct EC sum test, or overlap of test/train pairs. Save fixed seed, detailed primary/sensitivity results and summary. Independent cryptanalytic verification/PCC remains pending; local tests do not confer validation.

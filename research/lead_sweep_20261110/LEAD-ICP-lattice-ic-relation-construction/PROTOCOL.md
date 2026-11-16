# LEAD-ICP-lattice-ic-relation-construction

## 1. Lead as recorded
- Disposition: open_blocked (reviewed 2026-10-30; "proof and implementation gap", no mathematical obstruction claimed).
- Family: ic-prime. Title: Lattice index calculus - ideal/quaternion lattices, isogeny-adjoint witnesses and Newton-shift certificates as relation constructors on prime cyclic subgroups.
- Next-step excerpt (truncated): "Deliver the admitted polynomial successor: one pipeline on an explicit prime cyclic subgroup (F4..." ; assessment note: pipeline on F43/F97/F101 fixtures + F25 rank-two negative control. Cost: expensive (new pipeline; no theory yet). Why it might beat sqrt(p): "Relations from lattice short vectors not point decomposition; nothing demonstrated".
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 29-35); /home/user/crypto/research/ecdlp_subexp_desk_20261101/README.md (lines 94-107).

## 2. Hypothesis
Any relation constructor for a generic prime-order cyclic group that does not already hold discrete logs (the lattice of relations is itself the secret) needs, to collect m = 2(8n)^(1/3) independent four-term relations among m random group elements, more group operations than the matched rho reference sqrt(pi n/4), and the ratio grows with n (slope of log2(ops/rho) against log2 n is >= 0.05; the generic pair-collision model predicts about 1/6). A lattice constructor beating this would need a non-generic handle; the toy tests the generic baseline that such a handle must beat.

## 3. Preregistered prediction and falsification criterion
- Statistic: per size n, median over 30 seeds of (pair-product ops until m relations) / sqrt(pi n/4); plus the least-squares slope over six sizes (n ~ 2^12..2^22).
- Consistent: ratio >= 1 at every size and slope >= 0.05. Falsified: any size with median ratio < 1 (the generic oracle-free baseline would then already beat rho in this unit). Inconclusive: controls fail, A/A differs by > 25 %, or slope < 0.05.
- Positive control: a planted relation (e3 = e0+e1-e2 mod n) must be found in 30/30 trials at every size, and every found relation must verify against the known exponents.
- Null: elements drawn independently of the exponents (same instrument); its ratio should equal arm A within noise (reported, not gating).
- A/A arm: disjoint seed set (5000+) with the identical instrument; relative median difference <= 0.25.
- Decides: only whether the generic baseline for a lattice relation constructor is rho-beaten in this toy; nothing about an actual lattice method.

## 4. Method
Group: order-n subgroup of Z_p^*, p = 2n+1 safe prime (n the first safe-prime-pair above 2^b, b in 12,14,16,18,20,22), generator 4. Draw m random elements with known exponents. Enumerate pair products x_i x_j in index order, hashing; a collision between two disjoint pairs is a relation (verified mod n with exponents). Stop at m relations (ops = pair products). Seeds: arm A 1000+s, A/A 5000+s, positive 9000+s, null 7000+s, s<30. Rho reference analytic sqrt(pi n/4), unit = one group multiplication (GAE pricing at 0/0.1/1 is not applicable: both arms count only multiplications; no unpriced native work). Cost: under about 1 minute, under 50 MB, stdlib only.

## 5. Classification
runnable_local. A stdlib generic-group toy can sharpen the question (baseline a lattice constructor must beat); it does not implement the lattice pipeline.

## 6. What a run cannot establish
- It does not test lattice, quaternion or isogeny-adjoint constructors, nor elliptic curves (multiplicative toy group; EC group-op costs differ by a constant).
- Pair collisions are the generic baseline only; the F43/F97/F101 and F25 fixtures are not touched.
- Independence of relations is not rank-checked; the count is of distinct four-term relations.
- Toy sizes only; the slope is a finite-size fit. Results are labelled [pcc:unverified].

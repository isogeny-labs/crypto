# LEAD-EXO-global-lift-height

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_next_step (last reviewed 2026-10-24). Family: exotic.
- Title: Height-compressing global lifts (Q, number fields, function fields / elliptic surfaces).
- Next-step excerpt (truncated in leads.json): "Pursue both live branches: run IDEA-20261009-581f5d to time T_section(d,chi,p) for a bounded-hei..."
- Desk-note next step (ecdlp_subexp_desk_20261101 sec. 4 row 6): time T_section(d,chi,p) for a bounded-height section through a prescribed fibre point on E1 surfaces at Gaussian-heuristic degree, >=3 primes, random-system Groebner control; T > p^(1/2) poly(log p) closes the function-field variant, T <= p^0.45 on >=3 primes is a signal. Recorded: Q-lift variant closed by literature (xedni exponent 3); function-field P_lift(p)=(1-2/p+p^-3)/(2p^3), alpha=3, a viable source needs alpha<1/2.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 25, 181-187); /home/user/crypto/research/ecdlp_subexp_desk_20261101/README.md (lines 58-62, 105-109, 129-135).
- The full next step (Groebner timing) is not runnable here. This protocol instead re-derives the toy Q-lift height/coverage obstruction.

## 2. Hypothesis
Over Z, the integer tuples (a,b,x,y) with a,x,y in [0,B) and |b|<=B satisfying y^2 = x^3 + a x + b number N(B) ~ c*B^s with s in [1.5, 2.5] (counting heuristic s=2: three free variables, one equation forcing |b|<=B has density ~B^-2). Hence the bounded-height lifts needed to cover half of all p^3 targets (a,b,x,y mod p on the curve) require B50(p) growing at least like p^1 (heuristic p^1.5), far above the sqrt(p) budget of rho. So naive bounded-height Q-lifts are not a sub-rho source, consistent with the recorded alpha=3.

## 3. Preregistered prediction and falsification criterion
- Statistics: s_real = log-log least-squares slope of N(B) vs B over B in {6,8,12,16,24,32,48,64,96,128,200}; s_b50 = slope of log B50(p) vs log p over primes {7,11,13,17,19,23}, B50 = least height whose solution set covers >=50% of the p^3 targets (primes where 50% is not reached by BMAX=200 are dropped, need >=3).
- Consistent_with_prediction: 1.5<=s_real<=2.5, positive control slope within 0.1 of 3, both null rates within 0.01 of 1/11, and s_b50>=1.0. Falsified: s_real<1.2 or s_b50<0.5 (lifts denser than the heuristic, would reopen the lead for the toy family). Otherwise inconclusive.
- Negative control / null: random integer tuples mod p lie on the curve with rate 1/p (p=11, 60000 draws, seed 20261110).
- A/A arm: same null with seed 20261111; the two rates must agree within 0.01 of 1/11 (noise floor).
- Positive control: unconstrained b (height up to B^3) gives exactly B^3 tuples, slope 3.0, showing the slope estimator detects a different exponent.

## 4. Method
Single stdlib enumeration of a,x,y in [0,200)^3 with b=y^2-x^3-ax kept if |b|<=200 (8M iterations, a few seconds, tens of MB for the stored solutions). Solutions sorted by height max(a,x,y,|b|); N(B) by bisection; coverage via residue sets mod p accumulated in height order. Seeds fixed (20261110, 20261111). Cost estimate under 1 minute, under 200 MB.

## 5. Classification
runnable_local. The Groebner timing needs engines absent here, but a stdlib toy re-derives the recorded Q-lift height obstruction (closed branch) cheaply. It does not touch the open function-field branch.

## 6. What a run cannot establish
It says nothing about T_section, function-field/elliptic-surface sections, Groebner cost, or the Gaussian-heuristic degree. It uses one box-height family and tiny p, with boundary effects at small B; the exponent mapping to the recorded alpha=3 is a different parametrization and is not claimed identical. It cannot show any speedup over rho or index calculus. Results are [pcc:unverified].

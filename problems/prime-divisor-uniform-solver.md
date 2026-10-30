# Proved cheap relation supply for prime-field divisor systems is missing

- **Status:** investigating
- **Kind:** unresolved algorithm and complexity proof; separate cloud execution limit
- **Affected work:** [prime-field research continuation](../research/prime_one_shot_20261029/continuation/README.md)
- **Next action owner:** prime-field divisor-system research lane
- **Last updated:** 2026-10-30

## Problem

We cannot establish the requested novel sub-exponential prime-field ECDLP
algorithm because no fixed-base divisor procedure with a proved sufficient
sub-exponential cost and success probability has been constructed.

The selected construction uses odd prime whole-order E/Fp, odd arity
m=Theta(sqrt(log r)), and a power of two d. The original scalar/torus
versions require d dividing p-1 or p+1; a separately reviewed auxiliary
elliptic-curve construction removes that congruence restriction on paper
for sufficiently large fields.
The norm-function system enforces U|F_g through a shared radical remainder
circuit, with squarefree, target-separation and nonvanishing charts.
The original requirement remains a new classical algorithm with total cold
cost exp(o(log r)), not a call count, planted witness, generic decomposition,
precomputation advantage or fixed-size speed claim.

## Evidence and uncertainty

- **Observed:** [native solver](../research/prime_one_shot_20261029/continuation/solver/README.md)
  exactly matches90 complete model sets/400 models on E/F17 of order19.
  Actual direct mixed-target IC solves all18 nonzero targets through both
  point-only API and separate CLI. Independent native record replay passes.
  The current PDP visits p(p-1) coefficient pairs; it is exponential.
- **Derived:** scoped rationality, selected-chart coverage, finite reducedness
  and mixed-target nullspace recovery are supported by the continuation's
  written proofs and independent reviews. They leave a precise conditional
  reduction. A uniform complete oracle cost C(n)=exp(o(n)), including failed
  calls, would make the other overheads sub-exponential in the admissible family.
- **Missing algorithm:** no computable sub-exponential cap with a proved
  sufficient verified yield, or sufficient solving-degree bound, is supplied.
  Complete fast refutation is only a sufficient contract:
  the [selective contract](../research/prime_one_shot_20261029/continuation/theory/selective-oracle-contract.md)
  also permits false negatives if capped cost and verified yield are proved.
  A radical circuit, small solution count or low input degree alone does not
  establish either contract.
- **Applicability:** the scalar/torus versions retain their congruence
  restrictions. The [auxiliary extension](../research/prime_one_shot_20261029/continuation/theory/auxiliary-curve-extension.md)
  gives a charged expected sub-exponential construction over every sufficiently
  large prime field with an odd-prime-whole-order main input. Its radical
  rational-isogeny membership pipeline has not been implemented here. This
  supporting proof supplies no relation-finding algorithm.
- **Separate execution limit:** existing EKS smoke run40979213857 executed zero
  steps and was cancelled after its600-second queue limit. Current session has
  no arbitrary-command EKS endpoint/AWS identity. This is not evidence for or
  against the mathematical construction. Bound-witness msolve imports are
  frontend checks, not unknown-system performance evidence.
- **Not established:** novelty, a sufficient efficient solver, auxiliary-pipeline
  implementation, growing-size cost, a rho win, or exhaustive historical review.
  Specific strategy obstructions in the notes are not universal lower bounds.
- **Concurrent prior art:** the continuation's [tensor review](../research/prime_one_shot_20261029/continuation/tensor-review/README.md)
  identifies an existing claimed sub-exponential ECDLP theorem chain and records
  scoped proof checks without a concrete gap found. It is not an original
  algorithm from this session and does not prove the proposed norm-system
  certificate-degree bound. The initial focused audit omitted those manuscripts;
  that prior-art assessment is explicitly corrected.

## What would unblock it

- [ ] Supply an executable fixed-base implicit procedure, receiving only
  E,g,d,m,R and independent randomness, with independently checked rational
  relation extraction and correctly classified unknown/refutation outcomes.
- [ ] Prove a sufficient computable cost/yield contract on the good input family:
  either uniform complete exp(o(n)) cost, or a capped exp(o(n)) procedure with
  verified success probability at least exp(-o(n)). Include construction,
  failed calls, root extraction, verification and restarts. A low first-fall
  degree or a fitted timing exponent is insufficient.
- [x] Supply a supporting construction/bound without requiring a large2-power
  divisor of p-1 or p+1, while retaining the main odd-prime-whole-order input
  hypothesis. See the reviewed auxiliary-curve note and its classical sources.
- [ ] Implement and independently verify that auxiliary rational-isogeny
  membership pipeline before claiming it runs on unknown inputs.
- [ ] Freeze and execute isolated increasing-size natural-target comparisons,
  including positive/negative controls, full accounting, fastest external
  backend, independent replay and matched references before performance claims.

The first two items suffice for the conditional reduction when all charged
setup hypotheses are met. The reviewed auxiliary setup removes the modulus
congruence restriction on paper, but does not itself satisfy the algorithm
or novelty requirements.
The empirical item validates implementation and finite-size cost, not a missing
asymptotic proof.

## Next discriminating step

- **Proof obligation:** establish a sufficient capped cost/yield bound for actual
  elimination of the norm plus radical remainder ideal at balanced growing
  arity. Focus on positive-target first-witness cost; complete UNSAT cost is
  optional. Keep membership implicit rather than materializing binom(d,m)
  states. Small quotient degree on enough positive targets is now proved in
  the continuation; it does not bound initial elimination.
- **Inputs and controls:** natural targets; selected-chart satisfiable and
  nonempty-chart unsatisfiable cases; rational/nonrational regressions; norm
  remainder and original per-root/Graeffe circuits on identical domains.
- **Execution:** use the repository's prospective protocol and EKS route for
  scientific comparisons. Local native development checks remain fixed
  correctness tests. Do not substitute a quiet-host expectation for a solver
  proof; record all censoring, CPU/memory contention and frontend failures.
- **Outcomes:** a proved sub-exponential uniform bound plus executable solver
  closes the principal gap in the admissible family. A complete high-degree
  counterexample refutes only the proposed degree bound. A timeout or unavailable
  runner leaves the obligation unresolved and must not become a mathematical null.
- **Recheck trigger:** a new solver lemma/implementation, new admissible-family
  construction, or verified scientific compute availability.

## Resolution and history

2026-10-30: recorded after complete tiny unknown solving and supporting coverage
proofs. Subsequently weakened the sufficient solver contract to permit selective
capped search, and transferred the supporting analysis to the known PKM auxiliary
base with primary-source-checked construction cost. Neither supplies the relation
finder. The original goal remains unresolved; this record does not replace it.

# Corrected relation-finder experiment draft

Protocol proposal, not executed. Monthly isolab budget is exhausted; no new
paid job, timing campaign or budget override is authorized by this document.
No empirical gain or actual solver-cost lower bound is claimed
[pcc:unverified].

## Prerequisites before submission

- Recover/pin the L1 native closure source and Cargo manifest named by the
  committed manifest: d045562b147a294cccf75308a9223ba8422ded5351fd35e1a1abdac84b7a7537
  for closure/src/main.rs; 181ccc86f564594beb0aa41d9cafa467ec14a4d9e237da29e8c2c767ca02cb05
  for closure/Cargo.toml. The inspected committed study does not contain them.
  Alternatively provide an independently checked native replacement.
- Preserve exact input-polynomial JSON, generator version, curve/subgroup
  identities, target seed, factor-base definition and solver version. The
  inspected generator SHA matches0b4c781c3be5a99f51532ee116e9da9e6b7a2458958b7e2e5c650bbe41e97d2b.
  Do not execute inherited Python research; any new generator/checker is native.
- Cross-check polynomial identities, tower normal form and filtered-basis
  degree semantics before treating nonmembership output as a degree certificate.
- State the field family: a binary Kummer base needs2^t dividing p-1. Do not
  extrapolate to arbitrary prime fields without a construction and its cost.

## Separate the two workloads

1. **Degree diagnostic.** Preserve the original p16801793,m8,t3,seed1 cell and
   its planted control. A completed level9 closure or a complete derivation is
   informative. A time/RAM cap is censored, not a higher degree proof. This cell
   does not by itself decide an end-to-end attack cost.
2. **Actual relation finding.** Use a rational target R=aP+bQ generated online
   from a hidden-answer DLP instance. Factor-base construction, target masks,
   solver setup, witness extraction, signs, rejected candidates and all useful
   output processing are charged. Do not supply the norm/decomposition witness.
   Include naturally satisfiable and unsatisfiable cells; keep planted controls
   separate rather than extrapolating refutation cost to positive search.

Choose the parameter panel using corrected single-target accounting. Record
actual solver invocations >=1; if one solve outputs many candidates, count useful independent
relations rather than ordered tuples/sign permutations. Measure both one-output
and all-output modes when available. Repeated targets/caches are charged or
labelled secondary; no free precomputation or cross-target amortization.

## Required records

- Exact field/curve/subgroup slug; p,a,b,r,P,Q and their validation certificates.
- Eligible t, arity m, auxiliary variables, actual base size after rational-point
  filtering; duplicates/repetitions/signs and m!-type symmetry accounting.
- Every solve's status, actual sparse matrix dimensions/nonzeros, round degrees,
  output extraction work, distinct useful relations and rank increments.
- Final scalar extraction and exact xP=Q check. A nonzero target coefficient is
  required; known-witness/correlated-support controls are not the attack arm.
- Complete native raw records and independent replay/number recomputation. No
  leaderboard promotion without the standing validation gate.

## Host and measurement conditions

Use the repository's EKS/isolab protocol when resources become available.
For wall-time comparisons, one specified workload per isolated allocation,
fixed thread count/affinity, host inventory and peak RSS/cap/censoring records.
Compare with the fastest applicable external solver on identical inputs and
with the strongest protocol-matched native rho reference. Price all phases in
the same unit. Exact algebraic identities do not change with CPU contention,
although contention can make a fixed-time computation incomplete.

A higher-memory degree run needs a checked capacity estimate from serialized
rows/source, not an assumed140GB guarantee. Record process-concurrency and
memory caps. Preserve all failed runs and amendments; no post-hoc discard of
hard/high-cost or large-height cases.

## Interpretation

The Lean model correction says the declared m8 dense entropy charge does not
support its old single-target gain after the batch floor. It does not exclude
a cheaper sparse solver, new encoding or growing-parameter regime. A useful
next observation would be actual positive-solver work below that proxy, with
useful output and generation fully charged [pcc:unverified].

The original objective is2^{o(n)} bit work, n=ceil(log2 r). A fixed-m polynomial
in p with exponent below1/2 is a milestone, not that objective. Any eventual
sub-exponential claim needs uniform m(n),t(n), field-family eligibility, bit-cost
and output/rank bounds. Toy fits and a small certificate do not supply them.

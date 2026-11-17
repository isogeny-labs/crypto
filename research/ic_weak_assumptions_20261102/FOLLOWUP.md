# Six-obligation follow-up: first attacks and hard gates

Date: 2026-11-02

Status: two implementation advances validated; four theory/measurement lanes
sharpened to checkable next objects; no scientific performance run taken.

## Outcome

This continuation did not treat the six priorities as six interchangeable
benchmarks. It separated source-theorem admission, correctness proof kernels,
scientific panels, and mathematical lower-bound searches. The current state is:

- Tensor: exact BigUint D.4 guard mirror and first-boundary regression;
  predicate cost and aggregate complexity remain uncertified.
- Yield/rank: the named batch's off-main custody hold was verified before
  execution; `BATCH-79a580` may not be rerun, reclaimed, or silently adopted.
- Arity three: source-bound unit-equation explanations and independent replay;
  this is not yet a CaDiCaL propagator.
- PKM: the closure-dual target is now an exact nullspace certificate; no
  growing certificate has been found.
- Wagner: the exact `W` statistic, estimator gates, nulls, and both decision
  directions are recovered; a native frozen producer is missing.
- Xedni: selector quantifiers and a model-rescaling falsifier are explicit; no
  selected-height/global-minimal-discriminant theorem is proved.

The dependency and gate view is in
[`obligation-attack-graph.svg`](obligation-attack-graph.svg). The rendered
report is [`FOLLOWUP.pdf`](FOLLOWUP.pdf).

![Current attack graph: blue/green nodes have a new implementation artifact;
yellow/purple/red nodes are sharpened but still blocked.](obligation-attack-graph.png){width=78%}

## 1. Tensor contraction theorem-to-code audit

The audit used OpenAI Appendix D at commit `7ef5c0cc...` and the Bochum
presentation at `1ee44e56...`. The important correction is that the recursive
kernel is not an unguarded transcription. It has explicit runtime checks for
most structural invariants. The remaining gap is concentrated in theorem
admission and complexity accounting.

- **Graph and degree.** `Bipartite::from_boolean` performs the stated two-edge
  subdivision. `Setup::new` computes `ceil(s^(9/10))` exactly and forces
  fallback when the degree hypothesis fails. Both are mapped and fail closed.
- **D.4 parameters and guards.** The implementation uses the source `lambda`,
  leaf cutoff, epsilon `1/(128*1024)`, radix 64, `u_d`, `a_d`, `L`, and `D`.
  Variance, packing, and `2L<=Delta` are checked before recursion.
- **Packing.** Conditional expectation minimizes an integer second-moment
  objective. Every old edge is checked exactly once; packet count, padding,
  and each coordinate's group port balance are checked.
- **Modes and children.** Source-group orientation is checked while building
  incoming/outgoing forms. Strict children check eightfold edge shrink and the
  degree cap. Bounded mode tests exist; a general proof remains open.
- **Implicit tensors.** Packet descriptions store local matrices and forms,
  with no parent evaluator callback. Entries are evaluated at immutable
  creation precision and only then lifted.
- **Recovery.** Node precision, jet shape, grid coverage, denominators, and
  return to the base coefficient ring are checked. These are bounded controls,
  not a general formal proof.
- **Open admission gap.** Canonical IR provides a finite typed representation,
  but public `FnPredicate` may be opaque. Neither polynomial description size
  nor the uniform `exp(A*s^(9/10))` entry-cost hypothesis is enforceable for
  such a closure. Meter units are not a theorem cost model.
- **Open aggregate gap.** Local allocations are bounded and recursion is
  depth-first, but no source-level aggregate construction/space proof is bound
  to an admitted instance family.
- **Representation boundary.** Live setup is `usize`-indexed. This prevents an
  executable asymptotic branch but is not a counterexample to the theorem.

The new public module
[`admission.rs`](../../crates/hashed-ecdl-tensor/src/admission.rs) mirrors the
numeric source guards with `BigUint`. The CLI
`tensor-admission-audit [SIZE|2^K+N] [MAX_DEGREE] [DEPTH]` reports only numeric
admission and explicitly refuses to certify predicate or aggregate costs.

At the root, `u=floor(lambda/131072)`. The first even subdivided edge count
with nonzero `u` is therefore

```text
s0 = 2^131071 + 2.
```

At `s0`, degree two, and depth zero, the exact evaluator obtains
`lambda=131072`, `u=1`, `a=2`, `L=4`, `D=2^24`; every numeric D.4 guard passes,
while `s0` is not live-host representable. Four Rust regressions pass, including
the exact boundary and the 64-bit zero-scale statement. This establishes a
precise interface fact: practical public calls use exact fallback, while the
source recursion begins beyond the graph representation. It does not refute
the asymptotic theorem and does not validate a performance claim.

The next tensor step is not another small forced-recursion timing. It is a
typed admission object constructed from canonical network bytes that gives a
checkable description-size and per-operation upper bound for every predicate
family, followed by an aggregate construction/space proof. Arbitrary closures
must remain ineligible.

## 2. Fixed-budget yield-count-vector plus rank

The atlas's requested panel is not ready to dispatch. Its own problem record,
`reln-f2-p5-off-main-custody-unreconciled.md`, requires zero scientific runs
and forbids rederivation, reclaim, or receipt rewriting until source ownership
and native adoption are reconciled. The eight retained source blobs hash-match
their historical receipt, but the source commit is not an ancestor of inspected
main. Hash equality is custody evidence, not scientific admission.

The existing native `relation_yield_pruning` analyzer is also insufficient for
this question. It accepts episode aggregates (`relation_hit_trials`, verified
rows, rank, coverage, entropy), not the full per-target decomposition-count
vector or immutable sparse row population. It cannot reconstruct zero mass,
Fano factor, multiplicities, incremental rank, or graph nulls after the fact.

A clean successor must therefore emit, before any adaptive stop:

1. a fixed ordered natural-target vector and exact probe budget;
2. the exact count `c_D` for every target, including zeros and solver failures;
3. every independently replayed relation row with target and multiplicity;
4. rank after each row in the fixed order, dead columns, support components,
   2-core, and terminal deficiency;
5. a size-matched random-base arm and rewired/Erdos--Renyi graph nulls; and
6. source, fixture, binary, protocol, checkpoint, and output hashes.

The yield and rank analyses must consume the same write-once population. The
producer may report an operational recovery point, but collection must continue
to the frozen budget; otherwise the count-vector distribution is selection
biased. A new successor namespace may be built natively after its fixtures are
publicly derived. It must not inherit the historical batch identity.

## 3. Proof-eligible arity-three finite-field propagation

The canonical quadratic IR already binds every equation, variable role, field,
curve, target, and projected variable. The new
[`trail.rs`](../prime_shared_coefficient_sat_20261031/native/src/trail.rs)
adds a small independently replayable theory-proof kernel:

- premises are strictly ordered canonical field assignments;
- the explanation names one exact source equation and the canonical IR hash;
- substitution must leave either a nonzero constant (conflict) or exactly
  `a*x+b` with `a!=0` (unique propagation);
- nonlinear residuals, two remaining unknowns, satisfied equations, unknown
  variables, noncanonical residues, changed hashes, and changed conclusions
  fail closed.

Three Rust tests pass: an affine `target_zero` propagation, a constant conflict,
and rejection of a nonlinear/underassigned residual. A mutated propagated value
is rejected. This is the first proof-eligible finite-field trail rule in this
study, but it is intentionally not described as the production propagator.

The remaining implementation order is now concrete:

1. translate a checked field consequence into the Boolean selector literals
   that formed its premises and conclusion;
2. attach these clauses through CaDiCaL's external-propagator interface;
3. replay every emitted clause with this independent kernel plus the Boolean
   proof log;
4. add checked certificate types for source-equation linear combinations and
   guarded inversions, one rule at a time; and
5. only then freeze the `s=6..14` natural/planted/UNSAT panel.

The current kernel may be too weak to improve search. That is an empirical
outcome to measure later, not a reason to weaken proof eligibility.

## 4. PKM pseudo-solution and matrix width

The most direct lower-bound target can be stated as a finite certificate. For
degree `c`, construct the HKY closure space `V_{F,c}` in the tower normal-form
basis. A degree-`c` pseudo-solution is a linear functional `L` satisfying

```text
L(v) = 0 for every generated closure row v in V_{F,c},
L(1) = 1.
```

Equivalently, if `A_c` is the exact closure matrix with the constant column
distinguished, find `ell` with `A_c * ell = 0` and `ell_constant=1`. Direct
matrix multiplication verifies the certificate and proves `1` is not in the
closure, hence `d_F>c`. This is stronger and more relevant than another plot of
first-fall degree.

The Kummer group-algebra description suggests a structured ansatz: index tower
monomials by `z^k`, stratify by popcount, and allow the functional to depend on
the top `c` carry levels where multiplication wraps modulo `z^(2^t)-1`. The
one-summand and point-specialization maps are unsuitable because the existing
proof shows they collapse to constant bounds. A useful search must retain at
least the two-dimensional Kummer curve.

If such certificates persist for growing `c(t)`, they give an exact closure-
degree lower bound. A matrix-width claim still needs a separate argument:
degree alone does not lower-bound all representations or sparse work. The
certificate verifier should therefore report the actual normal-form column
support, row support, rank, nullity, and peak frontier. The existing `N=20`
240--380 GB estimate remains a secondary confirmation cell, not the proof
programme.

## 5. Wagner bucket variance

The exact missing quantity is not an informal bucket imbalance. For bucket
weights `q_c`, global agreement rate `delta`, and target-bucket rates `pi_c`,

```text
W = sum_c q_c * (pi_c/delta - 1)^2.
```

It yields the checked one-sided bound
`max_c(pi_c)/delta <= 1 + sqrt(W/min_c(q_c))`. When `q_c=Theta(1/M)` and
`M*delta=1+o(1)`, the Wagner penalty is bounded by
`G <= 1+sqrt(MW)(1+o(1))`. The proposed fit is
`W=Theta(M^a*p^(-1/2))`; the proof-repair direction needs `a<1/2` at
`M=p^(1/3)`.

Both outcomes must remain symmetric in interpretation. Small `W` supports a
proof attempt that retires the claimed filtering gain. Large/nondecaying `W`
breaks that repair and is only an attacker-favourable signal, not an attack by
itself. The positive control is the discrete-log interval pullback, which must
produce large `W`; SHA/matched-marginal shuffles are nulls. Exact whole-group
anchors at the two smallest primes and jackknife bias checks are mandatory.

The existing implementation lineage is legacy Python. Repository policy and
the scientific contract require a native Rust producer, frozen 8-prime ladder,
`M` grid, `10^7`-pair budget, checkpoint state, estimator replay, and EKS Spot
admission before execution. No result was manufactured from the old first-
moment data.

## 6. Xedni selected-lift/minimal-discriminant bridge

The quantifier order must be fixed before examples. A valid target has the
form: for one explicit selector `S`, one public curve/lift family, and every
finite-field input in that family, `S` terminates and its selected rational
points satisfy a uniform height bound against the global minimal discriminant.
The selector must also preserve the reduction map and prime-subgroup sampling.

Two easy substitutions are invalid:

- bounding every rational multiple is false on a positive-rank curve by the
  existing canonical-height theorem; and
- bounding against the discriminant of whichever integral model the algorithm
  happens to receive is not intrinsic, because admissible scaling multiplies
  the model discriminant by a twelfth power while leaving an isomorphic curve.

Therefore the first selector experiment must normalize to a global minimal
model, not merely a short or locally minimal model. A concrete initial selector
is allowed to be partial: enumerate rational lifts in increasing certified
naive-height boxes and select the lexicographically first lift that reduces to
the input point. The research question is then explicit: prove termination and
a uniform selected-height/minimal-discriminant ratio, or preserve the first
family for which the ratio is unbounded. Without termination, global
minimality, and good-reduction correspondence, this is only a search rule.

Even a successful selected-height theorem would leave Lang/direct gap,
Mordell--Weil rank and torsion fibres, useful nonzero denominators, cofactor,
and adaptive-hazard composition as separate graph nodes.

## Validation performed

The tensor module was built in Rust 1.90 with the repository lockfile; four
targeted tests passed. The default boundary CLI reported every numeric guard
true and `live_host_representable=false`. The arity-three native crate was built
with its lockfile; all three new trail tests passed. `git diff --check` is part
of the final repository validation.

No EKS scientific job was launched. Runtime/AWS readiness does not override the
yield custody hold, missing Wagner native producer, incomplete arity-three
propagator, PKM proof target, or xedni selector theorem.

# Prime-field continuation: sparse PKM and norm-branch falsifiers

**Date:** 2026-11-13  
**Status:** prospective protocol and evidence reconciliation; no experiment was run in this continuation.  
**Standing:** every research reading and proposed threshold below is `[pcc:unverified]`. No benchmark,
leaderboard, security, speed, or complexity claim is made.

This note resumes the five-lead chase in this directory after the later
single-target cost-floor audit and public norm-branch work landed on `main`.
It keeps historical outcomes unchanged and identifies the next tests that can
still alter the prime-field conclusion.

## Decision

1. The large-memory `D(8,3)` computation is now a **degree diagnostic**, not the
   decisive end-to-end experiment. The cost-floor audit proves that the old
   dense entropy-charge model cannot beat rho after enforcing one whole
   single-target solve, even if `D(8,3)=9`.
2. The best remaining experiment inside the PKM index-calculus lane is an
   **actual positive sparse relation-finder panel** with useful output, rank
   increment, setup, failed targets, and the whole-call floor all charged.
3. The cheapest new tensor/Boolean discriminator is the **signed public
   norm-branch determinant**, not another hashed-network or direct Semaev
   bit-blast. Its coefficient tensor has an explicit dense middle-layer
   structure. Exact balanced ranks should be measured before any SAT tuning.
4. Ordinary CDCL/SMT, the published hashed-factor-base tensor construction, and
   bit-level Semaev contraction are controls, not the current lead:
   - the hashed construction's published recursion is not admitted below an
     enormous graph size and its composed finite-size exponent is above rho;
   - measured Semaev Boolean encodings have width at least linear in the whole
     field-bit width, far above the enumeration exponent;
   - existing direct prime-field SAT/SMT probes are slower than lookup and have
     no measured sub-enumeration query exponent.

The current ordering by information gained per cost is therefore:

| priority | test | reason |
|---:|---|---|
| 1 | A: corrected sparse PKM relation finding | only surviving traditional prime-field IC mechanism with an explicit end-to-end gate |
| 2 | B: norm-branch coefficient-rank census | cheap exact falsifier for the newest Boolean/tensor mechanism |
| 3 | B2: source-bound SAT/proof-size panel | run only if B exposes compression; otherwise it is solver engineering without a mechanism |
| 4 | `D(8,3)` level-9 closure | useful degree certificate, but no longer decides the advertised attack |
| 5 | S4 post-HG probability and whole-Jacobian `r≈p` rerun | closure chores; neither currently carries a credible exponent mechanism |
| blocked | growing-rank lifting | requires an explicit public small-height lift selector before another scaling panel |

## A. Corrected sparse PKM relation-finder panel

### A1. Load-bearing quantity

Fix arity `m=8`, factor-base size `B`, and let the complete charged work of
one sparse solver invocation scale as

```
c(B) = B^(gamma + o(1)).
```

The optimistic all-output single-target model from the cost-floor audit is

```
W_all(p,B) = max(1, p/B^7) * c(B) + B^2.
```

The first term already assumes that one invocation returns every useful
independent relation available from its target; deficient output rank can only
increase work. Put `B=p^beta`. Optimizing this model gives

```
gamma <= 2:  E_all = 2/(9-gamma),
gamma >= 2:  E_all = gamma/7,
```

up to subpolynomial factors. Therefore the optimistic model beats exponent
`1/2` only if

```
gamma < 7/2.
```

If a solver returns at most one useful relation per invocation, the call floor
is `max(B,p/B^7)`, and the corresponding necessary gate tightens to
`gamma < 3`. Reusable setup must be charged separately and satisfy the same
chosen-parameter exponent budget. These are model-accounting gates, not lower
bounds on an unknown sparse solver.

This makes the next question precise: **does actual positive sparse work,
including witness extraction, scale below `B^3.5` in the most optimistic
all-output reading?** Solving degree alone cannot answer it.

### A2. Frozen panel to build

Use a growing family of primes with `2^t | p-1`, `B=2^t`, and ratios
`log_B p` bracketing 7, 8, and 9. The ratio-7 cells test the optimistic
whole-batch boundary; ratio 8 separates one-output from all-output behavior;
ratio 9 keeps naturally unsatisfiable targets in the panel. Start with the
smallest native-replayable cells and require two held-out larger `t` values
before fitting a slope.

For every cell:

- construct the curve, prime subgroup, Kummer base, and target from public
  data after the protocol hash is fixed;
- use natural `R=aP+bQ` targets with the answer hidden from the solver;
- keep satisfiable and unsatisfiable natural targets; planted decompositions
  are interface controls only and never enter the attack fit;
- run the strongest pinned sparse external solver and an independently checked
  native implementation on byte-identical polynomial inputs;
- record every sparse matrix dimension and nonzero count, arithmetic work,
  round degree, wall/CPU time, peak RSS, cap/censoring, invocation count,
  extracted decompositions, distinct useful rows, and rank increment;
- charge factor-base generation, target masks, signs, rejected candidates,
  witness extraction, relation filtering, linear algebra, and final
  `[x]P=Q` verification;
- compare the complete projection with a protocol-matched rho reference in the
  same unit.

Prerequisites are unchanged from
`research/prime_norm_rank_20261112/cost-floor-report/NEXT_TEST.md`: recover and
pin the missing L1 closure source named by the manifest, or supply an
independently checked replacement; preserve exact generator, solver, curve,
subgroup, target, and factor-base identities.

### A3. Decision rule

- **ADVANCE:** all correctness/replay controls pass; the two largest uncensored
  held-out sizes contain natural positive solves; useful-output rank is
  recorded; and the upper confidence bound for complete per-call work has
  `gamma < 3.5` in the optimistic all-output model. Promotion to an attack
  claim additionally requires the full projected exponent below the matched
  rho bound.
- **KILL this sparse mechanism at measured scale:** the lower confidence bound
  has `gamma >= 3.5`, or output/rank/setup charges force total exponent at
  least `1/2` on the preregistered family.
- **INCONCLUSIVE:** top cells censor, natural positives are absent, source
  replay fails, or the interval straddles the gate.

The `D(8,3)` completion may be run as a parallel diagnostic after source
recovery, but it must not substitute for A.

## B. Signed norm-branch tensor census

The latest norm work reduces branch selection to one exact square matrix. Let
`k` be the number of sampled support points and `d=k+1` include the fixed
target branch. Define

```
u = floor(d/2)+1,
v = floor((d-1)/2),
u+v = d.
```

For distinct abscissae `x_i`, nonzero ordinates `y_i`, variable signs
`epsilon_i in {+1,-1}`, and the fixed target sign, form

```
M(epsilon) = [ U | D(epsilon) W ],
U[i,j] = x_i^j       (0 <= j < u),
W[i,j] = y_i x_i^j  (0 <= j < v).
```

Under the guards in `DERIVATION.md` and the public-residue report,
`det M(epsilon)=0` is equivalent to a nonzero bounded `A/B` branch
certificate, which is then replayed through the full norm identity and group
sum.

Laplace expansion gives the source-bound sign polynomial

```
det M(epsilon)
  = sum_{|S|=v} sign(S) det(U[S^c]) det(W[S])
      product_{i in S} epsilon_i.
```

Every displayed coefficient is nonzero: the two minors are Vandermonde
determinants, multiplied in the second block by nonzero ordinates. After fixing
the target sign, the polynomial has every sign character on the two adjacent
middle levels `v` and `v-1`. This does not prove search hardness, but it
rules out a sparse low-degree-character explanation and names the tensor whose
compression must be demonstrated.

### B0. Semantic gate

On exhaustive tiny groups, regenerate the same instance in three independent
forms:

1. direct signed group sums;
2. singularity of `M(epsilon)`;
3. reconstructed `A,B,c` with the complete norm and target-branch checks.

Require identical model sets, least witnesses, and counts. Mutations must
include a target-sign flip, one ordinate flip, a duplicated abscissa, a changed
degree budget, and a changed curve coefficient. Invalid guarded inputs fail
closed.

### B1. Exact coefficient-flattening ranks

For `k` on a frozen increasing ladder, split the variable signs as evenly as
possible. Materialize the coefficient tensor by subset cardinality blocks and
compute exact ranks over `F_p`; compute canonical integer-lift ranks over
`Q` as a secondary diagnostic.

Paired arms:

- natural curve-derived rows;
- matched rows with independently randomized nonzero ordinate multipliers;
- independently randomized nonzero middle-layer coefficients;
- a synthetic separable low-rank positive control;
- fresh-process A/A regeneration.

Preserve the same support, split, dimensions, nonzero pattern, and target/model
count stratum wherever applicable. Record full block ranks, certified nonzero
minors, peak memory, and all cap/censoring outcomes.

- **ADVANCE to B2:** on both largest held-out sizes, natural normalized rank is
  at most half the matched-null rank and its `log2(rank)` slope is lower by a
  frozen nonzero margin; the positive control fires.
- **KILL the registered linear-message compression:** natural ranks meet the
  available balanced upper bound or track both nulls without a held-out slope
  gap.
- **INCONCLUSIVE:** rank computation censors or model-count conditioning cannot
  be performed.

A failure here does not prove that every nonlinear or algebraic branch search
is hard. It closes the specific exact separator-message mechanism.

### B2. Conditional SAT/proof-size panel

Run this stage only after B1 advances. Export the same canonical instances to:

- a source-bound `F_p` row-space theory with independently replayable conflict
  explanations;
- ordinary CNF/CaDiCaL as an encoding-cost control;
- one exact BDD/ZDD or variable-elimination baseline.

Use both natural SAT and exact natural UNSAT cells. A SAT result requires full
model/norm/group replay; UNSAT requires an externally checked proof on the
exact query. Record setup, decisions, conflicts, learned-clause widths,
trimmed-proof bytes, theory row operations, peak state, and timeouts. Fit
complete work against subgroup bits and compare with a matched rho run.

The SAT hypothesis advances only if the upper confidence bound on complete
work has exponent below `1/2` on held-out sizes and the natural/null effect
has the sign predicted by B1. Short refutations alone are not enough: positive
witness search and the final scalar recovery remain charged.

## Remaining closure work

- **L2/S4 post-HG lane:** run the already named success-probability decay panel
  only as a bounded closure test; include its probability in the complete
  relation and linear-algebra cost before interpreting a positive.
- **Whole-Jacobian transfer O3:** rerun with subgroup order `r` frozen
  proportional to `p`; the existing panel cannot estimate a slope in `p`
  because `r` varied independently.
- **Growing-rank lifting:** do not launch another height census until an
  executable public-input selector returns small-height preimages. Measurements
  on unsuccessful lifts do not bound successful lifts.
- **Research knowledge base:** unavailable in this continuation. Repeat
  novelty/prior-art queries before any novelty claim.

## Completion boundary

No finite campaign can disprove every coordinate-aware prime-field algorithm.
This protocol exhausts named mechanisms only at its declared representation,
family, and cost model. A survivor must pass the held-out scaling gate and then
be inserted into complete relation collection, linear algebra, memory, and
verification accounting before promotion.

# Foundational index-calculus attack queue

Date: 2026-11-17

This is the human execution view of the canonical claim graph. The authoritative
claims, dependencies and acceptance criteria are in
[`corpus.json`](corpus.json) plus
[`ic-six-obligations-20261117.json`](ic-six-obligations-20261117.json). The
generated priority order remains [`ATTACK_GRAPH.md`](ATTACK_GRAPH.md).

The queue does not promote an open bridge into evidence. Each lane has one
finite next object, an acceptance condition and a symmetric negative outcome.
A lane is complete only when the corresponding research target can be removed
or narrowed with linked evidence and regenerated graphs.

![Compact dependency view of the six attack lanes. Green nodes are checked,
blue nodes are definitions, yellow nodes are open, red is blocked, and pink is
conditional.](foundational-attack-queue.svg)

Editable source: [`foundational-attack-queue.dot`](foundational-attack-queue.dot);
vector rendering: [`foundational-attack-queue.svg`](foundational-attack-queue.svg).

## Attack order

| order | lane | present state | target label |
|---:|---|---|---|
| 1 | fixed-population yield and rank | blocked on write-once custody | yield/rank |
| 2 | tensor theorem admission | ready | tensor costs |
| 3 | arity-three proof composition | ready | arity-three replay |
| 4 | PKM lower bound and width | ready | PKM dual/width |
| 5 | Wagner representation filter | ready | Wagner W |
| 6 | xedni selected lifts | ready | xedni selector |

### 1. Fixed-population yield and rank

- **Checked starting point:** the mean count is a theorem; early-row and
  stopped aggregates do not determine target-wise shape or terminal rank.
- **Next attack object:** one immutable natural-target population containing
  all counts, failures, rows and incremental-rank/support data.
- **Decisive outcomes:** a measured distribution/rank law at the frozen scale,
  including either agreement or a preserved deviation. Custody failure leaves
  the target blocked.
- **Canonical target:** `RT-PRIME-YIELD-RANK-WRITEONCE`.

### 2. Tensor theorem admission

- **Checked starting point:** exact D.4 numeric admission begins at
  `2^131071+2`, beyond the public 64-bit graph representation.
- **Next attack object:** a typed predicate-family cost certificate followed
  by an aggregate construction/space proof.
- **Decisive outcomes:** the admitted family satisfies both source cost
  hypotheses, or the first predicate/aggregate counterexample is retained and
  the conditional route narrows.
- **Canonical target:** `RT-TENSOR-TYPED-COST-CERTIFICATION`.

### 3. Arity-three proof composition

- **Checked starting point:** one-equation affine propagation and
  constant-conflict explanations replay against the canonical IR hash.
- **Next attack object:** total field-value/literal binding and combined
  finite-field/Boolean proof replay.
- **Decisive outcomes:** every emitted clause replays, or the first
  encoding/composition mismatch becomes a regression fixture. Only then may
  solver scaling be measured.
- **Canonical target:** `RT-ARITY3-PROOF-COMPOSITION`.

### 4. PKM lower bound and width

- **Checked starting point:** `A_c ell=0`, `ell_constant=1` is a finite
  closure-degree certificate; degree alone does not bound width.
- **Next attack object:** an exact dual verifier, a growing structured
  certificate family and a separate representation-width theorem.
- **Decisive outcomes:** a growing lower bound plus proved work consequence, a
  bounded-degree certificate family, or a preserved failure that leaves the
  asymptotic question open.
- **Canonical target:** `RT-PKM-DUAL-WIDTH-CERTIFICATES`.

### 5. Wagner representation filter

- **Checked starting point:** `W=sum q_c(pi_c/delta-1)^2` is the missing bucket
  second moment.
- **Next attack object:** a native exact-anchor estimator and frozen scaling
  panel, followed separately by the filter-gain bridge.
- **Decisive outcomes:** small `W` bounds the claimed filtering gain;
  large/nondecaying `W` reopens the bridge but is not an attack until
  enumeration, yield and interface costs are proved.
- **Canonical target:** `RT-WAGNER-W-NATIVE`.

### 6. Xedni selected lifts

- **Checked starting point:** the finite counting/hazard core is checked under
  explicit hypotheses; selector and height/reduction interfaces remain open.
- **Next attack object:** one public family and explicit selector with
  termination, global-minimal-model height and subgroup-reduction certificates.
- **Decisive outcomes:** a complete bridge, explicit nontermination, unbounded
  selected-height ratio, or a reduction/subgroup counterfamily.
- **Canonical target:** `RT-XEDNI-SELECTOR-MINIMAL-HEIGHT`.

## Common completion contract

Every lane must leave the graph in a state another researcher can attack
without reconstructing its scope:

1. **Identity and quantifiers.** Name the exact family, field, curve/subgroup,
   parameter ladder and universal/existential quantifiers.
2. **Claim type.** Separate definition, theorem, conditional theorem,
   measurement, heuristic and open bridge. A native checker proves only the
   contract it replays.
3. **Certificate or immutable population.** Proof lanes emit independently
   replayable certificates; measurement lanes retain the fixed population,
   failures and controls needed to recompute the statistic.
4. **Symmetric outcomes.** State what a positive, null, negative, censored or
   custody-blocked outcome changes in the graph.
5. **Typed edges.** Add `conditions`, `enables`, `model_bridge`, `qualifies`,
   `obstructs` or `refutes` edges as appropriate. Do not rely on prose to carry
   a load-bearing dependency.
6. **No proxy promotion.** Mean yield is not distribution shape; early row
   independence is not terminal rank; first-fall degree is not matrix work;
   concentration is not an enumerable filter; a one-shot event is not an
   adaptive hazard theorem; numeric admission is not a complexity proof.
7. **Regeneration and validation.** Run `validate.mjs`, `render.mjs`,
   `attack-graph.mjs --check` and `weakness.mjs --check`, render affected
   Graphviz outputs, and update the research target or problem record in the
   same change.

## Dependency view

```text
typed tensor costs ------------------>
fixed yield distribution ------------>
terminal/reused relation rank -------> complete IC exponent/cost claim
proof-eligible solver trace ---------->
PKM degree + width/work bridge ------->
Wagner W + enumerable filter -------->

xedni selector termination ---------->
selected height / minimal model -----> conditional xedni obstruction
reduction + subgroup bridge ---------->
```

The xedni branch is separate from factor-base index calculus, but it shares the
same evidence discipline: every interface needed by the end-to-end conclusion
must be explicit and independently checkable.

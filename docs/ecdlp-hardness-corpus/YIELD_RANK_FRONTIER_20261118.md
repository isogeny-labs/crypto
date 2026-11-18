# Yield and rank frontier decomposition

Date: 2026-11-18

## Result

The canonical graph's two highest-impact index-calculus frontier nodes are
`HEUR-DECOMPOSITION-PROBABILITY` (14 downstream claims) and
`HEUR-RELATION-RANK` (11 downstream claims). They were too coarse to attack as
single units. This round splits them into 12 definitions/open obligations, 32
typed dependencies and six research targets in
[`ic-yield-rank-frontier-20261118.json`](ic-yield-rank-frontier-20261118.json).

No new distribution, rank or complexity result is claimed. The exact mean
theorems and existing small-field censuses constrain the new gaps; they do not
close them.

![Dependency view of fixed-population yield and relation rank. Green nodes are
checked or measured, blue nodes are definitions, yellow nodes are open, red
nodes are composition gates, and note-shaped nodes are executable research
targets.](yield-rank-frontier-20261118.svg)

Editable source:
[`yield-rank-frontier-20261118.dot`](yield-rank-frontier-20261118.dot).

## Requirement-to-evidence map

| requirement | graph evidence | state |
|---|---|---|
| Preserve every target, zero count, failure and verified witness | `DEF-IC-YIELD-COUNT-VECTOR`; `RT-IC-PUBLIC-POPULATION-CONTRACT` | contract ready; natural population blocked |
| Distinguish exact mean from target-wise law | `THEOREM-YIELD-CONSERVATION` qualifies `GAP-IC-YIELD-SCALING-LAW` | mean checked; law open |
| Prove that algebraic roots become usable relations | `GAP-IC-SOLVER-VISIBLE-YIELD`; `RT-IC-SOLVER-VISIBILITY-AUDIT` | ready to attack on exhaustive toy cells |
| Retain the complete relation population | `DEF-IC-RANK-SUPPORT-TRACE`; `GAP-IC-STOPPING-CENSORSHIP` | contract ready; natural population blocked |
| Separate row count, support and rank | `GAP-IC-ROW-ENSEMBLE-LAW`, `GAP-IC-SUPPORT-CONNECTIVITY-LAW`, `GAP-IC-TERMINAL-RANK-LAW` | open |
| Apply the correct linear-algebra success condition | `DEF-IC-KERNEL-USAGE-MODE`; `GAP-IC-KERNEL-MODE-SCOPE`; `RT-IC-KERNEL-MODE-CENSUS` | route audit ready |
| Compose usable yield into the required terminal rank | `GAP-IC-YIELD-RANK-COMPOSITION`; `RT-IC-YIELD-RANK-COMPOSITION` | blocked on component certificates |

## What is already known

| evidence | exact scope | does not establish |
|---|---|---|
| `THEOREM-YIELD-CONSERVATION` and `THM-RP-YIELD-SUM-COUNT` | the total/mean representation count for a fixed finite domain | variance, zero mass, tails, target uniformity or solver visibility |
| `MEAS-PF-YIELD-CENSUS` | exhaustive small-prime, arity-two counts; symmetry-dependent dispersion and the ordered/unordered pairing correction | an asymptotic law for natural higher-arity families |
| `MEAS-RANK-DEFICIT-CENSUS` | small-prime reusable right-kernel samples in which a fixed surplus of ten relations often misses full rank | a replacement terminal-rank scaling law |
| `THM-LEFT-KERNEL-RELATIONS` | single-target extraction from target-dependent relations under its explicit scalar-field hypotheses | a reusable factor-base logarithm table or multi-target amortization |

The key separation is therefore:

```text
exact global mean
      |
      +--/--> target-wise usable-yield law

early independent rows
      |
      +--/--> support connectivity and terminal reusable rank

single-target left kernel
      |
      +--/--> reusable right-kernel/full-rank table
```

Each crossed arrow is deliberately absent from the graph unless a separate
bridge is supplied.

## Finite attack order

1. **Freeze the public population contract.** Implement
   `RT-IC-PUBLIC-POPULATION-CONTRACT` with synthetic positive and negative
   fixtures. This is schema/checker work and does not authorize the scientific
   run.
2. **Audit solver visibility.** For one exact small prime-order curve and
   declared factor base, independently enumerate all group-law decompositions
   and compare them target by target with solver output. Preserve the first
   chart, multiplicity, rationality or verification mismatch.
3. **Classify kernel mode.** Audit the 11 claims currently downstream of
   `HEUR-RELATION-RANK`. Mark each as target-dependent left-kernel, reusable
   right-kernel/full-rank, or a separately proved alternative.
4. **Produce one admitted immutable population.** Only after custody and
   experiment admission, retain every registered target, failure, witness,
   normalized row and rank/support prefix. Do not reuse the held historical
   batch.
5. **Evaluate yield and rank from the same population.** A positive, null,
   negative, censored or custody-blocked result is retained symmetrically.
6. **Compose the certificate.** State exactly which scales and kernel mode the
   result covers. Do not extrapolate a finite panel into an asymptotic theorem.

## Acceptance details

### Public population contract

The immutable identity must bind the curve and prime-order subgroup, factor
base and symmetry quotient, target generator, arity, solver and checker
versions, scalar field, budgets, registered scale ladder and row-normalization
rule. Every target gets a terminal record, including zero decompositions,
solver failure, timeout and verification rejection.

### Solver-visible yield

The comparison uses two independent paths:

- direct finite group-law enumeration of mathematical witnesses; and
- the production algebraic solver followed by the production relation
  verifier.

Agreement is target-wise after the preregistered sign/order quotient. Aggregate
agreement alone is insufficient because errors can cancel.

### Rank and support

The replay record contains the exact normalized coefficient row over the
declared subgroup scalar field, rank after each prefix, duplicate class, live
and dead columns, support components and incidence-core size. Results are
reported separately for the single-target and reusable-table modes.

## Non-implications

- A conserved mean is not a Poisson law.
- An exhaustive toy census is not an asymptotic theorem.
- A solver success rate is not mathematical decomposition yield unless the
  witness correspondence is checked.
- Early row independence is not terminal rank.
- More rows than columns guarantees a nonzero left kernel, not a reusable
  full-column-rank table.
- A finite reusable-rank certificate does not prove a sub-rho attack exponent;
  solver, setup, filtering, linear algebra, descent, memory and verification
  must still compose on the same family.

## Reproduction

This was a corpus and source audit, not a scientific execution. Regenerate and
check it with:

```sh
node docs/ecdlp-hardness-corpus/validate.mjs
node docs/ecdlp-hardness-corpus/render.mjs
node docs/ecdlp-hardness-corpus/attack-graph.mjs --check
node docs/ecdlp-hardness-corpus/weakness.mjs --check
dot -Tsvg docs/ecdlp-hardness-corpus/yield-rank-frontier-20261118.dot \
  -o docs/ecdlp-hardness-corpus/yield-rank-frontier-20261118.svg
```

Primary repository evidence used by this decomposition is linked through
`S-IC-SIX-OBLIGATIONS-2026`, `S-CENSUS-ROUND4` and `S-WEAKNESS-LEAN` in the
machine fragment. No experiment or performance measurement was launched.

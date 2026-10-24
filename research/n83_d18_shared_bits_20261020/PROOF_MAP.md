# Proof map: shared root-bit encoding

The proof source is [SharedRootBits.lean](lean/SharedRootBits.lean).
It uses core Lean **4.34.1**, with no package dependencies.

| Claim | Lean declaration | Hypotheses and scope |
|:--|:--|:--|
| Factoring common root bits preserves every satisfying assignment. | `N83SharedRootBits.factorization` | Arbitrary selector and bit index types; at least one selected index; every reported common bit agrees with every representative. Both Boolean values are covered. |
| The replacement is valid within the rest of the unchanged model. | `N83SharedRootBits.withinUnchangedContext` | The same assumptions, plus an arbitrary unchanged proposition for the other CNF/XOR constraints. No field, curve, sign or phase predicate is removed. |
| Removed root-implication clauses equal `m*c*(K-1)`. | `N83SharedRootBits.clauseCount` | Natural numbers, `K=k+1`, `n=v+c`. Original `m*K*n` equals factored `m*(K*v+c)` plus `m*c*k`. The non-root clauses are unchanged. |

## Readable proof

Choose an active selector, which exists because the original exactly-one
encoding contains an at-least-one clause. At a shared bit, that selector's
original implication forces the root to the common value. Thus a common
unit follows from the original constraints. Conversely, a common unit
satisfies every original implication at that bit, whether its selector is
active or inactive. At varying bits the original implications remain.
This establishes equivalence for every assignment, including the choice
wires. Conjoining the unchanged surrounding constraints preserves it.

There are `K*n` original root implications per summand. Factoring retains
`K*v` implications and `c` units. Writing `K=k+1` and `n=v+c`, distributivity
gives the checked natural-number equality in `clauseCount`; the removed
count is `c*k` per summand and `m*c*k` for m equal domains.

## Implementation correspondence

`shared_root_bits` reports a value only when every representative's bit
equals it. `add_frobenius_orbit_domain` retains the original at-least-one
and sequential at-most-one clauses, representative sorting, all variable
allocations and every phase/XOR transform. With the opt-in policy it
emits one unit for each reported shared bit and keeps selector
implications for all other bits.

The Lean theorem formalizes these Boolean predicates and the clause-count
identity. It does not verify the Rust compiler or the entire SAT solver.
The implemented bridge tests enumerate selected
representatives, valid and forbidden phase codes, and every field element
at degree 3, alongside exact native count and cap checks. The complete
field/group relation remains subject to independent general-field replay.
The native bridge tests now pass as part of the updated-parent full release
library suite (4,478 passed, zero failed, 119 ignored); all 29 affected-example
tests and the six actual full-size constructions also pass. The
[validation-v3 run report](reports/run-20261024T0603Z/RUN_REPORT.md) links the
commands, source hashes and preserved failure/rerun receipts. The later
[validation-v4 report](reports/run-20261024T0640Z/RUN_REPORT.md) adds independent
construction-resource rejection checks and repeats the full native suites.
These executable
checks supplement the Lean predicate theorem; they are not a compiler proof
or an exhaustive proof about every 83-bit SAT instance.

## Checked proof boundary

The direct Lean build and the repository proof gate pass all three
theorems. `factorization` and `withinUnchangedContext` use no axioms;
`clauseCount` uses only `propext`. There is no native evaluation,
custom axiom or admitted proof. Build/audit logs are retained with the
validation receipts. `lean4checker` is not installed on this host, so
that additional independent kernel implementation has not been run.

The native construction tool issued `identity.certificate/v1`
`IDC1hbc6bfea0a569542b` for the count polynomial and replayed it with 32 extra
checker points; see [construction.json](construction.json). That
certificate is evaluated over the format's fixed prime field; the Lean
count theorem is over natural numbers. The certificate alone does not
establish Boolean equivalence or a solver runtime improvement.

# Concrete F17 relations and canonical degree in one Lean theorem

`FixtureGroupDegree.exact_relations_and_degree` connects the actual four
polynomial generators, their complete signed factor-base group relations and
their canonical homogeneous degree **3**, alongside the inherited dense
comparator's degree **5**. Both the root and degree sides use the same named
generator set; no supplied leading-form oracle is assumed.

The existing curve is `icv1-fp5-tm1-a77cc6af`, with base `FB1hed5b19904cc8`:
over F17, y²=x³+2x+2, h(x)=4x²+9x+9 and V={5,6,10,16}. These identifiers
and the existing FB1 storage object are reused from
[the immutable fixture](../secant/fixture.json). Its registry status remains
`registered=false`; no new curve or factor-base registration is claimed.

| Obligation | Checked evidence |
| --- | --- |
| Actual polynomial complete splitting and nonsingularity | `complete_split`, `discriminant_ne_zero`, `target_nonsingular` |
| Evaluate the actual generator set into the secant system | `eval_g1`, `eval_g2`, `eval_q`, `zeroes_iff_system` |
| Exact roots iff actual signed base-pair relation | `polynomial_roots_iff_relation` |
| Same generators on root and canonical-degree sides | `canonical_top_ideal`, `exact_relations_and_degree` |
| Every sign, tangent and repeated abscissa | `all_lifts_polynomial_complete`, inherited generic equivalence |
| Identity target | `identity_relations_iff`, separate from finite coordinates |
| Full transitive axiom census | 159 declarations across fourteen modules; zero failures, zero native trust |
| Prior published-source recheck | Three modules rebuilt; warnings-as-errors passed; all 22 declarations audited |

See [the statement and proof map](FORMALIZATION.md),
[illustrated report](reports/REPORT.md), [PDF](reports/report.pdf),
[verification metadata](verification/metadata.json) and
[the complete audit](verification/final/axioms.log).

## Reproduce

Install the pinned Lean toolchain, then run from this directory:

```sh
bash verify.sh /tmp/secant-fixture-receipts
```

The script checks thirty source/dependency inputs, builds the project, checks
the new module with warnings as errors, and runs the unchanged repository
transitive-axiom auditor over all fourteen source modules, including both
inherited projects. Its receipt records the exit code even on failure.
All nine Git dependency revisions match the lockfile. The two repository
dependencies use committed relative paths, without changes to their sealed
sources. Their duplicate SecantIdentity sources are byte-identical.

The native worker's existing Mathlib checkout supplied already compiled
dependencies at the exact pinned revisions. The published fixture sources
and project modules were rebuilt and checked. No successful cache-download
preflight or unavailable `lean4checker --fresh` result is claimed.

## Scope

The degree is the least homogeneous degree at which the quotient by the
canonical highest-form ideal vanishes. It is not affine F4 solving degree.
The dense side reuses `CanonicalFixture.degree_five_to_three`; this
continuation does not prove a new independent dense-to-group equivalence.
It adds a checked semantic link for the secant presentation and a single
combined statement, rather than a new numerical degree reduction or timing.

No relation search, solver benchmark or performance comparison was run.
Existing numerical diagrams, comparison dashboards and leaderboard rows stay
unchanged because no new measured figure is introduced. Existing sealed
records remain historical. This continuation closes the published-source
replay and axiom-audit gaps recorded in merged #3095.

The copied [identity certificate](identity.json), `IDC1h621ed1a51b5f3ca8`,
retains its original evaluated-tier status; the Lean source supplies the
universal algebraic and group proofs. Raw failed proof-development attempts
are retained in a deterministic archive beside the successful receipts.

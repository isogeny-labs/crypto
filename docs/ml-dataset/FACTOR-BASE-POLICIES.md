# Explicit factor-base policies

Omitting `--factor-base-policies` preserves plan/v1 and the original projected
small-x base. The frozen 720-cell pilot remains unchanged. To prepare a
separate plan/v2, save a JSON array such as:

```json
[
  {"kind": "legacy"},
  {"kind": "sampled_x_projected_control", "candidate_limit": 256},
  {"kind": "binary_subspace_intersection", "basis": [1, 2, 4], "candidate_limit": 8},
  {"kind": "binary_frobenius_union_intersection", "basis": [1, 2, 4], "candidate_limit": 1024}
]
```

Then freeze a plan with the native CLI:

```sh
crypto dataset plan --out policy-plan.json --factor-base-policies policies.json \
  --n 23-31,47-61 --m 2-15 --families koblitz,binary,prime \
  --solvers enumerate,bsgs,sat,weil,f4,f5,f6,root,pkm,double-large-prime
```

Planning performs no cryptography and launches no jobs. Production generation
still requires a separately frozen protocol, source identity, bounded batch,
resource envelope and publication budget. A recipe file cannot amend an
existing pilot. The CPU/S3 campaign's USD200 total ceiling remains unchanged.

The solver axis varies fastest, then policy, then twist and the existing
arity/key/generator/curve/field/family axes. Policy choices do not perturb the
curve, generator or target draws. `candidate_limit` is bounded at 65,536;
bases contain at most 12 independent binary basis masks, each a canonical
field element. Invalid or repeated recipes fail before generation.

The control samples x coordinates in an independent deterministic stream and
uses prime-primary projection plus actual cyclic membership. Its distribution
is a sampled projected image, not a uniformly sampled set of subgroup points.
The two subspace policies preserve literal x coordinates and retain only
points in the selected generator's cyclic subgroup. They enumerate their
declared domain subject to explicit caps; `factor_base_size` is not their
mathematical dimension or guaranteed cardinality. Frobenius unions use the
curve model's recorded Frobenius power and admit whole sign/orbit units.
Prime curves produce an explicit unsupported record for a binary policy.

The saved factor-base/v2 artifact binds the recipe, exact ordered points,
actions, candidates, construction counters and status. Complete, capped and
unsupported populations are distinct. Equal requested sizes or dimensions do
not imply equal actual points, signed columns or solver cost. Ordinary solver
labels bind their actual saved point domain; PKM and large-prime adapters keep
their separately verified transformed domains.

`crypto dataset verify` checks the policy against the frozen cell config and
replays the construction. Training exports allowlist the public recipe,
points and actions, keep construction counters/status as restricted labels,
and exclude seeds, logs and candidate history from model inputs. Unsupported
policies have provenance and labels but no matched solver-domain feature row.
Coverage reports add explicit policy axes and construction records only for
policy-aware cells. All variants stay in the existing whole-field-size split.

These policies enable future comparisons. Their implementation alone provides
no evidence that a base lowers degree, beats another backend, or proves a
first/last-fall-degree bound. See the [implementation design](../../research/ml_dataset_policies_20261028/DESIGN.md).

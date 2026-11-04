"""Complete deterministic metadata schedule; no numerical imports or trials."""

ARMS = ('candidate', 'aa')


def cell(plan, instance, arm):
    assert arm in ARMS
    n = plan.get('N', plan['r'])
    trailing = n - plan['cut']
    assert 0 < plan['cut'] < n
    basis = range(trailing) if trailing <= 19 else (0, trailing - 1)
    labels = ['zero', 'ones', 'seeded'] + ['basis-' + str(j) for j in basis]
    return dict(arm=arm, instance=str(instance), replicate=0,
                seed=plan['seed'], source_kind=plan['kind'], dimension=n,
                cut=plan['cut'], vector_seed=plan['seed'] + 991,
                lift_seed=plan['seed'] + 2003,
                directional_cases=[dict(label=label, transpose=tr)
                                   for label in labels for tr in (False, True)],
                valid_lift_labels=['zero', 'ones', 'seeded'],
                normalized_rule='append normalized iff reference S transpose has nonzero nullspace',
                obstruction_adapters=['rhs', 'primal', 'dual'])


def expanded(plan):
    assert len(plan) == 321
    assert sum(p['kind'] == 'source' for p in plan) == 81
    assert sum(p['kind'] == 'synthetic' for p in plan) == 240
    return [cell(p, i, arm) for arm in ARMS for i, p in enumerate(plan)]

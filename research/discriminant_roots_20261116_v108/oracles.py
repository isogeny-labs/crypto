import json, time, random, itertools, collections, hashlib
from .common import P, finish
from .kernel import Field, solve, gcd, derivative, delta_data, binary_irreducible, roots_product, product, Arithmetic
START = time.monotonic()
out = {'tiny_cases': 0, 'branches': {}, 'field_checks': [], 'original_complete': False}
error = None
try:
    d = json.loads((P / 'DATA.json').read_text())
    rng = random.Random(d['seed'])
    branches = collections.Counter()
    for p, mod, full in [(2, [0, 1], True), (2, [1, 1, 1], True), (2, [1, 1, 0, 1], False), (2, [1, 1, 0, 0, 1], False), (3, [0, 1], False), (3, [1, 0, 1], False), (5, [0, 1], False)]:
        f = Field(p, mod)
        base = Arithmetic(p, mod)
        cases = itertools.product(range(f.q), repeat=5) if full else [[rng.randrange(f.q) for _ in range(5)] for _ in range(64)]
        for coeff in cases:
            H = list(coeff) + [1]
            expected = []
            for x in range(f.q):
                a = list(H)
                count = 0
                while len(a) > 1:
                    value = 0
                    for c in reversed(a):
                        value = base.add(base.mul(value, x), c)
                    if value:
                        break
                    Q = [0] * (len(a) - 1)
                    Q[-1] = a[-1]
                    for i in range(len(a) - 2, 0, -1):
                        Q[i - 1] = base.add(a[i], base.mul(x, Q[i]))
                    a = Q
                    count += 1
                if count:
                    expected.append([x, count])
            for method in ['direct', 'radical', 'initial'] + (['dispatch', 'dispatch_initial'] if p == 2 else []):
                r = solve(H, f, method, d['seed'])
                assert r['roots'] == expected and r['complete']
                branches[r['branch']] += 1
            if p == 2:
                assert bool(delta_data(H, f)[2]) == (len(gcd(H, derivative(H, f), f)) == 1)
            out['tiny_cases'] += 1
    panel = {'binary': [], 'odd': []}
    for group in ['binary', 'odd']:
        for spec in d[group + '_fields']:
            f = Field(spec['p'], spec['mod'])
            base = Arithmetic(spec['p'], spec['mod'])
            if f.p == 2:
                assert binary_irreducible(f)
            for _ in range(12):
                a = rng.randrange(1, f.q)
                b = rng.randrange(f.q)
                assert f.mul(a, b) == base.mul(a, b) and f.sq(a) == base.sq(a) and (f.mul(a, f.inv(a)) == 1)
            xs = []
            while len(xs) < 5:
                value = rng.randrange(f.q)
                if value not in xs:
                    xs.append(value)
            t, a, b, c, e = xs
            cases = [('squarefree', roots_product(xs, f)), ('single_repeat', roots_product([t, t, a, b, c], f)), ('max_repeat', roots_product([t, a, a, b, b], f))]
            if f.p == 2:
                cases.append(('nonrational', product([t, 1], product([1, 1, 1], [1, 1, 1], f), f)))
            else:
                cases.append(('five_repeat', roots_product([t] * 5, f)))
            panel[group].append({'field': spec, 'cases': [{'name': name, 'H': H} for name, H in cases]})
            out['field_checks'].append({'name': spec['name'], 'p': f.p, 'degree': f.n, 'binary_irreducible_verified': f.p == 2})
    out['branches'] = dict(branches)
    raw = json.dumps(panel, separators=(',', ':')).encode()
    assert hashlib.sha256(raw).hexdigest() == d['expected_panel_sha256']
    (P / 'PANEL.json').open('xb').write(raw)
    finish(out, None, START, 'Independent literal all-roots/multiplicity oracles; exact discriminant branches, arithmetic checks and full-width field fixtures.', 'Freeze cold panels and compare against both direct Frobenius-GCD and standard radical preprocessing; no GB ratio.')
except Exception as e:
    finish(out, repr(e), START, 'Discriminant all-roots kernel oracle.', 'Preserve failure and change a concrete assumption in a new variant.')

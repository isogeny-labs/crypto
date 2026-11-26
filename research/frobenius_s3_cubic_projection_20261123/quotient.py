"""Check quotient S3 coordinates, solving degrees, and linear span."""
import argparse
import json
from heapq import heappop, heappush
from pathlib import Path
from random import Random

from profile import domains, factors, geometry, rank_f2, span, xor_selected
from selector_elimination import normalized, run_ranker, trace_consequence_span, witness_coefficients


def linear_map(field, evaluate):
    images = [evaluate(1 << j) for j in range(field.n)]
    return lambda word: xor_selected(images, word)


def half_trace(field, w):
    value = w
    for _ in range((field.n - 1) // 2):
        value = field.sq(field.sq(value)) ^ w
    return value


def remainder(word, pivots):
    for j in sorted(pivots, reverse=True):
        if word >> j & 1:
            word ^= pivots[j]
    return word


def insert(word, pivots):
    word = remainder(word, pivots)
    if word:
        pivots[word.bit_length() - 1] = word
    return bool(word)


def quotient_rows(field, basis, trace, half, constant, linear):
    z = [(1 << i) ^ trace(1 << i) for i in range(1, field.n)]
    pivots = {}
    for u in basis:
        insert(u, pivots)
    project = linear_map(field, lambda value: remainder(value, pivots))
    link = [{0} if constant >> i & 1 else set() for i in range(field.n)]
    membership = [set() for _ in range(field.n)]
    for j, s in enumerate(basis):
        selector = 1 << (len(z) + j)
        word = linear[j]
        for bit in range(field.n):
            if word >> bit & 1:
                link[bit].add(selector)
        for i, w in enumerate(z):
            m = selector | (1 << i)
            product = field.mul(field.sq(s), w)
            x_image = project(field.mul(s, half(w)))
            for bit in range(field.n):
                if product >> bit & 1:
                    link[bit].add(m)
                if x_image >> bit & 1:
                    membership[bit].add(m)
    return [sorted(row) for row in (*link, *membership) if row]


def expected_points(field, basis, trace, half, constant, linear):
    k = len(basis)
    values = span(basis)
    lookup = {x: i for i, x in enumerate(values)}
    original, lifted = set(), set()
    for selector, s in enumerate(values):
        rhs = constant ^ xor_selected(linear, selector)
        columns = [field.sq(x) ^ field.mul(x, s) for x in basis]
        rank = rank_f2(columns)
        if rank_f2([*columns, rhs]) != rank:
            continue
        if not s:
            return None
        assert rank == k - 1
        w = field.mul(rhs, field.inv(field.sq(s)))
        assert not trace(w)
        x = field.mul(s, half(w))
        assert x in lookup and field.sq(x) ^ field.mul(s, x) == rhs
        code = lookup[x]
        original.update({code | (selector << k), (code ^ selector) | (selector << k)})
        lifted.add((w >> 1) | (selector << (field.n - 1)))
    return original, lifted


def point_basis(points, width):
    """Reduced degree-reverse-lexicographic basis by evaluation at the points."""
    points = sorted(points)
    def order(m):
        return sum(m), tuple(-value for value in reversed(m))
    def mask(m):
        return sum(1 << i for i, exponent in enumerate(m) if exponent)
    one = (0,) * width
    queue = [(order(one), one)]
    seen = {one}
    standards, leads, basis, pivots = [], [], [], {}
    while queue:
        _, m = heappop(queue)
        if any(all(a >= b for a, b in zip(m, head)) for head in leads):
            continue
        word = mask(m)
        evals = sum(1 << i for i, x in enumerate(points) if x & word == word)
        lead = evals.bit_length() - 1
        recipe = 0
        while evals and lead in pivots:
            image, source = pivots[lead]
            evals ^= image
            recipe ^= source
            lead = evals.bit_length() - 1
        if not evals:
            leads.append(m)
            terms = {word}
            for i, standard in enumerate(standards):
                if recipe >> i & 1:
                    terms.symmetric_difference_update((mask(standard),))
            basis.append((sum(m), sorted(terms)))
            continue
        pivots[lead] = (evals, recipe ^ (1 << len(standards)))
        standards.append(m)
        for i in range(width):
            candidate = m[:i] + (m[i] + 1,) + m[i + 1:]
            if candidate not in seen:
                seen.add(candidate)
                heappush(queue, (order(candidate), candidate))
    assert len(standards) == len(points)
    return basis


def first_fall(rows, width):
    grades = [max(m.bit_count() for m in row) for row in rows]
    top = [[m for m in row if m.bit_count() == g] for row, g in zip(rows, grades)]
    homogeneous = {d: [m for m in domains(width, d) if m.bit_count() == d] for d in range(4)}
    for d in range(1, 4):
        sources = [(i, m) for i, g in enumerate(grades) for m in homogeneous.get(d - g, ())]
        slots = {source: j for j, source in enumerate(sources)}
        target = {m: j for j, m in enumerate(homogeneous[d])}
        image = [sum(1 << target[m | t] for t in top[i] if not m & t) for i, m in sources]
        kernel_dimension = len(sources) - rank_f2(image)
        def trivial():
            for i, g in enumerate(grades):
                for j in range(i, len(grades)):
                    for m in homogeneous.get(d - g - grades[j], ()):
                        row = sum(1 << slots[j, m | t] for t in top[i] if not m & t)
                        if i != j:
                            row ^= sum(1 << slots[i, m | t] for t in top[j] if not m & t)
                        yield row
        if kernel_dimension > rank_f2(trivial()):
            return d
    return None


def measures(binary, rows, points, k, left, *, max_degree=4):
    width = k + left
    basis = point_basis(points, width)
    hits, caps = {}, []
    for d in range(max_degree + 1):
        missing = [(i, row) for i, (degree, row) in enumerate(basis) if i not in hits and degree <= d]
        if not missing:
            continue
        active = [r for r in rows if max(m.bit_count() for m in r) <= d]
        if d < 2:
            pivots = {}
            monomials = {m: i for i, m in enumerate(domains(width, d))}
            encode = lambda row: sum(1 << monomials[m] for m in row)
            for row in active:
                insert(encode(row), pivots)
            members = [not remainder(encode(row), pivots) for _, row in missing]
        else:
            result = run_ranker(binary, active, left, d, 0, 45, True, right=k,
                                checks=[row for _, row in missing])
            members = result['members']
            caps.append(dict(degree=d, columns=result['columns'], timeout=result['timeout'],
                             row_xors=result['row_xors']))
        hits.update({i: d for (i, _), known in zip(missing, members) if known})
        if len(hits) == len(basis) or (d >= 2 and result['timeout']):
            break
    full = len(hits) == len(basis)
    return dict(first_fall=first_fall(rows, width), first_fall_cap=3,
                reduced_basis_degree=max(d for d, _ in basis),
                last_fall=max((hits[i] for i, (d, _) in enumerate(basis) if hits[i] > d), default=0) if full else None,
                solving_degree=max(hits.values()) if full else None, caps=caps)


def satisfies(rows, word):
    return all(not (sum(m & word == m for m in row) & 1) for row in rows)


def degree_panel(binary):
    for n in (7, 23):
        field, basis, _ = geometry(n, factors(n)[0])
        trace = linear_map(field, field.tr)
        half = linear_map(field, lambda w: half_trace(field, w))
        k = len(basis)
        targets = (range(1, 1 << n) if n == 7 else (6277527, 5846863, 7373655, 5025355))
        for t in targets:
            if not trace(t) or trace(field.sq(field.inv(t))):
                continue
            rows, constant, linear = normalized(field, basis, t)
            assert constant
            points = expected_points(field, basis, trace, half, constant, linear)
            if points is None:
                continue
            lifted = quotient_rows(field, basis, trace, half, constant, linear)
            for name, active, roots, left in (('original', rows, points[0], k), ('quotient', lifted, points[1], n-1)):
                assert all(satisfies(active, root) for root in roots)
                if n == 7:
                    assert roots == {x for x in range(1 << (k + left)) if satisfies(active, x)}
                print(json.dumps(dict(n=n, k=k, target=t, arm=name, variables=k+left, roots=len(roots),
                                      **measures(binary, active, roots, k, left))), flush=True)


def witness_panel(binary):
    path = Path(__file__).with_name('unrestricted-quartic-ranks.jsonl')
    for entry in map(json.loads, path.read_text().splitlines()):
        field, basis, _ = geometry(47, entry['g'])
        trace = linear_map(field, field.tr)
        half = linear_map(field, lambda w: half_trace(field, w))
        witness = entry['false_positive']
        _, constant, linear = normalized(field, basis, witness['target_x'])
        rows = quotient_rows(field, basis, trace, half, constant, linear)
        selector = witness_coefficients(basis, witness['sum_x'])
        assert constant ^ xor_selected(linear, selector) == witness['normalized_rhs']
        result = run_ranker(binary, rows, 46, 3, selector, 90, True, right=23)
        selector_rows = result.pop('selector_rows')
        result.pop('members')
        if not result['timeout']:
            trace_rows, index = trace_consequence_span(field, basis, constant, linear)
            measured = [sum(1 << index[m] for m in row) for row in selector_rows]
            result['two_trace_span_rank'] = rank_f2(trace_rows)
            result['union_rank'] = rank_f2([*trace_rows, *measured])
        print(json.dumps(dict(n=47, g=entry['g'], width=69, **result)), flush=True)


def span_panel():
    for n in (23, 47, 71, 79):
        for g in factors(n):
            field, basis, _ = geometry(n, g)
            trace = linear_map(field, field.tr)
            rng = Random(g ^ 0x9531)
            pivots, certificate, tested = {}, [], 0
            for _ in range(1000):
                u, v = (xor_selected(basis, rng.getrandbits(len(basis))) for _ in range(2))
                s = u ^ v
                if not s:
                    continue
                x0, x1 = u ^ 1, v ^ 1
                if any(trace(x ^ 1 ^ field.sq(field.inv(x))) for x in (x0, x1)):
                    continue
                tested += 1
                y = field.mul(u, field.inv(s))
                w = field.sq(y) ^ y
                assert not trace(w)
                if insert(w, pivots):
                    certificate.append([x0, x1, w])
                if len(pivots) == n - 1:
                    break
            assert len(pivots) == n - 1
            print(json.dumps(dict(n=n, g=g, rank=len(pivots), rational_pairs=tested, certificate=certificate)), flush=True)


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('check', choices=('degrees', 'witnesses', 'spans'))
    parser.add_argument('--ranker', type=Path)
    args = parser.parse_args()
    if args.check == 'spans':
        span_panel()
    else:
        if args.ranker is None:
            parser.error('--ranker is required for polynomial checks')
        (degree_panel if args.check == 'degrees' else witness_panel)(args.ranker.resolve())

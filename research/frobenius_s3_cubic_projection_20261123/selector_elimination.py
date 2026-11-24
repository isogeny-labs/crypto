"""Test raw Macaulay and bounded-closure separation of the saved S3 witnesses."""
import argparse
import json
from pathlib import Path
import subprocess

from profile import domains, geometry, rank_f2, xor_selected


def witness_coefficients(basis, value):
    pivots = {}
    for i, word in enumerate(basis):
        recipe = 1 << i
        while word:
            leading = word.bit_length() - 1
            if leading not in pivots:
                pivots[leading] = word, recipe
                break
            old, source = pivots[leading]
            word ^= old
            recipe ^= source
    mask = 0
    while value:
        word, source = pivots[value.bit_length() - 1]
        value ^= word
        mask ^= source
    return mask


def normalized(field, basis, target):
    def half(value):
        result = value
        for _ in range((field.n - 1) // 2):
            result = field.sq(field.sq(result)) ^ value
        return result

    start = field.mul(target, half(field.sq(field.inv(target))))
    const = start ^ (target if not field.tr(start) else 0) ^ 1
    p_linear = []
    for u in basis:
        w = field.mul(target, half(field.sq(u)))
        p_linear.append(w ^ (target if field.tr(w) else 0) ^ u)
    k = len(basis)
    words = [(0, const)]
    for i, u in enumerate(basis):
        words.append((1 << i, field.sq(u)))
        words.append((1 << (k + i), p_linear[i]))
        for j, v in enumerate(basis):
            words.append(((1 << i) | (1 << (k + j)), field.mul(u, v)))
    rows = []
    for bit in range(field.n):
        terms = [m for m, w in words if w >> bit & 1]
        if terms:
            rows.append(terms)
    return rows, const, p_linear



def run_ranker(binary, equations, k, degree, witness, limit, closure):
    rows = [f'{k} {degree} {int(closure)} {len(equations)} {witness} {limit}']
    rows.extend(' '.join(map(str, [len(row), *row])) for row in equations)
    result = subprocess.run([binary], input='\n'.join(rows) + '\n', text=True,
                            capture_output=True, check=True, timeout=limit + 60)
    return json.loads(result.stdout)


def controls(binary):
    # xy and xy+x expose x. Multiplying by w then reduces xw+y+1 to y+1.
    equations = [[5], [5, 1], [9, 4, 0]]
    for witness in (0, 1):
        raw = run_ranker(binary, equations, 2, 2, witness, 5, False)
        closed = run_ranker(binary, equations, 2, 2, witness, 5, True)
        assert not raw['timeout'] and not closed['timeout']
        assert not raw['rejected'] and closed['rejected'] == (witness == 0)


def chart(rows, witness, k):
    pivot = (witness & -witness).bit_length() - 1
    x, s = 1 << pivot, 1 << (k + pivot)
    remaining = [i for i in range(2 * k) if not ((1 << i) & (x | s))]

    def pack(mask):
        return sum(1 << j for j, i in enumerate(remaining) if mask >> i & 1)

    output = []
    for row in rows:
        terms = set()
        for m in row:
            if not m & x:
                terms.symmetric_difference_update((pack(m),))
        if terms:
            output.append(sorted(terms))
    return output, pack(witness << k) >> (k - 1), pivot


def trace_consequence_span(field, basis, constant, linear):
    k = len(basis)
    monomials = domains(k, 3)
    index = {m: j for j, m in enumerate(monomials)}
    values = {}
    for mask in monomials:
        s = xor_selected(basis, mask)
        p = constant ^ xor_selected(linear, mask)
        a2 = field.mul(s, field.sq(s)) ^ field.pow(s, 1 << (field.n - 1))
        values[mask] = field.tr(field.mul(s, p)) | (field.tr(field.mul(a2, p)) << 1)
    for i in range(k):
        bit = 1 << i
        for mask in monomials:
            if mask & bit:
                values[mask] ^= values[mask ^ bit]
    first = [m for m in monomials if values[m] & 1]
    assert all(m.bit_count() <= 2 for m in first)
    span = [sum(1 << index[m] for m in monomials if values[m] & 2)]
    for variable in [0, *(1 << i for i in range(k))]:
        row = 0
        for m in first:
            row ^= 1 << index[m | variable]
        span.append(row)
    return span, index


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--ranker', type=Path, required=True)
    parser.add_argument('--degree', type=int, choices=(3, 4), required=True)
    parser.add_argument('--closure', action='store_true')
    parser.add_argument('--chart', action='store_true')
    parser.add_argument('--limit', type=int, default=150)
    args = parser.parse_args()
    binary = args.ranker.resolve()
    controls(binary)
    path = Path(__file__).with_name('unrestricted-quartic-ranks.jsonl')
    for entry in map(json.loads, path.read_text().splitlines()):
        field, basis, _ = geometry(entry['n'], entry['g'])
        w = entry['false_positive']
        rows, constant, linear = normalized(field, basis, w['target_x'])
        witness = witness_coefficients(basis, w['sum_x'])
        assert constant ^ xor_selected(linear, witness) == w['normalized_rhs']
        width, pivot = len(basis), None
        if args.chart:
            rows, witness, pivot = chart(rows, witness, width)
            width -= 1
        result = run_ranker(binary, rows, width, args.degree, witness, args.limit, args.closure)
        selector_rows = result.pop('selector_rows')
        if args.closure and args.degree == 3 and not args.chart and not result['timeout']:
            trace_rows, index = trace_consequence_span(field, basis, constant, linear)
            measured = [sum(1 << index[m] for m in row) for row in selector_rows]
            assert rank_f2(measured) == result['selector_rank']
            result['two_trace_span_rank'] = rank_f2(trace_rows)
            result['union_rank'] = rank_f2([*trace_rows, *measured])
        print(json.dumps(dict(n=entry['n'], g=entry['g'], degree=args.degree, closure=args.closure,
                              chart_pivot=pivot, variables=2 * width, **result)), flush=True)


if __name__ == '__main__':
    main()

"""Exact structural matching/SCC audit; scientific runs only through lab.runner."""
import gzip
import hashlib
import itertools
import json
from pathlib import Path
import resource
import sys
import time

INPUT = Path('research/regular_block_20261115_v2/raw-profile.json.gz')
INPUT_SHA = 'e98420402585b5a00b35fbc938bb47d1669169db82c0d38ed40dbc43d47e122d'
START = time.monotonic()
PREDICATES = 0


def check(condition, message):
    global PREDICATES
    PREDICATES += 1
    if time.monotonic() - START > 80:
        raise TimeoutError('80-second work deadline')
    if not condition:
        raise AssertionError(message)


class Field:
    def __init__(self, p, modulus):
        self.p, self.modulus = p, tuple(modulus)
        self.n = len(modulus) - 1
        self.q = p ** self.n

    def digits(self, x):
        a = []
        for _ in range(self.n):
            a.append(x % self.p)
            x //= self.p
        return a

    def pack(self, a):
        return sum((x % self.p) * self.p ** i for i, x in enumerate(a))

    def add(self, a, b):
        return self.pack([x + y for x, y in zip(self.digits(a), self.digits(b))])

    def mul(self, a, b):
        aa, bb = self.digits(a), self.digits(b)
        c = [0] * (2 * self.n - 1)
        for i, x in enumerate(aa):
            for j, y in enumerate(bb):
                c[i + j] = (c[i + j] + x * y) % self.p
        for j in range(len(c) - 1, self.n - 1, -1):
            t = c[j]
            for i in range(self.n + 1):
                c[j - self.n + i] = (c[j - self.n + i] - t * self.modulus[i]) % self.p
        return self.pack(c[:self.n])

    def power(self, a, n):
        r = 1
        while n:
            if n & 1:
                r = self.mul(r, a)
            a = self.mul(a, a)
            n >>= 1
        return r


def canonical(equations, field):
    out = []
    for equation in equations:
        merged = {}
        for coefficient, powers in equation:
            powers = tuple(powers)
            merged[powers] = field.add(merged.get(powers, 0), coefficient)
        out.append([(v, k) for k, v in sorted(merged.items()) if v])
    return out


def support(equations):
    return [set(j for _, powers in equation for j, power in enumerate(powers) if power)
            for equation in equations]


def decomposition(incidence, n):
    """Matching, then matched-variable SCC graph; unmatched rows stay explicit."""
    variable_row = {}

    def augment(row, seen):
        for v in sorted(incidence[row]):
            if v in seen:
                continue
            seen.add(v)
            if v not in variable_row or augment(variable_row[v], seen):
                variable_row[v] = row
                return True
        return False

    for row in range(len(incidence)):
        augment(row, set())
    row_variable = {r: v for v, r in variable_row.items()}
    outer = sorted(set(range(n)) - set(variable_row))
    graph = {v: set() for v in variable_row}
    for row, pivot in row_variable.items():
        for v in incidence[row] - {pivot}:
            if v in graph:
                graph[v].add(pivot)
    seen, finish = set(), []

    def visit(v):
        seen.add(v)
        for w in sorted(graph[v]):
            if w not in seen:
                visit(w)
        finish.append(v)

    for v in sorted(graph):
        if v not in seen:
            visit(v)
    reverse = {v: set() for v in graph}
    for v in graph:
        for w in graph[v]:
            reverse[w].add(v)
    seen, components = set(), []

    def collect(v, comp):
        seen.add(v)
        comp.append(v)
        for w in sorted(reverse[v]):
            if w not in seen:
                collect(w, comp)

    for v in reversed(finish):
        if v not in seen:
            comp = []
            collect(v, comp)
            components.append(sorted(comp))
    positions = {v: i for i, block in enumerate(components) for v in block}
    for v in graph:
        for w in graph[v]:
            check(positions[v] <= positions[w], 'nontriangular SCC ordering')
    return dict(matching_size=len(variable_row), matched_row_variable=sorted(row_variable.items()),
                unmatched_variables=outer,
                unmatched_rows=sorted(set(range(len(incidence))) - set(row_variable)),
                blocks=components, block_sizes=[len(c) for c in components],
                largest_block=max(map(len, components), default=0),
                directed_dependency_edges=sum(map(len, graph.values())),
                incidence_nonzeros=sum(map(len, incidence)))


def evaluate(equation, assignment, field, repeated=False):
    value = 0
    for coefficient, powers in equation:
        term = coefficient
        for v, power in enumerate(powers):
            if repeated:
                for _ in range(power):
                    term = field.mul(term, assignment[v])
            elif power:
                term = field.mul(term, field.power(assignment[v], power))
        value = field.add(value, term)
    return value


def block_roots(raw, n, field):
    equations = canonical(raw, field)
    incidence = support(equations)
    shape = decomposition(incidence, n)
    roots = set()

    def ready_ok(assignment):
        assigned = set(assignment)
        for equation, variables in zip(equations, incidence):
            if variables <= assigned and evaluate(equation, assignment, field) != 0:
                return False
        return True

    def branch(index, assignment):
        if not ready_ok(assignment):
            return
        if index == len(shape['blocks']):
            check(len(assignment) == n, 'lost free variable')
            roots.add(tuple(assignment[j] for j in range(n)))
            return
        block = shape['blocks'][index]
        for values in itertools.product(range(field.q), repeat=len(block)):
            check(True, 'block work')
            child = dict(assignment)
            child.update(zip(block, values))
            branch(index + 1, child)

    for values in itertools.product(range(field.q), repeat=len(shape['unmatched_variables'])):
        branch(0, dict(zip(shape['unmatched_variables'], values)))
    return roots, shape


def dense_roots(raw, n, field):
    result = set()
    for values in itertools.product(range(field.q), repeat=n):
        check(True, 'dense oracle work')
        if all(evaluate(eq, values, field, repeated=True) == 0 for eq in raw):
            result.add(values)
    return result


def controls():
    records = []
    # Monic irreducibles: X, X^2+X+1, X^3+X+1, X^2+1 over the stated primes.
    for p, modulus in [(2, [0, 1]), (3, [0, 1]), (2, [1, 1, 1]),
                       (2, [1, 1, 0, 1]), (3, [1, 0, 1])]:
        f = Field(p, modulus)
        for a in range(f.q):
            check(f.power(a, f.q) == a, 'field Frobenius')
            if a:
                check(f.mul(a, f.power(a, f.q - 2)) == 1, 'field inverse')
            for b in range(f.q):
                for c in range(f.q):
                    check(f.mul(a, f.add(b, c)) == f.add(f.mul(a, b), f.mul(a, c)),
                          'distributivity')
        minus = p - 1
        fixtures = [
            ('frobenius_dependency', 2, [[(1, (p, 0)), (1, (0, 1))], [(1, (0, 1))]]),
            ('singular_pivot', 2, [[(1, (1, 1))], [(1, (0, 1))]]),
            ('all_free', 2, []),
            ('contradictory_zero_row', 2, [[], [(1, (0, 0))]]),
            ('triangular', 3, [[(1, (2, 0, 0)), (minus, (1, 0, 0))],
                              [(1, (0, 1, 0)), (minus, (1, 0, 0))],
                              [(1, (0, 0, 1)), (minus, (0, 1, 0))]]),
            ('coupled_cycle', 2, [[(1, (2, 0)), (1, (0, 1))],
                                  [(1, (0, 2)), (1, (1, 0))]]),
            ('cancelled_support', 2, [[(1, (1, 0)), (minus, (1, 0)), (1, (0, 1))]]),
            ('residual_contradiction', 2, [[(1, (1, 0)), (1, (0, 1))],
                                          [(1, (1, 0))],
                                          [(1, (0, 1)), (1, (0, 0))]]),
            ('one_equation_free_border', 3, [[(1, (1, 0, 0)), (1, (0, 1, 0)),
                                              (1, (0, 0, 1))]])]
        for name, n, raw in fixtures:
            found, shape = block_roots(raw, n, f)
            oracle = dense_roots(raw, n, f)
            check(found == oracle, 'all-roots mismatch ' + name)
            records.append(dict(field_order=f.q, prime=p, modulus=modulus,
                                fixture=name, roots=sorted(found), shape=shape))
    return records


def actual_audit():
    data = INPUT.read_bytes()
    check(hashlib.sha256(data).hexdigest() == INPUT_SHA, 'frozen evidence input changed')
    raw = json.loads(gzip.decompress(data))
    check(raw['error'] is None and raw['mode'] == 'profile', 'inadmissible evidence')
    audits = []
    for record in raw['out']['records']:
        rows = record['rows']
        check(len(rows) == 166, 'unexpected row count')
        check(all(type(mask) is int and 0 <= mask < (1 << 83) and rhs in (0, 1)
                  for mask, rhs in rows), 'invalid binary rows')
        incidence = [{j for j in range(83) if (mask >> j) & 1} for mask, _ in rows]
        shape = decomposition(incidence, 83)
        family = record['family']
        check(family['consistent'], 'only consistent frozen profile admitted')
        check(all(((mask & family['particular']).bit_count() & 1) == rhs
                  for mask, rhs in rows), 'recorded particular replay')
        for vector in family['basis']:
            check(all((mask & vector).bit_count() % 2 == 0 for mask, _ in rows),
                  'recorded kernel replay')
        for row in shape['unmatched_rows']:
            if not rows[row][0]:
                check(rows[row][1] == 0, 'unmatched contradiction')
        audits.append(dict(rep=record['rep'], shape=shape,
                           active_rows=sum(bool(mask) for mask, _ in rows),
                           recorded_rank=family['rank'], recorded_nullity=family['nullity'],
                           recorded_family_replayed=True,
                           affine_family_recomputed=False,
                           active_density=sum(len(s) for s in incidence) /
                           (83 * max(1, sum(bool(mask) for mask, _ in rows)))))
    return dict(input_sha256=INPUT_SHA, records=audits,
                original_339_332_encoder_audited=False,
                neighboring_parameter_delta_rank='UNOBSERVED', schur_fill='UNOBSERVED',
                global_original_complete=False, original_root_count_exact=None)


def main():
    eid = sys.argv[1]
    result = dict(schema='scientific_result_v1', experiment_id=eid, status='inconclusive',
                  summary='', evidence={}, next_actions=[])
    raw = dict(controls=None, actual=None, error=None, predicates=0)
    try:
        raw['controls'] = controls()
        raw['actual'] = actual_audit()
        result['status'] = 'passed'
        result['summary'] = 'Exact matched SCC decomposition passed tiny complete-root oracles and audited frozen83-column incidence.'
    except Exception as exc:
        raw['error'] = type(exc).__name__ + ': ' + str(exc)
        result['summary'] = raw['error']
    raw['predicates'] = PREDICATES
    artifact = Path(__file__).with_name('raw-controls.json.gz')
    with artifact.open('xb') as stream:
        stream.write(gzip.compress(json.dumps(raw, sort_keys=True).encode(), mtime=0))
    raw_sha = hashlib.sha256(artifact.read_bytes()).hexdigest()
    result['evidence'] = dict(controls=len(raw['controls'] or []), actual=raw['actual'],
                              predicates=PREDICATES, global_original_complete=False,
                              original_root_count_exact=None, qualified_speedup=None,
                              raw_sha256=raw_sha,
                              wall_seconds=time.monotonic() - START,
                              self_peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,
                              memory_scope='self peak RSS, not aggregate process tree')
    result['next_actions'] = ['Measure neighboring-parameter delta ranks and Schur fill before proposing update reuse.',
                              'Audit exact original chain incidence with a frozen encoder; retain all residual constraints.']
    # Raw output/hash and complete final-result serialization are paid before admission.
    serialized = json.dumps(result, sort_keys=True)
    if time.monotonic() - START > 84:
        result['status'] = 'inconclusive'
        result['summary'] = '84-second reporting deadline'
        result['evidence']['actual'] = None
        serialized = json.dumps(result, sort_keys=True)
    if time.monotonic() - START > 88:
        result['status'] = 'inconclusive'
        result['summary'] = '88-second final-output deadline'
        result['evidence']['actual'] = None
        serialized = json.dumps(result, sort_keys=True)
    print(serialized)


if __name__ == '__main__':
    main()

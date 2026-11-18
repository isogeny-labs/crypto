import json
import hashlib
from pathlib import Path
from research.residue_quotient_20261116_v99b.kernel import basis,preimage,apply,code
from research.guarded_local_search_20261116_v101.kernel import matrix
from research.local_split_circuit_20261116_v100.kernel import Circuit

def coordinate_map(E, affine, n):
    piv, K = basis(E)
    width = len(piv)
    assert set(piv) == set(range(width))
    R = [preimage(1 << i, piv) for i in range(width)]
    T = R + K
    assert len(T) == n and len(basis(T)[0]) == n
    assert all(apply(E, v) == 1 << i for i, v in enumerate(R))
    assert all(apply(E, v) == 0 for v in K)
    return {'E': E, 'affine': affine, 'R': R, 'K': K, 'T': T,
            'offset': apply(R, affine), 'width': width, 'n': n}

def old_value(mapping, new_value):
    return mapping['offset'] ^ apply(mapping['T'], new_value)

def new_value(mapping, theta):
    y = apply(mapping['E'], theta) ^ mapping['affine']
    residual = theta ^ apply(mapping['R'], apply(mapping['E'], theta))
    z = preimage(residual, basis(mapping['K'])[0])
    return y | (z << mapping['width'])

def actual_map(d):
    raw = Path(d['matrix']).read_bytes()
    assert hashlib.sha256(raw).hexdigest() == d['matrix_sha256']
    rows = matrix(d['matrix'])
    directions = rows[1:] + [[d['modulus'] << j, 0, 0, 0, 0, 0] for j in range(3)]
    assert len(directions) == 76
    E = [0] * 76
    affine = 0
    offset = 0
    for s, m in d['fields'][:d['first_blocks']]:
        for j, direction in enumerate(directions):
            E[j] ^= code(direction, s, m) << offset
        affine ^= code(rows[0], s, m) << offset
        offset += 4 * s
    mapping = coordinate_map(E, affine, 76)
    assert mapping['width'] == offset == 72 and len(mapping['K']) == 4
    return mapping

def prune(C, outputs):
    needed = set()
    stack = [w for r in outputs for field in r['wires'] for w in field]
    while stack:
        i = stack.pop()
        if i in needed:
            continue
        needed.add(i)
        node = C.nodes[i]
        if node[0] in ['and', 'xor', 'not']:
            stack.extend(node[1:])
    out = Circuit(C.n)
    mapped = {i: i for i in range(C.n + 2)}
    for i in sorted(needed):
        if i < C.n + 2:
            continue
        kind, a, *rest = C.nodes[i]
        mapped[i] = out.op(kind, mapped[a], mapped[rest[0]] if rest else None)
    result = [{**r, 'wires': [[mapped[w] for w in field] for field in r['wires']]} for r in outputs]
    return out, result

def rewire(source, outputs, mapping):
    n = source.n
    assert mapping['n'] == n
    C = Circuit(n)
    forms = [None] * len(source.nodes)
    forms[0], forms[1] = 0, 1
    for i in range(n):
        forms[i + 2] = ((mapping['offset'] >> i) & 1) | sum(((v >> i) & 1) << (j + 1) for j, v in enumerate(mapping['T']))
    cache = {0: 0, 1: 1, **{1 << (j + 1): j + 2 for j in range(n)}}
    known = {v: f for f, v in cache.items()}
    wires = {0: 0, 1: 1}
    def emit(form):
        if form not in cache:
            value = form & 1
            for j in range(n):
                if form >> (j + 1) & 1:
                    value = C.xor(value, j + 2)
            cache[form] = value
            known[value] = form
        return cache[form]
    def wire(i):
        if forms[i] is not None:
            return emit(forms[i])
        return wires[i]
    for i, node in enumerate(source.nodes[n + 2:], n + 2):
        kind, a, *rest = node
        fa = forms[a]
        fb = forms[rest[0]] if rest else None
        if kind == 'xor' and fa is not None and fb is not None:
            forms[i] = fa ^ fb
            continue
        if kind == 'not' and fa is not None:
            forms[i] = fa ^ 1
            continue
        if kind == 'and' and fa is not None and fb is not None:
            if fa == 0 or fb == 0 or fa ^ fb == 1:
                forms[i] = 0
                continue
            if fa == 1 or fa == fb:
                forms[i] = fb
                continue
            if fb == 1:
                forms[i] = fa
                continue
        w = C.op(kind, wire(a), wire(rest[0]) if rest else None)
        wires[i] = w
        if w in known:
            forms[i] = known[w]
    result = [{**r, 'wires': [[wire(w) for w in field] for field in r['wires']]} for r in outputs]
    C, result = prune(C, result)
    return C, result

def dependencies(C):
    support = [0, 0] + [1 << i for i in range(C.n)]
    for kind, a, *rest in C.nodes[C.n + 2:]:
        support.append(support[a] | (support[rest[0]] if rest else 0))
    return support

def compile_all(d):
    raw = Path(d['source_graph']).read_bytes()
    assert hashlib.sha256(raw).hexdigest() == d['source_graph_sha256']
    graph = json.loads(raw)
    source = Circuit(76)
    source.nodes = [tuple(n) for n in graph['nodes']]
    mapping = actual_map(d)
    C, outputs = rewire(source, graph['outputs'], mapping)
    support = dependencies(C)
    blocks = []
    offset = 0
    for r, (s, m) in zip(outputs[:d['first_blocks']], d['fields']):
        observed = 0
        for field in r['wires']:
            for w in field:
                observed |= support[w]
        allowed = ((1 << (4 * s)) - 1) << offset
        assert observed & ~allowed == 0
        blocks.append({'s': s, 'm': m, 'offset': offset, 'width': 4 * s,
                       'observed_primary_bits': observed.bit_count()})
        offset += 4 * s
    return C, outputs, mapping, blocks

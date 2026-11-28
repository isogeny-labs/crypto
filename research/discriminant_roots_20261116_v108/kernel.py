"""Independent implementation of known integrated initial-split baseline."""
from research.discriminant_roots_20261116_v107b.kernel import *
from research.discriminant_roots_20261116_v107b.kernel import solve as reference_solve

def quotient(a, b, f):
    q, r = divrem(a, b, f)
    assert not r
    return q

def squarefree_layers(H, f):
    H = monic(H, f)
    if len(H) <= 1:
        return []
    D = derivative(H, f)
    if not D:
        assert all(not c for i, c in enumerate(H) if i % f.p)
        root = [f.power(H[i], f.p ** (f.n - 1)) for i in range(0, len(H), f.p)]
        return [(a, e * f.p) for a, e in squarefree_layers(root, f)]
    C = gcd(H, D, f)
    W = quotient(H, C, f)
    layers = []
    e = 1
    while len(W) > 1:
        Y = gcd(W, C, f)
        Z = quotient(W, Y, f)
        if len(Z) > 1:
            layers.append((Z, e))
        W = Y
        C = quotient(C, Y, f)
        e += 1
    if len(C) > 1:
        assert all(not c for i, c in enumerate(C) if i % f.p)
        root = [f.power(C[i], f.p ** (f.n - 1)) for i in range(0, len(C), f.p)]
        layers.extend((a, e * f.p) for a, e in squarefree_layers(root, f))
    return layers

def trace_extract_group(R, f, seed):
    if f.p != 2:
        return extract(R, f, seed)
    pending = [R] if len(R) > 1 else []
    roots = []
    while pending:
        H = pending.pop()
        if len(H) == 2:
            roots.append(f.neg(H[0]))
            continue
        # Every root in this group already has the same absolute trace.
        for j in range(1, f.n):
            z = [0, 1 << j]
            T = []
            for _ in range(f.n):
                T = add(T, z, f)
                z = square(z, H, f)
            G = gcd(H, T, f)
            if 1 < len(G) < len(H):
                pending.extend([G, quotient(H, G, f)])
                break
        else:
            raise OverflowError('trace basis exhausted; incomplete')
    return sorted(roots)

def initial_squarefree_roots(H, f, seed):
    roots = []
    if len(H) <= 1:
        return roots
    if H[0] == 0:
        roots.append(0)
        H = H[1:]
    if len(H) <= 1:
        return roots
    if len(H) == 2:
        return sorted(roots + [f.neg(H[0])])
    if f.p == 2:
        z = [0, 1]
        T = []
        for _ in range(f.n):
            T = add(T, z, f)
            z = square(z, H, f)
        groups = [gcd(H, T, f), gcd(H, add(T, [1], f), f)]
    else:
        T = power([0, 1], (f.q - 1) // 2, H, f)
        groups = [gcd(H, add(T, [f.neg(1)], f), f), gcd(H, add(T, [1], f), f)]
    for group in groups:
        roots.extend(trace_extract_group(group, f, seed))
    return sorted(roots)

def initial_roots(H, f, seed):
    result = []
    for factor, multiplicity in squarefree_layers(H, f):
        result.extend([x, multiplicity] for x in initial_squarefree_roots(factor, f, seed))
    return sorted(result)

def solve(H, f, method, seed):
    if method not in ['initial', 'dispatch_initial']:
        return reference_solve(H, f, method, seed)
    if method == 'initial' or f.p != 2:
        result = initial_roots(H, f, seed)
        branch = 'initial'
    else:
        A, B, D = delta_data(H, f)
        d, c, b, a, t, _ = H
        if D:
            roots = initial_squarefree_roots(H, f, seed)
            branch = 'squarefree_initial'
        elif not A and not B:
            Q = [f.power(c, 1 << (f.n - 1)), f.power(a, 1 << (f.n - 1)), 1]
            roots = sorted(set([t] + [x for x, e in initial_roots(Q, f, seed)]))
            branch = 'quadratic_squared_initial'
        else:
            assert A
            u = f.mul(B, f.inv(A))
            J = [f.add(b, f.mul(t, u)), f.add(a, u), t, 1]
            assert product([u, 0, 1], J, f) == H
            roots = sorted(set([f.power(u, 1 << (f.n - 1))] + [x for x, e in initial_roots(J, f, seed)]))
            branch = 'cubic_repeated_initial'
        result = []
        for x in roots:
            current = H
            e = 0
            while len(current) > 1:
                q, r = divrem(current, [f.neg(x), 1], f)
                if r:
                    break
                current = q
                e += 1
            result.append([x, e])
    assert len({x for x, e in result}) == len(result)
    for x, e in result:
        current = H
        count = 0
        while len(current) > 1:
            q, r = divrem(current, [f.neg(x), 1], f)
            if r:
                break
            count += 1
            current = q
        assert count == e and e > 0
    return {'roots': result, 'complete': True, 'branch': branch}

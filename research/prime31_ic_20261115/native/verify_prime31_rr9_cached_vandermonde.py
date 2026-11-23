#!/usr/bin/env python3
"""Independent Python BigInt checker for B8 cached Vandermonde chart source."""
import json
import sys

P = 2147483647
A = 847514846
B = 2087444724

def mm(a, b):
    return (a * b) % P

def sub(a, b):
    return (a - b) % P

def determinant(matrix):
    m = [row[:] for row in matrix]
    n = len(m)
    d = 1
    for k in range(n):
        pivot = next((i for i in range(k, n) if m[i][k] % P), None)
        if pivot is None:
            return 0
        if pivot != k:
            m[k], m[pivot] = m[pivot], m[k]
            d = -d
        x = m[k][k] % P
        d = mm(d, x)
        inv = pow(x, P-2, P)
        for i in range(k+1, n):
            scale = mm(m[i][k], inv)
            for j in range(k+1, n):
                m[i][j] = sub(m[i][j], mm(scale, m[k][j]))
            m[i][k] = 0
    return d % P

def inverse(matrix):
    n = len(matrix)
    m = [[v % P for v in matrix[i]] + [int(i == j) for j in range(n)]
         for i in range(n)]
    for k in range(n):
        pivot = next((i for i in range(k, n) if m[i][k]), None)
        assert pivot is not None, "unexpected singular Vandermonde"
        m[k], m[pivot] = m[pivot], m[k]
        factor = pow(m[k][k], P - 2, P)
        for j in range(2 * n):
            m[k][j] = mm(m[k][j], factor)
        for i in range(n):
            if i == k:
                continue
            factor = m[i][k]
            for j in range(2 * n):
                m[i][j] = sub(m[i][j], mm(factor, m[k][j]))
    return [line[n:] for line in m]

def dot(x, y):
    return sum(a*b for a,b in zip(x,y)) % P

def matmul(X, Y):
    return [[dot(row, col) for col in zip(*Y)] for row in X]

def target_rows(points, target):
    rx, ry = target
    out = []
    for x,y in points:
        powers = [pow(x, j, P) for j in range(4)]
        out.append([mm(sub(x, rx), power) for power in powers]
                   + [mm((y+ry) % P, power) for power in powers])
    return out

def verify_one(s):
    target = s["target"]
    points = s["points"]
    xs = [pt[0] for pt in points]
    ys = [pt[1] for pt in points]
    rx, ry = target
    assert len(set(xs)) == 4 and all(x != rx for x in xs)
    assert all((y*y - (x*x*x + A*x + B)) % P == 0 for x,y in points)
    V = [[pow(x,j,P) for j in range(4)] for x in xs]
    vi = inverse(V)
    z = [mm((y+ry) % P, pow(sub(x,rx), P-2, P)) for x,y in points]
    zv = [[mm(z[i], v) for v in V[i]] for i in range(4)]
    expected = matmul(vi,zv)
    expected_det_a = mm(determinant(V),
                        mm(mm(sub(xs[0],rx),sub(xs[1],rx)),
                           mm(sub(xs[2],rx),sub(xs[3],rx))))
    assert expected == s["m"], ("chart mismatch",s["target_id"],s["half"])
    assert expected_det_a == s["det_a"], ("det mismatch",s["target_id"],s["half"])

def verify_pair(a,b):
    assert a["target"] == b["target"]
    big=target_rows(a["points"]+b["points"],a["target"])
    delta = [[sub(b["m"][i][j], a["m"][i][j]) for j in range(4)]
             for i in range(4)]
    schur=mm(mm(a["det_a"],b["det_a"]),determinant(delta))
    assert determinant(big)==schur,"independent 8x8 mismatch"

def main(path):
    with open(path,encoding="utf-8") as f:
        data=json.load(f)
    assert data["field_prime"]==P
    assert data["group_order"]==2147446831
    assert data["cache_unsigned_tables"]==8192
    assert data["B8_full_pair_tests_unchanged"]==4294967296
    runs=data["runs"]
    assert [r["mode"] for r in runs]==["fresh","cached","cached","fresh"]
    assert len({r["digest"] for r in runs})==1, "paired digests differ"
    samples=data["independent_python_samples"]
    assert len(samples)==128
    for s in samples:
        verify_one(s)
    pairs={}
    for s in samples:
        pairs.setdefault((s["target_id"],tuple(s["word"])),{})[s["half"]]=s
    # Need sample words aligned by signed numeric index, not identical leaf
    # meanings across halves: both halves export the same 32 word indices.
    assert len(pairs)==64
    for v in pairs.values():
        assert set(v)=={0,1}
        verify_pair(v[0],v[1])
    print(json.dumps({"python_independent_half_charts_verified":len(samples),
      "python_independent_full_8x8_pairs_verified":len(pairs),
      "paired_equal_digests":True,"external_signed_pcc":"unverified"}))

if __name__=="__main__":
    main(sys.argv[1])

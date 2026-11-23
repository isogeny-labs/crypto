#!/usr/bin/env python3
"""CT330 different-language, independently authored F103 diagnostic rho oracle.

A same-producer cross-check only: no native replay, timing or signed PCC.
"""
P = 103
R = 109
G = (1, 51)


def require(ok, message):
    if not ok:
        raise ValueError(message)


def add(a, b):
    if a is None:
        return b
    if b is None:
        return a
    x, y = a
    u, v = b
    if x == u and (y + v) % P == 0:
        return None
    num, den = (3*x*x+1, 2*y) if x == u else (v-y, u-x)
    slope = num * pow(den % P, P-2, P) % P
    xx = (slope*slope-x-u) % P
    return (xx, (slope*(x-xx)-y) % P)


def scalar(k, point):
    out = None
    while k:
        if k & 1:
            out = add(out, point)
        point = add(point, point)
        k >>= 1
    return out


def solve(target):
    ops = 0
    canonicalisations = 0
    negations = 0
    steps = 0
    collisions = 0
    useless = 0
    seen = {}

    def counted_add(a, b):
        nonlocal ops
        ops += 1
        return add(a, b)

    def counted_scalar(k, pt):
        result = None
        while k:
            if k & 1:
                result = counted_add(result, pt)
            pt = counted_add(pt, pt)
            k >>= 1
        return result

    def canonical(pt, a, b):
        nonlocal canonicalisations, negations
        canonicalisations += 1
        if pt is not None and pt[1] > P - pt[1]:
            negations += 1
            return (pt[0], P - pt[1]), (-a) % R, (-b) % R
        return pt, a, b

    require(counted_scalar(R, G) is None, "G order check")
    require(counted_scalar(R, target) is None, "Q order check")
    for seed in range(64):
        jumps = []
        for j in range(16):
            a = (1 + 17*j + 13*seed) % R
            b = (1 + 29*j + 11*seed) % R
            d = counted_add(counted_scalar(a, G), counted_scalar(b, target))
            jumps.append((a, b, d))
        a = (1 + 19*seed) % R
        b = (7 + 23*seed) % R
        point, a, b = canonical(
            counted_add(counted_scalar(a, G), counted_scalar(b, target)), a, b)
        for _ in range(4*R):
            if point in seen:
                collisions += 1
                previous_a, previous_b = seen[point]
                denominator = (previous_b-b) % R
                if denominator:
                    answer = (a-previous_a) * pow(denominator, -1, R) % R
                    require(scalar(answer, G) == target, "invalid rho collision")
                    require(counted_scalar(answer, G) == target, "rho verification")
                    return {
                        "log": answer, "ec_adds": ops,
                        "canonicalisations": canonicalisations,
                        "negations": negations, "walk_steps": steps,
                        "collisions": collisions, "useless_collisions": useless,
                        "restarts": seed+1, "table_entries": len(seen),
                    }
                useless += 1
                break
            seen[point] = (a, b)
            j = 0 if point is None else (131*point[0] + 17*point[1]) % 16
            da, db, delta = jumps[j]
            point = counted_add(point, delta)
            a = (a + da) % R
            b = (b + db) % R
            point, a, b = canonical(point, a, b)
            steps += 1
    raise ValueError("bounded rho did not recover public target")


def main():
    require(scalar(R, G) is None, "wrong subgroup")
    target73 = scalar(73, G)
    require(target73 == (42, 71), "wrong fixed public Q73")
    results = []
    for k in range(R):
        q = scalar(k, G)
        sample = solve(q)
        require(sample["log"] == k, "wrong recovered discrete log")
        results.append(sample)
    q73 = results[73]
    expected = {
        "log": 73, "ec_adds": 332, "canonicalisations": 3,
        "negations": 1, "walk_steps": 2, "collisions": 1,
        "useless_collisions": 0, "restarts": 1, "table_entries": 2,
    }
    require(q73 == expected, "unexpected source rho primitive vector")
    require(sum(r["ec_adds"] for r in results) == 48147,
            "unexpected 109-target source rho aggregate")
    print("CT330_PY_RHO_PASS logs=109 q73_adds=332 q73_steps=2 q73_restarts=1 aggregate_adds=48147 pcc=unverified")


if __name__ == "__main__":
    main()

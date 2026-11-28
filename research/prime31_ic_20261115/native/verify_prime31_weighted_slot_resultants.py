#!/usr/bin/env python3
"""Independent bigint affine replay of original-Rust weighted W8 slot resultants.

Producer only supplies summary/certificates. This implementation recomputes
all roots, coefficient vectors, positive/negative resultants and signed Q
from the frozen seed, with no imports from producer Rust. Not signed PCC.
"""
import hashlib
import json
import sys

P = 2147483647
A = 847514846
B = 2087444724
N = 2147446831
MASK = (1 << 64) - 1
WINDOWS = [list(range(start, start + 8)) for start in (0, 8, 16, 23)]


def splitmix(state):
    state = (state + 0x9e3779b97f4a7c15) & MASK
    z = state
    z = ((z ^ (z >> 30)) * 0xbf58476d1ce4e5b9) & MASK
    z = ((z ^ (z >> 27)) * 0x94d049bb133111eb) & MASK
    return state, z ^ (z >> 31)


def ec_add(q, r):
    if q is None:
        return r
    if r is None:
        return q
    x, y = q
    u, v = r
    if x == u and (y + v) % P == 0:
        return None
    if x == u:
        slope = ((3 * x * x + A) * pow(2 * y, -1, P)) % P
    else:
        slope = ((v - y) * pow(u - x, -1, P)) % P
    z = (slope * slope - x - u) % P
    return z, (slope * (x - z) - y) % P


def ec_mul(k, q):
    out = None
    while k:
        if k & 1:
            out = ec_add(out, q)
        q = ec_add(q, q)
        k >>= 1
    return out


def base31():
    points = []
    x = 0
    while len(points) < 31:
        yy = (x * x * x + A * x + B) % P
        y = pow(yy, (P + 1) // 4, P)
        if y != 0 and y * y % P == yy:
            points.append((x, min(y, P - y)))
        x += 1
    return points


def monic(roots):
    c = [1]
    for t in roots:
        n = [0] * (len(c) + 1)
        for j, v in enumerate(c):
            n[j] = (n[j] - v * t) % P
            n[j + 1] = (n[j + 1] + v) % P
        c = n
    assert len(c) == 9 and c[8] == 1
    return c


def resultant(c, support):
    product = 1
    for t in support:
        v = 0
        for z in reversed(c):
            v = (v * t + z) % P
        product = (product * v) % P
    return product


def jacobian_det(c, supports, roots):
    """Row-scaled Vandermonde determinant for eight independent resultants."""
    out = 1
    for i, support in enumerate(supports):
        for x in support:
            if x != roots[i]:
                v = 0
                for z in reversed(c):
                    v = (v * x + z) % P
                out = out * v % P
    for i in range(8):
        for j in range(i + 1, 8):
            out = out * (roots[j] - roots[i]) % P
    return out


def verify(data):
    assert data["schema"] == "prime31.weighted_slot_resultants.v1"
    assert all(data["curve"][k] == v for k, v in
               dict(p=P, a=A, b=B, n=N).items())
    assert data["seed"] == "0x534c4f545f524553"
    assert data["cases_per_window"] == 128
    assert data["public_only_unknown_index_extraction"] is False
    assert data["complete_ecdlp_solved"] is False
    assert data["target_conditioned_rr9_solved"] is False
    assert data["generic_sum_table_calls"] == 0
    assert data["slot_supports_pairwise_disjoint"] is True
    assert data["candidate_monic_degree"] == 8
    assert data["slot_membership_equations"] == 8
    assert data["membership_geometric_quotient_dimension"] == 8 ** 8
    assert ec_mul(N, (625037715, 2100627251)) is None

    base = base31()
    state = 0x534c4f545f524553
    tested = 0
    zero_positives = 0
    bad_first = 0
    nonsingular = 0
    for wi, w in enumerate(WINDOWS):
        fb = [base[j] for j in w]
        transformed = [[ec_mul(1 << i, pt) for pt in fb] for i in range(8)]
        support = [[q[0] for q in row] for row in transformed]
        assert len(set(x for row in support for x in row)) == 64
        per_window = data["windows"][wi]
        assert per_window["window"] == wi
        assert per_window["distinct_slot_roots"] == 64
        assert per_window["slot_root_count"] == 8
        assert per_window["positive"] == per_window["negative"] == 128
        assert per_window["first_base_x"] == [q[0] for q in fb]
        for case in range(128):
            choices, signs, roots = [], [], []
            weighted = None
            for i in range(8):
                state, rand = splitmix(state)
                j = rand % 8
                state, rand = splitmix(state)
                sign = rand & 1
                choices.append(j)
                signs.append(sign)
                t = transformed[i][j]
                roots.append(t[0])
                if sign:
                    t = (t[0], (-t[1]) % P)
                weighted = ec_add(weighted, t)
            assert weighted is not None
            assert (weighted[1] ** 2 - weighted[0] ** 3 -
                    A * weighted[0] - B) % P == 0
            c = monic(roots)
            assert len(set(roots)) == 8
            rs = [resultant(c, slot) for slot in support]
            assert rs == [0] * 8
            det = jacobian_det(c, support, roots)
            assert det != 0
            nonsingular += 1
            negroots = roots.copy()
            negroots[0] = support[1][(choices[1] + 1) % 8]
            assert len(set(negroots)) == 8
            negc = monic(negroots)
            nrs = [resultant(negc, slot) for slot in support]
            assert nrs[0] != 0 and nrs[1:] == [0] * 7
            tested += 1
            zero_positives += 8
            bad_first += 1
            if case == 0:
                first = data["first_case_per_window"][wi]
                assert first["window"] == wi
                assert first["choices"] == choices
                assert first["signs"] == signs
                assert first["positive_coefficients_c0_to_c8"] == c
                assert first["negative_coefficients_c0_to_c8"] == negc
                assert first["negative_slot0_resultant"] == nrs[0]
                assert first["jacobian_det"] == det
                assert first["weighted_target"] == list(weighted)
    assert tested == data["positive"] == data["negative"] == 512
    assert zero_positives == data["positive_zero_resultants"] == 4096
    assert bad_first == data["negative_only_slot0_failed"] == 512
    assert nonsingular == data["jacobian_nonsingular"] == 512
    return {"schema": "prime31.weighted_slot_resultants.independent.v1",
            "replayed": tested, "disjoint_windows": 4,
            "positive_zero_resultants": zero_positives,
            "negative_only_slot0_failed": bad_first,
            "jacobian_nonsingular": nonsingular,
            "pcc_signed": False}


def main():
    raw = open(sys.argv[1], "rb").read()
    data = json.loads(raw)
    result = verify(data)
    result["source_json_sha256"] = hashlib.sha256(raw).hexdigest()
    print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()

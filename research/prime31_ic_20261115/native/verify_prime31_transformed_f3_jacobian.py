#!/usr/bin/env python3
"""Independent full-cohort Prime31 F3 Jacobian replay; no Rust or F4 code imported.
Usage: python3 verify_prime31_transformed_f3_jacobian.py results.json
This is a planted-chain correctness verifier, NOT a public-only extractor.
Frozen protocol: rr9/TRANSFORMED_F3_JACOBIAN_PROTOCOL_20261119.md.
"""
import json
import sys

P = 2_147_483_647
A = 847_514_846
B = 2_087_444_724
N = 2_147_446_831
G = (625_037_715, 2_100_627_251)
MASK = (1 << 64) - 1
SEED = 0x4A41434F4249414E
WINDOWS = [
    list(range(0, 8)), list(range(8, 16)),
    list(range(16, 24)), list(range(23, 31)),
]

def curve_rhs(x):
    return (x**3 + A*x + B) % P

def inv(x):
    assert x % P
    return pow(x, P - 2, P)

def ec_add(u, v):
    if u is None:
        return v
    if v is None:
        return u
    x, y = u
    z, w = v
    if x == z and (y + w) % P == 0:
        return None
    if x == z:
        slope = ((3*x*x + A) * inv(2*y)) % P
    else:
        slope = ((w-y) * inv((z-x) % P)) % P
    xx = (slope*slope-x-z) % P
    yy = (slope*(x-xx)-y) % P
    return (xx, yy)

def ec_mul(k, u):
    result = None
    while k:
        if k & 1:
            result = ec_add(result, u)
        u = ec_add(u, u)
        k //= 2
    return result

def splitmix(state):
    state = (state + 0x9E3779B97F4A7C15) & MASK
    z = state
    z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & MASK
    z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & MASK
    return state, z ^ (z >> 31)

def s3(x, y, z):
    return ((x-y)**2*z*z
            - 2*((x+y)*(x*y+A)+2*B)*z
            + (x*y-A)**2 - 4*B*(x+y)) % P

def ds3_z(x, y, z):
    return (2*(x-y)**2*z - 2*((x+y)*(x*y+A)+2*B)) % P

def base31():
    out = []
    for x in range(10_000):
        rhs = curve_rhs(x)
        y = pow(rhs, (P+1)//4, P)
        if y != 0 and y*y % P == rhs:
            out.append((x, min(y, P-y)))
            if len(out) == 31:
                break
    assert len(out) == 31
    return out

def main(path):
    with open(path, encoding="utf-8") as f:
        record = json.load(f)
    assert record["schema"] == "prime31.transformed_f3_jacobian.v1"
    assert record["curve"]["p"] == P and record["curve"]["n"] == N
    assert int(record["seed"], 16) == SEED
    assert record["cases_per_window"] == 128
    assert record["tested"] == 512
    assert ec_mul(N, G) is None
    base = base31()
    state = SEED
    count = 0
    first_cases = []
    for wi, window in enumerate(WINDOWS):
        slots = [
            [ec_mul(1 << i, base[j]) for j in window]
            for i in range(8)
        ]
        assert len({point[0] for slot in slots for point in slot}) == 64
        for ci in range(128):
            leaves = []
            for i in range(8):
                state, choice = splitmix(state)
                state, sign = splitmix(state)
                p = slots[i][choice % 8]
                leaves.append((p[0], p[1] if sign & 1 == 0 else (-p[1]) % P))
            sums = [None] * 8
            sums[7] = leaves[7]
            for i in range(6, -1, -1):
                sums[i] = ec_add(leaves[i], sums[i+1])
            assert all(p is not None for p in sums)
            t = [p[0] for p in leaves]
            z = [p[0] for p in sums]
            hp = []
            for i in range(8):
                others = [q[0] for q in slots[i] if q[0] != t[i]]
                assert len(others) == 7
                d = 1
                for root in others:
                    d = d * (t[i] - root) % P
                assert d != 0
                hp.append(d)
            derivs = []
            for i in range(7):
                assert s3(t[i], z[i+1], z[i]) == 0
                d = ds3_z(t[i], z[i+1], z[i])
                assert d != 0
                assert d*d % P == 16*curve_rhs(t[i])*curve_rhs(z[i+1]) % P
                derivs.append(d)
            minor = 1
            for d in hp + derivs[1:]:
                minor = (minor*d) % P
            assert minor != 0
            if ci == 0:
                first_cases.append({
                    "window": wi, "target": list(sums[0]),
                    "leaf_x": t, "suffix_x": z,
                    "hprime": hp, "s3_z_derivative": derivs,
                    "jacobian_minor_up_to_sign": minor,
                })
            count += 1
    assert count == 512
    assert first_cases == record["first_case_per_window"], "independent first-case mismatch"
    assert record["ordinary"] == count and record["exceptional"] == 0
    assert record["selector_derivatives_nonzero"] == count*8
    assert record["f3_equations_and_derivatives_nonzero"] == count*7
    assert record["f3_discriminant_identities"] == count*7
    assert record["full_rank_14x14_pivots"] == count*14
    assert record["generic_sum_table_calls"] == 0
    assert record["public_only_unknown_index_extraction"] is False
    assert record["whole_solver_speedup_measured"] is False
    print(json.dumps({
        "schema": "prime31.transformed_f3_jacobian.independent.v1",
        "ordinary": count,
        "hprime_nonzero": count*8,
        "discriminant_s3": count*7,
        "jacobian_minor_factors": count*14,
        "first_cases_identical": len(first_cases),
        "pcc_signed": False,
    }, sort_keys=True))

if __name__ == "__main__":
    if len(sys.argv) != 2:
        raise SystemExit("usage: verify_prime31_transformed_f3_jacobian.py results.json")
    main(sys.argv[1])

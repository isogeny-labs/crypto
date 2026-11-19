#!/usr/bin/env python3
"""Independent original-curve BigInt replay of the RR9 public-polynomial derivative gate.

No import from Rust, no reliance on the candidate derivative subroutine, and
no generic 4+4 relation tables. Signed witnesses are test-fixture-only data.
"""
import json
import sys

P = 2147483647
A = 847514846
B = 2087444724
N = 2147446831
OLD = [1, 3, 5, 7, 11, 13, 17, 19]
TERNARY = [1, 3, 9, 27, 81, 243, 729, 2187]


def plus(p, q):
    if p is None:
        return q
    if q is None:
        return p
    x, y = p
    xx, yy = q
    if xx == x and (yy + y) % P == 0:
        return None
    if p == q:
        slope = (3 * x * x + A) * pow(2 * y, P - 2, P) % P
    else:
        slope = (yy - y) * pow(xx - x, P - 2, P) % P
    nx = (slope * slope - x - xx) % P
    return nx, (slope * (x - nx) - y) % P


def scalar(k, q):
    result = None
    while k:
        if k & 1:
            result = plus(result, q)
        q = plus(q, q)
        k >>= 1
    return result


def rank_fp(rows):
    mat = [row[:] for row in rows]
    rank = 0
    for col in range(7):
        pivot = next((i for i in range(rank, len(mat)) if mat[i][col] % P), None)
        if pivot is None:
            continue
        mat[rank], mat[pivot] = mat[pivot], mat[rank]
        inverse = pow(mat[rank][col], P - 2, P)
        mat[rank] = [x * inverse % P for x in mat[rank]]
        for i in range(rank + 1, len(mat)):
            factor = mat[i][col]
            mat[i] = [(x - factor * y) % P for x, y in zip(mat[i], mat[rank])]
        rank += 1
    return rank


def public_resultants_with_dual(r, coeffs, support):
    """Recompute exact H and dH in Fp[t]/t^2, not via Rust product gradients."""
    rx, ry = r
    h = []
    gradients = []
    for slot in support:
        derivatives = []
        for axis in range(7):
            product0, product1 = 1, 0
            for x, unused_y in slot:
                x0, x1, x2, x3 = 1, x, x*x % P, x*x*x % P
                powers = [x0, x1, x2, x3]
                a = sum(coeffs[i] * powers[i] for i in range(4)) % P
                v = (x3 + sum(coeffs[4+i] * powers[i] for i in range(3))) % P
                da = powers[axis] if axis < 4 else 0
                dv = powers[axis-4] if axis >= 4 else 0
                d = (x*x + rx*x + rx*rx + A) % P
                u = (x-rx) % P
                m0 = (d*v*v-u*a*a-2*ry*a*v) % P
                m1 = (2*d*v*dv-2*u*a*da-2*ry*(da*v+a*dv)) % P
                product1 = (product1*m0 + product0*m1) % P
                product0 = product0*m0 % P
            derivatives.append((product0, product1))
        assert len(set(pair[0] for pair in derivatives)) == 1
        h.append(derivatives[0][0])
        gradients.append([pair[1] for pair in derivatives])
    return h, gradients


def replay(record):
    assert record["schema"] == "prime31.rr9.public-resultant-gradient.v1"
    assert record["curve"] == {"p": P, "a": A, "b": B, "n": N, "cofactor": 1}
    assert not record["generic_group_sum_tables_used"]
    assert not record["public_unknown_chart_extractor_implemented"]
    assert not record["complete_nongeneric_ecdlp_solver_implemented"]
    assert int(record["formal_support_words_per_window"]) == 16 ** 8
    assert record["exact_uniform_mean_ordered_witnesses"] == f"{16**8}/{N}"
    for item in record["fixed_arity_scaling_necessary_bound"]:
        bits = item["hypothetical_group_size_bits"]
        assert item["min_single_window_uniform_shifts_log2_lower_bound"] == max(0, bits - 32)
        assert item["min_four_window_uniform_shifts_log2_lower_bound"] == max(0, bits - 34)
    total = 0
    for arm in record["arms"]:
        weights = OLD if arm["arm"] == "old" else TERNARY
        assert arm["weights"] == weights
        base = [tuple(p) for p in arm["b8_base_points"]]
        assert [p[0] for p in base] == [0, 1, 2, 3, 5, 6, 8, 9]
        assert all((y*y-x*x*x-A*x-B) % P == 0 for x,y in base)
        support = [[scalar(w, q) for q in base] for w in weights]
        assert all(p is not None for row in support for p in row)
        assert len({p[0] for row in support for p in row}) == 64
        assert arm["disjoint_public_slot_x"] == 64
        assert arm["planted_ranks_full_column"] == arm["fixture_count"]
        assert arm["dual_derivatives_verified"] == arm["fixture_count"]*56
        assert len(arm["fixtures"]) == arm["fixture_count"]
        for f in arm["fixtures"]:
            r = tuple(f["target"])
            c = f["chart"]
            assert len(c) == 7
            assert all(0 <= z < P for z in c)
            signed = []
            for j in range(8):
                i = f["signed_indices"][j]
                sign = f["signed_signs"][j]
                x,y = support[j][i]
                signed.append((x, y if sign == 1 else (-y) % P))
            summed = None
            for p in signed:
                summed = plus(summed, p)
            assert summed == r
            for x,y in signed:
                value_a = sum(c[i]*pow(x,i,P) for i in range(4)) % P
                value_v = (pow(x,3,P)+sum(c[4+i]*pow(x,i,P) for i in range(3))) % P
                assert ((x-r[0])*value_a+(y+r[1])*value_v) % P == 0
            h, jac = public_resultants_with_dual(r,c,support)
            assert h == [0]*8
            assert h == f["resultants"]
            assert jac == f["jacobian"]
            assert rank_fp(jac) == f["rank"] == 7
            altered = c[:]
            altered[0] = (altered[0]+1) % P
            bad, _ = public_resultants_with_dual(r,altered,support)
            assert bad == f["mutated_resultants"]
            assert any(z != 0 for z in bad)
            total += 1
    assert total == 32, f"full run requires 16 fixtures in each arm; got {total}"
    print(f"INDEPENDENT ORIGINAL-CURVE REPLAY PASS: {total} planted chart fixtures,"
          f" {total*56} exact derivatives, full rank 7/7, mutation negatives;"
          " zero generic four-sum tables; NO unknown-index extraction.")


if __name__ == "__main__":
    assert len(sys.argv) == 2
    with open(sys.argv[1], encoding="utf-8") as f:
        replay(json.load(f))

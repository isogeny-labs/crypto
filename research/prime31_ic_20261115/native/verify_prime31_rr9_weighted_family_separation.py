#!/usr/bin/env python3
"""Independent Python affine replay of ORIGINAL Prime31 weighted/fixed-RR9 audit.

This is source-separated arithmetic, not an ECDLP solver or signed PCC.
Input: original Rust release JSON prime31-weighted-family-separation.json.
"""
import json
import sys

P = 2147483647
A = 847514846
B = 2087444724
N = 2147446831
SEED = 0x5252395f57454947
MASK = (1 << 64) - 1
WINDOWS = [list(range(0, 8)), list(range(8, 16)), list(range(16, 24)),
           list(range(23, 31))]


def splitmix(state):
    state = (state + 0x9E3779B97F4A7C15) & MASK
    z = state
    z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & MASK
    z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & MASK
    return state, z ^ (z >> 31)


def negate(q):
    return (q[0], (-q[1]) % P) if q is not None else None


def add(q, r):
    if q is None:
        return r
    if r is None:
        return q
    x, y = q
    u, v = r
    if x == u and (y + v) % P == 0:
        return None
    slope = (((3 * x * x + A) * pow(2 * y, -1, P)) if q == r
             else ((v - y) * pow(u - x, -1, P))) % P
    xx = (slope * slope - x - u) % P
    return xx, (slope * (x - xx) - y) % P


def scalar(k, q):
    r = None
    while k:
        if k & 1:
            r = add(r, q)
        q = add(q, q)
        k >>= 1
    return r


def build_base():
    out = []
    for x in range(10000):
        rhs = (x * x * x + A * x + B) % P
        y = pow(rhs, (P + 1) // 4, P)
        if y * y % P == rhs and y:
            out.append((x, min(y, P - y)))
            if len(out) == 31:
                break
    assert len(out) == 31
    assert scalar(N, (625037715, 2100627251)) is None
    return out


def independent_replay():
    base = build_base()
    state = SEED
    totals = dict(weighted_targets_in_squarefree_support=0,
                  weighted_equal_same_signed_unweighted=0,
                  transformed_x_equal_fixed_hb=0,
                  weighted_identity=0)
    summaries = []
    examples = []
    for wi, win in enumerate(WINDOWS):
        fb = [base[i] for i in win]
        support = set()
        for signbits in range(256):
            q = None
            for i, leaf in enumerate(fb):
                q = add(q, negate(leaf) if (signbits >> i) & 1 else leaf)
            support.add(q)
        assert len(support) == 256
        hit = 0
        for case in range(128):
            perm = list(range(8))
            for i in range(7, 0, -1):
                state, rnd = splitmix(state)
                j = rnd % (i + 1)
                perm[i], perm[j] = perm[j], perm[i]
            signed = []
            for index in perm:
                state, rnd = splitmix(state)
                signed.append(negate(fb[index]) if rnd & 1 else fb[index])
            plain = None
            weighted = None
            tx = []
            for i, leaf in enumerate(signed):
                z = scalar(1 << i, leaf)
                assert z is not None
                weighted = add(weighted, z)
                plain = add(plain, leaf)
                tx.append(z[0])
            tx.sort()
            assert plain in support
            hit += weighted in support
            totals["weighted_targets_in_squarefree_support"] += weighted in support
            totals["weighted_equal_same_signed_unweighted"] += weighted == plain
            totals["transformed_x_equal_fixed_hb"] += tx == [p[0] for p in fb]
            totals["weighted_identity"] += weighted is None
            if case == 0:
                examples.append(dict(window=wi, signed=signed,
                    unweighted_target=plain, weighted_target=weighted,
                    transformed_x_sorted=tx))
        summaries.append(dict(window=wi, squarefree_support_size=256,
                              planted_weighted_fixtures=128,
                              weighted_targets_in_squarefree_support=hit))
    return totals, summaries, examples


def verify(record):
    assert record["schema"] == "prime31.rr9.weighted_family_separation.v1"
    assert record["curve"]["p"] == P and record["curve"]["n"] == N
    assert record["seed"] == f"0x{SEED:016x}"
    assert record["cases_per_window"] == 128
    assert record["planted_weighted_total"] == 512
    totals, summaries, examples = independent_replay()
    for k, val in totals.items():
        assert record[k] == val, (k, record[k], val)
    assert record["per_window"] == summaries
    assert record["first_case_per_window"] == [
        json.loads(json.dumps(x)) for x in examples]
    assert record["generic_squarefree_grader_used"] is True
    assert record["public_only_unknown_index_extraction"] is False
    assert record["complete_ecdlp_solved"] is False
    return dict(schema="prime31.rr9.weighted_family_independent_replay.v1",
                checked=512, per_window=128, positive_unweighted_controls=512,
                check_names=list(totals), independently_replayed=True,
                signed_pcc_verified=False)


if __name__ == "__main__":
    if len(sys.argv) != 2:
        raise SystemExit("usage: verify_prime31_rr9_weighted_family_separation.py RUST.json")
    with open(sys.argv[1], encoding="utf-8") as file:
        rec = json.load(file)
    print(json.dumps(verify(rec), indent=2, sort_keys=True))

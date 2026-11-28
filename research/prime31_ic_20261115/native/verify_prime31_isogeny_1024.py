#!/usr/bin/env python3
"""Independent integer arithmetic readback of native Rust E -> E' prime31 transport.

Checks the frozen 1024 known-answer witness corpus, not a fresh relation
extractor or complete new attack. Run after Rust's exact-source replay.
"""
import json
import sys
from pathlib import Path

P = 2_147_483_647
N = 2_147_446_831
SRC = (847_514_846, 2_087_444_724)
DST = (323_891_735, 1_985_248_359)
G = (625_037_715, 2_100_627_251)
GP = (1_767_020_261, 634_125_456)
U, TAU, V = 533_591_203, 1_555_600_864, 259_447_350


def neg(a):
    return None if a is None else (a[0], (-a[1]) % P)


def add(a, b, curve):
    if a is None:
        return b
    if b is None:
        return a
    x, y = a
    z, w = b
    if x == z and (y + w) % P == 0:
        return None
    slope = ((3 * x * x + curve[0]) * pow(2 * y, -1, P)
             if x == z else (w - y) * pow(z - x, -1, P)) % P
    xx = (slope * slope - x - z) % P
    yy = (slope * (x - xx) - y) % P
    return xx, yy


def mul(k, a, curve):
    r = None
    while k:
        if k & 1:
            r = add(r, a, curve)
        a = add(a, a, curve)
        k >>= 1
    return r


def on_curve(pt, curve):
    if pt is None:
        return True
    x, y = pt
    return (y * y - x * x * x - curve[0] * x - curve[1]) % P == 0


def phi(pt):
    if pt is None:
        return None
    x, y = pt
    h = (x - U) % P
    if h == 0:
        raise ValueError("unexpected public pole at rational kernel")
    t = pow(h, -1, P)
    return (
        (x + 2 * TAU * t + 4 * V * t * t) % P,
        (y * (1 - 2 * TAU * t * t - 8 * V * t * t * t)) % P,
    )


def first_x(curve):
    acc = []
    for x in range(256):
        rhs = (x**3 + curve[0] * x + curve[1]) % P
        y = pow(rhs, (P + 1) // 4, P)
        if y * y % P == rhs and y != 0:
            acc.append((x, min(y, P - y)))
            if len(acc) == 31:
                return acc
    raise ValueError("first rational-x base did not fit registered 256 x")


def read_point(v):
    if not isinstance(v, list) or len(v) != 2:
        raise ValueError("malformed affine point")
    q = tuple(v)
    if any(type(x) is not int or x < 0 or x >= P for x in q):
        raise ValueError("point outside base field")
    return q


def check(raw, report):
    assert raw["schema"] == "prime31.dickson31.complete_rank1024.v1"
    assert report["not_a_new_extractor"] is True
    assert report["not_full_cost_solver"] is True
    assert report["frozen_public_unknown_log_recoveries_rederived_from_witnesses"] == 1024
    basis = [read_point(q) for q in raw["base_points"]]
    logs = raw["recovered_factor_base_logs"]
    assert len(basis) == len(logs) == len(raw["basis_rows_raw"]) == 31
    assert all(on_curve(q, SRC) for q in basis)
    assert all(mul(int(k), G, SRC) == q for k, q in zip(logs, basis))
    for row in raw["basis_rows_raw"]:
        assert len(row) == 32
        assert sum(int(a) * int(b) for a, b in zip(row[:31], logs)) % N == row[31]
    mapped_basis = [phi(q) for q in basis]
    assert all(on_curve(q, DST) for q in mapped_basis)
    assert all(mul(int(k), GP, DST) == q for k, q in zip(logs, mapped_basis))
    assert phi(G) == GP
    corpus = raw["target_replay_transcript"]
    assert len(corpus) == 1024
    histogram = [0] * 9
    checked = set()
    for j, rec in enumerate(corpus):
        q = read_point(rec["public_target_q"])
        k = int(rec["recovered_log"])
        shift = int(rec["shift"])
        assert k < N and shift in range(9)
        assert q not in checked, ("duplicate public input", j)
        checked.add(q)
        histogram[shift] += 1
        q2 = phi(q)
        assert on_curve(q2, DST)
        assert mul(k, G, SRC) == q, ("source answer mismatch", j)
        assert mul(k, GP, DST) == q2, ("image answer mismatch", j)
        words = rec["signed_eight_token_witness"]
        assert len(words) == 8 and all(isinstance(t, int) and 0 <= t < 62 for t in words)
        s = None
        t = None
        total = 0
        for token in words:
            idx, sign = divmod(token, 2)
            term = basis[idx] if sign == 0 else neg(basis[idx])
            term2 = mapped_basis[idx] if sign == 0 else neg(mapped_basis[idx])
            total = (total + (logs[idx] if sign == 0 else -logs[idx])) % N
            s = add(s, term, SRC)
            t = add(t, term2, DST)
        assert s == add(q, mul(shift, G, SRC), SRC), ("signed source relation mismatch", j)
        assert t == add(q2, mul(shift, GP, DST), DST), ("signed target relation mismatch", j)
        assert (total - shift) % N == k, ("recovered witness log mismatch", j)
        assert rec["used_generic_bsgs_fallback"] is False
    assert histogram == [951, 70, 3, 0, 0, 0, 0, 0, 0]
    source_first = first_x(SRC)
    target_first = first_x(DST)
    overlap = len({phi(q)[0] for q in source_first} &
                  {q[0] for q in target_first})
    assert overlap == report["first_x31_on_source_transported_overlap_with_first_x31_on_target"]
    assert report["rank_from_original_31_equations"] == 31
    assert report["signed_eight_witnesses_replayed_on_both_curves"] == 2048
    assert report["original_and_transported_factor_log_checks"] == 62
    assert report["batch_public_1024_map"]["inversions"] == 1
    assert report["single_public_1024_map"]["inversions"] == 1024
    assert mul((corpus[0]["recovered_log"]+1) % N, G, SRC) != read_point(corpus[0]["public_target_q"])
    return {
        "validation": "producer-authored independent Python EC group laws, modular equations and map",
        "source": "frozen 1024-target public B31 witness corpus",
        "curve": "p=2^31-1, n=2147446831, cofactor=1",
        "basis_row_equations": 31,
        "basis_scalar_replays_on_source_and_destination": 62,
        "public_scalar_replays_on_source_and_destination": 2048,
        "signed_eight_witness_relations_on_source_and_destination": 2048,
        "first_shift_histogram": histogram,
        "source_transported_first_x_vs_native_target_first_x_overlap": overlap,
        "rust_single_inversions": 1024,
        "rust_batch_inversions": 1,
        "mutation_control": "wrong known log rejected",
        "new_extractor": False,
        "new_cold_solver_speedup": False,
        "signed_pcc": False,
    }


def main():
    if len(sys.argv) != 3:
        raise SystemExit("usage: verify_prime31_isogeny_1024.py FROZEN_RAW.json RUST_REPORT.json")
    raw = json.loads(Path(sys.argv[1]).read_text())
    report = json.loads(Path(sys.argv[2]).read_text())
    print(json.dumps(check(raw, report), sort_keys=True, indent=2))


if __name__ == "__main__":
    main()

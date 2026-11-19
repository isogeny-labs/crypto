#!/usr/bin/env python3
"""Independent BigInt replay of frozen Rust prime31 y-ranked/x-first complete ECDLP panel.

Usage: python3 research/prime31_ic_20261115/native/verify_b31_y_ranked.py ab.log ba.log
This checker implements fresh Python modular elliptic curve arithmetic; it imports
nothing from the candidate Rust crate. Known target scalars are read ONLY after
the candidate has emitted its results. A failed assertion aborts the CI job.
"""
import json
import sys

P = 2_147_483_647
N = 2_147_446_831
A = 847_514_846
B = 2_087_444_724
G = (625_037_715, 2_100_627_251)
PREFIX = "PRIME31_B31_Y_RANKED_COMPLETE_JSON="


def check_point(q):
    assert q is None or (
        isinstance(q, (tuple, list)) and len(q) == 2
        and 0 <= q[0] < P and 0 <= q[1] < P
        and (q[1] * q[1] - q[0] ** 3 - A * q[0] - B) % P == 0
    ), ("invalid original prime31 point", q)


def neg(q):
    return None if q is None else (q[0], (-q[1]) % P)


def plus(u, v):
    if u is None:
        return v
    if v is None:
        return u
    x, y = u
    z, w = v
    if x == z and (y + w) % P == 0:
        return None
    if x == z:
        assert y != 0
        slope = ((3 * x * x + A) * pow(2 * y, P - 2, P)) % P
    else:
        slope = ((w - y) * pow((z - x) % P, P - 2, P)) % P
    xx = (slope * slope - x - z) % P
    yy = (slope * (x - xx) - y) % P
    return (xx, yy)


def scalar(k, q):
    assert k >= 0
    out = None
    while k:
        if k & 1:
            out = plus(out, q)
        q = plus(q, q)
        k >>= 1
    return out


def first_rational_points(count):
    result = []
    x = 0
    while len(result) < count:
        rhs = (x * x * x + A * x + B) % P
        y = pow(rhs, (P + 1) // 4, P)
        if y * y % P == rhs:
            assert y != 0, "prime-order curve has no order-2 point"
            result.append((x, min(y, P - y)))
        x += 1
    return result


def load_panel(path):
    rows = []
    with open(path, encoding="utf-8", errors="replace") as file:
        for line in file:
            if PREFIX in line:
                obj = json.loads(line.split(PREFIX, 1)[1])
                assert obj["schema"] == "prime31.b31.y_ranked_complete_256q.v1"
                rows.append(obj)
    assert len(rows) == 1, ("expected exactly one complete panel in source log", path, len(rows))
    return rows[0]


def replay(paths):
    assert len(paths) == 2
    check_point(G)
    assert scalar(N, G) is None
    samples = first_rational_points(256)
    sorted_y = sorted(samples, key=lambda t: (t[1], t[0]))[:31]
    x31 = samples[:31]
    assert len(set(x for x, y in sorted_y)) == 31
    assert len(set(x for x, y in x31)) == 31

    expected_orders = [
        ["first_x31_paired", "y_ranked31_paired"],
        ["y_ranked31_paired", "first_x31_paired"],
    ]
    checks = {"factor_base_logs": 0, "rank_rows": 0, "target_logs": 0,
              "signed_witnesses": 0}
    records_by_base = {}
    summaries = []
    for run_no, path in enumerate(paths):
        obj = load_panel(path)
        assert obj["field_p"] == P and obj["subgroup_n"] == N
        assert obj["same_public_target_count"] == 256
        assert obj["panel_order"] == expected_orders[run_no]
        assert len(obj["panels"]) == 2
        for panel in obj["panels"]:
            name = panel["base_kind"]
            expected = x31 if name == "first_x31_paired" else sorted_y
            base = [tuple(q) for q in panel["factor_base_prefix_points"]]
            assert base == expected, ("wrong y/x selection", name)
            assert len(base) == 31 and len(set(base)) == 31
            assert panel["rank"] == 31 and panel["unknown_logs_recovered"] == 256
            assert panel["generic_BSGS_fallback_count"] == 0
            logs = panel["factor_base_logs"]
            assert len(logs) == 31
            for q, ell in zip(base, logs):
                check_point(q)
                assert scalar(ell, G) == q, ("bad basis logarithm", name, q)
                checks["factor_base_logs"] += 1
            rows = panel["signed_modular_relation_rows"]
            assert len(rows) == 31
            for row in rows:
                assert len(row) == 32
                assert sum(c * ell for c, ell in zip(row[:31], logs)) % N == row[31], (
                    "bad learned modular equation", name)
                checks["rank_rows"] += 1
            transcript = panel["unknown_log_results"]
            assert len(transcript) == 256
            canonical_records = []
            for i, row in enumerate(transcript):
                assert row["index"] == i
                q = None if row["public_Q"] is None else tuple(row["public_Q"])
                check_point(q)
                k = row["recovered_k"]
                assert 0 <= k < N
                assert scalar(k, G) == q, ("bad unknown logarithm", name, i)
                checks["target_logs"] += 1
                witness = row["signed_eight_witness"]
                assert witness is not None and len(witness) == 8
                shift = row["first_successful_shift"]
                assert isinstance(shift, int) and 0 <= shift <= 8
                total = None
                for t in witness:
                    assert isinstance(t, int) and 0 <= t < 62
                    point = base[t // 2]
                    total = plus(total, point if t % 2 == 0 else neg(point))
                assert total == plus(q, scalar(shift, G)), (
                    "invalid signed witness", name, i)
                checks["signed_witnesses"] += 1
                canonical_records.append((q, k, shift, tuple(witness)))
            if name in records_by_base:
                assert records_by_base[name] == canonical_records, (
                    "AB/BA results changed", name)
            else:
                records_by_base[name] = canonical_records
            summaries.append({
                "ordering": run_no,
                "base": name,
                "charged_total_ms": int(panel["charged_total_ns"]) / 1e6,
                "first_shift_hits": panel["first_successful_public_shift_histogram"][0],
                "target_failed_attempts": panel["target_exhaustive_negative_attempts"],
                "rank": panel["rank"],
                "unknown_logs": panel["unknown_logs_recovered"],
                "four_sum_representatives": panel["negation_quotient_representatives"],
            })
    assert all(value > 0 for value in checks.values())
    return {"standing": "[pcc:unverified]", "independent_bigint_arithmetic": True,
            "replay_checks": checks,
            "x_y_base_overlap": len(set(x31) & set(sorted_y)),
            "matched_corpus": True, "paired_AB_BA_identical": True,
            "panels": summaries}


if __name__ == "__main__":
    assert len(sys.argv) == 3, "pass AB and BA log paths"
    print("PRIME31_Y_RANKED_INDEPENDENT_REPLAY_JSON=" +
          json.dumps(replay(sys.argv[1:]), separators=(",", ":"), sort_keys=True))

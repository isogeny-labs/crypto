#!/usr/bin/env python3
"""Independent Python integer-arithmetic replay of a Prime31 Rust parity transcript.

This checker does not import Rust's Arithmetic, factor-base selection, scalar
routine, witness validation or parity membership code. It proves neither
non-generic extraction nor a full ECDLP cost advantage. [pcc:unverified]
"""
import hashlib
import itertools
import json
import sys

P = (1 << 31) - 1
A = 847_514_846
B_COEFF = 2_087_444_724
N = 2_147_446_831
G = (625_037_715, 2_100_627_251)
WEIGHTS = (1, 3, 5, 7, 11, 13, 17, 19)
SEED = 0x57534C4F54503131
MASK = (1 << 64) - 1


def ec_add(u, v):
    """Independent textbook affine group law with integer modular inverse."""
    if u is None:
        return v
    if v is None:
        return u
    x, y = u
    z, w = v
    if x == z:
        if (y + w) % P == 0:
            return None
        den, num = (2 * y) % P, (3 * x * x + A) % P
    else:
        den, num = (z - x) % P, (w - y) % P
    slope = num * pow(den, -1, P) % P
    xx = (slope * slope - x - z) % P
    yy = (slope * (x - xx) - y) % P
    return xx, yy


def neg(q):
    return None if q is None else (q[0], (-q[1]) % P)


def ec_mul(k, q):
    result = None
    while k:
        if k & 1:
            result = ec_add(result, q)
        q = ec_add(q, q)
        k >>= 1
    return result


def on_curve(q):
    if q is None:
        return True
    x, y = q
    return (0 <= x < P and 0 <= y < P and
            (y*y - x*x*x - A*x - B_COEFF) % P == 0)


def point(raw):
    if raw is None:
        return None
    assert isinstance(raw, list) and len(raw) == 2
    return tuple(raw)


def first_rational_base(count):
    base = []
    x = 0
    while len(base) < count:
        y = pow((x*x*x + A*x + B_COEFF) % P, (P+1)//4, P)
        if (y*y-x*x*x-A*x-B_COEFF) % P == 0:
            assert y != 0
            base.append((x, min(y, P-y)))
        x += 1
    return base


def splitmix64(state):
    state = (state + 0x9E3779B97F4A7C15) & MASK
    z = state
    z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & MASK
    z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & MASK
    return state, z ^ (z >> 31)


def toggle(s, x):
    if x in s:
        s.remove(x)
    else:
        s.add(x)


def b8_four_slot_parity(slots, start):
    # Deliberately separate from the Rust pair-table implementation.
    support = set()
    for choices in itertools.product(range(8), repeat=4):
        total = None
        for i, choice in enumerate(choices):
            total = ec_add(total, slots[start+i][choice])
        toggle(support, total)
    return support


def main(path):
    with open(path, encoding="utf-8") as f:
        d = json.load(f)
    assert (d["source_curve_p"], d["source_curve_a"],
            d["source_curve_b"], d["source_group_order"]) == (P, A, B_COEFF, N)
    assert tuple(d["generator"]) == G
    assert tuple(d["weights"]) == WEIGHTS
    assert d["fixed_seed"] == SEED
    assert ec_mul(N, G) is None and on_curve(G)
    base = first_rational_base(31)
    assert all(on_curve(q) for q in base)

    state = SEED
    residuals = []
    for _ in range(12):
        state, z = splitmix64(state)
        k = 1 + z % (N - 1)
        residuals.append(ec_mul(k, G))
    verified_rows = 0
    verified_witnesses = 0
    b8_parity_controls = 0
    for b, row in zip((8, 16, 31), d["experimental_panels"]):
        assert row["b"] == b and row["raw_words_per_four_slot_half"] == b**4
        assert row["unweighted_theorem_odd_support_size"] == b
        slots = [[ec_mul(w, p) for p in base[:b]] for w in WEIGHTS]
        uniform = {ec_mul(8, p) for p in base[:b]}
        assert len(uniform) == b
        expected_plants = (ec_mul(8, base[0]),
                           sum_points([slot[0] for slot in slots]))
        for j, item in enumerate(row["controls"]):
            q = point(item["public_target"])
            assert q == expected_plants[j] and on_curve(q)
            assert item["uniform_slot_odd_parity"] == (q in uniform)
            indices = item["weighted_word_if_odd"]
            assert bool(indices) == item["weighted_slot_odd_parity"]
            if indices is not None:
                assert all(0 <= idx < b for idx in indices)
                assert sum_points([slots[i][idx] for i,idx in enumerate(indices)]) == q
                verified_witnesses += 1
        if b == 8:
            left = b8_four_slot_parity(slots, 0)
            right = b8_four_slot_parity(slots, 4)
            assert len(left) == row["left_parity_support"]
            assert len(right) == row["right_parity_support"]
        for i, item in enumerate(row["targets"]):
            q = point(item["public_target"])
            assert q == residuals[i] and on_curve(q)
            assert item["uniform_slot_odd_parity"] == (q in uniform)
            indices = item["weighted_witness_index_per_slot_if_odd"]
            assert bool(indices) == item["weighted_slot_odd_parity"]
            if b == 8:
                parity = False
                for u in left:
                    if ec_add(q, neg(u)) in right:
                        parity = not parity
                assert parity == item["weighted_slot_odd_parity"]
                b8_parity_controls += 1
            if indices is not None:
                assert len(indices) == 8 and all(0 <= idx < b for idx in indices)
                assert sum_points([slots[j][idx] for j,idx in enumerate(indices)]) == q
                verified_witnesses += 1
            verified_rows += 1
    assert verified_rows == 36
    assert d["new_nongeneric_relations"] == 0 and d["new_complete_ecdpl_logs"] == 0
    if len(sys.argv) == 3:
        with open(sys.argv[2], encoding="utf-8") as frozen_file:
            frozen = json.load(frozen_file)
        # The frozen SHA identifies the archived *original* run, not this
        # fresh run. JSON contains variable timings and HashMap iteration
        # can choose a different valid witness. Compare semantic invariants.
        with open(path, "rb") as original:
            _fresh_run_sha256 = hashlib.sha256(original.read()).hexdigest()
        assert len(frozen["original_full_json_sha256"]) == 64
        frozen_checked = 0
        assert frozen["target_points"] == [list(q) for q in residuals]
        assert frozen["p"] == P and frozen["n"] == N
        assert frozen["weights"] == list(WEIGHTS) and int(frozen["seed_hex"], 16) == SEED
        assert len(frozen["experiments"]) == 3
        for row, selected in zip(d["experimental_panels"], frozen["experiments"]):
            assert selected["B"] == row["b"]
            assert selected["left_support"] == row["left_parity_support"]
            assert selected["right_support"] == row["right_parity_support"]
            assert selected["odd_count"] == row["odd_residuals"]
            assert selected["setup_group_additions"] == row["charged_candidate_setup_operations"]["group_additions"]
            assert selected["query_group_additions"] == row["public_query_group_additions"]
            assert selected["setup_fp_muls"] == row["charged_candidate_setup_operations"]["fp_multiplications"]
            assert selected["setup_fp_inversions"] == row["charged_candidate_setup_operations"]["fp_inversions"]
            assert [w is not None for w in selected["witnesses"]] == [
                x["weighted_slot_odd_parity"] for x in row["targets"]]
            b = row["b"]
            slots = [[ec_mul(weight, p) for p in base[:b]] for weight in WEIGHTS]
            for q, word in zip(residuals, selected["witnesses"]):
                if word is not None:
                    assert len(word) == 8 and all(0 <= i < b for i in word)
                    assert sum_points([slots[j][ix] for j, ix in enumerate(word)]) == q
                    frozen_checked += 1
        assert frozen_checked == 14
    print(json.dumps({"verifier": "independent Python integer EC group arithmetic",
                      "verified_public_residuals": verified_rows,
                      "verified_signed_or_weighted_witnesses": verified_witnesses,
                      "independent_exact_b8_parity_queries": b8_parity_controls,
                      "archived_original_curve_witnesses_rechecked": frozen_checked if len(sys.argv) == 3 else None,
                      "status":"pass",
                      "independent_signed_PCC":False}, sort_keys=True))


def sum_points(points):
    total = None
    for q in points:
        total = ec_add(total, q)
    return total


if __name__ == "__main__":
    if len(sys.argv) not in (2, 3):
        raise SystemExit("usage: verify_prime31_weighted_slot_parity.py rust-transcript.json [frozen-witness-summary.json]")
    main(sys.argv[1])

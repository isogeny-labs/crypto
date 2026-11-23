#!/usr/bin/env python3
"""Independent original-curve BigInt replay for the fixed Prime31 masked-slot panel.
No Rust import, no target discrete logs supplied to the candidate. [pcc:unverified]
"""
import json
import sys
from pathlib import Path

P = 2**31 - 1
A = 847514846
B = 2087444724
ORDER = 2147446831
G = (625037715, 2100627251)
MASK = (1 << 64) - 1
MASK_SEED = 0x4d41534b50333131
RESIDUAL_SEED = 0x57534c4f54503131


def mix(state):
    state = (state + 0x9e3779b97f4a7c15) & MASK
    z = state
    z = ((z ^ (z >> 30)) * 0xbf58476d1ce4e5b9) & MASK
    z = ((z ^ (z >> 27)) * 0x94d049bb133111eb) & MASK
    return z ^ (z >> 31), state


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
        num, den = (3 * x*x + A) % P, (2 * y) % P
    else:
        num, den = (w - y) % P, (z - x) % P
    lam = num * pow(den, -1, P) % P
    xx = (lam * lam - x - z) % P
    yy = (lam * (x - xx) - y) % P
    return (xx, yy)


def scalar(k, point=G):
    result = None
    while k:
        if k & 1:
            result = ec_add(result, point)
        point = ec_add(point, point)
        k >>= 1
    return result


def first_points(count):
    out = []
    for x in range(P):
        rhs = (x*x*x + A*x + B) % P
        y = pow(rhs, (P+1)//4, P)
        if y*y % P == rhs:
            assert y != 0
            out.append((x, min(y, P-y)))
        if len(out) == count:
            return out
    raise AssertionError("not enough rational abscissae")


def selected(base):
    keep = (len(base) + 1)//2
    result = []
    for slot in range(8):
        ranking = []
        for j in range(len(base)):
            score, _ = mix(MASK_SEED ^ ((slot+1) << 32) ^ (j+1))
            ranking.append((score,j))
        indices = sorted(j for _,j in sorted(ranking)[:keep])
        result.append(indices)
    return result


def as_pt(point):
    return None if point is None else tuple(point)


def eight_sum(slots, word):
    result = None
    for i, j in enumerate(word):
        result = ec_add(result, slots[i][j])
    return result


def half(slots, start):
    k = len(slots[0])
    pairs1 = [(ec_add(slots[start][i], slots[start+1][j]), (i,j))
              for i in range(k) for j in range(k)]
    pairs2 = [(ec_add(slots[start+2][i], slots[start+3][j]), (i,j))
              for i in range(k) for j in range(k)]
    odd = {}
    for p,a in pairs1:
        for q,b in pairs2:
            value = ec_add(p,q)
            if value in odd:
                del odd[value]
            else:
                odd[value] = a + b
    return odd


def check():
    path = Path(sys.argv[1])
    d = json.loads(path.read_text(encoding="utf-8"))
    assert (d["source_curve_p"], d["source_curve_a"], d["source_curve_b"]) == (P,A,B)
    assert d["source_group_order"] == ORDER
    assert tuple(d["generator"]) == G
    assert d["mask_seed_hex"] == f"{MASK_SEED:#018x}"
    assert d["residual_seed_hex"] == f"{RESIDUAL_SEED:#018x}"
    assert d["exhaustive_b4_original_curve_parity_checked"]
    assert scalar(ORDER) is None
    base = first_points(31)
    state = RESIDUAL_SEED
    targets = []
    for _ in range(12):
        value, state = mix(state)
        targets.append(scalar(1 + value%(ORDER-1)))
    outcomes = []
    replayed = 0
    independent_parity_cases = 0
    mutations_rejected = 0
    for panel in d["experimental_panels"]:
        size = panel["b"]
        assert size in (8,16,31)
        basis = base[:size]
        expected = selected(basis)
        assert panel["selected_base_indices"] == expected
        slots = [[basis[j] for j in indices] for indices in expected]
        k = (size+1)//2
        assert panel["per_slot_size"] == k
        assert panel["raw_words_per_four_slot_half"] == k**4
        assert panel["same_slot_odd_support_theorem"] == k
        assert panel["left_build_additions"] == 2*k*k + k**4
        assert panel["right_build_additions"] == 2*k*k + k**4
        assert len(panel["targets"]) == 12
        assert panel["public_residuals"] == 12
        odd = 0
        for i, row in enumerate(panel["targets"]):
            assert row["case"] == i
            assert as_pt(row["public_target"]) == targets[i]
            witness = row["masked_witness_slot_local_indices_if_odd"]
            assert row["masked_odd_parity"] == (witness is not None)
            if witness is not None:
                assert len(witness) == 8
                assert all(0 <= v < k for v in witness)
                assert eight_sum(slots,witness) == targets[i]
                replayed += 1
                # An adversarially changed signed/index witness must reject.
                mutated = witness.copy()
                mutated[0] = (mutated[0] + 1) % k
                assert eight_sum(slots,mutated) != targets[i]
                mutations_rejected += 1
                odd += 1
        assert odd == panel["odd_residuals"]
        assert panel["public_query_group_additions"] == (
            12*panel["left_parity_support"]+8*odd)
        planted = as_pt(panel["planted_known_relation_target"])
        assert planted == eight_sum(slots,[0]*8)
        plant_witness = panel["planted_target_witness_if_odd"]
        assert panel["planted_target_odd_parity"] == (plant_witness is not None)
        if plant_witness is not None:
            assert eight_sum(slots,plant_witness) == planted
            replayed += 1
        # Second implementation of the entire parity oracle, bounded to B8/B16.
        if size <= 16:
            left = half(slots,0)
            right = half(slots,4)
            assert (len(left),len(right)) == (
                panel["left_parity_support"],panel["right_parity_support"])
            for i, target in enumerate(targets):
                count = 0
                for q in left:
                    need = ec_add(target, None if q is None else (q[0],(-q[1])%P))
                    count ^= int(need in right)
                assert bool(count) == panel["targets"][i]["masked_odd_parity"]
                independent_parity_cases += 1
        outcomes.append({"b":size,"odd_of_12":odd,"left_support":panel["left_parity_support"],
                         "setup_adds":panel["candidate_setup_operations"]["group_additions"],
                         "query_adds":panel["public_query_group_additions"]})
    assert [p["b"] for p in d["experimental_panels"]] == [8,16,31]
    return {
        "verified":"PASS",
        "original_curve_and_generator":True,
        "exact_frozen_targets":12,
        "masked_target_rows":36,
        "witnesses_replayed":replayed,
        "witness_mutations_rejected":mutations_rejected,
        "full_independent_parity_decisions_B8_B16":independent_parity_cases,
        "B31_parity_independent":False,
        "panels":outcomes,
        "pcc":"unverified",
    }


if __name__ == "__main__":
    print(json.dumps(check(),sort_keys=True))

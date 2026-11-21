"""Construct and check rational word-two curves for the frozen bridge rows."""

import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ, ZZ, matrix

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
BRIDGE = ROOT / "xedni_word2_word6_bridge_20261121" / "result.json"


def rational_string(x):
    return str(x)


def residue(x, p):
    return int(GF(p)(x))


def folded_key(point):
    xy = tuple(map(int, point.xy()))
    neg_xy = tuple(map(int, (-point).xy()))
    return min(xy, neg_xy), 1 if xy <= neg_xy else -1


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    bridge_bytes = BRIDGE.read_bytes()
    assert hashlib.sha256(bridge_bytes).hexdigest() == protocol["bridge_result_sha256"]
    bridge = json.loads(bridge_bytes)
    expected = protocol["target_source_indices"]
    selected = {r["source_index"]: r for r in bridge["bridge_witness"]}
    rows = []
    p = protocol["field_prime"]
    E0 = EllipticCurve(GF(p), protocol["reduced_curve_a_invariants"])
    assert E0.cardinality() == 43
    for index in expected:
        entry = selected[index]
        xp, yp = map(ZZ, entry["P"])
        xq, yq = map(ZZ, entry["Q"])
        row = {"source_index": index, "P": entry["P"], "Q": entry["Q"]}
        if (xp-xq) % p == 0:
            row["status"] = "x_difference_divisible_by_47"
            rows.append(row)
            continue
        lam = QQ(yp-yq)/QQ(xp-xq)
        a2 = lam**2-2*xq-xp
        a4 = 2*yq*lam-3*xq**2-2*a2*xq
        a6 = yq**2-xq**3-a2*xq**2-a4*xq
        coeffs = [QQ(0), a2, QQ(0), a4, a6]
        row.update({"lambda": rational_string(lam),
                    "a_invariants": list(map(rational_string, coeffs)),
                    "max_coefficient_bits": max(max(abs(x.numerator()).nbits(), x.denominator().nbits()) for x in coeffs)})
        try:
            E = EllipticCurve(QQ, coeffs)
            P = E(xp, yp)
            Q = E(xq, yq)
            assert P + 2*Q == E(0)
            assert [residue(x, p) for x in coeffs] == [0, 0, 0, p-1, p-2]
            assert residue(E.discriminant(), p) != 0
            P0, Q0 = E0(entry["P"]), E0(entry["Q"])
            assert P0+2*Q0 == E0(0)
            row.update({"status": "PASS", "discriminant_mod_47": residue(E.discriminant(), p),
                        "rational_relation": True, "reduced_relation": True})
        except Exception as error:
            row.update({"status": "FAIL", "error": str(error)})
        rows.append(row)
    points = [P for P in E0.points() if P != E0(0)]
    columns = sorted({folded_key(P)[0] for P in points})
    assert len(points) == 42 and len(columns) == 21
    column_index = {xy: i for i, xy in enumerate(columns)}

    def relation_row(P, Q, coefficient):
        result = [0] * len(columns)
        for point, weight in ((P, 1), (Q, coefficient)):
            key, sign = folded_key(point)
            result[column_index[key]] += sign * weight
        return result

    six_rows = [relation_row(P, 7*P, 6) for P in points]
    bridge_rows = []
    for entry in bridge["bridge_witness"]:
        P, Q = E0(entry["P"]), E0(entry["Q"])
        bridge_rows.append(relation_row(P, Q, 2))
    step_ranks = [int(matrix(GF(43), six_rows+bridge_rows[:i]).rank()) for i in range(1, 7)]
    assert step_ranks == list(range(15, 21))
    print(json.dumps({"schema": "xedni-direct-word2-lifts-result-v1",
                      "status": "PASS" if all(r["status"] == "PASS" for r in rows) else "FAIL",
                      "curve_order_mod_47": 43, "folded_columns": len(columns),
                      "bridge_step_ranks": step_ranks, "rows": rows}, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()

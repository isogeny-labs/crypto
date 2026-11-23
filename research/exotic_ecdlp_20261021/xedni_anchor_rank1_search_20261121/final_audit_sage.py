"""Independent six-curve group, rank, reduction, and matrix replay."""

import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ, matrix

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
BRIDGE = ROOT / "xedni_word2_word6_bridge_20261121" / "result.json"
DIRECT_AUDIT = ROOT / "xedni_direct_bridge_ranks_20261121" / "audit-result.json"
REPLACE_AUDIT = ROOT / "xedni_bridge_rank1_replacements_20261121" / "audit-result.json"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def folded_key(point):
    xy = tuple(map(int, point.xy()))
    neg_xy = tuple(map(int, (-point).xy()))
    return min(xy, neg_xy), 1 if xy <= neg_xy else -1


def main():
    protocol = json.loads((HERE / "final-audit-protocol.json").read_text())
    assert sha(BRIDGE) == protocol["bridge_result_sha256"]
    assert sha(DIRECT_AUDIT) == protocol["direct_rank_audit_sha256"]
    assert sha(REPLACE_AUDIT) == protocol["replacement_audit_sha256"]
    bridge = json.loads(BRIDGE.read_text())
    assert [r["source_index"] for r in bridge["bridge_witness"]] == protocol["selected_source_order"]
    E0 = EllipticCurve(GF(47), [0,0,0,-1,-2])
    assert E0.cardinality() == 43
    nonzero = [P for P in E0.points() if P != E0(0)]
    columns = sorted({folded_key(P)[0] for P in nonzero})
    assert len(nonzero) == 42 and len(columns) == 21
    col = {xy:i for i,xy in enumerate(columns)}

    def row(P, Q, coefficient):
        v = [0]*21
        for point, c in ((P,1),(Q,coefficient)):
            key, sign = folded_key(point)
            v[col[key]] += sign*c
        return v

    word6 = [row(P,7*P,6) for P in nonzero]
    assert matrix(GF(43),word6).rank() == 14
    selected = []
    rows = []
    for item in bridge["bridge_witness"]:
        index = item["source_index"]
        coeffs = [QQ(x) for x in protocol["selected_coefficients"][str(index)]]
        E = EllipticCurve(QQ, coeffs)
        P = E(item["P"])
        Q = E(item["Q"])
        assert P+2*Q == E(0)
        assert [int(GF(47)(a)) for a in E.a_invariants()] == [0,0,0,46,45]
        assert int(GF(47)(E.discriminant())) != 0
        rank = int(E.rank(use_database=False, proof=True, algorithm="pari", only_use_mwrank=True))
        assert rank == 1
        P0,Q0 = E0(item["P"]),E0(item["Q"])
        assert P0+2*Q0 == E0(0)
        selected.append(row(P0,Q0,2))
        r = int(matrix(GF(43),word6+selected).rank())
        assert r == item["rank_after"]
        rows.append({"source_index":index,"rank":rank,"rank_after_row":r,
                     "j_invariant":str(E.j_invariant()),"a_invariants":list(map(str,coeffs))})
    print(json.dumps({"schema":"xedni-six-rank1-bridge-final-audit-v1", "status":"PASS",
                      "base_rank":14,"folded_columns":21,"selected_rank":20,
                      "curve_order_mod_47":43,"curves":rows},indent=2,sort_keys=True))


if __name__ == "__main__":
    main()

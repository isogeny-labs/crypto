"""Independent exact-fraction audit of the five direct rational lifts."""

import hashlib
import json
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
BRIDGE = HERE.parent / "xedni_word2_word6_bridge_20261121" / "result.json"


def mod(x, p):
    return (x.numerator * pow(x.denominator, -1, p)) % p


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    assert hashlib.sha256(BRIDGE.read_bytes()).hexdigest() == protocol["bridge_result_sha256"]
    original = {row["source_index"]: row for row in json.loads(BRIDGE.read_text())["bridge_witness"]}
    result = json.loads((HERE / "result.json").read_text())
    assert [row["source_index"] for row in result["rows"]] == protocol["target_source_indices"]
    assert result["bridge_step_ranks"] == [15, 16, 17, 18, 19, 20]
    for row in result["rows"]:
        index = row["source_index"]
        assert row["P"] == original[index]["P"] and row["Q"] == original[index]["Q"]
        xp, yp = map(F, row["P"])
        xq, yq = map(F, row["Q"])
        assert xp != xq and 2*yq != 0
        lam = (yp-yq)/(xp-xq)
        a1, a2, a3, a4, a6 = map(F, row["a_invariants"])
        assert (a1, a3) == (0, 0)
        assert str(lam) == row["lambda"]
        assert a2 == lam**2-2*xq-xp
        assert a4 == 2*yq*lam-3*xq*xq-2*a2*xq
        assert a6 == yq*yq-xq**3-a2*xq*xq-a4*xq
        for x, y in ((xp, yp), (xq, yq)):
            assert y*y == x**3+a2*x*x+a4*x+a6
        tangent = (3*xq*xq+2*a2*xq+a4)/(2*yq)
        assert tangent == lam
        assert tangent*tangent-a2-2*xq == xp
        assert tangent*(xq-xp)-yq == -yp
        assert list(map(lambda z: mod(z, 47), (a1,a2,a3,a4,a6))) == [0,0,0,46,45]
        assert row["status"] == "PASS"
    print(json.dumps({"schema": "xedni-direct-word2-lifts-audit-v1", "status": "PASS",
                      "checked_rational_relations": len(result["rows"]),
                      "bridge_step_ranks_from_sage": result["bridge_step_ranks"]}, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()

"""One capped Sage rank-method arm for a frozen direct bridge curve."""

import json
import sys
from pathlib import Path

from sage.all import EllipticCurve, QQ

HERE = Path(__file__).resolve().parent
SOURCE = HERE.parent / "xedni_direct_word2_lifts_20261121" / "result.json"


def main():
    index, method = int(sys.argv[1]), sys.argv[2]
    row = next(r for r in json.loads(SOURCE.read_text())["rows"] if r["source_index"] == index)
    E = EllipticCurve(QQ, [QQ(c) for c in row["a_invariants"]])
    Q = E([QQ(v) for v in row["Q"]])
    if method == "simon_two_descent":
        lower, upper, witnesses = E.simon_two_descent(known_points=[Q], limbigprime=0)
        result = {"status": "complete", "rank_lower": int(lower), "rank_upper": int(upper),
                  "witnesses": [[str(P[0]), str(P[1])] for P in witnesses]}
    elif method == "pari_proof_rank":
        rank = E.rank(use_database=False, proof=True, algorithm="pari", only_use_mwrank=True)
        result = {"status": "complete", "rank": int(rank), "proof": True}
    else:
        raise ValueError(method)
    print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()

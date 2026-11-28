"""Check one rational replacement curve and run one rank method."""

import json
import sys

from sage.all import EllipticCurve, GF, QQ


def main():
    payload = json.loads(sys.argv[1])
    method = sys.argv[2]
    E = EllipticCurve(QQ, [QQ(x) for x in payload["a_invariants"]])
    P = E([QQ(x) for x in payload["P"]])
    Q = E([QQ(x) for x in payload["Q"]])
    assert P+2*Q == E(0)
    assert [int(GF(47)(x)) for x in E.a_invariants()] == [0,0,0,46,45]
    assert int(GF(47)(E.discriminant())) != 0
    if method == "simon_two_descent":
        lower, upper, witnesses = E.simon_two_descent(known_points=[Q], limbigprime=0)
        result = {"rank_lower": int(lower), "rank_upper": int(upper),
                  "witnesses": [[str(P[0]),str(P[1])] for P in witnesses]}
    elif method == "pari_proof_rank":
        result = {"rank": int(E.rank(use_database=False, proof=True,
                    algorithm="pari", only_use_mwrank=True)), "proof": True}
    else:
        raise ValueError(method)
    print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()

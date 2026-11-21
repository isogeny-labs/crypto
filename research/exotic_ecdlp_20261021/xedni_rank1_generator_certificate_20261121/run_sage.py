"""Proof-aware free-generator and torsion check on the selected curve."""

import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ

HERE=Path(__file__).resolve().parent
ROOT=HERE.parent


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    p=json.loads((HERE/"protocol.json").read_text())
    assert sha(ROOT/"xedni_anchor_rank1_search_20261121"/"final-audit-result.json")==p["source_rank_audit_sha256"]
    assert sha(ROOT/"xedni_process_fresh_target_20261121"/"result.json")==p["source_target_result_sha256"]
    E=EllipticCurve(QQ,[QQ(v) for v in p["curve_a_invariants"]])
    Q=E([QQ(v) for v in p["Q"]])
    gens=E.gens(proof=True,use_database=False,algorithm="mwrank_lib",sat_bound=1000)
    certain=bool(E.gens_certain())
    torsion=E.torsion_subgroup()
    torsion_order=int(torsion.order())
    E0=EllipticCurve(GF(47),[0,0,0,-1,-2])
    Q0=E0([GF(47)(v) for v in p["Q"]])
    assert Q0.order()==43
    rank=int(E.rank(use_database=False,proof=True,algorithm="pari",only_use_mwrank=True))
    assert rank==1 and len(gens)==1
    g=gens[0]
    direct=Q==g or Q==-g
    bounded_matches=[n for n in range(-12,13) if n*g==Q]
    print(json.dumps({"schema":"xedni-rank1-generator-result-v1","status":"PASS",
        "rank":rank,"generators_certain":certain,"torsion_order":torsion_order,
        "generator_xy":[str(g[0]),str(g[1])],"Q_xy":[str(Q[0]),str(Q[1])],
        "Q_equals_signed_generator":direct,"bounded_multiplier_matches":bounded_matches,
        "reduced_Q_order":int(Q0.order())},indent=2,sort_keys=True))


if __name__=="__main__":main()

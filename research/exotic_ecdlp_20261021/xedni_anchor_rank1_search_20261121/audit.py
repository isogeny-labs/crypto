"""Independent exact-arithmetic and record audit of the replacement grid."""

import hashlib
import json
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
SOURCE = HERE.parent / "xedni_word2_word6_bridge_20261121" / "result.json"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def mod(x,p):
    return x.numerator*pow(x.denominator,-1,p)%p


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    assert sha(SOURCE) == protocol["source_result_sha256"]
    source = {r["source_index"]: r for r in json.loads(SOURCE.read_text())["bridge_witness"]}
    rows = [json.loads(s) for s in (HERE / "cells.jsonl").read_text().splitlines()]
    assert [(r["source_index"],r["u"],r["v"]) for r in rows] == [
        (i,u,v) for i in [protocol["source_index"]] for u in protocol["offset_order"]
        for v in protocol["offset_order"]]
    for r in rows:
        xp,yp = map(F, source[r["source_index"]]["P"])
        xq,yq = map(F, source[r["source_index"]]["Q"])
        lam=(yp-yq)/(xp-xq)
        a1,a2,a3,a4,a6=map(F,r["a_invariants"])
        assert a1==47*r["u"] and a3==47*r["v"]
        assert a2==lam*lam+a1*lam-2*xq-xp
        assert a4==lam*(2*yq+a1*xq+a3)-3*xq*xq-2*a2*xq+a1*yq
        assert a6==yq*yq+a1*xq*yq+a3*yq-xq**3-a2*xq*xq-a4*xq
        assert [mod(a,47) for a in (a1,a2,a3,a4,a6)] == [0,0,0,46,45]
        for x,y in ((xp,yp),(xq,yq)):
            assert y*y+a1*x*y+a3*y==x**3+a2*x*x+a4*x+a6
        tangent=(3*xq*xq+2*a2*xq+a4-a1*yq)/(2*yq+a1*xq+a3)
        assert tangent==lam
        assert tangent*tangent+a1*tangent-a2-2*xq==xp
        assert -(tangent+a1)*xp-(yq-tangent*xq)-a3==-yp-a1*xp-a3
        simon=r["simon"]
        assert simon["status"]=="complete" and simon["exit_code"]==0
        assert simon["elapsed_ns"]<25*10**9
        if simon["result"]["rank_upper"]<=1:
            pari=r["pari"]
            assert pari["status"]=="complete" and pari["exit_code"]==0
            assert pari["result"]["proof"] is True
            assert pari["result"]["rank"]==simon["result"]["rank_lower"]==simon["result"]["rank_upper"]==1
        else:
            assert r["pari"]["status"]=="not_selected_by_simon_gate"
    summary=json.loads((HERE/"summary.json").read_text())
    assert summary["cells"]==len(rows)==9
    for index in [protocol["source_index"]]:
        wins=[r for r in rows if r["source_index"]==index and r["pari"]["status"]=="complete" and r["pari"]["result"]["rank"]==1]
        assert summary["rank_one_witnesses"][str(index)]==[
            {"u":r["u"],"v":r["v"],"a_invariants":r["a_invariants"]} for r in wins]
    print(json.dumps({"schema":"xedni-anchor-rank1-search-audit-v1", "status":"PASS",
                      "cells":len(rows),"rank_one_counts":{str(i):len(summary["rank_one_witnesses"][str(i)]) for i in [protocol["source_index"]]},
                      "cells_sha256":sha(HERE/"cells.jsonl")},indent=2,sort_keys=True))


if __name__=="__main__":
    main()

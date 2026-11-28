"""Measure a single rational generator's lift height and reduced word span."""

import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ, matrix

HERE = Path(__file__).resolve().parent
SOURCE = HERE.parent / "xedni_anchor_rank1_search_20261121" / "final-audit-result.json"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def point_bits(point):
    return max(max(abs(v.numerator()).nbits(),v.denominator().nbits()) for v in point.xy())


def folded(point):
    xy=tuple(map(int,point.xy()))
    neg=tuple(map(int,(-point).xy()))
    return min(xy,neg), 1 if xy <= neg else -1


def main():
    p=json.loads((HERE/"protocol.json").read_text())
    assert sha(SOURCE.read_bytes())==p["source_final_audit_sha256"]
    source=json.loads(SOURCE.read_text())
    assert source["status"]=="PASS" and source["curves"][0]["a_invariants"]==p["rational_curve_a_invariants"]
    E=EllipticCurve(QQ,[QQ(v) for v in p["rational_curve_a_invariants"]])
    Q=E([QQ(v) for v in p["rational_generator"]])
    E0=EllipticCurve(GF(47),p["reduced_curve_a_invariants"])
    Q0=E0([GF(47)(v) for v in p["rational_generator"]])
    assert E0.cardinality()==p["subgroup_order"] and Q0.order()==p["subgroup_order"]
    points=[P for P in E0.points() if P!=E0(0)]
    columns=sorted({folded(P)[0] for P in points})
    assert len(columns)==21
    col={xy:i for i,xy in enumerate(columns)}

    def relation_row(P,B,m):
        row=[0]*21
        for point,c in ((P,1),(B,m)):
            xy,sgn=folded(point)
            row[col[xy]]+=sgn*c
        return row

    rows=[]
    matrix_rows=[]
    seen_folded=set()
    start,stop=p["multiplier_prefix"]
    assert start==1 and stop==21
    for n in range(start,stop+1):
        B=n*Q
        B0=n*Q0
        assert E0([GF(47)(v) for v in B.xy()])==B0
        seen_folded.add(folded(B0)[0])
        words=[]
        for m in p["relation_words"]:
            P=-m*B
            P0=-m*B0
            assert P+m*B==E(0)
            assert E0([GF(47)(v) for v in P.xy()])==P0
            matrix_rows.append(relation_row(P0,B0,m))
            words.append({"m":m,"P_bits":point_bits(P),
                          "P_xy_sha256":sha(str(tuple(P.xy())).encode())})
        rank=int(matrix(GF(43),matrix_rows).rank())
        row={"n":n,"Q_bits":point_bits(B),"Q_xy_sha256":sha(str(tuple(B.xy())).encode()),
             "word_points":words,"max_bits":max([point_bits(B)]+[w["P_bits"] for w in words]),
             "rank":rank,"folded_generator_points":len(seen_folded)}
        rows.append(row)
        print(json.dumps(row,sort_keys=True),flush=True)
    assert len(seen_folded)==21 and rows[-1]["rank"]==20
    (HERE/"summary.json").write_text(json.dumps({"schema":"xedni-fixed-rank1-height-summary-v1",
        "status":"PASS","prefix_count":len(rows),"final_rank":rows[-1]["rank"],
        "max_coordinate_bits":max(r["max_bits"] for r in rows),
        "first_rank20_prefix":next(r["n"] for r in rows if r["rank"]==20),
        "rows_sha256":sha((HERE/"rows.jsonl").read_bytes())},indent=2,sort_keys=True)+"\n")


if __name__=="__main__":
    main()

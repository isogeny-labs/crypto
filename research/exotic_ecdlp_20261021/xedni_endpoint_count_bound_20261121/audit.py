"""Independent point-group replay of the endpoint coverage table."""

import hashlib
import json
from pathlib import Path

HERE=Path(__file__).resolve().parent
SOURCE=HERE.parent/"xedni_fixed_rank1_height_20261121"
P=47
O=None


def add(A,B):
    if A is O:return B
    if B is O:return A
    x1,y1=A;x2,y2=B
    if x1==x2 and (y1+y2)%P==0:return O
    if A==B:
        slope=(3*x1*x1-1)*pow(2*y1,-1,P)%P
    else:
        slope=(y2-y1)*pow((x2-x1)%P,-1,P)%P
    x3=(slope*slope-x1-x2)%P
    return (x3,(slope*(x1-x3)-y1)%P)


def mul(n,A):
    result=O
    while n:
        if n&1:result=add(result,A)
        A=add(A,A);n//=2
    return result


def main():
    protocol=json.loads((HERE/"protocol.json").read_text())
    for name,key in (("rows.jsonl","source_rows_sha256"),("summary.json","source_summary_sha256")):
        assert hashlib.sha256((SOURCE/name).read_bytes()).hexdigest()==protocol[key]
    result=json.loads((HERE/"result.json").read_text())
    Q=(17,10)
    assert mul(43,Q) is O and len({mul(n,Q) for n in range(1,43)})==42
    assert [r["prefix"] for r in result["prefixes"]]==list(range(1,22))
    for rec in result["prefixes"]:
        M=rec["prefix"]
        points={mul(k%43,Q) for n in range(1,M+1) for c in (1,2,6) for k in (c*n,-c*n)}
        assert O not in points
        assert len(points)==rec["covered_targets"]
        assert rec["uniform_hit_probability_exact"]==f"{len(points)}/42"
        assert rec["generic_count_bound"]==min(42,6*M)
        source=json.loads((SOURCE/"rows.jsonl").read_text().splitlines()[M-1])
        assert (rec["relation_rank"],rec["max_coordinate_bits"])==(source["rank"],source["max_bits"])
    assert result["first_full_coverage_prefix"]==result["first_rank20_prefix"]==14
    print(json.dumps({"schema":"xedni-endpoint-count-audit-v1","status":"PASS",
                      "point_order":43,"prefixes":21,"coverage_at_13":result["prefixes"][12]["covered_targets"],
                      "coverage_at_14":result["prefixes"][13]["covered_targets"]},indent=2,sort_keys=True))


if __name__=="__main__":main()

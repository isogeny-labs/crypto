"""Independent Fraction and integer-modular replay of the fixed-curve panel."""

import hashlib
import json
from fractions import Fraction as F
from pathlib import Path

HERE=Path(__file__).resolve().parent
SOURCE=HERE.parent/"xedni_anchor_rank1_search_20261121"/"final-audit-result.json"
O=None


def neg(P):
    return O if P is O else (P[0],-P[1])


def add(P,Q,a2,a4):
    if P is O:return Q
    if Q is O:return P
    x1,y1=P;x2,y2=Q
    if x1==x2 and y1==-y2:return O
    if P==Q:
        slope=(3*x1*x1+2*a2*x1+a4)/(2*y1)
    else:
        slope=(y2-y1)/(x2-x1)
    x3=slope*slope-a2-x1-x2
    return (x3,slope*(x1-x3)-y1)


def mul(n,P,a2,a4):
    result=O
    while n:
        if n&1:result=add(result,P,a2,a4)
        P=add(P,P,a2,a4)
        n//=2
    return result


def bits(P):
    return max(max(abs(v.numerator).bit_length(),v.denominator.bit_length()) for v in P)


def modfrac(v,p):
    return v.numerator*pow(v.denominator,-1,p)%p


def fold(P):
    negp=(P[0],(-P[1])%47)
    return (min(P,negp),1 if P<=negp else -1)


def rank(rows,p):
    M=[[v%p for v in row] for row in rows]
    if not M:return 0
    r=0
    for c in range(len(M[0])):
        pivot=next((i for i in range(r,len(M)) if M[i][c]),None)
        if pivot is None:continue
        M[r],M[pivot]=M[pivot],M[r]
        inv=pow(M[r][c],-1,p)
        M[r]=[(v*inv)%p for v in M[r]]
        for i in range(r+1,len(M)):
            factor=M[i][c]
            if factor:M[i]=[(a-factor*b)%p for a,b in zip(M[i],M[r])]
        r+=1
        if r==len(M):break
    return r


def main():
    protocol=json.loads((HERE/"protocol.json").read_text())
    assert hashlib.sha256(SOURCE.read_bytes()).hexdigest()==protocol["source_final_audit_sha256"]
    rows=[json.loads(s) for s in (HERE/"rows.jsonl").read_text().splitlines()]
    assert [r["n"] for r in rows]==list(range(1,22))
    a2,a4=F(-47),F(751)
    Q=(F(17),F(10))
    # Enumerate the reduction independently using the same group formula over F47.
    def madd(P,Q):
        if P is O:return Q
        if Q is O:return P
        x1,y1=P;x2,y2=Q
        if x1==x2 and (y1+y2)%47==0:return O
        if P==Q:s=(3*x1*x1+2*(-47)*x1+751)*pow(2*y1,-1,47)%47
        else:s=(y2-y1)*pow((x2-x1)%47,-1,47)%47
        x3=(s*s-(-47)-x1-x2)%47
        return (x3,(s*(x1-x3)-y1)%47)
    def mmul(n,P):
        R=O
        while n:
            if n&1:R=madd(R,P)
            P=madd(P,P);n//=2
        return R
    q0=(17,10)
    points=[mmul(n,q0) for n in range(1,43)]
    assert len(set(points))==42 and mmul(43,q0) is O
    columns=sorted({fold(P)[0] for P in points})
    assert len(columns)==21
    col={v:i for i,v in enumerate(columns)}
    matrix_rows=[]
    for rec in rows:
        n=rec["n"]
        B=mul(n,Q,a2,a4)
        B0=mmul(n,q0)
        assert (modfrac(B[0],47),modfrac(B[1],47))==B0
        assert bits(B)==rec["Q_bits"]
        for word in rec["word_points"]:
            m=word["m"]
            P=neg(mul(m,B,a2,a4))
            P0=mmul((43-m*n)%43,q0)
            assert (modfrac(P[0],47),modfrac(P[1],47))==P0
            assert add(P,mul(m,B,a2,a4),a2,a4) is O
            assert bits(P)==word["P_bits"]
            vector=[0]*21
            for point,weight in ((P0,1),(B0,m)):
                xy,sgn=fold(point)
                vector[col[xy]]+=weight*sgn
            matrix_rows.append(vector)
        assert rank(matrix_rows,43)==rec["rank"]
        assert max([rec["Q_bits"]]+[w["P_bits"] for w in rec["word_points"]])==rec["max_bits"]
    summary=json.loads((HERE/"summary.json").read_text())
    assert summary["rows_sha256"]==hashlib.sha256((HERE/"rows.jsonl").read_bytes()).hexdigest()
    assert summary["first_rank20_prefix"]==14 and summary["max_coordinate_bits"]==16293
    coverage13={k*n%43 for n in range(1,14) for k in (1,2,6,-1,-2,-6)}
    coverage14={k*n%43 for n in range(1,15) for k in (1,2,6,-1,-2,-6)}
    assert len(coverage13)==40 and 15 not in coverage13 and 28 not in coverage13
    assert len(coverage14)==42
    print(json.dumps({"schema":"xedni-fixed-rank1-height-audit-v1","status":"PASS",
                      "prefixes":21,"first_rank20_prefix":14,"rank_at_21":20,
                      "bits_at_14":rows[13]["max_bits"],"bits_at_21":rows[20]["max_bits"],
                      "endpoint_coverage_at_13":len(coverage13),
                      "endpoint_coverage_at_14":len(coverage14)},indent=2,sort_keys=True))


if __name__=="__main__":main()

#!/usr/bin/env python3
"""Independent finite-field group replay with exact rational score order."""

import gzip
import hashlib
import json
from fractions import Fraction as F
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[2]


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def add(P,Q,c,p):
    if P is None:
        return Q
    if Q is None:
        return P
    a1,a2,a3,a4,a6=c
    x1,y1=P
    x2,y2=Q
    if x1==x2 and (y1+y2+a1*x1+a3)%p==0:
        return None
    if P==Q:
        num=3*x1*x1+2*a2*x1+a4-a1*y1
        den=2*y1+a1*x1+a3
        nu_num=-x1**3+a4*x1+2*a6-a3*y1
    else:
        num=y2-y1
        den=x2-x1
        nu_num=y1*x2-y2*x1
    inv=pow(den%p,-1,p)
    lam=num*inv%p
    nu=nu_num*inv%p
    x3=(lam*lam+a1*lam-a2-x1-x2)%p
    y3=(-(lam+a1)*x3-nu-a3)%p
    return x3,y3


def mul(k,P,c,p):
    out=None
    while k:
        if k&1:
            out=add(out,P,c,p)
        P=add(P,P,c,p)
        k//=2
    return out


def reduce_pair(pair,p):
    x,y=[F(z) for z in pair]
    return x.numerator*pow(x.denominator,-1,p)%p,y.numerator*pow(y.denominator,-1,p)%p


def main():
    protocol=json.loads((HERE/"protocol.json").read_bytes())
    result=json.loads((HERE/"result.json").read_bytes())
    compressed=(HERE/"first-hit-rows.jsonl.gz").read_bytes()
    assert sha(compressed)==result["first_hit_rows_gzip_sha256"]
    raw=gzip.decompress(compressed)
    assert sha(raw)==result["first_hit_rows_raw_sha256"]
    rows=[json.loads(line) for line in raw.splitlines()]
    assert len(rows)==result["target_rows"]==3802
    panel=json.loads((ROOT/protocol["source_panel_protocol"]).read_bytes())
    hqq,hqr,hrr=[F(z) for z in (panel["height_pairing_matrix"][0][0],
                                panel["height_pairing_matrix"][0][1],
                                panel["height_pairing_matrix"][1][1])]
    audited=[]
    for cell,summary in zip(protocol["cells"],result["cells"]):
        p,q,lam,B=[cell[k] for k in ("p","q","R_over_Q","radius")]
        curve=[int(v)%p for v in panel["rational_curve_coefficients"]]
        G=reduce_pair(panel["Q"],p)
        Rbar=reduce_pair(panel["R"],p)
        assert mul(q,G,curve,p) is None and mul(lam,G,curve,p)==Rbar
        scalar_of={mul(k,G,curve,p):k for k in range(q)}
        assert len(scalar_of)==q and scalar_of[None]==0
        labels=[(a,b) for a in range(-B,B+1) for b in range(-B,B+1) if (a,b)!=(0,0)]
        labels.sort(key=lambda z:(hqq*z[0]**2+2*hqr*z[0]*z[1]+hrr*z[1]**2,z))
        gmult={a:mul(a%q,G,curve,p) for a in range(-B,B+1)}
        rmult={b:mul(b%q,Rbar,curve,p) for b in range(-B,B+1)}
        first={}
        for i,(a,b) in enumerate(labels,1):
            P=add(gmult[a],rmult[b],curve,p)
            k=scalar_of[P]
            assert k==(a+lam*b)%q
            if k:
                first.setdefault(k,(i,a,b))
            if len(first)==q-1:
                break
        assert i==summary["coverage_index"]
        expected=[{"p":p,"q":q,"target_scalar":k,"first_hit_index":first[k][0],
                   "first_label":list(first[k][1:])} for k in range(1,q)]
        saved=[r for r in rows if r["p"]==p]
        assert saved==expected
        values=sorted(v[0] for v in first.values())
        n=q-1
        assert sum(values)==summary["sum_first_hit_indices"]
        assert values[n//2-1]+values[n//2]==summary["median_times_two"]
        assert values[(9*n+9)//10-1]==summary["p90_nearest_rank"]
        assert values[-1]==summary["max_first_hit_index"]
        audited.append({"p":p,"q":q,"candidate_evaluations_replayed":i,
                        "target_first_hits_verified":n,"sum_first_hit_indices":sum(values)})
    return {"schema":"xedni-first-hit-distribution-audit-v1","status":"PASS",
            "first_hit_rows_raw_sha256":sha(raw),"cells":audited}


if __name__=="__main__":
    print(json.dumps(main(),sort_keys=True,indent=2))

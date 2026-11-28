#!/usr/bin/env python3
"""Independent Fraction and finite-field replay of every completed candidate."""

import hashlib
import importlib.util
import json
from decimal import Decimal, getcontext
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
PANEL_AUDIT = HERE.parent / "xedni_preregistered_prime_panel_20261120" / "audit.py"
spec = importlib.util.spec_from_file_location("xedni_panel_audit", PANEL_AUDIT)
panel_audit = importlib.util.module_from_spec(spec)
spec.loader.exec_module(panel_audit)
multiples = panel_audit.multiples
finite_multiples = panel_audit.finite_multiples
reduce_point = panel_audit.reduce_point
bit_height = panel_audit.bit_height
sha = panel_audit.sha
add = panel_audit.add


def main():
    protocol_raw=(HERE/"protocol.json").read_bytes()
    protocol=json.loads(protocol_raw)
    result=json.loads((HERE/"result.json").read_bytes())
    assert sha(protocol_raw)==result["protocol_sha256"]
    raw=(HERE/"rows.jsonl").read_bytes()
    assert sha(raw)==result["rows_sha256"]
    rows=[json.loads(line) for line in raw.splitlines()]
    assert len(rows)==len(protocol["cells"])==result["cells"]
    panel=json.loads((ROOT/protocol["source_panel_protocol"]).read_bytes())
    curve=[F(z) for z in panel["rational_curve_coefficients"]]
    Q=tuple(F(z) for z in panel["Q"])
    R=tuple(F(z) for z in panel["R"])
    maxB=max(c["radius"] for c in protocol["cells"])
    qmult=multiples(Q,maxB,curve)
    rmult=multiples(R,maxB,curve)
    getcontext().prec=50
    hqq,hqr,hrr=[Decimal(z) for z in (panel["height_pairing_matrix"][0][0],
                                        panel["height_pairing_matrix"][0][1],
                                        panel["height_pairing_matrix"][1][1])]
    rebuilt=0
    for cell,row in zip(protocol["cells"],rows):
        p,q,lam,B=[cell[k] for k in ("p","q","R_over_Q","radius")]
        assert (row["p"],row["q"],row["R_over_Q"],row["radius"])==(p,q,lam,B)
        labels=[(a,b) for a in range(-B,B+1) for b in range(-B,B+1) if (a,b)!=(0,0)]
        labels.sort(key=lambda z:(hqq*z[0]**2+2*hqr*z[0]*z[1]+hrr*z[1]**2,z))
        assert len(labels)==row["candidate_cap"]
        field_curve=[int(z)%p for z in curve]
        G=reduce_point(Q,p)
        Rbar=reduce_point(R,p)
        gfm=finite_multiples(G,B,field_curve,p)
        rfm=finite_multiples(Rbar,B,field_curve,p)
        seen={}
        first=None
        peak=0
        for i,label in enumerate(labels[:row["candidate_evaluations"]],1):
            a,b=label
            P=add(qmult[a],rmult[b],curve)
            assert P is not None
            peak=max(peak,bit_height(P))
            red=reduce_point(P,p)
            assert red==add(gfm[a],rfm[b],field_curve,p)
            if first is None:
                if red is None:
                    first={"kind":"zero_reduction","index":i,"labels":[list(label)],
                           "delta":[a,b],"prefix_max_coordinate_bits":peak}
                elif red in seen:
                    old=seen[red]
                    da,db=a-old[0],b-old[1]
                    assert db%q and (-da*pow(db,-1,q))%q==lam
                    first={"kind":"finite_collision","index":i,
                           "labels":[list(old),list(label)],"delta":[da,db],
                           "prefix_max_coordinate_bits":peak}
            if red is not None:
                seen.setdefault(red,label)
            rebuilt+=1
        assert row["first_relation"]==first
        assert row["nonidentity_coverage"]==len(seen)
        assert row["peak_rational_coordinate_bits"]==peak
        if row["status"]=="full_coverage":
            assert len(seen)==q-1
        elif row["status"]=="coefficient_cap_exhausted":
            assert row["candidate_evaluations"]==len(labels) and len(seen)<q-1
        else:
            assert row["status"]=="wall_cap_exhausted"
        assert row["attempts_started"]>=row["candidate_evaluations"]
    return {"schema":"xedni-threshold-rational-cost-audit-v1","status":"PASS",
            "rows_sha256":sha(raw),"cells":len(rows),
            "independent_fraction_points_rebuilt":rebuilt}


if __name__=="__main__":
    print(json.dumps(main(),sort_keys=True,indent=2))

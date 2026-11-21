#!/usr/bin/env python3
"""Independent Fraction collision prefix and integer finite-field rho replay."""

import hashlib
import importlib.util
import json
from fractions import Fraction as F
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[2]
COEFF_SOURCE=HERE.parent/"xedni_coefficient_choice_20261120"/"audit.py"
RHO_SOURCE=HERE.parent/"xedni_fresh_target_pair_20261120"/"audit.py"


def import_file(name,path):
    spec=importlib.util.spec_from_file_location(name,path)
    module=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


coeff=import_file("xedni_coeff_exact",COEFF_SOURCE)
finite=import_file("xedni_rho_integer",RHO_SOURCE)


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def multiples(P,B,curve):
    out={0:None}
    cur=None
    for k in range(1,B+1):
        cur=coeff.add(cur,P,curve)
        out[k]=cur
        out[-k]=coeff.neg(cur,curve)
    return out


def reduce_point(P,p):
    if P is None or P[0].denominator%p==0:
        return None
    return tuple(v.numerator*pow(v.denominator,-1,p)%p for v in P)


def bits(P):
    return max(max(abs(v.numerator).bit_length(),v.denominator.bit_length()) for v in P)


def main():
    protocol_raw=(HERE/"protocol.json").read_bytes()
    protocol=json.loads(protocol_raw)
    result=json.loads((HERE/"result.json").read_bytes())
    assert sha(protocol_raw)==result["protocol_sha256"]
    rows_raw=(HERE/"pair-rows.jsonl").read_bytes()
    setup_raw=(HERE/"setup-rows.jsonl").read_bytes()
    assert sha(rows_raw)==result["pair_rows_sha256"]
    assert sha(setup_raw)==result["setup_rows_sha256"]
    rows=[json.loads(line) for line in rows_raw.splitlines()]
    setup=[json.loads(line) for line in setup_raw.splitlines()]
    assert len(rows)==20 and len(setup)==2
    source_raw=(ROOT/protocol["source_prime_rows"]).read_bytes()
    assert sha(source_raw)==protocol["source_prime_rows_raw_sha256"]
    source={x["p"]:x for x in map(json.loads,source_raw.splitlines())}
    panel=json.loads((ROOT/protocol["source_panel_protocol"]).read_bytes())
    curve=[F(z) for z in panel["rational_curve_coefficients"]]
    Q=tuple(F(z) for z in panel["Q"])
    R=tuple(F(z) for z in panel["R"])
    hqq,hqr,hrr=[F(z) for z in (panel["height_pairing_matrix"][0][0],
                                panel["height_pairing_matrix"][0][1],
                                panel["height_pairing_matrix"][1][1])]
    checked=[]
    for cell in protocol["cells"]:
        p,q,B=[cell[k] for k in ("p","q","coefficient_radius")]
        fcurve=[int(z)%p for z in curve]
        G=reduce_point(Q,p)
        T=reduce_point(R,p)
        assert G is not None and T is not None
        assert finite.mul(q,G,fcurve,p) is None
        labels=[(a,b) for a in range(-B,B+1) for b in range(-B,B+1) if (a,b)!=(0,0)]
        labels.sort(key=lambda z:(hqq*z[0]**2+2*hqr*z[0]*z[1]+hrr*z[1]**2,z))
        qmult=multiples(Q,B,curve)
        rmult=multiples(R,B,curve)
        gfm={a:finite.mul(a%q,G,fcurve,p) for a in range(-B,B+1)}
        rfm={b:finite.mul(b%q,T,fcurve,p) for b in range(-B,B+1)}
        seen={}
        peak=0
        relation=None
        for index,(a,b) in enumerate(labels,1):
            P=coeff.add(qmult[a],rmult[b],curve)
            assert P is not None
            peak=max(peak,bits(P))
            reduced=reduce_point(P,p)
            assert reduced==finite.add(gfm[a],rfm[b],fcurve,p)
            if reduced is None:
                relation={"kind":"zero_reduction","labels":[[a,b]],"delta":[a,b]}
            elif reduced in seen:
                old=seen[reduced]
                relation={"kind":"finite_collision","labels":[list(old),[a,b]],
                          "delta":[a-old[0],b-old[1]]}
            else:
                seen[reduced]=(a,b)
            if relation is not None:
                break
        da,db=relation["delta"]
        assert db%q
        lam=-da*pow(db,-1,q)%q
        assert finite.mul(lam,G,fcurve,p)==T
        saved=source[p]["first_relation"]
        assert saved=={"index":index,"kind":relation["kind"],"labels":relation["labels"],
                       "delta":relation["delta"],"derived_R_over_G":lam,
                       "prefix_max_coordinate_bits":peak}
        assert index==saved["index"] and lam==cell["expected_lambda_for_postsolve_audit"]
        setup_row=next(x for x in setup if x["p"]==p)
        assert setup_row["public_target"]==list(T) and setup_row["candidate_cap"]==len(labels)
        for repetition in range(1,protocol["repetitions_per_cell"]+1):
            pair=[x for x in rows if x["p"]==p and x["repetition"]==repetition]
            assert len(pair)==2
            pair.sort(key=lambda x:x["position"])
            expected=["collision","rho"] if repetition%2 else ["rho","collision"]
            assert [x["arm"] for x in pair]==expected
            for row in pair:
                assert row["status"]=="verified" and row["scalar"]==lam
                assert row["public_target"]==list(T)
                assert row["stop_ns"]-row["start_ns"]==row["online_ns"]>0
                if row["arm"]=="collision":
                    assert row["candidate_evaluations"]==row["attempts_started"]==index
                    assert row["relation"]==relation
                else:
                    replay=finite.replay_rho(G,T,fcurve,p,q,repetition,protocol["rho_seed_text"])
                    for key,value in replay.items():
                        assert row[key]==value
        checked.append({"p":p,"q":q,"exact_fraction_points_rebuilt":index,
                        "first_relation_coordinate_peak_bits":peak,"derived_lambda":lam,
                        "paired_repetitions_verified":protocol["repetitions_per_cell"]})
    return {"schema":"xedni-relation-acquisition-audit-v1","status":"PASS",
            "pair_rows_sha256":sha(rows_raw),"cells":checked,
            "fraction_group_law_source_sha256":sha(COEFF_SOURCE.read_bytes()),
            "integer_group_law_source_sha256":sha(RHO_SOURCE.read_bytes())}


if __name__=="__main__":
    print(json.dumps(main(),sort_keys=True,indent=2))

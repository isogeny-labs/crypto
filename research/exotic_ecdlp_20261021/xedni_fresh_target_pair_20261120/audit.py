#!/usr/bin/env python3
"""Independent integer group-law audit of fixtures, search order, and rho."""

import hashlib
import json
from decimal import Decimal, getcontext
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]


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
    from fractions import Fraction
    x,y=[Fraction(z) for z in pair]
    return (x.numerator*pow(x.denominator,-1,p)%p,
            y.numerator*pow(y.denominator,-1,p)%p)


def on_curve(P,c,p):
    if P is None:
        return True
    x,y=P
    a1,a2,a3,a4,a6=c
    return (y*y+a1*x*y+a3*y-x*x*x-a2*x*x-a4*x-a6)%p==0


def replay_rho(G,T,c,p,q,j,seed_text):
    updates=0
    degenerate=0
    for restart in range(64):
        seed=int(sha(f"{seed_text}:rho:{p}:{j}:{restart}".encode()),16)
        a,b=seed%q,(seed//q)%q
        X=add(mul(a,G,c,p),mul(b,T,c,p),c,p)
        def step(state):
            nonlocal updates
            P,u,v=state
            bucket=0 if P is None else P[0]%3
            updates+=1
            if bucket==0:
                return add(P,G,c,p),(u+1)%q,v
            if bucket==1:
                return add(P,T,c,p),u,(v+1)%q
            return add(P,P,c,p),(2*u)%q,(2*v)%q
        slow=(X,a,b)
        fast=(X,a,b)
        for iteration in range(1,100001):
            slow=step(slow)
            fast=step(step(fast))
            if slow[0]!=fast[0]:
                continue
            den=(slow[2]-fast[2])%q
            if den==0:
                degenerate+=1
                break
            k=(fast[1]-slow[1])*pow(den,-1,q)%q
            assert mul(k,G,c,p)==T
            return {"scalar":k,"walk_updates":updates,
                    "degenerate_collisions":degenerate,"restarts":restart,
                    "collision_iteration":iteration}
    return {"scalar":None,"walk_updates":updates,
            "degenerate_collisions":degenerate,"restarts":64}


def main():
    protocol_raw=(HERE/"protocol.json").read_bytes()
    protocol=json.loads(protocol_raw)
    result=json.loads((HERE/"result.json").read_bytes())
    assert sha(protocol_raw)==result["protocol_sha256"]
    rows_raw=(HERE/"arm-rows.jsonl").read_bytes()
    fixtures_raw=(HERE/"target-fixtures.jsonl").read_bytes()
    assert sha(rows_raw)==result["arm_rows_sha256"]
    assert sha(fixtures_raw)==result["target_fixtures_sha256"]
    rows=[json.loads(line) for line in rows_raw.splitlines()]
    fixtures=[json.loads(line) for line in fixtures_raw.splitlines()]
    assert len(rows)==result["arms"]==18 and len(fixtures)==result["targets"]==6
    panel=json.loads((ROOT/protocol["source_panel_protocol"]).read_bytes())
    getcontext().prec=50
    hqq,hqr,hrr=[Decimal(z) for z in (panel["height_pairing_matrix"][0][0],
                                        panel["height_pairing_matrix"][0][1],
                                        panel["height_pairing_matrix"][1][1])]
    verified_rho=0
    for cell in protocol["cells"]:
        p,q,lam,B=[cell[key] for key in ("p","q","R_over_Q","radius")]
        curve=[int(z)%p for z in panel["rational_curve_coefficients"]]
        G=reduce_pair(panel["Q"],p)
        Rbar=reduce_pair(panel["R"],p)
        assert on_curve(G,curve,p) and on_curve(Rbar,curve,p)
        assert mul(q,G,curve,p) is None and mul(lam,G,curve,p)==Rbar
        labels=[(a,b) for a in range(-B,B+1) for b in range(-B,B+1) if (a,b)!=(0,0)]
        labels.sort(key=lambda z:(hqq*z[0]**2+2*hqr*z[0]*z[1]+hrr*z[1]**2,z))
        for j in range(protocol["targets_per_cell"]):
            fixture=next(x for x in fixtures if x["p"]==p and x["target_index"]==j)
            digest=sha(f"{protocol['target_seed']}:{p}:{j}".encode())
            k=1+int(digest,16)%(q-1)
            T=mul(k,G,curve,p)
            assert fixture=={"p":p,"q":q,"target_index":j,"target":list(T),
                             "fixture_scalar":k,"target_digest":digest}
            first=next((i,label) for i,label in enumerate(labels,1)
                       if (label[0]+lam*label[1])%q==k)
            group=[r for r in rows if r["p"]==p and r["target_index"]==j]
            assert len(group)==3
            group.sort(key=lambda r:r["position"])
            assert [r["arm"] for r in group]==protocol["arm_order_by_target_index"][j]
            for r in group:
                assert r["target"]==list(T) and r["scalar"]==k and r["status"]=="verified"
                assert r["stop_ns"]-r["start_ns"]==r["online_ns"]>0
                if r["arm"] in ("rational","finite"):
                    assert r["candidate_evaluations"]==r["attempts_started"]==first[0]
                    assert r["matched_label"]==list(first[1])
                else:
                    replay=replay_rho(G,T,curve,p,q,j,protocol["target_seed"])
                    for key,value in replay.items():
                        assert r[key]==value
                    verified_rho+=1
    return {"schema":"xedni-fresh-target-pair-audit-v1","status":"PASS",
            "targets":len(fixtures),"arms":len(rows),"rho_walks_replayed":verified_rho,
            "arm_rows_sha256":sha(rows_raw),"target_fixtures_sha256":sha(fixtures_raw)}


if __name__=="__main__":
    print(json.dumps(main(),sort_keys=True,indent=2))

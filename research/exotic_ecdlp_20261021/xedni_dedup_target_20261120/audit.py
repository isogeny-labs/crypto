#!/usr/bin/env python3
"""Independent exact-score and finite-field replay of deduplicated searches."""

import hashlib
import importlib.util
import json
from fractions import Fraction as F
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[2]
AUDIT_SOURCE=HERE.parent/"xedni_fresh_target_pair_20261120"/"audit.py"
spec=importlib.util.spec_from_file_location("xedni_integer_group_audit",AUDIT_SOURCE)
group=importlib.util.module_from_spec(spec)
spec.loader.exec_module(group)


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def main():
    protocol_raw=(HERE/"protocol.json").read_bytes()
    protocol=json.loads(protocol_raw)
    result=json.loads((HERE/"result.json").read_bytes())
    assert sha(protocol_raw)==result["protocol_sha256"]
    files={}
    for stem,key in (("arm-rows.jsonl","arm_rows_sha256"),
                     ("target-fixtures.jsonl","target_fixtures_sha256"),
                     ("setup-rows.jsonl","setup_rows_sha256")):
        raw=(HERE/stem).read_bytes()
        assert sha(raw)==result[key]
        files[stem]=[json.loads(line) for line in raw.splitlines()]
    rows=files["arm-rows.jsonl"]
    fixtures=files["target-fixtures.jsonl"]
    setup_rows=files["setup-rows.jsonl"]
    assert len(rows)==result["arms"]==18 and len(fixtures)==result["targets"]==6
    panel=json.loads((ROOT/protocol["source_panel_protocol"]).read_bytes())
    hqq,hqr,hrr=[F(z) for z in (panel["height_pairing_matrix"][0][0],
                                panel["height_pairing_matrix"][0][1],
                                panel["height_pairing_matrix"][1][1])]
    previous_raw=(ROOT/protocol["previous_target_fixtures"]).read_bytes()
    assert sha(previous_raw)==protocol["previous_target_fixtures_raw_sha256"]
    previous={}
    for row in map(json.loads,previous_raw.splitlines()):
        previous.setdefault(row["p"],set()).add(row["fixture_scalar"])
    summaries=[]
    for cell,setup in zip(protocol["cells"],setup_rows):
        p,q,lam,B=[cell[k] for k in ("p","q","R_over_Q","radius")]
        curve=[int(z)%p for z in panel["rational_curve_coefficients"]]
        G=group.reduce_pair(panel["Q"],p)
        Rbar=group.reduce_pair(panel["R"],p)
        assert group.mul(q,G,curve,p) is None and group.mul(lam,G,curve,p)==Rbar
        labels=[(a,b) for a in range(-B,B+1) for b in range(-B,B+1) if (a,b)!=(0,0)]
        labels.sort(key=lambda z:(hqq*z[0]**2+2*hqr*z[0]*z[1]+hrr*z[1]**2,z))
        gmult={a:group.mul(a%q,G,curve,p) for a in range(-B,B+1)}
        rmult={b:group.mul(b%q,Rbar,curve,p) for b in range(-B,B+1)}
        first={}
        selected=[]
        for index,(a,b) in enumerate(labels,1):
            point=group.add(gmult[a],rmult[b],curve,p)
            assert point==group.mul((a+lam*b)%q,G,curve,p)
            if point is not None and point not in first:
                first[point]=(index,(a,b))
                selected.append((a,b))
        assert len(first)==len(selected)==q-1
        assert setup["p"]==p and setup["q"]==q and setup["radius"]==B
        assert setup["full_candidate_count"]==len(labels)
        assert setup["unique_nonidentity_count"]==len(selected)
        used=set()
        for j in range(protocol["targets_per_cell"]):
            digest=sha(f"{protocol['target_seed']}:{p}:{j}".encode())
            k=1+int(digest,16)%(q-1)
            assert k not in used and k not in previous[p]
            used.add(k)
            T=group.mul(k,G,curve,p)
            fixture=next(x for x in fixtures if x["p"]==p and x["target_index"]==j)
            assert fixture=={"p":p,"q":q,"target_index":j,"target":list(T),
                             "fixture_scalar":k,"target_digest":digest}
            baseline_index,label=first[T]
            dedup_index=selected.index(label)+1
            these=[x for x in rows if x["p"]==p and x["target_index"]==j]
            assert len(these)==3
            these.sort(key=lambda x:x["position"])
            assert [x["arm"] for x in these]==protocol["arm_order_by_target_index"][j]
            for row in these:
                assert row["status"]=="verified" and row["scalar"]==k and row["target"]==list(T)
                assert row["stop_ns"]-row["start_ns"]==row["online_ns"]>0
                if row["arm"] in ("baseline","dedup"):
                    expected=baseline_index if row["arm"]=="baseline" else dedup_index
                    assert row["candidate_evaluations"]==row["attempts_started"]==expected
                    assert row["matched_label"]==list(label)
                else:
                    replay=group.replay_rho(G,T,curve,p,q,j,protocol["target_seed"])
                    for key,value in replay.items():
                        assert row[key]==value
        summaries.append({"p":p,"q":q,"full_labels_replayed":len(labels),
                          "unique_nonidentity_labels":len(selected),
                          "fresh_targets_verified":len(used)})
    return {"schema":"xedni-dedup-target-audit-v1","status":"PASS",
            "group_law_source_sha256":sha(AUDIT_SOURCE.read_bytes()),
            "arm_rows_sha256":result["arm_rows_sha256"],"cells":summaries,
            "verified_arms":len(rows)}


if __name__=="__main__":
    print(json.dumps(main(),sort_keys=True,indent=2))

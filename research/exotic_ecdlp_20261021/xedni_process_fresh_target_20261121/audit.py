"""Independent exact-group and process-record audit."""

import hashlib
import json
from pathlib import Path

HERE=Path(__file__).resolve().parent
SOURCE=HERE.parent/"xedni_fixed_curve_target_accounting_20261121"
O=None


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def add(A,B):
    if A is O:return B
    if B is O:return A
    x1,y1=A;x2,y2=B
    if x1==x2 and (y1+y2)%47==0:return O
    slope=((3*x1*x1-1)*pow(2*y1,-1,47) if A==B else
           (y2-y1)*pow((x2-x1)%47,-1,47))%47
    x3=(slope*slope-x1-x2)%47
    return (x3,(slope*(x1-x3)-y1)%47)


def mul(n,A):
    R=O
    while n:
        if n&1:R=add(R,A)
        A=add(A,A);n//=2
    return R


def main():
    p=json.loads((HERE/"protocol.json").read_text())
    assert sha(SOURCE/"run_sage.py")==p["source_table_runner_sha256"]
    assert sha(SOURCE/"audit-result.json")==p["source_audit_sha256"]
    result=json.loads((HERE/"result.json").read_text())
    assert sha(HERE/"protocol.json")==result["protocol_sha256"]
    assert sha(HERE/"process-rows.jsonl")==result["rows_sha256"]
    rows=[json.loads(s) for s in (HERE/"process-rows.jsonl").read_text().splitlines()]
    assert [r["repetition"] for r in rows]==[1,2,3]
    k=1+int.from_bytes(hashlib.sha256(p["target_seed"].encode()).digest()[:8],"big")%42
    G=(17,10);target=mul(k,G)
    assert mul(43,G) is O and len({mul(n,G) for n in range(1,43)})==42
    assert result["target"]==list(target) and result["fixture_scalar_for_replay"]==k
    for rep,row in enumerate(rows,1):
        assert row["status"]=="complete" and row["exit_code"]==0
        assert row["launcher_wall_ns"]>0
        worker=row["worker"]
        assert worker["status"]=="PASS" and worker["repetition"]==rep
        assert worker["target"]==list(target) and worker["fixture_scalar_for_replay"]==k
        assert [a["arm"] for a in worker["arms"]]==p["arm_order"][rep-1].split(",")
        assert [a["position"] for a in worker["arms"]]==[1,2]
        setup=worker["setup"]
        assert setup["setup_ns"]==sum(setup["phases"].values())
        assert setup["relation_rows"]==28 and setup["folded_columns"]==21
        assert setup["relation_rank"]==20 and setup["anchored_rank"]==21
        assert setup["max_coordinate_bits"]==7241
        assert row["launcher_wall_ns"]>setup["setup_ns"]
        for arm in worker["arms"]:
            assert arm["status"]=="verified" and arm["scalar"]==k
            assert mul(arm["scalar"],G)==target
            assert arm["online_ns"]==arm["stop_ns"]-arm["start_ns"]>0
            if arm["arm"]=="table":
                assert arm["online_ns"]==arm["target_query_ns"]+arm["target_recovery_check_ns"]
            else:
                assert arm["walk_updates"]>0
        assert worker["arms"][0]["start_ns"]<worker["arms"][1]["start_ns"]
    assert result["completed_processes"]==result["processes"]==len(rows)==3
    print(json.dumps({"schema":"xedni-process-fresh-target-audit-v1","status":"PASS",
                      "processes":3,"verified_arms":6,"target":target,"scalar":k},indent=2,sort_keys=True))


if __name__=="__main__":main()

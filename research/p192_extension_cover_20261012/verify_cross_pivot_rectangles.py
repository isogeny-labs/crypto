"""Independent Sage replay of cut identities, matrix ranks, and queries."""

import hashlib
import json
from collections import defaultdict
from itertools import combinations
from pathlib import Path

from sage.all import EllipticCurve, GF, Integer, matrix

HERE = Path(__file__).resolve().parent
sha = lambda path: hashlib.sha256(Path(path).read_bytes()).hexdigest()
path = HERE / "type_i_cross_pivot_rectangles.json"
source = json.loads(path.read_text())
assert source["source_sha256"] == sha(HERE / "line_cross_pivot_rectangles.py")
for name, digest in source["input_sha256"].items():
    assert sha(HERE/name) == digest
p,r = Integer(source["field_prime"]),Integer(source["subgroup_order"])
F = GF(p)
E = EllipticCurve(F,[F(-3),F(Integer(
    "64210519E59C80E70FA7E9AB72243049FEB8DEECC146B9B1",16))])
O=E(0)


def point(coordinates):
    if coordinates is None:return O
    x,y=map(Integer,coordinates)
    assert (y*y-x*x*x+3*x-Integer(E.a6()))%p==0
    return E.point([F(x),F(y),F(1)],check=False)


columns=[point(Q) for Q in source["quotient_columns"]]
assert len(columns)==len(set(columns))==96
assert all(r*Q==O for Q in columns)
edges=source["edges"]
assert len(edges)==48
by_key={(e["pivot"],e["endpoint"]):e for e in edges}
assert len(by_key)==48
constant=point(source["line_section_constant"])
guide=json.loads((HERE/"type_i_line_guided_collector.json").read_text())
new=json.loads((HERE/"type_i_line_full_new_quotient_points.json").read_text())
selected=[point(Q) for Q in guide["base_points_fp"]+new]
fans=[json.loads((HERE/name).read_text()) for name in (
    "type_i_residual_pivot000_three.json",
    "type_i_residual_pivot001_cross.json",
    "type_i_residual_pivot002_cross.json")]
for e in edges:
    Q=columns[e["column_indices"][0]]+columns[e["column_indices"][1]]
    pivot=point(fans[e["pivot"]]["pivot_quotient_point"])
    assert Q+pivot+selected[e["endpoint"]]==constant

G=point([Integer("188DA80EB03090F67CBF20EB43A18800F4FF0AFD82FF1012",16),
         Integer("07192B95FFC8DA78631011ED6B24CDD573F977A11E794811",16)])
assert r*G==O
summary={}
for case in source["cases"]:
    col_indices=case["column_indices_global"]
    mapping={v:i for i,v in enumerate(col_indices)}
    selected_edges=[e for e in edges if e["column_indices"][0] in mapping]
    incidence={}
    for e in selected_edges:
        incidence[(e["pivot"],e["endpoint"])]=e
    assert len(selected_edges)==case["edges"]
    rows=[]
    targets=[]
    for u,v,j,k in case["rectangle_witnesses"]:
        row=[0]*len(col_indices)
        positive=[]
        negative=[]
        for pivot,endpoint,sign in ((u,j,1),(v,k,1),(u,k,-1),(v,j,-1)):
            edge=incidence[pivot,endpoint]
            for original in edge["column_indices"]:
                row[mapping[original]]+=sign
                (positive if sign==1 else negative).append(columns[original])
        target=sum(positive,O)
        assert target==sum(negative,O)
        targets.append(target)
        rows.append(row)
    assert rows==case["rectangle_rows"]
    assert len(rows)==case["relation_rows"]
    assert int(matrix(GF(r),rows).rank())==case["matrix_rank"]
    assert len(set(targets))==case["distinct_rectangle_targets"]
    assert [list(map(int,Q.xy())) if Q else None for Q in targets]== \
        case["rectangle_target_keys"]
    degrees=[sum(row[i]!=0 for row in rows) for i in range(len(col_indices))]
    assert sum(d>0 for d in degrees)==case["active_columns"]
    target_map=defaultdict(list)
    for index,target in enumerate(targets):target_map[target].append(index)
    for name,law in case["known_scalar_query_laws"].items():
        scalars=guide["uniform_scalars" if name=="uniform" else "guided_scalars"]
        hits=[]
        for i,encoded in enumerate(scalars):
            a=Integer(encoded)
            Q=-2*a*G
            if Q in target_map:hits.append([i,str(a),target_map[Q]])
        assert law["queries"]==len(scalars)==64
        assert hits==law["hits"]
    summary[case["name"]]={"rank":case["matrix_rank"],
                            "rows":len(rows),
                            "core_rows":case["two_core_rows"]}

receipt={
    "status":"independent_cross_pivot_rectangles_replay_pass",
    "producer_sha256":sha(path),
    "source_sha256":sha(__file__),
    "runtime_info_sha256":sha(HERE/"type_i_cross_pivot_rectangles_replay_runtime_info.json"),
    "cases":summary,
}
output=HERE/"type_i_cross_pivot_rectangles_replay.json"
output.write_text(json.dumps(receipt,indent=2,sort_keys=True)+"\n")
print("CROSS_PIVOT_RECTANGLES_REPLAY_PASS",json.dumps(summary,sort_keys=True),flush=True)

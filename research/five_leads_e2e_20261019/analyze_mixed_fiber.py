#!/usr/bin/env python3
"""Summarize complete small-cell oracle observations; no timing claim."""

import json
from collections import defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent


def cost(row, s, i, native):
    c = row["solver_total_ops"]
    arithmetic = c["m"] + s*c["s"] + i*c["i"]
    logical = row["random_draws"] + row["pair_insertions"] + row["third_point_lookups"] + row["verified_lifts"]
    return arithmetic + native*logical


def main():
    rows = [json.loads(line) for line in (HERE/"correctness/mixed-fiber-census-v1.jsonl").open()]
    assert len(rows) == 540
    cells = defaultdict(dict)
    for r in rows:
        assert r["all_solutions_match_exhaustive"]
        assert len(r["fibers"]) == 3 and len(r["point_counts"]) == 3
        key = (r["p"],r["a"],r["b"],r["d"],r["target_index"])
        assert r["arm"] not in cells[key]
        cells[key][r["arm"]] = r
    assert len(cells) == 180
    for cell in cells.values():
        assert sorted(cell) == ["homogeneous","mixed","random"]
        assert cell["homogeneous"]["point_counts"] == cell["mixed"]["point_counts"] == cell["random"]["point_counts"]

    summary=[]
    for p,d in ((97,8),(193,12),(257,16)):
        subset=[cell for key,cell in cells.items() if key[0]==p]
        assert len(subset)==60
        grid=[]
        for s in (0,0.5,1):
            for i in (1,10,100,1000):
                for native in (0,0.1,1):
                    totals={arm:sum(cost(cell[arm],s,i,native) for cell in subset) for arm in ("homogeneous","mixed","random")}
                    grid.append({"S_per_M":s,"I_per_M":i,"native_M_per_logical_op":native,
                                 "mixed_over_homogeneous":totals["mixed"]/totals["homogeneous"],
                                 "mixed_over_random":totals["mixed"]/totals["random"]})
        summary.append({"p":p,"fiber_cardinality":d,"curves":3,"targets":60,
                        "complete_solution_counts":{arm:sum(len(cell[arm]["complete_solutions"]) for cell in subset) for arm in ("homogeneous","mixed","random")},
                        "nonempty_targets":{arm:sum(bool(cell[arm]["complete_solutions"]) for cell in subset) for arm in ("homogeneous","mixed","random")},
                        "mixed_over_homogeneous_arithmetic_range":[min(x["mixed_over_homogeneous"] for x in grid if x["native_M_per_logical_op"]==0),max(x["mixed_over_homogeneous"] for x in grid if x["native_M_per_logical_op"]==0)],
                        "mixed_over_homogeneous_native_band":[min(x["mixed_over_homogeneous"] for x in grid),max(x["mixed_over_homogeneous"] for x in grid)],
                        "mixed_over_random_arithmetic_range":[min(x["mixed_over_random"] for x in grid if x["native_M_per_logical_op"]==0),max(x["mixed_over_random"] for x in grid if x["native_M_per_logical_op"]==0)],
                        "mixed_over_random_native_band":[min(x["mixed_over_random"] for x in grid),max(x["mixed_over_random"] for x in grid)],
                        "default_M_plus_0_75S_plus_10I":{
                            "mixed_over_homogeneous":sum(cost(cell["mixed"],0.75,10,0) for cell in subset)/sum(cost(cell["homogeneous"],0.75,10,0) for cell in subset),
                            "mixed_over_random":sum(cost(cell["mixed"],0.75,10,0) for cell in subset)/sum(cost(cell["random"],0.75,10,0) for cell in subset)},
                        "all_complete_solution_sets_verified":True})
    (HERE/"results").mkdir(exist_ok=True)
    (HERE/"results/mixed-fiber-summary.json").write_text(json.dumps({"schema":"mixed-fiber-summary/v1","rows":len(rows),"summary":summary},indent=2)+"\n")
    print(json.dumps(summary,indent=2))


if __name__=="__main__": main()

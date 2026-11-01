#!/usr/bin/env python3
"""Validate the full exact-order cohort and price paired cold solves."""

import csv
import json
from collections import defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent
ARMS = ("rho_plain", "rho_neg", "jac_rho", "genus_ic", "genus_choice2")


def price(record, s, i, native):
    c = record["counters"]
    arithmetic = c["field_multiplications"] + s * c["field_squarings"] + i * c["field_inversions"]
    source_add = c["source_add_multiplications"] + s * c["source_add_squarings"] + i * c["source_add_inversions"]
    logical = sum(value for name, value in c.items() if name.endswith("_uncharged"))
    return arithmetic / source_add + native * logical


def main():
    groups = defaultdict(list)
    records = []
    sessions = []
    for bits in (16, 20, 24, 28, 32, 36, 40):
        folder = HERE / f"sessions/order-genus-{bits}"
        session = json.loads((folder / "session.json").read_text())
        assert session["status"] == "complete" and session["records_written"] == 900
        sessions.append({"bits": bits, "id": session["session_id"], "records_sha256": session["records_sha256"]})
        for line in (folder / "records.jsonl").open():
            r = json.loads(line)
            assert r["outcome"]["status"] == "verified"
            assert r["outcome"]["matches_target"] and r["outcome"]["matches_planted"]
            assert r["arm"] in ARMS and not r["warmup"]
            records.append((bits, r))
            groups[(bits, r["workload"]["curve"]["slug"], r["workload"]["target_index"])].append(r)
    assert len(records) == 6300 and len(groups) == 1260
    curves = defaultdict(set)
    for bits, slug, target_index in groups:
        curves[(bits, slug)].add(target_index)
    assert len(curves) == 21
    assert all(len(indices) == 60 and indices == set(range(60)) for indices in curves.values())
    assert all(sum(key[0] == bits for key in curves) == 3 for bits in (16, 20, 24, 28, 32, 36, 40))
    for key, rows in groups.items():
        assert sorted(r["arm"] for r in rows) == sorted(ARMS), key
        assert len({r["workload"]["workload_id"] for r in rows}) == 1, key
        assert len({r["workload"]["target"][0] + ":" + r["workload"]["target"][1] for r in rows}) == 1, key

    out = []
    grid = []
    groupings = [("all", "all", list(groups.values()))]
    for bits in (16, 20, 24, 28, 32, 36, 40):
        groupings.append(("bits", str(bits), [rows for key, rows in groups.items() if key[0] == bits]))
    for bits, slug in sorted({(key[0], key[1]) for key in groups}):
        groupings.append(("curve", f"{bits}:{slug}", [rows for key, rows in groups.items() if key[:2] == (bits, slug)]))
    for kind, label, workloads in groupings:
        for s in (0, 0.5, 1):
            for i in (1, 10, 100, 1000):
                for native in (0, 0.1, 1):
                    totals = {arm: 0.0 for arm in ARMS}
                    for rows in workloads:
                        for r in rows:
                            totals[r["arm"]] += price(r, s, i, native)
                    row = {"kind":kind,"label":label,"targets":len(workloads),
                           "S_per_M":s,"I_per_M":i,"native_GAE_per_logical_operation":native,
                           **{f"{arm}_GAE":totals[arm] for arm in ARMS},
                           "choice_over_incumbent":totals["genus_choice2"]/totals["genus_ic"],
                           "choice_over_rho_neg":totals["genus_choice2"]/totals["rho_neg"]}
                    grid.append(row)
        subset = [r for rows in workloads for r in rows]
        by_arm = {arm:[r for r in subset if r["arm"] == arm] for arm in ARMS}
        choices = by_arm["genus_choice2"]
        incumbent = by_arm["genus_ic"]
        prices = [row for row in grid if row["kind"] == kind and row["label"] == label]
        out.append({"kind":kind,"label":label,"targets":len(workloads),
                    "all_verified":True,
                    "choice_over_incumbent_arithmetic_range":[min(x["choice_over_incumbent"] for x in prices if x["native_GAE_per_logical_operation"]==0),max(x["choice_over_incumbent"] for x in prices if x["native_GAE_per_logical_operation"]==0)],
                    "choice_over_incumbent_native_band":[min(x["choice_over_incumbent"] for x in prices),max(x["choice_over_incumbent"] for x in prices)],
                    "choice_over_rho_neg_arithmetic_range":[min(x["choice_over_rho_neg"] for x in prices if x["native_GAE_per_logical_operation"]==0),max(x["choice_over_rho_neg"] for x in prices if x["native_GAE_per_logical_operation"]==0)],
                    "choice_over_rho_neg_native_band":[min(x["choice_over_rho_neg"] for x in prices),max(x["choice_over_rho_neg"] for x in prices)],
                    "choice_processed_divisors":sum(r["counters"].get("processed_divisors",0) for r in choices),
                    "incumbent_processed_divisors":sum(r["counters"].get("processed_divisors",0) for r in incumbent),
                    "choice_split_relations":sum(r["counters"].get("split_relations",0) for r in choices),
                    "incumbent_split_relations":sum(r["counters"].get("split_relations",0) for r in incumbent),
                    "choice_graph_cycles":sum(r["counters"].get("graph_cycles",0) for r in choices),
                    "incumbent_graph_cycles":sum(r["counters"].get("graph_cycles",0) for r in incumbent),
                    "choice_component_policy_probes":sum(r["counters"].get("component_policy_probes_uncharged",0) for r in choices),
                    "choice_peak_logical_graph_bytes":max(r["counters"].get("logical_graph_bytes",0) for r in choices),
                    "incumbent_peak_logical_graph_bytes":max(r["counters"].get("logical_graph_bytes",0) for r in incumbent),
                    "choice_peak_logical_walk_seen_bytes":max(r["counters"].get("logical_walk_seen_bytes",0) for r in choices),
                    "incumbent_peak_logical_walk_seen_bytes":max(r["counters"].get("logical_walk_seen_bytes",0) for r in incumbent),
                    "choice_peak_large_prime_nodes":max(r["counters"].get("large_prime_nodes",0) for r in choices),
                    "incumbent_peak_large_prime_nodes":max(r["counters"].get("large_prime_nodes",0) for r in incumbent),
                    "choice_solved_without_graph_cycle":sum(r["counters"].get("graph_cycles",0)==0 for r in choices),
                    "incumbent_solved_without_graph_cycle":sum(r["counters"].get("graph_cycles",0)==0 for r in incumbent),
                    "choice_solved_at_first_graph_cycle":sum(r["counters"].get("first_graph_cycle_trial",0)>0 and r["counters"]["first_graph_cycle_trial"]==r["counters"]["processed_divisors"] for r in choices),
                    "incumbent_solved_at_first_graph_cycle":sum(r["counters"].get("first_graph_cycle_trial",0)>0 and r["counters"]["first_graph_cycle_trial"]==r["counters"]["processed_divisors"] for r in incumbent)})
    result = {"schema":"five-leads-choice2-analysis/v1","sessions":sessions,"records":len(records),"paired_targets":len(groups),"summary":out}
    (HERE / "results").mkdir(exist_ok=True)
    (HERE / "results/genus2-choice2-summary.json").write_text(json.dumps(result,indent=2)+"\n")
    with (HERE / "results/genus2-choice2-price-grid.csv").open("w",newline="") as f:
        writer = csv.DictWriter(f, fieldnames=grid[0].keys(), lineterminator="\n")
        writer.writeheader(); writer.writerows(grid)
    print(json.dumps(out[:8],indent=2))


if __name__ == "__main__":
    main()

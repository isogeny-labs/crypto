#!/usr/bin/env python3
"""Ablate exact peeled-target caching by identity, cubic, twist and full action."""
from __future__ import annotations

import hashlib
import json
from pathlib import Path

from analyze_k1_peeled_orbits import ROOT, STREAM, action, canonical


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    prior_path = ROOT / "K1_PEELED_ORBIT_REUSE_v1.json"
    prior = json.loads(prior_path.read_text())
    assert prior["status"] == "pass" and len(prior["runs"]) == 6
    prior_by_key = {(r["p"], r["seed"]): r for r in prior["runs"]}
    streams = {(rec["p"], rec["seed"]): rec
               for line in STREAM.read_text().splitlines()
               if (rec := json.loads(line))["p"] in (73, 97)
               and rec["line_powers"] == [0, 2]}
    assert len(streams) == 6
    rows = []
    for p in (73, 97):
        for seed in range(3):
            stream = streams[p, seed]
            raw_path = ROOT / f"quartic_twist_k1_scale_p{p}_seed{seed}_v1.json"
            raw = json.loads(raw_path.read_text())
            n = raw["n"]
            assert raw["factor_logs_verified"] is True
            signed_l2 = {sign * value % n
                         for point, value in zip(raw["base_coordinates"],
                                                 raw["physical_factor_logs"])
                         if point[0][2] != 0 for sign in (-1, 1)}
            assert len(signed_l2) == (96 if p == 73 else 72)
            groups = {
                "identity": (1,),
                "cubic": tuple(sorted({pow(raw["lambda_cubic"], i, n) for i in range(3)})),
                "frob": tuple(sorted({pow(raw["lambda_frob"], i, n) for i in range(8)})),
                "full24": action(n, raw["lambda_frob"], raw["lambda_cubic"]),
            }
            assert {key: len(value) for key, value in groups.items()} == {
                "identity": 1, "cubic": 3, "frob": 8, "full24": 24}
            assert set(groups["cubic"]) <= set(groups["full24"])
            assert set(groups["frob"]) <= set(groups["full24"])
            assert all((v * g) % n in signed_l2
                       for values in groups.values() for v in signed_l2 for g in values)
            peels = []
            for a, b, *_ in stream["query_trace"]:
                residual = (a + 271 * b) % n
                assert residual != 0
                values = [(residual - v) % n for v in signed_l2]
                assert 0 not in values
                peels.append(values)
            assert sum(map(len, peels)) == prior_by_key[p, seed]["finite_peels"]
            by_group = {}
            for name, group in groups.items():
                global_seen: set[int] = set()
                within = 0
                for query in peels:
                    local = {canonical(value, group, n) for value in query}
                    within += len(local)
                    global_seen.update(local)
                attempted = sum(map(len, peels))
                result = {"p": p, "seed": seed, "group": name, "action_order": len(group),
                          "residuals": len(peels), "finite_peels": attempted,
                          "local_duplicates": attempted - within,
                          "cross_query_duplicates_after_local": within - len(global_seen),
                          "unique_across_stream": len(global_seen),
                          "ideal_stream_cache_ratio": attempted / len(global_seen),
                          "producer_sha256": sha(raw_path)}
                by_group[name] = result
                rows.append(result)
            assert by_group["identity"]["unique_across_stream"] >= by_group["cubic"]["unique_across_stream"]
            assert by_group["identity"]["unique_across_stream"] >= by_group["frob"]["unique_across_stream"]
            assert by_group["full24"]["unique_across_stream"] <= by_group["cubic"]["unique_across_stream"]
            assert by_group["full24"]["unique_across_stream"] <= by_group["frob"]["unique_across_stream"]
            assert by_group["full24"]["local_duplicates"] == prior_by_key[p, seed]["intra_query_duplicates"]
            assert (by_group["full24"]["cross_query_duplicates_after_local"] ==
                    prior_by_key[p, seed]["cross_query_duplicates_after_local_dedup"])
    totals = {name: {"finite_peels": sum(r["finite_peels"] for r in rows if r["group"] == name),
                     "local_duplicates": sum(r["local_duplicates"] for r in rows if r["group"] == name),
                     "cross_query_duplicates": sum(r["cross_query_duplicates_after_local"]
                                                   for r in rows if r["group"] == name),
                     "unique_within_streams": sum(r["unique_across_stream"]
                                                  for r in rows if r["group"] == name)}
              for name in ("identity", "cubic", "frob", "full24")}
    for value in totals.values():
        value["ideal_stream_cache_ratio"] = value["finite_peels"] / value["unique_within_streams"]
    assert totals["full24"]["local_duplicates"] == prior["total_intra_query_duplicates"]
    assert totals["full24"]["cross_query_duplicates"] == prior["total_cross_query_duplicates"]
    out = {"schema": "quartic-twist-k1-peeled-subgroups/v1", "status": "pass",
           "protocol_commit": "433df0f52d",
           "prior_full24_sha256": sha(prior_path), "stream_sha256": sha(STREAM),
           "rows": rows, "totals": totals,
           "controls": {"expected_orders": True, "subgroups_contained": True,
                        "factor_sets_closed": True, "monotone_unique_counts": True,
                        "full24_reproduces_prior": True}}
    (ROOT / "K1_PEELED_SUBGROUP_REUSE_v1.json").write_text(json.dumps(out, indent=2) + "\n")
    print(json.dumps(totals, sort_keys=True))


if __name__ == "__main__":
    main()

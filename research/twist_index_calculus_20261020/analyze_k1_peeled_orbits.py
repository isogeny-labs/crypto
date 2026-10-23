#!/usr/bin/env python3
"""Exact scalar diagnostic for S4 peeled-target orbit reuse on frozen streams."""
from __future__ import annotations

import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent
STREAM = ROOT / "quartic_twist_mixed_v1.ndjson"


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def action(n: int, frob: int, cubic: int) -> tuple[int, ...]:
    values = tuple(sorted({pow(frob, i, n) * pow(cubic, j, n) % n
                           for i in range(8) for j in range(3)}))
    assert len(values) == 24 and 1 in values and n - 1 in values
    assert all((a * b) % n in values for a in values for b in values)
    return values


def canonical(value: int, group: tuple[int, ...], n: int) -> int:
    return min(value * g % n for g in group)


def main() -> None:
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
            audit_path = ROOT / f"quartic_twist_k1_scale_p{p}_seed{seed}_audit_v1.json"
            raw, audit = json.loads(raw_path.read_text()), json.loads(audit_path.read_text())
            assert raw["p"] == stream["p"] == audit["p"] == p
            assert raw["seed"] == stream["seed"] == audit["seed"] == seed
            assert raw["n"] == stream["n"]
            assert raw["factor_logs_verified"] is True
            assert audit["base_logs_verified"] == raw["base_size"]
            assert audit["recovered"] == raw["recovered"] == 271
            n = raw["n"]
            group = action(n, raw["lambda_frob"], raw["lambda_cubic"])
            assert (raw["lambda_frob"], raw["lambda_cubic"]) == (
                stream["lambda_frob"], stream["lambda_cubic"])
            l2 = [log for point, log in zip(raw["base_coordinates"], raw["physical_factor_logs"])
                  if point[0][2] != 0]
            assert len(l2) == (48 if p == 73 else 36)
            signed_l2 = {sign * value % n for value in l2 for sign in (-1, 1)}
            assert len(signed_l2) == 2 * len(l2)
            assert all((v * g) % n in signed_l2 for v in signed_l2 for g in group)
            bad_frob = (raw["lambda_frob"] + 1) % n
            assert any((v * bad_frob) % n not in signed_l2 for v in signed_l2)

            global_seen: set[int] = set()
            query_rows = []
            for index, (a, b, *_rest) in enumerate(stream["query_trace"]):
                residual = (a + 271 * b) % n
                assert residual != 0
                targets = [(residual - peel) % n for peel in signed_l2]
                infinity = sum(value == 0 for value in targets)
                canon = [canonical(value, group, n) for value in targets if value]
                assert all(canonical((value * g) % n, group, n) == canonical(value, group, n)
                           for value in targets if value for g in group)
                distinct = set(canon)
                global_seen.update(distinct)
                query_rows.append({"query_index": index, "finite_peels": len(canon),
                                   "infinity_peels": infinity,
                                   "distinct_within_query": len(distinct)})
            attempts = sum(q["finite_peels"] for q in query_rows)
            within = sum(q["distinct_within_query"] for q in query_rows)
            rows.append({"p": p, "seed": seed, "n": n, "residuals": len(query_rows),
                         "zero_residuals": 0,
                         "signed_l2_factors": len(signed_l2), "finite_peels": attempts,
                         "infinity_peels": sum(q["infinity_peels"] for q in query_rows),
                         "distinct_within_queries": within,
                         "intra_query_duplicates": attempts - within,
                         "distinct_across_stream": len(global_seen),
                         "cross_query_duplicates_after_local_dedup": within - len(global_seen),
                         "max_exact_orbit_cache_ratio_within": attempts / within,
                         "max_exact_orbit_cache_ratio_across": attempts / len(global_seen),
                         "queries": query_rows, "producer_sha256": sha(raw_path),
                         "independent_audit_sha256": sha(audit_path)})
    out = {"schema": "quartic-twist-k1-peeled-orbit-reuse/v1", "status": "pass",
           "protocol_commit": "8801c22aaa",
           "stream_sha256": sha(STREAM), "runs": rows,
           "total_finite_peels": sum(r["finite_peels"] for r in rows),
           "total_intra_query_duplicates": sum(r["intra_query_duplicates"] for r in rows),
           "total_cross_query_duplicates": sum(r["cross_query_duplicates_after_local_dedup"]
                                               for r in rows),
           "controls": {"action_order": 24, "factor_orbits_closed": True,
                        "all_target_images_canonical_equal": True,
                        "perturbed_eigenvalue_rejected": True}}
    (ROOT / "K1_PEELED_ORBIT_REUSE_v1.json").write_text(json.dumps(out, indent=2) + "\n")
    print(json.dumps({"runs": len(rows), "total_finite_peels": out["total_finite_peels"],
                      "total_intra_query_duplicates": out["total_intra_query_duplicates"],
                      "total_cross_query_duplicates": out["total_cross_query_duplicates"]}))


if __name__ == "__main__":
    main()

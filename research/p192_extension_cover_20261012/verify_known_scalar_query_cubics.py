"""Independent subgroup, overlap, and provenance replay for query cubics."""

import hashlib
import json
from collections import Counter, defaultdict
from pathlib import Path

from verify_adaptive_private_columns import GX, GY, add, mul, neg, on_curve

HERE = Path(__file__).resolve().parent
sha = lambda path: hashlib.sha256(Path(path).read_bytes()).hexdigest()
source_path = HERE / "type_i_known_scalar_query_cubic_roots.json"
prepared_path = HERE / "type_i_known_scalar_query_cubic_roots_input.json"
gp_input_path = HERE / "type_i_known_scalar_query_cubic_roots_input.gp"
gp_log_path = HERE / "type_i_known_scalar_query_cubic_roots_native.log"
guide_path = HERE / "type_i_line_guided_collector.json"
source, prepared, guide = (json.loads(path.read_text()) for path in
                           (source_path, prepared_path, guide_path))
assert source["source_sha256"] == sha(HERE / "line_known_scalar_query_cubics.py")
assert source["runtime_sha256"] == sha(HERE / "type_i_known_scalar_query_cubic_runtime_info.json")
assert source["input_sha256"][guide_path.name] == sha(guide_path)
assert prepared["source_sha256"] == sha(HERE / "prepare_known_scalar_query_cubics_gp.py")
assert prepared["census_sha256"] == sha(source_path)
assert prepared["gp_input_sha256"] == sha(gp_input_path)
assert gp_log_path.read_text().strip() == (
    "QUERY_CUBIC_NATIVE_PASS cases=256 roots=272 counts=[90, 113, 0, 53]")

p, r = int(source["field_prime"]), int(source["subgroup_order"])
assert p == int(guide["field_prime"]) and r == int(guide["subgroup_order"])
assert on_curve((GX, GY), p) and mul(r, (GX, GY), p) is None
assert source["queries"] == source["verified_pairs"] == 128
assert source["failures"] == []

counts = Counter()
points = defaultdict(list)
scalar_to_laws = defaultdict(set)
for row in source["rows"]:
    law, index, scalar = row["law"], row["query_index"], int(row["scalar"])
    assert row["status"] == "verified_query_cubic_pair"
    assert law in ("uniform", "character_guided") and 0 <= index < 64
    scalar_key = "guided_scalars" if law == "character_guided" else "uniform_scalars"
    assert row["scalar"] == guide[scalar_key][index]
    Q = mul((-2 * scalar) % r, (GX, GY), p)
    H = mul(pow(2, -1, r), Q, p)
    assert Q is not None and H is not None and add(H, H, p) == Q
    assert list(map(int, row["quotient"])) == list(Q)
    assert list(map(int, row["half_quotient"])) == list(H)
    scalar_to_laws[scalar].add(law)
    for role in ("positive", "half"):
        roots = row[role]["rational_points"]
        n = row[role]["rational_point_count"]
        assert n == len(roots) and n in (0, 1, 3)
        counts[f"{law}_{role}_{n}"] += 1
        for point in roots:
            points[tuple(map(int, point))].append((scalar, law, index, role))

assert dict(sorted(counts.items())) == source["rational_root_counts"]
assert len(points) == source["distinct_rational_points"] == 247
assert sum(map(len, points.values())) == 272
assert source["selected_geometry_hits"] == []
assert len(scalar_to_laws) == 117
assert sum(len(laws) == 2 for laws in scalar_to_laws.values()) == 11
repeats = [sources for sources in points.values() if len(sources) > 1]
assert len(repeats) == len(source["cross_cubic_repeats"]) == 25
assert all(len(sources) == 2 and len({source[0] for source in sources}) == 1
           and len({source[3] for source in sources}) == 1
           for sources in repeats)

out = {
    "status": "independent_known_scalar_query_cubic_replay_pass",
    "queries": len(source["rows"]), "unique_scalars": len(scalar_to_laws),
    "shared_scalars_between_laws": 11,
    "cubic_cases": 256, "rational_point_appearances": 272,
    "distinct_rational_points": len(points),
    "repeated_points_same_scalar_only": len(repeats),
    "repeated_points_distinct_scalars": 0,
    "selected_geometry_hits": 0,
    "rational_root_counts": dict(sorted(counts.items())),
    "source_sha256": sha(__file__), "census_sha256": sha(source_path),
    "gp_input_sha256": sha(gp_input_path), "gp_log_sha256": sha(gp_log_path),
}
path = HERE / "type_i_known_scalar_query_cubic_replay.json"
path.write_text(json.dumps(out, indent=2, sort_keys=True) + "\n")
print("QUERY_CUBIC_REPLAY_PASS", json.dumps({
    "unique_scalars": out["unique_scalars"],
    "distinct_rational_points": len(points),
    "distinct_scalar_repeats": 0,
}, sort_keys=True), flush=True)

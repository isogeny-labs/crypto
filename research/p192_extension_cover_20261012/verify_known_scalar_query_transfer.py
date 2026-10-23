"""Independent integer-group replay of the query-cubic affine transfer."""

import hashlib
import json
from pathlib import Path

from verify_adaptive_private_columns import GX, GY, add, mul, neg, on_curve

HERE = Path(__file__).resolve().parent
sha = lambda path: hashlib.sha256(Path(path).read_bytes()).hexdigest()
source_path = HERE / "type_i_known_scalar_query_transfer.json"
roots_path = HERE / "type_i_known_scalar_query_cubic_roots.json"
base_path = HERE / "type_i_quotient_base_probe.json"
source, roots, base = (json.loads(path.read_text()) for path in
                       (source_path, roots_path, base_path))
assert source["status"] == "bounded_known_scalar_query_cubic_transfer_screen"
assert source["source_sha256"] == sha(HERE / "line_known_scalar_query_transfer.py")
assert source["roots_sha256"] == sha(roots_path)
assert source["runtime_sha256"] == sha(HERE / "type_i_known_scalar_query_transfer_runtime_info.json")
p, r = int(source["field_prime"]), int(source["subgroup_order"])
h = int(base["candidate_cofactor"])
assert p == int(roots["field_prime"]) and r == int(roots["subgroup_order"])
assert on_curve((GX, GY), p) and mul(r, (GX, GY), p) is None
assert source["queries_checked"] == len(source["rows"]) == 27
assert source["common_affine_coefficients"] == []

root_lookup = {(row["law"], row["query_index"]): row for row in roots["rows"]}
constant = None
for row in source["rows"]:
    input_row = root_lookup[row["law"], row["query_index"]]
    assert row["scalar"] == input_row["scalar"]
    assert row["points"] == input_row["positive"]["rational_points"]
    assert len(row["points"]) == len(row["projected_points"]) == 3
    a = int(row["scalar"])
    Q = mul((-2 * a) % r, (GX, GY), p)
    assert Q == tuple(map(int, row["quotient"]))
    sigma = None
    for encoded in row["projected_points"]:
        P = tuple(map(int, encoded))
        assert on_curve(P, p)
        sigma = add(sigma, P, p)
    assert sigma == tuple(map(int, row["projected_sum"]))
    candidate = add(sigma, mul((2 * h) % r, Q, p), p)
    assert candidate is not None and on_curve(candidate, p)
    if constant is None:
        constant = candidate
    assert candidate == constant

out = {
    "status": "independent_known_scalar_query_affine_transfer_pass",
    "queries_checked": len(source["rows"]),
    "coefficient_mod_r": str((-2 * h) % r),
    "cofactor_mod_r": str(h % r),
    "shared_constant": list(constant),
    "known_scalar_difference_multiplier_mod_r": str((2 * h) % r),
    "source_sha256": sha(Path(__file__)),
    "transfer_sha256": sha(source_path),
    "cofactor_receipt_sha256": sha(base_path),
}
path = HERE / "type_i_known_scalar_query_transfer_replay.json"
path.write_text(json.dumps(out, indent=2, sort_keys=True) + "\n")
print("QUERY_TRANSFER_REPLAY_PASS", json.dumps({
    "queries": out["queries_checked"],
    "coefficient_mod_r": out["coefficient_mod_r"],
    "shared_constant": out["shared_constant"],
}, sort_keys=True), flush=True)

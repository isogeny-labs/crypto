"""Serialize frozen query cubics for an independent PARI/GP root replay."""

import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
source_path = HERE / "type_i_known_scalar_query_cubic_roots.json"
plane_path = HERE / "type_i_plane_quartic.json"
source, plane = (json.loads(path.read_text()) for path in (source_path, plane_path))
assert source["status"] == "bounded_known_scalar_query_cubic_root_census"
assert source["queries"] == source["verified_pairs"] == 128


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


terms = [[*map(int, label.split(",")[:2]), int(value)]
         for label, value in plane["quartic_coefficients"].items()]
cases = []
for row in source["rows"]:
    assert row["status"] == "verified_query_cubic_pair"
    for role in ("positive", "half"):
        item = row[role]
        cubic = item["cubic"]
        assert all(len(coefficient) == 1 for coefficient in
                   cubic["residual_x_polynomial"] +
                   cubic["residual_y_polynomial"])
        cases.append([
            [int(coefficient[0]) for coefficient in cubic["residual_x_polynomial"]],
            [int(coefficient[0]) for coefficient in cubic["residual_y_polynomial"]],
            [[int(value) for value in point] for point in item["rational_points"]],
        ])
assert len(cases) == 256
gp_path = HERE / "type_i_known_scalar_query_cubic_roots_input.gp"
with gp_path.open("w") as output:
    output.write(f"p={plane['p']};\n")
    output.write(f"terms={terms};\n")
    output.write(f"cases={cases};\n")
receipt = {
    "status": "prepared_known_scalar_query_cubic_native_replay",
    "cases": len(cases), "source_sha256": sha(Path(__file__)),
    "census_sha256": sha(source_path), "plane_sha256": sha(plane_path),
    "gp_input_sha256": sha(gp_path),
}
(HERE / "type_i_known_scalar_query_cubic_roots_input.json").write_text(
    json.dumps(receipt, indent=2, sort_keys=True) + "\n")
print("QUERY_CUBIC_NATIVE_INPUT_PASS", len(cases), flush=True)

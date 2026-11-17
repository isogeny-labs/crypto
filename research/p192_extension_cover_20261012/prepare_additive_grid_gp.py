"""Serialize all sixteen grid cubic cuts for independent PARI/GP replay."""

import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
source_path = HERE / "type_i_additive_grid.json"
plane_path = HERE / "type_i_plane_quartic.json"
source = json.loads(source_path.read_text())
plane = json.loads(plane_path.read_text())
assert source["status"] == "verified_p192_additive_grid_fixed_origin_probe"
assert source["reduced_classes"] == 16
assert len(source["rows"]) == len(source["representatives"]) == 16


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def coeff(encoded):
    assert all(len(item) == 1 for item in encoded)
    return [int(item[0]) for item in encoded]


terms = [[*map(int, label.split(",")[:2]), int(value)]
         for label, value in plane["quartic_coefficients"].items()]
anchors = [[int(v) for v in item["coordinates_Fp_decimal"]]
           for item in plane["known_points_verified"]
           if item["source"] == "base"][:3]
exponents = [[3, 0], [0, 3], [0, 0], [2, 1], [2, 0],
             [1, 2], [0, 2], [1, 0], [0, 1], [1, 1]]
cases = []
for row, representative in zip(source["rows"], source["representatives"]):
    assert row["status"] == "verified_prime_field_cubic_pair"
    assert representative["status"] == "verified_fixed_origin_degree_three"
    A, B = row["positive_input_residual"], row["negative_input_residual"]
    cases.append([
        coeff(A["residual_x_polynomial"]), coeff(A["residual_y_polynomial"]),
        coeff(B["residual_x_polynomial"]), coeff(B["residual_y_polynomial"]),
        list(map(int, representative["denominator_cubic_coefficients"])),
        coeff(representative["intermediate_x_polynomial"]),
        coeff(representative["intermediate_y_polynomial"]),
        list(map(int, representative["numerator_cubic_coefficients"])),
        coeff(representative["representative_x_polynomial"]),
        coeff(representative["representative_y_polynomial"]),
    ])
gp_path = HERE / "type_i_additive_grid_input.gp"
with gp_path.open("w") as output:
    output.write(f"p={plane['p']};\n")
    output.write(f"terms={terms};\n")
    output.write(f"exponents={exponents};\n")
    output.write(f"anchors={anchors};\n")
    output.write(f"cases={cases};\n")
receipt = {
    "status": "prepared_additive_grid_native_replay", "cases": len(cases),
    "source_sha256": sha(Path(__file__)),
    "reduction_sha256": sha(source_path),
    "plane_sha256": sha(plane_path),
    "gp_input_sha256": sha(gp_path),
}
(HERE / "type_i_additive_grid_input.json").write_text(
    json.dumps(receipt, indent=2, sort_keys=True) + "\n")
print("ADDITIVE_GRID_NATIVE_INPUT_PASS", len(cases), flush=True)

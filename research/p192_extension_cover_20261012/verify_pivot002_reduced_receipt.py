"""Bind the exact pivot-002 selection to independent cubic-cut replay."""

import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


reduced_path = HERE / "type_i_pivot002_reduced32.json"
input_path = HERE / "type_i_pivot002_reduced32_input.json"
gp_path = HERE / "type_i_pivot002_reduced32_input.gp"
gp_log_path = HERE / "type_i_pivot002_reduced32_native.log"
reduced = json.loads(reduced_path.read_text())
prepared = json.loads(input_path.read_text())
fans = [HERE / "type_i_residual_pivot002_cross.json"]
assert reduced["source_sha256"] == sha(HERE / "line_pivot002_reduced_classes.py")
assert reduced["fan_sha256"] == [sha(path) for path in fans]
assert reduced["quotient_constructor_sha256"] == sha(HERE / "line_quotient_cubic_probe.py")
assert reduced["reduction_constructor_sha256"] == sha(HERE / "line_quotient_fixed_origin.py")
assert reduced["plane_receipt_sha256"] == sha(HERE / "type_i_plane_quartic.json")
assert reduced["density_receipt_sha256"] == sha(HERE / "type_i_character_density.json")
assert prepared["source_sha256"] == sha(HERE / "prepare_pivot002_gp.py")
assert prepared["reduction_sha256"] == sha(reduced_path)
assert prepared["gp_input_sha256"] == sha(gp_path)
assert prepared["cases"] == reduced["input_classes"] == reduced["reduced_classes"] == 32
assert gp_log_path.read_text().strip() == (
    "PIVOT002_FIXED_ORIGIN_NATIVE_PASS cases=32 prime_bits=192")

selected = set()
expected = []
for pivot, path in ((2, fans[0]),):
    fan = json.loads(path.read_text())
    count = 0
    for row_index, row in enumerate(fan["rows"]):
        for side, point in enumerate(row["projected_residuals"]):
            coordinates = tuple(map(int, point))
            if coordinates in selected:
                continue
            selected.add(coordinates)
            expected.append((pivot, row_index, side, coordinates))
            count += 1
            if count == 32:
                break
        if count == 32:
            break
    assert count == 32
assert len(expected) == len(selected) == 32
for index, (row, representative, selected_row) in enumerate(zip(
        reduced["rows"], reduced["representatives"], expected)):
    pivot, fan_row, side, coordinates = selected_row
    assert row["index"] == representative["index"] == index
    assert (row["pivot"], row["fan_row_index"], row["residual_side"]) == (
        pivot, fan_row, side)
    assert tuple(map(int, row["quotient"])) == coordinates
    assert representative["quotient"] == row["quotient"]
    assert row["status"] == "verified_prime_field_cubic_pair"
    assert representative["status"] == "verified_fixed_origin_degree_three"

output = {
    "status": "verified_pivot002_reduced_receipt",
    "classes_replayed": len(expected), "classes_per_fan": 32,
    "reduction_sha256": sha(reduced_path), "gp_input_sha256": sha(gp_path),
    "gp_log_sha256": sha(gp_log_path),
    "gp_verifier_source_sha256": sha(HERE / "verify_pivot002_reduced_native.gp"),
    "source_sha256": sha(Path(__file__)),
    "boundary": "Selection, source hashes, frozen cubic input, and independent GP pass marker are bound; the GP script checks concrete quartic cuts.",
}
(HERE / "type_i_pivot002_reduced32_replay.json").write_text(
    json.dumps(output, indent=2, sort_keys=True) + "\n")
print("PIVOT002_REDUCED_RECEIPT_PASS", len(expected))

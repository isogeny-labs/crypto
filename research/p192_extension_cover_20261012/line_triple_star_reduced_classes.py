"""Construct fixed-origin degree-three representatives for a 16-endpoint star."""

import hashlib
import json
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
FAN_NAMES = ["type_i_residual_pivot000_three.json",
             "type_i_residual_pivot001_cross.json",
             "type_i_residual_pivot002_cross.json"]
sha = lambda path: hashlib.sha256(Path(path).read_bytes()).hexdigest()


def prelude(name):
    path = HERE / name
    source = path.read_text()
    marker = "start = time.monotonic_ns()"
    assert source.count(marker) == 1
    namespace = {"__file__": str(path), "__name__": name[:-3] + "_prelude"}
    exec(compile(source.split(marker)[0], str(path), "exec"), namespace)
    return namespace, source


def main():
    screen_path = HERE / "type_i_triple_star_screen.json"
    screen = json.loads(screen_path.read_text())
    assert screen["source_sha256"] == sha(HERE / "line_triple_star_screen.py")
    assert screen["status"] == "verified_p192_source_free_triple_star_screen"
    assert screen["cut_edges"] == 48 and screen["projected_classes"] == 96
    endpoints = list(map(int, screen["selected_endpoints"]))
    fans = [json.loads((HERE / name).read_text()) for name in FAN_NAMES]
    quotient, _ = prelude("line_quotient_cubic_probe.py")
    reduction, reduction_source = prelude("line_quotient_fixed_origin.py")
    density_path = HERE / "type_i_character_density.json"
    density = json.loads(density_path.read_text())
    coefficients = [int(v) for v in density["sextic_coefficients_decimal"]]
    E, L, Integer = quotient["E"], quotient["L"], quotient["Integer"]
    r, O, inv2 = quotient["r"], quotient["O"], quotient["inv2"]
    assert str(quotient["p"]) == screen["field_prime"]
    assert str(r) == screen["subgroup_order"]

    def character(point):
        x = int(point[0].polynomial()[0])
        value = 0
        for coefficient in reversed(coefficients):
            value = (value * x + coefficient) % int(quotient["p"])
        assert value != 0
        return "+" if pow(value, (int(quotient["p"]) - 1) // 2,
                           int(quotient["p"])) == 1 else "-"

    samples = []
    for pivot, fan in enumerate(fans):
        assert fan.get("pivot_index", 0) == pivot
        assert screen["input_sha256"][FAN_NAMES[pivot]] == sha(HERE / FAN_NAMES[pivot])
        by_endpoint = {int(row["selected_index"]): (index, row)
                       for index, row in enumerate(fan["rows"])}
        assert len(by_endpoint) == len(fan["rows"])
        assert not set(endpoints) & set(map(int, fan["source_line_selected_pair"]))
        for endpoint in endpoints:
            fan_row_index, fan_row = by_endpoint[endpoint]
            for side, Q in enumerate(fan_row["projected_residuals"]):
                samples.append((pivot, endpoint, fan_row_index, side,
                                tuple(map(int, Q))))
    assert len(samples) == len({row[4] for row in samples}) == 96

    rows = []
    start = time.monotonic_ns()
    for index, (pivot, endpoint, fan_row_index, side, (xx, yy)) in enumerate(samples):
        Q = E(L(Integer(xx)), L(Integer(yy)))
        assert Q != O and r * Q == O
        H = inv2 * Q
        assert 2 * H == Q
        rows.append({
            "index": index, "pivot": pivot, "selected_endpoint": endpoint,
            "fan_row_index": fan_row_index, "residual_side": side,
            "quotient": [str(xx), str(yy)],
            "cell": character(H) + character(Q),
            "positive_input_residual": quotient["residual"](Q),
            "negative_input_residual": quotient["residual"](H),
            "status": "verified_prime_field_cubic_pair",
        })
        if (index + 1) % 16 == 0:
            print("TRIPLE_STAR_CUBIC_PROGRESS", index + 1, len(samples), flush=True)

    reduction["source"] = {"rows": rows}
    reduction["representatives"] = []
    loop = reduction_source[
        reduction_source.index('for item in source["rows"]:'):
        reduction_source.index("\nsuccesses =")
    ]
    exec(compile(loop, str(HERE / "line_quotient_fixed_origin.py"), "exec"), reduction)
    representatives = reduction["representatives"]
    assert len(representatives) == 96
    successes = sum(item["status"] == "verified_fixed_origin_degree_three"
                    for item in representatives)
    output = {
        "status": "bounded_triple_star_fixed_origin_reduction",
        "candidate_id": None,
        "field_prime": str(quotient["p"]), "subgroup_order": str(r),
        "selected_endpoints": endpoints,
        "input_classes": len(samples), "reduced_classes": successes,
        "unreduced_classes": len(samples) - successes,
        "rows": rows, "representatives": representatives,
        "origin_x_polynomial": reduction["encode_poly"](reduction["origin_h"]),
        "origin_y_polynomial": reduction["encode_poly"](reduction["origin_y"]),
        "class_identity": "G_cut=B+T+U and F_cut=A+U+D imply [D-T]=[B-A]",
        "elapsed_ns_unisolated": time.monotonic_ns() - start,
        "source_sha256": sha(__file__), "screen_sha256": sha(screen_path),
        "fan_sha256": {name: sha(HERE / name) for name in FAN_NAMES},
        "quotient_constructor_sha256": sha(HERE / "line_quotient_cubic_probe.py"),
        "reduction_constructor_sha256": sha(HERE / "line_quotient_fixed_origin.py"),
        "plane_receipt_sha256": sha(HERE / "type_i_plane_quartic.json"),
        "density_receipt_sha256": sha(density_path),
        "runtime_sha256": sha(HERE / "type_i_triple_star_runtime_info.json"),
        "boundary": "96 exact source-free triple-star quotient classes. Each successful two-cubic section pair yields a fixed-origin degree-three representative; independent PARI replay is separate.",
    }
    path = HERE / "type_i_triple_star_reduced96.json"
    path.write_text(json.dumps(output, indent=2, sort_keys=True) + "\n")
    print("TRIPLE_STAR_REDUCED", successes, "/", len(samples), sha(path), flush=True)


if __name__ == "__main__":
    main()

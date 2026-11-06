"""Test the quotient sum of fully split known-scalar query cubics.

For each distinct query with three rational positive-cubic points, project
all three quartic points through the checked three-step Frobenius map and
compare the sum with small integer multiples of the known quotient Q.
"""

import hashlib
import json
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
sha = lambda path: hashlib.sha256(Path(path).read_bytes()).hexdigest()


def prelude(name, marker):
    path = HERE / name
    source = path.read_text()
    assert source.count(marker) == 1
    namespace = {"__file__": str(path), "__name__": name[:-3] + "_prelude"}
    exec(compile(source.split(marker)[0], str(path), "exec"), namespace)
    return namespace


def main():
    roots_path = HERE / "type_i_known_scalar_query_cubic_roots.json"
    roots = json.loads(roots_path.read_text())
    assert roots["verified_pairs"] == 128
    ns = prelude("line_residual_pivot_three.py", "start_ns = time.monotonic_ns()")
    inverse, E6, E0 = ns["inverse"], ns["E6"], ns["E0"]
    trace3, scalar3, anchor_trace3 = (ns[name] for name in
                                      ("trace3", "scalar3", "anchor_trace3"))
    F = ns["dt"].gm.F
    r = ns["dt"].r

    def project(coords):
        cover, ys = inverse.invert_affine(F(int(coords[0])), F(int(coords[1])))
        image = inverse.quotient_image(E6, cover, ys, 0)
        P = scalar3 * (trace3(image) - anchor_trace3)
        assert r * P == E0(0)
        return P

    def key(P):
        return None if P == E0(0) else list(map(int, ns["projector"].key(P)))

    seen = set()
    selected = []
    start = time.monotonic_ns()
    for row in roots["rows"]:
        if row["scalar"] in seen or row["positive"]["rational_point_count"] != 3:
            continue
        seen.add(row["scalar"])
        Q = E0(F(int(row["quotient"][0])), F(int(row["quotient"][1])))
        projections = [project(point) for point in row["positive"]["rational_points"]]
        sigma = sum(projections, E0(0))
        selected.append({
            "law": row["law"], "query_index": row["query_index"],
            "scalar": row["scalar"], "quotient": row["quotient"],
            "points": row["positive"]["rational_points"],
            "projected_points": [key(P) for P in projections],
            "projected_sum": key(sigma),
            "candidate_constants": {str(k): key(sigma - k * Q)
                                    for k in range(-12, 13)},
        })
        print("QUERY_TRANSFER_PROGRESS", len(selected), flush=True)
    assert selected

    common = [k for k in range(-12, 13)
              if len({json.dumps(row["candidate_constants"][str(k)])
                      for row in selected}) == 1]
    out = {
        "status": "bounded_known_scalar_query_cubic_transfer_screen",
        "candidate_id": None,
        "field_prime": roots["field_prime"],
        "subgroup_order": roots["subgroup_order"],
        "selection_policy": "all distinct scalar queries whose positive cubic has three rational points",
        "queries_checked": len(selected),
        "coefficient_search_interval": [-12, 12],
        "common_affine_coefficients": common,
        "rows": selected,
        "elapsed_ns_unisolated": time.monotonic_ns() - start,
        "source_sha256": sha(__file__),
        "roots_sha256": sha(roots_path),
        "projector_source_sha256": sha(HERE / "frobenius_trace_projection.py"),
        "inverse_source_sha256": sha(HERE / "plane_point_inverse_cached.py"),
        "runtime_sha256": sha(HERE / "type_i_known_scalar_query_transfer_runtime_info.json"),
        "boundary": "Numerical quotient-sum identity screen for fully split positive query cubics. A shared affine coefficient is a concrete proof target; the full divisor-class bridge and relation-stage rank remain separate.",
    }
    path = HERE / "type_i_known_scalar_query_transfer.json"
    path.write_text(json.dumps(out, indent=2, sort_keys=True) + "\n")
    print("QUERY_TRANSFER_SCREEN_PASS", json.dumps({
        "queries_checked": len(selected),
        "common_affine_coefficients": common,
        "elapsed_s_unisolated": round(out["elapsed_ns_unisolated"] / 1e9, 3),
    }, sort_keys=True), flush=True)


if __name__ == "__main__":
    main()

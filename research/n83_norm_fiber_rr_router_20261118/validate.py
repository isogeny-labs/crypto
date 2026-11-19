#!/usr/bin/env python3
"""Independent arithmetic replay for the n=9 norm-fiber RR certificate."""

from __future__ import annotations

import argparse
import hashlib
import json
import math
from pathlib import Path
from typing import Any

R = 127
BOUNDS = (8, 16, 32, 64, 96, 126)


def inverse(value: int) -> int:
    return pow(value, -1, R)


def eliminate(rows: list[list[int]], rhs: list[int]) -> tuple[int, bool, list[int] | None]:
    if not rows:
        return 0, all(value == 0 for value in rhs), None
    matrix = [row[:] for row in rows]
    values = rhs[:]
    columns = len(matrix[0])
    pivot_row = 0
    pivots: list[int] = []
    for column in range(columns):
        found = next(
            (row for row in range(pivot_row, len(matrix)) if matrix[row][column] % R),
            None,
        )
        if found is None:
            continue
        matrix[pivot_row], matrix[found] = matrix[found], matrix[pivot_row]
        values[pivot_row], values[found] = values[found], values[pivot_row]
        scale = inverse(matrix[pivot_row][column] % R)
        matrix[pivot_row] = [(entry * scale) % R for entry in matrix[pivot_row]]
        values[pivot_row] = values[pivot_row] * scale % R
        for row in range(len(matrix)):
            if row == pivot_row:
                continue
            factor = matrix[row][column] % R
            if factor:
                matrix[row] = [
                    (left - factor * right) % R
                    for left, right in zip(matrix[row], matrix[pivot_row], strict=True)
                ]
                values[row] = (values[row] - factor * values[pivot_row]) % R
        pivots.append(column)
        pivot_row += 1
        if pivot_row == len(matrix):
            break
    consistent = all(any(entry % R for entry in row) or value % R == 0 for row, value in zip(matrix, values, strict=True))
    solution = None
    if consistent and len(pivots) == columns:
        solution = [0] * columns
        for row, column in enumerate(pivots):
            solution[column] = values[row] % R
    return len(pivots), consistent, solution


def close(left: float, right: float) -> bool:
    return math.isclose(left, right, rel_tol=1e-13, abs_tol=1e-15)


def ratio(numerator: int, denominator: int) -> float:
    return numerator / denominator if denominator else 0.0


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def validate_arm(
    arm: dict[str, Any],
    factor_base: list[dict[str, Any]],
    checks: list[str],
) -> None:
    counts = arm["counts"]
    relations = arm["relations"]
    require(counts["candidates"] == arm["class_count"] * 512, f"{arm['name']}: candidate total")
    require(counts["split"] == len(relations), f"{arm['name']}: split count")
    require(
        counts["exact_degree_four"] == counts["split"] + counts["nonsplit"],
        f"{arm['name']}: exact partition",
    )
    require(
        counts["exact_degree_four"] + sum(counts["rejections"].values())
        == counts["candidates"],
        f"{arm['name']}: candidate partition",
    )
    require(counts["consistency_failures"] == 0, f"{arm['name']}: consistency failures")

    fb_index_by_log = {entry["discrete_log"]: entry["index"] for entry in factor_base}
    expected_logs = [entry["discrete_log"] for entry in factor_base]
    for relation in relations:
        labels = relation["projected_label_logs"]
        require(len(labels) == 4, f"{arm['name']}: relation arity")
        require(all(0 <= label < R for label in labels), f"{arm['name']}: label range")
        require(sum(labels) % R == relation["rhs_log"], f"{arm['name']}: relation equation")

    cells = {cell["factor_base_bound"]: cell for cell in arm["grid"]}
    require(tuple(sorted(cells)) == BOUNDS, f"{arm['name']}: bound grid")
    for bound in BOUNDS:
        selected: list[tuple[dict[str, Any], list[int]]] = []
        for relation in relations:
            row = [0] * bound
            admitted = True
            for label in relation["projected_label_logs"]:
                if label == 0:
                    continue
                index = fb_index_by_log[label]
                if index >= bound:
                    admitted = False
                    break
                row[index] = (row[index] + 1) % R
            if admitted:
                selected.append((relation, row))
        rows = [row for _, row in selected]
        rhs = [relation["rhs_log"] for relation, _ in selected]
        rank, consistent, solution = eliminate(rows, rhs)
        nonzeros = sum(sum(entry != 0 for entry in row) for row in rows)
        raw_components = len(selected) * 4
        raw_nonidentity = sum(
            sum(label != 0 for label in relation["projected_label_logs"])
            for relation, _ in selected
        )
        distinct_all = sum(
            len(set(relation["projected_label_logs"])) for relation, _ in selected
        )
        distinct_matrix = sum(
            len({label for label in relation["projected_label_logs"] if label})
            for relation, _ in selected
        )
        global_labels = {
            label
            for relation, _ in selected
            for label in relation["projected_label_logs"]
            if label
        }
        collisions = raw_components - distinct_all
        cell = cells[bound]
        exact = {
            "rows": len(rows),
            "columns": bound,
            "nonzeros": nonzeros,
            "rank": rank,
            "raw_projected_components": raw_components,
            "raw_nonidentity_components": raw_nonidentity,
            "sum_distinct_labels_including_identity": distinct_all,
            "sum_distinct_matrix_labels": distinct_matrix,
            "global_distinct_matrix_labels": len(global_labels),
            "projected_collision_count_including_identity": collisions,
            "csr_bytes_u32_columns_u16_values": nonzeros * 6 + (len(rows) + 1) * 4,
        }
        for field, value in exact.items():
            require(cell[field] == value, f"{arm['name']} B={bound}: {field}")
        floats = {
            "rank_per_row": ratio(rank, len(rows)),
            "rank_per_candidate": ratio(rank, counts["candidates"]),
            "relation_probability_per_candidate": ratio(len(rows), counts["candidates"]),
            "relation_probability_per_exact_representation": ratio(
                len(rows), counts["exact_degree_four"]
            ),
            "projected_collision_rate_including_identity": ratio(collisions, raw_components),
        }
        for field, value in floats.items():
            require(close(cell[field], value), f"{arm['name']} B={bound}: {field}")
        require(cell["consistent"] == consistent, f"{arm['name']} B={bound}: consistency")
        require(cell["full_rank"] == (rank == bound), f"{arm['name']} B={bound}: full rank")
        expected_match = None if solution is None else solution == expected_logs[:bound]
        require(
            cell["solution_matches_enumerated_logs"] == expected_match,
            f"{arm['name']} B={bound}: solved label logs",
        )
    checks.append(f"arm:{arm['name']}")


def validate(document: dict[str, Any], source: Path) -> dict[str, Any]:
    checks: list[str] = []
    require(document["schema"] == "crypto.n83-norm-fiber-rr-router-audit/v1", "schema")
    require(document["status"] == "pass", "producer status")
    field = document["field"]
    require(field["degree"] == 9 and field["cardinality"] == 512, "field")
    require(
        field["enumerated_target_group_order"] == field["predicted_target_group_order"] == 508,
        "target group order",
    )
    subgroup = document["subgroup"]
    require(subgroup["order"] == 127 and subgroup["cofactor"] == 4, "subgroup order")
    require(subgroup["unique_multiples"] == 127 and subgroup["order_check"], "subgroup certificate")
    kernel = document["kernel"]
    require(kernel["kernel_nonidentity"], "kernel nonidentity")
    require(kernel["kernel_pushforward_is_identity"], "kernel pushforward")
    require(not kernel["seed_pushforward_is_identity"], "non-kernel seed control")
    checks.extend(("field", "subgroup", "kernel"))

    factor_base = document["factor_base"]
    require(len(factor_base) == 126, "factor-base size")
    require([entry["index"] for entry in factor_base] == list(range(126)), "factor-base indices")
    require(
        {entry["discrete_log"] for entry in factor_base} == set(range(1, 127)),
        "factor-base logs",
    )
    point_keys = [
        (entry["point"]["infinity"], entry["point"]["x"], entry["point"]["y"])
        for entry in factor_base
    ]
    require(point_keys == sorted(point_keys), "factor-base canonical order")
    checks.append("factor_base")

    arms = document["arms"]
    require(
        [arm["name"] for arm in arms]
        == ["transported_k0", "kernel_coset", "ordinary_ambient", "exceptional"],
        "arm order",
    )
    for arm in arms:
        validate_arm(arm, factor_base, checks)

    blind = document["blind_individual_logs"]
    require([entry["arm"] for entry in blind] == ["transported_k0", "kernel_coset"], "blind arms")
    for entry in blind:
        require(entry["successes"] == sum(trial["success"] for trial in entry["trials"]), "blind successes")
        require(
            entry["total_candidates"] == sum(trial["candidates_tried"] for trial in entry["trials"]),
            "blind candidate total",
        )
        for trial in entry["trials"]:
            if trial["success"]:
                require(
                    trial["recovered_log"] == trial["planted_log_for_verification"],
                    "blind recovered log",
                )
        checks.append(f"blind:{entry['arm']}")

    gate = document["advancement_gate"]
    require(not gate["finite_crossover_against_source_rho_predicted"], "finite crossover gate")
    require(not gate["advance_to_n13"], "n=13 stop gate")
    require(close(gate["registered_balanced_total_exponent_in_q"], 1.6), "modeled exponent")
    require(close(gate["source_rho_exponent_in_q"], 0.5), "rho exponent")
    checks.append("advancement_gate")

    input_sha256 = hashlib.sha256(source.read_bytes()).hexdigest()
    arm_by_name = {arm["name"]: arm for arm in arms}
    b64 = {
        name: next(
            cell
            for cell in arm_by_name[name]["grid"]
            if cell["factor_base_bound"] == 64
        )
        for name in ("transported_k0", "kernel_coset", "ordinary_ambient")
    }
    headline_values = {
        "transported_b64_rows": b64["transported_k0"]["rows"],
        "transported_b64_rank": b64["transported_k0"]["rank"],
        "kernel_b64_rows": b64["kernel_coset"]["rows"],
        "kernel_b64_rank": b64["kernel_coset"]["rank"],
        "ambient_b64_rows": b64["ordinary_ambient"]["rows"],
        "ambient_b64_rank": b64["ordinary_ambient"]["rank"],
        "transported_blind_successes": blind[0]["successes"],
        "kernel_blind_successes": blind[1]["successes"],
    }
    return {
        "schema": "crypto.n83-norm-fiber-rr-router-validator/v1",
        "status": "pass",
        "input": str(source),
        "input_sha256": input_sha256,
        "raw_data": [{"path": str(source), "sha256": input_sha256}],
        "headline": [
            {
                "name": name,
                "claimed": value,
                "recomputed": value,
                "rel_tol": 0.0,
            }
            for name, value in headline_values.items()
        ],
        "checks": checks,
        "check_count": len(checks),
    }


def self_test() -> None:
    rows = [[1, 2, 0], [0, 1, 3], [4, 0, 1]]
    expected = [11, 29, 83]
    rhs = [sum(a * b for a, b in zip(row, expected, strict=True)) % R for row in rows]
    rank, consistent, solution = eliminate(rows, rhs)
    require(rank == 3 and consistent and solution == expected, "elimination self-test")
    rank, consistent, solution = eliminate([[1, 1], [2, 2]], [1, 3])
    require(rank == 1 and not consistent and solution is None, "inconsistency self-test")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("certificate", nargs="?", type=Path)
    parser.add_argument("--output", type=Path)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        self_test()
        print("validator self-test: pass")
        return
    if args.certificate is None:
        parser.error("certificate is required unless --self-test is used")
    document = json.loads(args.certificate.read_text())
    report = validate(document, args.certificate)
    encoded = json.dumps(report, indent=2, sort_keys=True) + "\n"
    if args.output:
        args.output.write_text(encoded)
    else:
        print(encoded, end="")


if __name__ == "__main__":
    main()

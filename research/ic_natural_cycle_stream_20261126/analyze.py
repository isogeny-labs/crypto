#!/usr/bin/env python3
"""Measure signed-cycle and projected-solver stopping events on a fixed stream."""

from __future__ import annotations

import argparse
import hashlib
import json
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable

MODULUS = 2_147_446_831
TARGET_COLUMN = 4_294_967_295
RHS_COLUMN = -1


def canonical(terms: Iterable[Iterable[int]], modulus: int) -> dict[int, int]:
    out: dict[int, int] = {}
    previous = -1
    for raw_column, raw_coefficient in terms:
        column = int(raw_column)
        coefficient = int(raw_coefficient) % modulus
        if column <= previous:
            raise ValueError("terms must be strictly ordered by column")
        previous = column
        if coefficient == 0:
            raise ValueError("zero coefficient is not canonical")
        out[column] = coefficient
    return out


def add_scaled(
    destination: dict[int, int], source: dict[int, int], scale: int, modulus: int
) -> None:
    if scale % modulus == 0:
        return
    for column, coefficient in source.items():
        value = (destination.get(column, 0) + scale * coefficient) % modulus
        if value:
            destination[column] = value
        else:
            destination.pop(column, None)


class SparseRank:
    """Incremental sparse row rank over a prime field."""

    def __init__(self, modulus: int):
        self.modulus = modulus
        self.pivots: dict[int, dict[int, int]] = {}

    @property
    def rank(self) -> int:
        return len(self.pivots)

    def reduce(self, row: dict[int, int]) -> dict[int, int]:
        reduced = {c: v % self.modulus for c, v in row.items() if v % self.modulus}
        for pivot in sorted(self.pivots):
            coefficient = reduced.get(pivot, 0)
            if coefficient:
                add_scaled(reduced, self.pivots[pivot], -coefficient, self.modulus)
        return reduced

    def add(self, row: dict[int, int]) -> bool:
        reduced = self.reduce(row)
        if not reduced:
            return False
        pivot = min(reduced)
        inverse = pow(reduced[pivot], -1, self.modulus)
        normalized = {
            column: (coefficient * inverse) % self.modulus
            for column, coefficient in reduced.items()
        }
        self.pivots[pivot] = normalized
        return True


@dataclass
class CarriedRow:
    large: dict[int, int]
    small: dict[int, int]
    rhs: int
    source_weights: dict[int, int]

    def scaled(self, scale: int, modulus: int) -> "CarriedRow":
        return CarriedRow(
            {c: (v * scale) % modulus for c, v in self.large.items()},
            {c: (v * scale) % modulus for c, v in self.small.items()},
            (self.rhs * scale) % modulus,
            {c: (v * scale) % modulus for c, v in self.source_weights.items()},
        )


class LargeEliminator:
    """Eliminate LP columns while carrying small payload and source weights."""

    def __init__(self, modulus: int):
        self.modulus = modulus
        self.pivots: dict[int, CarriedRow] = {}

    @property
    def rank(self) -> int:
        return len(self.pivots)

    def process(self, row: CarriedRow) -> tuple[bool, CarriedRow]:
        current = CarriedRow(
            dict(row.large), dict(row.small), row.rhs, dict(row.source_weights)
        )
        for pivot in sorted(self.pivots):
            coefficient = current.large.get(pivot, 0)
            if not coefficient:
                continue
            basis = self.pivots[pivot]
            add_scaled(current.large, basis.large, -coefficient, self.modulus)
            add_scaled(current.small, basis.small, -coefficient, self.modulus)
            current.rhs = (current.rhs - coefficient * basis.rhs) % self.modulus
            add_scaled(
                current.source_weights,
                basis.source_weights,
                -coefficient,
                self.modulus,
            )
        if not current.large:
            return False, current
        pivot = min(current.large)
        normalized = current.scaled(pow(current.large[pivot], -1, self.modulus), self.modulus)
        self.pivots[pivot] = normalized
        return True, normalized


class GainDsu:
    """Track ordinary and gain-consistent closures of two-entry rows."""

    def __init__(self, modulus: int):
        self.modulus = modulus
        self.parent: dict[int, int] = {}
        self.weight: dict[int, int] = {}

    def find(self, vertex: int) -> tuple[int, int]:
        if vertex not in self.parent:
            self.parent[vertex] = vertex
            self.weight[vertex] = 1
            return vertex, 1
        parent = self.parent[vertex]
        if parent == vertex:
            return vertex, 1
        root, parent_weight = self.find(parent)
        self.weight[vertex] = self.weight[vertex] * parent_weight % self.modulus
        self.parent[vertex] = root
        return root, self.weight[vertex]

    def add_edge(self, large: dict[int, int]) -> tuple[bool, bool]:
        if len(large) != 2:
            raise ValueError("gain DSU accepts exactly two entries")
        (u, a), (v, b) = sorted(large.items())
        root_u, potential_u = self.find(u)
        root_v, potential_v = self.find(v)
        gain = (-a * pow(b, -1, self.modulus)) % self.modulus
        if root_u == root_v:
            implied = potential_v * pow(potential_u, -1, self.modulus) % self.modulus
            return True, implied == gain
        # x_v = potential_v*x_root_v and x_u = potential_u*x_root_u.
        # Set x_root_v = gain*potential_u/potential_v*x_root_u.
        self.parent[root_v] = root_u
        self.weight[root_v] = (
            gain * potential_u * pow(potential_v, -1, self.modulus)
        ) % self.modulus
        return False, False


def event_record(
    *,
    panel_position: int,
    source_position: int,
    projected: CarriedRow,
    lp_rank: int,
    observed_large_columns: int,
) -> dict[str, object]:
    return {
        "panel_position": panel_position,
        "source_position": source_position,
        "lp_rank": lp_rank,
        "observed_large_columns": observed_large_columns,
        "projected_target_coefficient": projected.small.get(TARGET_COLUMN, 0),
        "projected_small_nonzeros": len(projected.small),
        "projected_rhs": projected.rhs,
        "witness": [[column, coefficient] for column, coefficient in sorted(projected.source_weights.items())],
    }


def analyze_panel(
    rows: list[dict[str, object]], *, exactly_two_large: bool, modulus: int
) -> dict[str, object]:
    selected = [row for row in rows if (len(row["large"]) == 2 or not exactly_two_large)]
    eliminator = LargeEliminator(modulus)
    projected = SparseRank(modulus)
    projected_without_target = SparseRank(modulus)
    augmented = SparseRank(modulus)
    dsu = GainDsu(modulus) if exactly_two_large else None

    first_graph_cycle = None
    first_balanced_cycle = None
    first_lp_dependency = None
    first_target_projection = None
    first_projected_rank = None
    first_target_unique = None
    dependency_count = 0
    target_projecting_dependencies = 0
    seen_large_columns: set[int] = set()

    for panel_index, raw in enumerate(selected, start=1):
        source_position = int(raw["id"]) + 1
        large = canonical(raw["large"], modulus)
        seen_large_columns.update(large)
        small = canonical(raw["small"], modulus)
        rhs = int(raw["rhs"]) % modulus

        if dsu is not None:
            closes, balanced = dsu.add_edge(large)
            if closes and first_graph_cycle is None:
                first_graph_cycle = {
                    "panel_position": panel_index,
                    "source_position": source_position,
                }
            if balanced and first_balanced_cycle is None:
                first_balanced_cycle = {
                    "panel_position": panel_index,
                    "source_position": source_position,
                }

        independent, reduced = eliminator.process(
            CarriedRow(large, small, rhs, {int(raw["id"]): 1})
        )
        if independent:
            continue

        dependency_count += 1
        record = event_record(
            panel_position=panel_index,
            source_position=source_position,
            projected=reduced,
            lp_rank=eliminator.rank,
            observed_large_columns=len(seen_large_columns),
        )
        if first_lp_dependency is None:
            first_lp_dependency = record
        if reduced.small.get(TARGET_COLUMN, 0):
            target_projecting_dependencies += 1
            if first_target_projection is None:
                first_target_projection = record

        full_increased = projected.add(reduced.small)
        no_target = {c: v for c, v in reduced.small.items() if c != TARGET_COLUMN}
        projected_without_target.add(no_target)
        with_rhs = dict(reduced.small)
        if reduced.rhs:
            with_rhs[RHS_COLUMN] = reduced.rhs
        augmented.add(with_rhs)
        if full_increased and first_projected_rank is None:
            first_projected_rank = record

        target_unique = (
            projected.rank == projected_without_target.rank + 1
            and augmented.rank == projected.rank
        )
        if target_unique and first_target_unique is None:
            first_target_unique = record

    consistent = augmented.rank == projected.rank
    return {
        "selection": "exactly_two_large" if exactly_two_large else "all_admitted_arity_le_2",
        "source_rows_observed": len(rows),
        "panel_rows": len(selected),
        "distinct_large_columns_observed": len(seen_large_columns),
        "source_censor_position": int(rows[-1]["id"]) + 1 if rows else 0,
        "lp_rank": eliminator.rank,
        "lp_left_nullity": len(selected) - eliminator.rank,
        "dependency_count": dependency_count,
        "target_projecting_dependencies": target_projecting_dependencies,
        "projected_rank": projected.rank,
        "projected_rank_without_target": projected_without_target.rank,
        "augmented_rank": augmented.rank,
        "consistent": consistent,
        "target_unique_at_censor": consistent
        and projected.rank == projected_without_target.rank + 1,
        "events": {
            "first_graph_cycle": first_graph_cycle,
            "first_balanced_simple_cycle": first_balanced_cycle,
            "first_lp_dependency": first_lp_dependency,
            "first_nonzero_target_projection": first_target_projection,
            "first_projected_rank_increase": first_projected_rank,
            "first_exact_target_uniqueness": first_target_unique,
        },
    }


def load_rows(path: Path) -> list[dict[str, object]]:
    rows = [json.loads(line) for line in path.read_text().splitlines() if line.strip()]
    previous_id = -1
    for row in rows:
        row_id = int(row.get("id", -1))
        if row_id <= previous_id:
            raise ValueError(f"row ids are not strictly increasing at {row_id}")
        previous_id = row_id
        if len(row.get("large", [])) > 2:
            raise ValueError(f"row {row_id} exceeds admitted arity two")
    return rows


def run_self_test() -> None:
    q = 7

    def row(identifier: int, large: list[list[int]], small: list[list[int]]) -> dict[str, object]:
        return {"id": identifier, "large": large, "small": small, "rhs": 0}

    triangle = [
        row(0, [[0, 1], [1, 1]], [[TARGET_COLUMN, 1]]),
        row(1, [[1, 1], [2, 1]], [[TARGET_COLUMN, 1]]),
        row(2, [[0, 1], [2, 1]], [[TARGET_COLUMN, 1]]),
    ]
    triangle_result = analyze_panel(triangle, exactly_two_large=True, modulus=q)
    assert triangle_result["events"]["first_graph_cycle"]["panel_position"] == 3
    assert triangle_result["events"]["first_lp_dependency"] is None

    square = [
        row(0, [[0, 1], [1, 1]], [[TARGET_COLUMN, 1]]),
        row(1, [[1, 1], [2, 1]], [[TARGET_COLUMN, 1]]),
        row(2, [[2, 1], [3, 1]], [[TARGET_COLUMN, 1]]),
        row(3, [[0, 1], [3, 1]], [[TARGET_COLUMN, 2]]),
    ]
    square_result = analyze_panel(square, exactly_two_large=True, modulus=q)
    assert square_result["events"]["first_balanced_simple_cycle"]["panel_position"] == 4
    assert square_result["events"]["first_lp_dependency"]["panel_position"] == 4
    assert square_result["events"]["first_nonzero_target_projection"]["panel_position"] == 4
    assert square_result["events"]["first_exact_target_uniqueness"]["panel_position"] == 4

    projected_only = [
        row(0, [[0, 1], [1, 1]], [[8, 1], [TARGET_COLUMN, 1]]),
        row(1, [[0, 1], [1, 1]], [[8, 2], [TARGET_COLUMN, 2]]),
    ]
    projected_result = analyze_panel(projected_only, exactly_two_large=True, modulus=q)
    assert projected_result["events"]["first_projected_rank_increase"]["panel_position"] == 2
    assert projected_result["events"]["first_exact_target_uniqueness"] is None
    print("PASS planted triangle/square/projection controls")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", type=Path)
    parser.add_argument("--output", type=Path)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        run_self_test()
        if args.input is None:
            return
    if args.input is None or args.output is None:
        parser.error("--input and --output are required outside self-test-only mode")

    input_bytes = args.input.read_bytes()
    rows = load_rows(args.input)
    result = {
        "schema": "ic-natural-cycle-stopping/v1",
        "modulus": MODULUS,
        "target_column": TARGET_COLUMN,
        "input": {
            "path": str(args.input),
            "sha256": hashlib.sha256(input_bytes).hexdigest(),
            "rows": len(rows),
            "declared_large_prime_pool_size": 168,
            "historical_source_seed": "0x7033315f6d340101",
            "producer_commit": "577d6c2c1c4a527604d965e6fb62c44df7902f36",
            "producer_blob": "7fd27b9888058399d8814cef373d373243e6a1e5",
            "admitted_prefix_blob": "f1b02205745a74d1771578a68f0f1f435f4a8823",
        },
        "panels": {
            "two_lp_only": analyze_panel(rows, exactly_two_large=True, modulus=MODULUS),
            "all_admitted": analyze_panel(rows, exactly_two_large=False, modulus=MODULUS),
        },
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()

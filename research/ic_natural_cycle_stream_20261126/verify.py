#!/usr/bin/env python3
"""Independent direct-rank verifier for natural-cycle stopping results."""

from __future__ import annotations

import argparse
import hashlib
import json
from collections import deque
from pathlib import Path

TARGET = 4_294_967_295
LARGE_NS = 0
SMALL_NS = 1
RHS_NS = 2


def add_scaled(destination, source, scale, modulus):
    for key, value in source.items():
        updated = (destination.get(key, 0) + scale * value) % modulus
        if updated:
            destination[key] = updated
        else:
            destination.pop(key, None)


def reverse_pivot_rank(rows, modulus):
    """Sparse Gaussian rank using greatest-column pivots (not analyzer code)."""
    basis = {}
    for original in rows:
        row = {key: value % modulus for key, value in original.items() if value % modulus}
        while row:
            pivot = max(row)
            if pivot not in basis:
                inverse = pow(row[pivot], -1, modulus)
                basis[pivot] = {
                    key: value * inverse % modulus for key, value in row.items()
                }
                break
            add_scaled(row, basis[pivot], -row[pivot], modulus)
    return len(basis)


def encoded_rows(rows, *, large=True, small=False, target=True, rhs=False):
    encoded = []
    for row in rows:
        output = {}
        if large:
            for column, coefficient in row["large"]:
                output[(LARGE_NS, column)] = coefficient
        if small:
            for column, coefficient in row["small"]:
                if target or column != TARGET:
                    output[(SMALL_NS, column)] = coefficient
        elif target:
            for column, coefficient in row["small"]:
                if column == TARGET:
                    output[(SMALL_NS, column)] = coefficient
        if rhs and row["rhs"]:
            output[(RHS_NS, 0)] = row["rhs"]
        encoded.append(output)
    return encoded


def ranks(rows, modulus):
    rank_l = reverse_pivot_rank(encoded_rows(rows, target=False), modulus)
    rank_lt = reverse_pivot_rank(encoded_rows(rows, target=True), modulus)
    rank_ls = reverse_pivot_rank(encoded_rows(rows, small=True), modulus)
    rank_ls_without_target = reverse_pivot_rank(
        encoded_rows(rows, small=True, target=False), modulus
    )
    rank_augmented = reverse_pivot_rank(
        encoded_rows(rows, small=True, rhs=True), modulus
    )
    return {
        "l": rank_l,
        "lt": rank_lt,
        "ls": rank_ls,
        "ls_without_target": rank_ls_without_target,
        "augmented": rank_augmented,
    }


def first_cycle_by_bfs(rows, modulus):
    adjacency = {}
    first_cycle = None
    first_balanced = None
    for position, row in enumerate(rows, start=1):
        (u, a), (v, b) = row["large"]
        edge_gain = -a * pow(b, -1, modulus) % modulus
        queue = deque([(u, 1)])
        seen = {u}
        path_gain = None
        while queue:
            vertex, gain = queue.popleft()
            if vertex == v:
                path_gain = gain
                break
            for neighbour, step_gain in adjacency.get(vertex, []):
                if neighbour not in seen:
                    seen.add(neighbour)
                    queue.append((neighbour, gain * step_gain % modulus))
        if path_gain is not None:
            record = {
                "panel_position": position,
                "source_position": row["id"] + 1,
            }
            if first_cycle is None:
                first_cycle = record
            if path_gain == edge_gain and first_balanced is None:
                first_balanced = record
        adjacency.setdefault(u, []).append((v, edge_gain))
        adjacency.setdefault(v, []).append((u, pow(edge_gain, -1, modulus)))
    return first_cycle, first_balanced


def replay_witness(event, rows_by_id, modulus):
    large = {}
    small = {}
    rhs = 0
    for source_id, weight in event["witness"]:
        row = rows_by_id[source_id]
        add_scaled(large, dict(row["large"]), weight, modulus)
        add_scaled(small, dict(row["small"]), weight, modulus)
        rhs = (rhs + weight * row["rhs"]) % modulus
    assert not large, "reported witness does not cancel the LP block"
    assert small.get(TARGET, 0) == event["projected_target_coefficient"]
    assert len(small) == event["projected_small_nonzeros"]
    assert rhs == event["projected_rhs"]


def check_transition(panel_rows, event, predicate, modulus):
    if event is None:
        assert not predicate(ranks(panel_rows, modulus))
        return
    position = event["panel_position"]
    assert not predicate(ranks(panel_rows[: position - 1], modulus))
    assert predicate(ranks(panel_rows[:position], modulus))


def verify_panel(name, panel, all_rows, modulus):
    rows = (
        [row for row in all_rows if len(row["large"]) == 2]
        if panel["selection"] == "exactly_two_large"
        else list(all_rows)
    )
    assert len(rows) == panel["panel_rows"]
    final = ranks(rows, modulus)
    assert final["l"] == panel["lp_rank"]
    assert len(rows) - final["l"] == panel["lp_left_nullity"]
    assert final["ls"] - final["l"] == panel["projected_rank"]
    assert (
        final["ls_without_target"] - final["l"]
        == panel["projected_rank_without_target"]
    )
    assert final["augmented"] - final["l"] == panel["augmented_rank"]
    assert (final["augmented"] == final["ls"]) == panel["consistent"]

    events = panel["events"]
    dependency = events["first_lp_dependency"]
    if dependency is None:
        assert final["l"] == len(rows)
    else:
        position = dependency["panel_position"]
        assert ranks(rows[: position - 1], modulus)["l"] == position - 1
        assert ranks(rows[:position], modulus)["l"] == position - 1

    check_transition(
        rows,
        events["first_nonzero_target_projection"],
        lambda value: value["lt"] > value["l"],
        modulus,
    )
    check_transition(
        rows,
        events["first_projected_rank_increase"],
        lambda value: value["ls"] > value["l"],
        modulus,
    )
    check_transition(
        rows,
        events["first_exact_target_uniqueness"],
        lambda value: value["ls"] == value["augmented"]
        and value["ls"] == value["ls_without_target"] + 1,
        modulus,
    )

    rows_by_id = {row["id"]: row for row in all_rows}
    for event_name in (
        "first_lp_dependency",
        "first_nonzero_target_projection",
        "first_projected_rank_increase",
        "first_exact_target_uniqueness",
    ):
        event = events[event_name]
        if event is not None:
            replay_witness(event, rows_by_id, modulus)

    if panel["selection"] == "exactly_two_large":
        graph_cycle, balanced_cycle = first_cycle_by_bfs(rows, modulus)
        assert graph_cycle == events["first_graph_cycle"]
        assert balanced_cycle == events["first_balanced_simple_cycle"]
    print(f"PASS {name}: direct ranks, transitions, graph paths, witness replay")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", type=Path, required=True)
    parser.add_argument("--results", type=Path, required=True)
    args = parser.parse_args()

    input_bytes = args.input.read_bytes()
    result = json.loads(args.results.read_text())
    assert hashlib.sha256(input_bytes).hexdigest() == result["input"]["sha256"]
    assert result["schema"] == "ic-natural-cycle-stopping/v1"
    modulus = result["modulus"]
    rows = [json.loads(line) for line in input_bytes.splitlines() if line.strip()]
    for name, panel in result["panels"].items():
        verify_panel(name, panel, rows, modulus)


if __name__ == "__main__":
    main()

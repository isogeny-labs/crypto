#!/usr/bin/env python3
"""Audit opt-in chained-S3 phase markers and the exact public N83 query."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
import re
import sys

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parents[1]))
import chain_search_supervisor as group  # noqa: E402

PREFIX = "N83_CHAIN_PHASE "
QUERY = re.compile(
    r'RelationQueryStarted \{ trial: ([1-9][0-9]*), '
    r'coefficient_a: "(0|[1-9][0-9]*)", '
    r'coefficient_b: "(0|[1-9][0-9]*)", '
    r'target_x: "(0|[1-9][0-9]*)", '
    r'target_y: "(0|[1-9][0-9]*)", '
    r'target_infinity: (true|false) \}'
)
ORDER = (
    "entry", "x_domain_ready", "closure_started", "closure_ready",
    "domain_counted", "index_ready", "model_build_started", "model_built",
    "domain_install_started", "domain_installed", "solver_enter",
)


def require(ok: bool, reason: str) -> None:
    if not ok:
        raise ValueError(reason)


def phases_from_text(text: str) -> list[dict]:
    phases = []
    for line in text.splitlines():
        if PREFIX not in line:
            continue
        row = json.loads(line.split(PREFIX, 1)[1])
        require(row.get("schema") == "n83.chain-s3-phase/v1"
                and isinstance(row.get("phase"), str)
                and type(row.get("elapsed_ms")) in (int, float)
                and row["elapsed_ms"] >= 0,
                "malformed chained-S3 phase receipt")
        phases.append(row)
    require(phases, "phase tracing produced no flushed marker")
    return phases


def check_phase_order(phases: list[dict]) -> None:
    groups = []
    for row in phases:
        if row["phase"] == "entry":
            groups.append([])
        require(groups, "phase stream started without entry")
        groups[-1].append(row)
    for group_rows in groups:
        times = [row["elapsed_ms"] for row in group_rows]
        require(times == sorted(times), "phase elapsed time went backward")
        names = [row["phase"] for row in group_rows]
        last = -1
        for marker in ORDER:
            if marker in names:
                position = names.index(marker)
                require(position > last, f"phase {marker} is out of order")
                last = position
        for marker in ("solver_sat", "solver_unsat", "solver_unknown"):
            if marker in names:
                require("solver_enter" in names and
                        names.index(marker) > names.index("solver_enter"),
                        f"{marker} appeared before solver entry")


def positive(log: Path) -> dict:
    content = log.read_text()
    phases = phases_from_text(content)
    check_phase_order(phases)
    names = [row["phase"] for row in phases]
    require("solver_enter" in names and "group_lift_accepted" in names,
            "positive control did not pass through solver and group lift")
    require("test result: ok. 1 passed" in content,
            "positive-control Rust test did not pass")
    return {
        "schema": "n83.chain-s3-phase-positive-audit/v1",
        "status": "PASS_VERIFIED_SMALL_FIELD_TRACE",
        "phase_markers": len(phases),
        "decomposition_calls": names.count("entry"),
        "solver_entries": names.count("solver_enter"),
        "group_lifts_accepted": names.count("group_lift_accepted"),
        "selected_best_total_runtime": None,
    }


def query_from_events(events: Path, fixture: Path) -> dict:
    rows = [json.loads(line) for line in events.read_text().splitlines()]
    matches = [QUERY.fullmatch(row.get("event", "")) for row in rows]
    queries = [match for match in matches if match is not None]
    require(len(queries) == 1, "expected exactly one flushed public-query event")
    trial, a, b, x, y, infinity = queries[0].groups()
    require(int(trial) == 1 and 0 <= int(a) < group.PRIMARY_ORDER
            and 0 < int(b) < group.PRIMARY_ORDER,
            "public-query coefficients are outside the subgroup")
    corpus = json.loads(fixture.read_text())
    targets = [row for row in corpus["targets"]
               if row.get("a") == 0 and row.get("fixture") == 0]
    require(corpus.get("schema") == "n83.public-probe-corpus/v1"
            and len(targets) == 1, "public fixture zero is absent or ambiguous")
    target = group.parse_point(targets[0]["point"])
    require(target is not None and group.point_mul(target, group.PRIMARY_ORDER) is None,
            "public fixture zero is outside the subgroup")
    expected = group.point_add(
        group.point_mul(group.PRIMARY_GENERATOR, int(a)),
        group.point_mul(target, int(b)),
    )
    observed = None if infinity == "true" else group.parse_point([x, y])
    require(observed == expected, "flushed public query fails independent group replay")
    return {"trial": 1, "coefficient_a": a, "coefficient_b": b,
            "target": None if expected is None else [str(word) for word in expected],
            "status": "PASS_ORIGINAL_GROUP_QUERY"}


def native(stderr: Path, events: Path, fixture: Path) -> dict:
    phases = phases_from_text(stderr.read_text())
    check_phase_order(phases)
    names = [row["phase"] for row in phases]
    require(names.count("entry") == 1 and phases[0]["points"] == 679936
            and phases[0]["orbits"] == 4096 and phases[0]["summands"] == 5,
            "native trace is not the exact K4096 five-summand query")
    values = {row["phase"]: row for row in phases}
    if "x_domain_ready" in values:
        require(values["x_domain_ready"]["x_coordinates"] == 339968,
                "native K4096 x-domain size differs")
    if "closure_ready" in values:
        require(values["closure_ready"]["representatives"] == 4096
                and values["closure_ready"]["closure_x_coordinates"] == 339968,
                "native K4096 closure differs")
    if "domain_counted" in values:
        require(values["domain_counted"]["domain_clauses"] == 1773110,
                "native K4096 domain-clause count differs")
    if "domain_installed" in values:
        require(values["domain_installed"]["variables"] == 131958
                and values["domain_installed"]["clauses"] == 2021197
                and values["domain_installed"]["xors"] == 4980,
                "native K4096 installed model differs")
    if "solver_enter" in values:
        require(values["solver_enter"]["remaining_conflicts"] == 1000,
                "native search conflict cap differs")
    return {
        "schema": "n83.k4096-native-phase-audit/v1",
        "status": "PASS_PHASE_AND_QUERY_CUSTODY",
        "public_query": query_from_events(events, fixture),
        "phase_markers": len(phases),
        "last_phase": phases[-1]["phase"],
        "solver_entry_observed": "solver_enter" in names,
        "terminal_solver_phase_observed": any(name in names for name in
                                              ("solver_sat", "solver_unsat", "solver_unknown")),
        "selected_best_total_runtime": None,
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--positive-log", type=Path)
    parser.add_argument("--stderr", type=Path)
    parser.add_argument("--events", type=Path)
    parser.add_argument("--fixture", type=Path)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    if args.positive_log is not None:
        require(args.stderr is None and args.events is None and args.fixture is None,
                "positive and native audit inputs cannot be mixed")
        result = positive(args.positive_log)
    else:
        require(args.stderr is not None and args.events is not None
                and args.fixture is not None, "native audit inputs are incomplete")
        result = native(args.stderr, args.events, args.fixture)
    with args.output.open("x") as stream:
        json.dump(result, stream, indent=2, sort_keys=True)
        stream.write("\n")
    print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()

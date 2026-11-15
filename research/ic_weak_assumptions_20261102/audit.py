#!/usr/bin/env python3
"""Validate and score the coupled index-calculus obligation graph."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


REQUIRED_OBLIGATIONS = {
    "O-SOLVE",
    "O-NONGENERIC",
    "O-YIELD",
    "O-RANK",
    "O-GEOMETRY",
    "O-E2E",
}
ADMITTED_STATUSES = {"proved", "proved_internal", "measured_scaling", "not_applicable"}


def fixed_arity_gate(model: dict[str, Any]) -> dict[str, Any]:
    m = model["m"]
    a = model.get("setup_exponent_a")
    e = model.get("query_exponent_e")
    threshold_a = m / 2
    threshold_e = m / 2 - 1
    complete = isinstance(a, (int, float)) and isinstance(e, (int, float))
    return {
        "complete": complete,
        "threshold_a": threshold_a,
        "threshold_e": threshold_e,
        "passes": bool(complete and a < threshold_a and e < threshold_e),
        "total_p_exponent": max(a, e + 1) / m if complete else None,
    }


def validate(data: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if data.get("schema") != "ic-weak-assumptions/v1":
        errors.append("unexpected schema")
    vocabulary = set(data.get("status_vocabulary", []))
    obligations = data.get("obligations", [])
    ids = [item.get("id") for item in obligations]
    if set(ids) != REQUIRED_OBLIGATIONS or len(ids) != len(set(ids)):
        errors.append("obligation identifiers must be the exact six required IDs")
    for obligation in obligations:
        unknown = set(obligation.get("depends_on", [])) - REQUIRED_OBLIGATIONS
        if unknown:
            errors.append(f"{obligation.get('id')}: unknown dependencies {sorted(unknown)}")
    route_ids: set[str] = set()
    for route in data.get("routes", []):
        rid = route.get("id", "<missing>")
        if rid in route_ids:
            errors.append(f"duplicate route {rid}")
        route_ids.add(rid)
        route_obligations = route.get("obligations", {})
        if set(route_obligations) != REQUIRED_OBLIGATIONS:
            errors.append(f"{rid}: must type all six obligations")
        for oid, claim in route_obligations.items():
            if claim.get("status") not in vocabulary:
                errors.append(f"{rid}/{oid}: unknown status {claim.get('status')!r}")
            if not claim.get("basis"):
                errors.append(f"{rid}/{oid}: missing basis")
        model = route.get("cost_model", {})
        if model.get("kind") == "fixed_arity":
            if not isinstance(model.get("m"), int) or model["m"] < 2:
                errors.append(f"{rid}: fixed-arity model needs integer m >= 2")
            else:
                fixed_arity_gate(model)
    return errors


def score(data: dict[str, Any]) -> dict[str, Any]:
    routes = []
    for route in data["routes"]:
        statuses = {key: value["status"] for key, value in route["obligations"].items()}
        obligations_pass = all(status in ADMITTED_STATUSES for status in statuses.values())
        model = route["cost_model"]
        if model["kind"] == "fixed_arity":
            cost_gate = fixed_arity_gate(model)
        elif model["kind"] == "empirical_p_exponent":
            cost_gate = {
                "complete": True,
                "passes": model["measured_beta"] < model["required_below"],
                "measured_beta": model["measured_beta"],
                "required_below": model["required_below"],
            }
        else:
            cost_gate = {"complete": False, "passes": False}
        routes.append(
            {
                "id": route["id"],
                "disposition": route["disposition"],
                "statuses": statuses,
                "obligations_pass": obligations_pass,
                "cost_gate": cost_gate,
                "ready": bool(obligations_pass and cost_gate["passes"]),
            }
        )
    return {"schema": "ic-weak-assumptions-audit/v1", "routes": routes}


def markdown(report: dict[str, Any]) -> str:
    lines = [
        "| route | obligations admitted | cost complete | cost passes | ready |",
        "|---|---:|---:|---:|---:|",
    ]
    for route in report["routes"]:
        yes = lambda value: "yes" if value else "no"
        lines.append(
            f"| {route['id']} | {yes(route['obligations_pass'])} | "
            f"{yes(route['cost_gate']['complete'])} | {yes(route['cost_gate']['passes'])} | "
            f"{yes(route['ready'])} |"
        )
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", type=Path, default=Path(__file__).with_name("obligations.json"))
    parser.add_argument("--format", choices=("json", "markdown"), default="markdown")
    args = parser.parse_args()
    data = json.loads(args.input.read_text())
    errors = validate(data)
    if errors:
        for error in errors:
            print(f"error: {error}")
        return 1
    report = score(data)
    if args.format == "json":
        print(json.dumps(report, indent=2, sort_keys=True))
    else:
        print(markdown(report))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

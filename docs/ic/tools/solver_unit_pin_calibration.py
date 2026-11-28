#!/usr/bin/env python3
"""Patch `docs/ic/calibration.json` with solver/native unit keys from `ic bench` JSON.

Unlike `boundary_pin_calibration.py`, this script updates **one key per instance**
and leaves every other field of the calibration table unchanged.

usage: solver_unit_pin_calibration.py --key KEY --instance REGIME/NAME \\
         report.json [report.json ...] [--calibration PATH] [--max-spread R] [--dry]

Supported keys:

- Solver-block keys (`ns_per_conflict`, `ns_per_monomial_op`, …): median over
  reports of `(decomposition.solver.wall_ns / decomposition.solver.ops) /
  calibration.ns_per_add` for rows whose solver block matches the frozen
  configuration.

- `ns_per_hashed_tensor_step`: median of contraction-timer ratios from rows whose
  oracle is `hashed-tensor`, using
  `hashed_tensor_contraction_wall_ns` and `hashed_tensor_primitive_steps` in the
  relation-phase native counters (falling back to summing setup + relation
  `hashed_tensor_steps` when the timer fields are absent).
"""
from __future__ import annotations

import argparse
import json
import statistics
import sys
from collections import defaultdict
from pathlib import Path
from typing import Any

SOLVER_KEYS = {
    "ns_per_conflict": {"op_units": {"conflicts"}},
    "ns_per_monomial_op": {"op_units": {"monomial operations", "monomial ops"}},
    "ns_per_monomial_test": {"op_units": {"monomial tests"}},
    "ns_per_f4_muladd": {"op_units": {"F4 mul-adds", "F4 muladds"}},
    "ns_per_vector_xor_8x32": {"op_units": {"vector XORs (8×32)"}},
    "ns_per_vector_xor_16x32": {"op_units": {"vector XORs (16×32)"}},
}

DEFAULT_CAL = Path("docs/ic/calibration.json")


def load_reports(paths: list[Path]) -> list[dict[str, Any]]:
    return [json.loads(p.read_text(encoding="utf-8")) for p in paths]


def ns_per_add(report: dict[str, Any]) -> float:
    cal = report.get("calibration") or {}
    add = cal.get("ns_per_add")
    if add is None or add <= 0:
        raise ValueError(f"{report.get('instance')}: missing calibration.ns_per_add")
    return float(add)


def ratios_solver_block(report: dict[str, Any], key: str) -> list[float]:
    spec = SOLVER_KEYS.get(key)
    if not spec:
        return []
    add = ns_per_add(report)
    out: list[float] = []
    for row in report.get("rows") or []:
        solver = (row.get("decomposition") or {}).get("solver")
        if not solver:
            continue
        ops = solver.get("ops") or 0
        wall = solver.get("wall_ns") or 0
        if ops <= 0 or wall <= 0:
            continue
        unit = (solver.get("op_unit") or "").strip()
        if unit not in spec["op_units"]:
            continue
        out.append((wall / ops) / add)
    return out


def native_get(cost: dict[str, Any], name: str) -> int:
    native = cost.get("native") or {}
    return int(native.get(name) or 0)


def ratios_hashed_tensor(report: dict[str, Any]) -> list[float]:
    add = ns_per_add(report)
    out: list[float] = []
    for row in report.get("rows") or []:
        spec = row.get("spec") or {}
        if (spec.get("oracle") or "") != "hashed-tensor":
            continue
        decomp = row.get("decomposition") or {}
        rel = decomp.get("cost") or {}
        wall = native_get(rel, "hashed_tensor_contraction_wall_ns")
        steps = native_get(rel, "hashed_tensor_primitive_steps")
        if wall > 0 and steps > 0:
            out.append((wall / steps) / add)
            continue
        fb = row.get("factor_base") or {}
        setup_steps = native_get(fb.get("cost") or {}, "hashed_tensor_steps")
        rel_steps = native_get(rel, "hashed_tensor_steps")
        total_steps = setup_steps + rel_steps
        if total_steps <= 0:
            continue
        total_wall = (fb.get("cost") or {}).get("wall_ns", 0) + rel.get("wall_ns", 0)
        if total_wall <= 0:
            continue
        out.append((total_wall / total_steps) / add)
    return out


def key_ratios(reports: list[dict[str, Any]], key: str) -> list[float]:
    if key == "ns_per_hashed_tensor_step":
        ratios: list[float] = []
        for rep in reports:
            ratios.extend(ratios_hashed_tensor(rep))
        return ratios
    if key in SOLVER_KEYS:
        ratios = []
        for rep in reports:
            ratios.extend(ratios_solver_block(rep, key))
        return ratios
    raise SystemExit(f"unsupported key {key!r}")


def merge_pin(
    cal_path: Path,
    instance: str,
    key: str,
    value: float,
    *,
    dry: bool,
) -> None:
    doc = json.loads(cal_path.read_text(encoding="utf-8"))
    instances = doc.setdefault("instances", {})
    if instance not in instances:
        raise SystemExit(f"no calibration row for {instance!r}")
    row = instances[instance]
    old = row.get(key)
    row[key] = round(value, 6)
    note = doc.setdefault("solver_unit_pin_notes", {})
    note[instance] = {
        "key": key,
        "previous": old,
        "pinned": row[key],
    }
    if dry:
        print(json.dumps({instance: row}, indent=2))
        return
    cal_path.write_text(json.dumps(doc, indent=1) + "\n", encoding="utf-8")
    print(f"wrote {key}={row[key]} to {instance} in {cal_path}")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--key", required=True)
    parser.add_argument(
        "--instance",
        required=True,
        help="Table row, e.g. prime/bench-20bit",
    )
    parser.add_argument("reports", nargs="+", type=Path)
    parser.add_argument("--calibration", type=Path, default=DEFAULT_CAL)
    parser.add_argument(
        "--max-spread",
        type=float,
        default=1.25,
        help="Reject pin when max/min per-run ratios exceed this (acceptance §1)",
    )
    parser.add_argument("--dry", action="store_true")
    args = parser.parse_args()

    reports = load_reports(args.reports)
    ratios = key_ratios(reports, args.key)
    if len(ratios) < 1:
        raise SystemExit(f"no ratios extracted for {args.key} from {len(reports)} report(s)")

    spread = max(ratios) / min(ratios) if len(ratios) > 1 else 1.0
    if spread > args.max_spread:
        raise SystemExit(
            f"spread {spread:.3f} > {args.max_spread}: refusing to pin "
            f"(ratios={ratios})"
        )
    median = statistics.median(ratios)
    merge_pin(
        args.calibration,
        args.instance,
        args.key,
        median,
        dry=args.dry,
    )
    print(
        f"median ratio {median:.6f} over {len(ratios)} sample(s), spread {spread:.3f}"
    )


if __name__ == "__main__":
    main()
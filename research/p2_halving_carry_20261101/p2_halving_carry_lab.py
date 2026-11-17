#!/usr/bin/env python3
"""H111: test a first-layer p^2 halving carry as a parity oracle.

The 24 predicates and every control were frozen before this implementation.
The runner exhausts the small prime-order cohort and only invokes the
parity-guided decoder for predicates that are exact on a complete row.
"""

from __future__ import annotations

import argparse
from dataclasses import asdict
import hashlib
import json
import math
import platform
import random
import statistics
import time
from pathlib import Path
from typing import Optional, Sequence

from p2_lift_defect_cocycle_lab import (
    CURVES,
    Curve,
    GENERATOR_MULTIPLIERS,
    OperationCounter,
    affine_add_lifted,
    deterministic_seed,
    formal_tau,
    hensel_lift,
    mixed_kernel_sum,
    orbit_points,
    scientific_payload_sha256,
    section_record,
)


PREREGISTRATION_COMMIT = "65104bc"
CANONICAL_PR = "https://github.com/isogeny-labs/crypto/pull/3118"
REFERENCE_PR = "https://github.com/isogeny-labs/cryptanalysis/pull/1349"
CONTROL_CURVES = ("E101", "E127")
SECTION_CONTROLS = 8
LABEL_CONTROLS = 8
RESULT_PATH = Path(__file__).resolve().parent / "results/p2-halving-carry.json"

Point = Optional[tuple[int, int]]


def carry_for_sections(
    curve: Curve,
    half: tuple[int, int],
    target: tuple[int, int],
    half_u: int,
    target_u: int,
    counter: OperationCounter,
) -> tuple[int, dict]:
    """Return kappa for [2]s_u(H)-s_v(Q), with structural certificates."""
    lifted_half = hensel_lift(curve, half, half_u, counter)
    lifted_target = hensel_lift(curve, target, target_u, counter)
    doubled = affine_add_lifted(curve, lifted_half, lifted_half, counter)
    if doubled is None:
        raise AssertionError("lifted nonzero half doubled to identity")
    reduced_double = (doubled[0] % curve.p, doubled[1] % curve.p)
    if reduced_double != target:
        raise AssertionError("lifted doubling reduced to the wrong target")
    neg_target = (lifted_target[0], (-lifted_target[1]) % (curve.p * curve.p))
    kernel = mixed_kernel_sum(curve, doubled, neg_target, counter)
    kappa = formal_tau(curve, kernel, counter)
    certificate = {
        "half_u": half_u % curve.p,
        "target_u": target_u % curve.p,
        "reduced_double": list(reduced_double),
        "kernel_reduces_to_identity": kernel[2] % curve.p == 0,
    }
    if not certificate["kernel_reduces_to_identity"]:
        raise AssertionError("halving carry missed the reduction kernel")
    return kappa, certificate


def canonical_and_torsion_record(
    curve: Curve, target: tuple[int, int]
) -> tuple[int, dict, OperationCounter, OperationCounter]:
    candidate_counter = OperationCounter()
    control_counter = OperationCounter()
    half = curve.mul((curve.r + 1) // 2, target)
    if half is None or curve.add(half, half) != target:
        raise AssertionError("public group half certificate failed")
    kappa, certificate = carry_for_sections(
        curve, half, target, 0, 0, candidate_counter
    )

    _, half_section, half_counter = section_record(curve, half)
    _, target_section, target_counter = section_record(curve, target)
    control_counter.merge(half_counter)
    control_counter.merge(target_counter)
    torsion_kappa, torsion_certificate = carry_for_sections(
        curve,
        half,
        target,
        half_section["torsion_u"],
        target_section["torsion_u"],
        control_counter,
    )
    if torsion_kappa != 0:
        raise AssertionError("homomorphic torsion section has nonzero carry")
    return kappa, {
        "half": list(half),
        "canonical": certificate,
        "torsion_half_u": half_section["torsion_u"],
        "torsion_target_u": target_section["torsion_u"],
        "torsion_carry": torsion_kappa,
        "torsion": torsion_certificate,
    }, candidate_counter, control_counter


def predicate_specs() -> list[dict]:
    return [
        {
            "id": f"offset_{offset:+d}:{kind}:complement_{complement}",
            "offset": offset,
            "kind": kind,
            "complement": complement,
        }
        for offset in (-1, 0, 1)
        for kind in ("zero", "integer_lsb", "centered", "legendre")
        for complement in (0, 1)
    ]


PREDICATES = predicate_specs()


def predicate_bit(value: int, prime: int, spec: dict) -> int:
    z = (value + spec["offset"]) % prime
    if spec["kind"] == "zero":
        bit = int(z == 0)
    elif spec["kind"] == "integer_lsb":
        bit = z & 1
    elif spec["kind"] == "centered":
        bit = int(z > (prime - 1) // 2)
    elif spec["kind"] == "legendre":
        bit = int(z != 0 and pow(z, (prime - 1) // 2, prime) == prime - 1)
    else:
        raise ValueError(f"unknown predicate kind: {spec['kind']}")
    return bit ^ spec["complement"]


def score_predicates(values: Sequence[int], labels: Sequence[int], prime: int) -> dict:
    if len(values) != len(labels) or not values:
        raise ValueError("predicate word/label mismatch")
    scores = []
    for spec in PREDICATES:
        matches = sum(
            predicate_bit(value, prime, spec) == label
            for value, label in zip(values, labels)
        )
        scores.append({
            **spec,
            "matches": matches,
            "targets": len(labels),
            "accuracy": matches / len(labels),
            "exact": matches == len(labels),
        })
    best = max(item["matches"] for item in scores)
    return {
        "predicate_count": len(scores),
        "scores": scores,
        "best_matches": best,
        "best_accuracy": best / len(labels),
        "best_predicate_ids": [item["id"] for item in scores if item["matches"] == best],
        "exact_predicate_ids": [item["id"] for item in scores if item["exact"]],
    }


def spec_by_id(predicate_id: str) -> dict:
    matches = [item for item in PREDICATES if item["id"] == predicate_id]
    if len(matches) != 1:
        raise ValueError(f"unknown predicate id: {predicate_id}")
    return matches[0]


def carry_value(curve: Curve, target: tuple[int, int], counter: OperationCounter) -> int:
    half = curve.mul((curve.r + 1) // 2, target)
    if half is None:
        raise AssertionError("nonzero target has identity half")
    return carry_for_sections(curve, half, target, 0, 0, counter)[0]


def decode_target(
    curve: Curve,
    generator: tuple[int, int],
    target: tuple[int, int],
    predicate_id: str,
) -> dict:
    spec = spec_by_id(predicate_id)
    current: Point = target
    recovered = 0
    position = 0
    counter = OperationCounter()
    trace = []
    while current is not None and position <= curve.r.bit_length():
        value = carry_value(curve, current, counter)
        bit = predicate_bit(value, curve.p, spec)
        trace.append({"position": position, "kappa": value, "bit": bit})
        if bit:
            current = curve.add(current, curve.neg(generator))
        current = curve.mul((curve.r + 1) // 2, current)
        recovered |= bit << position
        position += 1
    verified = current is None and recovered < curve.r and curve.mul(recovered, generator) == target
    return {
        "recovered_scalar": recovered,
        "queries": position,
        "verified": verified,
        "trace": trace,
        "cost": asdict(counter),
        "base_group_subtractions": sum(item["bit"] for item in trace),
        "base_group_halving_scalar_multiplications": position,
        "ec_validations": 1,
    }


def decode_row(
    curve: Curve,
    generator: tuple[int, int],
    points: Sequence[Point],
    exact_predicate_ids: Sequence[str],
) -> dict | None:
    if not exact_predicate_ids:
        return None
    formulas = []
    for predicate_id in exact_predicate_ids:
        records = []
        for scalar, target in enumerate(points[1:], start=1):
            if target is None:
                raise AssertionError
            decoded = decode_target(curve, generator, target, predicate_id)
            records.append({"target_scalar": scalar, **decoded})
        formulas.append({
            "predicate_id": predicate_id,
            "records": records,
            "recovered": sum(item["verified"] for item in records),
            "targets": curve.r - 1,
            "maximum_queries": max(item["queries"] for item in records),
            "all_verified": all(item["verified"] for item in records),
        })
    return {"formulas": formulas, "all_formulas_verified": all(x["all_verified"] for x in formulas)}


def evaluate_generator(curve: Curve, generator: tuple[int, int], multiplier: int) -> dict:
    points = orbit_points(curve, generator)
    values = []
    records = []
    candidate_counter = OperationCounter()
    control_counter = OperationCounter()
    started = time.perf_counter_ns()
    for scalar, target in enumerate(points[1:], start=1):
        if target is None:
            raise AssertionError
        value, structural, candidate_cost, control_cost = canonical_and_torsion_record(curve, target)
        values.append(value)
        records.append({"scalar": scalar, "point": list(target), "kappa": value, **structural})
        candidate_counter.merge(candidate_cost)
        control_counter.merge(control_cost)
    elapsed = time.perf_counter_ns() - started
    labels = [scalar & 1 for scalar in range(1, curve.r)]
    scoring = score_predicates(values, labels, curve.p)
    decoder = decode_row(curve, generator, points, scoring["exact_predicate_ids"])
    return {
        "generator_multiplier": multiplier,
        "generator": list(generator),
        "word": values,
        "word_sha256": hashlib.sha256(json.dumps(values, separators=(",", ":")).encode()).hexdigest(),
        "target_records": records,
        "predicate_scoring": scoring,
        "decoder": decoder,
        "cost": {
            "candidate_query": asdict(candidate_counter),
            "torsion_structural_control": asdict(control_counter),
            "combined_measured": {
                key: asdict(candidate_counter)[key] + asdict(control_counter)[key]
                for key in asdict(candidate_counter)
            },
            "evaluation_elapsed_ns": elapsed,
            "base_group_halving_scalar_multiplications": curve.r - 1,
            "predicate_evaluations": (curve.r - 1) * len(PREDICATES),
            "online_predicate_evaluations_per_query": 1,
            "legendre_modular_multiplications_upper_per_query": 2 * curve.p.bit_length(),
            "stored_output_elements": len(values),
            "advice_bits": 0,
            "generic_bsgs_group_operations_reference": math.ceil(math.sqrt(curve.r)),
            "generic_rho_ideal_group_operations_reference": math.sqrt(math.pi * curve.r / 2),
        },
    }


def random_section_word(curve: Curve, generator: tuple[int, int], replicate: int) -> list[int]:
    points = orbit_points(curve, generator)
    values = []
    for scalar, target in enumerate(points[1:], start=1):
        if target is None:
            raise AssertionError
        half = curve.mul((curve.r + 1) // 2, target)
        if half is None:
            raise AssertionError
        half_u = deterministic_seed("H111", curve.name, "half-section", replicate, scalar, *half) % curve.p
        target_u = deterministic_seed("H111", curve.name, "target-section", replicate, scalar, *target) % curve.p
        value, _ = carry_for_sections(
            curve, half, target, half_u, target_u, OperationCounter()
        )
        values.append(value)
    return values


def matched_parity_labels(curve: Curve, replicate: int) -> list[int]:
    labels: list[Optional[int]] = [None] * curve.r
    labels[0] = 0
    rng = random.Random(deterministic_seed("H111", curve.name, "matched-labels", replicate))
    for scalar in range(1, curve.r):
        partner = curve.r - scalar
        if labels[scalar] is None:
            bit = rng.randrange(2)
            labels[scalar] = bit
            labels[partner] = 1 - bit
    if any(value is None for value in labels):
        raise AssertionError("matched label construction incomplete")
    return [int(value) for value in labels[1:]]


def matched_controls(curve: Curve, generator: tuple[int, int], canonical_word: Sequence[int]) -> dict:
    parity_labels = [scalar & 1 for scalar in range(1, curve.r)]
    sections = []
    labels = []
    for replicate in range(SECTION_CONTROLS):
        word = random_section_word(curve, generator, replicate)
        sections.append({
            "replicate": replicate,
            "word_sha256": hashlib.sha256(json.dumps(word, separators=(",", ":")).encode()).hexdigest(),
            "predicate_scoring": score_predicates(word, parity_labels, curve.p),
        })
    for replicate in range(LABEL_CONTROLS):
        control_labels = matched_parity_labels(curve, replicate)
        labels.append({
            "replicate": replicate,
            "labels_sha256": hashlib.sha256(json.dumps(control_labels, separators=(",", ":")).encode()).hexdigest(),
            "negation_complementary": all(
                control_labels[scalar - 1] + control_labels[curve.r - scalar - 1] == 1
                for scalar in range(1, curve.r)
            ),
            "predicate_scoring": score_predicates(canonical_word, control_labels, curve.p),
        })
    return {"random_sections": sections, "matched_parity_labels": labels}


def coordinate_isomorphism_control(curve: Curve) -> dict:
    lam, p = 2, curve.p
    transformed = Curve(
        p,
        curve.a * pow(lam, 4, p) % p,
        curve.b * pow(lam, 6, p) % p,
        curve.r,
        f"{curve.name}-iso-{lam}",
    )
    base = curve.first_point()
    generator = (base[0] * pow(lam, 2, p) % p, base[1] * pow(lam, 3, p) % p)
    if not transformed.on_curve(generator):
        raise AssertionError("coordinate isomorphism missed transformed curve")
    row = evaluate_generator(transformed, generator, 1)
    return {
        "lambda": lam,
        "transformed_parameters": {"p": p, "a": transformed.a, "b": transformed.b, "r": transformed.r},
        "word_sha256": row["word_sha256"],
        "predicate_scoring": row["predicate_scoring"],
        "torsion_zero_for_all_targets": all(x["torsion_carry"] == 0 for x in row["target_records"]),
        "cost": row["cost"],
    }


def run_experiment(
    curves: Sequence[Curve] = CURVES,
    multipliers: Sequence[int] = GENERATOR_MULTIPLIERS,
    control_replicates: bool = True,
) -> dict:
    started = time.perf_counter_ns()
    curve_results = []
    controls = {}
    isomorphisms = {}
    rows = []
    for curve in curves:
        if curve.r == curve.p:
            raise AssertionError("H111 excludes anomalous r=p curves")
        base = curve.first_point()
        generator_rows = []
        for multiplier in multipliers:
            generator = curve.mul(multiplier, base)
            if generator is None:
                raise AssertionError("generator relabeling reached identity")
            row = evaluate_generator(curve, generator, multiplier)
            generator_rows.append(row)
            rows.append(row)
        curve_results.append({
            "curve": curve.name,
            "parameters": {"p": curve.p, "a": curve.a, "b": curve.b, "r": curve.r},
            "canonical_generator": list(base),
            "generators": generator_rows,
        })
        if control_replicates and curve.name in CONTROL_CURVES:
            controls[curve.name] = matched_controls(curve, base, generator_rows[0]["word"])
            isomorphisms[curve.name] = coordinate_isomorphism_control(curve)

    exact_rows = [row for row in rows if row["predicate_scoring"]["exact_predicate_ids"]]
    best = [row["predicate_scoring"]["best_accuracy"] for row in rows]
    decoded = [row["decoder"] for row in exact_rows if row["decoder"] is not None]
    result = {
        "experiment": "H111 first-layer p2 halving carry parity oracle",
        "date": "2026-11-01",
        "preregistration_commit": PREREGISTRATION_COMMIT,
        "pull_requests": {"canonical": CANONICAL_PR, "reference": REFERENCE_PR},
        "protocol": {
            "generator_multipliers": list(multipliers),
            "predicate_family": PREDICATES,
            "section_controls_per_primary_curve": SECTION_CONTROLS if control_replicates else 0,
            "matched_label_controls_per_primary_curve": LABEL_CONTROLS if control_replicates else 0,
            "index_calculus_used": False,
            "pollard_rho_used_as_candidate": False,
        },
        "curves": curve_results,
        "matched_controls": controls,
        "coordinate_isomorphism_controls": isomorphisms,
        "summary": {
            "curve_rows": len(rows),
            "targets": sum(curve.r - 1 for curve in curves) * len(multipliers),
            "predicate_count": len(PREDICATES),
            "best_accuracy_minimum": min(best),
            "best_accuracy_median": statistics.median(best),
            "best_accuracy_maximum": max(best),
            "rows_with_exact_predicate": len(exact_rows),
            "rows_decoded": len(decoded),
            "decoded_targets": sum(
                formula["recovered"]
                for decoder in decoded
                for formula in decoder["formulas"]
            ),
            "all_structural_checks_passed": True,
            "all_torsion_carries_zero": all(
                record["torsion_carry"] == 0
                for curve in curve_results
                for row in curve["generators"]
                for record in row["target_records"]
            ),
        },
        "environment": {
            "python": platform.python_version(),
            "platform": platform.platform(),
            "processor": platform.processor(),
        },
        "total_elapsed_ns": time.perf_counter_ns() - started,
    }
    result["scientific_payload_sha256"] = scientific_payload_sha256(result)
    return result


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=RESULT_PATH)
    parser.add_argument("--quick", action="store_true", help="one curve/generator without the 32-control panel")
    args = parser.parse_args()
    result = run_experiment(
        curves=CURVES[:1] if args.quick else CURVES,
        multipliers=GENERATOR_MULTIPLIERS[:1] if args.quick else GENERATOR_MULTIPLIERS,
        control_replicates=not args.quick,
    )
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps({
        "output": str(args.output),
        "scientific_payload_sha256": result["scientific_payload_sha256"],
        **result["summary"],
    }, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()

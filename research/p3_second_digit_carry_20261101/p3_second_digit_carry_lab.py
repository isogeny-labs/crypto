#!/usr/bin/env python3
"""H112: test the second digit of a canonical p^3 halving carry.

The 144 predicates and controls were frozen before implementation. Low-digit
only predicates from H111 are deliberately absent.
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
import sys
import time
from pathlib import Path
from typing import Optional, Sequence

SCRIPT_DIR = Path(__file__).resolve().parent
H111_DIR = SCRIPT_DIR.parent / "p2_halving_carry_20261101"
sys.path.insert(0, str(H111_DIR))

from p2_lift_defect_cocycle_lab import (  # noqa: E402
    CURVES,
    Curve,
    GENERATOR_MULTIPLIERS,
    ModRing,
    OperationCounter,
    deterministic_seed,
    orbit_points,
    scientific_payload_sha256,
)


PREREGISTRATION_COMMIT = "8f5cc6c"
CANONICAL_PR = "https://github.com/isogeny-labs/crypto/pull/3121"
REFERENCE_PR = "https://github.com/isogeny-labs/cryptanalysis/pull/1351"
CONTROL_CURVES = ("E101", "E127")
SECTION_CONTROLS = 8
LABEL_CONTROLS = 8
RESULT_PATH = SCRIPT_DIR / "p3-second-digit-carry.json"

Point = Optional[tuple[int, int]]
Jacobian = tuple[int, int, int]


def on_lifted_curve_p3(curve: Curve, point: tuple[int, int], counter: OperationCounter) -> bool:
    ring = ModRing(curve.p**3, counter)
    x, y = point
    y2 = ring.mul(y, y)
    x2 = ring.mul(x, x)
    x3 = ring.mul(x2, x)
    rhs = ring.add(ring.add(x3, ring.mul(curve.a, x)), curve.b)
    return y2 == rhs


def hensel_lift_p3(
    curve: Curve, point: tuple[int, int], u1: int, u2: int, counter: OperationCounter
) -> tuple[int, int]:
    p, n = curve.p, curve.p**3
    x, y = point
    lifted_x = (x + p * (u1 % p) + p * p * (u2 % p)) % n
    rhs = (lifted_x**3 + curve.a * lifted_x + curve.b) % n
    lifted_y = y
    modulus = p
    for _ in range(2):
        discrepancy = rhs - lifted_y * lifted_y
        if discrepancy % modulus:
            raise AssertionError("Hensel y step lacks required divisibility")
        counter.modular_inversions += 1
        delta = (discrepancy // modulus) * pow(2 * lifted_y, -1, p) % p
        counter.modular_multiplications += 2
        counter.modular_additions += 1
        lifted_y = (lifted_y + modulus * delta) % (modulus * p)
        modulus *= p
    lifted = (lifted_x, lifted_y % n)
    counter.lift_constructions += 1
    if not on_lifted_curve_p3(curve, lifted, counter):
        raise AssertionError("p^3 Hensel lift missed lifted curve")
    if (lifted[0] % p, lifted[1] % p) != point:
        raise AssertionError("p^3 Hensel lift has wrong reduction")
    return lifted


def affine_add_p3(curve: Curve, left: Point, right: Point, counter: OperationCounter) -> Point:
    if left is None:
        return right
    if right is None:
        return left
    n, p = curve.p**3, curve.p
    ring = ModRing(n, counter)
    x1, y1 = left
    x2, y2 = right
    if left == right:
        denominator = 2 * y1 % n
        if denominator % p == 0:
            raise ZeroDivisionError("nonunit p^3 doubling denominator")
        numerator = ring.add(ring.mul(3, ring.mul(x1, x1)), curve.a)
        counter.affine_doublings += 1
    else:
        denominator = ring.sub(x2, x1)
        if denominator % p == 0:
            raise ZeroDivisionError("nonunit p^3 addition denominator")
        numerator = ring.sub(y2, y1)
        counter.affine_additions += 1
    slope = ring.mul(numerator, ring.inv(denominator))
    x3 = ring.sub(ring.sub(ring.mul(slope, slope), x1), x2)
    y3 = ring.sub(ring.mul(slope, ring.sub(x1, x3)), y1)
    out = (x3, y3)
    if not on_lifted_curve_p3(curve, out, counter):
        raise AssertionError("p^3 affine addition left lifted curve")
    return out


def affine_mul_p3(
    curve: Curve, scalar: int, point: tuple[int, int], counter: OperationCounter
) -> Point:
    counter.scalar_multiplications += 1
    out: Point = None
    addend: Point = point
    while scalar:
        if scalar & 1:
            out = affine_add_p3(curve, out, addend, counter)
        scalar >>= 1
        if scalar:
            addend = affine_add_p3(curve, addend, addend, counter)
    return out


def on_projective_p3(curve: Curve, point: Jacobian, counter: OperationCounter) -> bool:
    ring = ModRing(curve.p**3, counter)
    x, y, z = point
    y2 = ring.mul(y, y)
    x2 = ring.mul(x, x)
    x3 = ring.mul(x2, x)
    z2 = ring.mul(z, z)
    z4 = ring.mul(z2, z2)
    z6 = ring.mul(z4, z2)
    rhs = ring.add(ring.add(x3, ring.mul(curve.a, ring.mul(x, z4))), ring.mul(curve.b, z6))
    return y2 == rhs


def mixed_kernel_sum_p3(
    curve: Curve, left: tuple[int, int], right: tuple[int, int], counter: OperationCounter
) -> Jacobian:
    ring = ModRing(curve.p**3, counter)
    x1, y1 = left
    x2, y2 = right
    h = ring.sub(x2, x1)
    two_h = ring.mul(2, h)
    i = ring.mul(two_h, two_h)
    j = ring.mul(h, i)
    rr = ring.mul(2, ring.sub(y2, y1))
    v = ring.mul(x1, i)
    x3 = ring.sub(ring.sub(ring.mul(rr, rr), j), ring.mul(2, v))
    y3 = ring.sub(ring.mul(rr, ring.sub(v, x3)), ring.mul(2, ring.mul(y1, j)))
    z3 = two_h
    out = (x3, y3, z3)
    counter.kernel_mixed_additions += 1
    if not on_projective_p3(curve, out, counter):
        raise AssertionError("p^3 mixed kernel sum left projective curve")
    return out


def formal_word_p3(curve: Curve, kernel: Jacobian, counter: OperationCounter) -> int:
    p, n = curve.p, curve.p**3
    ring = ModRing(n, counter)
    x, y, z = kernel
    if z % p != 0 or y % p == 0:
        raise AssertionError("invalid p^3 reduction-kernel chart")
    t = ring.mul((-ring.mul(x, z)) % n, ring.inv(y))
    if t % p:
        raise AssertionError("p^3 formal word lacks p divisibility")
    counter.formal_extractions += 1
    return t // p % (p * p)


def r_multiple_word_p3(
    curve: Curve, lifted: tuple[int, int], split: int, counter: OperationCounter
) -> int:
    left = affine_mul_p3(curve, split, lifted, counter)
    right = affine_mul_p3(curve, curve.r - split, lifted, counter)
    if left is None or right is None:
        raise AssertionError("proper p^3 split reached identity")
    if curve.add((left[0] % curve.p, left[1] % curve.p), (right[0] % curve.p, right[1] % curve.p)) is not None:
        raise AssertionError("p^3 split reductions do not cancel")
    return formal_word_p3(curve, mixed_kernel_sum_p3(curve, left, right, counter), counter)


def section_word_p3(
    curve: Curve, point: tuple[int, int], u1: int, u2: int, split: int, counter: OperationCounter
) -> int:
    return r_multiple_word_p3(curve, hensel_lift_p3(curve, point, u1, u2, counter), split, counter)


def torsion_correction_p3(curve: Curve, point: tuple[int, int]) -> tuple[tuple[int, int], dict, OperationCounter]:
    counter = OperationCounter()
    split = (curve.r - 1) // 2
    independent_split = max(1, curve.r // 3)
    word00 = section_word_p3(curve, point, 0, 0, split, counter)
    word10 = section_word_p3(curve, point, 1, 0, split, counter)
    low_slope = (word10 - word00) % curve.p
    if low_slope == 0:
        raise AssertionError("zero first correction slope at p^3")
    u1 = -(word00 % curve.p) * pow(low_slope, -1, curve.p) % curve.p
    word_u10 = section_word_p3(curve, point, u1, 0, split, counter)
    if word_u10 % curve.p:
        raise AssertionError("first correction did not kill low digit")
    word_u11 = section_word_p3(curve, point, u1, 1, split, counter)
    delta = (word_u11 - word_u10) % (curve.p * curve.p)
    if delta % curve.p:
        raise AssertionError("second correction changed the low digit")
    high_slope = delta // curve.p % curve.p
    if high_slope == 0:
        raise AssertionError("zero second correction slope at p^3")
    u2 = -(word_u10 // curve.p) * pow(high_slope, -1, curve.p) % curve.p
    final_word = section_word_p3(curve, point, u1, u2, split, counter)
    independent_word = section_word_p3(curve, point, u1, u2, independent_split, counter)
    if final_word != 0 or independent_word != 0:
        raise AssertionError("two-digit torsion correction did not kill [r]")
    return (u1, u2), {
        "word_00": word00,
        "word_10": word10,
        "low_slope": low_slope,
        "word_u1_0": word_u10,
        "word_u1_1": word_u11,
        "high_slope": high_slope,
        "u1": u1,
        "u2": u2,
        "final_word": final_word,
        "independent_word": independent_word,
    }, counter


def carry_for_sections_p3(
    curve: Curve,
    half: tuple[int, int],
    target: tuple[int, int],
    half_u: tuple[int, int],
    target_u: tuple[int, int],
    counter: OperationCounter,
) -> tuple[tuple[int, int], dict]:
    lifted_half = hensel_lift_p3(curve, half, *half_u, counter)
    lifted_target = hensel_lift_p3(curve, target, *target_u, counter)
    doubled = affine_add_p3(curve, lifted_half, lifted_half, counter)
    if doubled is None or (doubled[0] % curve.p, doubled[1] % curve.p) != target:
        raise AssertionError("p^3 doubled half reduced incorrectly")
    neg_target = (lifted_target[0], -lifted_target[1] % (curve.p**3))
    kernel = mixed_kernel_sum_p3(curve, doubled, neg_target, counter)
    word = formal_word_p3(curve, kernel, counter)
    digits = (word % curve.p, word // curve.p)
    if word != digits[0] + curve.p * digits[1]:
        raise AssertionError("p^3 digit recombination failed")
    return digits, {
        "half_u": list(half_u),
        "target_u": list(target_u),
        "word": word,
        "digits_recombine": True,
        "kernel_reduces_to_identity": kernel[2] % curve.p == 0,
    }


def base_bit(z: int, prime: int, kind: str) -> int:
    z %= prime
    if kind == "zero":
        return int(z == 0)
    if kind == "integer_lsb":
        return z & 1
    if kind == "centered":
        return int(z > (prime - 1) // 2)
    if kind == "legendre":
        return int(z != 0 and pow(z, (prime - 1) // 2, prime) == prime - 1)
    raise ValueError(kind)


def predicate_specs() -> list[dict]:
    kinds = ("zero", "integer_lsb", "centered", "legendre")
    depth = [
        {"id": f"depth:{channel}:offset_{offset:+d}:{kind}:complement_{complement}",
         "family": "depth", "channel": channel, "offset": offset, "kind": kind, "complement": complement}
        for channel in ("high", "sum", "difference")
        for offset in (-1, 0, 1)
        for kind in kinds
        for complement in (0, 1)
    ]
    cross = [
        {"id": f"cross:low_{low_offset:+d}:high_{high_offset:+d}:{kind}:complement_{complement}",
         "family": "cross", "low_offset": low_offset, "high_offset": high_offset,
         "kind": kind, "complement": complement}
        for low_offset in (-1, 0, 1)
        for high_offset in (-1, 0, 1)
        for kind in kinds
        for complement in (0, 1)
    ]
    return depth + cross


PREDICATES = predicate_specs()


def predicate_bit(digits: tuple[int, int], prime: int, spec: dict) -> int:
    low, high = digits
    if spec["family"] == "depth":
        channel = {"high": high, "sum": low + high, "difference": high - low}[spec["channel"]]
        bit = base_bit(channel + spec["offset"], prime, spec["kind"])
    elif spec["family"] == "cross":
        bit = base_bit(low + spec["low_offset"], prime, spec["kind"])
        bit ^= base_bit(high + spec["high_offset"], prime, spec["kind"])
    else:
        raise ValueError("unknown predicate family")
    return bit ^ spec["complement"]


def score_predicates(words: Sequence[tuple[int, int]], labels: Sequence[int], prime: int) -> dict:
    scores = []
    for spec in PREDICATES:
        matches = sum(predicate_bit(word, prime, spec) == label for word, label in zip(words, labels))
        scores.append({**spec, "matches": matches, "targets": len(labels), "accuracy": matches / len(labels), "exact": matches == len(labels)})
    best = max(item["matches"] for item in scores)
    return {
        "predicate_count": len(scores),
        "scores": scores,
        "best_matches": best,
        "best_accuracy": best / len(labels),
        "best_predicate_ids": [x["id"] for x in scores if x["matches"] == best],
        "exact_predicate_ids": [x["id"] for x in scores if x["exact"]],
    }


def spec_by_id(predicate_id: str) -> dict:
    matches = [item for item in PREDICATES if item["id"] == predicate_id]
    if len(matches) != 1:
        raise ValueError(predicate_id)
    return matches[0]


def carry_value_p3(curve: Curve, target: tuple[int, int], counter: OperationCounter) -> tuple[int, int]:
    half = curve.mul((curve.r + 1) // 2, target)
    if half is None:
        raise AssertionError
    return carry_for_sections_p3(curve, half, target, (0, 0), (0, 0), counter)[0]


def decode_target(curve: Curve, generator: tuple[int, int], target: tuple[int, int], predicate_id: str) -> dict:
    spec = spec_by_id(predicate_id)
    current: Point = target
    recovered = 0
    position = 0
    counter = OperationCounter()
    while current is not None and position <= curve.r.bit_length():
        digits = carry_value_p3(curve, current, counter)
        bit = predicate_bit(digits, curve.p, spec)
        if bit:
            current = curve.add(current, curve.neg(generator))
        current = curve.mul((curve.r + 1) // 2, current)
        recovered |= bit << position
        position += 1
    return {
        "recovered_scalar": recovered,
        "queries": position,
        "verified": current is None and recovered < curve.r and curve.mul(recovered, generator) == target,
        "cost": asdict(counter),
        "ec_validations": 1,
    }


def decode_row(curve: Curve, generator: tuple[int, int], points: Sequence[Point], exact_ids: Sequence[str]) -> dict | None:
    if not exact_ids:
        return None
    formulas = []
    for predicate_id in exact_ids:
        records = []
        for scalar, target in enumerate(points[1:], start=1):
            if target is None:
                raise AssertionError
            records.append({"target_scalar": scalar, **decode_target(curve, generator, target, predicate_id)})
        formulas.append({"predicate_id": predicate_id, "records": records, "recovered": sum(x["verified"] for x in records), "targets": curve.r - 1, "all_verified": all(x["verified"] for x in records)})
    return {"formulas": formulas, "all_formulas_verified": all(x["all_verified"] for x in formulas)}


def evaluate_generator(
    curve: Curve,
    generator: tuple[int, int],
    multiplier: int,
    torsion_cache: dict[tuple[int, int], tuple[tuple[int, int], dict]],
    torsion_setup_counter: OperationCounter,
) -> dict:
    points = orbit_points(curve, generator)
    words = []
    records = []
    candidate_counter = OperationCounter()
    torsion_check_counter = OperationCounter()
    started = time.perf_counter_ns()
    for scalar, target in enumerate(points[1:], start=1):
        if target is None:
            raise AssertionError
        half = curve.mul((curve.r + 1) // 2, target)
        if half is None or curve.add(half, half) != target:
            raise AssertionError("public p^3 half failed")
        digits, candidate = carry_for_sections_p3(curve, half, target, (0, 0), (0, 0), candidate_counter)
        for point in (half, target):
            if point not in torsion_cache:
                correction, proof, cost = torsion_correction_p3(curve, point)
                torsion_cache[point] = (correction, proof)
                torsion_setup_counter.merge(cost)
        half_correction, half_proof = torsion_cache[half]
        target_correction, target_proof = torsion_cache[target]
        torsion_digits, torsion = carry_for_sections_p3(curve, half, target, half_correction, target_correction, torsion_check_counter)
        if torsion_digits != (0, 0):
            raise AssertionError("two-layer torsion carry is nonzero")
        words.append(digits)
        records.append({
            "scalar": scalar, "point": list(target), "half": list(half), "digits": list(digits),
            "candidate": candidate, "torsion_half_correction": list(half_correction),
            "torsion_target_correction": list(target_correction), "torsion_half_proof": half_proof,
            "torsion_target_proof": target_proof, "torsion_digits": list(torsion_digits), "torsion": torsion,
        })
    scoring = score_predicates(words, [k & 1 for k in range(1, curve.r)], curve.p)
    return {
        "generator_multiplier": multiplier,
        "generator": list(generator),
        "word": [list(x) for x in words],
        "word_sha256": hashlib.sha256(json.dumps(words, separators=(",", ":")).encode()).hexdigest(),
        "target_records": records,
        "predicate_scoring": scoring,
        "decoder": decode_row(curve, generator, points, scoring["exact_predicate_ids"]),
        "cost": {
            "candidate_query": asdict(candidate_counter),
            "torsion_carry_checks": asdict(torsion_check_counter),
            "evaluation_elapsed_ns": time.perf_counter_ns() - started,
            "base_group_halving_scalar_multiplications": curve.r - 1,
            "predicate_evaluations": (curve.r - 1) * len(PREDICATES),
            "online_predicate_evaluations_per_query": 1,
            "stored_output_elements": 2 * len(words),
            "advice_bits": 0,
            "generic_bsgs_group_operations_reference": math.ceil(math.sqrt(curve.r)),
            "generic_rho_ideal_group_operations_reference": math.sqrt(math.pi * curve.r / 2),
        },
    }


def random_section_word(curve: Curve, generator: tuple[int, int], replicate: int) -> list[tuple[int, int]]:
    values = []
    for scalar, target in enumerate(orbit_points(curve, generator)[1:], start=1):
        if target is None:
            raise AssertionError
        half = curve.mul((curve.r + 1) // 2, target)
        if half is None:
            raise AssertionError
        hu = tuple(deterministic_seed("H112", curve.name, "half", replicate, digit, scalar, *half) % curve.p for digit in (1, 2))
        tu = tuple(deterministic_seed("H112", curve.name, "target", replicate, digit, scalar, *target) % curve.p for digit in (1, 2))
        values.append(carry_for_sections_p3(curve, half, target, hu, tu, OperationCounter())[0])
    return values


def matched_labels(curve: Curve, replicate: int) -> list[int]:
    labels: list[Optional[int]] = [None] * curve.r
    labels[0] = 0
    rng = random.Random(deterministic_seed("H112", curve.name, "matched-labels", replicate))
    for scalar in range(1, curve.r):
        partner = curve.r - scalar
        if labels[scalar] is None:
            bit = rng.randrange(2)
            labels[scalar], labels[partner] = bit, 1 - bit
    return [int(x) for x in labels[1:]]


def matched_controls(curve: Curve, generator: tuple[int, int], canonical_word: Sequence[tuple[int, int]]) -> dict:
    parity = [k & 1 for k in range(1, curve.r)]
    sections = []
    labels = []
    for replicate in range(SECTION_CONTROLS):
        word = random_section_word(curve, generator, replicate)
        sections.append({"replicate": replicate, "word_sha256": hashlib.sha256(json.dumps(word, separators=(",", ":")).encode()).hexdigest(), "predicate_scoring": score_predicates(word, parity, curve.p)})
    for replicate in range(LABEL_CONTROLS):
        control = matched_labels(curve, replicate)
        labels.append({"replicate": replicate, "labels_sha256": hashlib.sha256(json.dumps(control, separators=(",", ":")).encode()).hexdigest(), "negation_complementary": all(control[k - 1] + control[curve.r - k - 1] == 1 for k in range(1, curve.r)), "predicate_scoring": score_predicates(canonical_word, control, curve.p)})
    return {"random_sections": sections, "matched_parity_labels": labels}


def coordinate_isomorphism_control(curve: Curve) -> dict:
    lam, p = 2, curve.p
    transformed = Curve(p, curve.a * pow(lam, 4, p) % p, curve.b * pow(lam, 6, p) % p, curve.r, f"{curve.name}-iso-{lam}")
    base = curve.first_point()
    generator = (base[0] * pow(lam, 2, p) % p, base[1] * pow(lam, 3, p) % p)
    cache: dict[tuple[int, int], tuple[tuple[int, int], dict]] = {}
    setup = OperationCounter()
    row = evaluate_generator(transformed, generator, 1, cache, setup)
    return {"lambda": lam, "transformed_parameters": {"p": p, "a": transformed.a, "b": transformed.b, "r": transformed.r}, "word_sha256": row["word_sha256"], "predicate_scoring": row["predicate_scoring"], "torsion_zero_for_all_targets": all(x["torsion_digits"] == [0, 0] for x in row["target_records"]), "cost": row["cost"], "torsion_setup_cost": asdict(setup)}


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
            raise AssertionError("H112 excludes anomalous r=p curves")
        base = curve.first_point()
        cache: dict[tuple[int, int], tuple[tuple[int, int], dict]] = {}
        setup = OperationCounter()
        generator_rows = []
        for multiplier in multipliers:
            generator = curve.mul(multiplier, base)
            if generator is None:
                raise AssertionError
            row = evaluate_generator(curve, generator, multiplier, cache, setup)
            generator_rows.append(row)
            rows.append(row)
        curve_results.append({"curve": curve.name, "parameters": {"p": curve.p, "a": curve.a, "b": curve.b, "r": curve.r}, "canonical_generator": list(base), "torsion_setup_cost": asdict(setup), "torsion_cache_points": len(cache), "generators": generator_rows})
        if control_replicates and curve.name in CONTROL_CURVES:
            controls[curve.name] = matched_controls(curve, base, [tuple(x) for x in generator_rows[0]["word"]])
            isomorphisms[curve.name] = coordinate_isomorphism_control(curve)

    exact_rows = [row for row in rows if row["predicate_scoring"]["exact_predicate_ids"]]
    best = [row["predicate_scoring"]["best_accuracy"] for row in rows]
    decoded = [row["decoder"] for row in exact_rows if row["decoder"] is not None]
    result = {
        "experiment": "H112 p3 second-digit halving carry parity oracle",
        "date": "2026-11-01",
        "preregistration_commit": PREREGISTRATION_COMMIT,
        "pull_requests": {"canonical": CANONICAL_PR, "reference": REFERENCE_PR},
        "protocol": {"generator_multipliers": list(multipliers), "predicate_family": PREDICATES, "section_controls_per_primary_curve": SECTION_CONTROLS if control_replicates else 0, "matched_label_controls_per_primary_curve": LABEL_CONTROLS if control_replicates else 0, "low_digit_only_predicates": 0, "index_calculus_used": False, "pollard_rho_used_as_candidate": False},
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
            "decoded_targets": sum(f["recovered"] for d in decoded for f in d["formulas"]),
            "all_structural_checks_passed": True,
            "all_torsion_carries_zero": all(record["torsion_digits"] == [0, 0] for c in curve_results for row in c["generators"] for record in row["target_records"]),
        },
        "environment": {"python": platform.python_version(), "platform": platform.platform(), "processor": platform.processor()},
        "total_elapsed_ns": time.perf_counter_ns() - started,
    }
    result["scientific_payload_sha256"] = scientific_payload_sha256(result)
    return result


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=RESULT_PATH)
    parser.add_argument("--quick", action="store_true")
    args = parser.parse_args()
    result = run_experiment(CURVES[:1] if args.quick else CURVES, GENERATOR_MULTIPLIERS[:1] if args.quick else GENERATOR_MULTIPLIERS, not args.quick)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"output": str(args.output), "scientific_payload_sha256": result["scientific_payload_sha256"], **result["summary"]}, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
"""Independent fail-closed replay for the public RR9 full-eight F4 screen.

This module deliberately contains its own finite-field, polynomial, Gaussian
elimination, and elliptic-curve code.  It does not import the Rust producer or
any project algebra implementation.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import stat
import sys
from pathlib import Path
from typing import Any, Iterable, Mapping, Sequence


P = 2_147_483_647
CURVE_A = 847_514_846
CURVE_B = 2_087_444_724
GROUP_ORDER = 2_147_446_831
BASE_X = (0, 1, 2, 3, 5, 6, 8, 9)
RESIDUALS = (
    (1_911_579_686, 548_820_096),
    (625_037_715, 2_100_627_251),
    (1_923_055_775, 307_731_458),
)

PRODUCER_SCHEMA = "prime31.rr9.full8-f4-screen.v2"
INPUT_SCHEMA = "prime31.rr9.full8-input-system.v1"
LINEAR_SCHEMA = "prime31.rr9.full8-linear-extraction.v1"
BASIS_SCHEMA = "prime31.rr9.full8-basis-evidence.v1"
UNIT_SCHEMA = "prime31.rr9.full8-unit-identity.v1"
OUTPUT_SCHEMA = "prime31.rr9.full8-independent-replay.v1"
INPUT_HASH_ENCODING = (
    "sha256(compact serde_json UTF-8 of emitted input_system; "
    "object keys lexicographic)"
)
NEGATIVE_STATUS_SCHEMA = "prime31.rr9.full8-negative-certificate.v1"
EVIDENCE_LIMITS_SCHEMA = "prime31.rr9.full8-evidence-limits.v1"
RESOURCE_REFUSAL_SCHEMA = "prime31.rr9.full8-resource-refusal.v1"

N_VARIABLES = 7
RADIX_LEVELS = 4
RADIX_CUTOFF_EXPONENT = 1 << RADIX_LEVELS
FROZEN_SOLVING_DEGREE = 9
ZERO_EXPONENT = (0,) * N_VARIABLES
MAX_INPUT_BYTES = 128 * 1024 * 1024
MAX_BASIS_POLYNOMIALS = 100_000
MAX_BASIS_TERMS = 1_000_000
MAX_CERTIFICATE_TERMS = 1_000_000
MAX_EXPONENT = 255
MAX_BASIS_TOTAL_DEGREE = 9
MAX_CERTIFICATE_TOTAL_DEGREE = 7
MAX_CERTIFICATE_PRODUCT_DEGREE = 9
MAX_PRODUCT_TERMS = 2_000_000
MAX_INTERNAL_PROVENANCE_TERMS = 3_000_000
MAX_INTERNAL_RECOMPOSITION_PRODUCT_WORK = 2_000_000

SparsePolynomial = dict[tuple[int, ...], int]
Point = tuple[int, int] | None


class VerificationError(ValueError):
    """A malformed or incorrectly bound producer artifact."""


class ReplayError(VerificationError):
    """A well-formed artifact whose mathematical replay fails."""


class ArtifactTooLarge(VerificationError):
    """A producer artifact exceeded the bounded reader's byte cap."""

    def __init__(self, observed_bytes: int, *, exact: bool) -> None:
        self.observed_bytes = observed_bytes
        self.exact = exact
        qualifier = "" if exact else "at least "
        super().__init__(
            f"producer artifact is {qualifier}{observed_bytes} bytes; "
            f"limit is {MAX_INPUT_BYTES}"
        )


class ArtifactCustodyError(VerificationError):
    """The producer artifact was not one stable, regular, no-follow file."""


def _require(condition: bool, message: str) -> None:
    if not condition:
        raise VerificationError(message)


def _replay_require(condition: bool, message: str) -> None:
    if not condition:
        raise ReplayError(message)


def _is_int(value: Any) -> bool:
    return type(value) is int


def _field_element(value: Any, context: str, *, nonzero: bool = False) -> int:
    _require(_is_int(value), f"{context} is not an integer")
    _require(0 <= value < P, f"{context} is not a canonical F_p element")
    if nonzero:
        _require(value != 0, f"{context} is zero in a sparse encoding")
    return value


def _integer(
    value: Any,
    context: str,
    minimum: int = 0,
    maximum: int | None = None,
) -> int:
    _require(_is_int(value), f"{context} is not an integer")
    _require(value >= minimum, f"{context} is below {minimum}")
    if maximum is not None:
        _require(value <= maximum, f"{context} exceeds {maximum}")
    return value


def _exact_integer_binding(value: Any, expected: int, context: str) -> int:
    observed = _integer(value, context, maximum=(1 << 64) - 1)
    _require(observed == expected, f"{context} differs")
    return observed


def _exact_public_point(
    raw: Any, expected: tuple[int, int], context: str
) -> tuple[int, int]:
    _require(isinstance(raw, list) and len(raw) == 2,
             f"{context} is not a two-coordinate list")
    x_coord = _field_element(raw[0], f"{context}[0]")
    y_coord = _field_element(raw[1], f"{context}[1]")
    point = (x_coord, y_coord)
    _require(point == expected, f"{context} differs")
    return point


def _exact_keys(value: Any, keys: set[str], context: str) -> Mapping[str, Any]:
    _require(isinstance(value, dict), f"{context} is not an object")
    actual = set(value)
    _require(actual == keys, f"{context} keys differ: {sorted(actual ^ keys)}")
    return value


def _required_keys(value: Any, keys: set[str], context: str) -> Mapping[str, Any]:
    _require(isinstance(value, dict), f"{context} is not an object")
    missing = keys - set(value)
    _require(not missing, f"{context} is missing keys: {sorted(missing)}")
    return value


def _reject_duplicate_keys(pairs: Iterable[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise VerificationError(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def load_json_strict(raw: bytes) -> Mapping[str, Any]:
    """Parse one JSON object while rejecting duplicate keys and non-JSON numbers."""

    _require(len(raw) <= MAX_INPUT_BYTES, "producer artifact exceeds size limit")

    def reject_constant(value: str) -> None:
        raise VerificationError(f"non-JSON numeric constant: {value}")

    def reject_float(value: str) -> None:
        raise VerificationError(f"floating-point JSON number is not allowed: {value}")

    try:
        result = json.loads(
            raw,
            object_pairs_hook=_reject_duplicate_keys,
            parse_constant=reject_constant,
            parse_float=reject_float,
        )
    except VerificationError:
        raise
    except (UnicodeDecodeError, RecursionError, ValueError, MemoryError) as exc:
        raise VerificationError(f"invalid JSON: {exc}") from exc
    _require(isinstance(result, dict), "producer artifact root is not an object")
    return result


def read_artifact_bounded(path: Path) -> bytes:
    """Read at most the frozen artifact cap plus one detection byte."""

    flags = (
        os.O_RDONLY
        | getattr(os, "O_CLOEXEC", 0)
        | getattr(os, "O_NONBLOCK", 0)
    )
    nofollow = getattr(os, "O_NOFOLLOW", 0)
    flags |= nofollow
    before_open = os.lstat(path)
    if stat.S_ISLNK(before_open.st_mode):
        raise ArtifactCustodyError("producer artifact path is a symlink")
    descriptor = os.open(path, flags)
    try:
        before = os.fstat(descriptor)
        if not stat.S_ISREG(before.st_mode):
            raise ArtifactCustodyError("producer artifact is not a regular file")
        after_open = os.lstat(path)
        if (
            before_open.st_dev != before.st_dev
            or before_open.st_ino != before.st_ino
            or after_open.st_dev != before.st_dev
            or after_open.st_ino != before.st_ino
            or stat.S_ISLNK(after_open.st_mode)
        ):
            raise ArtifactCustodyError(
                "producer artifact path changed while it was opened"
            )
        if before.st_size > MAX_INPUT_BYTES:
            raise ArtifactTooLarge(before.st_size, exact=True)

        chunks: list[bytes] = []
        observed = 0
        while observed <= MAX_INPUT_BYTES:
            chunk = os.read(
                descriptor,
                min(1024 * 1024, MAX_INPUT_BYTES + 1 - observed),
            )
            if not chunk:
                break
            chunks.append(chunk)
            observed += len(chunk)
        after = os.fstat(descriptor)
        custody_fields = (
            "st_dev",
            "st_ino",
            "st_size",
            "st_mtime_ns",
            "st_ctime_ns",
        )
        if any(getattr(before, field) != getattr(after, field)
               for field in custody_fields):
            raise ArtifactCustodyError(
                "producer artifact changed while it was being read"
            )
        if observed != before.st_size:
            raise ArtifactCustodyError(
                "producer artifact byte count differs from its stable file size"
            )
        if observed > MAX_INPUT_BYTES:
            raise ArtifactTooLarge(observed, exact=True)
        return b"".join(chunks)
    finally:
        os.close(descriptor)


def _compact_json_bytes(value: Any) -> bytes:
    # serde_json::Value uses recursively lexicographic object keys without its
    # preserve_order feature.  sort_keys gives identical compact UTF-8 bytes.
    return json.dumps(
        value,
        sort_keys=True,
        ensure_ascii=False,
        separators=(",", ":"),
    ).encode()


def _sha256(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def _univariate_multiply(left: Sequence[int], right: Sequence[int]) -> list[int]:
    result = [0] * (len(left) + len(right) - 1)
    for i, x in enumerate(left):
        for j, y in enumerate(right):
            result[i + j] = (result[i + j] + x * y) % P
    return result


def h8_coefficients() -> list[int]:
    result = [1]
    for x_coord in BASE_X:
        result = _univariate_multiply(result, [(-x_coord) % P, 1])
    return result


def _add_sparse_term(
    polynomial: SparsePolynomial,
    exponent: tuple[int, ...],
    coefficient: int,
) -> None:
    coefficient %= P
    if not coefficient:
        return
    updated = (polynomial.get(exponent, 0) + coefficient) % P
    if updated:
        polynomial[exponent] = updated
    else:
        polynomial.pop(exponent, None)


def original_quadrics(
    residual: tuple[int, int], h8: Sequence[int] | None = None
) -> list[SparsePolynomial]:
    """Reconstruct the eight coefficient quadrics from the public equation."""

    if h8 is None:
        h8 = h8_coefficients()
    _require(len(h8) == 9, "H8 does not have degree eight")
    r_coord, s_coord = residual
    d_coefficients = ((r_coord * r_coord + CURVE_A) % P, r_coord, 1)
    quadrics: list[SparsePolynomial] = [{} for _ in range(8)]

    def variable_exponent(index: int | None) -> tuple[int, ...]:
        exponent = [0] * N_VARIABLES
        if index is not None:
            exponent[index] = 1
        return tuple(exponent)

    def product_exponent(
        left: int | None, right: int | None
    ) -> tuple[int, ...]:
        exponent = list(variable_exponent(left))
        if right is not None:
            exponent[right] += 1
        return tuple(exponent)

    # A=a0+...+a3*X^3 and V=v0+v1*X+v2*X^2+X^3.
    v_index = lambda index: None if index == 3 else index + 4
    for i in range(4):
        for j in range(4):
            for k, d_coefficient in enumerate(d_coefficients):
                if i + j + k < 8:
                    _add_sparse_term(
                        quadrics[i + j + k],
                        product_exponent(v_index(i), v_index(j)),
                        d_coefficient,
                    )
            _add_sparse_term(
                quadrics[i + j], product_exponent(i, j), r_coord
            )
            if i + j + 1 < 8:
                _add_sparse_term(
                    quadrics[i + j + 1], product_exponent(i, j), -1
                )
            _add_sparse_term(
                quadrics[i + j],
                product_exponent(i, v_index(j)),
                -2 * s_coord,
            )

    for index in range(8):
        _add_sparse_term(quadrics[index], ZERO_EXPONENT, -h8[index])
    return quadrics


def _encoded_terms(
    polynomial: SparsePolynomial, *, descending: bool
) -> list[dict[str, Any]]:
    return [
        {"exponents": list(exponent), "coefficient": polynomial[exponent]}
        for exponent in sorted(polynomial, reverse=descending)
    ]


def expected_input_system(case: int) -> dict[str, Any]:
    _require(_is_int(case) and 0 <= case < len(RESIDUALS), "invalid case")
    h8 = h8_coefficients()
    quadrics = original_quadrics(RESIDUALS[case], h8)
    # Field insertion order is part of the compact serde/Python hash contract.
    return {
        "schema": INPUT_SCHEMA,
        "p": P,
        "case": case,
        "public_residual": list(RESIDUALS[case]),
        "base_x": list(BASE_X),
        "h8_coefficients": h8,
        "input_quadrics": [
            _encoded_terms(polynomial, descending=True) for polynomial in quadrics
        ],
    }


def input_system_sha256(case: int) -> str:
    return _sha256(_compact_json_bytes(expected_input_system(case)))


def expected_radix_embedding(open_top_pairs_omitted: int = 0) -> dict[str, Any]:
    """Return the frozen ordinary-to-radix embedding admission receipt."""

    _integer(
        open_top_pairs_omitted,
        "open_top_pairs_omitted",
        maximum=(1 << 64) - 1,
    )
    blocks = [
        [variable * RADIX_LEVELS + level for level in range(RADIX_LEVELS)]
        for variable in range(N_VARIABLES)
    ]
    level_weights = [1 << level for level in range(RADIX_LEVELS)]
    carry_rules = []
    for variable in range(N_VARIABLES):
        for level in range(RADIX_LEVELS):
            tower_index = variable * RADIX_LEVELS + level
            has_next = level + 1 < RADIX_LEVELS
            carry_rules.append(
                {
                    "tower_index": tower_index,
                    "ordinary_variable": variable,
                    "level": level,
                    "weight": 1 << level,
                    "square_maps_to_tower_index": (
                        tower_index + 1 if has_next else None
                    ),
                    "top_square_maps_to_zero": not has_next,
                }
            )
    open_top_mask = sum(
        1 << (variable * RADIX_LEVELS + RADIX_LEVELS - 1)
        for variable in range(N_VARIABLES)
    )
    return {
        "schema": "prime31.rr9.full8-radix-embedding.v1",
        "ordinary_variables": N_VARIABLES,
        "tower_variables": N_VARIABLES * RADIX_LEVELS,
        "free_variables": 0,
        "levels_per_variable": RADIX_LEVELS,
        "blocks": blocks,
        "level_weights": level_weights,
        "tower_weights": level_weights * N_VARIABLES,
        "carry_rules": carry_rules,
        "artificial_top_relation_exponent": RADIX_CUTOFF_EXPONENT,
        "frozen_max_weighted_degree": FROZEN_SOLVING_DEGREE,
        "requested_open_top_pair_omission_mask": open_top_mask,
        "open_top_pair_omission_mask": open_top_mask,
        "open_top_rules_selected": N_VARIABLES,
        "open_top_pairs_omitted": open_top_pairs_omitted,
        "open_top_reductions": 0,
        "open_top_receipt_valid": True,
        "exact_block_rule_weight_layout_valid": True,
        "input_embedding_roundtrip": True,
        "all_reported_step_degrees_bounded": True,
        "all_basis_monomials_decoded_below_top_relation": True,
        "top_rule_reductions_excluded_by_degree_bound": True,
        "admissible": True,
    }


def _verify_radix_embedding(raw: Any) -> None:
    _require(isinstance(raw, dict), "radix_embedding is not an object")
    omitted = _integer(
        raw.get("open_top_pairs_omitted"),
        "radix_embedding.open_top_pairs_omitted",
        maximum=(1 << 64) - 1,
    )
    expected = expected_radix_embedding(omitted)
    _require(
        raw == expected and _compact_json_bytes(raw) == _compact_json_bytes(expected),
        "radix/open-top admission evidence differs",
    )


def _radix_open_top_summary(radix_embedding: Mapping[str, Any]) -> dict[str, Any]:
    return {
        "schema": radix_embedding["schema"],
        "frozen_max_weighted_degree": radix_embedding[
            "frozen_max_weighted_degree"
        ],
        "requested_open_top_pair_omission_mask": radix_embedding[
            "requested_open_top_pair_omission_mask"
        ],
        "open_top_pair_omission_mask": radix_embedding[
            "open_top_pair_omission_mask"
        ],
        "open_top_rules_selected": radix_embedding["open_top_rules_selected"],
        "open_top_pairs_omitted": radix_embedding["open_top_pairs_omitted"],
        "open_top_reductions": radix_embedding["open_top_reductions"],
        "open_top_receipt_valid": radix_embedding["open_top_receipt_valid"],
    }


def _parse_exponent(
    raw: Any, context: str, *, max_total_degree: int
) -> tuple[int, ...]:
    _require(isinstance(raw, list) and len(raw) == N_VARIABLES,
             f"{context} must contain seven exponents")
    exponent = []
    for index, value in enumerate(raw):
        _require(_is_int(value), f"{context}[{index}] is not an integer")
        _require(0 <= value <= MAX_EXPONENT,
                 f"{context}[{index}] exceeds the exponent bound")
        exponent.append(value)
    result = tuple(exponent)
    _require(sum(result) <= max_total_degree,
             f"{context} exceeds the total-degree bound")
    return result


def _parse_sparse_polynomial(
    raw: Any,
    context: str,
    *,
    allow_empty: bool,
    max_total_degree: int,
) -> SparsePolynomial:
    _require(isinstance(raw, list), f"{context} is not a term list")
    if not allow_empty:
        _require(bool(raw), f"{context} is an empty polynomial")
    polynomial: SparsePolynomial = {}
    previous: tuple[int, ...] | None = None
    for index, raw_term in enumerate(raw):
        term = _exact_keys(
            raw_term, {"exponents", "coefficient"}, f"{context}[{index}]"
        )
        exponent = _parse_exponent(
            term["exponents"],
            f"{context}[{index}].exponents",
            max_total_degree=max_total_degree,
        )
        coefficient = _field_element(
            term["coefficient"], f"{context}[{index}].coefficient", nonzero=True
        )
        if previous is not None:
            _require(exponent < previous,
                     f"{context} terms are not strict descending lexicographic")
        previous = exponent
        _require(exponent not in polynomial, f"{context} repeats a monomial")
        polynomial[exponent] = coefficient
    return polynomial


def _evaluate_sparse(polynomial: SparsePolynomial, values: Sequence[int]) -> int:
    _require(len(values) == N_VARIABLES, "wrong evaluation-vector length")
    result = 0
    for exponent, coefficient in polynomial.items():
        term = coefficient
        for value, power in zip(values, exponent):
            term = term * pow(value, power, P) % P
        result = (result + term) % P
    return result


def _basis_evidence(raw: Any) -> tuple[list[SparsePolynomial], list[list[int]]]:
    evidence = _exact_keys(
        raw,
        {
            "schema",
            "polynomial_count",
            "term_count",
            "canonical_polynomials",
            "blocker",
        },
        "basis_evidence",
    )
    _require(evidence["schema"] == BASIS_SCHEMA, "wrong basis-evidence schema")
    _require(evidence["blocker"] is None,
             "completed basis evidence reports a decoding blocker")
    polynomial_count = _integer(
        evidence["polynomial_count"], "basis_evidence.polynomial_count"
    )
    term_count = _integer(evidence["term_count"], "basis_evidence.term_count")
    _require(polynomial_count <= MAX_BASIS_POLYNOMIALS,
             "basis polynomial count exceeds limit")
    _require(term_count <= MAX_BASIS_TERMS, "basis term count exceeds limit")
    encoded = evidence["canonical_polynomials"]
    _require(isinstance(encoded, list), "basis polynomial encoding is not a list")
    _require(len(encoded) == polynomial_count, "basis polynomial count mismatch")

    basis: list[SparsePolynomial] = []
    observed_terms = 0
    for index, raw_polynomial in enumerate(encoded):
        polynomial = _parse_sparse_polynomial(
            raw_polynomial,
            f"basis_evidence.canonical_polynomials[{index}]",
            allow_empty=False,
            max_total_degree=MAX_BASIS_TOTAL_DEGREE,
        )
        observed_terms += len(polynomial)
        _require(observed_terms <= MAX_BASIS_TERMS, "basis term limit exceeded")
        basis.append(polynomial)
    _require(observed_terms == term_count, "basis term count mismatch")

    linear_rows: list[list[int]] = []
    for polynomial in basis:
        degree = max(sum(exponent) for exponent in polynomial)
        if degree > 1:
            continue
        row = [0] * (N_VARIABLES + 1)
        for exponent, coefficient in polynomial.items():
            if exponent == ZERO_EXPONENT:
                row[-1] = coefficient
                continue
            _require(sum(exponent) == 1, "malformed degree-one basis monomial")
            variable = exponent.index(1)
            row[variable] = coefficient
        linear_rows.append(row)
    return basis, linear_rows


def _parse_rows(raw: Any, context: str) -> list[list[int]]:
    _require(isinstance(raw, list), f"{context} is not a list")
    _require(len(raw) <= MAX_BASIS_POLYNOMIALS, f"{context} has too many rows")
    result = []
    for row_index, raw_row in enumerate(raw):
        _require(
            isinstance(raw_row, list) and len(raw_row) == N_VARIABLES + 1,
            f"{context}[{row_index}] is not an eight-entry row",
        )
        result.append(
            [
                _field_element(value, f"{context}[{row_index}][{column}]")
                for column, value in enumerate(raw_row)
            ]
        )
    return result


def _linear_rref(rows: Sequence[Sequence[int]]) -> tuple[list[list[int]], list[int]]:
    """RREF the seven coefficient columns, carrying the constant column."""

    result = [list(row) for row in rows]
    pivot_row = 0
    pivot_columns: list[int] = []
    for column in range(N_VARIABLES):
        pivot = next(
            (index for index in range(pivot_row, len(result))
             if result[index][column] != 0),
            None,
        )
        if pivot is None:
            continue
        result[pivot_row], result[pivot] = result[pivot], result[pivot_row]
        inverse = pow(result[pivot_row][column], P - 2, P)
        for index in range(column, N_VARIABLES + 1):
            result[pivot_row][index] = result[pivot_row][index] * inverse % P
        for row_index in range(len(result)):
            if row_index == pivot_row:
                continue
            factor = result[row_index][column]
            if factor:
                for index in range(column, N_VARIABLES + 1):
                    result[row_index][index] = (
                        result[row_index][index]
                        - factor * result[pivot_row][index]
                    ) % P
        pivot_columns.append(column)
        pivot_row += 1
        if pivot_row == len(result):
            break
    return result, pivot_columns


def _linear_evidence(
    raw: Any, basis_linear_rows: Sequence[Sequence[int]]
) -> list[int] | None:
    evidence = _exact_keys(
        raw,
        {
            "schema",
            "raw_rows",
            "rref_rows",
            "pivot_columns",
            "coefficient_rank",
            "augmented_rank",
            "unique_solution",
            "recovered_coefficients",
            "blocker",
        },
        "linear_extraction",
    )
    _require(evidence["schema"] == LINEAR_SCHEMA, "wrong linear-evidence schema")
    raw_rows = _parse_rows(evidence["raw_rows"], "linear_extraction.raw_rows")
    _require(raw_rows == [list(row) for row in basis_linear_rows],
             "linear raw rows do not match serialized basis")
    expected_rref, expected_pivots = _linear_rref(raw_rows)
    reported_rref = _parse_rows(
        evidence["rref_rows"], "linear_extraction.rref_rows"
    )
    _require(reported_rref == expected_rref, "reported RREF differs from replay")

    pivots = evidence["pivot_columns"]
    _require(isinstance(pivots, list), "pivot_columns is not a list")
    _require(all(_is_int(value) for value in pivots),
             "pivot_columns contains a non-integer")
    _require(pivots == expected_pivots, "pivot columns differ from replay")
    coefficient_rank = len(expected_pivots)
    inconsistent_rows = [
        row for row in expected_rref
        if all(value == 0 for value in row[:N_VARIABLES]) and row[-1] != 0
    ]
    augmented_rank = coefficient_rank + bool(inconsistent_rows)
    reported_coefficient_rank = _integer(
        evidence["coefficient_rank"],
        "linear_extraction.coefficient_rank",
        maximum=N_VARIABLES,
    )
    reported_augmented_rank = _integer(
        evidence["augmented_rank"],
        "linear_extraction.augmented_rank",
        maximum=N_VARIABLES + 1,
    )
    _require(reported_coefficient_rank == coefficient_rank,
             "coefficient rank differs from replay")
    _require(reported_augmented_rank == augmented_rank,
             "augmented rank differs from replay")
    unique = coefficient_rank == N_VARIABLES and augmented_rank == coefficient_rank
    _require(type(evidence["unique_solution"]) is bool,
             "unique_solution is not boolean")
    _require(evidence["unique_solution"] == unique,
             "unique_solution differs from replay")

    recovered: list[int] | None = None
    if unique:
        recovered = [0] * N_VARIABLES
        for row_index, column in enumerate(expected_pivots):
            recovered[column] = (-expected_rref[row_index][-1]) % P
        reported = evidence["recovered_coefficients"]
        _require(isinstance(reported, list) and len(reported) == N_VARIABLES,
                 "linear recovered_coefficients has wrong shape")
        canonical = [
            _field_element(value, f"linear recovered_coefficients[{index}]")
            for index, value in enumerate(reported)
        ]
        _require(canonical == recovered,
                 "linear recovered coefficients differ from replay")
        _require(evidence["blocker"] is None,
                 "unique linear extraction reports a blocker")
    else:
        _require(evidence["recovered_coefficients"] is None,
                 "non-unique linear extraction reports coefficients")
        _require(isinstance(evidence["blocker"], str) and evidence["blocker"],
                 "non-unique linear extraction lacks a blocker")
    return recovered


def _direct_m(coefficients: Sequence[int], residual: tuple[int, int]) -> list[int]:
    a_coefficients = list(coefficients[:4])
    v_coefficients = list(coefficients[4:]) + [1]
    r_coord, s_coord = residual
    d_coefficients = [(r_coord * r_coord + CURVE_A) % P, r_coord, 1]
    dvv = _univariate_multiply(
        d_coefficients,
        _univariate_multiply(v_coefficients, v_coefficients),
    )
    x_minus_r_aa = _univariate_multiply(
        [(-r_coord) % P, 1],
        _univariate_multiply(a_coefficients, a_coefficients),
    )
    av = _univariate_multiply(a_coefficients, v_coefficients)
    result = []
    for index in range(9):
        value = dvv[index]
        if index < len(x_minus_r_aa):
            value -= x_minus_r_aa[index]
        if index < len(av):
            value -= 2 * s_coord * av[index]
        result.append(value % P)
    return result


def _on_curve(point: Point) -> bool:
    if point is None:
        return True
    x_coord, y_coord = point
    return (
        y_coord * y_coord
        - x_coord * x_coord * x_coord
        - CURVE_A * x_coord
        - CURVE_B
    ) % P == 0


def _add_points(left: Point, right: Point) -> Point:
    if left is None:
        return right
    if right is None:
        return left
    x_coord, y_coord = left
    u_coord, v_coord = right
    if x_coord == u_coord and (y_coord + v_coord) % P == 0:
        return None
    if x_coord == u_coord:
        _replay_require(y_coord == v_coord, "invalid equal-x curve points")
        _replay_require(y_coord != 0, "invalid doubling denominator")
        slope = (3 * x_coord * x_coord + CURVE_A) * pow(2 * y_coord, -1, P) % P
    else:
        slope = (v_coord - y_coord) * pow(u_coord - x_coord, -1, P) % P
    result_x = (slope * slope - x_coord - u_coord) % P
    result_y = (slope * (x_coord - result_x) - y_coord) % P
    result = (result_x, result_y)
    _replay_require(_on_curve(result), "curve addition left the curve")
    return result


def _signed_point_replay(
    coefficients: Sequence[int], residual: tuple[int, int]
) -> list[list[int]]:
    _replay_require(_on_curve(residual), "public residual is not on the curve")
    a_coefficients = coefficients[:4]
    v_coefficients = list(coefficients[4:]) + [1]
    r_coord, s_coord = residual
    selected: list[tuple[int, int]] = []
    for x_coord in BASE_X:
        rhs = (x_coord**3 + CURVE_A * x_coord + CURVE_B) % P
        y_coord = pow(rhs, (P + 1) // 4, P)
        _replay_require(y_coord != 0 and y_coord * y_coord % P == rhs,
                        "factor-base ordinate is not a nonzero square")
        a_value = sum(
            coefficient * pow(x_coord, index, P)
            for index, coefficient in enumerate(a_coefficients)
        ) % P
        v_value = sum(
            coefficient * pow(x_coord, index, P)
            for index, coefficient in enumerate(v_coefficients)
        ) % P
        candidates = []
        for candidate_y in (y_coord, (-y_coord) % P):
            relation = (
                (x_coord - r_coord) * a_value
                + (candidate_y + s_coord) * v_value
            ) % P
            if relation == 0:
                candidates.append((x_coord, candidate_y))
        _replay_require(len(candidates) == 1,
                        "coefficient chart did not select exactly one sign")
        selected.append(candidates[0])

    total: Point = None
    for point in selected:
        _replay_require(_on_curve(point), "selected point is not on the curve")
        total = _add_points(total, point)
    _replay_require(total == residual,
                    "signed full-point sum differs from public residual")
    return [[x_coord, y_coord] for x_coord, y_coord in selected]


def _multiply_add_sparse(
    accumulator: SparsePolynomial,
    left: SparsePolynomial,
    right: SparsePolynomial,
) -> int:
    peak = len(accumulator)
    # Match the producer's BTreeMap replay order so the independently observed
    # transient accumulator peak is exactly comparable.
    for left_exponent in sorted(left):
        left_coefficient = left[left_exponent]
        for right_exponent in sorted(right):
            right_coefficient = right[right_exponent]
            exponent = tuple(
                x_value + y_value
                for x_value, y_value in zip(left_exponent, right_exponent)
            )
            _require(max(exponent, default=0) <= MAX_EXPONENT,
                     "unit-certificate product exponent exceeds bound")
            _require(sum(exponent) <= MAX_CERTIFICATE_PRODUCT_DEGREE,
                     "unit-certificate product degree exceeds bound")
            _add_sparse_term(
                accumulator,
                exponent,
                left_coefficient * right_coefficient,
            )
            _require(len(accumulator) <= MAX_PRODUCT_TERMS,
                     "unit-certificate expansion exceeds term bound")
            peak = max(peak, len(accumulator))
    return peak


def _certificate_product_work(
    multipliers: Sequence[Mapping[Any, Any]],
    quadrics: Sequence[Mapping[Any, Any]],
) -> int:
    _require(len(multipliers) == len(quadrics),
             "certificate product-work generator count differs")
    work = 0
    for multiplier, quadric in zip(multipliers, quadrics):
        work += len(multiplier) * len(quadric)
        _require(
            work <= MAX_INTERNAL_RECOMPOSITION_PRODUCT_WORK,
            "unit-certificate internal recomposition product-work limit exceeded",
        )
    return work


def verify_unit_identity(
    raw_certificate: Any,
    *,
    case: int,
    input_sha256: str,
    quadrics: Sequence[SparsePolynomial],
) -> tuple[int, int, int]:
    """Replay an explicit sum_i multiplier_i*q_i = 1 certificate."""

    certificate = _exact_keys(
        raw_certificate,
        {
            "schema",
            "case",
            "public_residual",
            "input_system_sha256",
            "multipliers",
        },
        "negative_unit_identity",
    )
    _require(certificate["schema"] == UNIT_SCHEMA,
             "wrong unit-identity schema")
    _require(_is_int(certificate["case"]) and certificate["case"] == case,
             "unit identity binds the wrong case")
    _exact_public_point(
        certificate["public_residual"],
        RESIDUALS[case],
        "negative_unit_identity.public_residual",
    )
    _require(
        isinstance(certificate["input_system_sha256"], str)
        and certificate["input_system_sha256"] == input_sha256,
             "unit identity binds the wrong input system")
    raw_multipliers = certificate["multipliers"]
    _require(isinstance(raw_multipliers, list) and len(raw_multipliers) == 8,
             "unit identity must contain eight multipliers")
    _require(len(quadrics) == 8, "unit identity received the wrong quadrics")
    multipliers = []
    term_count = 0
    for index, raw_polynomial in enumerate(raw_multipliers):
        polynomial = _parse_sparse_polynomial(
            raw_polynomial,
            f"negative_unit_identity.multipliers[{index}]",
            allow_empty=True,
            max_total_degree=MAX_CERTIFICATE_TOTAL_DEGREE,
        )
        term_count += len(polynomial)
        _require(term_count <= MAX_CERTIFICATE_TERMS,
                 "unit identity exceeds term bound")
        multipliers.append(polynomial)

    product_work = _certificate_product_work(multipliers, quadrics)
    accumulated: SparsePolynomial = {}
    accumulator_peak = 0
    for multiplier, quadric in zip(multipliers, quadrics):
        accumulator_peak = max(
            accumulator_peak,
            _multiply_add_sparse(accumulated, multiplier, quadric),
        )
    _replay_require(accumulated == {ZERO_EXPONENT: 1},
                    "unit identity does not expand to exactly one")
    return term_count, accumulator_peak, product_work


def _canonical_coefficients(raw: Any, context: str) -> list[int]:
    _require(isinstance(raw, list) and len(raw) == N_VARIABLES,
             f"{context} must have seven entries")
    return [
        _field_element(value, f"{context}[{index}]")
        for index, value in enumerate(raw)
    ]


def _solver_completion_checks(document: Mapping[str, Any]) -> None:
    for key in ("complete_basis", "inconsistent", "timed_out", "oversize"):
        _require(type(document[key]) is bool, f"{key} is not boolean")
    _require(document["complete_basis"] is True, "basis is incomplete")
    _require(document["timed_out"] is False, "producer timed out")
    _require(document["oversize"] is False, "producer exceeded a size cap")
    pairs_above = _integer(document["pairs_above_bound"], "pairs_above_bound")
    _require(pairs_above == 0,
             "producer left pairs above the degree bound")
    _require(document["staircase_at_stop"] is None,
             "producer stopped with an incomplete staircase")


def _negative_status(document: Mapping[str, Any], has_identity: bool) -> None:
    status = document.get("negative_certificate")
    status = _exact_keys(
        status,
        {
            "schema",
            "available",
            "replayed",
            "internal_trace_available",
            "internal_trace_kept_rows",
            "blocker",
        },
        "negative_certificate",
    )
    _require(status["schema"] == NEGATIVE_STATUS_SCHEMA,
             "wrong negative-certificate status schema")
    _require(type(status["available"]) is bool, "negative available is not boolean")
    _require(type(status["replayed"]) is bool, "negative replayed is not boolean")
    _require(type(status["internal_trace_available"]) is bool,
             "internal_trace_available is not boolean")
    if status["internal_trace_available"]:
        _integer(status["internal_trace_kept_rows"], "internal_trace_kept_rows")
    else:
        _require(status["internal_trace_kept_rows"] is None,
                 "unavailable internal trace reports kept rows")
    _require(status["available"] == has_identity,
             "negative availability disagrees with certificate presence")
    if has_identity:
        _require(status["replayed"] is True,
                 "producer did not replay its supplied unit identity")
        _require(status["blocker"] is None,
                 "available unit identity reports a blocker")
    else:
        _require(status["replayed"] is False,
                 "missing unit identity is marked replayed")
        _require(isinstance(status["blocker"], str) and status["blocker"],
                 "missing unit identity lacks a blocker")
    if "negative_unit_identity_replayed" in document:
        legacy_replayed = document["negative_unit_identity_replayed"]
        _require(type(legacy_replayed) is bool,
                 "legacy negative replay flag is not boolean")
        _require(legacy_replayed == has_identity,
                 "legacy negative replay flag disagrees with certificate")


def _validate_internal_observation(
    value: int, kind: Any, cap: int, context: str
) -> None:
    _require(isinstance(kind, str), f"{context} kind is not a string")
    if value == 0:
        _require(kind == "not_observed",
                 f"{context} zero requires not_observed")
    elif kind == "exact":
        _require(value > cap, f"{context} exact count does not exceed its cap")
    elif kind == "lower_bound":
        _require(value == cap + 1,
                 f"{context} lower bound must be cap plus one")
    else:
        raise VerificationError(f"{context} nonzero count has no observation kind")


def _parse_evidence_limits(raw: Any) -> dict[str, Any]:
    limits = _exact_keys(
        raw,
        {
            "schema",
            "max_final_json_bytes",
            "max_basis_polynomials",
            "max_basis_terms",
            "max_certificate_terms",
            "max_certificate_product_accumulator_terms",
            "max_internal_provenance_terms",
            "max_internal_recomposition_product_work",
            "basis_polynomials_observed",
            "basis_terms_observed",
            "certificate_terms_observed",
            "certificate_product_accumulator_peak",
            "internal_provenance_terms_observed_lower_bound",
            "internal_provenance_terms_observation_kind",
            "internal_recomposition_product_work_observed_lower_bound",
            "internal_recomposition_product_work_observation_kind",
            "attempted_final_pretty_json_bytes",
            "basis_within_limits",
            "certificate_within_limits",
            "certificate_product_accumulator_within_limit",
            "internal_provenance_within_limit",
            "internal_recomposition_product_work_within_limit",
            "all_pre_serialization_limits_satisfied",
            "final_json_size_checked_before_write",
            "final_json_within_limit",
        },
        "evidence_limits",
    )
    expected_constants = {
        "max_final_json_bytes": MAX_INPUT_BYTES,
        "max_basis_polynomials": MAX_BASIS_POLYNOMIALS,
        "max_basis_terms": MAX_BASIS_TERMS,
        "max_certificate_terms": MAX_CERTIFICATE_TERMS,
        "max_certificate_product_accumulator_terms": MAX_PRODUCT_TERMS,
        "max_internal_provenance_terms": MAX_INTERNAL_PROVENANCE_TERMS,
        "max_internal_recomposition_product_work": (
            MAX_INTERNAL_RECOMPOSITION_PRODUCT_WORK
        ),
    }
    _require(limits["schema"] == EVIDENCE_LIMITS_SCHEMA,
             "wrong evidence-limits schema")
    for field, expected in expected_constants.items():
        observed = _integer(limits[field], f"evidence_limits.{field}")
        _require(observed == expected, f"evidence limit field {field} differs")
    for field in (
        "basis_polynomials_observed",
        "basis_terms_observed",
        "certificate_terms_observed",
        "certificate_product_accumulator_peak",
        "internal_provenance_terms_observed_lower_bound",
        "internal_recomposition_product_work_observed_lower_bound",
    ):
        _integer(limits[field], f"evidence_limits.{field}", maximum=(1 << 64) - 1)
    for field in (
        "internal_provenance_terms_observation_kind",
        "internal_recomposition_product_work_observation_kind",
    ):
        _require(
            isinstance(limits[field], str),
            f"evidence_limits.{field} is not a string",
        )
        _require(
            limits[field] in {"exact", "lower_bound", "not_observed"},
            f"evidence_limits.{field} is not a recognized observation kind",
        )
    _validate_internal_observation(
        limits["internal_provenance_terms_observed_lower_bound"],
        limits["internal_provenance_terms_observation_kind"],
        MAX_INTERNAL_PROVENANCE_TERMS,
        "internal provenance observation",
    )
    _validate_internal_observation(
        limits["internal_recomposition_product_work_observed_lower_bound"],
        limits["internal_recomposition_product_work_observation_kind"],
        MAX_INTERNAL_RECOMPOSITION_PRODUCT_WORK,
        "internal recomposition observation",
    )
    attempted = limits["attempted_final_pretty_json_bytes"]
    if attempted is not None:
        _integer(
            attempted,
            "evidence_limits.attempted_final_pretty_json_bytes",
            maximum=(1 << 64) - 1,
        )
    for field in (
        "basis_within_limits",
        "certificate_within_limits",
        "certificate_product_accumulator_within_limit",
        "internal_provenance_within_limit",
        "internal_recomposition_product_work_within_limit",
        "all_pre_serialization_limits_satisfied",
        "final_json_size_checked_before_write",
        "final_json_within_limit",
    ):
        _require(type(limits[field]) is bool,
                 f"evidence limit check {field} is not boolean")
    return dict(limits)


def _evidence_limits(
    raw: Any,
    *,
    basis_polynomials: int,
    basis_terms: int,
    certificate_terms: int,
    certificate_product_accumulator_peak: int,
) -> dict[str, Any]:
    limits = _parse_evidence_limits(raw)
    expected_observations = {
        "basis_polynomials_observed": basis_polynomials,
        "basis_terms_observed": basis_terms,
        "certificate_terms_observed": certificate_terms,
        "certificate_product_accumulator_peak": (
            certificate_product_accumulator_peak
        ),
        "internal_provenance_terms_observed_lower_bound": 0,
        "internal_recomposition_product_work_observed_lower_bound": 0,
    }
    for field, expected in expected_observations.items():
        _require(limits[field] == expected,
                 f"evidence limit field {field} differs")
    _require(limits["attempted_final_pretty_json_bytes"] is None,
             "successful artifact reports an attempted oversized JSON size")
    for field in (
        "basis_within_limits",
        "certificate_within_limits",
        "certificate_product_accumulator_within_limit",
        "internal_provenance_within_limit",
        "internal_recomposition_product_work_within_limit",
        "all_pre_serialization_limits_satisfied",
        "final_json_size_checked_before_write",
        "final_json_within_limit",
    ):
        _require(limits[field] is True, f"evidence limit check {field} failed")
    return limits


def _resource_refusal_receipt(
    document: Mapping[str, Any], *, producer_artifact_sha256: str
) -> dict[str, Any]:
    required = {
        "schema",
        "case",
        "public_residual",
        "base_x",
        "p",
        "curve_a",
        "curve_b",
        "n",
        "input_quadrics",
        "h8_coefficients",
        "input_system",
        "input_system_hash_encoding",
        "input_system_canonical_json_bytes",
        "input_system_sha256",
        "no_sign_mask_or_secret_scalar_passed",
        "status",
        "outcome",
        "admissible_positive",
        "admissible_negative",
        "complete_basis",
        "inconsistent",
        "timed_out",
        "oversize",
        "pairs_above_bound",
        "staircase_at_stop",
        "degree_reached",
        "solving_degree_max",
        "radix_embedding",
        "evidence_limits",
        "resource_refusal",
        "pcc",
    }
    _exact_keys(document, required, "resource-censored producer artifact")
    _require(document["schema"] == PRODUCER_SCHEMA, "wrong producer schema")
    case = document["case"]
    _require(_is_int(case) and 0 <= case < len(RESIDUALS), "invalid case")
    residual = RESIDUALS[case]
    _exact_public_point(document["public_residual"], residual, "public_residual")
    _require(
        isinstance(document["base_x"], list)
        and all(_is_int(value) for value in document["base_x"])
        and document["base_x"] == list(BASE_X),
        "unexpected factor-base x values",
    )
    _exact_integer_binding(document["p"], P, "p")
    _exact_integer_binding(document["curve_a"], CURVE_A, "curve_a")
    _exact_integer_binding(document["curve_b"], CURVE_B, "curve_b")
    _exact_integer_binding(document["n"], GROUP_ORDER, "n")
    _exact_integer_binding(document["input_quadrics"], 8, "input_quadrics")
    _require(document["no_sign_mask_or_secret_scalar_passed"] is True,
             "producer did not attest public-only input")
    h8 = h8_coefficients()
    reported_h8 = document["h8_coefficients"]
    _require(
        isinstance(reported_h8, list)
        and len(reported_h8) == len(h8)
        and all(_is_int(value) for value in reported_h8)
        and reported_h8 == h8,
        "H8 coefficients differ",
    )
    expected_system = expected_input_system(case)
    expected_system_bytes = _compact_json_bytes(expected_system)
    expected_system_hash = _sha256(expected_system_bytes)
    _require(
        document["input_system"] == expected_system
        and _compact_json_bytes(document["input_system"]) == expected_system_bytes,
        "serialized input system differs from independent reconstruction",
    )
    _require(isinstance(document["input_system_hash_encoding"], str)
             and document["input_system_hash_encoding"] == INPUT_HASH_ENCODING,
             "unexpected input-system hash encoding")
    _exact_integer_binding(
        document["input_system_canonical_json_bytes"],
        len(expected_system_bytes),
        "input_system_canonical_json_bytes",
    )
    _require(isinstance(document["input_system_sha256"], str)
             and document["input_system_sha256"] == expected_system_hash,
             "input-system SHA-256 differs from canonical reconstruction")
    _require(document["status"] == "resource_censored",
             "resource refusal has the wrong producer status")
    _require(document["outcome"] == "resource_censored",
             "resource refusal has the wrong producer outcome")
    _require(document["admissible_positive"] is False,
             "resource refusal marks a positive admissible")
    _require(document["admissible_negative"] is False,
             "resource refusal marks a negative admissible")
    for field in ("complete_basis", "inconsistent", "timed_out", "oversize"):
        _require(type(document[field]) is bool,
                 f"resource refusal field {field} is not boolean")
    pairs_above = _integer(
        document["pairs_above_bound"], "resource refusal pairs_above_bound"
    )
    degree_reached = _integer(
        document["degree_reached"], "resource refusal degree_reached"
    )
    solving_degree = _integer(
        document["solving_degree_max"], "resource refusal solving_degree_max"
    )
    staircase_at_stop = document["staircase_at_stop"]
    if staircase_at_stop is not None:
        _integer(
            staircase_at_stop,
            "resource refusal staircase_at_stop",
            maximum=(1 << 64) - 1,
        )
    _verify_radix_embedding(document["radix_embedding"])
    _require(degree_reached <= FROZEN_SOLVING_DEGREE,
             "resource refusal reached an unsafe degree")
    _require(solving_degree <= FROZEN_SOLVING_DEGREE,
             "resource refusal reports an unsafe solving degree")
    expected_complete = (
        not document["timed_out"]
        and not document["oversize"]
        and pairs_above == 0
        and staircase_at_stop is None
    )
    _require(document["complete_basis"] is expected_complete,
             "resource refusal solver-completion fields are inconsistent")
    _require(document["pcc"] == "unverified", "unexpected PCC status")

    limits = _parse_evidence_limits(document["evidence_limits"])
    refusal = _exact_keys(
        document["resource_refusal"],
        {
            "schema",
            "reason",
            "basis_polynomials_observed",
            "basis_terms_observed",
            "certificate_terms_observed",
            "certificate_product_accumulator_peak",
            "internal_provenance_terms_observed_lower_bound",
            "internal_provenance_terms_observation_kind",
            "internal_recomposition_product_work_observed_lower_bound",
            "internal_recomposition_product_work_observation_kind",
            "attempted_final_pretty_json_bytes",
            "suppressed_evidence",
        },
        "resource_refusal",
    )
    _require(refusal["schema"] == RESOURCE_REFUSAL_SCHEMA,
             "wrong resource-refusal schema")
    _require(refusal["suppressed_evidence"] is True,
             "resource refusal did not suppress unavailable evidence")
    for field in (
        "basis_polynomials_observed",
        "basis_terms_observed",
        "certificate_terms_observed",
        "certificate_product_accumulator_peak",
        "internal_provenance_terms_observed_lower_bound",
        "internal_recomposition_product_work_observed_lower_bound",
    ):
        observed = _integer(
            refusal[field], f"resource_refusal.{field}", maximum=(1 << 64) - 1
        )
        _require(observed == limits[field],
                 f"resource refusal field {field} differs from evidence limits")
    for field in (
        "internal_provenance_terms_observation_kind",
        "internal_recomposition_product_work_observation_kind",
    ):
        _require(
            isinstance(refusal[field], str),
            f"resource_refusal.{field} is not a string",
        )
        _require(
            refusal[field] == limits[field],
            f"resource refusal field {field} differs from evidence limits",
        )
    attempted = refusal["attempted_final_pretty_json_bytes"]
    if attempted is not None:
        _integer(
            attempted,
            "resource_refusal.attempted_final_pretty_json_bytes",
            maximum=(1 << 64) - 1,
        )
    _require(attempted == limits["attempted_final_pretty_json_bytes"],
             "attempted final JSON size bindings differ")

    basis_within = (
        limits["basis_polynomials_observed"] <= MAX_BASIS_POLYNOMIALS
        and limits["basis_terms_observed"] <= MAX_BASIS_TERMS
    )
    certificate_within = (
        limits["certificate_terms_observed"] <= MAX_CERTIFICATE_TERMS
    )
    accumulator_within = (
        limits["certificate_product_accumulator_peak"] <= MAX_PRODUCT_TERMS
    )
    internal_provenance_within = (
        limits["internal_provenance_terms_observed_lower_bound"]
        <= MAX_INTERNAL_PROVENANCE_TERMS
    )
    internal_recomposition_within = (
        limits["internal_recomposition_product_work_observed_lower_bound"]
        <= MAX_INTERNAL_RECOMPOSITION_PRODUCT_WORK
    )
    all_pre_serialization = (
        basis_within
        and certificate_within
        and accumulator_within
        and internal_provenance_within
        and internal_recomposition_within
    )
    final_within = attempted is None or attempted <= MAX_INPUT_BYTES
    expected_flags = {
        "basis_within_limits": basis_within,
        "certificate_within_limits": certificate_within,
        "certificate_product_accumulator_within_limit": accumulator_within,
        "internal_provenance_within_limit": internal_provenance_within,
        "internal_recomposition_product_work_within_limit": (
            internal_recomposition_within
        ),
        "all_pre_serialization_limits_satisfied": all_pre_serialization,
        "final_json_size_checked_before_write": True,
        "final_json_within_limit": final_within,
    }
    for field, expected in expected_flags.items():
        _require(limits[field] is expected,
                 f"resource refusal limit flag {field} differs")

    if limits["basis_polynomials_observed"] > MAX_BASIS_POLYNOMIALS:
        expected_reason = "basis_polynomial_limit_exceeded"
    elif limits["basis_terms_observed"] > MAX_BASIS_TERMS:
        expected_reason = "basis_term_limit_exceeded"
    elif limits["certificate_terms_observed"] > MAX_CERTIFICATE_TERMS:
        expected_reason = "certificate_term_limit_exceeded"
    elif limits["certificate_product_accumulator_peak"] > MAX_PRODUCT_TERMS:
        expected_reason = "certificate_product_accumulator_limit_exceeded"
    elif (
        limits["internal_provenance_terms_observed_lower_bound"]
        > MAX_INTERNAL_PROVENANCE_TERMS
    ):
        expected_reason = "internal_provenance_term_limit_exceeded"
    elif (
        limits["internal_recomposition_product_work_observed_lower_bound"]
        > MAX_INTERNAL_RECOMPOSITION_PRODUCT_WORK
    ):
        expected_reason = "internal_recomposition_product_work_limit_exceeded"
    elif attempted is not None and attempted > MAX_INPUT_BYTES:
        expected_reason = "final_json_limit_exceeded"
    else:
        raise VerificationError("resource refusal does not exceed a frozen limit")
    _require(refusal["reason"] == expected_reason,
             "resource refusal reason differs from observed limit breach")
    if expected_reason == "certificate_product_accumulator_limit_exceeded":
        _require(
            limits["certificate_product_accumulator_peak"]
            == MAX_PRODUCT_TERMS + 1,
            "accumulator refusal did not preserve the first violating peak",
        )
    if expected_reason == "final_json_limit_exceeded":
        _require(all_pre_serialization,
                 "final JSON refusal also contains a pre-serialization breach")
    else:
        _require(attempted is None,
                 "pre-serialization refusal reports an attempted final JSON size")

    return {
        "schema": OUTPUT_SCHEMA,
        "producer_artifact_sha256": producer_artifact_sha256,
        "verifier_source_sha256": _sha256(Path(__file__).read_bytes()),
        "case": case,
        "public_residual": list(residual),
        "base_x": list(BASE_X),
        "input_system_sha256": expected_system_hash,
        "disposition": "resource_censored",
        "pass": False,
        "recovered_coefficients": None,
        "selected_points": None,
        "negative_certificate_terms": None,
        "negative_certificate_product_accumulator_peak": None,
        "negative_certificate_recomposition_product_work": None,
        "evidence_limits": limits,
        "resource_refusal": dict(refusal),
        "radix_open_top": _radix_open_top_summary(document["radix_embedding"]),
        "checks": {
            "producer_schema_bound": True,
            "public_curve_and_case_bound": True,
            "h8_recomputed": True,
            "original_eight_quadrics_recomputed": True,
            "input_system_sha256_recomputed": True,
            "solver_completion_fields_preserved": True,
            "radix_embedding_and_open_top_admitted": True,
            "resource_limit_constants_bound": True,
            "resource_observations_preserved": True,
            "resource_refusal_reason_recomputed": True,
            "suppressed_evidence_not_required": True,
        },
        "independent_implementation": True,
        "pcc": "unverified",
    }


def verify_document(
    document: Mapping[str, Any], *, producer_artifact_sha256: str
) -> dict[str, Any]:
    """Verify a parsed producer artifact and return the independent receipt."""

    if document.get("resource_refusal") is not None:
        return _resource_refusal_receipt(
            document, producer_artifact_sha256=producer_artifact_sha256
        )

    required = {
        "schema",
        "case",
        "public_residual",
        "base_x",
        "p",
        "curve_a",
        "curve_b",
        "n",
        "input_quadrics",
        "h8_coefficients",
        "input_system",
        "input_system_hash_encoding",
        "input_system_canonical_json_bytes",
        "input_system_sha256",
        "no_sign_mask_or_secret_scalar_passed",
        "complete_basis",
        "inconsistent",
        "timed_out",
        "oversize",
        "pairs_above_bound",
        "staircase_at_stop",
        "basis_size",
        "degree_reached",
        "solving_degree_max",
        "radix_embedding",
        "recovered_coefficients",
        "linear_extraction",
        "basis_evidence",
        "negative_unit_identity",
        "negative_certificate",
        "status",
        "outcome",
        "admissible_positive",
        "admissible_negative",
        "positive_replay_passed",
        "evidence_limits",
        "resource_refusal",
    }
    _required_keys(document, required, "producer artifact")
    _require(document["resource_refusal"] is None,
             "admissible artifact unexpectedly contains a resource refusal")
    _require(document["schema"] == PRODUCER_SCHEMA, "wrong producer schema")
    case = document["case"]
    _require(_is_int(case) and 0 <= case < len(RESIDUALS), "invalid case")
    residual = RESIDUALS[case]
    _exact_public_point(document["public_residual"], residual, "public_residual")
    _require(
        isinstance(document["base_x"], list)
        and all(_is_int(value) for value in document["base_x"])
        and document["base_x"] == list(BASE_X),
        "unexpected factor-base x values",
    )
    _exact_integer_binding(document["p"], P, "p")
    _exact_integer_binding(document["curve_a"], CURVE_A, "curve_a")
    _exact_integer_binding(document["curve_b"], CURVE_B, "curve_b")
    _exact_integer_binding(document["n"], GROUP_ORDER, "n")
    _exact_integer_binding(document["input_quadrics"], 8, "input_quadrics")
    _require(document["no_sign_mask_or_secret_scalar_passed"] is True,
             "producer did not attest public-only input")
    _require((4 * CURVE_A**3 + 27 * CURVE_B**2) % P != 0,
             "public curve is singular")
    _solver_completion_checks(document)

    h8 = h8_coefficients()
    reported_h8 = document["h8_coefficients"]
    _require(
        isinstance(reported_h8, list)
        and len(reported_h8) == len(h8)
        and all(_is_int(value) for value in reported_h8)
        and reported_h8 == h8,
        "H8 coefficients differ",
    )
    _require(h8[-1] == 1 and len(h8) == 9, "H8 is not monic degree eight")
    expected_system = expected_input_system(case)
    expected_system_bytes = _compact_json_bytes(expected_system)
    expected_system_hash = _sha256(expected_system_bytes)
    _require(
        document["input_system"] == expected_system
        and _compact_json_bytes(document["input_system"]) == expected_system_bytes,
             "serialized input system differs from independent reconstruction")
    _require(isinstance(document["input_system_hash_encoding"], str)
             and document["input_system_hash_encoding"] == INPUT_HASH_ENCODING,
             "unexpected input-system hash encoding")
    _exact_integer_binding(
        document["input_system_canonical_json_bytes"],
        len(expected_system_bytes),
        "input_system_canonical_json_bytes",
    )
    _require(isinstance(document["input_system_sha256"], str)
             and document["input_system_sha256"] == expected_system_hash,
             "input-system SHA-256 differs from canonical reconstruction")
    degree_reached = _integer(document["degree_reached"], "degree_reached")
    solving_degree = _integer(
        document["solving_degree_max"], "solving_degree_max"
    )
    _require(degree_reached <= FROZEN_SOLVING_DEGREE,
             "reported degree exceeds the frozen radix-safe bound")
    _require(solving_degree <= FROZEN_SOLVING_DEGREE,
             "solving degree exceeds the frozen radix-safe bound")
    radix_embedding = document["radix_embedding"]
    _verify_radix_embedding(radix_embedding)

    basis, basis_linear_rows = _basis_evidence(document["basis_evidence"])
    _require(_integer(document["basis_size"], "basis_size") == len(basis),
             "basis size differs")
    recovered = _linear_evidence(document["linear_extraction"], basis_linear_rows)
    quadrics = original_quadrics(residual, h8)
    checks: dict[str, bool] = {
        "producer_schema_bound": True,
        "public_curve_and_case_bound": True,
        "h8_recomputed": True,
        "original_eight_quadrics_recomputed": True,
        "input_system_sha256_recomputed": True,
        "radix_embedding_and_open_top_admitted": True,
        "completed_uncensored_basis_reported": True,
        "basis_artifact_parsed": True,
        "linear_rows_derived_from_basis": True,
        "linear_rref_and_ranks_recomputed": True,
    }

    if case == 0:
        _require(document["inconsistent"] is False,
                 "positive case is marked inconsistent")
        _require(document["status"] == "admissible",
                 "positive producer status is not admissible")
        _require(document["outcome"] == "positive_verified",
                 "positive producer outcome differs")
        _require(document["admissible_positive"] is True,
                 "producer did not admit the positive")
        _require(document["admissible_negative"] is False,
                 "positive case is marked as an admitted negative")
        _require(document["positive_replay_passed"] is True,
                 "producer positive replay did not pass")
        coefficients = _canonical_coefficients(
            document["recovered_coefficients"], "recovered_coefficients"
        )
        _require(recovered == coefficients,
                 "top-level coefficients differ from linear replay")
        for key in (
            "original_eight_coefficients_checked",
            "candidate_vanishes_on_f4_basis",
            "full_signed_point_sum_checked",
        ):
            _require(document.get(key) is True, f"producer check {key} did not pass")
        _replay_require(
            all(_evaluate_sparse(quadric, coefficients) == 0 for quadric in quadrics),
            "a reconstructed original quadric is nonzero",
        )
        _replay_require(_direct_m(coefficients, residual) == h8,
                        "direct M=H8 coefficient identity failed")
        _replay_require(
            all(_evaluate_sparse(polynomial, coefficients) == 0
                for polynomial in basis),
            "recovered coefficients do not vanish on the serialized basis",
        )
        selected_points = _signed_point_replay(coefficients, residual)
        _require(document["negative_unit_identity"] is None,
                 "positive case unexpectedly contains a negative certificate")
        _negative_status(document, False)
        checks.update(
            {
                "unique_linear_solution_recomputed": True,
                "all_eight_original_equations_zero": True,
                "direct_m_equals_h8": True,
                "serialized_basis_vanishes": True,
                "all_eight_original_points_verified": True,
                "signed_point_sum_matches_residual": True,
                "negative_unit_identity_not_applicable": True,
            }
        )
        disposition = "positive_verified"
        passed = True
        certificate_terms = None
        certificate_accumulator_peak = None
        certificate_product_work = None
    else:
        _require(document["inconsistent"] is True,
                 "negative case is not marked inconsistent")
        _require(document["recovered_coefficients"] is None,
                 "negative case reports recovered coefficients")
        _require(recovered is None, "negative linear evidence reports a solution")
        basis_contains_unit = any(
            len(polynomial) == 1
            and ZERO_EXPONENT in polynomial
            and polynomial[ZERO_EXPONENT] != 0
            for polynomial in basis
        )
        _require(basis_contains_unit,
                 "inconsistent serialized basis lacks a nonzero constant")
        raw_certificate = document.get("negative_unit_identity")
        has_identity = raw_certificate is not None
        _negative_status(document, has_identity)
        _require(document["admissible_positive"] is False,
                 "negative case is marked as an admitted positive")
        _require(document["positive_replay_passed"] is False,
                 "negative case reports a positive replay")
        if has_identity:
            _require(document["status"] == "admissible",
                     "certified negative producer status is not admissible")
            _require(document["outcome"] == "negative_verified",
                     "certified negative producer outcome differs")
            _require(document["admissible_negative"] is True,
                     "producer did not admit its certified negative")
        else:
            _require(document["status"] == "inadmissible",
                     "unsupported negative status differs")
            _require(
                document["outcome"] == "negative_observation_certificate_unavailable",
                "unsupported negative outcome differs",
            )
            _require(document["admissible_negative"] is False,
                     "uncertified negative is marked admissible")
        checks.update(
            {
                "inconsistent_basis_contains_unit": True,
                "negative_unit_identity_replayed": False,
            }
        )
        selected_points = None
        coefficients = None
        certificate_terms = None
        certificate_accumulator_peak = None
        certificate_product_work = None
        if has_identity:
            (
                certificate_terms,
                certificate_accumulator_peak,
                certificate_product_work,
            ) = verify_unit_identity(
                raw_certificate,
                case=case,
                input_sha256=expected_system_hash,
                quadrics=quadrics,
            )
            checks["negative_unit_identity_replayed"] = True
            disposition = "negative_verified"
            passed = True
        else:
            disposition = "certificate_unsupported"
            passed = False

    evidence_limits = _evidence_limits(
        document["evidence_limits"],
        basis_polynomials=len(basis),
        basis_terms=sum(len(polynomial) for polynomial in basis),
        certificate_terms=certificate_terms or 0,
        certificate_product_accumulator_peak=certificate_accumulator_peak or 0,
    )
    checks["producer_and_verifier_evidence_limits_aligned"] = True
    checks["certificate_recomposition_product_work_bounded"] = (
        certificate_product_work is None
        or certificate_product_work <= MAX_INTERNAL_RECOMPOSITION_PRODUCT_WORK
    )

    verifier_source_sha256 = _sha256(Path(__file__).read_bytes())
    return {
        "schema": OUTPUT_SCHEMA,
        "producer_artifact_sha256": producer_artifact_sha256,
        "verifier_source_sha256": verifier_source_sha256,
        "case": case,
        "public_residual": list(residual),
        "base_x": list(BASE_X),
        "input_system_sha256": expected_system_hash,
        "disposition": disposition,
        "pass": passed,
        "recovered_coefficients": coefficients,
        "selected_points": selected_points,
        "negative_certificate_terms": certificate_terms,
        "negative_certificate_product_accumulator_peak": (
            certificate_accumulator_peak
        ),
        "negative_certificate_recomposition_product_work": (
            certificate_product_work
        ),
        "evidence_limits": evidence_limits,
        "resource_refusal": None,
        "radix_open_top": _radix_open_top_summary(radix_embedding),
        "checks": checks,
        "independent_implementation": True,
        "pcc": "unverified",
    }


def verify_bytes(raw: bytes) -> dict[str, Any]:
    artifact_hash = _sha256(raw)
    return verify_document(
        load_json_strict(raw), producer_artifact_sha256=artifact_hash
    )


def _failure_receipt(
    raw: bytes, exc: VerificationError, document: Mapping[str, Any] | None
) -> dict[str, Any]:
    case = document.get("case") if isinstance(document, dict) else None
    residual = document.get("public_residual") if isinstance(document, dict) else None
    system_hash = (
        document.get("input_system_sha256") if isinstance(document, dict) else None
    )
    disposition = (
        "verification_failed" if isinstance(exc, ReplayError)
        else "invalid_producer_artifact"
    )
    return {
        "schema": OUTPUT_SCHEMA,
        "producer_artifact_sha256": _sha256(raw),
        "verifier_source_sha256": _sha256(Path(__file__).read_bytes()),
        "case": case,
        "public_residual": residual,
        "base_x": list(BASE_X),
        "input_system_sha256": system_hash,
        "disposition": disposition,
        "pass": False,
        "recovered_coefficients": None,
        "selected_points": None,
        "negative_certificate_terms": None,
        "negative_certificate_product_accumulator_peak": None,
        "negative_certificate_recomposition_product_work": None,
        "evidence_limits": None,
        "resource_refusal": None,
        "radix_open_top": None,
        "checks": {},
        "error": str(exc),
        "independent_implementation": True,
        "pcc": "unverified",
    }


def _unhashed_input_receipt(
    exc: VerificationError,
    *,
    observed_bytes: int | None,
    size_is_exact: bool | None,
) -> dict[str, Any]:
    """Emit a bounded refusal when custody rules forbid reading or hashing."""

    return {
        "schema": OUTPUT_SCHEMA,
        "producer_artifact_sha256": None,
        "producer_artifact_sha256_complete": False,
        "producer_artifact_size_bytes_observed": observed_bytes,
        "producer_artifact_size_is_exact": size_is_exact,
        "verifier_source_sha256": _sha256(Path(__file__).read_bytes()),
        "case": None,
        "public_residual": None,
        "base_x": list(BASE_X),
        "input_system_sha256": None,
        "disposition": "invalid_producer_artifact",
        "pass": False,
        "recovered_coefficients": None,
        "selected_points": None,
        "negative_certificate_terms": None,
        "negative_certificate_product_accumulator_peak": None,
        "negative_certificate_recomposition_product_work": None,
        "evidence_limits": None,
        "resource_refusal": None,
        "radix_open_top": None,
        "checks": {},
        "error": str(exc),
        "independent_implementation": True,
        "pcc": "unverified",
    }


def _oversized_input_receipt(exc: ArtifactTooLarge) -> dict[str, Any]:
    return _unhashed_input_receipt(
        exc,
        observed_bytes=exc.observed_bytes,
        size_is_exact=exc.exact,
    )


def main(argv: Sequence[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("producer_json", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args(argv)
    try:
        raw = read_artifact_bounded(args.producer_json)
    except ArtifactTooLarge as exc:
        receipt = _oversized_input_receipt(exc)
    except ArtifactCustodyError as exc:
        receipt = _unhashed_input_receipt(
            exc, observed_bytes=None, size_is_exact=None
        )
    except OSError as exc:
        parser.error(str(exc))
    else:
        document: Mapping[str, Any] | None = None
        try:
            document = load_json_strict(raw)
            receipt = verify_document(
                document, producer_artifact_sha256=_sha256(raw)
            )
        except VerificationError as exc:
            receipt = _failure_receipt(raw, exc, document)
    encoded = json.dumps(receipt, indent=2, sort_keys=True) + "\n"
    if args.output is None:
        sys.stdout.write(encoded)
    else:
        args.output.write_text(encoded)
    if receipt["pass"]:
        return 0
    return 1 if receipt["disposition"] in {
        "certificate_unsupported", "verification_failed", "resource_censored"
    } else 2


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
"""Deterministic public/synthetic tests for the independent full8 verifier.

The fixtures use only the three registered public residuals and the already
published case-0 coefficient vector.  They do not read or derive any blinded
case, sign mask, target scalar, producer output, or producer source code.
"""

from __future__ import annotations

import copy
import contextlib
import io
import json
import os
import sys
import tempfile
import unittest
from pathlib import Path


sys.path.insert(0, str(Path(__file__).resolve().parent))
import verify_full8_f4 as verifier  # noqa: E402


PUBLIC_POSITIVE_COEFFICIENTS = [
    455_835_358,
    518_652_820,
    754_716_693,
    932_383_545,
    62_510_441,
    1_884_059_825,
    1_142_049_404,
]


def _term(exponents: list[int], coefficient: int) -> dict[str, object]:
    return {"exponents": exponents, "coefficient": coefficient}


def _linear_basis(
    coefficients: list[int],
) -> tuple[dict[str, object], dict[str, object]]:
    polynomials = []
    rows = []
    zero = [0] * verifier.N_VARIABLES
    for variable, coefficient in enumerate(coefficients):
        exponent = [0] * verifier.N_VARIABLES
        exponent[variable] = 1
        constant = (-coefficient) % verifier.P
        polynomials.append([_term(exponent, 1), _term(zero.copy(), constant)])
        row = [0] * (verifier.N_VARIABLES + 1)
        row[variable] = 1
        row[-1] = constant
        rows.append(row)
    basis = {
        "schema": verifier.BASIS_SCHEMA,
        "polynomial_count": len(polynomials),
        "term_count": sum(len(polynomial) for polynomial in polynomials),
        "canonical_polynomials": polynomials,
        "blocker": None,
    }
    linear = {
        "schema": verifier.LINEAR_SCHEMA,
        "raw_rows": rows,
        "rref_rows": copy.deepcopy(rows),
        "pivot_columns": list(range(verifier.N_VARIABLES)),
        "coefficient_rank": verifier.N_VARIABLES,
        "augmented_rank": verifier.N_VARIABLES,
        "unique_solution": True,
        "recovered_coefficients": coefficients.copy(),
        "blocker": None,
    }
    return basis, linear


def _evidence_limits(
    basis_polynomials: int,
    basis_terms: int,
    certificate_terms: int = 0,
    certificate_product_accumulator_peak: int = 0,
) -> dict[str, object]:
    return {
        "schema": verifier.EVIDENCE_LIMITS_SCHEMA,
        "max_final_json_bytes": verifier.MAX_INPUT_BYTES,
        "max_basis_polynomials": verifier.MAX_BASIS_POLYNOMIALS,
        "max_basis_terms": verifier.MAX_BASIS_TERMS,
        "max_certificate_terms": verifier.MAX_CERTIFICATE_TERMS,
        "max_certificate_product_accumulator_terms": verifier.MAX_PRODUCT_TERMS,
        "max_internal_provenance_terms": verifier.MAX_INTERNAL_PROVENANCE_TERMS,
        "max_internal_recomposition_product_work": (
            verifier.MAX_INTERNAL_RECOMPOSITION_PRODUCT_WORK
        ),
        "basis_polynomials_observed": basis_polynomials,
        "basis_terms_observed": basis_terms,
        "certificate_terms_observed": certificate_terms,
        "certificate_product_accumulator_peak": (
            certificate_product_accumulator_peak
        ),
        "internal_provenance_terms_observed_lower_bound": 0,
        "internal_provenance_terms_observation_kind": "not_observed",
        "internal_recomposition_product_work_observed_lower_bound": 0,
        "internal_recomposition_product_work_observation_kind": "not_observed",
        "attempted_final_pretty_json_bytes": None,
        "basis_within_limits": True,
        "certificate_within_limits": True,
        "certificate_product_accumulator_within_limit": True,
        "internal_provenance_within_limit": True,
        "internal_recomposition_product_work_within_limit": True,
        "all_pre_serialization_limits_satisfied": True,
        "final_json_size_checked_before_write": True,
        "final_json_within_limit": True,
    }


def _common_fixture(case: int) -> dict[str, object]:
    input_system = verifier.expected_input_system(case)
    return {
        "schema": verifier.PRODUCER_SCHEMA,
        "case": case,
        "public_residual": list(verifier.RESIDUALS[case]),
        "base_x": list(verifier.BASE_X),
        "p": verifier.P,
        "curve_a": verifier.CURVE_A,
        "curve_b": verifier.CURVE_B,
        "n": verifier.GROUP_ORDER,
        "input_quadrics": 8,
        "h8_coefficients": verifier.h8_coefficients(),
        "input_system": input_system,
        "input_system_hash_encoding": verifier.INPUT_HASH_ENCODING,
        "input_system_canonical_json_bytes": len(
            verifier._compact_json_bytes(input_system)
        ),
        "input_system_sha256": verifier._sha256(
            verifier._compact_json_bytes(input_system)
        ),
        "no_sign_mask_or_secret_scalar_passed": True,
        "complete_basis": True,
        "timed_out": False,
        "oversize": False,
        "pairs_above_bound": 0,
        "staircase_at_stop": None,
        "degree_reached": verifier.FROZEN_SOLVING_DEGREE,
        "solving_degree_max": verifier.FROZEN_SOLVING_DEGREE,
        "radix_embedding": verifier.expected_radix_embedding(),
        "negative_unit_identity": None,
        "resource_refusal": None,
        "negative_certificate": {
            "schema": verifier.NEGATIVE_STATUS_SCHEMA,
            "available": False,
            "replayed": False,
            "internal_trace_available": False,
            "internal_trace_kept_rows": None,
            "blocker": "solver_did_not_observe_a_unit_basis",
        },
    }


def positive_fixture(
    coefficients: list[int] | None = None,
) -> dict[str, object]:
    if coefficients is None:
        coefficients = PUBLIC_POSITIVE_COEFFICIENTS.copy()
    basis, linear = _linear_basis(coefficients)
    fixture = _common_fixture(0)
    fixture.update(
        {
            "inconsistent": False,
            "status": "admissible",
            "outcome": "positive_verified",
            "admissible_positive": True,
            "admissible_negative": False,
            "positive_replay_passed": True,
            "basis_size": 7,
            "recovered_coefficients": coefficients.copy(),
            "linear_extraction": linear,
            "basis_evidence": basis,
            "evidence_limits": _evidence_limits(7, 14),
            "original_eight_coefficients_checked": True,
            "candidate_vanishes_on_f4_basis": True,
            "full_signed_point_sum_checked": True,
        }
    )
    return fixture


def negative_unsupported_fixture(case: int = 1) -> dict[str, object]:
    zero = [0] * verifier.N_VARIABLES
    constant_row = [0] * (verifier.N_VARIABLES + 1)
    constant_row[-1] = 1
    basis = {
        "schema": verifier.BASIS_SCHEMA,
        "polynomial_count": 1,
        "term_count": 1,
        "canonical_polynomials": [[_term(zero, 1)]],
        "blocker": None,
    }
    linear = {
        "schema": verifier.LINEAR_SCHEMA,
        "raw_rows": [constant_row],
        "rref_rows": [constant_row.copy()],
        "pivot_columns": [],
        "coefficient_rank": 0,
        "augmented_rank": 1,
        "unique_solution": False,
        "recovered_coefficients": None,
        "blocker": "inconsistent_constant_row",
    }
    fixture = _common_fixture(case)
    fixture.update(
        {
            "inconsistent": True,
            "status": "inadmissible",
            "outcome": "negative_observation_certificate_unavailable",
            "admissible_positive": False,
            "admissible_negative": False,
            "positive_replay_passed": False,
            "basis_size": 1,
            "recovered_coefficients": None,
            "linear_extraction": linear,
            "basis_evidence": basis,
            "evidence_limits": _evidence_limits(1, 1),
            "negative_certificate": {
                "schema": verifier.NEGATIVE_STATUS_SCHEMA,
                "available": False,
                "replayed": False,
                "internal_trace_available": False,
                "internal_trace_kept_rows": None,
                "blocker": "f4_refutation_trace_was_not_available",
            },
        }
    )
    return fixture


def resource_refusal_fixture(reason: str) -> dict[str, object]:
    common = _common_fixture(0)
    public_fields = {
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
    }
    fixture = {field: copy.deepcopy(common[field]) for field in public_fields}
    observations = {
        "basis_polynomials_observed": 7,
        "basis_terms_observed": 14,
        "certificate_terms_observed": 0,
        "certificate_product_accumulator_peak": 0,
        "internal_provenance_terms_observed_lower_bound": 0,
        "internal_provenance_terms_observation_kind": "not_observed",
        "internal_recomposition_product_work_observed_lower_bound": 0,
        "internal_recomposition_product_work_observation_kind": "not_observed",
    }
    attempted = None
    if reason == "basis_polynomial_limit_exceeded":
        observations["basis_polynomials_observed"] = (
            verifier.MAX_BASIS_POLYNOMIALS + 1
        )
    elif reason == "basis_term_limit_exceeded":
        observations["basis_terms_observed"] = verifier.MAX_BASIS_TERMS + 1
    elif reason == "certificate_term_limit_exceeded":
        observations["certificate_terms_observed"] = (
            verifier.MAX_CERTIFICATE_TERMS + 1
        )
    elif reason == "certificate_product_accumulator_limit_exceeded":
        observations["certificate_product_accumulator_peak"] = (
            verifier.MAX_PRODUCT_TERMS + 1
        )
    elif reason == "internal_provenance_term_limit_exceeded":
        observations["internal_provenance_terms_observed_lower_bound"] = (
            verifier.MAX_INTERNAL_PROVENANCE_TERMS + 1
        )
        observations["internal_provenance_terms_observation_kind"] = "lower_bound"
    elif reason == "internal_recomposition_product_work_limit_exceeded":
        observations[
            "internal_recomposition_product_work_observed_lower_bound"
        ] = verifier.MAX_INTERNAL_RECOMPOSITION_PRODUCT_WORK + 1
        observations[
            "internal_recomposition_product_work_observation_kind"
        ] = "lower_bound"
    elif reason == "final_json_limit_exceeded":
        attempted = verifier.MAX_INPUT_BYTES + 1
    else:
        raise AssertionError(f"unknown fixture reason: {reason}")
    basis_within = (
        observations["basis_polynomials_observed"]
        <= verifier.MAX_BASIS_POLYNOMIALS
        and observations["basis_terms_observed"] <= verifier.MAX_BASIS_TERMS
    )
    certificate_within = (
        observations["certificate_terms_observed"]
        <= verifier.MAX_CERTIFICATE_TERMS
    )
    accumulator_within = (
        observations["certificate_product_accumulator_peak"]
        <= verifier.MAX_PRODUCT_TERMS
    )
    internal_provenance_within = (
        observations["internal_provenance_terms_observed_lower_bound"]
        <= verifier.MAX_INTERNAL_PROVENANCE_TERMS
    )
    internal_recomposition_within = (
        observations["internal_recomposition_product_work_observed_lower_bound"]
        <= verifier.MAX_INTERNAL_RECOMPOSITION_PRODUCT_WORK
    )
    all_pre = (
        basis_within
        and certificate_within
        and accumulator_within
        and internal_provenance_within
        and internal_recomposition_within
    )
    limits = {
        "schema": verifier.EVIDENCE_LIMITS_SCHEMA,
        "max_final_json_bytes": verifier.MAX_INPUT_BYTES,
        "max_basis_polynomials": verifier.MAX_BASIS_POLYNOMIALS,
        "max_basis_terms": verifier.MAX_BASIS_TERMS,
        "max_certificate_terms": verifier.MAX_CERTIFICATE_TERMS,
        "max_certificate_product_accumulator_terms": verifier.MAX_PRODUCT_TERMS,
        "max_internal_provenance_terms": verifier.MAX_INTERNAL_PROVENANCE_TERMS,
        "max_internal_recomposition_product_work": (
            verifier.MAX_INTERNAL_RECOMPOSITION_PRODUCT_WORK
        ),
        **observations,
        "attempted_final_pretty_json_bytes": attempted,
        "basis_within_limits": basis_within,
        "certificate_within_limits": certificate_within,
        "certificate_product_accumulator_within_limit": accumulator_within,
        "internal_provenance_within_limit": internal_provenance_within,
        "internal_recomposition_product_work_within_limit": (
            internal_recomposition_within
        ),
        "all_pre_serialization_limits_satisfied": all_pre,
        "final_json_size_checked_before_write": True,
        "final_json_within_limit": attempted is None,
    }
    fixture.update(
        {
            "status": "resource_censored",
            "outcome": "resource_censored",
            "admissible_positive": False,
            "admissible_negative": False,
            "complete_basis": True,
            "inconsistent": False,
            "timed_out": False,
            "oversize": False,
            "pairs_above_bound": 0,
            "staircase_at_stop": None,
            "degree_reached": verifier.FROZEN_SOLVING_DEGREE,
            "solving_degree_max": verifier.FROZEN_SOLVING_DEGREE,
            "radix_embedding": verifier.expected_radix_embedding(),
            "evidence_limits": limits,
            "resource_refusal": {
                "schema": verifier.RESOURCE_REFUSAL_SCHEMA,
                "reason": reason,
                **observations,
                "attempted_final_pretty_json_bytes": attempted,
                "suppressed_evidence": True,
            },
            "pcc": "unverified",
        }
    )
    return fixture


def _run_cli_document(document: dict[str, object]) -> tuple[int, dict[str, object]]:
    with tempfile.TemporaryDirectory() as temporary:
        path = Path(temporary) / "producer.json"
        path.write_text(json.dumps(document), encoding="utf-8")
        stdout = io.StringIO()
        with contextlib.redirect_stdout(stdout):
            exit_code = verifier.main([str(path)])
    return exit_code, json.loads(stdout.getvalue())


class StrictInputTests(unittest.TestCase):
    def test_deeply_nested_json_is_a_bounded_verification_error(self) -> None:
        raw = b"[" * 10_000 + b"0" + b"]" * 10_000
        with self.assertRaisesRegex(verifier.VerificationError, "invalid JSON"):
            verifier.load_json_strict(raw)

    def test_pathological_integer_is_a_bounded_verification_error(self) -> None:
        raw = b'{"n":' + b"9" * 10_000 + b"}"
        with self.assertRaisesRegex(verifier.VerificationError, "invalid JSON"):
            verifier.load_json_strict(raw)

    def test_every_float_token_is_rejected_even_in_an_extra_field(self) -> None:
        for literal in (b"1.0", b"1e999", b"-1e999"):
            with self.subTest(literal=literal):
                raw = b'{"extra":{"value":' + literal + b"}}"
                with self.assertRaisesRegex(
                    verifier.VerificationError, "floating-point"
                ):
                    verifier.load_json_strict(raw)

    def test_sparse_oversized_file_is_rejected_before_read_or_hash(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "oversized.json"
            with path.open("wb") as stream:
                stream.truncate(verifier.MAX_INPUT_BYTES + 1)
            with self.assertRaises(verifier.ArtifactTooLarge) as caught:
                verifier.read_artifact_bounded(path)
            self.assertTrue(caught.exception.exact)
            self.assertEqual(
                caught.exception.observed_bytes, verifier.MAX_INPUT_BYTES + 1
            )
            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = verifier.main([str(path)])
            receipt = json.loads(stdout.getvalue())
            self.assertEqual(exit_code, 2)
            self.assertFalse(receipt["pass"])
            self.assertIsNone(receipt["producer_artifact_sha256"])
            self.assertFalse(receipt["producer_artifact_sha256_complete"])
            self.assertEqual(
                receipt["producer_artifact_size_bytes_observed"],
                verifier.MAX_INPUT_BYTES + 1,
            )

    def test_regular_file_is_read_from_one_stable_descriptor(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "small.json"
            path.write_bytes(b"{}")
            self.assertEqual(verifier.read_artifact_bounded(path), b"{}")

    def test_symlink_artifact_is_rejected_without_following(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            target = directory / "target.json"
            link = directory / "artifact.json"
            target.write_bytes(b"{}")
            link.symlink_to(target)
            with self.assertRaisesRegex(
                verifier.ArtifactCustodyError, "symlink"
            ):
                verifier.read_artifact_bounded(link)
            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = verifier.main([str(link)])
            receipt = json.loads(stdout.getvalue())
            self.assertEqual(exit_code, 2)
            self.assertIsNone(receipt["producer_artifact_sha256"])
            self.assertIn("symlink", receipt["error"])

    def test_fifo_artifact_is_rejected_without_blocking(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "artifact.fifo"
            os.mkfifo(path)
            with self.assertRaisesRegex(
                verifier.ArtifactCustodyError, "not a regular file"
            ):
                verifier.read_artifact_bounded(path)


class PositiveReplayTests(unittest.TestCase):
    def verify(self, document: dict[str, object]) -> dict[str, object]:
        return verifier.verify_document(
            document, producer_artifact_sha256="11" * 32
        )

    def test_public_positive_fixture_passes(self) -> None:
        receipt = self.verify(positive_fixture())
        self.assertTrue(receipt["pass"])
        self.assertEqual(receipt["disposition"], "positive_verified")
        self.assertEqual(
            receipt["recovered_coefficients"], PUBLIC_POSITIVE_COEFFICIENTS
        )
        self.assertEqual(len(receipt["selected_points"]), 8)
        self.assertTrue(all(receipt["checks"].values()))

    def test_canonical_input_hash_vectors(self) -> None:
        # Cross-language vectors: the Rust producer hashes recursively sorted
        # serde_json::Value maps with compact UTF-8 encoding.
        self.assertEqual(
            [verifier.input_system_sha256(case) for case in range(3)],
            [
                "05ede9c68813a0e3ef4da239a2adb5bcbfaa7e563b56a6c42439b29be1bf0707",
                "90ea3b65be87c566c1e141927d143f3fd46e3b12807d034805bfb19f8ba49b2c",
                "4a16e53d8a3efe2dc874661a9fc568ed526beeb42d818981bd48ebebe3699755",
            ],
        )

    def test_coherent_wrong_candidate_fails_original_equations(self) -> None:
        wrong = PUBLIC_POSITIVE_COEFFICIENTS.copy()
        wrong[0] = (wrong[0] + 1) % verifier.P
        with self.assertRaises(verifier.ReplayError):
            self.verify(positive_fixture(wrong))

    def test_public_input_mutation_is_rejected(self) -> None:
        fixture = positive_fixture()
        fixture["input_system"]["input_quadrics"][0][0]["coefficient"] ^= 1
        with self.assertRaisesRegex(
            verifier.VerificationError, "serialized input system"
        ):
            self.verify(fixture)

    def test_json_boolean_is_not_a_field_element(self) -> None:
        fixture = positive_fixture()
        fixture["h8_coefficients"][-1] = True
        with self.assertRaisesRegex(verifier.VerificationError, "H8 coefficients"):
            self.verify(fixture)

    def test_integer_public_bindings_reject_equal_valued_floats(self) -> None:
        fields_and_values = (
            ("p", float(verifier.P)),
            ("curve_a", float(verifier.CURVE_A)),
            ("curve_b", float(verifier.CURVE_B)),
            ("n", float(verifier.GROUP_ORDER)),
            ("input_quadrics", 8.0),
        )
        for field, value in fields_and_values:
            with self.subTest(field=field):
                fixture = positive_fixture()
                fixture[field] = value
                with self.assertRaises(verifier.VerificationError):
                    self.verify(fixture)

        fixture = positive_fixture()
        fixture["public_residual"][0] = float(verifier.RESIDUALS[0][0])
        with self.assertRaises(verifier.VerificationError):
            self.verify(fixture)
        fixture = positive_fixture()
        fixture["base_x"][1] = 1.0
        with self.assertRaises(verifier.VerificationError):
            self.verify(fixture)
        fixture = positive_fixture()
        byte_count = fixture["input_system_canonical_json_bytes"]
        fixture["input_system_canonical_json_bytes"] = float(byte_count)
        with self.assertRaises(verifier.VerificationError):
            self.verify(fixture)

    def test_linear_row_mutation_is_rejected(self) -> None:
        fixture = positive_fixture()
        fixture["linear_extraction"]["raw_rows"][0][-1] ^= 1
        with self.assertRaisesRegex(verifier.VerificationError, "raw rows"):
            self.verify(fixture)

    def test_basis_mutation_is_rejected(self) -> None:
        fixture = positive_fixture()
        fixture["basis_evidence"]["canonical_polynomials"][0][1][
            "coefficient"
        ] ^= 1
        with self.assertRaises(verifier.VerificationError):
            self.verify(fixture)

    def test_censor_flag_is_rejected(self) -> None:
        fixture = positive_fixture()
        fixture["timed_out"] = True
        with self.assertRaisesRegex(verifier.VerificationError, "timed out"):
            self.verify(fixture)

    def test_radix_open_top_mutation_is_rejected(self) -> None:
        fixture = positive_fixture()
        fixture["radix_embedding"]["top_rule_reductions_excluded_by_degree_bound"] = False
        with self.assertRaisesRegex(verifier.VerificationError, "radix/open-top"):
            self.verify(fixture)

    def test_radix_rule_layout_mutation_is_rejected(self) -> None:
        fixture = positive_fixture()
        fixture["radix_embedding"]["exact_block_rule_weight_layout_valid"] = False
        with self.assertRaisesRegex(verifier.VerificationError, "radix/open-top"):
            self.verify(fixture)

    def test_negative_open_top_counter_is_rejected(self) -> None:
        fixture = positive_fixture()
        fixture["radix_embedding"]["open_top_pairs_omitted"] = -1
        with self.assertRaisesRegex(verifier.VerificationError, "below 0"):
            self.verify(fixture)

    def test_evidence_limit_mutation_is_rejected(self) -> None:
        fixture = positive_fixture()
        fixture["evidence_limits"]["max_certificate_terms"] -= 1
        with self.assertRaisesRegex(verifier.VerificationError, "max_certificate_terms"):
            self.verify(fixture)

    def test_evidence_observation_mutation_is_rejected(self) -> None:
        fixture = positive_fixture()
        fixture["evidence_limits"]["basis_terms_observed"] += 1
        with self.assertRaisesRegex(verifier.VerificationError, "basis_terms_observed"):
            self.verify(fixture)

    def test_unhashable_evidence_kind_cli_is_invalid_artifact(self) -> None:
        fields = (
            "internal_provenance_terms_observation_kind",
            "internal_recomposition_product_work_observation_kind",
        )
        for field in fields:
            for mutation in ([], {}):
                with self.subTest(field=field, mutation=type(mutation).__name__):
                    fixture = positive_fixture()
                    fixture["evidence_limits"][field] = mutation
                    exit_code, receipt = _run_cli_document(fixture)
                    self.assertEqual(exit_code, 2)
                    self.assertFalse(receipt["pass"])
                    self.assertEqual(
                        receipt["disposition"], "invalid_producer_artifact"
                    )
                    self.assertIn("not a string", receipt["error"])

    def test_final_json_prewrite_check_is_required(self) -> None:
        fixture = positive_fixture()
        fixture["evidence_limits"]["final_json_size_checked_before_write"] = False
        with self.assertRaisesRegex(
            verifier.VerificationError, "final_json_size_checked_before_write"
        ):
            self.verify(fixture)

    def test_duplicate_json_key_is_rejected(self) -> None:
        raw = b'{"case":0,"case":1}'
        with self.assertRaisesRegex(verifier.VerificationError, "duplicate"):
            verifier.load_json_strict(raw)


class ResourceRefusalTests(unittest.TestCase):
    REASONS = (
        "basis_polynomial_limit_exceeded",
        "basis_term_limit_exceeded",
        "certificate_term_limit_exceeded",
        "certificate_product_accumulator_limit_exceeded",
        "internal_provenance_term_limit_exceeded",
        "internal_recomposition_product_work_limit_exceeded",
        "final_json_limit_exceeded",
    )

    def verify(self, document: dict[str, object]) -> dict[str, object]:
        return verifier.verify_document(
            document, producer_artifact_sha256="55" * 32
        )

    def test_each_bound_resource_refusal_is_well_formed_nonpass(self) -> None:
        for reason in self.REASONS:
            with self.subTest(reason=reason):
                receipt = self.verify(resource_refusal_fixture(reason))
                self.assertFalse(receipt["pass"])
                self.assertEqual(receipt["disposition"], "resource_censored")
                self.assertEqual(receipt["resource_refusal"]["reason"], reason)
                self.assertTrue(receipt["radix_open_top"]["open_top_receipt_valid"])
                self.assertEqual(
                    receipt["radix_open_top"]["open_top_rules_selected"], 7
                )
                self.assertTrue(all(receipt["checks"].values()))

    def test_refusal_reason_must_match_first_observed_breach(self) -> None:
        fixture = resource_refusal_fixture("basis_polynomial_limit_exceeded")
        fixture["resource_refusal"]["reason"] = "basis_term_limit_exceeded"
        with self.assertRaisesRegex(verifier.VerificationError, "reason differs"):
            self.verify(fixture)

    def test_refusal_counts_must_match_limit_receipt(self) -> None:
        fixture = resource_refusal_fixture("certificate_term_limit_exceeded")
        fixture["resource_refusal"]["certificate_terms_observed"] += 1
        with self.assertRaisesRegex(verifier.VerificationError, "differs"):
            self.verify(fixture)

    def test_accumulator_refusal_preserves_first_violating_peak(self) -> None:
        fixture = resource_refusal_fixture(
            "certificate_product_accumulator_limit_exceeded"
        )
        mutated = verifier.MAX_PRODUCT_TERMS + 2
        fixture["resource_refusal"][
            "certificate_product_accumulator_peak"
        ] = mutated
        fixture["evidence_limits"][
            "certificate_product_accumulator_peak"
        ] = mutated
        with self.assertRaisesRegex(verifier.VerificationError, "first violating peak"):
            self.verify(fixture)

    def test_internal_exact_jump_and_cap_plus_one_fallback_are_valid(self) -> None:
        fallback = resource_refusal_fixture(
            "internal_provenance_term_limit_exceeded"
        )
        fallback_receipt = self.verify(fallback)
        self.assertEqual(fallback_receipt["disposition"], "resource_censored")
        self.assertEqual(
            fallback["evidence_limits"][
                "internal_provenance_terms_observation_kind"
            ],
            "lower_bound",
        )

        exact = resource_refusal_fixture(
            "internal_recomposition_product_work_limit_exceeded"
        )
        exact_value = verifier.MAX_INTERNAL_RECOMPOSITION_PRODUCT_WORK + 42
        for container in (exact["evidence_limits"], exact["resource_refusal"]):
            container[
                "internal_recomposition_product_work_observed_lower_bound"
            ] = exact_value
            container[
                "internal_recomposition_product_work_observation_kind"
            ] = "exact"
        exact_receipt = self.verify(exact)
        self.assertEqual(exact_receipt["disposition"], "resource_censored")

    def test_internal_lower_bound_must_be_exactly_cap_plus_one(self) -> None:
        fixture = resource_refusal_fixture(
            "internal_provenance_term_limit_exceeded"
        )
        mutated = verifier.MAX_INTERNAL_PROVENANCE_TERMS + 42
        for container in (fixture["evidence_limits"], fixture["resource_refusal"]):
            container["internal_provenance_terms_observed_lower_bound"] = mutated
        with self.assertRaisesRegex(verifier.VerificationError, "cap plus one"):
            self.verify(fixture)

    def test_unhashable_refusal_kinds_cli_are_invalid_artifacts(self) -> None:
        fields = (
            "internal_provenance_terms_observation_kind",
            "internal_recomposition_product_work_observation_kind",
        )
        for container_name in ("evidence_limits", "resource_refusal"):
            for field in fields:
                for mutation in ([], {}):
                    with self.subTest(
                        container=container_name,
                        field=field,
                        mutation=type(mutation).__name__,
                    ):
                        fixture = resource_refusal_fixture(
                            "basis_term_limit_exceeded"
                        )
                        fixture[container_name][field] = mutation
                        exit_code, receipt = _run_cli_document(fixture)
                        self.assertEqual(exit_code, 2)
                        self.assertFalse(receipt["pass"])
                        self.assertEqual(
                            receipt["disposition"], "invalid_producer_artifact"
                        )
                        self.assertIn("not a string", receipt["error"])

    def test_final_size_refusal_requires_an_actual_oversize_attempt(self) -> None:
        fixture = resource_refusal_fixture("final_json_limit_exceeded")
        fixture["resource_refusal"]["attempted_final_pretty_json_bytes"] = (
            verifier.MAX_INPUT_BYTES
        )
        fixture["evidence_limits"]["attempted_final_pretty_json_bytes"] = (
            verifier.MAX_INPUT_BYTES
        )
        fixture["evidence_limits"]["final_json_within_limit"] = True
        with self.assertRaisesRegex(verifier.VerificationError, "does not exceed"):
            self.verify(fixture)

    def test_refusal_public_input_mutation_is_invalid(self) -> None:
        fixture = resource_refusal_fixture("basis_term_limit_exceeded")
        fixture["input_system_sha256"] = "00" * 32
        with self.assertRaisesRegex(verifier.VerificationError, "SHA-256"):
            self.verify(fixture)

    def test_refusal_integer_bindings_reject_equal_valued_floats(self) -> None:
        fixture = resource_refusal_fixture("basis_term_limit_exceeded")
        fixture["p"] = float(verifier.P)
        with self.assertRaises(verifier.VerificationError):
            self.verify(fixture)
        fixture = resource_refusal_fixture("basis_term_limit_exceeded")
        fixture["public_residual"][0] = float(verifier.RESIDUALS[0][0])
        with self.assertRaises(verifier.VerificationError):
            self.verify(fixture)
        fixture = resource_refusal_fixture("basis_term_limit_exceeded")
        byte_count = fixture["input_system_canonical_json_bytes"]
        fixture["input_system_canonical_json_bytes"] = float(byte_count)
        with self.assertRaises(verifier.VerificationError):
            self.verify(fixture)

    def test_refusal_must_actually_suppress_evidence(self) -> None:
        fixture = resource_refusal_fixture("certificate_term_limit_exceeded")
        fixture["resource_refusal"]["suppressed_evidence"] = False
        with self.assertRaisesRegex(verifier.VerificationError, "did not suppress"):
            self.verify(fixture)

    def test_refusal_cannot_claim_admission(self) -> None:
        fixture = resource_refusal_fixture("basis_polynomial_limit_exceeded")
        fixture["admissible_positive"] = True
        with self.assertRaisesRegex(verifier.VerificationError, "marks a positive"):
            self.verify(fixture)

    def test_refusal_rejects_unsuppressed_bulk_field(self) -> None:
        fixture = resource_refusal_fixture("basis_term_limit_exceeded")
        fixture["basis_evidence"] = positive_fixture()["basis_evidence"]
        with self.assertRaisesRegex(verifier.VerificationError, "keys differ"):
            self.verify(fixture)

    def test_refusal_solver_completion_fields_must_be_consistent(self) -> None:
        fixture = resource_refusal_fixture("certificate_term_limit_exceeded")
        fixture["timed_out"] = True
        with self.assertRaisesRegex(verifier.VerificationError, "inconsistent"):
            self.verify(fixture)

    def test_refusal_staircase_stop_must_be_optional_usize(self) -> None:
        for mutation in ("bad", 1.5, -1):
            with self.subTest(mutation=mutation):
                fixture = resource_refusal_fixture("basis_term_limit_exceeded")
                fixture["complete_basis"] = False
                fixture["staircase_at_stop"] = mutation
                with self.assertRaisesRegex(
                    verifier.VerificationError, "staircase_at_stop"
                ):
                    self.verify(fixture)

    def test_refusal_radix_receipt_mutation_is_invalid(self) -> None:
        fixture = resource_refusal_fixture(
            "internal_recomposition_product_work_limit_exceeded"
        )
        fixture["radix_embedding"]["exact_block_rule_weight_layout_valid"] = False
        with self.assertRaisesRegex(verifier.VerificationError, "radix/open-top"):
            self.verify(fixture)


class NegativeCertificateTests(unittest.TestCase):
    @staticmethod
    def ordered_synthetic_identity() -> tuple[
        list[dict[tuple[int, ...], int]], dict[str, object]
    ]:
        x_exponent = (1,) + (0,) * (verifier.N_VARIABLES - 1)
        x_squared = (2,) + (0,) * (verifier.N_VARIABLES - 1)
        # x*x + 1*(1-x^2) = 1.  The unequal multipliers make both
        # multiplier order and original-equation order observable.
        quadrics = [
            {x_exponent: 1},
            {verifier.ZERO_EXPONENT: 1, x_squared: verifier.P - 1},
        ] + [{} for _ in range(6)]
        certificate = {
            "schema": verifier.UNIT_SCHEMA,
            "case": 1,
            "public_residual": list(verifier.RESIDUALS[1]),
            "input_system_sha256": "33" * 32,
            "multipliers": [
                [_term(list(x_exponent), 1)],
                [_term([0] * verifier.N_VARIABLES, 1)],
                [],
                [],
                [],
                [],
                [],
                [],
            ],
        }
        return quadrics, certificate

    def test_missing_original_generator_identity_is_unsupported(self) -> None:
        receipt = verifier.verify_document(
            negative_unsupported_fixture(), producer_artifact_sha256="22" * 32
        )
        self.assertFalse(receipt["pass"])
        self.assertEqual(receipt["disposition"], "certificate_unsupported")
        self.assertFalse(receipt["checks"]["negative_unit_identity_replayed"])

    def test_legacy_negative_replay_flag_requires_boolean(self) -> None:
        fixture = negative_unsupported_fixture()
        fixture["negative_unit_identity_replayed"] = 0
        with self.assertRaisesRegex(verifier.VerificationError, "not boolean"):
            verifier.verify_document(
                fixture, producer_artifact_sha256="22" * 32
            )

    def test_synthetic_unit_identity_replays_exactly(self) -> None:
        # This exercises the generic certificate algebra without asserting a
        # certificate for either registered negative public system.
        quadrics, certificate = self.ordered_synthetic_identity()
        self.assertEqual(
            verifier.verify_unit_identity(
                certificate,
                case=1,
                input_sha256="33" * 32,
                quadrics=quadrics,
            ),
            (2, 2, 3),
        )

    def test_internal_recomposition_work_is_bounded_before_expansion(self) -> None:
        # range is O(1) storage, so this exercises the count gate without
        # materializing a multi-million-term certificate in the test process.
        multipliers = [range(verifier.MAX_CERTIFICATE_TERMS)]
        quadrics = [range(3)]
        with self.assertRaisesRegex(verifier.VerificationError, "product-work"):
            verifier._certificate_product_work(multipliers, quadrics)

    def test_certificate_exponent_above_degree_seven_is_rejected(self) -> None:
        quadrics, certificate = self.ordered_synthetic_identity()
        certificate["multipliers"][0][0]["exponents"][0] = 8
        with self.assertRaisesRegex(verifier.VerificationError, "degree bound"):
            verifier.verify_unit_identity(
                certificate,
                case=1,
                input_sha256="33" * 32,
                quadrics=quadrics,
            )

    def test_certificate_residual_rejects_equal_valued_float(self) -> None:
        quadrics, certificate = self.ordered_synthetic_identity()
        certificate["public_residual"][0] = float(verifier.RESIDUALS[1][0])
        with self.assertRaisesRegex(verifier.VerificationError, "not an integer"):
            verifier.verify_unit_identity(
                certificate,
                case=1,
                input_sha256="33" * 32,
                quadrics=quadrics,
            )

    def test_multiplier_order_mutation_is_rejected(self) -> None:
        quadrics, certificate = self.ordered_synthetic_identity()
        certificate["multipliers"][0], certificate["multipliers"][1] = (
            certificate["multipliers"][1],
            certificate["multipliers"][0],
        )
        with self.assertRaisesRegex(verifier.ReplayError, "exactly one"):
            verifier.verify_unit_identity(
                certificate,
                case=1,
                input_sha256="33" * 32,
                quadrics=quadrics,
            )

    def test_original_equation_order_mutation_is_rejected(self) -> None:
        quadrics, certificate = self.ordered_synthetic_identity()
        quadrics[0], quadrics[1] = quadrics[1], quadrics[0]
        with self.assertRaisesRegex(verifier.ReplayError, "exactly one"):
            verifier.verify_unit_identity(
                certificate,
                case=1,
                input_sha256="33" * 32,
                quadrics=quadrics,
            )

    def test_zero_identity_for_public_negative_is_rejected(self) -> None:
        fixture = negative_unsupported_fixture()
        fixture["negative_unit_identity"] = {
            "schema": verifier.UNIT_SCHEMA,
            "case": 1,
            "public_residual": list(verifier.RESIDUALS[1]),
            "input_system_sha256": fixture["input_system_sha256"],
            "multipliers": [[] for _ in range(8)],
        }
        fixture["negative_certificate"] = {
            "schema": verifier.NEGATIVE_STATUS_SCHEMA,
            "available": True,
            "replayed": True,
            "internal_trace_available": True,
            "internal_trace_kept_rows": 1,
            "blocker": None,
        }
        fixture["status"] = "admissible"
        fixture["outcome"] = "negative_verified"
        fixture["admissible_negative"] = True
        with self.assertRaisesRegex(verifier.ReplayError, "exactly one"):
            verifier.verify_document(
                fixture, producer_artifact_sha256="44" * 32
            )


if __name__ == "__main__":
    unittest.main()

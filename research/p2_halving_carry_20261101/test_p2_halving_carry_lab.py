import unittest

import p2_halving_carry_lab as lab


class P2HalvingCarryTests(unittest.TestCase):
    def test_frozen_predicate_family_has_exactly_twenty_four_members(self):
        self.assertEqual(len(lab.PREDICATES), 24)
        self.assertEqual(len({item["id"] for item in lab.PREDICATES}), 24)
        self.assertEqual({item["offset"] for item in lab.PREDICATES}, {-1, 0, 1})

    def test_canonical_carry_and_torsion_zero_control(self):
        curve = lab.CURVES[0]
        target = curve.mul(37, curve.first_point())
        value, record, candidate_counter, control_counter = lab.canonical_and_torsion_record(curve, target)
        self.assertIsInstance(value, int)
        self.assertTrue(0 <= value < curve.p)
        self.assertEqual(record["torsion_carry"], 0)
        self.assertTrue(record["canonical"]["kernel_reduces_to_identity"])
        self.assertEqual(candidate_counter.formal_extractions, 1)
        self.assertGreater(control_counter.formal_extractions, 1)

    def test_public_half_encodes_even_and_wrapped_odd_integer_halves(self):
        curve = lab.CURVES[0]
        generator = curve.first_point()
        for scalar in range(1, curve.r):
            target = curve.mul(scalar, generator)
            half = curve.mul((curve.r + 1) // 2, target)
            expected = scalar // 2 if scalar % 2 == 0 else (scalar + curve.r) // 2
            self.assertEqual(half, curve.mul(expected, generator))
            self.assertEqual(curve.add(half, half), target)

    def test_predicate_complements_are_pointwise_complementary(self):
        curve = lab.CURVES[0]
        for offset in (-1, 0, 1):
            for kind in ("zero", "integer_lsb", "centered", "legendre"):
                zero = next(x for x in lab.PREDICATES if x["offset"] == offset and x["kind"] == kind and x["complement"] == 0)
                one = next(x for x in lab.PREDICATES if x["offset"] == offset and x["kind"] == kind and x["complement"] == 1)
                for value in range(curve.p):
                    self.assertEqual(
                        lab.predicate_bit(value, curve.p, zero) + lab.predicate_bit(value, curve.p, one),
                        1,
                    )

    def test_matched_labels_preserve_negation_complement(self):
        curve = lab.CURVES[0]
        labels = lab.matched_parity_labels(curve, 3)
        self.assertEqual(len(labels), curve.r - 1)
        for scalar in range(1, curve.r):
            self.assertEqual(labels[scalar - 1] + labels[curve.r - scalar - 1], 1)

    def test_exact_synthetic_parity_oracle_drives_complete_decoder(self):
        curve = lab.CURVES[0]
        generator = curve.first_point()
        original_carry = lab.carry_value
        original_spec = lab.spec_by_id
        original_bit = lab.predicate_bit
        scalar_by_point = {curve.mul(k, generator): k for k in range(1, curve.r)}
        try:
            lab.carry_value = lambda _curve, target, _counter: scalar_by_point[target] & 1
            lab.spec_by_id = lambda _predicate_id: {"synthetic": True}
            lab.predicate_bit = lambda value, _prime, _spec: value
            for scalar in range(1, curve.r):
                target = curve.mul(scalar, generator)
                decoded = lab.decode_target(curve, generator, target, "synthetic")
                self.assertTrue(decoded["verified"])
                self.assertEqual(decoded["recovered_scalar"], scalar)
        finally:
            lab.carry_value = original_carry
            lab.spec_by_id = original_spec
            lab.predicate_bit = original_bit

    def test_quick_protocol_executes_without_controls(self):
        result = lab.run_experiment(
            curves=lab.CURVES[:1],
            multipliers=lab.GENERATOR_MULTIPLIERS[:1],
            control_replicates=False,
        )
        self.assertEqual(result["summary"]["targets"], 82)
        self.assertEqual(result["summary"]["predicate_count"], 24)
        self.assertTrue(result["summary"]["all_structural_checks_passed"])
        self.assertTrue(result["summary"]["all_torsion_carries_zero"])
        self.assertEqual(result["matched_controls"], {})
        self.assertEqual(result["scientific_payload_sha256"], lab.scientific_payload_sha256(result))


if __name__ == "__main__":
    unittest.main()

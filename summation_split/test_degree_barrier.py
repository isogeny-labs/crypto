"""Regression tests for the Semaev signed-fiber/lex-degree barrier."""
import csv
import os
import tempfile
import unittest

from .degree_barrier import CELLS, check_case, run


class LexDegreeBarrierTest(unittest.TestCase):
    def test_smallest_admissible_field_witness(self):
        data = check_case(5, 1, 1, (0, 3, 4))
        self.assertEqual(data['degree'], 4)
        self.assertEqual(data['root_count'], 4)
        self.assertEqual(data['distinct_signed_states'], 8)
        self.assertTrue(data['coefficients_equal_product'])

    def test_50_complete_resultant_certificates_per_cell(self):
        with tempfile.TemporaryDirectory() as out:
            rows = run(out, samples=50)
            self.assertEqual(len(rows), len(CELLS)*50)
            with open(os.path.join(out, 'degree_barrier_gate.csv'), newline='') as stream:
                actual = list(csv.DictReader(stream))
            self.assertEqual(len(actual), 400)
            for row in actual:
                self.assertEqual(row['exact_polynomial_factorization'], 'True')
                self.assertEqual(row['lex_elimination_degree_lower_bound'],
                                 row['full_resultant_X_degree'])
                self.assertEqual(row['d_F4'], '')
                self.assertEqual(row['first_fall_degree'], '')

    def test_gate_cannot_be_reduced(self):
        with tempfile.TemporaryDirectory() as out:
            with self.assertRaises(ValueError):
                run(out, samples=49)


if __name__ == '__main__':
    unittest.main()

"""Algebraic regression tests, including independent Gröbner-basis checks."""
import tempfile
import unittest
from pathlib import Path

from degree_trace import (Row, analyze, ec_system, groebner_reference,
                          hilbert_trace, load_system, trace_solve, verify_proof)


class DegreeTraceTests(unittest.TestCase):
    def setUp(self):
        self.p = 101
        # x^2-y=0, xy-1=0. A cubic computation discovers y^2-x=0.
        self.f = [{(2, 0): 1, (0, 1): 100}, {(1, 1): 1, (0, 0): 100}]
        self.g = {(0, 2): 1, (1, 0): 100}

    def test_redundant_consequence_reduces_observed_degree(self):
        reference = groebner_reference(self.f, ['x', 'y'], self.p)
        original, _ = trace_solve(self.f, self.p, reference, 5)
        augmented, _ = trace_solve(self.f+[self.g], self.p, reference, 5)
        self.assertEqual(original['solving_degree'], 3)
        self.assertEqual(augmented['solving_degree'], 2)
        # g = x*f_1-y*f_0, a certified redundant equation.
        verify_proof(Row(self.g, {(1, (1, 0)): 1, (0, (0, 1)): 100}), self.f, self.p)

    def test_hilbert_artinian_and_nonartinian(self):
        # S/(x^2,y^2) has Hilbert function 1,2,1,0, hence d_reg=3.
        h = hilbert_trace([{(2, 0): 1}, {(0, 2): 1}], 101, 5)
        self.assertEqual([t['hilbert_function'] for t in h['trace']], [1, 2, 1, 0])
        self.assertEqual(h['top_form_d_reg'], 3)
        not_artinian = hilbert_trace(self.f, 101, 5)
        self.assertIsNone(not_artinian['top_form_d_reg'])
        self.assertTrue(all(t['hilbert_function'] > 0 for t in not_artinian['trace']))

    def test_cap_reports_incomplete_instead_of_guessing_degree(self):
        ref = groebner_reference(self.f, ['x', 'y'], 101)
        result, _ = trace_solve(self.f, 101, ref, 2)
        self.assertEqual(result['status'], 'degree_cap')
        self.assertIsNone(result['solving_degree'])

    def test_column_budget_is_checked_before_matrix_generation(self):
        ref = groebner_reference(self.f, ['x', 'y'], 101)
        result, _ = trace_solve(self.f, 101, ref, 3, max_columns=2)
        self.assertEqual(result['status'], 'column_budget')

    def test_composite_modulus_rejected(self):
        with self.assertRaises(ValueError):
            analyze(self.f, ['x', 'y'], 2**53)

    def test_three_variable_certificate(self):
        f = [{(1, 0, 0): 1}, {(0, 1, 0): 1}, {(0, 0, 1): 1}]
        ref = groebner_reference(f, ['x', 'y', 'z'], 101)
        run, archive = trace_solve(f, 101, ref, 3)
        self.assertEqual(run['solving_degree'], 1)
        for _, row in archive:
            verify_proof(row, f, 101)

    def test_large_prime_ec_ablations_preserve_ideal(self):
        for p in [2**31-1, 9007199254740881]:
            f, variables, _ = ec_system(p)
            report = analyze(f, variables, p, 6, 4)
            self.assertEqual(report['baseline']['solving_degree'], 5)
            self.assertEqual(report['greedy_final']['solving_degree'], 4)
            self.assertEqual(report['end_to_end_peak_degree'], 5)
            self.assertEqual(report['greedy_final']['top_forms']['top_form_d_reg'], 4)
            self.assertTrue(report['end_to_end_complete'])

    def test_invalid_import_exponents(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory)/'invalid.json'
            path.write_text('{"prime":101,"variables":["x"],"polynomials":[[{"exponents":[-1],"coefficient":1}]]}')
            with self.assertRaises(ValueError):
                load_system(path)


if __name__ == '__main__':
    unittest.main()

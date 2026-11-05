"""Synthetic stage-accounting checks; no cryptographic measurements."""
import unittest
import cold_analyze as c
from test_analyze import panel, raw, HASH

class ColdTests(unittest.TestCase):
    def test_setup_cost_can_reverse_query_verdict(self):
        rows = panel()
        rows[2]['counts'][1][0][0] += 20
        result = c.analyze(raw(rows), HASH, 0)
        self.assertEqual(result['query_analysis']['verdict'], 'counted_pareto_improvement')
        self.assertEqual(result['verdict'], 'counted_pareto_regression')
        for pair in result['pairs']:
            self.assertEqual(pair['delta'], dict(field_products=19, field_inversions=0))
        self.assertEqual(result['pairs'][0]['baseline']['field_products'], 18)
        self.assertEqual(result['setup_prepare']['candidate']['field_products'], 26)

    def test_setup_inversions_and_all_stage_rejection(self):
        rows = panel()
        rows[2]['counts'][1][4][1] = 2
        result = c.analyze(raw(rows), HASH, 0)
        self.assertEqual(result['verdict'], 'counted_tradeoff_or_mixed')
        self.assertEqual(result['pairs'][0]['weighted_model']['candidate_better'], 'w < numerator/denominator')
        self.assertEqual(result['pairs'][0]['weighted_model']['numerator'], 1)
        self.assertEqual(result['pairs'][0]['weighted_model']['denominator'], 2)
        rows[3]['counts'][1][0][0] += 1
        with self.assertRaises(c.Invalid): c.analyze(raw(rows), HASH, 0)

if __name__ == '__main__': unittest.main()

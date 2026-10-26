"""Guard the declared finite grid and its source-pinned admission gate."""

import importlib.util
import json
from pathlib import Path
import unittest


HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("n83_full_mask_census", HERE / "run.py")
census = importlib.util.module_from_spec(spec)
spec.loader.exec_module(census)
analysis_spec = importlib.util.spec_from_file_location("n83_full_mask_analysis", HERE / "analyze.py")
analysis = importlib.util.module_from_spec(analysis_spec)
analysis_spec.loader.exec_module(analysis)
domain_spec = importlib.util.spec_from_file_location("n83_full_mask_domain", HERE / "domain_screen.py")
domain = importlib.util.module_from_spec(domain_spec)
domain_spec.loader.exec_module(domain)
pair_spec = importlib.util.spec_from_file_location("n83_full_mask_pair", HERE / "pair_screen.py")
pair = importlib.util.module_from_spec(pair_spec)
pair_spec.loader.exec_module(pair)
replay_spec = importlib.util.spec_from_file_location("n83_full_mask_replay", HERE / "audit_replay.py")
replay = importlib.util.module_from_spec(replay_spec)
replay_spec.loader.exec_module(replay)
support_spec = importlib.util.spec_from_file_location("n83_full_mask_support", HERE / "support_screen.py")
support = importlib.util.module_from_spec(support_spec)
support_spec.loader.exec_module(support)


class CensusTest(unittest.TestCase):
    def setUp(self):
        self.protocol = json.loads((HERE / "protocol.json").read_bytes())

    def test_every_pattern_and_parent_formula_is_addressed_once(self):
        rows = list(census.specs(self.protocol))
        self.assertEqual(len(rows), 192)
        self.assertEqual(len({row["name"] for row in rows}), 192)
        for trial in (0, 1, 2):
            for pool in ("hash", "gray"):
                for small, residual in ((5, 1), (4, 2)):
                    masks = {row["identity_mask"] for row in rows
                             if (row["trial"], row["pool"],
                                 row["base_summands"], row["residual_summands"])
                             == (trial, pool, small, residual)}
                    self.assertEqual(masks, set(range(16)))
        parent = json.loads(census.PARENT.read_bytes())
        self.assertTrue(set(parent["formulas"]).issubset({row["name"] for row in rows}))

    def test_clipped_or_duplicate_mask_grid_fails_closed(self):
        for masks in (list(range(15)), list(range(15)) + [14]):
            altered = dict(self.protocol, identity_masks=masks)
            with self.assertRaisesRegex(ValueError, "frozen full identity-mask grid"):
                list(census.specs(altered))

    def test_adjacent_identity_bits_are_structurally_impossible(self):
        possible = {mask for mask in range(16) if analysis.status(mask)
                    == "NO_UNCONDITIONAL_IDENTITY_CONTRADICTION"}
        self.assertEqual(possible, {0, 1, 2, 4, 5, 8, 9, 10})
        self.assertEqual({mask for mask in range(16) if analysis.status(mask)
                          == "UNSAT_BY_ADJACENT_IDENTITIES"},
                         {3, 6, 7, 11, 12, 13, 14, 15})

    def test_final_identity_requires_target_in_last_pool(self):
        without_target = {mask for mask in range(16) if domain.classify(mask, False)
                          == "NO_PROVEN_IMPOSSIBILITY_FROM_THESE_GATES"}
        self.assertEqual(without_target, {0, 1, 2, 4, 5})
        with_target = {mask for mask in range(16) if domain.classify(mask, True)
                       == "NO_PROVEN_IMPOSSIBILITY_FROM_THESE_GATES"}
        self.assertEqual(with_target, {0, 1, 2, 4, 5, 8, 9, 10})

    def test_independent_pair_group_arithmetic(self):
        from chain_search_supervisor import PRIMARY_GENERATOR, point_add
        generator = PRIMARY_GENERATOR
        self.assertIsNone(point_add(generator, pair.negate(generator)))
        doubled = point_add(generator, generator)
        self.assertEqual(pair.pair_hits(doubled, {generator}, {generator}), 1)
        self.assertEqual(pair.pair_hits(doubled, {pair.negate(generator)},
                                        {generator}), 0)

    def test_repeat_row_tamper_is_rejected(self):
        first = json.loads((HERE / "census.json").read_bytes())
        second = json.loads((HERE / "census-repeat.json").read_bytes())
        replay.compare_rows(first, second)
        second["rows"][17]["sha256"] = "0" * 64
        with self.assertRaisesRegex(ValueError, "exact deterministic export replay"):
            replay.compare_rows(first, second)

    def test_prefix_cancellation_reduces_the_exact_support_ceiling(self):
        sizes = [support.count_sums(11, 7, 4, 2, mask) for mask in (0, 1, 2)]
        self.assertEqual(sizes, [28028, 1848, 308])
        self.assertGreater(sizes[0], sizes[1])
        self.assertGreater(sizes[1], sizes[2])


if __name__ == "__main__":
    unittest.main()

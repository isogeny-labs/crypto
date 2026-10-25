"""Guard the declared finite grid and its source-pinned admission gate."""

import importlib.util
import json
from pathlib import Path
import unittest


HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("n83_full_mask_census", HERE / "run.py")
census = importlib.util.module_from_spec(spec)
spec.loader.exec_module(census)


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


if __name__ == "__main__":
    unittest.main()

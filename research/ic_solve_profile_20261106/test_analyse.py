"""Self-check of analyse.py on a SYNTHETIC fixture (invented numbers, not a measurement)."""
import io, json, os, tempfile, unittest
import analyse


class T(unittest.TestCase):
    def test_shares_and_ceiling(self):
        rows = []
        for i, (a, b) in enumerate([(60, 40), (50, 50), (70, 30)]):
            rows.append({"arm": "x", "warmup": False,
                         "phases": [{"name": "collect", "wall_ns": a}, {"name": "la", "wall_ns": b},
                                    {"name": "descent", "wall_ns": None}]})
        rows.append({"arm": "x", "warmup": True, "phases": [{"name": "collect", "wall_ns": 1}]})
        with tempfile.NamedTemporaryFile("w", suffix=".jsonl", delete=False) as f:
            f.write("\n".join(json.dumps(r) for r in rows))
        try:
            runs, un = analyse.load([f.name])
        finally:
            os.unlink(f.name)
        s = analyse.summarise(runs, un)["x"]
        self.assertEqual(s["runs"], 3)
        self.assertAlmostEqual(s["phases"]["collect"]["share"], 0.6)
        self.assertAlmostEqual(s["phases"]["collect"]["amdahl_ceiling"], 2.5)
        self.assertAlmostEqual(s["phases"]["collect"]["sd_share"], 0.1)
        self.assertEqual(s["unclocked"], ["descent"])


if __name__ == "__main__":
    unittest.main()

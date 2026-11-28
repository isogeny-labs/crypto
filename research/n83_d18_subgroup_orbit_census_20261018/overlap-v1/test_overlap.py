"""Structural corruption, independent replay and support-count controls."""

from __future__ import annotations

from itertools import combinations_with_replacement, product
import gzip
import hashlib
import json
from pathlib import Path
import shutil
import tempfile
import unittest
from unittest.mock import patch

import audit
import overlap


def fixture(path: Path, arm: str, starts: tuple[int, ...], *, change: str = "") -> str:
    policy = overlap.OBJECTS[arm]["policy"]
    seed = overlap.OBJECTS[arm]["seed"]
    header = {"schema": "n83.factor-base/v1", "curve_slug": "icv1-f2m83-tm6151469093347-debefd74",
              "curve_a": 0, "curve_b": "1", "cofactor": "4", "n": 83,
              "modulus_low_terms": [0, 1, 2, 45], "closure": "signed_frobenius",
              "subgroup_order": str(overlap.ORDER), "orbit_columns": len(starts),
              "point_count": 166 * len(starts), "policy": policy, "seed": seed,
              "frobenius_lambda": "2", "generator": ["1", "2"],
              "field_encoding": "synthetic-structural-test"}
    with gzip.open(path, "wb") as out:
        out.write((json.dumps(header) + "\n").encode())
        for col, start in enumerate(starts):
            for phase, negative in product(range(83), (False, True)):
                x, y = start + phase, 1000 + start * 2 + phase * 2 + int(negative)
                if change == "duplicate_label" and col == 0 and phase == 1 and not negative:
                    phase = 0
                if change == "duplicate_point" and col == 0 and phase == 1 and not negative:
                    x, y = start, 1000 + start * 2
                row = {"column": col, "phase": phase, "negative": negative,
                       "coefficient": str(((-1 if negative else 1)
                                           * pow(2, phase, overlap.ORDER)) % overlap.ORDER),
                       "point": [str(x), str(y)]}
                out.write((json.dumps(row, separators=(",", ":")) + "\n").encode())
    return hashlib.sha256(path.read_bytes()).hexdigest()


class OverlapTest(unittest.TestCase):
    def test_support_matches_explicit_multisets(self) -> None:
        u, d = {0, 1, 2}, {1, 2, 3}
        for a, b in ((1, 1), (2, 1), (2, 2), (3, 2)):
            explicit = {tuple(sorted(left + right))
                        for left in combinations_with_replacement(sorted(u), a)
                        for right in combinations_with_replacement(sorted(d), b)}
            self.assertEqual(overlap.mixed_support(1, 1, 2, a, b), len(explicit))
        self.assertEqual(overlap.mixed_support(0, 0, 3, 2, 2), overlap.multichoose(3, 4))

    def test_two_archive_independent_replay_and_forged_count(self) -> None:
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            u_path, d_path = root / "u.gz", root / "d.gz"
            u_hash = fixture(u_path, "U", (10, 200))
            d_hash = fixture(d_path, "D", (10, 400))
            shutil.copy2(Path(overlap.__file__), root / "overlap.py")
            with patch.object(overlap, "K", 2), patch.object(audit, "K", 2), \
                 patch.dict(overlap.OBJECTS["U"], sha256=u_hash), \
                 patch.dict(overlap.OBJECTS["D"], sha256=d_hash), \
                 patch.dict(audit.PINNED, {"U": (u_hash, "public_x_hash", 2026100801),
                                             "D": (d_hash, "direct_subgroup_x_d18", 0)}):
                result = overlap.make(u_path, d_path)
                self.assertEqual(result["shared_orbits"], 1)
                self.assertEqual(result["union_orbits"], 3)
                receipt = root / "receipt.json"
                receipt.write_text(json.dumps(result))
                independent = audit.audit(u_path, d_path, receipt)
                self.assertEqual(independent["status"], "PASS")
                result["shared_orbits"] += 1
                receipt.write_text(json.dumps(result))
                with self.assertRaisesRegex(ValueError, "forged producer shared_orbits"):
                    audit.audit(u_path, d_path, receipt)

    def test_corruption_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            for change in ("duplicate_label", "duplicate_point"):
                path = root / f"{change}.gz"
                digest = fixture(path, "U", (10,), change=change)
                with self.assertRaises(ValueError):
                    overlap.parse_archive(path, arm="U", k=1, expected_sha256=digest)
                with self.assertRaises(ValueError):
                    audit.replay_archive(path, "U", k=1, expected_sha256=digest)
            good = root / "good.gz"
            fixture(good, "U", (10,))
            with self.assertRaisesRegex(ValueError, "SHA-256 differs"):
                overlap.parse_archive(good, arm="U", k=1)
            with self.assertRaisesRegex(ValueError, "SHA-256 differs"):
                audit.replay_archive(good, "U", k=1)


if __name__ == "__main__":
    unittest.main()

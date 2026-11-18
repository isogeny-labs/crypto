#!/usr/bin/env python3
import importlib.util
import hashlib
import math
import sys
import tempfile
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("ghs_analyze", HERE / "analyze.py")
assert SPEC and SPEC.loader
A = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = A
SPEC.loader.exec_module(A)


def seal(directory: Path) -> None:
    lines = []
    for path in sorted(path for path in directory.iterdir() if path.name != "SHA256SUMS"):
        digest = hashlib.sha256(path.read_bytes()).hexdigest()
        lines.append(f"{digest}  {path.name}")
    (directory / "SHA256SUMS").write_text("\n".join(lines) + "\n", encoding="utf-8")


def split_fixture(root: Path, split: str, binary_digest: str | None = None) -> Path:
    directory = root / split
    directory.mkdir()
    binary = directory / "ghs-strong-rho"
    binary.write_bytes(f"binary-{split}".encode())
    actual_binary_digest = hashlib.sha256(binary.read_bytes()).hexdigest()
    (directory / "host.txt").write_text(f"split={split}\n", encoding="utf-8")
    (directory / "source_revision.txt").write_text("a" * 40 + "\n", encoding="utf-8")
    source_rows = {
        "PROTOCOL.md": A.PROTOCOL_SHA256,
        "main.rs": A.MAIN_SHA256,
        "strong_rho.rs": A.STRONG_SHA256,
        "pod_run.sh": A.POD_RUN_SHA256,
    }
    (directory / "source.sha256").write_text(
        "".join(
            f"{digest}  /opt/crypto/research/quartic_ghs_strong_rho_20261114/{name}\n"
            for name, digest in source_rows.items()
        ),
        encoding="utf-8",
    )
    (directory / "binary.sha256").write_text(
        f"{binary_digest or actual_binary_digest}  /work/results/ghs-strong-rho\n",
        encoding="utf-8",
    )
    (directory / "selftest.out").write_text("SELFTEST OK\n", encoding="utf-8")
    exits = [f"seed={seed} rc=0" for seed in A.SEEDS]
    if split == "low":
        exits.append("replay=match")
    exits.append("finish=2026-11-15T00:00:00Z rc=0")
    (directory / "exit_codes.txt").write_text("\n".join(exits) + "\n", encoding="utf-8")
    seal(directory)
    return directory


class AnalysisTests(unittest.TestCase):
    def test_split_manifests_require_exact_coverage(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            directories = [split_fixture(root, split) for split in ("low", "mid", "high", "top")]
            audit = A.verify_sha_manifests(root)
            self.assertEqual(audit["manifests"], 4)

            (directories[0] / "unlisted.txt").write_text("not sealed\n", encoding="utf-8")
            with self.assertRaisesRegex(A.Invalid, "coverage differ"):
                A.verify_sha_manifests(root)

    def test_recorded_binary_digest_must_match_retrieved_binary(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            split_fixture(root, "low", "0" * 64)
            for split in ("mid", "high", "top"):
                split_fixture(root, split)
            with self.assertRaisesRegex(A.Invalid, "recorded binary hash mismatch"):
                A.verify_sha_manifests(root)

    def test_component_pricing(self) -> None:
        component = A.Components(mul=100, inv=2, lmul=3, band=4)
        self.assertEqual(component.price(10.0, 2.0, 0.0), 126.0)
        self.assertEqual(component.price(10.0, 2.0, 1.0), 390.0)

    def test_clustered_ratio_is_deterministic_and_contains_point(self) -> None:
        ic = {1: 10.0, 2: 12.0, 3: 14.0}
        rho = {1: 20.0, 2: 24.0, 3: 28.0}
        first = A.ratio_interval(ic, rho, 4051, 0.1)
        second = A.ratio_interval(ic, rho, 4051, 0.1)
        self.assertEqual(first, second)
        self.assertAlmostEqual(first[0], 0.5)
        self.assertLessEqual(first[1], first[0])
        self.assertGreaterEqual(first[2], first[0])

    def test_strong_count_identity(self) -> None:
        row = {}
        for prefix in ("setup", "walk", "verify"):
            row[f"{prefix}.fq_mul"] = 1
            row[f"{prefix}.fq_inv"] = 2
        for prefix in ("setup_native", "walk_native"):
            row[f"{prefix}.canon"] = 1
            row[f"{prefix}.lookahead_test"] = 2
            row[f"{prefix}.dp"] = 3
            row[f"{prefix}.table"] = 4
            row[f"{prefix}.cycle_cmp"] = 5
            row[f"{prefix}.band_ops"] = 15
            row[f"{prefix}.scalar_mul"] = 6
        total, setup, walk, verify = A.strong_components(row)
        self.assertEqual((total.mul, total.inv, total.lmul, total.band), (3, 6, 12, 30))
        self.assertEqual(setup.band, 15)
        self.assertEqual(walk.band, 15)
        self.assertEqual(verify.band, 0)

    def test_bad_band_identity_is_rejected(self) -> None:
        row = {}
        for prefix in ("setup", "walk", "verify"):
            row[f"{prefix}.fq_mul"] = 0
            row[f"{prefix}.fq_inv"] = 0
        for prefix in ("setup_native", "walk_native"):
            for field in ("canon", "lookahead_test", "dp", "table", "cycle_cmp", "scalar_mul"):
                row[f"{prefix}.{field}"] = 0
            row[f"{prefix}.band_ops"] = 0
        row["walk_native.band_ops"] = 1
        with self.assertRaisesRegex(A.Invalid, "band identity"):
            A.strong_components(row)

    def test_non_authoritative_rho_counts_are_rejected(self) -> None:
        with self.assertRaisesRegex(A.Invalid, "non-authoritative rho field counts"):
            A.parse_rho_record(
                {"rec": "rho", "arm": "fast-b8", "count_authoritative": False},
                q=223,
                seed=1,
                target=17,
                ell=101,
                arm="fast-b8",
                expected_index=0,
            )

    def test_grid_and_repetitions_are_frozen(self) -> None:
        self.assertEqual(len(A.GRID), 11)
        self.assertEqual(A.GRID[-1], 4194271)
        self.assertEqual(A.BOOTSTRAPS, 10_000)
        self.assertTrue(math.isclose(math.sqrt(18446308672764918653), 4_294_916_608.0, rel_tol=1e-6))

    def test_h1_verdict_uses_only_admitted_sizes(self) -> None:
        sizes = [
            {"q": 1, "admitted": True, "weights": [{"ci95": [0.5, 0.8]}]},
            {"q": 2, "admitted": False, "weights": [{"ci95": [1.1, 1.2]}]},
        ]
        verdict = A.classify_h1(sizes)
        self.assertEqual(verdict["verdict"], "limited_unadmitted_sizes")
        self.assertEqual(verdict["admitted_sizes"], [1])
        self.assertEqual(verdict["unadmitted_sizes"], [2])
        self.assertTrue(verdict["all_admitted_upper_bounds_below_one"])

    def test_h1_admitted_failure_precedes_limited_status(self) -> None:
        unresolved = [
            {"q": 1, "admitted": True, "weights": [{"ci95": [0.8, 1.1]}]},
            {"q": 2, "admitted": False, "weights": [{"ci95": [0.2, 0.3]}]},
        ]
        rejected = [
            {"q": 1, "admitted": True, "weights": [{"ci95": [1.01, 1.2]}]},
            {"q": 2, "admitted": False, "weights": [{"ci95": [0.2, 0.3]}]},
        ]
        self.assertEqual(A.classify_h1(unresolved)["verdict"], "unresolved_interval_crosses_one")
        self.assertEqual(A.classify_h1(rejected)["verdict"], "rejected")


if __name__ == "__main__":
    unittest.main()

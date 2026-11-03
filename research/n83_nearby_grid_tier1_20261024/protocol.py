"""Frozen 19-base current-source N83 Tier 1 screening protocol."""

import hashlib
import json
from pathlib import Path

from research.n83_online_finalists_20261018.generate_candidates import canonical_id

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
GRID = ROOT / "research/n83_nearby_full_grid_20261024"
CATALOG = GRID / "catalog.json"
CATALOG_SHA256 = "d03c8fd394d83f3433c7f7c75a18ee9941177941a1737f459771d1d7b0d450a7"
TARGETS = HERE / "target.jsonl"
ANSWERS = HERE / "answer.jsonl"
WORKLOAD = HERE / "workload.json"
FIXTURE_MANIFEST = HERE / "fixture_manifest.json"
EXCLUSION_AUDIT = HERE / "exclusion_audit.json"
GENERATOR = HERE / "generate_fixture.py"
TARGET_DOMAIN = "n83-nearby-grid-tier1-screen-20261024-v1"
RHO_CONFIG = ROOT / "research/n83_dense_root_20261013/rho_config.json"
SOURCE = ROOT / "examples/koblitz_orbit_dlp_fast.rs"
RHO_SOURCE = ROOT / "examples/n83_rho_optimization_eval.rs"
SOURCE_SHA256 = "58e3ee492093de5f6b760b96e3aba819fa479cdecd22b1adee94f0ecb26026d8"
RHO_SOURCE_SHA256 = "fb14072f4fa5d08e18d8b79130b6d58a220adae0f76369cc9be949691c83eaa0"
LOCK_SHA256 = "ac5cd7a3459be776180afe1eb2975c76b1a88f84bd1e99aa20475927ec260a90"
TARGETS_SHA256 = "1fcaa14edaa8bfeb185b8cd1e254ea5774cedbd27082573bd865ff2f42a94f52"
ANSWERS_SHA256 = "f09019049ebb186985519e81298b6f58e1967dff5ef2c6e4f1c28e50e1334d64"
FIXTURE_MANIFEST_SHA256 = "9468441faec2eda49d8b41b21f90cd5f413ddb56509dfd7c5143d6ee39b3e96a"
EXCLUSION_AUDIT_SHA256 = "8987b1ef6beb2c6f16d2c9eeac7ca8c6809f616177ce8ddb3e658ac1361a9a34"
WORKLOAD_SHA256 = "fa1f3061c40e9b55a88b8235873b735a651bfead8a74649896cd3e35741a2181"
GENERATOR_SHA256 = "455bdf7ae7edd1861247ae64b281973580332ed2c24d7c999a5dfe7d1c395493"
RHO_CONFIG_SHA256 = "cf76ebb5372acfc1355b88374f875980be66cdffba1a82cb6e4c2f28bcfa723b"
WORKLOAD_ID = "052f3657ce24"
ONE_TARGET_SHA256 = TARGETS_SHA256
BASELINE_K = 1107
ROUNDS = 2
BLOCK_SIZE = 3
IC_CAP_SECONDS = 900
RHO_CAP_SECONDS = 120
GLOBAL_CAP_SECONDS = 240 * 60
RSS_CAP_BYTES = 4 * 1024**3
MIN_MEMORY_BYTES = 5 * 1024**3
MIN_DISK_BYTES = 4 * 1024**3
RHO_SEED_BASE = 2026122400
AA_SPREAD_LIMIT = 0.20


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def candidates():
    if digest(CATALOG) != CATALOG_SHA256:
        raise ValueError("nearby-grid catalog differs")
    catalog = json.loads(CATALOG.read_text())
    if (catalog["status"] != "PASS_DERIVED_PREFIX_IDENTITIES" or
            catalog["source_sha256"] != SOURCE_SHA256 or
            catalog["candidate_count"] != 19 or
            catalog["anchor_ks"] != [1104, 1107, 1120]):
        raise ValueError("nearby-grid catalog is not the frozen source")
    result = {}
    for row in catalog["candidates"]:
        k = row["k"]
        path = GRID / "identities" / f"candidate_{row['candidate_id']}.json"
        manifest = json.loads(path.read_text())
        if (manifest["candidate_id"] != row["candidate_id"] or
                canonical_id(manifest) != row["candidate_id"] or
                manifest["implementation"]["source_sha256"]["ic"] !=
                SOURCE_SHA256 or
                manifest["factor_base"]["actual_usable_points"] != 166 * k or
                manifest["factor_base"]["effective_columns"] != k or
                manifest["factor_base"]["ordered_base_dump_sha256"] !=
                row["ordered_base_dump_sha256"] or
                manifest["factor_base"]["source_base_hash_blake3"] !=
                row["source_base_hash_blake3"]):
            raise ValueError(f"K{k} exact candidate differs")
        result[k] = (row["candidate_id"], digest(path))
    if sorted(result) != list(range(1102, 1121)):
        raise ValueError("nearby-grid K coverage differs")
    return result


CANDIDATES = candidates()


def events():
    """Opposite-order sweeps in short, independently controlled blocks."""
    rows = []
    challengers = [k for k in CANDIDATES if k != BASELINE_K]
    blocks_per_round = (len(challengers) + BLOCK_SIZE - 1) // BLOCK_SIZE
    for round_index in range(ROUNDS):
        order = challengers if round_index % 2 == 0 else list(reversed(challengers))
        for block_index in range(0, len(order), BLOCK_SIZE):
            block = block_index // BLOCK_SIZE
            prefix = f"r{round_index:02d}b{block:02d}"
            seed = RHO_SEED_BASE + round_index * blocks_per_round + block
            tags = {"round": round_index, "block": block}
            rows.append({"name": f"{prefix}_rho_before", "kind": "rho",
                         **tags, "seed": seed})
            rows.append({"name": f"{prefix}_baseline_before", "kind": "ic",
                         **tags, "k": BASELINE_K,
                         "candidate_id": CANDIDATES[BASELINE_K][0]})
            for k in order[block_index:block_index + BLOCK_SIZE]:
                rows.append({"name": f"{prefix}_k{k}", "kind": "ic",
                             **tags, "k": k,
                             "candidate_id": CANDIDATES[k][0]})
            rows.append({"name": f"{prefix}_baseline_after", "kind": "ic",
                         **tags, "k": BASELINE_K,
                         "candidate_id": CANDIDATES[BASELINE_K][0]})
            rows.append({"name": f"{prefix}_rho_after", "kind": "rho",
                         **tags, "seed": seed})
    return rows

"""Frozen current-source nearby-K one-target Tier 1 protocol."""

from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
NEARBY = ROOT / "research/n83_nearby_k_current_source_20261021"
FRESH = ROOT / "research/n83_highk_beyond4405_20261021"
TARGETS = FRESH / "holdout_targets.jsonl"
ANSWERS = FRESH / "holdout_answers.jsonl"
RHO_CONFIG = ROOT / "research/n83_dense_root_20261013/rho_config.json"
SOURCE = ROOT / "examples/koblitz_orbit_dlp_fast.rs"
RHO_SOURCE = ROOT / "examples/n83_rho_optimization_eval.rs"
SOURCE_SHA256 = "58e3ee492093de5f6b760b96e3aba819fa479cdecd22b1adee94f0ecb26026d8"
RHO_SOURCE_SHA256 = "fb14072f4fa5d08e18d8b79130b6d58a220adae0f76369cc9be949691c83eaa0"
LOCK_SHA256 = "ac5cd7a3459be776180afe1eb2975c76b1a88f84bd1e99aa20475927ec260a90"
TARGETS_SHA256 = "a0e1f76ddec6ae901d5e766a942ad0b35299e10f175243fe7abd0fc679040a96"
ANSWERS_SHA256 = "624701aae9a27ce1efc27f72c53005f71f719072b04aa911eca98803e58ad02e"
RHO_CONFIG_SHA256 = "cf76ebb5372acfc1355b88374f875980be66cdffba1a82cb6e4c2f28bcfa723b"
WORKLOAD_ID = "f3f41cbb7544"
ONE_TARGET_SHA256 = "3140614ffb4de3a9006be2f8b7570df8921005db7f95979e236a908937303a36"
CANDIDATES = {
    1104: ("IC1N83Ckb1a1fb183264PDP4rootRCguidedLAgaussTDdirectISO0h2233626bba52",
           "3bd01dbd6a064992f476b01ab127a16f7ac86bd0f068e85daf28b12e170eac50"),
    1107: ("IC1N83Ckb1a1fb183762PDP4rootRCguidedLAgaussTDdirectISO0hf1851768e85a",
           "67cbd5d4f12ab5bfe703714ab66f0c779b5d708aaa79a6399b13d4217b4f9176"),
    1120: ("IC1N83Ckb1a1fb185920PDP4rootRCguidedLAgaussTDdirectISO0hc21c54dc0a94",
           "9359d4dc0e93c3c51d6a00b6bd70c3d45a13bffbb92edd62d180bede876f2639"),
}
ROUNDS = 5
IC_CAP_SECONDS = 900
RHO_CAP_SECONDS = 120
GLOBAL_CAP_SECONDS = 120 * 60
RSS_CAP_BYTES = 4 * 1024**3
MIN_MEMORY_BYTES = 5 * 1024**3
MIN_DISK_BYTES = 4 * 1024**3
RHO_SEED_BASE = 2026122100


def events():
    """Two same-point rho controls and two K1107 A/A controls per round."""
    rows = []
    for round_index in range(ROUNDS):
        prefix = f"r{round_index:02d}"
        rows.append({"name": f"{prefix}_rho_before", "kind": "rho",
                     "round": round_index, "seed": RHO_SEED_BASE + 2 * round_index})
        order = (1104, 1120) if round_index % 2 == 0 else (1120, 1104)
        for name, k in (("baseline_before", 1107), (f"k{order[0]}", order[0]),
                        (f"k{order[1]}", order[1]), ("baseline_after", 1107)):
            rows.append({"name": f"{prefix}_{name}", "kind": "ic",
                         "round": round_index, "k": k,
                         "candidate_id": CANDIDATES[k][0]})
        rows.append({"name": f"{prefix}_rho_after", "kind": "rho",
                     "round": round_index, "seed": RHO_SEED_BASE + 2 * round_index + 1})
    return rows

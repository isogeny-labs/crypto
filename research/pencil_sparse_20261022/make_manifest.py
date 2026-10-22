"""Index immutable run artifacts and their SHA-256 checksums; no arithmetic."""

import hashlib
import json
import platform
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parent
RESULTS = ROOT / "results"
ARCHIVE = ROOT.parent / "five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst"
ONLINE_BINARY_SHA256 = "40820a6179e903b2482c1eefc7c088222e0d1e1b3ce260a72aa6901c220d742b"
PEEL_BINARY_SHA256 = "539144816c83d765da6ea5731f34b5a2d3a8bd75519d02721342f56c797ce6a8"


def digest(path):
    sha = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            sha.update(block)
    return sha.hexdigest()


validated = json.loads((RESULTS / "validation.json").read_text())
repo_root = ROOT.parent.parent
native_files = sorted((file for file in validated["files"]
                       if file["path"].startswith(
                           "research/pencil_sparse_20261022/results/p")),
                      key=lambda file: file["path"])
records = []
for native in native_files:
    path = repo_root / native["path"]
    stem = path.with_suffix("")
    exit_file = stem.with_suffix(".exit")
    replay = repo_root / native["replay"] if native["replay"] else None
    mode = native["solver_mode"]
    pre_peel_online_p28 = native["bits"] == 28 and mode == "online"
    run_binary = (digest(ROOT / "target/release/pencil-sparse")
                  if "cap25m" in path.name else
                  PEEL_BINARY_SHA256 if mode == "peel" else ONLINE_BINARY_SHA256)
    replay_binary = (digest(ROOT / "target/release/pencil-sparse")
                     if "cap25m" in path.name else
                     PEEL_BINARY_SHA256 if mode == "peel" or pre_peel_online_p28
                     else ONLINE_BINARY_SHA256)
    records.append({
        "raw_path": str(path.relative_to(ROOT)),
        "raw_sha256": digest(path),
        "setup_bits": native["bits"],
        "setup_p": native["p"],
        "solver_mode": mode,
        "run_binary_sha256": run_binary,
        "target_rows": native["targets"],
        "verified_candidate_rows": native["verified_candidate_rows"],
        "failed_candidate_rows": native["failed_candidate_rows"],
        "failure_codes": native["failure_codes"],
        "process_exit": int(exit_file.read_text().strip()) if exit_file.exists() else None,
        "replay_path": str(replay.relative_to(ROOT)) if replay else None,
        "replay_binary_sha256": replay_binary if replay else None,
        "replay_equivalence": native["replay_equivalence"],
        "replay_sha256": digest(replay) if replay else None,
        "replay_rows": native["replay_rows"],
    })

files = sorted(path for path in ROOT.rglob("*")
               if path.is_file() and "target" not in path.parts
               and "__pycache__" not in path.parts and path.suffix != ".pyc"
               and path != RESULTS / "manifest.json")
manifest = {
    "schema": "pencil-sparse-manifest/v1",
    "generated_utc": datetime.now(timezone.utc).isoformat(),
    "host": platform.node(),
    "machine": platform.machine(),
    "archive_path": str(ARCHIVE.relative_to(ROOT.parent.parent)),
    "archive_sha256": digest(ARCHIVE),
    "current_binary_sha256": digest(ROOT / "target/release/pencil-sparse"),
    "records": records,
    "artifact_sha256": {str(path.relative_to(ROOT)): digest(path) for path in files},
}
previous_path = RESULTS / "manifest.json"
if previous_path.exists():
    previous = json.loads(previous_path.read_text())
    if "amendment" in previous:
        manifest["amendment"] = previous["amendment"]
    manifest["extension_source_manifest_sha256"] = previous.get(
        "extension_source_manifest_sha256", digest(previous_path))
(RESULTS / "manifest.json").write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n")

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


def digest(path):
    sha = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            sha.update(block)
    return sha.hexdigest()


raw_files = sorted(path for path in RESULTS.glob("p*-*.jsonl"))
records = []
for path in raw_files:
    rows = [json.loads(line) for line in path.read_text().splitlines()]
    assert rows[0]["schema"] == "pencil-cohort-setup/v1"
    stem = path.with_suffix("")
    exit_file = stem.with_suffix(".exit")
    replay = RESULTS / "replay" / path.name
    records.append({
        "raw_path": str(path.relative_to(ROOT)),
        "raw_sha256": digest(path),
        "setup_bits": rows[0]["bits"],
        "setup_p": rows[0]["p"],
        "solver_mode": rows[1].get("solver_mode", "online") if len(rows) > 1 else None,
        "run_binary_sha256": (digest(ROOT / "target/release/pencil-sparse")
                              if len(rows) > 1 and rows[1].get("solver_mode") == "peel"
                              else ONLINE_BINARY_SHA256),
        "target_rows": len(rows) - 1,
        "verified_candidate_rows": sum(row["verified"] is True for row in rows[1:]),
        "failed_candidate_rows": sum(row["verified"] is not True for row in rows[1:]),
        "failure_codes": sorted({row["failure"] for row in rows[1:] if row["failure"]}),
        "process_exit": int(exit_file.read_text().strip()) if exit_file.exists() else None,
        "replay_path": str(replay.relative_to(ROOT)) if replay.exists() else None,
        "replay_sha256": digest(replay) if replay.exists() else None,
        "replay_rows": len(replay.read_text().splitlines()) - 1 if replay.exists() else None,
    })

files = sorted(path for path in ROOT.rglob("*")
               if path.is_file() and "target" not in path.parts
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
(RESULTS / "manifest.json").write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n")

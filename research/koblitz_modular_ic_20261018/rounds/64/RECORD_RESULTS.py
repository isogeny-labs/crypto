"""Record exact terminal receipts and historical byte pins; no field algebra."""
from datetime import datetime
from pathlib import Path
import hashlib
import json
import re
import subprocess

h = Path(__file__).resolve().parent
repo = h.parents[3]
rows = []
pins = []

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

for raw in sorted((h / "runs").iterdir()):
    if not raw.is_dir():
        continue
    revision = (raw / "source-revision.txt").read_text().strip()
    code = int((raw / "exit.txt").read_text())
    log = (raw / "native.log").read_text()
    errors = [line for line in log.splitlines() if re.match(r"^\s*\?", line)]
    marker = "BRIDGE64_ALL_CHECKS_PASS"
    accepted = code == 0 and not errors and marker in log.splitlines()
    start = (raw / "started-utc.txt").read_text().strip()
    finish = (raw / "finished-utc.txt").read_text().strip()
    results = [line for line in log.splitlines() if line.startswith("BRIDGE64_RESULT ")]
    progress = [line for line in log.splitlines() if line.startswith("BRIDGE64_BASIS ")]
    mapping = {"input.sing": "BRIDGE64.sing", "driver.sh": "NATIVE_DRIVER.sh",
               "launcher.sh": "REPLAY_CAS.sh", "PROTOCOL.md": "PROTOCOL.md",
               "protocol.json": "protocol.json", "donors.json": "donors.json"}
    for local, source in mapping.items():
        git_path = str((h / source).relative_to(repo))
        old = subprocess.check_output(["git", "show", f"{revision}:{git_path}"], cwd=repo)
        assert old == (raw / local).read_bytes(), (raw.name, local)
        pins.append({"attempt": raw.name, "artifact": local, "git_source": git_path,
                     "revision": revision, "sha256": digest(raw / local)})
    rows.append({"attempt": raw.name, "source_revision": revision,
                 "kind": "Singular", "status": "accepted" if accepted else "rejected",
                 "native_exit": code, "marker": marker,
                 "marker_present": marker in log.splitlines(),
                 "error_count": len(errors), "first_errors": errors[:10],
                 "result_lines_accepted": accepted, "result_lines": results,
                 "basis_progress_count": len(progress),
                 "outcome": "completed" if accepted else "native_error" if errors else "time_cap" if code == 124 else "missing_marker",
                 "started_utc": start, "finished_utc": finish,
                 "native_wall_seconds": (datetime.fromisoformat(finish.replace("Z", "+00:00"))
                                         - datetime.fromisoformat(start.replace("Z", "+00:00"))).total_seconds(),
                 "artifacts": [{"path": str(path.relative_to(h)), "sha256": digest(path),
                                "bytes": path.stat().st_size}
                               for path in sorted(raw.iterdir()) if path.is_file()]})

assert [r["attempt"] for r in rows] == [f"bridge64-0{i}" for i in range(1, 5)]
assert [r["status"] for r in rows] == ["rejected"] * 3 + ["accepted"]
assert all(r["error_count"] > 0 for r in rows[:3])
assert rows[3]["basis_progress_count"] == 32
assert len(rows[3]["result_lines"]) == 1
fields = dict(part.split("=", 1) for part in rows[3]["result_lines"][0].split()[1:])
assert fields == {"actual_degree": "32", "old_source_equations": "2",
                  "old_basis_functions": "32", "image_rank": "32",
                  "both_inverse_products": "2", "changed_quadratic_rejected": "1",
                  "changed_image_rejected": "1"}
assert (h / "runs/bridge64-04/bridge-basis64.sing").exists()
results = {"schema": "koblitz.results/v1", "round": 64,
           "runs": rows, "input_pins": pins,
           "protocol_metadata_disclosure": "Four historical versions are preserved in immutable raw copies; attempts01-03 rejected despite late markers, attempt04 accepted."}
(h / "results.json").write_text(json.dumps(results, indent=2) + "\n")
protocols = []
for row in rows:
    path = h / "runs" / row["attempt"] / "protocol.json"
    protocols.append({"attempt": row["attempt"], "path": str(path.relative_to(h)),
                      "sha256": digest(path), "frozen_revision": row["source_revision"]})
export = {"schema": "experiment.export/v1", "experiment_id": "KOBLITZ-R64-BRIDGE",
          "claim_class": "exact_computation", "protocols": protocols,
          "raw_archive": "runs.tar.xz; content address in .artifacts.json after custody publication",
          "results": {"path": "results.json", "sha256": digest(h / "results.json")},
          "flow": {status: sum(row["status"] == status for row in rows)
                   for status in ("accepted", "rejected", "interrupted")},
          "canonical_schema_validation": "unavailable in selected Git index and disk; manual byte and contract checks only",
          "notes": "Actual degree32 source bridge and full32 basis isomorphism inputs checked. Integral closure, proper normalization and valuations are next experiments."}
(h / "export.json").write_text(json.dumps(export, indent=2) + "\n")
print(json.dumps({"attempts": len(rows), **export["flow"], "historical_byte_pins": len(pins)}))

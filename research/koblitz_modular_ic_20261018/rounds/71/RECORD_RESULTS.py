"""Record strict R71 native acceptance and historical source-byte custody."""
from datetime import datetime
from pathlib import Path
import hashlib, json, re, subprocess

h = Path(__file__).resolve().parent
repo = h.parents[3]
raw = h / "runs/rational-fibers71-01"
assert [p.name for p in (h / "runs").iterdir() if p.is_dir()] == [raw.name]
rev = (raw / "source-revision.txt").read_text().strip()
code = int((raw / "exit.txt").read_text())
log = (raw / "native.log").read_text()
errors = [s for s in log.splitlines() if re.match(r"^\s*\?", s)]
marker = "RATIONAL_FIBERS71_ALL_CHECKS_PASS"
results = [s for s in log.splitlines() if s.startswith("FIBER71_RESULT ")]
degrees = [s for s in log.splitlines() if s.startswith("FIBER71_DEGREE ")]
assert code == 0 and not errors and marker in log.splitlines()
assert len(results) == 2 and len(degrees) == 2
parse = lambda s: dict(x.split("=", 1) for x in s.split()[1:])
assert [parse(s) for s in results] == [
    {"fiber": "0", "algebra_dimension": "32", "stable_steps": "5", "reduced_dimension": "4", "branches": "2"},
    {"fiber": "1", "algebra_dimension": "32", "stable_steps": "4", "reduced_dimension": "8", "branches": "4"},
]
assert [parse(s) for s in degrees] == [
    {"fiber": "0", "degree": "2", "multiplicity": "2"},
    {"fiber": "1", "degree": "2", "multiplicity": "4"},
]
pins = []
for local, source in {"input.sing": "RATIONAL_FIBERS71.sing", "driver.sh": "NATIVE_DRIVER.sh", "launcher.sh": "REPLAY_CAS.sh", "PROTOCOL.md": "PROTOCOL.md", "protocol.json": "protocol.json", "donors.json": "donors.json"}.items():
    path = str((h / source).relative_to(repo))
    actual = (raw / local).read_bytes()
    assert subprocess.check_output(["git", "show", rev + ":" + path], cwd=repo) == actual
    pins.append({"artifact": local, "git_source": path, "revision": rev, "sha256": hashlib.sha256(actual).hexdigest()})
for row in json.loads((raw / "donors.json").read_text()):
    p = h / row["path"]
    assert p.stat().st_size == row["bytes"] and hashlib.sha256(p.read_bytes()).hexdigest() == row["sha256"]
assert (raw / "rational-fibers71.sing").exists()
start = (raw / "started-utc.txt").read_text().strip()
finish = (raw / "finished-utc.txt").read_text().strip()
digest = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
row = {"attempt": raw.name, "source_revision": rev, "kind": "Singular", "status": "accepted",
       "native_exit": code, "marker": marker, "marker_present": True, "error_count": 0,
       "result_lines": results, "degree_lines": degrees, "started_utc": start, "finished_utc": finish,
       "native_wall_seconds": (datetime.fromisoformat(finish.replace("Z", "+00:00")) - datetime.fromisoformat(start.replace("Z", "+00:00"))).total_seconds(),
       "artifacts": [{"path": str(p.relative_to(h)), "sha256": digest(p), "bytes": p.stat().st_size}
                     for p in sorted(raw.iterdir()) if p.is_file()]}
result = {"schema": "koblitz.results/v1", "round": 71, "runs": [row], "input_pins": pins,
          "donors_checked": len(json.loads((raw / "donors.json").read_text()))}
(h / "results.json").write_text(json.dumps(result, indent=2) + "\n")
export = {"schema": "experiment.export/v1", "experiment_id": "KOBLITZ-R71-RATIONAL-FIBER-RESIDUES",
          "claim_class": "exact_computation",
          "protocols": [{"attempt": raw.name, "path": str((raw / "protocol.json").relative_to(h)),
                         "sha256": digest(raw / "protocol.json"), "frozen_revision": rev}],
          "raw_archive": "runs.tar.xz; content address in .artifacts.json after custody publication",
          "results": {"path": "results.json", "sha256": digest(h / "results.json")},
          "flow": {"accepted": 1, "rejected": 0, "interrupted": 0},
          "canonical_schema_validation": "unavailable in selected Git index and disk; manual contract/source-byte checks only",
          "notes": "Both rational fibers: stable reduced algebras F4^2 and F4^4. Ramification and valuations remain uncomputed."}
(h / "export.json").write_text(json.dumps(export, indent=2) + "\n")
print(json.dumps({"accepted": 1, "historical_byte_pins": len(pins), "donors_checked": result["donors_checked"], "fibers": results}))

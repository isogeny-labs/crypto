"""Record strict R72 acceptance, all branch factors, and historical source bytes."""
from datetime import datetime
from pathlib import Path
import hashlib, json, re, subprocess

h = Path(__file__).resolve().parent
repo = h.parents[3]
raw = h / "runs/rational-local72-01"
assert [p.name for p in (h / "runs").iterdir() if p.is_dir()] == [raw.name]
rev = (raw / "source-revision.txt").read_text().strip()
code = int((raw / "exit.txt").read_text())
log = (raw / "native.log").read_text()
errors = [s for s in log.splitlines() if re.match(r"^\s*\?", s)]
marker = "RATIONAL_LOCAL72_ALL_CHECKS_PASS"
results = [s for s in log.splitlines() if s.startswith("LOCAL72_RESULT ")]
factors = [s for s in log.splitlines() if s.startswith("LOCAL72_FACTOR ")]
assert code == 0 and not errors and marker in log.splitlines()
parse = lambda s: dict(x.split("=", 1) for x in s.split()[1:])
assert [parse(s) for s in results] == [
    {"fiber": "0", "algebra_dimension": "32", "reduced_dimension": "4", "branches": "2", "nonzero_idempotents": "3", "primitive_count": "2", "local_length_sum": "32", "ramification_sum": "16"},
    {"fiber": "1", "algebra_dimension": "32", "reduced_dimension": "8", "branches": "4", "nonzero_idempotents": "15", "primitive_count": "4", "local_length_sum": "32", "ramification_sum": "16"},
]
assert [parse(s) for s in factors] == [
    {"fiber": "0", "mask": "1", "residue_degree": "2", "local_length": "28", "ramification": "14"},
    {"fiber": "0", "mask": "3", "residue_degree": "2", "local_length": "4", "ramification": "2"},
    {"fiber": "1", "mask": "6", "residue_degree": "2", "local_length": "2", "ramification": "1"},
    {"fiber": "1", "mask": "7", "residue_degree": "2", "local_length": "14", "ramification": "7"},
    {"fiber": "1", "mask": "8", "residue_degree": "2", "local_length": "2", "ramification": "1"},
    {"fiber": "1", "mask": "10", "residue_degree": "2", "local_length": "14", "ramification": "7"},
]
pins = []
for local, source in {"input.sing": "RATIONAL_LOCAL72.sing", "driver.sh": "NATIVE_DRIVER.sh", "launcher.sh": "REPLAY_CAS.sh", "PROTOCOL.md": "PROTOCOL.md", "protocol.json": "protocol.json", "donors.json": "donors.json"}.items():
    path = str((h / source).relative_to(repo))
    actual = (raw / local).read_bytes()
    assert subprocess.check_output(["git", "show", rev + ":" + path], cwd=repo) == actual
    pins.append({"artifact": local, "git_source": path, "revision": rev, "sha256": hashlib.sha256(actual).hexdigest()})
for row in json.loads((raw / "donors.json").read_text()):
    p = h / row["path"]
    assert p.stat().st_size == row["bytes"] and hashlib.sha256(p.read_bytes()).hexdigest() == row["sha256"]
assert (raw / "rational-local72.sing").exists()
start = (raw / "started-utc.txt").read_text().strip()
finish = (raw / "finished-utc.txt").read_text().strip()
digest = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
row = {"attempt": raw.name, "source_revision": rev, "kind": "Singular", "status": "accepted",
       "native_exit": code, "marker": marker, "marker_present": True, "error_count": 0,
       "result_lines": results, "factor_lines": factors, "started_utc": start, "finished_utc": finish,
       "native_wall_seconds": (datetime.fromisoformat(finish.replace("Z", "+00:00")) - datetime.fromisoformat(start.replace("Z", "+00:00"))).total_seconds(),
       "artifacts": [{"path": str(p.relative_to(h)), "sha256": digest(p), "bytes": p.stat().st_size}
                     for p in sorted(raw.iterdir()) if p.is_file()]}
result = {"schema": "koblitz.results/v1", "round": 72, "runs": [row], "input_pins": pins,
          "donors_checked": len(json.loads((raw / "donors.json").read_text()))}
(h / "results.json").write_text(json.dumps(result, indent=2) + "\n")
export = {"schema": "experiment.export/v1", "experiment_id": "KOBLITZ-R72-RATIONAL-FIBER-RAMIFICATION",
          "claim_class": "exact_computation",
          "protocols": [{"attempt": raw.name, "path": str((raw / "protocol.json").relative_to(h)),
                         "sha256": digest(raw / "protocol.json"), "frozen_revision": rev}],
          "raw_archive": "runs.tar.xz; content address in .artifacts.json after custody publication",
          "results": {"path": "results.json", "sha256": digest(h / "results.json")},
          "flow": {"accepted": 1, "rejected": 0, "interrupted": 0},
          "canonical_schema_validation": "unavailable in selected Git index and disk; manual contract/source-byte checks only",
          "notes": "Complete rational-fiber local lengths and ramification: z0 e14,2; z1 e1,7,1,7, all f2. Uniformizers and valuations remain uncomputed."}
(h / "export.json").write_text(json.dumps(export, indent=2) + "\n")
print(json.dumps({"accepted": 1, "historical_byte_pins": len(pins), "donors_checked": result["donors_checked"], "factors": factors}))

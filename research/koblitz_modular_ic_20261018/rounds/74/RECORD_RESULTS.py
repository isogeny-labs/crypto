"""Record strict R74 acceptance, rejected late marker and source-byte pins."""
from datetime import datetime
from pathlib import Path
import hashlib, json, re, subprocess

h = Path(__file__).resolve().parent
repo = h.parents[3]
names = ["degree-five-local74-01", "degree-five-local74-02"]
assert sorted(p.name for p in (h / "runs").iterdir() if p.is_dir()) == sorted(names)
digest = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
rows, pins = [], []
for name in names:
    raw = h / "runs" / name
    rev = (raw / "source-revision.txt").read_text().strip()
    code = int((raw / "exit.txt").read_text())
    log = (raw / "native.log").read_text()
    errors = [s for s in log.splitlines() if re.match(r"^\s*\?", s)]
    marker = "DEGREE_FIVE_LOCAL74_ALL_CHECKS_PASS"
    results = [s for s in log.splitlines() if s.startswith("LOCAL74_RESULT ")]
    factors = [s for s in log.splitlines() if s.startswith("LOCAL74_FACTOR ")]
    accepted = code == 0 and not errors and marker in log.splitlines() and len(results) == 1 and len(factors) == 6
    for local, source in {"input.sing": "DEGREE_FIVE_LOCAL74.sing", "driver.sh": "NATIVE_DRIVER.sh", "launcher.sh": "REPLAY_CAS.sh", "PROTOCOL.md": "PROTOCOL.md", "protocol.json": "protocol.json", "donors.json": "donors.json"}.items():
        path = str((h / source).resolve().relative_to(repo))
        actual = (raw / local).read_bytes()
        assert subprocess.check_output(["git", "show", rev + ":" + path], cwd=repo) == actual
        pins.append({"attempt": name, "artifact": local, "git_source": path,
                     "revision": rev, "sha256": digest(raw / local)})
    for donor in json.loads((raw / "donors.json").read_text()):
        p = h / donor["path"]
        assert p.stat().st_size == donor["bytes"] and digest(p) == donor["sha256"]
    start = (raw / "started-utc.txt").read_text().strip()
    finish = (raw / "finished-utc.txt").read_text().strip()
    rows.append({"attempt": name, "source_revision": rev, "kind": "Singular",
                 "status": "accepted" if accepted else "rejected", "native_exit": code,
                 "marker": marker, "marker_present": marker in log.splitlines(),
                 "error_count": len(errors), "first_errors": errors[:8],
                 "result_lines": results, "factor_lines": factors,
                 "started_utc": start, "finished_utc": finish,
                 "native_wall_seconds": (datetime.fromisoformat(finish.replace("Z", "+00:00")) - datetime.fromisoformat(start.replace("Z", "+00:00"))).total_seconds(),
                 "artifacts": [{"path": str(p.relative_to(h)), "sha256": digest(p), "bytes": p.stat().st_size}
                               for p in sorted(raw.iterdir()) if p.is_file()]})
assert [r["status"] for r in rows] == ["rejected", "accepted"]
assert rows[0]["native_exit"] == 0 and rows[0]["marker_present"] and rows[0]["error_count"] > 0
assert rows[1]["native_exit"] == 0 and rows[1]["error_count"] == 0
parse = lambda s: dict(x.split("=", 1) for x in s.split()[1:])
assert parse(rows[1]["result_lines"][0]) == {
    "base_degree": "5", "fiber_dimension": "32", "relative_fixed_dimension": "6",
    "absolute_fixed_dimension": "6", "nonzero_idempotents": "63",
    "primitive_count": "6", "local_length_sum": "32", "different_budget": "12"}
assert [parse(s) for s in rows[1]["factor_lines"]] == [
    {"mask": "14", "residue_degree": "2", "absolute_degree": "10", "local_length": "2", "ramification": "1"},
    {"mask": "15", "residue_degree": "14", "absolute_degree": "70", "local_length": "14", "ramification": "1"},
    {"mask": "17", "residue_degree": "1", "absolute_degree": "5", "local_length": "1", "ramification": "1"},
    {"mask": "22", "residue_degree": "1", "absolute_degree": "5", "local_length": "7", "ramification": "7"},
    {"mask": "48", "residue_degree": "1", "absolute_degree": "5", "local_length": "7", "ramification": "7"},
    {"mask": "52", "residue_degree": "1", "absolute_degree": "5", "local_length": "1", "ramification": "1"},
]
assert (h / "runs" / names[1] / "degree-five-local74.sing").exists()
result = {"schema": "koblitz.results/v1", "round": 74, "runs": rows,
          "input_pins": pins,
          "protocol_metadata_disclosure": "Version2 intmat-entry assignment and integer-div repair frozen after rejected late-marker attempt 01 and before fresh attempt 02."}
(h / "results.json").write_text(json.dumps(result, indent=2) + "\n")
export = {"schema": "experiment.export/v1", "experiment_id": "KOBLITZ-R74-DEGREE-FIVE-LOCAL-FACTORS",
          "claim_class": "exact_computation",
          "protocols": [{"attempt": r["attempt"], "path": "runs/" + r["attempt"] + "/protocol.json",
                         "sha256": digest(h / "runs" / r["attempt"] / "protocol.json"),
                         "frozen_revision": r["source_revision"]} for r in rows],
          "raw_archive": "runs.tar.xz; content address in .artifacts.json after custody publication",
          "results": {"path": "results.json", "sha256": digest(h / "results.json")},
          "flow": {"accepted": 1, "rejected": 1, "interrupted": 0},
          "canonical_schema_validation": "unavailable in selected Git index and disk; manual contract/source-byte checks only",
          "notes": "All six degree-five local factors: lengths 2,14,1,7,7,1 and ramification 1,1,1,7,7,1 with relative degrees 2,14,1,1,1,1. Valuations and infinity separate."}
(h / "export.json").write_text(json.dumps(export, indent=2) + "\n")
print(json.dumps({"attempts": 2, "accepted": 1, "rejected": 1,
                  "historical_byte_pins": len(pins), "factors": rows[1]["factor_lines"]}))

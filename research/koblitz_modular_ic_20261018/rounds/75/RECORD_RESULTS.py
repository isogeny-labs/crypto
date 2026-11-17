"""Record both full R75 native attempts and their historical source bytes."""
from datetime import datetime
from pathlib import Path
import hashlib, json, re, subprocess

h = Path(__file__).resolve().parent
repo = h.parents[3]
specs = [
    ("rational-uniform75-01", "RATIONAL_UNIFORM75.sing", "RATIONAL_UNIFORM75_ALL_CHECKS_PASS", "RAT75_RESULT ", "RAT75_FACTOR "),
    ("five-uniform75-01", "FIVE_UNIFORM75.sing", "FIVE_UNIFORM75_ALL_CHECKS_PASS", "FIVE75_RESULT ", "FIVE75_FACTOR "),
]
assert sorted(p.name for p in (h / "runs").iterdir() if p.is_dir()) == sorted(x[0] for x in specs)
digest = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
parse = lambda s: dict(x.split("=", 1) for x in s.split()[1:])
rows, pins = [], []
for name, source_input, marker, result_prefix, factor_prefix in specs:
    raw = h / "runs" / name
    rev = (raw / "source-revision.txt").read_text().strip()
    code = int((raw / "exit.txt").read_text())
    log = (raw / "native.log").read_text()
    errors = [s for s in log.splitlines() if re.match(r"^\s*\?", s)]
    results = [s for s in log.splitlines() if s.startswith(result_prefix)]
    factors = [s for s in log.splitlines() if s.startswith(factor_prefix)]
    accepted = code == 0 and not errors and marker in log.splitlines() and len(results) == 1 and len(factors) == 6
    assert accepted, name
    for local, source in {"input.sing": source_input, "driver.sh": "NATIVE_DRIVER.sh", "launcher.sh": "REPLAY_CAS.sh", "PROTOCOL.md": "PROTOCOL.md", "protocol.json": "protocol.json", "donors.json": "donors.json"}.items():
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
                 "status": "accepted", "native_exit": code, "marker": marker,
                 "marker_present": True, "error_count": 0,
                 "result_lines": results, "factor_lines": factors,
                 "started_utc": start, "finished_utc": finish,
                 "native_wall_seconds": (datetime.fromisoformat(finish.replace("Z", "+00:00")) - datetime.fromisoformat(start.replace("Z", "+00:00"))).total_seconds(),
                 "artifacts": [{"path": str(p.relative_to(h)), "sha256": digest(p), "bytes": p.stat().st_size}
                               for p in sorted(raw.iterdir()) if p.is_file()]})
assert parse(rows[0]["result_lines"][0]) == {"branches": "6", "ramified": "4", "unramified": "2", "candidates": "128", "all_nilpotency_checks": "1"}
assert parse(rows[1]["result_lines"][0]) == {"branches": "6", "ramified": "2", "unramified": "4", "candidates": "64", "all_nilpotency_checks": "1"}
assert [parse(s) for s in rows[0]["factor_lines"]] == [
    {"fiber": "0", "mask": "1", "e": "14", "residue_degree": "2", "first_basis": "5", "successes": "9", "candidates": "32"},
    {"fiber": "0", "mask": "3", "e": "2", "residue_degree": "2", "first_basis": "1", "successes": "16", "candidates": "32"},
    {"fiber": "1", "mask": "6", "e": "1", "residue_degree": "2", "parameter": "base_prime"},
    {"fiber": "1", "mask": "7", "e": "7", "residue_degree": "2", "first_basis": "5", "successes": "11", "candidates": "32"},
    {"fiber": "1", "mask": "8", "e": "1", "residue_degree": "2", "parameter": "base_prime"},
    {"fiber": "1", "mask": "10", "e": "7", "residue_degree": "2", "first_basis": "17", "successes": "10", "candidates": "32"},
]
assert [parse(s) for s in rows[1]["factor_lines"]] == [
    {"mask": "14", "e": "1", "residue_degree": "2", "parameter": "base_prime"},
    {"mask": "15", "e": "1", "residue_degree": "14", "parameter": "base_prime"},
    {"mask": "17", "e": "1", "residue_degree": "1", "parameter": "base_prime"},
    {"mask": "22", "e": "7", "residue_degree": "1", "first_basis": "1", "successes": "26", "candidates": "32"},
    {"mask": "48", "e": "7", "residue_degree": "1", "first_basis": "1", "successes": "25", "candidates": "32"},
    {"mask": "52", "e": "1", "residue_degree": "1", "parameter": "base_prime"},
]
assert (h / "runs" / specs[0][0] / "rational-uniform75.sing").exists()
assert (h / "runs" / specs[1][0] / "five-uniform75.sing").exists()
result = {"schema": "koblitz.results/v1", "round": 75, "runs": rows, "input_pins": pins,
          "candidate_total": 192, "ramified_branch_count": 6, "unramified_branch_count": 6}
(h / "results.json").write_text(json.dumps(result, indent=2) + "\n")
export = {"schema": "experiment.export/v1", "experiment_id": "KOBLITZ-R75-FINITE-UNIFORMIZER-CLASSES",
          "claim_class": "exact_computation",
          "protocols": [{"attempt": r["attempt"], "path": "runs/" + r["attempt"] + "/protocol.json",
                         "sha256": digest(h / "runs" / r["attempt"] / "protocol.json"),
                         "frozen_revision": r["source_revision"]} for r in rows],
          "raw_archive": "runs.tar.xz; content address in .artifacts.json after custody publication",
          "results": {"path": "results.json", "sha256": digest(h / "results.json")},
          "flow": {"accepted": 2, "rejected": 0, "interrupted": 0},
          "canonical_schema_validation": "unavailable in selected Git index and disk; manual contract/source-byte checks only",
          "notes": "Complete 192-candidate searches certify finite-fiber uniformizer classes at six ramified branches; six unramified branches use base primes. Global lifts and valuations separate."}
(h / "export.json").write_text(json.dumps(export, indent=2) + "\n")
print(json.dumps({"accepted": 2, "historical_byte_pins": len(pins), "candidate_total": 192}))

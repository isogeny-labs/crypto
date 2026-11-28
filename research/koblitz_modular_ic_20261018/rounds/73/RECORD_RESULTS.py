"""Record all R73 native and Lean attempts, including late-marker failure."""
from datetime import datetime
from pathlib import Path
import hashlib, json, re, subprocess

h = Path(__file__).resolve().parent
repo = h.parents[3]
native_names = ["degree-five73-01", "degree-five73-02"]
lean_names = ["degree-five-lean73-01", "degree-five-lean73-02", "degree-five-lean73-03"]
assert sorted(p.name for p in (h / "runs").iterdir() if p.is_dir()) == sorted(native_names + lean_names)
digest = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
rows, pins = [], []
for name in native_names:
    raw = h / "runs" / name
    rev = (raw / "source-revision.txt").read_text().strip()
    code = int((raw / "exit.txt").read_text())
    log = (raw / "native.log").read_text()
    errors = [s for s in log.splitlines() if re.match(r"^\s*\?", s)]
    marker = "DEGREE_FIVE73_ALL_CHECKS_PASS"
    results = [s for s in log.splitlines() if s.startswith("FIVE73_RESULT ")]
    degrees = [s for s in log.splitlines() if s.startswith("FIVE73_DEGREE ")]
    accepted = code == 0 and not errors and marker in log.splitlines() and len(results) == 1
    for local, source in {"input.sing": "DEGREE_FIVE73.sing", "driver.sh": "NATIVE_DRIVER.sh", "launcher.sh": "REPLAY_CAS.sh", "PROTOCOL.md": "PROTOCOL.md", "protocol.json": "protocol.json", "donors.json": "donors.json"}.items():
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
                 "result_lines": results, "degree_lines": degrees,
                 "started_utc": start, "finished_utc": finish,
                 "native_wall_seconds": (datetime.fromisoformat(finish.replace("Z", "+00:00")) - datetime.fromisoformat(start.replace("Z", "+00:00"))).total_seconds(),
                 "artifacts": [{"path": str(p.relative_to(h)), "sha256": digest(p), "bytes": p.stat().st_size}
                               for p in sorted(raw.iterdir()) if p.is_file()]})
assert [r["status"] for r in rows] == ["rejected", "accepted"]
assert rows[0]["native_exit"] == 0 and rows[0]["marker_present"] and rows[0]["error_count"] > 0
assert rows[1]["native_exit"] == 0 and rows[1]["error_count"] == 0
parse = lambda s: dict(x.split("=", 1) for x in s.split()[1:])
assert parse(rows[1]["result_lines"][0]) == {
    "base_degree": "5", "fiber_dimension_over_F32": "32", "fiber_dimension_over_F2": "160",
    "stable_steps": "2", "reduced_dimension_over_F32": "20", "branches": "6"}
assert [parse(s) for s in rows[1]["degree_lines"]] == [
    {"relative_degree": "1", "absolute_degree": "5", "multiplicity": "4"},
    {"relative_degree": "2", "absolute_degree": "10", "multiplicity": "1"},
    {"relative_degree": "14", "absolute_degree": "70", "multiplicity": "1"}]
assert (h / "runs" / native_names[1] / "degree-five73.sing").exists()
claims = {"FiveFiber73.reduced_degree_sum", "FiveFiber73.degree_fourteen_unramified",
          "FiveFiber73.rational_tame_discriminant"}
for name in lean_names:
    raw = h / "runs" / name
    rev = (raw / "source-revision.txt").read_text().strip()
    compile_code = int((raw / "compile.exit").read_text())
    audit_code = int((raw / "audit.exit").read_text()) if (raw / "audit.exit").exists() else None
    audit = (raw / "audit.log").read_text() if (raw / "audit.log").exists() else ""
    declarations = re.findall(r"^ok\s+(FiveFiber73\.\S+)", audit, re.M)
    accepted = compile_code == audit_code == 0 and claims.issubset(declarations) and "0 failing, 0 trusted" in audit
    for local, source in {"driver.sh": "LEAN_DRIVER.sh", "launcher.sh": "REPLAY_LEAN.sh",
                          "FiveFiber73.lean": "lean/FiveFiber73.lean",
                          "LEAN-PROTOCOL.md": "LEAN-PROTOCOL.md",
                          "AuditAxioms.lean": "../../../../tools/lean/AuditAxioms.lean"}.items():
        path = str((h / source).resolve().relative_to(repo))
        actual = (raw / local).read_bytes()
        assert subprocess.check_output(["git", "show", rev + ":" + path], cwd=repo) == actual
        pins.append({"attempt": name, "artifact": local, "git_source": path,
                     "revision": rev, "sha256": digest(raw / local)})
    rows.append({"attempt": name, "source_revision": rev, "kind": "Lean",
                 "status": "accepted" if accepted else "rejected",
                 "compile_exit": compile_code, "audit_exit": audit_code,
                 "audited_declarations": declarations,
                 "authored_claims": sorted(claims.intersection(declarations)),
                 "artifacts": [{"path": str(p.relative_to(h)), "sha256": digest(p), "bytes": p.stat().st_size}
                               for p in sorted(raw.iterdir()) if p.is_file()]})
assert [r["status"] for r in rows] == ["rejected", "accepted", "rejected", "rejected", "accepted"]
assert all(r["compile_exit"] != 0 and r["audit_exit"] is None for r in rows[2:4])
assert rows[4]["compile_exit"] == rows[4]["audit_exit"] == 0 and set(rows[4]["authored_claims"]) == claims
result = {"schema": "koblitz.results/v1", "round": 73, "runs": rows,
          "input_pins": pins, "authored_proved_claims": sorted(claims),
          "protocol_metadata_disclosure": "Version2 source-ring transport repair after rejected late-marker native run; Lean v2/v3 import repairs after rejected compiles. Each was frozen before a fresh attempt."}
(h / "results.json").write_text(json.dumps(result, indent=2) + "\n")
export = {"schema": "experiment.export/v1", "experiment_id": "KOBLITZ-R73-DEGREE-FIVE-FIBER-RESIDUES",
          "claim_class": "exact_computation",
          "protocols": [{"attempt": r["attempt"], "path": "runs/" + r["attempt"] + "/" + ("protocol.json" if r["kind"] == "Singular" else "LEAN-PROTOCOL.md"),
                         "sha256": digest(h / "runs" / r["attempt"] / ("protocol.json" if r["kind"] == "Singular" else "LEAN-PROTOCOL.md")),
                         "frozen_revision": r["source_revision"]} for r in rows],
          "raw_archive": "runs.tar.xz; content address in .artifacts.json after custody publication",
          "results": {"path": "results.json", "sha256": digest(h / "results.json")},
          "flow": {"accepted": 2, "rejected": 3, "interrupted": 0},
          "canonical_schema_validation": "unavailable in selected Git index and disk; manual contract/source-byte checks only",
          "notes": "Full degree-five F32 fiber: reduced relative dimension20, degrees 1x4,2,14; conditional Lean arithmetic proves degree14 branch unramified given weighted different budget. Ramification elsewhere and valuations separate."}
(h / "export.json").write_text(json.dumps(export, indent=2) + "\n")
print(json.dumps({"attempts": 5, "accepted": 2, "rejected": 3,
                  "historical_byte_pins": len(pins), "degree_profile": rows[1]["degree_lines"]}))

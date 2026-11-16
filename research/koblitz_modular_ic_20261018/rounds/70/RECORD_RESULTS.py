"""Record both R70 terminal attempts, including the rejected late marker."""
from datetime import datetime
from pathlib import Path
import hashlib, json, re, subprocess

h = Path(__file__).resolve().parent
repo = h.parents[3]
names = ["sixth-frobenius70-01", "sixth-frobenius70-02"]
lean_names = ["frobenius-lean70-01", "frobenius-lean70-02", "frobenius-lean70-03"]
assert sorted(p.name for p in (h / "runs").iterdir() if p.is_dir()) == sorted(names + lean_names)
digest = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
rows, pins = [], []
for name in names:
    raw = h / "runs" / name
    rev = (raw / "source-revision.txt").read_text().strip()
    code = int((raw / "exit.txt").read_text())
    log = (raw / "native.log").read_text()
    errors = [s for s in log.splitlines() if re.match(r"^\s*\?", s)]
    marker = "SIXTH_FROBENIUS70_ALL_CHECKS_PASS"
    lines = [s for s in log.splitlines() if s.startswith("FROB70_RESULT ")]
    accepted = code == 0 and not errors and marker in log.splitlines() and len(lines) == 1
    for local, source in {"input.sing": "SIXTH_FROBENIUS70.sing", "driver.sh": "NATIVE_DRIVER.sh", "launcher.sh": "REPLAY_CAS.sh", "PROTOCOL.md": "PROTOCOL.md", "protocol.json": "protocol.json", "donors.json": "donors.json"}.items():
        path = str((h / source).resolve().relative_to(repo))
        actual = (raw / local).read_bytes()
        assert subprocess.check_output(["git", "show", rev + ":" + path], cwd=repo) == actual
        pins.append({"attempt": name, "artifact": local, "git_source": path,
                     "revision": rev, "sha256": hashlib.sha256(actual).hexdigest()})
    for donor in json.loads((raw / "donors.json").read_text()):
        p = h / donor["path"]
        assert p.stat().st_size == donor["bytes"] and digest(p) == donor["sha256"]
    start = (raw / "started-utc.txt").read_text().strip()
    finish = (raw / "finished-utc.txt").read_text().strip()
    rows.append({"attempt": name, "source_revision": rev, "kind": "Singular",
                 "status": "accepted" if accepted else "rejected", "native_exit": code,
                 "marker": marker, "marker_present": marker in log.splitlines(),
                 "error_count": len(errors), "first_errors": errors[:8], "result_lines": lines,
                 "started_utc": start, "finished_utc": finish,
                 "native_wall_seconds": (datetime.fromisoformat(finish.replace("Z", "+00:00")) - datetime.fromisoformat(start.replace("Z", "+00:00"))).total_seconds(),
                 "artifacts": [{"path": str(p.relative_to(h)), "sha256": digest(p), "bytes": p.stat().st_size}
                               for p in sorted(raw.iterdir()) if p.is_file()]})
assert [r["status"] for r in rows] == ["rejected", "accepted"]
assert rows[0]["native_exit"] == 0 and rows[0]["marker_present"] and rows[0]["error_count"] > 0
assert rows[1]["native_exit"] == 0 and rows[1]["error_count"] == 0 and len(rows[1]["result_lines"]) == 1
fields = dict(x.split("=", 1) for x in rows[1]["result_lines"][0].split()[1:])
assert {k: fields[k] for k in ("order_rank", "old_quotient", "square_columns", "binary_rows", "square_rank", "survivor_dimension", "new_quotient_length", "new_order_quotient", "rational_nonzero_entries", "sixth_basis_columns", "rank_witness_rows", "stabilized_order", "changed_inverse_rejected")} == {
    "order_rank": "32", "old_quotient": "2", "square_columns": "2",
    "binary_rows": "24", "square_rank": "2", "survivor_dimension": "0",
    "new_quotient_length": "2", "new_order_quotient": "0",
    "rational_nonzero_entries": "64", "sixth_basis_columns": "32",
    "rank_witness_rows": "1,2", "stabilized_order": "1",
    "changed_inverse_rejected": "1"}
certificate = (h / "runs" / names[1] / "square-kernel70.sing").read_text()
assert "binaryMap70[1,1]=1;" in certificate
assert "binaryMap70[2,1]=1;" in certificate
assert "binaryMap70[2,2]=1;" in certificate
assert "binaryMap70[1,2]=" not in certificate
assert "int rank70=2;int kernelDimension70=0;" in certificate
for filename in ("quotient-basis70.sing", "square-kernel70.sing", "sixth-lattice-polynomial70.sing", "sixth-lattice-field70.sing"):
    assert (h / "runs" / names[1] / filename).exists()
claims = {"Frobenius70.two_row_kernel_zero", "Frobenius70.saved_row_minor_zero",
          "Frobenius70.squareIter_mem", "Frobenius70.stable_sandwich_eq_closure",
          "Frobenius70.closure_subset_descend", "Frobenius70.closure_eq_order_of_six_steps"}
for name in lean_names:
    raw = h / "runs" / name
    rev = (raw / "source-revision.txt").read_text().strip()
    compile_code = int((raw / "compile.exit").read_text())
    audit_code = int((raw / "audit.exit").read_text()) if (raw / "audit.exit").exists() else None
    audit = (raw / "audit.log").read_text() if (raw / "audit.log").exists() else ""
    declarations = re.findall(r"^ok\s+(Frobenius70\.\S+)", audit, re.M)
    expected = claims if name.endswith("-03") else claims - {"Frobenius70.closure_subset_descend", "Frobenius70.closure_eq_order_of_six_steps"}
    accepted = compile_code == audit_code == 0 and expected.issubset(declarations) and "0 failing, 0 trusted" in audit
    for local, source in {"driver.sh": "LEAN_DRIVER.sh", "launcher.sh": "REPLAY_LEAN.sh",
                          "Frobenius70.lean": "lean/Frobenius70.lean",
                          "LEAN-PROTOCOL.md": "LEAN-PROTOCOL.md",
                          "AuditAxioms.lean": "../../../../tools/lean/AuditAxioms.lean"}.items():
        path = str((h / source).resolve().relative_to(repo))
        actual = (raw / local).read_bytes()
        assert subprocess.check_output(["git", "show", rev + ":" + path], cwd=repo) == actual
        pins.append({"attempt": name, "artifact": local, "git_source": path,
                     "revision": rev, "sha256": hashlib.sha256(actual).hexdigest()})
    rows.append({"attempt": name, "source_revision": rev, "kind": "Lean",
                 "status": "accepted" if accepted else "rejected",
                 "compile_exit": compile_code, "audit_exit": audit_code,
                 "audited_declarations": declarations,
                 "authored_claims": sorted(claims.intersection(declarations)),
                 "artifacts": [{"path": str(p.relative_to(h)), "sha256": digest(p), "bytes": p.stat().st_size}
                               for p in sorted(raw.iterdir()) if p.is_file()]})
assert [r["status"] for r in rows] == ["rejected", "accepted", "rejected", "accepted", "accepted"]
assert rows[2]["compile_exit"] != 0 and rows[2]["audit_exit"] is None
assert rows[3]["compile_exit"] == rows[3]["audit_exit"] == 0
assert rows[4]["compile_exit"] == rows[4]["audit_exit"] == 0 and set(rows[4]["authored_claims"]) == claims
result = {"schema": "koblitz.results/v1", "round": 70, "runs": rows,
          "input_pins": pins, "accepted_rank_minor": [[1, 0], [1, 1]],
          "authored_proved_claims": sorted(claims),
          "protocol_metadata_disclosure": "Version2 zero-column repair frozen after rejected first attempt and before fresh affected data."}
(h / "results.json").write_text(json.dumps(result, indent=2) + "\n")
export = {"schema": "experiment.export/v1", "experiment_id": "KOBLITZ-R70-SIXTH-FROBENIUS",
          "claim_class": "exact_computation",
          "protocols": [{"attempt": r["attempt"], "path": "runs/" + r["attempt"] + "/" + ("protocol.json" if r["kind"] == "Singular" else "LEAN-PROTOCOL.md"),
                         "sha256": digest(h / "runs" / r["attempt"] / ("protocol.json" if r["kind"] == "Singular" else "LEAN-PROTOCOL.md")),
                         "frozen_revision": r["source_revision"]} for r in rows],
          "raw_archive": "runs.tar.xz; content address in .artifacts.json after custody publication",
          "results": {"path": "results.json", "sha256": digest(h / "results.json")},
          "flow": {"accepted": 3, "rejected": 2, "interrupted": 0},
          "canonical_schema_validation": "unavailable in selected Git index and disk; manual contract/source-byte checks only",
          "notes": "Full two-class square map rank2; zero survivors; V6=old order and stable under its checked multiplication. First late-marker native run and first Lean compile rejected; six conditional authored Lean implications compile and audit, including the direct six-step containment theorem."}
(h / "export.json").write_text(json.dumps(export, indent=2) + "\n")
print(json.dumps({"attempts": 5, "accepted": 3, "rejected": 2,
                  "historical_byte_pins": len(pins), "rank_minor": result["accepted_rank_minor"]}))

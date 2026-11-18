"""Record the complete R67 native receipt and historical source bytes."""
from datetime import datetime
from pathlib import Path
import hashlib, json, re, subprocess

h = Path(__file__).resolve().parent
repo = h.parents[3]
raw = h / "runs/third-frobenius67-01"
assert [p.name for p in (h / "runs").iterdir() if p.is_dir()] == [raw.name]
rev = (raw / "source-revision.txt").read_text().strip()
code = int((raw / "exit.txt").read_text())
log = (raw / "native.log").read_text()
errors = [s for s in log.splitlines() if re.match(r"^\s*\?", s)]
marker = "THIRD_FROBENIUS67_ALL_CHECKS_PASS"
lines = [s for s in log.splitlines() if s.startswith("FROB67_RESULT ")]
assert code == 0 and not errors and marker in log.splitlines() and len(lines) == 1
fields = dict(x.split("=", 1) for x in lines[0].split()[1:])
assert {k: fields[k] for k in ("order_rank", "old_quotient", "square_columns", "binary_rows", "square_rank", "survivor_dimension", "new_quotient_length", "third_basis_columns", "changed_kernel_rejected", "changed_inverse_rejected")} == {
    "order_rank": "32", "old_quotient": "32", "square_columns": "32",
    "binary_rows": "215", "square_rank": "24", "survivor_dimension": "8",
    "new_quotient_length": "24", "third_basis_columns": "32",
    "changed_kernel_rejected": "1", "changed_inverse_rejected": "1"}
pins = []
for local, source in {"input.sing": "THIRD_FROBENIUS67.sing", "driver.sh": "NATIVE_DRIVER.sh", "launcher.sh": "REPLAY_CAS.sh", "PROTOCOL.md": "PROTOCOL.md", "protocol.json": "protocol.json", "donors.json": "donors.json"}.items():
    path = str((h / source).relative_to(repo))
    actual = (raw / local).read_bytes()
    assert subprocess.check_output(["git", "show", rev + ":" + path], cwd=repo) == actual
    pins.append({"artifact": local, "git_source": path, "revision": rev,
                 "sha256": hashlib.sha256(actual).hexdigest()})
for row in json.loads((raw / "donors.json").read_text()):
    p = h / row["path"]
    assert p.stat().st_size == row["bytes"] and hashlib.sha256(p.read_bytes()).hexdigest() == row["sha256"]
for name in ("quotient-basis67.sing", "square-kernel67.sing", "third-lattice-polynomial67.sing", "third-lattice-field67.sing"):
    assert (raw / name).exists()
start = (raw / "started-utc.txt").read_text().strip()
finish = (raw / "finished-utc.txt").read_text().strip()
digest = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
row = {"attempt": raw.name, "source_revision": rev, "kind": "Singular",
       "status": "accepted", "native_exit": code, "marker": marker,
       "marker_present": True, "error_count": 0, "result_lines": lines,
       "started_utc": start, "finished_utc": finish,
       "native_wall_seconds": (datetime.fromisoformat(finish.replace("Z", "+00:00")) - datetime.fromisoformat(start.replace("Z", "+00:00"))).total_seconds(),
       "artifacts": [{"path": str(p.relative_to(h)), "sha256": digest(p), "bytes": p.stat().st_size}
                     for p in sorted(raw.iterdir()) if p.is_file()]}
result = {"schema": "koblitz.results/v1", "round": 67, "runs": [row], "input_pins": pins,
          "donors_checked": len(json.loads((raw / "donors.json").read_text()))}
(h / "results.json").write_text(json.dumps(result, indent=2) + "\n")
export = {"schema": "experiment.export/v1", "experiment_id": "KOBLITZ-R67-THIRD-FROBENIUS",
          "claim_class": "exact_computation",
          "protocols": [{"attempt": raw.name, "path": str((raw / "protocol.json").relative_to(h)),
                         "sha256": digest(raw / "protocol.json"), "frozen_revision": rev}],
          "raw_archive": "runs.tar.xz; content address in .artifacts.json after custody publication",
          "results": {"path": "results.json", "sha256": digest(h / "results.json")},
          "flow": {"accepted": 1, "rejected": 0, "interrupted": 0},
          "canonical_schema_validation": "unavailable in selected Git index and disk; manual contract/source-byte checks only",
          "notes": "Complete 32-class third square-preimage: rank24, survivors8, rank32 V3 lattice; further steps required for equality."}
(h / "export.json").write_text(json.dumps(export, indent=2) + "\n")
print(json.dumps({"accepted": 1, "historical_byte_pins": len(pins), "square_rank": fields["square_rank"], "survivors": fields["survivor_dimension"]}))

"""Record immutable terminal receipts and historical Git byte pins; no algebra."""
from datetime import datetime
from pathlib import Path
import hashlib
import json
import re
import subprocess

here = Path(__file__).resolve().parent
repo = here.parents[3]
claims = {f"Half63.{x}" for x in (
    "square_injective", "root_fixed", "pole_cap_half",
    "restriction_involution", "separator_blocks_preimage")}
rows = []
pins = []

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def parse_fields(line):
    return dict(part.split("=", 1) for part in line.split()[1:])

for raw in sorted((here / "runs").iterdir()):
    if not raw.is_dir():
        continue
    revision = (raw / "source-revision.txt").read_text().strip()
    row = {"attempt": raw.name, "source_revision": revision}
    if (raw / "input.sing").exists():
        source = (raw / "input.sing").read_text()
        matches = re.findall(r'print\("([A-Z0-9_]+_ALL_CHECKS_PASS)"\)', source)
        assert len(matches) == 1, (raw.name, matches)
        marker = matches[0]
        code = int((raw / "exit.txt").read_text())
        log = (raw / "native.log").read_text()
        errors = [line for line in log.splitlines() if re.match(r"^\s*\?", line)]
        results = [line for line in log.splitlines() if re.match(r"^[A-Z0-9_]+_(?:RESULT|LINEAR) ", line)]
        accepted = code == 0 and not errors and marker in log.splitlines()
        source_candidates = []
        for name in ("GENERATOR112.sing", "CHECK112.sing", "HALF_DESCENT63.sing"):
            git_path = str((here / name).relative_to(repo))
            historical = subprocess.run(["git", "show", f"{revision}:{git_path}"],
                                        cwd=repo, capture_output=True)
            if historical.returncode == 0 and historical.stdout == (raw / "input.sing").read_bytes():
                source_candidates.append(name)
        assert len(source_candidates) == 1, (raw.name, source_candidates)
        mapping = {"input.sing": source_candidates[0], "driver.sh": "NATIVE_DRIVER.sh",
                   "launcher.sh": "REPLAY_CAS.sh", "PROTOCOL.md": "PROTOCOL.md",
                   "protocol.json": "protocol.json", "donors.json": "donors.json"}
        start = (raw / "started-utc.txt").read_text().strip()
        finish = (raw / "finished-utc.txt").read_text().strip()
        row.update(kind="Singular", status="accepted" if accepted else "rejected",
                   native_exit=code, marker=marker, marker_present=marker in log.splitlines(),
                   error_count=len(errors), first_errors=errors[:8],
                   result_lines_accepted=accepted, result_lines=results,
                   outcome="completed" if accepted else "native_error" if errors else "time_cap" if code == 124 else "missing_marker",
                   started_utc=start, finished_utc=finish,
                   native_wall_seconds=(datetime.fromisoformat(finish.replace("Z", "+00:00"))
                                        - datetime.fromisoformat(start.replace("Z", "+00:00"))).total_seconds())
    else:
        compile_code = int((raw / "compile.exit").read_text())
        audit_code = int((raw / "audit.exit").read_text()) if (raw / "audit.exit").exists() else None
        audit = (raw / "audit.log").read_text() if (raw / "audit.log").exists() else ""
        declarations = re.findall(r"^ok\s+(Half63\.\S+)", audit, re.M)
        accepted = (compile_code == audit_code == 0 and claims.issubset(declarations)
                    and "0 failing, 0 trusted" in audit)
        mapping = {"driver.sh": "LEAN_DRIVER.sh", "launcher.sh": "REPLAY_LEAN.sh",
                   "Half63.lean": "lean/Half63.lean", "LEAN-PROTOCOL.md": "LEAN-PROTOCOL.md",
                   "AuditAxioms.lean": None}
        row.update(kind="Lean", status="accepted" if accepted else "rejected",
                   compile_exit=compile_code, audit_exit=audit_code,
                   audited_declarations=declarations,
                   authored_claims=sorted(claims.intersection(declarations)))
    for local, source in mapping.items():
        git_path = str((here / source).relative_to(repo)) if source else "tools/lean/AuditAxioms.lean"
        old = subprocess.check_output(["git", "show", f"{revision}:{git_path}"], cwd=repo)
        assert old == (raw / local).read_bytes(), (raw.name, local)
        pins.append({"attempt": raw.name, "artifact": local,
                     "revision": revision, "git_source": git_path,
                     "sha256": digest(raw / local)})
    row["artifacts"] = [{"path": str(path.relative_to(here)),
                         "sha256": digest(path), "bytes": path.stat().st_size}
                        for path in sorted(raw.iterdir()) if path.is_file()]
    rows.append(row)

accepted = {row["attempt"]: row for row in rows if row["status"] == "accepted"}
required = {"generator112-01", "check112-01", "half-descent63-02", "half-lean63-01"}
assert required.issubset(accepted), sorted(accepted)
rejected = {row["attempt"]: row for row in rows if row["status"] == "rejected"}
assert set(rejected) == {"half-descent63-01"}
assert rejected["half-descent63-01"]["error_count"] > 0
assert len(rows) == 5

gen = [parse_fields(line) for line in accepted["generator112-01"]["result_lines"]
       if line.startswith("GEN112_RESULT ")]
check = [parse_fields(line) for line in accepted["check112-01"]["result_lines"]
         if line.startswith("CHECK112_RESULT ")]
assert len(gen) == len(check) == 1
assert gen[0]["source_degree"] == check[0]["source_degree"] == "112"
assert gen[0]["whole_source_identities"] == check[0]["whole_source_identities"] == "124"
assert gen[0]["derivative_inverses"] == check[0]["derivative_inverses"] == "2"
half = accepted["half-descent63-02"]["result_lines"]
linear = {fields["family"]: fields for fields in
          (parse_fields(line) for line in half if line.startswith("HALF63_LINEAR "))}
source = {fields["family"]: fields for fields in
          (parse_fields(line) for line in half if line.startswith("HALF63_RESULT "))}
assert set(linear) == set(source) == {"mixed", "cubic"}
for family, dimension, fixed, rank, identities in (("mixed", 10, 6, 4, 20),
                                                    ("cubic", 128, 64, 64, 256)):
    a, b = linear[family], source[family]
    assert int(a["root_dimension"]) == int(b["root_dimension"]) == dimension
    assert int(a["fixed_dimension"]) == int(b["actual_fixed_roots"]) == fixed
    assert int(a["trace_rank"]) == rank and fixed + rank == dimension
    assert int(b["whole_source_root_bindings"]) == dimension
    assert int(b["source_involution_identities"]) == identities
    assert a["one_traceable"] == b["one_traceable"] == "1"
assert sum(int(row["source_involution_identities"]) for row in source.values()) == 276
assert set(accepted["half-lean63-01"]["authored_claims"]) == claims

results = {"schema": "koblitz.results/v1", "round": 63, "runs": rows,
           "input_pins": pins, "authored_proved_claims": sorted(claims),
           "protocol_metadata_disclosure": "Versions1-4 copied into prior immutable raw attempts; version5 repairs omitted retained R61 ambient-constant export before fresh attempt02. Attempt01 has a late marker but is rejected for Singular errors."}
(here / "results.json").write_text(json.dumps(results, indent=2) + "\n")
protocols = []
for row in rows:
    raw = here / "runs" / row["attempt"]
    name = "LEAN-PROTOCOL.md" if row["kind"] == "Lean" else "protocol.json"
    path = raw / name
    protocols.append({"attempt": raw.name, "path": str(path.relative_to(here)),
                      "sha256": digest(path), "frozen_revision": row["source_revision"]})
export = {"schema": "experiment.export/v1", "experiment_id": "KOBLITZ-R63",
          "claim_class": "exact_computation", "protocols": protocols,
          "raw_archive": "runs.tar.xz; content address in .artifacts.json after custody publication",
          "results": {"path": "results.json", "sha256": digest(here / "results.json")},
          "flow": {status: sum(row["status"] == status for row in rows)
                   for status in ("accepted", "rejected", "interrupted")},
          "canonical_schema_validation": "unavailable in selected Git index and disk; manual source-byte and exact contract checks only",
          "notes": "Complete actual degree112 source identities and degree32 half-space involution/trace-one source functions; five conditional Lean theorems. Proper normalization, valuations, level77 map, selected991 action and subgroup transfer remain separate."}
(here / "export.json").write_text(json.dumps(export, indent=2) + "\n")
print(json.dumps({"workflows": len(rows), **export["flow"],
                  "historical_byte_pins": len(pins), "authored_claims": len(claims)}))

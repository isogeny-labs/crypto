#!/usr/bin/env python3
"""Run the frozen 336-system full-selector PDP input generation panel."""
import hashlib
import json
import os
import subprocess
import time
from pathlib import Path

study = Path(__file__).resolve().parents[2]
here = Path(__file__).resolve().parent
attempt = here / "attempt-05"
protocol = study / "protocol-model-full-domain-v2.json"
protocol_sha = "cca68d2c08a192476a2d291f1749f08f99399537e69425fc2f01ef50c4c75fdd"
binary = study / "target/release/pdp"
binary_sha = "2ebd29a31146755f8a79b454b4d6d1673b2ed979ffedfc807218d2d8605b37d4"
matched = study / "evidence/pdp/baseline"
output_root = attempt / "inputs"
if output_root.exists():
    raise SystemExit(f"preserving existing inputs directory: {output_root}")
assert hashlib.sha256(protocol.read_bytes()).hexdigest() == protocol_sha
assert hashlib.sha256(binary.read_bytes()).hexdigest() == binary_sha
output_root.mkdir()
env = dict(os.environ, AWS_DEFAULT_REGION="us-east-2")
records = []
for bit in range(27, 34):
    candidate = f"spike{bit}"
    bases = Path(f"/tmp/n83-full-domain-bases-attempt05/{candidate}")
    out = output_root / candidate
    log = attempt / f"generate-{candidate}.log"
    started = time.time()
    proc = subprocess.run([str(binary), "generate", str(out), str(bases), str(matched), candidate], cwd=study.parent.parent, env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    log.write_bytes(proc.stdout)
    row = {"candidate_id": candidate, "coordinate_bit": bit, "command_exit_code": proc.returncode,
           "elapsed_seconds_diagnostic_only": time.time()-started, "log_path": str(log.relative_to(attempt)),
           "log_sha256": hashlib.sha256(log.read_bytes()).hexdigest(), "output_exists": out.exists()}
    if proc.returncode == 0:
        index_path = out / "inputs.json"
        rows = json.loads(index_path.read_text())
        assert len(rows) == 48, (candidate, len(rows))
        assert all(r["candidate_id"] == candidate for r in rows)
        for r in rows:
            assert len(r["factor_base_membership"]) == r["m"]
            assert all(x["present"] for x in r["factor_base_membership"])
            if r["target_kind"] == "planted":
                assert len(r["selected_base_selectors"]) == r["m"]
                assert all(0 <= int(x) < (1 << 27) for x in r["selected_base_selectors"])
                assert [int(x["selector"]) for x in r["factor_base_membership"]] == r["selected_base_selectors"]
        saved_source = attempt / "base-sources" / candidate
        dest_source = out / "base-source"
        dest_source.mkdir()
        for name in ["index.json", "s3-file-index.json", "source-manifest.json",
                     "koblitz-manifest.json", "binary-manifest.json", "prime-manifest.json"]:
            (dest_source / name).write_bytes((saved_source / name).read_bytes())
        case_files = sorted(p for p in out.iterdir() if p.is_dir())
        row.update({"input_rows":len(rows), "case_directories":len(case_files),
                    "input_index_sha256":hashlib.sha256(index_path.read_bytes()).hexdigest(),
                    "all_membership_checks_passed":True,
                    "full_domain_planted_selector_checks_passed":True})
    else:
        row["failure"] = "generator exited nonzero; partial output and log retained; no retry overwrote this candidate path"
    records.append(row)
    (attempt / f"status-{candidate}.json").write_text(json.dumps(row, indent=2)+"\n")
    print(json.dumps(row), flush=True)
    if proc.returncode != 0:
        break

result = {"schema":"n83.full-domain-input-generation/v1", "protocol_sha256":protocol_sha,
          "generator_sha256":hashlib.sha256(binary.read_bytes()).hexdigest(), "planned_systems":336,
          "expected_bases":7, "completed_bases":sum(r.get("input_rows")==48 for r in records),
          "completed_systems":48*sum(r.get("input_rows")==48 for r in records),
          "all_inputs_verified":len(records)==7 and all(r.get("all_membership_checks_passed") and r.get("full_domain_planted_selector_checks_passed") for r in records),
          "records":records,
          "layout":"attempt-05/inputs/spikeNN/inputs.json plus one family-m-matched-target directory per case, each with input.json and system.sing; base-source holds input/family/S3 indexes and exact source manifests."}
(attempt / "generation-summary.json").write_text(json.dumps(result,indent=2)+"\n")
print(json.dumps({k:v for k,v in result.items() if k!="records"}, indent=2))
if result["completed_bases"] != 7:
    raise SystemExit(1)

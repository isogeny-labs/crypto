"""Freeze compiler outputs and qualification-only metadata; no mathematical work."""
from pathlib import Path
import hashlib
import json
import re
import shutil
import sys

study = Path(__file__).resolve().parent
repo = study.parents[1]
version = sys.argv[1]
assert version in ["current", "historical"]
for step in ["worker", "unit", "native"]:
    assert (study / f"raw/{version}-{step}.exit").read_text().strip() == "0"
    assert (study / f"raw/{version}-{step}.source-after.exit").read_text().strip() == "0"
target = Path("/dev/shm/relation-opt-build-target-20261127")
out = Path(f"/dev/shm/relation-opt-{version}-workers-20261127")
assert not out.exists()
out.mkdir()
sources = {"root-worker": target / "release/root-worker"}
unit = (study / f"raw/{version}-unit.stderr").read_text()
native = (study / f"raw/{version}-native.stderr").read_text()
sources["root-worker-unit"] = Path(re.search(r"Executable unittests src/bin/root-worker.rs \(([^)]+)\)", unit)[1])
for name, filename in [("native-original", "flat_constructor"), ("native-pairgraph", "native_pairgraph")]:
    sources[name] = Path(re.search(rf"Executable tests/{filename}.rs \(([^)]+)\)", native)[1])
for name, source in sources.items():
    shutil.copy2(source, out / name)
    (out / name).chmod(0o500)
prior = "relation_scan_call_instrument_20261127" if version == "current" else "relation_root_call_instrument_20261127"
pins = json.loads((repo / f"research/{prior}/SYNTHETIC_INPUT_PINS.json").read_text())
index = 4 if version == "current" else 3
pins["worker_paths"][index] = str(out / "root-worker")
pins["worker_sha256"][index] = hashlib.sha256((out / "root-worker").read_bytes()).hexdigest()
pins["scope"] = "Optimized no-clock correctness ONLY; no performance pins, replacement of frozen controls or timing authorization"
pins["synthetic_fixture"] = True
pins["canonical_protocol_validation"] = "pending"
pins["performance_observations_collected"] = 0
(study / f"{version.upper()}_INPUT_PINS.json").write_text(json.dumps(pins, indent=2) + "\n")
qualifier = Path("/dev/shm/relation-scan-call-workers-20261127/root-transport-qualify") if version == "current" else Path("/dev/shm/relation-call-instrument-workers-20261127/root-transport-qualify")
tools = [qualifier, Path("/dev/shm/relation-root-workers-20261127/baseline-audit"), Path("/dev/shm/relation-root-workers-20261127/pairgraph-audit")]
receipt = {
    "schema": "optimized-frozen-worker-precall/v1", "version": version,
    "source": f"SOURCE_{version.upper()}.sha256",
    "workers": {str(p): hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(out.iterdir())},
    "independent_checker_tools": {str(p): hashlib.sha256(p.read_bytes()).hexdigest() for p in tools},
    "qualification_pins_sha256": hashlib.sha256((study / f"{version.upper()}_INPUT_PINS.json").read_bytes()).hexdigest(),
    "profile": json.loads((study / "PLAN.json").read_text())["profile"],
    "worker_mode": "--qualify only", "performance_observations_collected": 0,
    "15x_admitted": False, "signed_execution_attestation": False,
}
(study / f"{version.upper()}_PRECALL.json").write_text(json.dumps(receipt, indent=2) + "\n")
print(json.dumps(receipt))

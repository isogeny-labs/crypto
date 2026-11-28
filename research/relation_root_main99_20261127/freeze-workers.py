"""Freeze file bytes and source custody only; no mathematical execution."""
from pathlib import Path
import hashlib
import json
import re
import shutil

study = Path(__file__).resolve().parent
assert (study / "raw/build.exit").read_text().strip() == "0"
assert not (study / "PRECALL.json").exists()
artifacts = dict(re.findall(
    r"Executable tests/(flat_constructor|native_pairgraph)\.rs \(([^\n]+)\)",
    (study / "raw/build.stderr").read_text(),
))
assert set(artifacts) == {"flat_constructor", "native_pairgraph"}
workers = {}
for target, lane in [("flat_constructor", "original"), ("native_pairgraph", "pairgraph")]:
    source = Path(artifacts[target])
    frozen = Path("/dev/shm/relation-root-workers-20261127") / ("main99-" + lane)
    assert not frozen.exists()
    shutil.copy2(source, frozen)
    frozen.chmod(0o555)
    digest = hashlib.sha256(frozen.read_bytes()).hexdigest()
    assert digest == hashlib.sha256(source.read_bytes()).hexdigest()
    workers[lane] = {"path": str(frozen), "sha256": digest, "bytes": frozen.stat().st_size}
auditors = {}
for name in ["baseline-audit", "pairgraph-audit"]:
    p = Path("/dev/shm/relation-root-workers-20261127") / name
    assert p.is_file() and not p.stat().st_mode & 0o222
    auditors[name] = {"path": str(p), "sha256": hashlib.sha256(p.read_bytes()).hexdigest()}
receipt = {
    "schema": "main99-immutable-workers-precall/v1",
    "source": "SOURCE.sha256",
    "source_roster_sha256": hashlib.sha256((study / "SOURCE.sha256").read_bytes()).hexdigest(),
    "workers": workers,
    "auditors": auditors,
    "metadata_scripts_sha256": {name: hashlib.sha256((study / name).read_bytes()).hexdigest()
                                for name in ["run.sh", "freeze-workers.py"]},
    "cpu": 0,
    "numerical_threads": 1,
    "outputs": ["/dev/shm/relation-main99-original-20261127",
                "/dev/shm/relation-main99-pairgraph-20261127"],
    "native_cases": 36,
    "native_matched_pairs": 108,
    "benchmark": False,
    "signed_execution": False,
}
assert all(not Path(name).exists() for name in receipt["outputs"])
with (study / "PRECALL.json").open("x") as stream:
    json.dump(receipt, stream, indent=2)
    stream.write("\n")
print("Fresh immutable native workers frozen before all main99 calls.")

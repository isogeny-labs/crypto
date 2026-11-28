"""Read-only manifest parser/provenance audit; never invokes Cargo or changes sources."""
from pathlib import Path
from collections import Counter
import hashlib
import json
import tomllib

HERE = Path(__file__).resolve().parent


def identity(path):
    data = path.read_bytes()
    return {
        "file": path.name,
        "bytes": len(data),
        "sha256": hashlib.sha256(data).hexdigest(),
        "git_blob": hashlib.sha1(b"blob " + str(len(data)).encode() + b"\0" + data).hexdigest(),
    }


bad_path = HERE / "Cargo.failed-fc184.toml"
good_path = HERE / "Cargo.main-f0.toml"
bad = bad_path.read_bytes()
good = good_path.read_bytes()
metadata = json.loads((HERE / "source_metadata.json").read_text())
assert identity(bad_path)["git_blob"] == "2c5622514938938b7c91c34629d7e2dd877c1d33"
assert identity(good_path)["git_blob"] == "8d6da017040e8edbb2cd8b016a94cb2e62c7d1ca"

try:
    tomllib.loads(bad.decode())
except tomllib.TOMLDecodeError as exc:
    bad_error = str(exc)
else:
    raise AssertionError("The historical failing manifest unexpectedly parsed")

parsed = tomllib.loads(good.decode())
reconstructed = bad
insertions = []
for name, header in (
    ("curve-transfer-sweep", "[[bin]]"),
    ("odd-transfer-catalog", "[[bin]]"),
    ("floor_crater_bridge_cost", "[[example]]"),
):
    marker = f'name = "{name}"\n'.encode()
    assert reconstructed.count(marker) == 1
    reconstructed = reconstructed.replace(marker, header.encode() + b"\n" + marker, 1)
    insertions.append({"target": name, "inserted_header": header})
assert reconstructed == good, "Current main must equal the historical file plus exactly these three headers"

target_checks = {}
for kind in ("bin", "example"):
    entries = parsed.get(kind, [])
    counts = Counter(row["name"] for row in entries)
    duplicates = sorted(name for name, count in counts.items() if count > 1)
    assert not duplicates
    target_checks[kind] = {"count": len(entries), "duplicate_names": duplicates}

receipt = {
    "scope": "Manifest text/provenance and TOML parsing only; no Cargo build or cryptographic execution",
    "inputs": [identity(bad_path), identity(good_path)],
    "historical_parse_error": bad_error,
    "current_main_tomllib_parse": "success",
    "exact_reconstruction": {"insertions": insertions, "added_lines": 3, "deleted_lines": 0, "equals_current_main_bytes": True},
    "target_inventory": target_checks,
    "metadata_sha256": hashlib.sha256((HERE / "source_metadata.json").read_bytes()).hexdigest(),
    "failure_excerpt_sha256": hashlib.sha256((HERE / "failed-job-excerpt.log").read_bytes()).hexdigest(),
    "source_manifest_mutated": False,
    "new_patch_required_on_pinned_main": False,
}
(HERE / "receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
print(json.dumps(receipt, indent=2))

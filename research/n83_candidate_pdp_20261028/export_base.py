"""Package native enumeration receipts; this does not compute curve results."""
import copy
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
BASE = HERE.parent / "n83_fixed_factor_base_20261028"


def read(path):
    return json.loads(path.read_text())


def artifact(rel, kind):
    path = HERE / rel
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(4 * 1024 * 1024), b""):
            digest.update(block)
    return dict(path=rel, kind=kind, bytes=path.stat().st_size, sha256=digest.hexdigest())


protocol = read(HERE / "protocol-base.json")
index = read(HERE / "data/index.json")
assert index["complete"] and index["candidate_id"] == "spike40"
export = read(BASE / "experiment.export.json")
template = copy.deepcopy(export["runs"][0])
export["export_id"] = protocol["protocol_id"]
export["protocol"] = dict(path="protocol-base.json", protocol_id=protocol["protocol_id"],
                          version=1, sha256=protocol["freeze"]["protocol_sha256"])
export["source"]["commit"] = protocol["freeze"]["commit"]
export["producer"]["session"] = "n83-candidate-spike40-e27-20261028-r1"
export["host"]["sha256"] = artifact("evidence/host.txt", "host")["sha256"]
export["arms"] = [dict(id=a["id"], role=a["role"], commit=a["instrument"]["commit"],
                       binary_sha256=a["instrument"]["binary_sha256"], image_digest=None)
                  for a in protocol["arms"]]
export["files"] = []
export["runs"] = []
for family in index["families"]:
    name = family["family"]
    manifest_ref = artifact(f"data/{name}/manifest.json", "summary")
    assert manifest_ref["sha256"] == family["manifest_sha256"]
    export["files"].append(manifest_ref)
    manifest = read(HERE / manifest_ref["path"])
    assert len(manifest["shards"]) == 128
    for i, shard in enumerate(manifest["shards"]):
        assert shard["start"] == i * 2**20 and shard["end"] == (i + 1) * 2**20
        assert shard["verified_all_candidates"] is True
        raw = artifact(f"data/{name}/{shard['file']}", "raw")
        assert raw["sha256"] == shard["sha256"] and raw["bytes"] == shard["bytes"]
        receipt = artifact(f"data/{name}/part-{i:04}.json", "raw")
        assert read(HERE / receipt["path"]) == shard
        export["files"].extend([raw, receipt])
        row = copy.deepcopy(template)
        row.update(arm=name, instance_id=str(i), run_id=f"{name}-{i:04}", artifacts=[raw, receipt],
                   backend_ref=protocol["extensions"]["raw_prefix"] + raw["path"])
        row["measurements"]["counted"] = {key: shard[key] for key in ["point_count", "liftable_x"]}
        row["verification"]["verifier"] = "Native all-candidate curve/root-pair and omission checks; SHA256 checked during packaging; same-instrument replay separately retained"
        export["runs"].append(row)
for rel in ["data/index.json", "FORMAT.json", "evidence/terminal.json", "evidence/controls.json",
            "evidence/producer.log", "evidence/raw-manifest.sha256", "evidence/storage.json"]:
    export["files"].append(artifact(rel, "raw"))
export["analysis"] = dict(script=artifact("src/bin/factor_base.rs", "analysis"),
                          outputs=[artifact("data/index.json", "summary")],
                          blinded=False, blinding_key_sha256=None, unmasked_at=None)
export["notes"] = [dict(by="producer", kind="observation", text=text) for text in [
    "All 384 shards completed. One complete candidate base per original curve is reused across m2..9.",
    "N83FB02 stores selector_index and y; FORMAT.json defines the mandatory selector-to-x mapping.",
    "This enumeration measures no FFD, LFD, regularity, solver speed or ECDLP cost. No elite claim.",
    "Periodic uploader logged transient rename warnings; terminal sync succeeded. No data was discarded.",
]]
assert len(export["runs"]) == 384
(HERE / "experiment-base.export.json").write_text(json.dumps(export, indent=2) + "\n")
print("Packaged 384 native receipts and their verified hashes")

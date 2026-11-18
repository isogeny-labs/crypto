#!/usr/bin/env python3
"""Replay and S3-custody three pinned current-source N83 finalist bases."""

import argparse
import gzip
import hashlib
import importlib.util
import json
import math
import subprocess
import tarfile
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
GROUP = ROOT / "research/n83_indexed5_20261013/analyze.py"
GROUP_SHA256 = "e34d06b3275d86dcb3a659b79adf87dcdc078cebb6f510c4b5c42403d83a1804"
SOURCE_SHA256 = "4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507"
UPSTREAM_HEAD = "0021730ef93baae79522b848877879e6284d2911"
PREFIX = "s3://crypto-autoresearcher/factor-bases/icv1/etc/n83_current_source_finalists_20261019/a1/objects"
TARGETS_SHA256 = "89c7208dd4fc7a5621cf727b3fac9a6714f04f66a312e516b418c31764e7547e"
ANSWERS_SHA256 = "363adc7f9bded3f4a25a80c7b27da5a2305afb30ee6741932cbe76f455c2865b"
EXPECTED = {
    992: {"archive_sha256": "79863413acfe406585bc6e2337ae16a46e07f5b6c806633f7651d7c83d3b43ab",
          "manifest_sha256": "1e79e06e1247beac75de070c95f79996b22ad4cd8d3c184dde6c5de5adf706c1",
          "raw_sha256": "0b700b7bbc6264d732d28c60b8e3ee139ecb826653d1063bc16c49b80822c146"},
    1024: {"archive_sha256": "6c426a5ea2e3ade1af45801d13959458e69b7f67faae2565e406070fb6c06cec",
           "manifest_sha256": "49005af73b7349cccfc56d2b9df38f368f112339cd753257db390792568a2773",
           "raw_sha256": "6ae95097b960d2a0e5972c2f5a8931cbfd6bd5051f08f8206ead5d9abb0d79cd"},
    3300: {"archive_sha256": "4bfe2cbe3592fe4261abf07f4e92184c2db456ac2ebdaf4befe79a087fb6bfd3",
           "manifest_sha256": "8af182db3c194e5036dc534781b9ce80cc1c6dee2ece8b10dc31442da9593c61",
           "raw_sha256": "5759bd2723e06bb53c88a8fd768cb3725404f79f66627817a430f9ea5e9eca66"},
}
FILES = ("base_dump.jsonl", "pilot.json", "launch.json", "verification.json",
         "stdout.jsonl", "target.jsonl", "targets_result.jsonl")


def sha256(data):
    return hashlib.sha256(data).hexdigest()


def load_group():
    if sha256(GROUP.read_bytes()) != GROUP_SHA256:
        raise ValueError("independent group replay source differs")
    spec = importlib.util.spec_from_file_location("n83_custody_group", GROUP)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def read_tar(path):
    out = {}
    with tarfile.open(path, "r:gz") as archive:
        for name in FILES:
            members = [m for m in archive.getmembers()
                       if m.isfile() and m.name.removeprefix("./").split("/")[-1] == name]
            if len(members) != 1:
                raise ValueError(f"{path.name}: expected one {name}")
            out[name] = archive.extractfile(members[0]).read()
    return out


def replay(k, pilot_path, manifests, old_target, old_scalar, group):
    expected = EXPECTED[k]
    if sha256(pilot_path.read_bytes()) != expected["archive_sha256"]:
        raise ValueError(f"K{k} pilot archive SHA-256 differs")
    files = read_tar(pilot_path)
    if sha256(files["base_dump.jsonl"]) != expected["raw_sha256"]:
        raise ValueError(f"K{k} ordered base dump SHA-256 differs")
    names = list(manifests.glob(f"candidate_*fb{166*k}PDP4rootRCguidedLAgaussTDdirectISO0h*.json"))
    if len(names) != 1 or sha256(names[0].read_bytes()) != expected["manifest_sha256"]:
        raise ValueError(f"K{k} candidate manifest differs")
    manifest = json.loads(names[0].read_text())
    base_spec = manifest["factor_base"]
    body = json.loads(files["base_dump.jsonl"])
    pilot = json.loads(files["pilot.json"])
    launch = json.loads(files["launch.json"])
    prior = json.loads(files["verification.json"])
    summary = json.loads(files["stdout.jsonl"])
    target = json.loads(files["target.jsonl"])
    result = json.loads(files["targets_result.jsonl"])
    if (base_spec["effective_columns"] != k or
            base_spec["actual_usable_points"] != 166 * k or
            base_spec["ordered_base_dump_sha256"] != expected["raw_sha256"] or
            manifest["curve"]["curve_id"] != "EC1N83Ckb1a1haacb69ee9f37" or
            manifest["implementation"]["source_sha256"]["ic"] != SOURCE_SHA256 or
            launch["source_sha256"] != SOURCE_SHA256 or
            launch["candidate_id"] != manifest["candidate_id"] or
            summary["base_hash"] != base_spec["source_base_hash_blake3"] or
            summary["rank"] != k or
            summary["factor_base_points"] != 166 * k or
            pilot["status"] != "PASS" or not all(pilot["checks"].values()) or
            prior["status"] != "PASS" or not all(prior["checks"].values()) or
            prior["candidate_manifest_sha256"] != expected["manifest_sha256"]):
        raise ValueError(f"K{k} source or pilot identity differs")
    coords = group.replay_base(body, summary)
    point = group.point_from_json(target)
    scalar = int(result["recovered_scalar"])
    relation = None
    indices = result["point_indices"]
    if not (isinstance(indices, list) and len(indices) == 4 and all(
            isinstance(i, int) and 0 <= i < len(coords) for i in indices)):
        raise ValueError(f"K{k} four-point relation indices invalid")
    for index in indices:
        relation = group.add(relation, coords[index])
    phases = [float(result[name]) for name in (
        "target_query_ms", "target_pdp_ms", "target_relation_check_ms",
        "target_descent_ms", "target_recovery_check_ms")]
    if (target != old_target or result["target"] != target or
            scalar != old_scalar or group.multiply(group.G, scalar) != point or
            not group.on_curve(point) or group.multiply(point, group.R) is not None or
            relation != point or
            not result["group_verified"] or result["exit_code"] != 0 or
            summary["targets_solved"] != 1 or summary["targets_failed"] != 0 or
            any(not math.isfinite(value) or value < 0 for value in phases) or
            not math.isfinite(float(result["target_ms"])) or
            abs(math.fsum(phases) - float(result["target_ms"])) > 1):
        raise ValueError(f"K{k} scalar, relation or phase replay failed")
    return {"k": k, "pilot_archive_sha256": expected["archive_sha256"],
            "candidate_id": manifest["candidate_id"],
            "candidate_manifest_sha256": expected["manifest_sha256"],
            "source_base_hash_blake3": base_spec["source_base_hash_blake3"],
            "ordered_base_dump_sha256": expected["raw_sha256"],
            "point_count": len(coords), "rank": summary["rank"],
            "independent_replay": "PASS_FULL_BASE_RELATION_SCALAR_PHASES",
            "pilot_members_sha256": {name: sha256(data) for name, data in sorted(files.items())}}, files["base_dump.jsonl"]


def head(uri):
    key = uri.removeprefix("s3://crypto-autoresearcher/")
    call = subprocess.run(("aws", "s3api", "head-object", "--bucket",
                           "crypto-autoresearcher", "--key", key,
                           "--output", "json"), capture_output=True, text=True)
    if call.returncode:
        if "Not Found" in call.stderr or "404" in call.stderr:
            return None
        raise RuntimeError(f"S3 HEAD failed: {call.stderr.strip()}")
    return json.loads(call.stdout)


def aws_cp(source, destination, metadata=None):
    command = ["aws", "s3", "cp", str(source), str(destination),
               "--no-progress", "--only-show-errors"]
    if metadata is not None:
        command.extend(("--metadata", metadata))
    subprocess.run(command, check=True, capture_output=True, text=True)


def custody(row, raw, output_dir, upload):
    k = row["k"]
    packed = gzip.compress(raw, compresslevel=9, mtime=0)
    packed_sha = sha256(packed)
    path = output_dir / f"{packed_sha}.jsonl.gz"
    if path.exists() and path.read_bytes() != packed:
        raise ValueError(f"K{k} local content-addressed object conflicts")
    path.write_bytes(packed)
    if sha256(gzip.decompress(path.read_bytes())) != row["ordered_base_dump_sha256"]:
        raise ValueError(f"K{k} local gzip replay failed")
    uri = f"{PREFIX}/{path.name}"
    result = {"k": k, "s3_uri": uri, "gzip_sha256": packed_sha,
              "gzip_bytes": len(packed), "raw_sha256": sha256(raw),
              "raw_bytes": len(raw), "status": "LOCAL_ONLY"}
    if upload:
        metadata = (f"raw-sha256={sha256(raw)},"
                    f"base-hash={row['source_base_hash_blake3']},"
                    f"role=k{k}{'wide3' if k == 3300 else 'root11'}")
        if head(uri) is None:
            aws_cp(path, uri, metadata)
        observed = head(uri)
        if (observed is None or observed["ContentLength"] != len(packed) or
                observed.get("Metadata", {}).get("raw-sha256") != sha256(raw) or
                observed.get("Metadata", {}).get("base-hash") !=
                row["source_base_hash_blake3"]):
            raise ValueError(f"K{k} S3 HEAD metadata differs")
        with tempfile.TemporaryDirectory(dir=output_dir) as folder:
            downloaded = Path(folder) / "object.jsonl.gz"
            aws_cp(uri, downloaded)
            if (sha256(downloaded.read_bytes()) != packed_sha or
                    sha256(gzip.decompress(downloaded.read_bytes())) != sha256(raw)):
                raise ValueError(f"K{k} S3 downloaded bytes differ")
        result.update({"status": "PASS_DOWNLOADED_SHA256",
                       "s3_etag": observed["ETag"],
                       "s3_last_modified": observed["LastModified"]})
    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--pilots-dir", type=Path, required=True)
    parser.add_argument("--manifests-dir", type=Path, required=True)
    parser.add_argument("--old-targets", type=Path, required=True)
    parser.add_argument("--old-answers", type=Path, required=True)
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument("--receipt", type=Path, required=True)
    parser.add_argument("--upload", action="store_true")
    args = parser.parse_args()
    if (sha256(args.old_targets.read_bytes()) != TARGETS_SHA256 or
            sha256(args.old_answers.read_bytes()) != ANSWERS_SHA256):
        raise ValueError("pilot public target or scalar sidecar differs")
    old_target = json.loads(args.old_targets.read_text().splitlines()[0])
    old_scalar = int(json.loads(args.old_answers.read_text().splitlines()[0])["scalar"])
    group = load_group()
    rows, raw_bases = [], {}
    for k in EXPECTED:
        row, raw = replay(k, args.pilots_dir / f"k{k}.tar.gz",
                          args.manifests_dir, old_target, old_scalar, group)
        rows.append(row)
        raw_bases[k] = raw
        print(f"K{k}: {row['independent_replay']}", flush=True)
    args.output_dir.mkdir(parents=True, exist_ok=True)
    objects = [custody(row, raw_bases[row["k"]], args.output_dir, args.upload)
               for row in rows]
    receipt = {"schema": "n83.current-source-base-custody/v1",
               "status": "PASS_S3_ROUNDTRIP" if args.upload else "LOCAL_ONLY",
               "upstream_head_sha": UPSTREAM_HEAD,
               "ic_source_sha256": SOURCE_SHA256,
               "independent_group_source_sha256": GROUP_SHA256,
               "pilot_public_target_sha256": TARGETS_SHA256,
               "pilot_answer_sidecar_sha256": ANSWERS_SHA256,
               "destination_prefix": PREFIX,
               "replays": rows, "objects": objects}
    serialized = json.dumps(receipt, indent=2, sort_keys=True) + "\n"
    if args.receipt.exists() and args.receipt.read_text() != serialized:
        raise ValueError("existing custody receipt differs")
    args.receipt.write_text(serialized)
    print(json.dumps({"status": receipt["status"], "objects": objects},
                     sort_keys=True))


if __name__ == "__main__":
    main()

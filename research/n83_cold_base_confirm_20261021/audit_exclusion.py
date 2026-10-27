"""Audit the sealed confirmation points against the frozen N83 inventory."""

import argparse
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import subprocess
import tarfile


def digest(blob):
    return hashlib.sha256(blob).hexdigest()


def module(path):
    spec = importlib.util.spec_from_file_location("n83_maxk_audit", path)
    value = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(value)
    return value


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--inventory-repo", type=Path, required=True)
    parser.add_argument("--targets", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    repo = args.inventory_repo.resolve(strict=True)
    prior_audit = module(repo / "research/n83_two_tier_root_20261021/audit_maxk_fresh.py")
    head = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=repo,
                                   text=True).strip()
    old_blob = prior_audit.git_blob(prior_audit.OLD_AUDIT)
    if digest(old_blob) != prior_audit.OLD_AUDIT_SHA256:
        raise ValueError("frozen prior inventory differs")
    old = json.loads(old_blob)
    expected = {item["path"]: item["sha256"] for item in old["prior_files"]}
    expected.update(prior_audit.EXTRA)
    current = repo / "research/n83_two_tier_root_20261021"
    for name in ("maxk_screen_targets.jsonl", "maxk_holdout_targets.jsonl"):
        path = current / name
        relative = str(path.relative_to(repo))
        expected[relative] = digest(path.read_bytes())
    existing = set()
    files = []
    for relative, sha in sorted(expected.items()):
        blob = prior_audit.git_blob(relative)
        if digest(blob) != sha:
            raise ValueError(f"inventory input differs: {relative}")
        rows = prior_audit.points(blob)
        existing.update(rows)
        files.append({"path": relative, "sha256": sha, "point_rows": len(rows)})
    archive_blob = prior_audit.ARCHIVE.read_bytes()
    if digest(archive_blob) != prior_audit.ARCHIVE_SHA256:
        raise ValueError("high-K archive differs")
    with tarfile.open(fileobj=io.BytesIO(archive_blob), mode="r:gz") as archive:
        for name, sha in prior_audit.MEMBERS.items():
            blob = archive.extractfile(name).read()
            if digest(blob) != sha:
                raise ValueError(f"archived target differs: {name}")
            existing.update(prior_audit.points(blob))
    local_blob = prior_audit.LOCAL_TARGET.read_bytes()
    if digest(local_blob) != prior_audit.LOCAL_TARGET_SHA256:
        raise ValueError("local target differs")
    existing.update(prior_audit.points(local_blob))
    targets = args.targets.resolve(strict=True).read_bytes()
    fresh = prior_audit.points(targets)
    if len(fresh) != 16 or len(set(fresh)) != 16:
        raise ValueError("fresh target count or distinctness differs")
    result = {"schema": "n83.cold-base-confirm-exclusion/v1",
              "status": "PASS_ZERO_OVERLAP" if not set(fresh) & existing
                        else "OVERLAP_FOUND",
              "inventory_git_head": head,
              "old_audit_sha256": digest(old_blob),
              "inventory_files": files,
              "highk_archive_sha256": digest(archive_blob),
              "local_target_sha256": digest(local_blob),
              "prior_unique_point_count": len(existing),
              "fresh_target_sha256": digest(targets),
              "fresh_target_count": len(fresh),
              "overlap_count": len(set(fresh) & existing)}
    args.output.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps({k: result[k] for k in
                      ("status", "prior_unique_point_count", "fresh_target_count",
                       "overlap_count", "fresh_target_sha256")}, sort_keys=True))
    return 0 if result["status"] == "PASS_ZERO_OVERLAP" else 1


if __name__ == "__main__":
    raise SystemExit(main())

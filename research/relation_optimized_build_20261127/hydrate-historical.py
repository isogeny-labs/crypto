"""Materialize exact Git blobs and audit source metadata; no experiment arithmetic."""
from pathlib import Path
import hashlib
import json
import shutil
import subprocess

repo = Path(__file__).resolve().parents[2]
study = Path(__file__).resolve().parent
plan = json.loads((study / "PLAN.json").read_text())
eligibility = plan["historical_eligibility"]
current = json.loads((study / "CURRENT_QUAL_OUTCOME.json").read_text())
assert current["all_complete"] is True, "current qualification incomplete"
target = Path(plan["resources"]["fresh_target"])
target_bytes = int(subprocess.check_output(["du", "-sb", str(target)]).split()[0])
free_bytes = shutil.disk_usage("/dev/shm").free
memory_bytes = int(Path("/sys/fs/cgroup/memory.current").read_text())
assert target_bytes <= eligibility["target_max_bytes_before_old"], "target headroom"
assert free_bytes >= eligibility["tmpfs_min_free_bytes_before_old"], "tmpfs headroom"
assert memory_bytes <= eligibility["cgroup_max_bytes_before_old"], "memory headroom"
head = plan["historical_head"]
manifest_path = plan["historical_source_context"]
manifest = subprocess.check_output(["git", "show", head + ":" + manifest_path], cwd=repo)
expected = dict(line.decode().split("  ", 1)[::-1] for line in manifest.splitlines())
tree = subprocess.check_output(["git", "ls-tree", "-r", "-z", head, "--", *expected], cwd=repo)
objects = {}
for entry in tree.split(b"\0"):
    if not entry:
        continue
    info, path = entry.split(b"\t", 1)
    mode, kind, oid = info.decode().split()
    path = path.decode()
    assert kind == "blob" and mode in ["100644", "100755"], (path, mode, kind)
    objects[path] = (mode, oid)
assert set(objects) == set(expected), "missing historical source path"
sizes = subprocess.check_output(
    ["git", "cat-file", "--batch-check=%(objectname) %(objecttype) %(objectsize)"],
    input="".join(oid + "\n" for _, oid in objects.values()).encode(), cwd=repo,
)
source_bytes = 0
for line in sizes.decode().splitlines():
    oid, kind, size = line.split()
    assert kind == "blob"
    source_bytes += int(size)
assert source_bytes <= 268435456, "historical source cap256MiB"
out = Path("/dev/shm/relation-opt-historical-source-20261127")
assert not out.exists(), "create-new historical source"
out.mkdir()
proc = subprocess.Popen(["git", "cat-file", "--batch"], stdin=subprocess.PIPE, stdout=subprocess.PIPE, cwd=repo)
for path, (mode, oid) in objects.items():
    relative = Path(path)
    assert not relative.is_absolute() and ".." not in relative.parts
    proc.stdin.write((oid + "\n").encode())
    proc.stdin.flush()
    actual_oid, kind, size = proc.stdout.readline().decode().split()
    assert actual_oid == oid and kind == "blob"
    data = proc.stdout.read(int(size))
    assert len(data) == int(size) and proc.stdout.read(1) == b"\n"
    assert hashlib.sha256(data).hexdigest() == expected[path], path
    destination = out / relative
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_bytes(data)
    destination.chmod(0o755 if mode == "100755" else 0o644)
proc.stdin.close()
assert proc.wait() == 0
(study / "SOURCE_HISTORICAL.sha256").write_bytes(manifest)
receipt = {
    "schema": "optimized-historical-source-materialization/v1", "head": head,
    "source_root": str(out), "source_manifest": manifest_path,
    "source_manifest_sha256": hashlib.sha256(manifest).hexdigest(),
    "files": len(expected), "bytes": source_bytes, "all_file_hashes_match": True,
    "all_file_modes_match": True, "source_patch_applied": False,
    "eligibility": {"target_bytes": target_bytes, "tmpfs_free_bytes": free_bytes, "cgroup_memory_bytes": memory_bytes},
    "source_tree_has_no_git_metadata": True,
    "root_manifest_has_no_build_script_or_build_dependencies": True,
    "performance_observations_collected": 0,
}
(study / "HISTORICAL_SOURCE.json").write_text(json.dumps(receipt, indent=2) + "\n")
print(json.dumps(receipt))

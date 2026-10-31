#!/usr/bin/env python3
"""Administrative checkpoint controller; cryptography stays in the native binary."""
import argparse
import fcntl
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import time
import uuid

HERE = Path(__file__).resolve().parent
ROOT = Path("/dev/shm/ml-dataset-tmpfs-continuation-20261031")
OLD_ROOT = Path("/var/tmp/ml-dataset-local-campaign-20261031")
BINDINGS = json.loads((HERE / "bindings.json").read_text()) if (HERE / "bindings.json").is_file() else {}
START = BINDINGS.get("selected_start", 18)
BINARY = Path("/home/ec2-user/ml-dataset-work-20261028/clean-build-02/crypto")
RUNNER = Path("/home/ec2-user/ml-dataset-work-20261028/runtime-preflight-runner-test-01/runner")
STORE = Path("/home/ec2-user/ml-dataset-work-20261028/graph-export-crypto")
SOURCE = "c41bec150e16e4c4f9a00634002b633b821de262"
HASHES = {
    BINARY: "b8e0881ec5e46f60c23e6bbb49ad7c3677b9168199f5787b68d89169bfaaa77e",
    RUNNER: "10258a68826f3ac123aa3190a4604377f652625f287d99990ab8d833879bd26b",
    STORE: "0d965332c529967b066ddc99819f47cdd57baeccde59a37cbdee94455cbc17b3",
    HERE / "protocol.json": BINDINGS.get("protocol_file_sha256", "unfrozen"),
    HERE / "plan.json": BINDINGS.get("plan_file_sha256", "unfrozen"),
}
BUCKET = "setec-astronomy-artifacts"
AWS_ENV = {
    "AWS_CONFIG_FILE": "/tmp/ml-dataset-aws-config-20261028",
    "AWS_PROFILE": "ml-dataset-checkpoint",
    "AWS_DEFAULT_REGION": "us-east-2",
    "CRYPTO_STORE_REMOTE": "s3://setec-astronomy-artifacts/store",
    "CRYPTO_STORE_INDEX": str(ROOT / "store-index.json"),
}


def digest(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def write_json(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_name(path.name + ".tmp-" + uuid.uuid4().hex)
    with open(tmp, "x", encoding="utf-8") as f:
        json.dump(value, f, sort_keys=True, indent=2)
        f.write("\n")
        f.flush()
        os.fsync(f.fileno())
    os.replace(tmp, path)
    fd = os.open(path.parent, os.O_RDONLY)
    try:
        os.fsync(fd)
    finally:
        os.close(fd)


def command(argv, timeout, stdout=None, env=None):
    started = time.time_ns()
    try:
        p = subprocess.run(
            list(map(str, argv)), stdin=subprocess.DEVNULL, stdout=subprocess.PIPE,
            stderr=subprocess.PIPE, timeout=timeout, start_new_session=True,
            env=env, check=False,
        )
        status = {"exit_code": p.returncode, "timed_out": False}
        out, err = p.stdout, p.stderr
    except subprocess.TimeoutExpired as e:
        # subprocess.run kills the direct process. A supervised native runner
        # normally exits itself after 90+5 seconds; any orphan is detected
        # before the next shard and blocks advancement.
        status = {"exit_code": None, "timed_out": True}
        out, err = e.stdout or b"", e.stderr or b""
    status["started_unix_ns"] = str(started)
    status["finished_unix_ns"] = str(time.time_ns())
    status["argv"] = list(map(str, argv))
    status["stdout_sha256"] = hashlib.sha256(out).hexdigest()
    status["stderr_sha256"] = hashlib.sha256(err).hexdigest()
    if stdout:
        stdout.parent.mkdir(parents=True, exist_ok=True)
        stdout.write_bytes(out)
        stdout.with_suffix(".stderr").write_bytes(err)
    return status


def stage(shard, name, argv, timeout, env=None):
    receipt = shard / (name + ".json")
    if receipt.exists():
        return json.loads(receipt.read_text())
    log = shard / (name + ".stdout")
    if log.exists() or log.with_suffix(".stderr").exists():
        raise RuntimeError(f"unreceipted {name} output at {shard}; preserve for manual reconciliation")
    result = command(argv, timeout, log, env)
    write_json(receipt, result)
    return result


def active_worker(shard):
    needle = os.fsencode(str(shard / "runner"))
    for proc in Path("/proc").glob("[0-9]*"):
        if int(proc.name) == os.getpid():
            continue
        try:
            argv = (proc / "cmdline").read_bytes()
        except (OSError, PermissionError):
            continue
        if needle in argv and (b"crypto" in argv or b"runner" in argv):
            return proc.name
    return None


def admission():
    if not BINDINGS or not isinstance(START, int) or not 1 <= START <= 63:
        raise RuntimeError("continuation bindings not filed and frozen")
    for path, expected in HASHES.items():
        if not path.is_file() or digest(path) != expected:
            raise RuntimeError(f"pinned input mismatch: {path}")
    build = json.loads(Path("/home/ec2-user/ml-dataset-work-20261028/clean-build-02/receipt.json").read_text())
    if SOURCE not in json.dumps(build):
        raise RuntimeError("clean-build receipt lacks source commit")
    plan = json.loads((HERE / "plan.json").read_text())
    protocol = json.loads((HERE / "protocol.json").read_text())
    ext = protocol.get("extensions", {}).get("ml_dataset_tmpfs_continuation", {})
    if protocol.get("freeze", {}).get("status") != "frozen" or ext.get("selected_shards") != f"{START}..63 inclusive" or ext.get("selected_cell_count") != 64 - START:
        raise RuntimeError("frozen continuation interval differs from bindings")
    if plan.get("protocol_sha256") != HASHES[HERE / "protocol.json"]:
        raise RuntimeError("plan is not bound to frozen protocol file")
    if plan.get("field_sizes") != [23] or plan.get("arities") != [2] or plan.get("families") != ["prime"] or plan.get("solvers") != ["bsgs"] or not plan.get("twists"):
        raise RuntimeError("unexpected native plan axes")
    if plan.get("curve_seeds") != 8 or plan.get("generators") != 2 or plan.get("keys") != 2:
        raise RuntimeError("unexpected native population size")
    pre = command([BINARY, "dataset", "preflight", "--plan", HERE / "plan.json", "--protocol", HERE / "protocol.json"], 30)
    if pre["exit_code"] != 0:
        raise RuntimeError("native preflight failed")
    out = subprocess.check_output(["findmnt", "-n", "-o", "FSTYPE", "--target", "/dev/shm"], timeout=10).decode().strip()
    if out != "tmpfs":
        raise RuntimeError("continuation output is not tmpfs")
    for index in range(START):
        if not (OLD_ROOT / "custody" / f"shard-{index:02d}-published.json").is_file():
            raise RuntimeError(f"original XFS custody missing at index {index}")
    for index in range(START, 64):
        if (OLD_ROOT / f"shard-{index:02d}" / "attempt-start.json").exists() or (OLD_ROOT / "custody" / f"shard-{index:02d}-published.json").exists():
            raise RuntimeError(f"original XFS overlap at index {index}")
    mem_available = next((int(line.split()[1]) * 1024 for line in Path("/proc/meminfo").read_text().splitlines() if line.startswith("MemAvailable:")), 0)
    tmpfs_free = shutil.disk_usage("/dev/shm").free
    if tmpfs_free < 8 * 1024 ** 3 or mem_available < 16 * 1024 ** 3:
        raise RuntimeError("tmpfs or host memory below admission floor")
    if not Path(AWS_ENV["AWS_CONFIG_FILE"]).is_file():
        raise RuntimeError("checkpoint AWS config unavailable")
    return {"schema": "ml-dataset.tmpfs-continuation-admission/v1", "selected_start": START, "selected_cells": 64 - START, "native_plan_cells": 64, "prior_xfs_custody_cells": START, "plan_sha256": HASHES[HERE / "plan.json"], "protocol_file_sha256": HASHES[HERE / "protocol.json"], "source_commit": SOURCE, "binary_sha256": HASHES[BINARY], "runner_sha256": HASHES[RUNNER], "store_sha256": HASHES[STORE], "filesystem": out, "tmpfs_free_bytes": tmpfs_free, "mem_available_bytes": mem_available, "scientific_computation_performed": False}


def attempt(shard, index):
    marker = shard / "attempt-start.json"
    runner_dir = shard / "runner"
    if not marker.exists():
        if runner_dir.exists():
            raise RuntimeError(f"unmarked runner directory: {runner_dir}")
        mem_available = next((int(line.split()[1]) * 1024 for line in Path("/proc/meminfo").read_text().splitlines() if line.startswith("MemAvailable:")), 0)
        if shutil.disk_usage("/dev/shm").free < 8 * 1024 ** 3 or mem_available < 16 * 1024 ** 3:
            raise RuntimeError("tmpfs or host memory below floor; stop before new attempt")
        if (OLD_ROOT / f"shard-{index:02d}" / "attempt-start.json").exists():
            raise RuntimeError(f"original XFS overlap at shard {index}")
        write_json(marker, {"schema": "ml-dataset.tmpfs-continuation-attempt/v1", "index": index, "protocol_file_sha256": HASHES[HERE / "protocol.json"], "plan_file_sha256": HASHES[HERE / "plan.json"], "created_unix_ns": str(time.time_ns()), "one_attempt_only": True, "prior_xfs_custody_cells": START})
        argv = [RUNNER, BINARY, SOURCE, f"prime23-bsgs-{index:02d}", "90", runner_dir, "--", "generate", "--plan", HERE / "plan.json", "--protocol", HERE / "protocol.json", "--out", runner_dir / "attempt", "--max-cases", "1", "--shard-index", str(index), "--shard-count", "64", "--seconds-per-case", "30", "--memory-mib", "2048", "--trace-mib", "64"]
        result = command(argv, 115, shard / "launcher.stdout")
        write_json(shard / "launcher.json", result)
    elif not (shard / "launcher.json").exists():
        if active_worker(shard):
            raise RuntimeError(f"active worker for shard {index}")
        write_json(shard / "launcher.json", {"schema": "ml-dataset.tmpfs-continuation-interrupted/v1", "index": index, "interrupted_before_launcher_receipt": True, "exit_code": None})
    return json.loads((shard / "launcher.json").read_text())


def finalize(shard, index):
    if active_worker(shard):
        raise RuntimeError(f"active worker for shard {index}")
    native = shard / "runner" / "attempt"
    status = {}
    if native.is_dir():
        for name, args in [
            ("verify", ["verify", "--out", native]),
            ("report", ["report", "--out", native, "--destination", shard / "report"]),
            ("export", ["export", "--out", native, "--destination", shard / "export", "--cyclic-subgroup-ecdlp"]),
        ]:
            try:
                status[name] = stage(shard, name, [BINARY, "dataset"] + args, 60)["exit_code"]
            except RuntimeError:
                status[name] = "interrupted_unreceipted"
    else:
        status = {"verify": "missing_native_attempt", "report": "missing_native_attempt", "export": "missing_native_attempt"}
    status["runner_receipt_present"] = (shard / "runner" / "receipt.json").is_file()
    status["launcher"] = json.loads((shard / "launcher.json").read_text())
    status["independently_solved_ecdlp_cells"] = 0
    status["artifact_hashes_verified"] = False
    if status.get("verify") == 0:
        try:
            verified = json.loads((shard / "verify.stdout").read_text())
            status["independently_solved_ecdlp_cells"] = verified.get("independently_solved_ecdlp_cells", 0)
            status["artifact_hashes_verified"] = verified.get("artifact_hashes_verified") is True
        except (ValueError, OSError):
            pass
    status["verified_and_exported"] = (all(status.get(k) == 0 for k in ("verify", "report", "export")) and status["artifact_hashes_verified"] and status["independently_solved_ecdlp_cells"] == 1)
    write_json(shard / "local-status.json", status)
    return status


def custody(shard, index, status):
    archive_dir = ROOT / "archives"
    archive_dir.mkdir(exist_ok=True)
    archive = archive_dir / f"shard-{index:02d}.tar.gz"
    if not archive.exists():
        temp = archive.with_name(archive.name + ".tmp-" + uuid.uuid4().hex)
        packed = command(["tar", "-C", ROOT, "-czf", temp, shard.name], 120)
        if packed["exit_code"] != 0 or not temp.is_file():
            raise RuntimeError(f"archive creation failed for shard {index}; partial temp retained")
        os.replace(temp, archive)
    sha = digest(archive)
    size = archive.stat().st_size
    env = os.environ.copy()
    env.update(AWS_ENV)
    logical = f"restricted/ml-dataset-tmpfs-continuation-20261031/shard-{index:02d}.tar.gz"
    store_receipt = shard.parent / "custody" / f"shard-{index:02d}-store.json"
    store_receipt.parent.mkdir(exist_ok=True)
    if not store_receipt.exists():
        store_cmd = [STORE, "store", "--json", "push", archive, "--name", logical, "--description", "Restricted native tmpfs ECDLP cell including witnesses and traces", "--provenance", f"frozen protocol {HASHES[HERE / 'protocol.json']} shard {index} source {SOURCE}"]
        pushed = command(store_cmd, 120, shard.parent / "custody" / f"shard-{index:02d}-store.stdout", env)
        if pushed["exit_code"] != 0:
            raise RuntimeError(f"S3 store failed for shard {index}; local archive retained")
        write_json(store_receipt, pushed)
    key = f"store/sha256/{sha[:2]}/{sha}"
    download = shard.parent / "custody" / f"shard-{index:02d}-fresh-{uuid.uuid4().hex}.tar.gz"
    got = command(["aws", "s3api", "get-object", "--bucket", BUCKET, "--key", key, download], 120, shard.parent / "custody" / f"shard-{index:02d}-get.stdout", env)
    if got["exit_code"] != 0 or not download.is_file() or digest(download) != sha or download.stat().st_size != size:
        raise RuntimeError(f"fresh S3 readback mismatch for shard {index}")
    download.unlink()
    receipt = {"schema": "ml-dataset.tmpfs-continuation-cell-custody/v1", "index": index, "archive_sha256": sha, "archive_bytes": size, "s3_uri": f"s3://{BUCKET}/{key}", "logical_name": logical, "fresh_get_byte_comparison": True, "native_status": status, "protocol_file_sha256": HASHES[HERE / "protocol.json"], "plan_file_sha256": HASHES[HERE / "plan.json"], "prior_xfs_custody_cells": START}
    local = shard.parent / "custody" / f"shard-{index:02d}.json"
    if local.exists() and json.loads(local.read_text()) != receipt:
        raise RuntimeError("immutable local cell receipt differs")
    if not local.exists():
        write_json(local, receipt)
    # Store the small discovery receipt via the same approved content-addressed
    # prefix. The logical-name index is checkpointed to Git by the campaign owner.
    index_name = f"evidence/ml-dataset-tmpfs-continuation-20261031/shard-{index:02d}-custody.json"
    pushed_index = command([STORE, "store", "--json", "push", local, "--name", index_name, "--description", "Local dataset cell custody pointer", "--provenance", f"protocol {HASHES[HERE / 'protocol.json']} shard {index}"], 120, shard.parent / "custody" / f"shard-{index:02d}-index-put.stdout", env)
    if pushed_index["exit_code"] != 0:
        raise RuntimeError(f"custody pointer store failed for shard {index}")
    pointer_sha = digest(local)
    key2 = f"store/sha256/{pointer_sha[:2]}/{pointer_sha}"
    copied = shard.parent / "custody" / f"index-get-{index:02d}-{uuid.uuid4().hex}.json"
    get = command(["aws", "s3api", "get-object", "--bucket", BUCKET, "--key", key2, copied], 60, shard.parent / "custody" / f"shard-{index:02d}-index-get.stdout", env)
    if get["exit_code"] != 0 or not copied.is_file() or copied.read_bytes() != local.read_bytes():
        raise RuntimeError(f"fixed cell index S3 readback mismatch for shard {index}")
    copied.unlink()
    write_json(shard.parent / "custody" / f"shard-{index:02d}-published.json", {"schema": "ml-dataset.tmpfs-continuation-published/v1", "archive_sha256": sha, "pointer_sha256": pointer_sha, "archive_fresh_get": True, "pointer_fresh_get": True})
    return receipt


def git_ready_checkpoint(index):
    pointer = ROOT / "custody" / f"shard-{index:02d}.json"
    published = ROOT / "custody" / f"shard-{index:02d}-published.json"
    if not pointer.is_file() or not published.is_file():
        raise RuntimeError(f"missing local custody checkpoint for shard {index}")
    saved = HERE / "checkpoints" / f"shard-{index:02d}.json"
    content = {"schema": "ml-dataset.tmpfs-continuation-git-checkpoint/v1", "index": index, "custody": json.loads(pointer.read_text()), "published": json.loads(published.read_text()), "store_index_sha256": digest(ROOT / "store-index.json")}
    if saved.exists():
        if json.loads(saved.read_text()) != content:
            # The mutable logical-name index gains later entries. The original
            # per-cell snapshot is immutable and remains the Git checkpoint.
            old = json.loads(saved.read_text())
            content["store_index_sha256"] = old.get("store_index_sha256")
            if old != content:
                raise RuntimeError(f"Git checkpoint differs for shard {index}")
    else:
        write_json(saved, content)


def run():
    # Hold the original controller's lock throughout continuation so neither
    # process can launch an overlapping native shard.
    old_lock = open(OLD_ROOT / "campaign.lock", "a+")
    try:
        fcntl.flock(old_lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
    except BlockingIOError:
        raise RuntimeError("original XFS campaign is active; continuation refused")
    admission_receipt = admission()
    ROOT.mkdir(mode=0o700, parents=True, exist_ok=True)
    os.chmod(ROOT, 0o700)
    lock = open(ROOT / "campaign.lock", "a+")
    try:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
    except BlockingIOError:
        raise RuntimeError("another campaign controller owns the lock")
    if not (ROOT / "admission.json").exists():
        admission_receipt["campaign_started_unix_ns"] = str(time.time_ns())
        write_json(ROOT / "admission.json", admission_receipt)
    admitted = json.loads((ROOT / "admission.json").read_text())
    if admitted.get("protocol_file_sha256") != HASHES[HERE / "protocol.json"] or admitted.get("plan_sha256") != HASHES[HERE / "plan.json"]:
        raise RuntimeError("existing campaign admission binds different frozen inputs")
    deadline_ns = int(admitted["campaign_started_unix_ns"]) + 12 * 3600 * 10 ** 9
    for index in range(START, 64):
        shard = ROOT / f"shard-{index:02d}"
        cell_receipt = ROOT / "custody" / f"shard-{index:02d}-published.json"
        if cell_receipt.exists():
            published = json.loads(cell_receipt.read_text())
            pointer = ROOT / "custody" / f"shard-{index:02d}.json"
            archive = ROOT / "archives" / f"shard-{index:02d}.tar.gz"
            if not pointer.is_file() or not archive.is_file() or digest(pointer) != published.get("pointer_sha256") or digest(archive) != published.get("archive_sha256"):
                raise RuntimeError(f"published shard {index} has a local custody mismatch")
            git_ready_checkpoint(index)
            continue
        if time.time_ns() > deadline_ns:
            raise RuntimeError("12-hour campaign admission ceiling reached")
        shard.mkdir(exist_ok=True)
        attempt(shard, index)
        status = finalize(shard, index)
        receipt = custody(shard, index, status)
        git_ready_checkpoint(index)
        print(json.dumps({"index": index, "verified_and_exported": status["verified_and_exported"], "archive_sha256": receipt["archive_sha256"], "fresh_get": True}), flush=True)
    print(json.dumps({"selected": 64 - START, "custody_receipts": 64 - START, "complete": True}), flush=True)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("mode", choices=["check", "run"])
    args = parser.parse_args()
    try:
        if args.mode == "check":
            print(json.dumps(admission(), sort_keys=True))
        else:
            run()
    except Exception as exc:
        print(f"campaign stopped: {exc}", file=sys.stderr)
        sys.exit(1)
